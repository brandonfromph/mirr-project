import os

for g in range(4):
    l1_router = f"//! Auto-generated L1 NoC Interconnect Router for Group {g}\n"
    l1_router += f"pattern noc_l1_router_{g}(\n"
    l1_router += """    clk: signal in bool,
    rst_n: signal in bool,
"""
    for i in range(16):
        l1_router += f"    tx_valid_{i}: signal in bool,\n"
        l1_router += f"    tx_data_{i}: signal in u64,\n"
    
    l1_router += """    downlink_valid: signal in bool,
    downlink_data: signal in u64,
"""
    for i in range(16):
        l1_router += f"    rx_valid_{i}: signal out bool,\n"
        l1_router += f"    rx_data_{i}: signal out u64,\n"

    l1_router += """    uplink_valid: signal out bool,
    uplink_data: signal out u64
) {
"""
    for i in range(18):
        l1_router += f"    signal sv_{i}: internal bool;\n"
        l1_router += f"    signal sd_{i}: internal u64;\n"
    
    l1_router += "\n    // --- MAPE-K Monitor State ---\n"
    for i in range(16):
        l1_router += f"    signal heartbeat_{i}: internal u32;\n"
        l1_router += f"    signal core_dead_{i}: internal bool;\n"

    l1_router += "\n    reflex r_dl { on always { sv_0 = downlink_valid; sd_0 = downlink_data; } }\n"

    for i in range(16):
        l1_router += f"    guard n{i} {{\n        when (!tx_valid_{i}) for 1 cycles;\n    }}\n"
        l1_router += f"    guard y{i} {{\n        when (tx_valid_{i}) for 1 cycles;\n    }}\n"
        l1_router += f"    reflex r{i} {{ on y{i} {{ sv_{i+1} = true; sd_{i+1} = tx_data_{i}; heartbeat_{i} = 0; }} on n{i} {{ sv_{i+1} = sv_{i}; sd_{i+1} = sd_{i}; heartbeat_{i} = heartbeat_{i} + 1; }} }}\n"
        
        # MAPE-K Analyze & Plan
        l1_router += f"    reflex analyze_{i} {{ on always {{ core_dead_{i} = heartbeat_{i} > 1024; }} }}\n"

    l1_router += """
    signal dest_id: internal u64;
    signal payload: internal u64;
    signal is_local: internal bool;

    reflex extract {
        on always {
            dest_id = (sd_16 >> 48) & 4095;
            payload = sd_16 & 281474976710655; // (1<<48)-1
"""
    l1_router += f"            is_local = (dest_id >= {g*16}) && (dest_id < {(g+1)*16});\n"
    l1_router += """        }
    }

    // Default outputs
    reflex defaults {
        on always {
            uplink_valid = false;
            uplink_data = 0;
"""
    for i in range(16):
        l1_router += f"            rx_valid_{i} = false;\n"
        l1_router += f"            rx_data_{i} = 0;\n"
    l1_router += """        }
    }
"""

    for i in range(16):
        # MAPE-K Execute (Bypass Dead Cores)
        l1_router += f"    guard p{i}_alive {{\n        when ((dest_id == {g*16 + i}) && !core_dead_{i}) for 1 cycles;\n    }}\n"
        l1_router += f"    guard p{i}_dead {{\n        when ((dest_id == {g*16 + i}) && core_dead_{i}) for 1 cycles;\n    }}\n"
        l1_router += f"    reflex out{i}_alive {{ on p{i}_alive {{ rx_valid_{i} = sv_16; rx_data_{i} = payload; }} }}\n"

    l1_router += """
    guard ext {
        when (!is_local) for 1 cycles;
    }
    
    // Bounce packets for dead local cores back to L2
    signal bounce_valid: internal bool;
    signal bounce_data: internal u64;

    // Check if any local core is dead and is the destination
    signal should_bounce: internal bool;
    reflex eval_bounce_cond {
        on always {
            should_bounce = false;
"""
    for i in range(16):
        l1_router += f"            should_bounce = should_bounce || (dest_id == {g*16 + i} && core_dead_{i});\n"
        
    l1_router += """        }
    }

    guard bounce_y { when (should_bounce) for 1 cycles; }
    guard bounce_n { when (!should_bounce) for 1 cycles; }

    reflex do_bounce_y { on bounce_y { bounce_valid = sv_16; bounce_data = sd_16 | 9223372036854775808; } }
    reflex do_bounce_n { on bounce_n { bounce_valid = false; bounce_data = 0; } }

    reflex out_ext { on ext { uplink_valid = sv_16 || bounce_valid; uplink_data = sd_16 | bounce_data; } }
}

"""
    l1_router += f"module noc_l1_router_{g} {{}}\n"

    with open(f"reflex_soc/interconnect/noc_l1_router_{g}.mirr", "w") as f:
        f.write(l1_router)

l2_router = """//! Auto-generated L2 Global NoC Interconnect Router
pattern noc_l2_router(
    clk: signal in bool,
    rst_n: signal in bool,
"""
for i in range(4):
    l2_router += f"    uplink_valid_{i}: signal in bool,\n"
    l2_router += f"    uplink_data_{i}: signal in u64,\n"
