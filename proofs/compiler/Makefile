# Makefile for MIRR Verified Compiler Pass Proofs

.PHONY: all clean check-manifest

# Auto-detect: prefer 'rocq compile' (Rocq 9.0+), fall back to 'coqc'.
COQC := $(shell command -v rocq >/dev/null 2>&1 && echo "rocq compile" || (command -v coqc >/dev/null 2>&1 && echo "coqc"))

VFILES := ConstFold.v Simplify.v
VOFILES := $(VFILES:.v=.vo)
GLOBFILES := $(VFILES:.v=.glob)

ifeq ($(strip $(COQC)),)
all:
	@echo "SKIP: Neither rocq nor coqc found; proofs gate is informational in this environment."
else
all: $(VOFILES)

%.vo: %.v
	$(COQC) $<
endif

clean:
	rm -f $(VOFILES) $(GLOBFILES) .*.aux
