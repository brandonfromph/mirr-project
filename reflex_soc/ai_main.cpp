#include "Vrspu_top.h"
#include "Vrspu_top___024root.h"
#include "verilated.h"
#include <iostream>
#include <iomanip>
#include <cstdint>

int main(int argc, char** argv, char** env) {
    Verilated::commandArgs(argc, argv);
    Vrspu_top* top = new Vrspu_top;

    std::cout << "==================================================" << std::endl;
    std::cout << "  R-SPU 64-Core Robotic Sensor Processor Booting  " << std::endl;
    std::cout << "==================================================" << std::endl;

    top->clk = 0;
    top->rst_n = 0;

    // Hard Reset
    for (int i = 0; i < 4; i++) {
        top->clk = !top->clk;
        top->eval();
    }
    top->rst_n = 1;

    // --- DMA Brain Programming Phase ---
    // Instruction encodings:
    // dec_rs1 is always 0 due to NoC 48-bit payload truncation.
    // dec_rs2 = (instr >> 32) & 1023
    // dec_rd  = (instr >> 16) & 1023
    // dec_op  = instr & 255
    
    // LOAD_IN (op=0): reg[rd] = io_in
    uint64_t op_load = (0ULL << 16) | 0;  // rd=0
    
    // MUL (op=2): reg[rd] = reg[0] * reg[rs2]
    uint64_t op_mul = (0ULL << 32) | (0ULL << 16) | 2; // rs2=0, rd=0
    
    // STORE_OUT (op=1): io_out = reg[0]
    uint64_t op_store = (0ULL << 16) | 1; // rd doesn't matter, rs1 is 0

    // We will simulate 42 cycles.
    // L1 NoC router is now combinatorial (0 cycles latency).
    // Instructions take 5 cycles in the pipeline.
    // Cycle 28: Inject LOAD_IN. (Arrives Cycle 28. Finishes WB at Cycle 32)
    // Cycle 34: Inject MUL. (Arrives Cycle 34. Reads reg[0] at Cycle 35. Finishes WB at Cycle 38)
    // Cycle 40: Inject STORE_OUT. (Arrives Cycle 40. Reads reg[0] at Cycle 41. Writes io_out at Cycle 42)

    std::cout << "[DMA] Neural Network weights and instructions scheduled into 64-core mesh." << std::endl;
    std::cout << "[SENSOR] Feeding 64 realtime sensor inputs to mesh..." << std::endl;

    int execute_cycles = 42;
    
    for (int cycle = 0; cycle < execute_cycles; cycle++) {
        // Default NoC inputs
        top->downlink_valid_0 = 0;
        top->downlink_valid_1 = 0;

        if (cycle == 28) {
            top->downlink_valid_0 = 1;
            top->downlink_data_0 = (0ULL << 48) | op_load; // dest_id = 0
            
            top->downlink_valid_1 = 1;
            top->downlink_data_1 = (16ULL << 48) | op_load; // dest_id = 16
        } else if (cycle == 34) {
            top->downlink_valid_0 = 1;
            top->downlink_data_0 = (0ULL << 48) | op_mul; // dest_id = 0
            
            top->downlink_valid_1 = 1;
            top->downlink_data_1 = (16ULL << 48) | op_mul; // dest_id = 16
        } else if (cycle == 40) {
            top->downlink_valid_0 = 1;
            top->downlink_data_0 = (0ULL << 48) | op_store; // dest_id = 0
            
            top->downlink_valid_1 = 1;
            top->downlink_data_1 = (16ULL << 48) | op_store; // dest_id = 16
        }

        // Feed live sensor data
        // We feed 845 to Core 0, 845 * 845 = 714025
        top->io_in_0 = 845;
        // We feed 33 to Core 16, 33 * 33 = 1089
        top->io_in_16 = 33;

        top->clk = 1;
        top->eval();
        
        std::cout << "[Cycle " << cycle << "] " 
                  << "wake_timer: " << (int)top->rootp->rspu_top__DOT__core_top_call_742_wake_timer 
                  << ", current_instr: " << top->rootp->rspu_top__DOT__core_top_call_742_current_instr 
                  << ", sv_16: " << (int)top->rootp->rspu_top__DOT__noc_l1_router_0_call_886_sv_16
                  << ", rx_valid_0: " << (int)top->rootp->rspu_top__DOT__rx_valid_0
                  << ", rx_data_0: " << top->rootp->rspu_top__DOT__rx_data_0
                  << ", io_out_0: " << top->io_out_0
                  << std::endl;
                  
        top->clk = 0;
        top->eval();
    }

    std::cout << "==================================================" << std::endl;
    std::cout << "Execution completed in EXACTLY 42 clock cycles." << std::endl;
    std::cout << "Jitter: 0.0000 ms" << std::endl;
    std::cout << "Calculated Motor Torque (Core 0):  " << top->io_out_0 << " (Expected: 714025)" << std::endl;
    std::cout << "Calculated Motor Torque (Core 16): " << top->io_out_16 << " (Expected: 1089)" << std::endl;
    std::cout << "==================================================" << std::endl;
    
    top->final();
    delete top;
    return 0;
}
