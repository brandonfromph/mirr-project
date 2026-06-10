// Auto-generated SVA bind file from MIRR compiler
// Bind target: watchdog_timer
// Use with: read_verilog -sv <this_file> (formal verification only)

module watchdog_timer_sva (
  input  logic        clk,
  input  logic        rst_n,
  input  logic        heartbeat,
  input  logic        watchdog_reset,
  input  logic        system_alive
);

  // property: reset_on_timeout
  assert property (@(posedge clk)
    (!heartbeat) |-> watchdog_reset);

  // property: no_hang
  assert property (@(posedge clk)
    !(((!heartbeat) & (!watchdog_reset))));

endmodule

bind watchdog_timer watchdog_timer_sva u_sva (.*);
