module test_sva(input clk, rst_n, a, b);
  always @(posedge clk) begin
    if (rst_n) assert property (a |-> b);
  end
endmodule
