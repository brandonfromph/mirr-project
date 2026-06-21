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
  always @(posedge clk) begin
    assert((airway_pressure > 10));
  end

  // property: no_spurious_clamp
  always @(posedge clk) begin
    assert(!((clamp_valve & (airway_pressure > 200))));
  end

  // property: low_triggers_clamp
  always @(posedge clk) begin
    assert((!((airway_pressure < 50))) || (clamp_valve));
  end

  // property: clamp_reachable
  reg [31:0] prop_clamp_reachable_timer;
  always @(posedge clk) begin
    if (clamp_valve) prop_clamp_reachable_timer <= 0;
    else prop_clamp_reachable_timer <= prop_clamp_reachable_timer + 1;
  end
  always @(posedge clk) begin
    cover(prop_clamp_reachable_timer < 100);
  end

  // property: clamp_follows_drop
  reg [4:0] prop_clamp_follows_drop_trig_shift;
  always @(posedge clk) begin
    prop_clamp_follows_drop_trig_shift <= {prop_clamp_follows_drop_trig_shift[3:0], (airway_pressure < 50)};
  end
  always @(posedge clk) begin
    assert((!(prop_clamp_follows_drop_trig_shift[4])) || (clamp_valve));
  end

endmodule

bind pressure_monitor pressure_monitor_sva u_sva (.*);
