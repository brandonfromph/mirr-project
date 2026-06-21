// Auto-generated SVA bind file from MIRR compiler
// Bind target: test_fail
// Use with: read_verilog -sv <this_file> (formal verification only)

module test_fail_sva (
  input  logic        clk,
  input  logic        rst_n,
  input  logic        a,
  input  logic        b
);

  // property: must_be_false
  always @(posedge clk) begin
    assert((b == 1'b0));
  end

endmodule

bind test_fail test_fail_sva u_sva (.*);
