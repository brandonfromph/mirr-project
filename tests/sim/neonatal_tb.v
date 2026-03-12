// Testbench: neonatal_respirator.mirr -> neonatal_respirator module
// Tests: airway_pressure below 50 for 1000+ cycles triggers clamp_valve.
`timescale 1ns/1ps

module neonatal_tb;
    reg clk, rst_n;
    reg respirator_enable;
    reg [15:0] airway_pressure;
    wire clamp_valve;

    neonatal_respirator dut (
        .clk(clk),
        .rst_n(rst_n),
        .respirator_enable(respirator_enable),
        .airway_pressure(airway_pressure),
        .clamp_valve(clamp_valve)
    );

    initial clk = 0;
    always #5 clk = ~clk;

    integer cycle;
    initial begin
        rst_n = 0;
        respirator_enable = 1;
        airway_pressure = 100; // safe pressure
        #20 rst_n = 1;

        // Drop pressure below threshold for 1100 cycles (> 1000 required)
        @(posedge clk);
        airway_pressure = 30; // below 50
        for (cycle = 0; cycle < 1100; cycle = cycle + 1) begin
            @(posedge clk);
        end

        // Check that clamp_valve is asserted
        if (clamp_valve === 1'b1) begin
            $display("PASS: neonatal_tb — clamp_valve asserted after sustained pressure drop");
        end else begin
            $display("FAIL: neonatal_tb — clamp_valve not asserted (got %b)", clamp_valve);
        end

        #10 $finish;
    end

    // Timeout watchdog
    initial begin
        #100000;
        $display("FAIL: neonatal_tb — timeout");
        $finish;
    end
endmodule
