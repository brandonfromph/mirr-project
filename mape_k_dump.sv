// --- MAPE-K Top -------------------------------------------
module mirr_mape_k_top #(
  parameter N_SIGNALS    = 2,
  parameter N_PROPERTIES = 2,
  parameter N_ACTIONS    = 2,
  parameter K_DEPTH      = 128
) (
  input  logic clk,
  input  logic rst_n,
  input  logic [N_SIGNALS-1:0][31:0] sensor_in,
  output logic        emergency_active,
  output logic [N_SIGNALS-1:0][31:0] signal_override,
  output logic [N_SIGNALS-1:0]       override_en
);

  // Monitor -> Analyze
  logic [N_SIGNALS-1:0][31:0] shadow;
  logic        sample_valid;

  // Analyze -> Plan
  logic [1:0] violation_vec;
  logic [0:0] top_violation_idx;

  // Plan -> Execute
  logic [0:0] selected_action_idx;
  logic        action_valid;

  // Knowledge write channel
  logic [7:0] k_count;
  logic        k_full;
  logic [31:0] tick_counter;

  always_ff @(posedge clk or negedge rst_n) begin
    if (!rst_n)
      tick_counter <= 32'd0;
    else
      tick_counter <= tick_counter + 1;
  end

  mirr_monitor #(
    .N_SIGNALS   (N_SIGNALS),
    .TRACE_DEPTH (64)
  ) u_monitor (
    .clk          (clk),
    .rst_n        (rst_n),
    .sensor_in    (sensor_in),
    .shadow       (shadow),
    .sample_valid (sample_valid)
  );

  mirr_analyze #(
    .N_SIGNALS    (N_SIGNALS),
    .N_PROPERTIES (N_PROPERTIES)
  ) u_analyze (
    .clk               (clk),
    .rst_n             (rst_n),
    .shadow            (shadow),
    .sample_valid      (sample_valid),
    .violation_vec     (violation_vec),
    .top_violation_idx (top_violation_idx)
  );

  mirr_plan #(
    .N_PROPERTIES (N_PROPERTIES),
    .N_ACTIONS    (N_ACTIONS)
  ) u_plan (
    .clk                (clk),
    .rst_n              (rst_n),
    .violation_vec      (violation_vec),
    .selected_action_idx(selected_action_idx),
    .action_valid       (action_valid)
  );

  mirr_execute #(
    .N_SIGNALS (N_SIGNALS),
    .N_ACTIONS (N_ACTIONS)
  ) u_execute (
    .clk                (clk),
    .rst_n              (rst_n),
    .selected_action_idx(selected_action_idx),
    .action_valid       (action_valid),
    .signal_override    (signal_override),
    .override_en        (override_en),
    .emergency_active   (emergency_active)
  );

  mirr_knowledge #(
    .DEPTH (K_DEPTH)
  ) u_knowledge (
    .clk           (clk),
    .rst_n         (rst_n),
    .wr_en         (action_valid),
    .wr_action_idx (selected_action_idx),
    .wr_tick       (tick_counter),
    .count         (k_count),
    .full          (k_full)
  );

endmodule
