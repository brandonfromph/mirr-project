module ram(
  input wire clk,
  input wire [9:0] addr,
  input wire [63:0] din,
  output wire [63:0] dout
);
  reg [63:0] mem [0:1023];
  always @(posedge clk) begin
    mem[addr] <= din;
  end
  assign dout = mem[addr];
endmodule

module regfile(
  input wire clk,
  input wire [5:0] rs1,
  input wire [5:0] rs2,
  input wire [5:0] rd,
  input wire we,
  input wire [63:0] din,
  output wire [63:0] val1,
  output wire [63:0] val2
);
  reg [63:0] regs [0:63];
  always @(posedge clk) begin
    if (we && rd != 0)
      regs[rd] <= din;
  end
  assign val1 = regs[rs1];
  assign val2 = regs[rs2];
endmodule
