# Makefile for MIRR language-theory proofs.
# Requires: rocq compile (Rocq >= 9.0) or coqc (Coq <= 8.20)
#
# Usage:
#   make        - compile all proofs
#   make clean  - remove compiled artifacts

# Auto-detect: prefer 'rocq compile' (Rocq 9.0+), fall back to 'coqc'.
# If neither exists, this gate is informational in local environments.
COQC := $(shell command -v rocq >/dev/null 2>&1 && echo "rocq compile" || (command -v coqc >/dev/null 2>&1 && echo "coqc"))
COQFLAGS := -R . MirrLanguage

VFILES := \
  Semantics.v \
  Determinism.v \
  DFA.v \
  Decidability.v \
  SubTuring.v \
  Compression.v

VOFILES := $(VFILES:.v=.vo)

ifeq ($(strip $(COQC)),)
all:
	@echo "SKIP: Neither rocq nor coqc found; language proofs gate is informational in this environment."
else
all: $(VOFILES)

%.vo: %.v
	$(COQC) $(COQFLAGS) $<
endif

check-manifest:
	@actual="$$(find . -type f -name '*.v' | sed 's#^\./##' | LC_ALL=C sort | tr '\n' '|')"; \
	expected="$$(printf '%s\n' $(VFILES) | LC_ALL=C sort | tr '\n' '|')"; \
	if [ "$$actual" != "$$expected" ]; then \
		echo "FAIL: proofs/language manifest drift detected"; \
		echo "expected: $$expected"; \
		echo "actual:   $$actual"; \
		exit 1; \
	fi

check-no-admitted:
	@if grep -n "Admitted\\." -- *.v >/dev/null; then \
		echo "FAIL: admitted proof found in proofs/language"; \
		grep -n "Admitted\\." -- *.v; \
		exit 1; \
	fi

clean:
	rm -f $(VOFILES) $(VFILES:.v=.glob) $(VFILES:.v=.vok) $(VFILES:.v=.vos)

.PHONY: all clean check-manifest check-no-admitted
