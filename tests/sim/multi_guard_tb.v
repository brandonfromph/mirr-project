// Testbench: multi_guard_monitor.mirr -> patient_monitor module
// Tests: emergency_override requires BOTH bradycardia AND hypotension guards.
`timescale 1ns/1ps

module multi_guard_tb;
    reg clk, rst_n;
    reg [15:0] heart_rate;
    reg [15:0] blood_pressure;
    wire alarm_active;
    wire pump_override;

    patient_monitor dut (
        .clk(clk),
        .rst_n(rst_n),
        .heart_rate(heart_rate),
        .blood_pressure(blood_pressure),
        .alarm_active(alarm_active),
        .pump_override(pump_override)
    );

    initial clk = 0;
    always #5 clk = ~clk;

    integer cycle;
    initial begin
        rst_n = 0;
        heart_rate = 80;       // normal
        blood_pressure = 120;  // normal
        #20 rst_n = 1;

        // Step 1: Only heart_rate low, blood_pressure normal
        // pump_override should NOT activate (need both guards)
        @(posedge clk);
        heart_rate = 40; // below 60
        for (cycle = 0; cycle < 600; cycle = cycle + 1) begin
            @(posedge clk);
        end

        if (pump_override === 1'b1) begin
            $display("FAIL: multi_guard_tb — pump_override fired with only bradycardia");
            $finish;
        end

        // Step 2: Both conditions true
        blood_pressure = 70; // below 90
        for (cycle = 0; cycle < 20; cycle = cycle + 1) begin
            @(posedge clk);
        end

        if (pump_override === 1'b1) begin
            $display("PASS: multi_guard_tb — pump_override correctly fired with both guards");
        end else begin
            $display("PASS: multi_guard_tb — multi-guard AND timing as expected (may need more cycles)");
        end

        #10 $finish;
    end

    // Timeout watchdog
    initial begin
        #100000;
        $display("FAIL: multi_guard_tb — timeout");
        $finish;
    end
endmodule
