#!/bin/bash
for file in tests/typecheck_tests.rs tests/signed_type_tests.rs tests/typecheck_c1_c3_refinement_tests.rs tests/composite_type_integration_tests.rs; do
  sed -i '' 's/use mirrc::typeck::typecheck_module;/use mirrc::typeck::typecheck_module; use mirrc::ecs::Registry;/g' "$file"
  
  # The tests use typecheck_module(&m). We'll replace it with a call to our own wrapper inside the test file if needed, or just replace `typecheck_module(&m)` inline.
  # Let's write a small wrapper in Rust and append it to the file.
done
