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

# ══════════════════════════════════════════════════════════════════════════════
# CONTAINER DEV TARGETS
# ══════════════════════════════════════════════════════════════════════════════

.PHONY: dev-build dev-shell dev-test dev-proofs

dev-build:
	docker compose build dev

dev-shell:
	docker compose run --rm dev bash

dev-test:
	docker compose run --rm dev cargo nextest run --workspace --no-fail-fast --test-threads 12

dev-proofs:
	docker compose run --rm proofs make -C width && \
	docker compose run --rm proofs make -C language && \
	docker compose run --rm proofs make -C rspu && \
	docker compose run --rm proofs make -C compiler && \
	docker compose run --rm proofs make -C mape_k && \
	docker compose run --rm proofs make -C cert