for i in range(4):
    l2_router += f"    downlink_valid_{i}: signal out bool,\n"
    l2_router += f"    downlink_data_{i}: signal out u64" + (",\n" if i < 3 else "\n")
l2_router += """) {
"""
for i in range(6):
    l2_router += f"    signal sv_{i}: internal bool;\n"
    l2_router += f"    signal sd_{i}: internal u64;\n"

l2_router += """
    reflex r_base { on always { sv_0 = false; sd_0 = 0; } }
"""
for i in range(4):
    l2_router += f"    // Adaptive Buffer for L1 Group {i}\n"
    l2_router += f"    signal buf_valid_{i}: internal bool;\n"
    l2_router += f"    signal buf_data_{i}: internal u64;\n"
    l2_router += f"    signal is_bounce_{i}: internal bool;\n"
    l2_router += f"    signal act_valid_{i}: internal bool;\n"
    l2_router += f"    signal act_data_{i}: internal u64;\n"
    
    l2_router += f"    reflex parse_bounce_{i} {{\n"
    l2_router += f"        on always {{\n"
    l2_router += f"            is_bounce_{i} = uplink_valid_{i} && ((uplink_data_{i} >> 63) == 1);\n"
    l2_router += f"        }}\n"
    l2_router += f"    }}\n\n"

    # Define native mutually exclusive guards to avoid compiler overlapping guard checks
    l2_router += f"    guard g_bounce_{i} {{ when (is_bounce_{i}) for 1 cycles; }}\n"
    l2_router += f"    guard g_norm_{i} {{ when (!is_bounce_{i} && uplink_valid_{i}) for 1 cycles; }}\n"
    l2_router += f"    guard g_retry_{i} {{ when (!uplink_valid_{i} && buf_valid_{i}) for 1 cycles; }}\n"
    l2_router += f"    guard g_idle_{i} {{ when (!uplink_valid_{i} && !buf_valid_{i}) for 1 cycles; }}\n\n"

    l2_router += f"    reflex r_b_{i} {{\n"
    l2_router += f"        on g_bounce_{i} {{\n"
    l2_router += f"            buf_valid_{i} = true;\n"
    l2_router += f"            buf_data_{i} = uplink_data_{i} & 9223372036854775807;\n"
    l2_router += f"            act_valid_{i} = false;\n"
    l2_router += f"            act_data_{i} = 0;\n"
    l2_router += f"        }}\n"
    l2_router += f"    }}\n"

    l2_router += f"    reflex r_n_{i} {{\n"
    l2_router += f"        on g_norm_{i} {{\n"
    l2_router += f"            act_valid_{i} = true;\n"
    l2_router += f"            act_data_{i} = uplink_data_{i};\n"
    l2_router += f"        }}\n"
    l2_router += f"    }}\n"

    l2_router += f"    reflex r_r_{i} {{\n"
    l2_router += f"        on g_retry_{i} {{\n"
    l2_router += f"            act_valid_{i} = true;\n"
    l2_router += f"            act_data_{i} = buf_data_{i};\n"
    l2_router += f"            buf_valid_{i} = false;\n"
    l2_router += f"        }}\n"
    l2_router += f"    }}\n"

    l2_router += f"    reflex r_i_{i} {{\n"
    l2_router += f"        on g_idle_{i} {{\n"
    l2_router += f"            act_valid_{i} = false;\n"
    l2_router += f"            act_data_{i} = 0;\n"
    l2_router += f"        }}\n"
    l2_router += f"    }}\n\n"

    l2_router += f"    guard n{i} {{\n        when (!act_valid_{i}) for 1 cycles;\n    }}\n"
    l2_router += f"    guard y{i} {{\n        when (act_valid_{i}) for 1 cycles;\n    }}\n"
    l2_router += f"    reflex r{i} {{ on y{i} {{ sv_{i+1} = true; sd_{i+1} = act_data_{i}; }} on n{i} {{ sv_{i+1} = sv_{i}; sd_{i+1} = sd_{i}; }} }}\n"

l2_router += """
    signal dest_id: internal u64;
    signal payload: internal u64;

    reflex extract {
        on always {
            dest_id = (sd_4 >> 48) & 4095;
            payload = sd_4; // Keep full payload including destination for L1
        }
    }

    reflex defaults {
        on always {
"""
for i in range(4):
    l2_router += f"            downlink_valid_{i} = false;\n"
    l2_router += f"            downlink_data_{i} = 0;\n"
l2_router += """        }
    }

"""

for i in range(4):
    l2_router += f"    guard p{i} {{\n        when ((dest_id >= {i*16}) && (dest_id < {(i+1)*16})) for 1 cycles;\n    }}\n"
    l2_router += f"    reflex out{i} {{ on p{i} {{ downlink_valid_{i} = sv_4; downlink_data_{i} = payload; }} }}\n"

l2_router += """}

module noc_l2_router {}
"""

with open("reflex_soc/interconnect/noc_l2_router.mirr", "w") as f:
    f.write(l2_router)

print("Generated noc_l1_router.mirr and noc_l2_router.mirr")

