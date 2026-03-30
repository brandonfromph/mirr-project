import os
import re

def fix_file(file_path):
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Replace validate_module(&p.module) with validate_module(&p.module, None)
    # Handle both &m and m, and nested calls.
    # We look for validate_module(arg) where arg does not contain a comma.
    
    # This regex looks for validate_module( followed by something that is not a comma, and then )
    # It might be too simple if there are nested calls, but let's see.
    # The pattern from grep was validate_module(&m).unwrap() etc.
    
    new_content = re.sub(r'validate_module\(([^,)]+)\)', r'validate_module(\1, None)', content)
    
    if new_content != content:
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(new_content)
        return True
    return False

test_dir = 'tests'
for root, dirs, files in os.walk(test_dir):
    for file in files:
        if file.endswith('.rs'):
            file_path = os.path.join(root, file)
            if fix_file(file_path):
                print(f"Fixed {file_path}")
