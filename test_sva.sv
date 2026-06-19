module test_sva(input clk, rst_n, a, b);
  assert property (@(posedge clk) disable iff (!rst_n) (a |-> b));
endmodule
