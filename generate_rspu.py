import os

def generate():
    os.makedirs("rspu_chip/core", exist_ok=True)
    os.makedirs("rspu_chip/memory", exist_ok=True)
    os.makedirs("rspu_chip/io", exist_ok=True)

    # Generate 16 Memory files (Leafs)
    for i in range(16):
        with open(f"rspu_chip/memory/mem_{i}.mirr", "w") as f:
            f.write(f"def mem_block_{i}(clk: signal in bool, addr: signal in u16, data: signal out u16) {{\n")
            f.write("    reflect {\n")
            f.write("        signal valid: internal bool;\n")
            f.write("        guard mem_ready {\n")
            f.write("            when ${clk} && ${addr} > 0\n")
            f.write("            for 1 cycles;\n")
            f.write("        }\n")
            f.write("        reflex set_data {\n")
            f.write("            on mem_ready {\n")
            f.write(f"                ${{data}} = ${{addr}} + {i};\n")
            f.write("                valid = true;\n")
            f.write("            }\n")
            f.write("        }\n")
            f.write("    }\n")
            f.write("}\n")
            
            # Add some extra bulk (stress test lines)
            for j in range(10):
                f.write(f"def mem_bulk_{i}_{j}(x: signal in bool, y: signal out bool) {{\n")
                f.write("    reflect { reflex pass { on ${x} { ${y} = true; } } }\n")
                f.write("}\n")
            
            # Every file needs a module
            f.write(f"\nmodule memory_subsystem_{i} {{}}\n")

    # Generate 16 ALU Cores that import Memory
    for i in range(16):
        with open(f"rspu_chip/core/alu_{i}.mirr", "w") as f:
            f.write(f"import \"memory/mem_{i}.mirr\" as mem;\n\n")
            f.write(f"def alu_core_{i}(clk: signal in bool, op1: signal in u16, op2: signal out u16) {{\n")
            f.write("    reflect {\n")
            f.write(f"        mem_block_{i}(${{clk}}, ${{op1}}, ${{op2}});\n")
            f.write("        guard alu_active {\n")
            f.write("            when ${clk}\n")
            f.write("            for 1 cycles;\n")
            f.write("        }\n")
            for j in range(16):
                f.write(f"        signal stage_{j}: internal bool;\n")
            f.write("    }\n")
            f.write("}\n")

            # Bulk
            for j in range(10):
                f.write(f"def alu_bulk_{i}_{j}(x: signal in bool, y: signal out bool) {{\n")
                f.write("    reflect { reflex pass { on ${x} { ${y} = true; } } }\n")
                f.write("}\n")
                
            f.write(f"\nmodule core_subsystem_{i} {{}}\n")

    # Generate Top Module
    with open("rspu_chip/rspu_top.mirr", "w") as f:
        for i in range(16):
            f.write(f"import \"core/alu_{i}.mirr\" as core{i};\n")
        
        f.write("\nmodule rspu_top {\n")
        f.write("    signal sys_clk: in bool;\n")
        for i in range(16):
            f.write(f"    signal addr_{i}: in u16;\n")
            f.write(f"    signal data_{i}: out u16;\n")
            f.write(f"    alu_core_{i}(sys_clk, addr_{i}, data_{i});\n")
            
        f.write("\n    property all_cores_active {\n")
        f.write("        always (sys_clk -> (addr_0 > 0));\n")
        f.write("    }\n")
        f.write("}\n")

    print("Generated Multi-File RSPU Chip Project in rspu_chip/")

if __name__ == "__main__":
    generate()