.PHONY: build test clippy fmt clean doc examples bootstrap bootstrap-check

build:
	cargo build

test:
	cargo test

clippy:
	cargo clippy -- -D warnings

fmt:
	cargo fmt --all

clean:
	cargo clean

doc:
	cargo doc --open

examples:
	cargo build --examples

# ══════════════════════════════════════════════════════════════════════════════
# BOOTSTRAP TARGETS
# ══════════════════════════════════════════════════════════════════════════════

# Build self-hosted compiler and run parity tests
bootstrap:
	@echo "=== Stage 1: Build Rust reference compiler ==="
	cargo build --release --bin mirr-compile
	@echo "=== Stage 2: Run parity tests ==="
	cargo test bootstrap_parity --release -- --nocapture

# Check bootstrap parity without rebuilding
bootstrap-check:
	cargo test bootstrap_parity --release -- --nocapture
