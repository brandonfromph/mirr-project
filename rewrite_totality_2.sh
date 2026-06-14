#!/bin/bash
# Re-apply sed because first attempt was messy
sed -i '' 's/module: \&Module/registry: \&crate::ecs::Registry/g' src/totality/mod.rs
sed -i '' 's/check_resource_bounds(module/check_resource_bounds(registry/g' src/totality/mod.rs
sed -i '' 's/check_output_completeness(module/check_output_completeness(registry/g' src/totality/mod.rs
sed -i '' 's/check_guard_coverage(module/check_guard_coverage(registry/g' src/totality/mod.rs
sed -i '' 's/check_temporal_bound(module/check_temporal_bound(registry/g' src/totality/mod.rs
sed -i '' 's/check_dependency_acyclicity(module/check_dependency_acyclicity(registry/g' src/totality/mod.rs
sed -i '' 's/build_property_summary(module/build_property_summary(registry/g' src/totality/mod.rs
