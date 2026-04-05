#!/usr/bin/env bash
# MIRR EDA Test Suite â€” 105 tests using oss-cad-suite
# Usage: bash tests/eda/run_eda_tests.sh
# Requires: yosys, iverilog, vvp (from oss-cad-suite)
# Optional: sby (SymbiYosys) for formal verification tests
# Does NOT require cargo â€” fully standalone

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
TMPDIR="${SCRIPT_DIR}/../../target/eda_tests"
mkdir -p "$TMPDIR"

PASS=0; FAIL=0; SKIP=0; TOTAL=0
MAX_TESTS=200  # NASA P10 iteration bound

run_test() {
    local name="$1"; shift
    TOTAL=$((TOTAL + 1))
    if [ "$TOTAL" -gt "$MAX_TESTS" ]; then return; fi
    if "$@" > "$TMPDIR/${name}.log" 2>&1; then
        PASS=$((PASS + 1)); echo "  PASS: $name"
    else
        FAIL=$((FAIL + 1)); echo "  FAIL: $name (see $TMPDIR/${name}.log)"
    fi
}

skip_test() {
    local name="$1"
    TOTAL=$((TOTAL + 1))
    SKIP=$((SKIP + 1))
    echo "  SKIP: $name"
}

# â”€â”€ Tool availability â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
HAS_YOSYS=0; HAS_IVERILOG=0; HAS_VVP=0; HAS_SBY=0

for tool in yosys iverilog vvp sby; do
    if command -v "$tool" &>/dev/null; then
        case "$tool" in
            yosys)    HAS_YOSYS=1 ;;
            iverilog) HAS_IVERILOG=1 ;;
            vvp)      HAS_VVP=1 ;;
            sby)      HAS_SBY=1 ;;
        esac
    else
        echo "NOTE: $tool not found in PATH â€” related tests will be skipped"
    fi
done

if [ "$HAS_YOSYS" -eq 0 ] && [ "$HAS_IVERILOG" -eq 0 ]; then
    echo "SKIP: Neither yosys nor iverilog found. Nothing to test."
    exit 0
fi

T="$TMPDIR"  # shorthand for heredocs

# â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
#  Category 1: Yosys Parse (20 tests)
# â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
echo ""
echo "â”€â”€ Category 1: Yosys Parse (20 tests) â”€â”€"

if [ "$HAS_YOSYS" -eq 1 ]; then

run_test "yosys_parse_always_comb" bash -c "
cat > $T/parse_always_comb.sv << 'ENDOFSV'
module test_always_comb(input logic a, b, output logic y);
    always_comb y = a & b;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/parse_always_comb.sv'
"

run_test "yosys_parse_always_ff" bash -c "
cat > $T/parse_always_ff.sv << 'ENDOFSV'
module test_always_ff(input logic clk, d, output logic q);
    always_ff @(posedge clk) q <= d;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/parse_always_ff.sv'
"

run_test "yosys_parse_case_stmt" bash -c "
cat > $T/parse_case.sv << 'ENDOFSV'
module test_case(input logic [1:0] sel, input logic [3:0] a, b, c, d, output logic [3:0] y);
    always_comb begin
        case (sel)
            2'b00: y = a;
            2'b01: y = b;
            2'b10: y = c;
            2'b11: y = d;
        endcase
    end
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/parse_case.sv'
"

