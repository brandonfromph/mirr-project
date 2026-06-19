module test_sva(input clk, rst_n, a, b);
  always @(posedge clk) begin
    if (rst_n) begin
      if (a) assert(b);
    end
  end
endmodule
