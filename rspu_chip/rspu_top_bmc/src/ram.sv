// R-SPU 2.0 Scratchpad RAM
// Scaled to 16384 words (14-bit addressing)
module ram (
    input  logic        clk,
    input  logic [13:0] addr,
    input  logic [63:0] din,
    output logic [63:0] dout
);
    logic [63:0] mem [0:16383];

    always_ff @(posedge clk) begin
        dout <= mem[addr];
        if (1'b1) begin // Write-always enabled for simplicity in this model
             mem[addr] <= din;
        end
    end

endmodule
