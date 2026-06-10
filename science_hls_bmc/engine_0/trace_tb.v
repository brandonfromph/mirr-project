`ifndef VERILATOR
module testbench;
  reg [4095:0] vcdfile;
  reg clock;
`else
module testbench(input clock, output reg genclock);
  initial genclock = 1;
`endif
  reg genclock = 1;
  reg [31:0] cycle = 0;
  reg [0:0] PI_rst_n;
  reg [7:0] PI_a;
  reg [7:0] PI_b;
  reg [7:0] PI_c;
  wire [0:0] PI_clk = clock;
  ScienceHls UUT (
    .rst_n(PI_rst_n),
    .a(PI_a),
    .b(PI_b),
    .c(PI_c),
    .clk(PI_clk)
  );
`ifndef VERILATOR
  initial begin
    if ($value$plusargs("vcd=%s", vcdfile)) begin
      $dumpfile(vcdfile);
      $dumpvars(0, testbench);
    end
    #5 clock = 0;
    while (genclock) begin
      #5 clock = 0;
      #5 clock = 1;
    end
  end
`endif
  initial begin
`ifndef VERILATOR
    #1;
`endif
    // UUT.$auto$async2sync.\cc:107:execute$160  = 1'b0;
    // UUT.$auto$async2sync.\cc:116:execute$164  = 1'b1;
    UUT._witness_.anyinit_procdff_103 = 8'b00000000;
    UUT._witness_.anyinit_procdff_108 = 8'b00000001;
    UUT._witness_.anyinit_procdff_113 = 8'b00000000;
    UUT._witness_.anyinit_procdff_118 = 8'b00000000;
    UUT._witness_.anyinit_procdff_123 = 8'b00000001;
    UUT._witness_.anyinit_procdff_128 = 8'b00000000;
    UUT._witness_.anyinit_procdff_133 = 8'b00000000;
    UUT._witness_.anyinit_procdff_73 = 1'b1;
    UUT._witness_.anyinit_procdff_78 = 1'b0;
    UUT._witness_.anyinit_procdff_83 = 1'b0;
    UUT._witness_.anyinit_procdff_88 = 17'b00000000000000001;
    UUT._witness_.anyinit_procdff_93 = 8'b00000001;
    UUT._witness_.anyinit_procdff_98 = 8'b00000000;

    // state 0
    PI_rst_n = 1'b1;
    PI_a = 8'b00000001;
    PI_b = 8'b00000001;
    PI_c = 8'b00000001;
  end
  always @(posedge clock) begin
    // state 1
    if (cycle == 0) begin
      PI_rst_n <= 1'b1;
      PI_a <= 8'b00000001;
      PI_b <= 8'b00000001;
      PI_c <= 8'b00000001;
    end

    genclock <= cycle < 1;
    cycle <= cycle + 1;
  end
endmodule
