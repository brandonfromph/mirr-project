// Testbench: shift_register_guard.mirr -> short_delay_monitor module
// Tests: sensor_active held for 8+ cycles causes alert_lamp to go high.
`timescale 1ns/1ps

module shift_register_tb;
    reg clk, rst_n;
    reg sensor_active;
    wire alert_lamp;

    short_delay_monitor dut (
        .clk(clk),
        .rst_n(rst_n),
        .sensor_active(sensor_active),
        .alert_lamp(alert_lamp)
    );

    initial clk = 0;
    always #5 clk = ~clk;

    integer cycle;
    initial begin
        rst_n = 0;
        sensor_active = 0;
        #20 rst_n = 1;

        // Hold sensor_active high for 12 cycles (> 8 required)
        @(posedge clk);
        sensor_active = 1;
        for (cycle = 0; cycle < 12; cycle = cycle + 1) begin
            @(posedge clk);
        end

        // Check that alert_lamp is asserted
        if (alert_lamp === 1'b1) begin
            $display("PASS: shift_register_tb — alert_lamp asserted after sustained activation");
        end else begin
            $display("FAIL: shift_register_tb — alert_lamp not asserted (got %b)", alert_lamp);
        end

        #10 $finish;
    end

    // Timeout watchdog
    initial begin
        #10000;
        $display("FAIL: shift_register_tb — timeout");
        $finish;
    end
endmodule
