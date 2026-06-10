// Auto-generated SVA bind file from MIRR compiler
// Bind target: fir_filter
// Use with: read_verilog -sv <this_file> (formal verification only)

module fir_filter_sva (
  input  logic        clk,
  input  logic        rst_n,
  input  logic [15:0] coeff_0,
  input  logic [15:0] coeff_1,
  input  logic [15:0] coeff_2,
  input  logic [15:0] coeff_3,
  input  logic [15:0] sample_in,
  input  logic        sample_valid,
  input  logic [31:0] filter_out
);

  // property: output_bounded
  assert property (@(posedge clk)
    (filter_out < 4294967295));

endmodule

bind fir_filter fir_filter_sva u_sva (.*);
