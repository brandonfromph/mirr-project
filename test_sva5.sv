module test_sva(input clk, rst_n, a, b);
  assert property (a |-> b);
endmodule
