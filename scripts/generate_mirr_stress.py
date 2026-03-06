#!/usr/bin/env python3
"""(Deprecated) Python MIRR stress-test generator prototype.

This legacy script was provided as an initial prototype during Phase 1
language exploration.  It has since been superseded by the canonical Rust
implementation located at `src/bin/generate_mirr_stress.rs`.  The Python version
is retained for historical reference and may be removed in a future cleanup
cycle.

The generator emits MIRR modules parameterized by pattern type
(`mux_forest`, `temporal_chain`, `width_chain`) and size.  Use the Rust binary
for current workflows.
"""
import argparse


import random

def generate_mux_forest(size: int) -> str:
    """Produce a MIRR module that defines a chain of guards and reflexes
    whose control signals are logically related.  This isn't a perfect
    "mutate" hierarchy, but it exercises signal declarations, expressions,
    guards, and reflex assignments.  The size parameter controls how many
    internal stages are emitted.
    """
    lines: list[str] = []
    lines.append("module mux_forest {\n")
    # input signal
    lines.append("    signal inp: in bool;\n")
    # output signal to force at least one write
    lines.append("    signal out: out bool;\n")

    # declare intermediate internal signals
    for i in range(size):
        lines.append(f"    signal s{i}: internal bool;\n")

    lines.append("\n")

    # create a single guard that triggers each reflex (same guard name reused)
    lines.append("    guard trigger {\n")
    lines.append("        when inp\n")
    lines.append("        for 1 cycles;\n")
    lines.append("    }\n\n")

    # reflex chain: s0 = inp, s1 = s0 | inp, s2 = s1 | inp, ..., out = s(N-1)\n
    for i in range(size):
        expr = "inp" if i == 0 else f"s{i-1} || inp"
        lines.append(f"    reflex r{i} {{\n")
        lines.append("        on trigger {\n")
        lines.append(f"            s{i} = {expr};\n")
        lines.append("        }\n")
        lines.append("    }\n\n")

    # final reflex driving the output
    if size > 0:
        last = f"s{size-1}"
    else:
        last = "inp"
    lines.append("    reflex r_out {\n")
    lines.append("        on trigger {\n")
    lines.append(f"            out = {last};\n")
    lines.append("        }\n")
    lines.append("    }\n")

    lines.append("}\n")
    return ''.join(lines)


def generate_temporal_chain(size: int) -> str:
    """Emit a sequence of guards with increasing delay values and reflexes
    that assign a boolean path.  This exercises the "for N cycles" syntax.
    """
    lines: list[str] = []
    lines.append("module temporal_chain {\n")
    lines.append("    signal inp: in bool;\n")
    lines.append("    signal out: out bool;\n")
    for i in range(size):
        lines.append(f"    signal t{i}: internal bool;\n")
    lines.append("\n")
    for i in range(size):
        lines.append(f"    guard g{i} {{\n")
        lines.append(f"        when inp\n")
        lines.append(f"        for {i} cycles;\n")
        lines.append("    }\n\n")
        lines.append(f"    reflex r{i} {{\n")
        lines.append(f"        on g{i} {{\n")
        if i == 0:
            lines.append("            t0 = inp;\n")
        else:
            lines.append(f"            t{i} = t{i-1};\n")
        lines.append("        }\n")
        lines.append("    }\n\n")
    lines.append("    reflex r_out {\n")
    lines.append("        on g0 {\n")
    lines.append("            out = t0;\n")
    lines.append("        }\n")
    lines.append("    }\n")
    lines.append("}\n")
    return ''.join(lines)


def generate_width_chain(size: int) -> str:
    """Create an arithmetic chain using unsigned signals and additions.  The
    purpose is to exercise the expression parser with arithmetic and signal
    declarations with types (uN) though the type is not used by parser.
    """
    lines: list[str] = []
    lines.append("module width_chain {\n")
    lines.append("    signal x: in u8;\n")
    for i in range(size):
        lines.append(f"    signal w{i}: internal u16;\n")
    lines.append("    signal out: out u16;\n\n")
    for i in range(size):
        lines.append(f"    reflex a{i} {{\n")
        lines.append("        on x {\n")
        if i == 0:
            lines.append("            w0 = x;\n")
        else:
            lines.append(f"            w{i} = w{i-1} + x;\n")
        lines.append("        }\n")
        lines.append("    }\n\n")
    lines.append("    reflex a_out {\n")
    lines.append("        on x {\n")
    if size > 0:
        lines.append(f"            out = w{size-1};\n")
    else:
        lines.append("            out = x;\n")
    lines.append("        }\n")
    lines.append("    }\n")
    lines.append("}\n")
    return ''.join(lines)


def main() -> None:
    parser = argparse.ArgumentParser(description="Generate MIRR stress test code")
    parser.add_argument("--type", choices=["mux_forest","temporal_chain","width_chain"], required=True,
                        help="Which template to emit")
    parser.add_argument("--size", type=int, default=100,
                        help="Rough size parameter for the template")
    args = parser.parse_args()

    if args.type == "mux_forest":
        code = generate_mux_forest(args.size)
    elif args.type == "temporal_chain":
        code = generate_temporal_chain(args.size)
    elif args.type == "width_chain":
        code = generate_width_chain(args.size)
    else:
        code = ""

    print(code)


if __name__ == "__main__":
    main()