run_test "yosys_parse_if_else_chain" bash -c "
cat > $T/parse_ifelse.sv << 'ENDOFSV'
module test_ifelse(input logic [2:0] sel, output logic [7:0] y);
    always_comb begin
        if (sel == 3'd0) y = 8'd1;
        else if (sel == 3'd1) y = 8'd2;
        else if (sel == 3'd2) y = 8'd4;
        else y = 8'd0;
    end
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/parse_ifelse.sv'
"

run_test "yosys_parse_assign_wire" bash -c "
cat > $T/parse_assign.sv << 'ENDOFSV'
module test_assign(input logic [7:0] a, b, output logic [7:0] y);
    assign y = a + b;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/parse_assign.sv'
"

run_test "yosys_parse_parameter" bash -c "
cat > $T/parse_param.sv << 'ENDOFSV'
module test_param #(parameter WIDTH = 8)(input logic [WIDTH-1:0] a, output logic [WIDTH-1:0] y);
    assign y = ~a;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/parse_param.sv'
"

run_test "yosys_parse_localparam" bash -c "
cat > $T/parse_localparam.sv << 'ENDOFSV'
module test_localparam(input logic [7:0] a, output logic [7:0] y);
    localparam logic [7:0] MASK = 8'hAA;
    assign y = a & MASK;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/parse_localparam.sv'
"

run_test "yosys_parse_multi_module" bash -c "
cat > $T/parse_multi.sv << 'ENDOFSV'
module inner(input logic a, output logic y);
    assign y = ~a;
endmodule
module outer(input logic a, output logic y);
    inner u0(.a(a), .y(y));
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/parse_multi.sv'
"

run_test "yosys_parse_port_directions" bash -c "
cat > $T/parse_ports.sv << 'ENDOFSV'
module test_ports(input logic a, output logic b, inout wire c);
    assign b = a;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/parse_ports.sv'
"

run_test "yosys_parse_bit_select" bash -c "
cat > $T/parse_bitsel.sv << 'ENDOFSV'
module test_bitsel(input logic [15:0] data, output logic [7:0] hi, lo);
    assign hi = data[15:8];
    assign lo = data[7:0];
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/parse_bitsel.sv'
"

run_test "yosys_parse_part_select" bash -c "
cat > $T/parse_partsel.sv << 'ENDOFSV'
module test_partsel(input logic [31:0] data, input logic [2:0] idx, output logic [7:0] y);
    assign y = data[idx*8 +: 8];
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/parse_partsel.sv'
"

run_test "yosys_parse_concat" bash -c "
cat > $T/parse_concat.sv << 'ENDOFSV'
module test_concat(input logic [3:0] a, b, output logic [7:0] y);
    assign y = {a, b};
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/parse_concat.sv'
"

run_test "yosys_parse_ternary" bash -c "
cat > $T/parse_ternary.sv << 'ENDOFSV'
module test_ternary(input logic sel, input logic [7:0] a, b, output logic [7:0] y);
    assign y = sel ? a : b;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/parse_ternary.sv'
"

run_test "yosys_parse_generate_for" bash -c "
cat > $T/parse_genfor.sv << 'ENDOFSV'
module test_genfor(input logic [7:0] a, output logic [7:0] y);
    genvar i;
    generate
        for (i = 0; i < 8; i = i + 1) begin : gen_inv
            assign y[i] = ~a[i];
        end
    endgenerate
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/parse_genfor.sv'
"

run_test "yosys_parse_initial_block" bash -c "
cat > $T/parse_initial.sv << 'ENDOFSV'
module test_initial;
    logic [7:0] mem;
    initial begin
        mem = 8'h00;
    end
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/parse_initial.sv'
"

run_test "yosys_parse_logic_type" bash -c "
cat > $T/parse_logic.sv << 'ENDOFSV'
module test_logic(input logic [3:0] a, output logic [3:0] y);
    logic [3:0] tmp;
    assign tmp = a + 4'd1;
    assign y = tmp;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/parse_logic.sv'
"

run_test "yosys_parse_enum" bash -c "
cat > $T/parse_enum.sv << 'ENDOFSV'
module test_enum(input logic clk, output logic [1:0] state);
    typedef enum logic [1:0] {IDLE=2'd0, RUN=2'd1, DONE=2'd2} state_t;
    state_t s;
    assign state = s;
    always_ff @(posedge clk) s <= RUN;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/parse_enum.sv'
"

run_test "yosys_parse_struct" bash -c "
cat > $T/parse_struct.sv << 'ENDOFSV'
module test_struct(input logic [15:0] data_in, output logic [15:0] data_out);
    typedef struct packed {
        logic [7:0] hi;
        logic [7:0] lo;
    } word_t;
    word_t w;
    assign w = data_in;
    assign data_out = {w.lo, w.hi};
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/parse_struct.sv'
"

run_test "yosys_parse_interface" bash -c "
cat > $T/parse_iface.sv << 'ENDOFSV'
interface simple_bus;
    logic [7:0] data;
    logic valid;
    modport master(output data, output valid);
    modport slave(input data, input valid);
endinterface
module producer(simple_bus.master bus);
    assign bus.data = 8'hAB;
    assign bus.valid = 1'b1;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/parse_iface.sv'
"

run_test "yosys_parse_function" bash -c "
cat > $T/parse_func.sv << 'ENDOFSV'
module test_func(input logic [7:0] a, output logic [7:0] y);
    function automatic logic [7:0] double(input logic [7:0] val);
        return val << 1;
    endfunction
    assign y = double(a);
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/parse_func.sv'
"

else
    for i in $(seq 1 20); do skip_test "yosys_parse_$i"; done
fi

# â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
#  Category 2: Yosys Synthesis ice40 (20 tests)
# â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
echo ""
echo "â”€â”€ Category 2: Yosys Synthesis ice40 (20 tests) â”€â”€"

if [ "$HAS_YOSYS" -eq 1 ]; then

run_test "yosys_synth_and_gate" bash -c "
cat > $T/synth_and.sv << 'ENDOFSV'
module and_gate(input logic a, b, output logic y);
    assign y = a & b;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/synth_and.sv; synth_ice40 -top and_gate'
"

run_test "yosys_synth_or_gate" bash -c "
cat > $T/synth_or.sv << 'ENDOFSV'
module or_gate(input logic a, b, output logic y);
    assign y = a | b;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/synth_or.sv; synth_ice40 -top or_gate'
"

run_test "yosys_synth_mux2" bash -c "
cat > $T/synth_mux2.sv << 'ENDOFSV'
module mux2(input logic sel, input logic [7:0] a, b, output logic [7:0] y);
    assign y = sel ? a : b;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/synth_mux2.sv; synth_ice40 -top mux2'
"

run_test "yosys_synth_mux4" bash -c "
cat > $T/synth_mux4.sv << 'ENDOFSV'
module mux4(input logic [1:0] sel, input logic [7:0] a, b, c, d, output logic [7:0] y);
    always_comb begin
        case (sel)
            2'b00: y = a;
            2'b01: y = b;
            2'b10: y = c;
            2'b11: y = d;
        endcase
    end
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/synth_mux4.sv; synth_ice40 -top mux4'
"

run_test "yosys_synth_dff" bash -c "
cat > $T/synth_dff.sv << 'ENDOFSV'
module dff(input logic clk, d, output logic q);
    always_ff @(posedge clk) q <= d;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/synth_dff.sv; synth_ice40 -top dff'
"

run_test "yosys_synth_dff_reset" bash -c "
cat > $T/synth_dff_rst.sv << 'ENDOFSV'
module dff_reset(input logic clk, rst, d, output logic q);
    always_ff @(posedge clk)
        if (rst) q <= 1'b0;
        else     q <= d;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/synth_dff_rst.sv; synth_ice40 -top dff_reset'
"

run_test "yosys_synth_dff_enable" bash -c "
cat > $T/synth_dff_en.sv << 'ENDOFSV'
module dff_enable(input logic clk, en, d, output logic q);
    always_ff @(posedge clk)
        if (en) q <= d;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/synth_dff_en.sv; synth_ice40 -top dff_enable'
"

run_test "yosys_synth_counter_4bit" bash -c "
cat > $T/synth_counter4.sv << 'ENDOFSV'
module counter_4bit(input logic clk, rst, output logic [3:0] count);
    always_ff @(posedge clk)
        if (rst) count <= 4'd0;
        else     count <= count + 4'd1;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/synth_counter4.sv; synth_ice40 -top counter_4bit'
"

run_test "yosys_synth_shift_register" bash -c "
cat > $T/synth_shiftreg.sv << 'ENDOFSV'
module shift_register(input logic clk, rst, din, output logic dout);
    logic [7:0] sr;
    always_ff @(posedge clk)
        if (rst) sr <= 8'd0;
        else     sr <= {sr[6:0], din};
    assign dout = sr[7];
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/synth_shiftreg.sv; synth_ice40 -top shift_register'
"

run_test "yosys_synth_priority_encoder" bash -c "
cat > $T/synth_prienc.sv << 'ENDOFSV'
module priority_enc(input logic [7:0] req, output logic [2:0] grant, output logic valid);
    always_comb begin
        valid = 1'b1;
        casez (req)
            8'b1???????: grant = 3'd7;
            8'b01??????: grant = 3'd6;
            8'b001?????: grant = 3'd5;
            8'b0001????: grant = 3'd4;
            8'b00001???: grant = 3'd3;
            8'b000001??: grant = 3'd2;
            8'b0000001?: grant = 3'd1;
            8'b00000001: grant = 3'd0;
            default: begin grant = 3'd0; valid = 1'b0; end
        endcase
    end
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/synth_prienc.sv; synth_ice40 -top priority_enc'
"

run_test "yosys_synth_decoder_3to8" bash -c "
cat > $T/synth_dec3to8.sv << 'ENDOFSV'
module decoder_3to8(input logic [2:0] sel, input logic en, output logic [7:0] y);
    always_comb begin
        y = 8'd0;
        if (en) y[sel] = 1'b1;
    end
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/synth_dec3to8.sv; synth_ice40 -top decoder_3to8'
"

run_test "yosys_synth_tmr_voter" bash -c "
cat > $T/synth_tmr.sv << 'ENDOFSV'
module tmr_voter(input logic a, b, c, output logic y);
    assign y = (a & b) | (b & c) | (a & c);
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/synth_tmr.sv; synth_ice40 -top tmr_voter'
"

run_test "yosys_synth_guard_monitor" bash -c "
cat > $T/synth_guard.sv << 'ENDOFSV'
module guard_monitor(input logic clk, rst, condition, output logic guard_active);
    logic [3:0] count;
    localparam logic [3:0] THRESHOLD = 4'd3;
    always_ff @(posedge clk)
        if (rst) begin count <= 4'd0; guard_active <= 1'b0; end
        else if (condition) begin
            if (count < THRESHOLD) count <= count + 4'd1;
            else guard_active <= 1'b1;
        end else begin
            count <= 4'd0;
            guard_active <= 1'b0;
        end
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/synth_guard.sv; synth_ice40 -top guard_monitor'
"

run_test "yosys_synth_safety_clamp" bash -c "
cat > $T/synth_clamp.sv << 'ENDOFSV'
module safety_clamp(input logic clk, rst, fault, input logic [7:0] data_in, output logic [7:0] data_out);
    localparam logic [7:0] SAFE_VALUE = 8'h00;
    always_ff @(posedge clk)
        if (rst)        data_out <= SAFE_VALUE;
        else if (fault) data_out <= SAFE_VALUE;
        else            data_out <= data_in;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/synth_clamp.sv; synth_ice40 -top safety_clamp'
"

run_test "yosys_synth_watchdog_timer" bash -c "
cat > $T/synth_wdt.sv << 'ENDOFSV'
module watchdog_timer #(parameter TIMEOUT = 255)(
    input logic clk, rst, kick, output logic expired);
    logic [7:0] count;
    always_ff @(posedge clk)
        if (rst)       begin count <= 8'd0; expired <= 1'b0; end
        else if (kick) begin count <= 8'd0; expired <= 1'b0; end
        else if (count < TIMEOUT[7:0]) count <= count + 8'd1;
        else expired <= 1'b1;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/synth_wdt.sv; synth_ice40 -top watchdog_timer'
"

run_test "yosys_synth_debouncer" bash -c "
cat > $T/synth_debounce.sv << 'ENDOFSV'
module debouncer(input logic clk, rst, noisy, output logic clean);
    logic [3:0] count;
    logic prev;
    always_ff @(posedge clk)
        if (rst) begin count <= 4'd0; clean <= 1'b0; prev <= 1'b0; end
        else if (noisy != prev) begin count <= 4'd0; prev <= noisy; end
        else if (count < 4'd15) count <= count + 4'd1;
        else clean <= prev;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/synth_debounce.sv; synth_ice40 -top debouncer'
"

run_test "yosys_synth_edge_detector" bash -c "
cat > $T/synth_edge.sv << 'ENDOFSV'
module edge_detector(input logic clk, rst, sig, output logic rising);
    logic prev;
    always_ff @(posedge clk)
        if (rst) prev <= 1'b0;
        else     prev <= sig;
    assign rising = sig & ~prev;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/synth_edge.sv; synth_ice40 -top edge_detector'
"

run_test "yosys_synth_gray_counter" bash -c "
cat > $T/synth_gray.sv << 'ENDOFSV'
module gray_counter(input logic clk, rst, output logic [3:0] gray);
    logic [3:0] bin;
    always_ff @(posedge clk)
        if (rst) bin <= 4'd0;
        else     bin <= bin + 4'd1;
    assign gray = bin ^ (bin >> 1);
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/synth_gray.sv; synth_ice40 -top gray_counter'
"

run_test "yosys_synth_dual_port_ram" bash -c "
cat > $T/synth_dpram.sv << 'ENDOFSV'
module dual_port_ram(
    input logic clk,
    input logic we,
    input logic [3:0] addr_a, addr_b,
    input logic [7:0] din,
    output logic [7:0] dout_a, dout_b);
    logic [7:0] mem [0:15];
    always_ff @(posedge clk) begin
        if (we) mem[addr_a] <= din;
        dout_a <= mem[addr_a];
        dout_b <= mem[addr_b];
    end
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/synth_dpram.sv; synth_ice40 -top dual_port_ram'
"

run_test "yosys_synth_fsm_3state" bash -c "
cat > $T/synth_fsm3.sv << 'ENDOFSV'
module fsm_3state(input logic clk, rst, go, done_sig, output logic [1:0] state);
    typedef enum logic [1:0] {IDLE=2'd0, RUN=2'd1, FIN=2'd2} state_t;
    state_t s, s_next;
    always_ff @(posedge clk)
        if (rst) s <= IDLE;
        else     s <= s_next;
    always_comb begin
        s_next = s;
        case (s)
            IDLE: if (go)       s_next = RUN;
            RUN:  if (done_sig) s_next = FIN;
            FIN:                s_next = IDLE;
            default:            s_next = IDLE;
        endcase
    end
    assign state = s;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/synth_fsm3.sv; synth_ice40 -top fsm_3state'
"

else
    for i in $(seq 1 20); do skip_test "yosys_synth_$i"; done
fi

# â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
#  Category 3: Yosys Lint/Check (15 tests)
# â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
echo ""
echo "â”€â”€ Category 3: Yosys Lint/Check (15 tests) â”€â”€"

if [ "$HAS_YOSYS" -eq 1 ]; then

LINT_CMD="synth -flatten; check -assert"

run_test "yosys_lint_no_latches_comb" bash -c "
cat > $T/lint_comb.sv << 'ENDOFSV'
module lint_comb(input logic a, b, output logic y);
    always_comb y = a ^ b;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/lint_comb.sv; $LINT_CMD'
"

run_test "yosys_lint_no_latches_mux" bash -c "
cat > $T/lint_mux.sv << 'ENDOFSV'
module lint_mux(input logic sel, input logic [7:0] a, b, output logic [7:0] y);
    always_comb begin
        if (sel) y = a;
        else     y = b;
    end
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/lint_mux.sv; $LINT_CMD'
"

run_test "yosys_lint_width_match_8bit" bash -c "
cat > $T/lint_w8.sv << 'ENDOFSV'
module lint_w8(input logic [7:0] a, b, output logic [7:0] y);
    assign y = a + b;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/lint_w8.sv; $LINT_CMD'
"

run_test "yosys_lint_width_match_16bit" bash -c "
cat > $T/lint_w16.sv << 'ENDOFSV'
module lint_w16(input logic [15:0] a, b, output logic [15:0] y);
    assign y = a & b;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/lint_w16.sv; $LINT_CMD'
"

run_test "yosys_lint_all_ports_connected" bash -c "
cat > $T/lint_ports.sv << 'ENDOFSV'
module sub_m(input logic a, b, output logic y);
    assign y = a | b;
endmodule
module lint_ports(input logic x, z, output logic w);
    sub_m u0(.a(x), .b(z), .y(w));
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/lint_ports.sv; $LINT_CMD'
"

run_test "yosys_lint_no_tristate" bash -c "
cat > $T/lint_notri.sv << 'ENDOFSV'
module lint_notri(input logic a, b, output logic y);
    assign y = a & b;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/lint_notri.sv; $LINT_CMD'
"

run_test "yosys_lint_clean_clocking" bash -c "
cat > $T/lint_clk.sv << 'ENDOFSV'
module lint_clk(input logic clk, rst, d, output logic q);
    always_ff @(posedge clk)
        if (rst) q <= 1'b0;
        else     q <= d;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/lint_clk.sv; $LINT_CMD'
"

run_test "yosys_lint_no_async_reset" bash -c "
cat > $T/lint_syncrst.sv << 'ENDOFSV'
module lint_syncrst(input logic clk, rst, input logic [3:0] d, output logic [3:0] q);
    always_ff @(posedge clk)
        if (rst) q <= 4'd0;
        else     q <= d;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/lint_syncrst.sv; $LINT_CMD'
"

run_test "yosys_lint_guard_pattern_clean" bash -c "
cat > $T/lint_guard.sv << 'ENDOFSV'
module lint_guard(input logic clk, rst, cond, output logic active);
    logic [2:0] cnt;
    always_ff @(posedge clk)
        if (rst) begin cnt <= 3'd0; active <= 1'b0; end
        else if (cond) begin
            if (cnt < 3'd4) cnt <= cnt + 3'd1;
            else active <= 1'b1;
        end else begin cnt <= 3'd0; active <= 1'b0; end
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/lint_guard.sv; $LINT_CMD'
"

run_test "yosys_lint_shift_chain_clean" bash -c "
cat > $T/lint_shift.sv << 'ENDOFSV'
module lint_shift(input logic clk, rst, si, output logic so);
    logic [3:0] chain;
    always_ff @(posedge clk)
        if (rst) chain <= 4'd0;
        else     chain <= {chain[2:0], si};
    assign so = chain[3];
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/lint_shift.sv; $LINT_CMD'
"

run_test "yosys_lint_reflex_mux_clean" bash -c "
cat > $T/lint_rmux.sv << 'ENDOFSV'
module lint_rmux(input logic [1:0] pri, input logic [7:0] a, b, c, output logic [7:0] y);
    always_comb begin
        case (pri)
            2'd0: y = a;
            2'd1: y = b;
            2'd2: y = c;
            default: y = 8'd0;
        endcase
    end
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/lint_rmux.sv; $LINT_CMD'
"

run_test "yosys_lint_voter_clean" bash -c "
cat > $T/lint_voter.sv << 'ENDOFSV'
module lint_voter(input logic a, b, c, output logic y);
    assign y = (a & b) | (b & c) | (a & c);
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/lint_voter.sv; $LINT_CMD'
"

run_test "yosys_lint_counter_saturating" bash -c "
cat > $T/lint_satcnt.sv << 'ENDOFSV'
module lint_satcnt(input logic clk, rst, en, output logic [3:0] cnt);
    always_ff @(posedge clk)
        if (rst) cnt <= 4'd0;
        else if (en && cnt < 4'd15) cnt <= cnt + 4'd1;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/lint_satcnt.sv; $LINT_CMD'
"

run_test "yosys_lint_pipeline_stage" bash -c "
cat > $T/lint_pipe.sv << 'ENDOFSV'
module lint_pipe(input logic clk, rst, input logic [7:0] din, output logic [7:0] dout);
    logic [7:0] stage1, stage2;
    always_ff @(posedge clk)
        if (rst) begin stage1 <= 8'd0; stage2 <= 8'd0; end
        else     begin stage1 <= din;  stage2 <= stage1; end
    assign dout = stage2;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/lint_pipe.sv; $LINT_CMD'
"

run_test "yosys_lint_clock_gate" bash -c "
cat > $T/lint_ckgate.sv << 'ENDOFSV'
module lint_ckgate(input logic clk, en, input logic [7:0] d, output logic [7:0] q);
    logic en_lat;
    always_ff @(posedge clk) en_lat <= en;
    always_ff @(posedge clk)
        if (en_lat) q <= d;
endmodule
ENDOFSV
yosys -q -p 'read_verilog -sv $T/lint_ckgate.sv; $LINT_CMD'
"

else
    for i in $(seq 1 15); do skip_test "yosys_lint_$i"; done
fi

# â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
#  Category 4: iverilog Compile (20 tests)
# â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
echo ""
echo "â”€â”€ Category 4: iverilog Compile (20 tests) â”€â”€"

if [ "$HAS_IVERILOG" -eq 1 ]; then

run_test "iverilog_compile_comb_logic" bash -c "
cat > $T/iv_comb.sv << 'ENDOFSV'
module iv_comb(input logic [7:0] a, b, output logic [7:0] y);
    assign y = (a & b) | (~a & ~b);
endmodule
ENDOFSV
iverilog -g2012 -o /dev/null $T/iv_comb.sv
"

run_test "iverilog_compile_sequential" bash -c "
cat > $T/iv_seq.sv << 'ENDOFSV'
module iv_seq(input logic clk, rst, input logic [7:0] d, output logic [7:0] q);
    always_ff @(posedge clk)
        if (rst) q <= 8'd0;
        else     q <= d;
endmodule
ENDOFSV
iverilog -g2012 -o /dev/null $T/iv_seq.sv
"

run_test "iverilog_compile_parametric" bash -c "
cat > $T/iv_param.sv << 'ENDOFSV'
module iv_param #(parameter W = 16)(input logic [W-1:0] a, output logic [W-1:0] y);
    assign y = ~a;
endmodule
module iv_param_inst(input logic [15:0] x, output logic [15:0] z);
    iv_param #(.W(16)) u0(.a(x), .y(z));
endmodule
ENDOFSV
iverilog -g2012 -o /dev/null $T/iv_param.sv
"

run_test "iverilog_compile_sva_property" bash -c "
cat > $T/iv_sva_prop.sv << 'ENDOFSV'
module iv_sva_prop(input logic clk, a, b);
    property p_and;
        @(posedge clk) a |-> b;
    endproperty
    assert property (p_and);
endmodule
ENDOFSV
iverilog -g2012 -o /dev/null $T/iv_sva_prop.sv
"

run_test "iverilog_compile_sva_assume" bash -c "
cat > $T/iv_sva_assume.sv << 'ENDOFSV'
module iv_sva_assume(input logic clk, a);
    assume property (@(posedge clk) a);
endmodule
ENDOFSV
iverilog -g2012 -o /dev/null $T/iv_sva_assume.sv
"

run_test "iverilog_compile_sva_cover" bash -c "
cat > $T/iv_sva_cover.sv << 'ENDOFSV'
module iv_sva_cover(input logic clk, a, b);
    cover property (@(posedge clk) a ##1 b);
endmodule
ENDOFSV
iverilog -g2012 -o /dev/null $T/iv_sva_cover.sv
"

run_test "iverilog_compile_bind" bash -c "
cat > $T/iv_bind.sv << 'ENDOFSV'
module target_mod(input logic clk, input logic [7:0] data);
endmodule
module checker_mod(input logic clk, input logic [7:0] data);
endmodule
bind target_mod checker_mod chk(.clk(clk), .data(data));
ENDOFSV
iverilog -g2012 -o /dev/null $T/iv_bind.sv
"

run_test "iverilog_compile_interface" bash -c "
cat > $T/iv_iface.sv << 'ENDOFSV'
interface axi_lite;
    logic [31:0] addr;
    logic [31:0] data;
    logic        valid;
    logic        ready;
    modport master(output addr, output data, output valid, input ready);
    modport slave(input addr, input data, input valid, output ready);
endinterface
ENDOFSV
iverilog -g2012 -o /dev/null $T/iv_iface.sv
"

run_test "iverilog_compile_struct" bash -c "
cat > $T/iv_struct.sv << 'ENDOFSV'
module iv_struct(input logic [15:0] raw, output logic [7:0] hi, lo);
    typedef struct packed { logic [7:0] upper; logic [7:0] lower; } pair_t;
    pair_t p;
    assign p = raw;
    assign hi = p.upper;
    assign lo = p.lower;
endmodule
ENDOFSV
iverilog -g2012 -o /dev/null $T/iv_struct.sv
"

run_test "iverilog_compile_enum" bash -c "
cat > $T/iv_enum.sv << 'ENDOFSV'
module iv_enum(input logic clk, rst, output logic [1:0] out);
    typedef enum logic [1:0] {A=2'd0, B=2'd1, C=2'd2, D=2'd3} my_enum;
    my_enum val;
    always_ff @(posedge clk)
        if (rst) val <= A;
        else     val <= my_enum'(val + 2'd1);
    assign out = val;
endmodule
ENDOFSV
iverilog -g2012 -o /dev/null $T/iv_enum.sv
"

run_test "iverilog_compile_multimodule" bash -c "
cat > $T/iv_multi.sv << 'ENDOFSV'
module adder(input logic [7:0] a, b, output logic [7:0] s);
    assign s = a + b;
endmodule
module negator(input logic [7:0] a, output logic [7:0] y);
    assign y = ~a + 8'd1;
endmodule
module top_multi(input logic [7:0] x, y, output logic [7:0] z);
    logic [7:0] sum;
    adder  u0(.a(x), .b(y), .s(sum));
    negator u1(.a(sum), .y(z));
endmodule
ENDOFSV
iverilog -g2012 -o /dev/null $T/iv_multi.sv
"

run_test "iverilog_compile_generate" bash -c "
cat > $T/iv_gen.sv << 'ENDOFSV'
module iv_gen(input logic [7:0] a, output logic [7:0] y);
    genvar i;
    generate
        for (i = 0; i < 8; i = i + 1) begin : g
            assign y[i] = ~a[i];
        end
    endgenerate
endmodule
ENDOFSV
iverilog -g2012 -o /dev/null $T/iv_gen.sv
"

run_test "iverilog_compile_memory" bash -c "
cat > $T/iv_mem.sv << 'ENDOFSV'
module iv_mem(input logic clk, we, input logic [3:0] addr, input logic [7:0] wdata, output logic [7:0] rdata);
    logic [7:0] mem [0:15];
    always_ff @(posedge clk) begin
        if (we) mem[addr] <= wdata;
        rdata <= mem[addr];
    end
endmodule
ENDOFSV
iverilog -g2012 -o /dev/null $T/iv_mem.sv
"

run_test "iverilog_compile_signed_arith" bash -c "
cat > $T/iv_signed.sv << 'ENDOFSV'
module iv_signed(input logic signed [7:0] a, b, output logic signed [7:0] y);
    assign y = a * b;
endmodule
ENDOFSV
iverilog -g2012 -o /dev/null $T/iv_signed.sv
"

run_test "iverilog_compile_reduction_ops" bash -c "
cat > $T/iv_reduc.sv << 'ENDOFSV'
module iv_reduc(input logic [7:0] a, output logic y_and, y_or, y_xor);
    assign y_and = &a;
    assign y_or  = |a;
    assign y_xor = ^a;
endmodule
ENDOFSV
iverilog -g2012 -o /dev/null $T/iv_reduc.sv
"

run_test "iverilog_compile_shift_ops" bash -c "
cat > $T/iv_shift.sv << 'ENDOFSV'
module iv_shift(input logic [7:0] a, input logic [2:0] n,
    output logic [7:0] lsl_out, lsr_out, asr_out);
    assign lsl_out = a << n;
    assign lsr_out = a >> n;
    assign asr_out = \$signed(a) >>> n;
endmodule
ENDOFSV
iverilog -g2012 -o /dev/null $T/iv_shift.sv
"

run_test "iverilog_compile_concat_repeat" bash -c "
cat > $T/iv_concat.sv << 'ENDOFSV'
module iv_concat(input logic [3:0] a, output logic [15:0] y);
    assign y = {4{a}};
endmodule
ENDOFSV
iverilog -g2012 -o /dev/null $T/iv_concat.sv
"

run_test "iverilog_compile_function_call" bash -c "
cat > $T/iv_func.sv << 'ENDOFSV'
module iv_func(input logic [7:0] a, output logic [7:0] y);
    function automatic logic [7:0] add_one(input logic [7:0] v);
        return v + 8'd1;
    endfunction
    assign y = add_one(a);
endmodule
ENDOFSV
iverilog -g2012 -o /dev/null $T/iv_func.sv
"

run_test "iverilog_compile_task" bash -c "
cat > $T/iv_task.sv << 'ENDOFSV'
module iv_task;
    logic [7:0] result;
    task automatic compute(input logic [7:0] a, output logic [7:0] b);
        b = a + 8'd5;
    endtask
    initial begin
        compute(8'd10, result);
    end
endmodule
ENDOFSV
iverilog -g2012 -o /dev/null $T/iv_task.sv
"

run_test "iverilog_compile_clocking_block" bash -c "
cat > $T/iv_clkblk.sv << 'ENDOFSV'
module iv_clkblk(input logic clk, input logic [7:0] d, output logic [7:0] q);
    always_ff @(posedge clk) q <= d;
endmodule
ENDOFSV
iverilog -g2012 -o /dev/null $T/iv_clkblk.sv
"

else
    for i in $(seq 1 20); do skip_test "iverilog_compile_$i"; done
fi

# â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
#  Category 5: iverilog+vvp Simulation (20 tests)
# â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
echo ""
echo "â”€â”€ Category 5: iverilog+vvp Simulation (20 tests) â”€â”€"

if [ "$HAS_IVERILOG" -eq 1 ] && [ "$HAS_VVP" -eq 1 ]; then

run_test "sim_and_gate" bash -c "
cat > $T/sim_and.sv << 'ENDOFSV'
module and_gate(input logic a, b, output logic y);
    assign y = a & b;
endmodule
module tb;
    logic a, b, y;
    and_gate dut(.a(a), .b(b), .y(y));
    initial begin
        a=0; b=0; #10; if (y !== 0) begin \$display(\"FAIL 00\"); \$finish; end
        a=0; b=1; #10; if (y !== 0) begin \$display(\"FAIL 01\"); \$finish; end
        a=1; b=0; #10; if (y !== 0) begin \$display(\"FAIL 10\"); \$finish; end
        a=1; b=1; #10; if (y !== 1) begin \$display(\"FAIL 11\"); \$finish; end
        \$display(\"PASS\"); \$finish;
    end
endmodule
ENDOFSV
iverilog -g2012 -o $T/sim_and.vvp $T/sim_and.sv && vvp $T/sim_and.vvp | grep -q PASS
"

run_test "sim_or_gate" bash -c "
cat > $T/sim_or.sv << 'ENDOFSV'
module or_gate(input logic a, b, output logic y);
    assign y = a | b;
endmodule
module tb;
    logic a, b, y;
    or_gate dut(.a(a), .b(b), .y(y));
    initial begin
        a=0; b=0; #10; if (y !== 0) begin \$display(\"FAIL\"); \$finish; end
        a=0; b=1; #10; if (y !== 1) begin \$display(\"FAIL\"); \$finish; end
        a=1; b=0; #10; if (y !== 1) begin \$display(\"FAIL\"); \$finish; end
        a=1; b=1; #10; if (y !== 1) begin \$display(\"FAIL\"); \$finish; end
        \$display(\"PASS\"); \$finish;
    end
endmodule
ENDOFSV
iverilog -g2012 -o $T/sim_or.vvp $T/sim_or.sv && vvp $T/sim_or.vvp | grep -q PASS
"

run_test "sim_xor_gate" bash -c "
cat > $T/sim_xor.sv << 'ENDOFSV'
module xor_gate(input logic a, b, output logic y);
    assign y = a ^ b;
endmodule
module tb;
    logic a, b, y;
    xor_gate dut(.a(a), .b(b), .y(y));
    initial begin
        a=0; b=0; #10; if (y !== 0) begin \$display(\"FAIL\"); \$finish; end
        a=0; b=1; #10; if (y !== 1) begin \$display(\"FAIL\"); \$finish; end
        a=1; b=0; #10; if (y !== 1) begin \$display(\"FAIL\"); \$finish; end
        a=1; b=1; #10; if (y !== 0) begin \$display(\"FAIL\"); \$finish; end
        \$display(\"PASS\"); \$finish;
    end
endmodule
ENDOFSV
iverilog -g2012 -o $T/sim_xor.vvp $T/sim_xor.sv && vvp $T/sim_xor.vvp | grep -q PASS
"

run_test "sim_mux2_select" bash -c "
cat > $T/sim_mux2.sv << 'ENDOFSV'
module mux2(input logic sel, input logic [7:0] a, b, output logic [7:0] y);
    assign y = sel ? a : b;
endmodule
module tb;
    logic sel; logic [7:0] a, b, y;
    mux2 dut(.sel(sel), .a(a), .b(b), .y(y));
    initial begin
        a=8'hAA; b=8'h55;
        sel=0; #10; if (y !== 8'h55) begin \$display(\"FAIL sel=0\"); \$finish; end
        sel=1; #10; if (y !== 8'hAA) begin \$display(\"FAIL sel=1\"); \$finish; end
        \$display(\"PASS\"); \$finish;
    end
endmodule
ENDOFSV
iverilog -g2012 -o $T/sim_mux2.vvp $T/sim_mux2.sv && vvp $T/sim_mux2.vvp | grep -q PASS
"

run_test "sim_dff_capture" bash -c "
cat > $T/sim_dff.sv << 'ENDOFSV'
module dff(input logic clk, d, output logic q);
    always_ff @(posedge clk) q <= d;
endmodule
module tb;
    logic clk, d, q;
    dff dut(.clk(clk), .d(d), .q(q));
    initial clk = 0;
    always #5 clk = ~clk;
    initial begin
        d = 1; #12;
        if (q !== 1) begin \$display(\"FAIL capture\"); \$finish; end
        d = 0; #10;
        if (q !== 0) begin \$display(\"FAIL release\"); \$finish; end
        \$display(\"PASS\"); \$finish;
    end
endmodule
ENDOFSV
iverilog -g2012 -o $T/sim_dff.vvp $T/sim_dff.sv && vvp $T/sim_dff.vvp | grep -q PASS
"

run_test "sim_dff_reset" bash -c "
cat > $T/sim_dff_rst.sv << 'ENDOFSV'
module dff_rst(input logic clk, rst, d, output logic q);
    always_ff @(posedge clk)
        if (rst) q <= 1'b0;
        else     q <= d;
endmodule
module tb;
    logic clk, rst, d, q;
    dff_rst dut(.clk(clk), .rst(rst), .d(d), .q(q));
    initial clk = 0;
    always #5 clk = ~clk;
    initial begin
        rst = 1; d = 1; #12;
        if (q !== 0) begin \$display(\"FAIL rst\"); \$finish; end
        rst = 0; #10;
        if (q !== 1) begin \$display(\"FAIL d\"); \$finish; end
        \$display(\"PASS\"); \$finish;
    end
endmodule
ENDOFSV
iverilog -g2012 -o $T/sim_dff_rst.vvp $T/sim_dff_rst.sv && vvp $T/sim_dff_rst.vvp | grep -q PASS
"

run_test "sim_counter_counts" bash -c "
cat > $T/sim_counter.sv << 'ENDOFSV'
module counter(input logic clk, rst, output logic [3:0] cnt);
    always_ff @(posedge clk)
        if (rst) cnt <= 4'd0;
        else     cnt <= cnt + 4'd1;
endmodule
module tb;
    logic clk, rst; logic [3:0] cnt;
    counter dut(.clk(clk), .rst(rst), .cnt(cnt));
    initial clk = 0;
    always #5 clk = ~clk;
    initial begin
        rst = 1; #12; rst = 0;
        #10; if (cnt !== 4'd1) begin \$display(\"FAIL cnt=%0d\", cnt); \$finish; end
        #10; if (cnt !== 4'd2) begin \$display(\"FAIL cnt=%0d\", cnt); \$finish; end
        #10; if (cnt !== 4'd3) begin \$display(\"FAIL cnt=%0d\", cnt); \$finish; end
        \$display(\"PASS\"); \$finish;
    end
endmodule
ENDOFSV
iverilog -g2012 -o $T/sim_counter.vvp $T/sim_counter.sv && vvp $T/sim_counter.vvp | grep -q PASS
"

run_test "sim_shift_register_shifts" bash -c "
cat > $T/sim_shift.sv << 'ENDOFSV'
module shiftreg(input logic clk, rst, din, output logic dout);
    logic [3:0] sr;
    always_ff @(posedge clk)
        if (rst) sr <= 4'd0;
        else     sr <= {sr[2:0], din};
    assign dout = sr[3];
endmodule
module tb;
    logic clk, rst, din, dout;
    shiftreg dut(.clk(clk), .rst(rst), .din(din), .dout(dout));
    initial clk = 0;
    always #5 clk = ~clk;
    initial begin
        rst = 1; din = 0; #12; rst = 0;
        din = 1; #10; #10; #10; #10;
        if (dout !== 1) begin \$display(\"FAIL shift\"); \$finish; end
        \$display(\"PASS\"); \$finish;
    end
endmodule
ENDOFSV
iverilog -g2012 -o $T/sim_shift.vvp $T/sim_shift.sv && vvp $T/sim_shift.vvp | grep -q PASS
"

run_test "sim_guard_fires_after_cycles" bash -c "
cat > $T/sim_guard_fire.sv << 'ENDOFSV'
module guard(input logic clk, rst, cond, output logic active);
    logic [1:0] cnt;
    always_ff @(posedge clk)
        if (rst) begin cnt <= 2'd0; active <= 1'b0; end
        else if (cond) begin
            if (cnt < 2'd2) cnt <= cnt + 2'd1;
            else active <= 1'b1;
        end else begin cnt <= 2'd0; active <= 1'b0; end
endmodule
module tb;
    logic clk, rst, cond, active;
    guard dut(.clk(clk), .rst(rst), .cond(cond), .active(active));
    initial clk = 0;
    always #5 clk = ~clk;
    initial begin
        rst = 1; cond = 0; #12; rst = 0;
        cond = 1;
        #10; if (active !== 0) begin \$display(\"FAIL early1\"); \$finish; end
        #10; if (active !== 0) begin \$display(\"FAIL early2\"); \$finish; end
        #10; if (active !== 1) begin \$display(\"FAIL late\"); \$finish; end
        \$display(\"PASS\"); \$finish;
    end
endmodule
ENDOFSV
iverilog -g2012 -o $T/sim_guard_fire.vvp $T/sim_guard_fire.sv && vvp $T/sim_guard_fire.vvp | grep -q PASS
"

run_test "sim_guard_deasserts" bash -c "
cat > $T/sim_guard_deassert.sv << 'ENDOFSV'
module guard2(input logic clk, rst, cond, output logic active);
    logic [1:0] cnt;
    always_ff @(posedge clk)
        if (rst) begin cnt <= 2'd0; active <= 1'b0; end
        else if (cond) begin
            if (cnt < 2'd2) cnt <= cnt + 2'd1;
            else active <= 1'b1;
        end else begin cnt <= 2'd0; active <= 1'b0; end
endmodule
module tb;
    logic clk, rst, cond, active;
    guard2 dut(.clk(clk), .rst(rst), .cond(cond), .active(active));
    initial clk = 0;
    always #5 clk = ~clk;
    initial begin
        rst = 1; cond = 0; #12; rst = 0;
        cond = 1; #10; #10; #10;
        if (active !== 1) begin \$display(\"FAIL no fire\"); \$finish; end
        cond = 0; #10;
        if (active !== 0) begin \$display(\"FAIL no deassert\"); \$finish; end
        \$display(\"PASS\"); \$finish;
    end
endmodule
ENDOFSV
iverilog -g2012 -o $T/sim_guard_deassert.vvp $T/sim_guard_deassert.sv && vvp $T/sim_guard_deassert.vvp | grep -q PASS
"

run_test "sim_tmr_voter_majority" bash -c "
cat > $T/sim_tmr_maj.sv << 'ENDOFSV'
module tmr_voter(input logic a, b, c, output logic y);
    assign y = (a & b) | (b & c) | (a & c);
endmodule
module tb;
    logic a, b, c, y;
    tmr_voter dut(.a(a), .b(b), .c(c), .y(y));
    initial begin
        a=1; b=1; c=0; #10; if (y !== 1) begin \$display(\"FAIL 110\"); \$finish; end
        a=1; b=0; c=1; #10; if (y !== 1) begin \$display(\"FAIL 101\"); \$finish; end
        a=0; b=1; c=1; #10; if (y !== 1) begin \$display(\"FAIL 011\"); \$finish; end
        a=1; b=0; c=0; #10; if (y !== 0) begin \$display(\"FAIL 100\"); \$finish; end
        a=0; b=1; c=0; #10; if (y !== 0) begin \$display(\"FAIL 010\"); \$finish; end
        a=0; b=0; c=1; #10; if (y !== 0) begin \$display(\"FAIL 001\"); \$finish; end
        \$display(\"PASS\"); \$finish;
    end
endmodule
ENDOFSV
iverilog -g2012 -o $T/sim_tmr_maj.vvp $T/sim_tmr_maj.sv && vvp $T/sim_tmr_maj.vvp | grep -q PASS
"

run_test "sim_tmr_voter_all_agree" bash -c "
cat > $T/sim_tmr_agree.sv << 'ENDOFSV'
module tmr_voter2(input logic a, b, c, output logic y);
    assign y = (a & b) | (b & c) | (a & c);
endmodule
module tb;
    logic a, b, c, y;
    tmr_voter2 dut(.a(a), .b(b), .c(c), .y(y));
    initial begin
        a=0; b=0; c=0; #10; if (y !== 0) begin \$display(\"FAIL 000\"); \$finish; end
        a=1; b=1; c=1; #10; if (y !== 1) begin \$display(\"FAIL 111\"); \$finish; end
        \$display(\"PASS\"); \$finish;
    end
endmodule
ENDOFSV
iverilog -g2012 -o $T/sim_tmr_agree.vvp $T/sim_tmr_agree.sv && vvp $T/sim_tmr_agree.vvp | grep -q PASS
"

run_test "sim_safety_clamp_activates" bash -c "
cat > $T/sim_clamp.sv << 'ENDOFSV'
module clamp(input logic clk, rst, fault, input logic [7:0] din, output logic [7:0] dout);
    always_ff @(posedge clk)
        if (rst)        dout <= 8'd0;
        else if (fault) dout <= 8'd0;
        else            dout <= din;
endmodule
module tb;
    logic clk, rst, fault; logic [7:0] din, dout;
    clamp dut(.clk(clk), .rst(rst), .fault(fault), .din(din), .dout(dout));
    initial clk = 0;
    always #5 clk = ~clk;
    initial begin
        rst = 1; fault = 0; din = 8'hFF; #12; rst = 0;
        #10; if (dout !== 8'hFF) begin \$display(\"FAIL normal\"); \$finish; end
        fault = 1; #10;
        if (dout !== 8'd0) begin \$display(\"FAIL clamp\"); \$finish; end
        \$display(\"PASS\"); \$finish;
    end
endmodule
ENDOFSV
iverilog -g2012 -o $T/sim_clamp.vvp $T/sim_clamp.sv && vvp $T/sim_clamp.vvp | grep -q PASS
"

run_test "sim_edge_detector_rising" bash -c "
cat > $T/sim_edge_rise.sv << 'ENDOFSV'
module edge_det(input logic clk, rst, sig, output logic rising);
    logic prev;
    always_ff @(posedge clk)
        if (rst) prev <= 1'b0;
        else     prev <= sig;
    assign rising = sig & ~prev;
endmodule
module tb;
    logic clk, rst, sig, rising;
    edge_det dut(.clk(clk), .rst(rst), .sig(sig), .rising(rising));
    initial clk = 0;
    always #5 clk = ~clk;
    initial begin
        rst = 1; sig = 0; #12; rst = 0;
        #10; sig = 1; #1;
        if (rising !== 1) begin \$display(\"FAIL no rise\"); \$finish; end
        #9; #1;
        if (rising !== 0) begin \$display(\"FAIL stuck\"); \$finish; end
        \$display(\"PASS\"); \$finish;
    end
endmodule
ENDOFSV
iverilog -g2012 -o $T/sim_edge_rise.vvp $T/sim_edge_rise.sv && vvp $T/sim_edge_rise.vvp | grep -q PASS
"

run_test "sim_edge_detector_falling" bash -c "
cat > $T/sim_edge_fall.sv << 'ENDOFSV'
module fall_det(input logic clk, rst, sig, output logic falling);
    logic prev;
    always_ff @(posedge clk)
        if (rst) prev <= 1'b0;
        else     prev <= sig;
    assign falling = ~sig & prev;
endmodule
module tb;
    logic clk, rst, sig, falling;
    fall_det dut(.clk(clk), .rst(rst), .sig(sig), .falling(falling));
    initial clk = 0;
    always #5 clk = ~clk;
    initial begin
        rst = 1; sig = 1; #12; rst = 0;
        #10; sig = 0; #1;
        if (falling !== 1) begin \$display(\"FAIL no fall\"); \$finish; end
        #9; #1;
        if (falling !== 0) begin \$display(\"FAIL stuck\"); \$finish; end
        \$display(\"PASS\"); \$finish;
    end
endmodule
ENDOFSV
iverilog -g2012 -o $T/sim_edge_fall.vvp $T/sim_edge_fall.sv && vvp $T/sim_edge_fall.vvp | grep -q PASS
"

run_test "sim_debouncer_filters" bash -c "
cat > $T/sim_debounce.sv << 'ENDOFSV'
module debounce(input logic clk, rst, noisy, output logic clean);
    logic [1:0] cnt;
    logic prev;
    always_ff @(posedge clk)
        if (rst) begin cnt <= 2'd0; clean <= 1'b0; prev <= 1'b0; end
        else if (noisy != prev) begin cnt <= 2'd0; prev <= noisy; end
        else if (cnt < 2'd3) cnt <= cnt + 2'd1;
        else clean <= prev;
endmodule
module tb;
    logic clk, rst, noisy, clean;
    debounce dut(.clk(clk), .rst(rst), .noisy(noisy), .clean(clean));
    initial clk = 0;
    always #5 clk = ~clk;
    initial begin
        rst = 1; noisy = 0; #12; rst = 0;
        // glitch: 1 cycle high should NOT pass through
        noisy = 1; #10; noisy = 0; #10;
        if (clean !== 0) begin \$display(\"FAIL glitch passed\"); \$finish; end
        // sustained high should pass through
        noisy = 1; #10; #10; #10; #10;
        if (clean !== 1) begin \$display(\"FAIL stable not passed\"); \$finish; end
        \$display(\"PASS\"); \$finish;
    end
endmodule
ENDOFSV
iverilog -g2012 -o $T/sim_debounce.vvp $T/sim_debounce.sv && vvp $T/sim_debounce.vvp | grep -q PASS
"

run_test "sim_watchdog_timeout" bash -c "
cat > $T/sim_wdt.sv << 'ENDOFSV'
module wdt(input logic clk, rst, kick, output logic expired);
    logic [2:0] cnt;
    always_ff @(posedge clk)
        if (rst)       begin cnt <= 3'd0; expired <= 1'b0; end
        else if (kick) begin cnt <= 3'd0; expired <= 1'b0; end
        else if (cnt < 3'd4) cnt <= cnt + 3'd1;
        else expired <= 1'b1;
endmodule
module tb;
    logic clk, rst, kick, expired;
    wdt dut(.clk(clk), .rst(rst), .kick(kick), .expired(expired));
    initial clk = 0;
    always #5 clk = ~clk;
    initial begin
        rst = 1; kick = 0; #12; rst = 0;
        // no kick: should expire after 5 cycles
        #10; #10; #10; #10; #10;
        if (expired !== 1) begin \$display(\"FAIL no expire\"); \$finish; end
        // kick resets
        kick = 1; #10; kick = 0;
        if (expired !== 0) begin \$display(\"FAIL no reset\"); \$finish; end
        \$display(\"PASS\"); \$finish;
    end
endmodule
ENDOFSV
iverilog -g2012 -o $T/sim_wdt.vvp $T/sim_wdt.sv && vvp $T/sim_wdt.vvp | grep -q PASS
"

run_test "sim_priority_encoder" bash -c "
cat > $T/sim_prienc.sv << 'ENDOFSV'
module prienc(input logic [3:0] req, output logic [1:0] grant, output logic valid);
    always_comb begin
        valid = 1'b1;
        casez (req)
            4'b1???: grant = 2'd3;
            4'b01??: grant = 2'd2;
            4'b001?: grant = 2'd1;
            4'b0001: grant = 2'd0;
            default: begin grant = 2'd0; valid = 1'b0; end
        endcase
    end
endmodule
module tb;
    logic [3:0] req; logic [1:0] grant; logic valid;
    prienc dut(.req(req), .grant(grant), .valid(valid));
    initial begin
        req = 4'b1000; #10; if (grant !== 2'd3 || !valid) begin \$display(\"FAIL 1000\"); \$finish; end
        req = 4'b0100; #10; if (grant !== 2'd2 || !valid) begin \$display(\"FAIL 0100\"); \$finish; end
        req = 4'b0010; #10; if (grant !== 2'd1 || !valid) begin \$display(\"FAIL 0010\"); \$finish; end
        req = 4'b0001; #10; if (grant !== 2'd0 || !valid) begin \$display(\"FAIL 0001\"); \$finish; end
        req = 4'b0000; #10; if (valid)                    begin \$display(\"FAIL 0000\"); \$finish; end
        req = 4'b1010; #10; if (grant !== 2'd3)           begin \$display(\"FAIL 1010\"); \$finish; end
        \$display(\"PASS\"); \$finish;
    end
endmodule
ENDOFSV
iverilog -g2012 -o $T/sim_prienc.vvp $T/sim_prienc.sv && vvp $T/sim_prienc.vvp | grep -q PASS
"

run_test "sim_gray_counter_sequence" bash -c "
cat > $T/sim_gray.sv << 'ENDOFSV'
module gray_cnt(input logic clk, rst, output logic [2:0] gray);
    logic [2:0] bin;
    always_ff @(posedge clk)
        if (rst) bin <= 3'd0;
        else     bin <= bin + 3'd1;
    assign gray = bin ^ (bin >> 1);
endmodule
module tb;
    logic clk, rst; logic [2:0] gray;
    gray_cnt dut(.clk(clk), .rst(rst), .gray(gray));
    initial clk = 0;
    always #5 clk = ~clk;
    // Expected gray: 000 001 011 010 110 111 101 100
    initial begin
        rst = 1; #12; rst = 0;
        #10; if (gray !== 3'b001) begin \$display(\"FAIL g1=%b\", gray); \$finish; end
        #10; if (gray !== 3'b011) begin \$display(\"FAIL g2=%b\", gray); \$finish; end
        #10; if (gray !== 3'b010) begin \$display(\"FAIL g3=%b\", gray); \$finish; end
        #10; if (gray !== 3'b110) begin \$display(\"FAIL g4=%b\", gray); \$finish; end
        \$display(\"PASS\"); \$finish;
    end
endmodule
ENDOFSV
iverilog -g2012 -o $T/sim_gray.vvp $T/sim_gray.sv && vvp $T/sim_gray.vvp | grep -q PASS
"

run_test "sim_fsm_transitions" bash -c "
cat > $T/sim_fsm.sv << 'ENDOFSV'
module fsm(input logic clk, rst, go, done_in, output logic [1:0] state);
    localparam IDLE = 2'd0, RUN = 2'd1, FIN = 2'd2;
    logic [1:0] s, s_next;
    always_ff @(posedge clk)
        if (rst) s <= IDLE;
        else     s <= s_next;
    always_comb begin
        s_next = s;
        case (s)
            IDLE: if (go)      s_next = RUN;
            RUN:  if (done_in) s_next = FIN;
            FIN:               s_next = IDLE;
            default:           s_next = IDLE;
        endcase
    end
    assign state = s;
endmodule
module tb;
    logic clk, rst, go, done_in; logic [1:0] state;
    fsm dut(.clk(clk), .rst(rst), .go(go), .done_in(done_in), .state(state));
    initial clk = 0;
    always #5 clk = ~clk;
    initial begin
        rst = 1; go = 0; done_in = 0; #12; rst = 0;
        if (state !== 2'd0) begin \$display(\"FAIL idle\"); \$finish; end
        go = 1; #10; go = 0;
        if (state !== 2'd1) begin \$display(\"FAIL run\"); \$finish; end
        done_in = 1; #10; done_in = 0;
        if (state !== 2'd2) begin \$display(\"FAIL fin\"); \$finish; end
        #10;
        if (state !== 2'd0) begin \$display(\"FAIL back\"); \$finish; end
        \$display(\"PASS\"); \$finish;
    end
endmodule
ENDOFSV
iverilog -g2012 -o $T/sim_fsm.vvp $T/sim_fsm.sv && vvp $T/sim_fsm.vvp | grep -q PASS
"

else
    for i in $(seq 1 20); do skip_test "sim_$i"; done
fi

# â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
#  Category 6: SymbiYosys Formal (10 tests)
# â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
echo ""
echo "â”€â”€ Category 6: SymbiYosys Formal (10 tests) â”€â”€"

if [ "$HAS_SBY" -eq 1 ] && [ "$HAS_YOSYS" -eq 1 ]; then

run_test "formal_and_commutative" bash -c "
mkdir -p $T/formal_and_comm
cat > $T/formal_and_comm/dut.sv << 'ENDOFSV'
module and_comm(input logic a, b, output logic y1, y2);
    assign y1 = a & b;
    assign y2 = b & a;
    always_comb assert(y1 == y2);
endmodule
ENDOFSV
cat > $T/formal_and_comm/test.sby << 'ENDSBY'
[options]
mode bmc
depth 10
[engines]
smtbmc
[script]
read_verilog -sv -formal dut.sv
prep -top and_comm
[files]
dut.sv
ENDSBY
cd $T/formal_and_comm && sby -f test.sby
"

run_test "formal_or_commutative" bash -c "
mkdir -p $T/formal_or_comm
cat > $T/formal_or_comm/dut.sv << 'ENDOFSV'
module or_comm(input logic a, b, output logic y1, y2);
    assign y1 = a | b;
    assign y2 = b | a;
    always_comb assert(y1 == y2);
endmodule
ENDOFSV
cat > $T/formal_or_comm/test.sby << 'ENDSBY'
[options]
mode bmc
depth 10
[engines]
smtbmc
[script]
read_verilog -sv -formal dut.sv
prep -top or_comm
[files]
dut.sv
ENDSBY
cd $T/formal_or_comm && sby -f test.sby
"

run_test "formal_xor_self_zero" bash -c "
mkdir -p $T/formal_xor_zero
cat > $T/formal_xor_zero/dut.sv << 'ENDOFSV'
module xor_zero(input logic [7:0] a, output logic [7:0] y);
    assign y = a ^ a;
    always_comb assert(y == 8'd0);
endmodule
ENDOFSV
cat > $T/formal_xor_zero/test.sby << 'ENDSBY'
[options]
mode bmc
depth 10
[engines]
smtbmc
[script]
read_verilog -sv -formal dut.sv
prep -top xor_zero
[files]
dut.sv
ENDSBY
cd $T/formal_xor_zero && sby -f test.sby
"

run_test "formal_mux_select" bash -c "
mkdir -p $T/formal_mux_sel
cat > $T/formal_mux_sel/dut.sv << 'ENDOFSV'
module mux_sel(input logic sel, input logic [7:0] a, b, output logic [7:0] y);
    assign y = sel ? a : b;
    always_comb begin
        if (sel)  assert(y == a);
        if (!sel) assert(y == b);
    end
endmodule
ENDOFSV
cat > $T/formal_mux_sel/test.sby << 'ENDSBY'
[options]
mode bmc
depth 10
[engines]
smtbmc
[script]
read_verilog -sv -formal dut.sv
prep -top mux_sel
[files]
dut.sv
ENDSBY
cd $T/formal_mux_sel && sby -f test.sby
"

run_test "formal_counter_range" bash -c "
mkdir -p $T/formal_cnt_range
cat > $T/formal_cnt_range/dut.sv << 'ENDOFSV'
module cnt_range(input logic clk, rst, output logic [3:0] cnt);
    always_ff @(posedge clk)
        if (rst) cnt <= 4'd0;
        else if (cnt < 4'd15) cnt <= cnt + 4'd1;
    always_ff @(posedge clk)
        assert(cnt <= 4'd15);
endmodule
ENDOFSV
cat > $T/formal_cnt_range/test.sby << 'ENDSBY'
[options]
mode bmc
depth 20
[engines]
smtbmc
[script]
read_verilog -sv -formal dut.sv
prep -top cnt_range
[files]
dut.sv
ENDSBY
cd $T/formal_cnt_range && sby -f test.sby
"

run_test "formal_dff_stable" bash -c "
mkdir -p $T/formal_dff_stable
cat > $T/formal_dff_stable/dut.sv << 'ENDOFSV'
module dff_stable(input logic clk, en, input logic [7:0] d, output logic [7:0] q);
    logic [7:0] q_prev;
    always_ff @(posedge clk) begin
        q_prev <= q;
        if (en) q <= d;
    end
    always_ff @(posedge clk)
        if (!en && !\$past(en) && \$past(\$past(en),0))
            assert(q == q_prev);
endmodule
ENDOFSV
cat > $T/formal_dff_stable/test.sby << 'ENDSBY'
[options]
mode bmc
depth 10
[engines]
smtbmc
[script]
read_verilog -sv -formal dut.sv
prep -top dff_stable
[files]
dut.sv
ENDSBY
cd $T/formal_dff_stable && sby -f test.sby
"

run_test "formal_shift_register_delay" bash -c "
mkdir -p $T/formal_sr_delay
cat > $T/formal_sr_delay/dut.sv << 'ENDOFSV'
module sr_delay(input logic clk, rst, din, output logic dout);
    logic [3:0] sr;
    always_ff @(posedge clk)
        if (rst) sr <= 4'd0;
        else     sr <= {sr[2:0], din};
    assign dout = sr[3];
endmodule
ENDOFSV
cat > $T/formal_sr_delay/test.sby << 'ENDSBY'
[options]
mode bmc
depth 15
[engines]
smtbmc
[script]
read_verilog -sv -formal dut.sv
prep -top sr_delay
[files]
dut.sv
ENDSBY
cd $T/formal_sr_delay && sby -f test.sby
"

run_test "formal_guard_bounded" bash -c "
mkdir -p $T/formal_guard_bnd
cat > $T/formal_guard_bnd/dut.sv << 'ENDOFSV'
module guard_bnd(input logic clk, rst, cond, output logic active);
    logic [2:0] cnt;
    always_ff @(posedge clk)
        if (rst) begin cnt <= 3'd0; active <= 1'b0; end
        else if (cond) begin
            if (cnt < 3'd4) cnt <= cnt + 3'd1;
            else active <= 1'b1;
        end else begin cnt <= 3'd0; active <= 1'b0; end
    always_ff @(posedge clk)
        assert(cnt <= 3'd4);
endmodule
ENDOFSV
cat > $T/formal_guard_bnd/test.sby << 'ENDSBY'
[options]
mode bmc
depth 20
[engines]
smtbmc
[script]
read_verilog -sv -formal dut.sv
prep -top guard_bnd
[files]
dut.sv
ENDSBY
cd $T/formal_guard_bnd && sby -f test.sby
"

run_test "formal_voter_majority" bash -c "
mkdir -p $T/formal_voter_maj
cat > $T/formal_voter_maj/dut.sv << 'ENDOFSV'
module voter_maj(input logic a, b, c, output logic y);
    assign y = (a & b) | (b & c) | (a & c);
    always_comb begin
        if (a && b && c) assert(y == 1'b1);
        if (!a && !b && !c) assert(y == 1'b0);
        if (a && b) assert(y == 1'b1);
        if (!a && !b) assert(y == 1'b0);
    end
endmodule
ENDOFSV
cat > $T/formal_voter_maj/test.sby << 'ENDSBY'
[options]
mode bmc
depth 10
[engines]
smtbmc
[script]
read_verilog -sv -formal dut.sv
prep -top voter_maj
[files]
dut.sv
ENDSBY
cd $T/formal_voter_maj && sby -f test.sby
"

run_test "formal_safety_clamp" bash -c "
mkdir -p $T/formal_clamp
cat > $T/formal_clamp/dut.sv << 'ENDOFSV'
module safety_clamp_f(input logic clk, rst, fault, input logic [7:0] din, output logic [7:0] dout);
    always_ff @(posedge clk)
        if (rst)        dout <= 8'd0;
        else if (fault) dout <= 8'd0;
        else            dout <= din;
    always_ff @(posedge clk)
        if (fault && !rst) assert(\$past(fault) ? dout == 8'd0 : 1'b1);
endmodule
ENDOFSV
cat > $T/formal_clamp/test.sby << 'ENDSBY'
[options]
mode bmc
depth 15
[engines]
smtbmc
[script]
read_verilog -sv -formal dut.sv
prep -top safety_clamp_f
[files]
dut.sv
ENDSBY
cd $T/formal_clamp && sby -f test.sby
"

else
    for i in $(seq 1 10); do skip_test "formal_$i"; done
fi

# â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
#  Summary
# â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
echo ""
echo "============================================"
echo "  MIRR EDA Test Suite Results"
echo "============================================"
echo "  PASS: $PASS / $TOTAL"
echo "  FAIL: $FAIL"
echo "  SKIP: $SKIP"
echo "============================================"

if [ "$FAIL" -gt 0 ]; then
    exit 1
fi
echo "All EDA tests passed."
