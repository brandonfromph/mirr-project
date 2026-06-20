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
    
    reflex out_ext { on ext { uplink_valid = sv_16; uplink_data = sd_16; } }
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

for i in range(4):
    l2_router += f"    signal buf_valid_{i}: internal bool;\n"
    l2_router += f"    signal buf_data_{i}: internal u64;\n"
    l2_router += f"    signal is_reflex_{i}: internal bool;\n"
    l2_router += f"    signal is_normal_{i}: internal bool;\n"
    l2_router += f"    signal req_valid_{i}: internal bool;\n"
    l2_router += f"    signal req_data_{i}: internal u64;\n"

for i in range(4):
    l2_router += f"    signal w_r_{i}: internal bool;\n"
    l2_router += f"    signal w_n_{i}: internal bool;\n"

l2_router += """
    signal any_r: internal bool;
    signal any_w: internal bool;
    
    reflex req_eval {
        on always {
"""
for i in range(4):
    # Reflex check: bit 62 is 1
    l2_router += f"            is_reflex_{i} = uplink_valid_{i} && ((uplink_data_{i} >> 62) == 1);\n"
    l2_router += f"            is_normal_{i} = uplink_valid_{i} && !is_reflex_{i};\n"
    # Current active request is either the new reflex, or the buffered packet if no new reflex
    # Wait, if a normal packet arrives, we buffer it if it doesn't win.
    l2_router += f"            req_valid_{i} = is_reflex_{i} || is_normal_{i} || buf_valid_{i};\n"
    l2_router += f"            // We prioritize the incoming packet over the buffer for processing if both exist\n"
    # Wait, we can't do conditional logic easily for data. Let's just process uplink directly if valid, else buffer.
    # Actually, we can just process reflex > normal > buffer
l2_router += """        }
    }
"""

l2_router += """
    reflex win_eval {
        on always {
"""
# Priority preemption logic
for i in range(4):
    prefix_r = " && ".join([f"!w_r_{j}" for j in range(i)])
    if prefix_r:
        l2_router += f"            w_r_{i} = is_reflex_{i} && {prefix_r};\n"
    else:
        l2_router += f"            w_r_{i} = is_reflex_{i};\n"

l2_router += "            any_r = w_r_0 || w_r_1 || w_r_2 || w_r_3;\n"

for i in range(4):
    prefix_n = " && ".join([f"!w_n_{j}" for j in range(i)])
    if prefix_n:
        l2_router += f"            w_n_{i} = (is_normal_{i} || buf_valid_{i}) && !any_r && {prefix_n};\n"
    else:
        l2_router += f"            w_n_{i} = (is_normal_{i} || buf_valid_{i}) && !any_r;\n"

l2_router += """        }
    }
"""

# Extract the winning payload
l2_router += """
    signal win_valid: internal bool;
    signal win_data: internal u64;
    signal win_dest: internal u64;

"""

for i in range(4):
    # Guard for when this channel wins reflex
    l2_router += f"    guard g_win_r_{i} {{ when (w_r_{i}) for 1 cycles; }}\n"
    l2_router += f"    reflex do_win_r_{i} {{ on g_win_r_{i} {{ win_valid = true; win_data = uplink_data_{i}; }} }}\n"
    
    # Guard for when this channel wins normal
    l2_router += f"    guard g_win_n_{i}_up {{ when (w_n_{i} && is_normal_{i}) for 1 cycles; }}\n"
    l2_router += f"    reflex do_win_n_{i}_up {{ on g_win_n_{i}_up {{ win_valid = true; win_data = uplink_data_{i}; }} }}\n"

    l2_router += f"    guard g_win_n_{i}_buf {{ when (w_n_{i} && !is_normal_{i} && buf_valid_{i}) for 1 cycles; }}\n"
    l2_router += f"    reflex do_win_n_{i}_buf {{ on g_win_n_{i}_buf {{ win_valid = true; win_data = buf_data_{i}; }} }}\n"
    
    # Buffering logic: if a normal packet arrives but doesn't win, we store it in the buffer.
    # If it wins, we clear the buffer (if it was from the buffer).
    l2_router += f"    guard g_buf_store_{i} {{ when (is_normal_{i} && !w_n_{i}) for 1 cycles; }}\n"
    l2_router += f"    reflex do_buf_store_{i} {{ on g_buf_store_{i} {{ buf_valid_{i} = true; buf_data_{i} = uplink_data_{i}; }} }}\n"
    
    l2_router += f"    guard g_buf_clear_{i} {{ when (w_n_{i} && !is_normal_{i}) for 1 cycles; }}\n"
    l2_router += f"    reflex do_buf_clear_{i} {{ on g_buf_clear_{i} {{ buf_valid_{i} = false; buf_data_{i} = 0; }} }}\n"
    
    # Maintain buffer if no new arrival and didn't win
    l2_router += f"    guard g_buf_keep_{i} {{ when (!is_normal_{i} && !w_n_{i}) for 1 cycles; }}\n"
    l2_router += f"    reflex do_buf_keep_{i} {{ on g_buf_keep_{i} {{ buf_valid_{i} = buf_valid_{i}; buf_data_{i} = buf_data_{i}; }} }}\n"

l2_router += """
    guard g_no_win { when (!w_r_0 && !w_r_1 && !w_r_2 && !w_r_3 && !w_n_0 && !w_n_1 && !w_n_2 && !w_n_3) for 1 cycles; }
    reflex do_no_win { on g_no_win { win_valid = false; win_data = 0; } }

    reflex extract_dest {
        on always {
            win_dest = (win_data >> 48) & 4095;
        }
    }

    reflex route_defaults {
        on always {
"""
for i in range(4):
    l2_router += f"            downlink_valid_{i} = false;\n"
    l2_router += f"            downlink_data_{i} = 0;\n"
l2_router += """        }
    }
"""

for i in range(4):
    l2_router += f"    guard p_out_{i} {{\n        when (win_valid && (win_dest >= {i*16}) && (win_dest < {(i+1)*16})) for 1 cycles;\n    }}\n"
    l2_router += f"    reflex do_out_{i} {{ on p_out_{i} {{ downlink_valid_{i} = win_valid; downlink_data_{i} = win_data; }} }}\n"

l2_router += """}

module noc_l2_router {}
"""

with open("reflex_soc/interconnect/noc_l2_router.mirr", "w") as f:
    f.write(l2_router)

print("Generated noc_l1_router.mirr and noc_l2_router.mirr")
