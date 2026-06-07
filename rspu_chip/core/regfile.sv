// R-SPU Register File
// Optimized for BRAM inference in Yosys
module regfile (
    input  logic        clk,
    input  logic [7:0]  rs1,
    input  logic [7:0]  rs2,
    input  logic [7:0]  rd,
    input  logic        we,
    input  logic [63:0] din,
    output logic [63:0] dout1,
    output logic [63:0] dout2
);
    logic [63:0] regs [0:255];

    // Synchronous Write
    always_ff @(posedge clk) begin
        if (we) begin
            regs[rd] <= din;
        end
    end

    // Combinational Read (Asynchronous for regfile performance)
    // Note: To infer true BRAM, reads should be synchronous. 
    // However, regfiles often use distributed RAM (LUTRAM) for single-cycle read.
    assign dout1 = regs[rs1];
    assign dout2 = regs[rs2];

endmodule
