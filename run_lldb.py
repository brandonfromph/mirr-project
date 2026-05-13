import sys
import subprocess
import os
import json

def run_lldb(binary_path, test_name):
    print(f"\n[{test_name}] Running LLDB on {binary_path}...")
    cmd = [
        "lldb",
        "--batch",
        "-o", "run",
        "-o", "thread backtrace all",
        "-o", "quit",
        binary_path
    ]
    
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=30)
    except subprocess.TimeoutExpired:
        print(" -> TIMEOUT! (Process might be waiting for stdin or deadlocked)")
        return
    
    if "exited with status = 0" in result.stdout:
        print(" -> Execution completed successfully (Status 0). No crashes detected.")
    elif "stop reason =" in result.stdout:
        print(" -> EXCEPTION OR CRASH DETECTED!")
        lines = result.stdout.split('\n')
        for i, line in enumerate(lines):
            if "stop reason" in line:
                for j in range(i, min(i+20, len(lines))):
                    print(lines[j])
                break
    else:
        print(" -> Unknown status or panic caught by rust runtime. LLDB didn't trap a segfault.")

if __name__ == "__main__":
    binaries = []
    with open("target/test_build.json", "r") as f:
        for line in f:
            if not line.strip(): continue
            try:
                data = json.loads(line)
                if data.get("reason") == "compiler-artifact":
                    if data.get("profile", {}).get("test", False) and data.get("executable"):
                        binaries.append((data["executable"], data.get("target", {}).get("name", "unknown")))
            except:
                pass
    
    print(f"Found {len(binaries)} test binaries.")
    # Run all of them!
    for bin_path, name in binaries:
        if os.path.exists(bin_path):
            run_lldb(bin_path, name)
