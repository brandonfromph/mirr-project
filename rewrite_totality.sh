#!/bin/bash
sed -i '' 's/module: \&Module/registry: \&crate::ecs::Registry/g' src/totality/mod.rs
sed -i '' 's/(module, target_spec)/(registry, target_spec)/g' src/totality/mod.rs
sed -i '' 's/(module)/(registry)/g' src/totality/mod.rs
