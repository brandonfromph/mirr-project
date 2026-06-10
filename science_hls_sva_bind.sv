// Auto-generated SVA bind file from MIRR compiler
// Bind target: ScienceHls
// Use with: read_verilog -sv <this_file> (formal verification only)

module ScienceHls_sva (
  input  logic        clk,
  input  logic        rst_n,
  input  logic signed [ 7:0] a,
  input  logic signed [ 7:0] b,
  input  logic signed [ 7:0] c,
  input  logic signed [16:0] out_val,
  input  logic signed [ 7:0] a_d1,
  input  logic signed [ 7:0] a_d2,
  input  logic signed [ 7:0] a_d3,
  input  logic signed [ 7:0] b_d1,
  input  logic signed [ 7:0] b_d2,
  input  logic signed [ 7:0] b_d3,
  input  logic signed [ 7:0] c_d1,
  input  logic signed [ 7:0] c_d2,
  input  logic signed [ 7:0] c_d3,
  input  logic        rst_n_d1,
  input  logic        rst_n_d2,
  input  logic        rst_n_d3
);

  // property: math_check
  always @(posedge clk) begin
    if (rst_n && ((((a_d3 == a) && (b_d3 == b)) && (c_d3 == c)) && rst_n_d3)) assert ((out_val == ((a + b) * c)));
  end

endmodule

bind ScienceHls ScienceHls_sva u_sva (.*);
