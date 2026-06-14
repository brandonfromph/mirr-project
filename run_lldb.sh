#!/bin/bash
lldb /Users/brandonc.blay/.cargo_target_cache/debug/mirr << 'LLDB_EOF'
run compile rspu_chip/rspu_top.mirr --emit verilog &
sleep 2
process interrupt
thread backtrace all
quit
LLDB_EOF
