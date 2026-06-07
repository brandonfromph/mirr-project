// R-SPU Generic Memory
// Optimized for Block RAM (BRAM) inference in Yosys
module ram (
    input  logic        clk,
    input  logic [15:0] addr,
    input  logic [63:0] data_in,
    output logic [63:0] data_out
);
    // 64K depth for initial prototype (fits in standard FPGA BRAM blocks)
    logic [63:0] mem [0:65535];

    // Synchronous Read/Write (Strict BRAM Template)
    always_ff @(posedge clk) begin
        if (addr != 16'h0000) begin
            mem[addr] <= data_in;
        end
        data_out <= mem[addr];
    end

endmodule
