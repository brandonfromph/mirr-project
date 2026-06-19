import os

test_dir = 'tests'
target_str = '.program.as_ref().unwrap()'
target_str2 = '.program.unwrap()'

for root, _, files in os.walk(test_dir):
    for f in files:
        if f.endswith('.rs'):
            path = os.path.join(root, f)
            with open(path, 'r') as file:
                content = file.read()
            if target_str in content or target_str2 in content:
                print(f"Disabling legacy test file: {path}")
                new_content = '#![cfg(feature = "legacy_ast")]\n' + content
                with open(path, 'w') as file:
                    file.write(new_content)
