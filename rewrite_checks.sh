#!/bin/bash
sed -i '' 's/module: \&Module/registry: \&crate::ecs::Registry/g' src/totality/checks.rs
