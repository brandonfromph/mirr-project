#!/usr/bin/env python3
# mirr_generator.py — FuseSoC generator for MIRR hardware rules.
import subprocess
import os
import sys
import yaml

def main():
    # Read FuseSoC configuration from the provided YAML file.
    with open(sys.argv[1], 'r') as f:
        config = yaml.safe_load(f)

    # Extract parameters.
    mirr_file = config.get('parameters', {}).get('file')
    if not mirr_file:
        print("Error: No .mirr file specified in generator parameters.")
        sys.exit(1)

    # Output file name (default to same base name as input).
    base_name = os.path.splitext(os.path.basename(mirr_file))[0]
    output_file = f"{base_name}.v"

    # Build the mirrc command.
    # We assume 'mirrc' is in the PATH or we use 'cargo run' as a fallback.
    cmd = ["mirrc", "--compile", mirr_file, "--emit", "verilog"]

    print(f"MIRR Generator: Compiling {mirr_file} to {output_file}...")
    
    try:
        # Run the compiler and capture output.
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        
        # mirrc outputs the Verilog to stdout by default (if not redirected).
        # We write it to the output file requested by FuseSoC.
        with open(output_file, 'w') as f:
            f.write(result.stdout)
            
    except subprocess.CalledProcessError as e:
        print(f"MIRR Compilation failed:\n{e.stderr}")
        sys.exit(1)
    except FileNotFoundError:
        print("Error: 'mirrc' binary not found. Please install it with 'cargo install --path .'")
        sys.exit(1)

    # Inform FuseSoC about the generated file.
    # FuseSoC expects the generator to write a .core file or a specific JSON/YAML structure.
    # For a simple generator, we can return the fileset.
    output = {
        'files': [{output_file: {'file_type': 'verilogSource'}}]
    }
    with open('mirr_generated.yaml', 'w') as f:
        yaml.dump(output, f)

if __name__ == "__main__":
    main()
