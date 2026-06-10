// Auto-generated SVA bind file from MIRR compiler
// Bind target: ScienceHls
// Use with: read_verilog -sv <this_file> (formal verification only)

module ScienceHls_sva (
);

  // property: math_check
  assert property (@(posedge clk)
    (out_val == ((a + b) * c)));

endmodule

bind ScienceHls ScienceHls_sva u_sva (.*);
