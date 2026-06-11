module test(input clk, input rst_n, input signed [63:0] robot_angle);
  always @(posedge clk) begin
      if (rst_n) begin
          assert(robot_angle < 100000);
      end
  end
endmodule
