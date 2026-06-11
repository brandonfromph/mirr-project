module test(input clk, input rst_n, input signed [63:0] robot_angle);
  property p;
    @(posedge clk) disable iff (!rst_n) (robot_angle < 100000);
  endproperty
  assert property (p);
endmodule
