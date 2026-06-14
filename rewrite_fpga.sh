#!/bin/bash
sed -i '' 's/module: &Module/registry: \&crate::ecs::Registry/g' src/emit/fpga_scaffold.rs
sed -i '' 's/module\.name/registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string())/g' src/emit/fpga_scaffold.rs
sed -i '' 's/for s in &module\.signals/for s in \&get_ports(registry)/g' src/emit/fpga_scaffold.rs
sed -i '' 's/let width = signal_width(&s\.ty\.signal_type());/let width = s.width;/g' src/emit/fpga_scaffold.rs
sed -i '' 's/let module = &result\.program\.module;/let registry = result.ecs_registry.as_ref().unwrap();/g' src/emit/fpga_scaffold.rs
sed -i '' 's/(module,/(registry,/g' src/emit/fpga_scaffold.rs
sed -i '' 's/(module)/(registry)/g' src/emit/fpga_scaffold.rs
