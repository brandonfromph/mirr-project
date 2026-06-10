// R-SPU 2.0 Register File
// Scaled to 1024 registers (10-bit addressing) for RS-16 SoC
module regfile (
    input  logic        clk,
    input  logic [9:0]  rs1,
    input  logic [9:0]  rs2,
    input  logic [9:0]  rd,
    input  logic        we,
    input  logic [63:0] din,
    output logic [63:0] dout1,
    output logic [63:0] dout2
);
    logic [63:0] regs [0:1023];

    // Synchronous Write
    always_ff @(posedge clk) begin
        if (we) begin
            regs[rd] <= din;
        end
    end

    // Combinatorial Read
    assign dout1 = regs[rs1];
    assign dout2 = regs[rs2];

endmodule
