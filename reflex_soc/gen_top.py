import sys
import os

def gen_top():
    top = """import "core/core_top.mirr" as core;
import "interconnect/noc_l1_router_0.mirr" as l1_0;
import "interconnect/noc_l1_router_1.mirr" as l1_1;
import "interconnect/noc_l1_router_2.mirr" as l1_2;
import "interconnect/noc_l1_router_3.mirr" as l1_3;

module rspu_top {
    signal clk: in bool;
    signal rst_n: in bool;

    // --- Core Signals (Full 64-Core Design) ---
    for i in 0..64 {
        signal instr[i]: internal u64;
        signal pc[i]: internal u32;
        signal core_data[i]: internal u64;
        signal rx_valid[i]: internal bool;
        signal rx_data[i]: internal u64;
        signal tx_valid[i]: internal bool;
        signal tx_data[i]: internal u64;

        // Physical Outputs
        signal out_pc[i]: out u32;
        signal out_data[i]: out u64;
        
        // Physical I/O
        signal io_in[i]: in u64;
        signal io_out[i]: out u64;
    }

    signal global_trap: out bool;



    // --- 64-Core Integration ---
    for i in 0..64 {
        core::core_top(clk, rst_n, io_in[i], io_out[i], pc[i], core_data[i], rx_valid[i], rx_data[i], tx_valid[i], tx_data[i]);
    }

    // --- Hierarchical NoC Interconnect ---
    
    // --- DMA Interface (C++ Replaces L2 Router) ---
    for i in 0..4 {
        signal uplink_valid[i]: out bool;
        signal uplink_data[i]: out u64;
        signal downlink_valid[i]: in bool;
        signal downlink_data[i]: in u64;
    }


"""
    # L1 Routers
    for group in range(4):
        base = group * 16
        top += f"    // L1 Router {group} (Cores {base}-{base+15})\n"
        top += f"    l1_{group}::noc_l1_router_{group}(\n"
        top += f"        clk, rst_n,\n"
        
        # TX inputs
        for i in range(16):
            core_id = base + i
            top += f"        tx_valid_{core_id}, tx_data_{core_id},\n"
            
        # Downlink input
        top += f"        downlink_valid_{group}, downlink_data_{group},\n"
        
        # RX outputs
        for i in range(16):
            core_id = base + i
            top += f"        rx_valid_{core_id}, rx_data_{core_id},\n"
            
        # Uplink output
        top += f"        uplink_valid_{group}, uplink_data_{group}\n"
        top += f"    );\n\n"

    top += "    signal trap_0: internal bool;\n"
    top += "    signal trap_1: internal bool;\n"
    top += "    signal trap_2: internal bool;\n"
    top += "    signal trap_3: internal bool;\n"
    top += """    // --- Closed-Loop Verification Logic ---

    reflex collect_diagnostics {
        on always {
            for i in 0..64 {
                out_pc[i] = pc[i];
                out_data[i] = core_data[i];
            }
"""
    
    for group in range(4):
        base = group * 16
        trap_chunk = " || ".join([f"tx_valid_{base+i}" for i in range(16)])
        top += f"            trap_{group} = {trap_chunk};\n"

    top += f"            global_trap = trap_0 || trap_1 || trap_2 || trap_3;\n"
    top += """        }
    }

}
"""

    with open(os.path.join(os.path.dirname(__file__), "rspu_top.mirr"), "w") as f:
        f.write(top)

if __name__ == "__main__":
    gen_top()
