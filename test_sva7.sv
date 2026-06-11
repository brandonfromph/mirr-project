module test(input clk, input rst_n, input a, input b);
  always @(posedge clk) begin
      assert property (a |-> b);
  end
endmodule
