// Auto-generated SVA bind file from MIRR compiler
// Bind target: pressure_monitor
// Use with: read_verilog -sv <this_file> (formal verification only)

module pressure_monitor_sva (
  input  logic        clk,
  input  logic        rst_n,
  input  logic [15:0] airway_pressure,
  input  logic        clamp_valve
);

  // property: pressure_bounded
  assert property (@(posedge clk)
    (airway_pressure > 10));

  // property: no_spurious_clamp
  assert property (@(posedge clk)
    !((clamp_valve & (airway_pressure > 200))));

  // property: low_triggers_clamp
  assert property (@(posedge clk)
    (airway_pressure < 50) |-> clamp_valve);

  // property: clamp_reachable
  cover property (@(posedge clk)
    ##[1:100] clamp_valve);

  // property: clamp_follows_drop
  assert property (@(posedge clk)
    (airway_pressure < 50) |-> ##5 clamp_valve);

endmodule

bind pressure_monitor pressure_monitor_sva u_sva (.*);
