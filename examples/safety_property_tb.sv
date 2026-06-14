// Auto-generated testbench by MIRR compiler (FPGA-001)
// Do not edit — regenerate from .mirr source.
`timescale 1ns / 1ps

module pressure_monitor_tb;

  // Clock and reset
  logic clk;
  logic rst_n;

  // DUT port signals
  logic [15:0] tb_airway_pressure;
  logic        tb_clamp_valve;

  // Clock generation: 100 MHz (10 ns period)
  initial clk = 1'b0;
  always #5 clk = ~clk;

  // DUT instantiation
  pressure_monitor dut (
    .clk(clk),
    .rst_n(rst_n),
    .airway_pressure(tb_airway_pressure),
    .clamp_valve(tb_clamp_valve)
  );

  // Stimulus sequence
  initial begin
    // Phase 1: Reset
    rst_n = 1'b0;
    tb_airway_pressure = '0;
    repeat(10) @(posedge clk);
    rst_n = 1'b1;

    // Phase 2: Drive inputs to max range
    tb_airway_pressure = 16'hFFFF;
    repeat(200) @(posedge clk);

    // Phase 3: Return to zero
    tb_airway_pressure = '0;
    repeat(50) @(posedge clk);

    $display("Testbench pressure_monitor complete.");
    $finish;
  end

endmodule
