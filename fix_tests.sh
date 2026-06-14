#!/bin/bash
# Fix Registry::default()
grep -l "Registry::default()" tests/*.rs tests/**/*.rs | xargs sed -i '' 's/Registry::default()/Registry::new()/g'

# Fix run_totality_check calls in tests
grep -l "run_totality_check(" tests/*.rs tests/**/*.rs | xargs sed -i '' 's/run_totality_check(/run_totality_check_on_module(/g'
