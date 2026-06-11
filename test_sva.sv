module test(input clk, input rst_n, input signed [63:0] robot_angle);
  assert property (@(posedge clk) disable iff (!rst_n) (((robot_angle < 100000) & (robot_angle > (-100000)))));
endmodule
