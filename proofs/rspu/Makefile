# Makefile for R-SPU ISA proofs.
# Requires: rocq compile (Rocq >= 9.0) or coqc (Coq <= 8.20)
#
# Usage:
#   make        - compile all proofs
#   make clean  - remove compiled artifacts

# Auto-detect: prefer 'rocq compile' (Rocq 9.0+), fall back to 'coqc'
COQC := $(shell command -v rocq >/dev/null 2>&1 && echo "rocq compile" || echo "coqc")
COQFLAGS := -R . MirrRspu

VFILES := Encoding.v TaggedWord.v
VOFILES := $(VFILES:.v=.vo)

all: $(VOFILES)

# No inter-file dependencies (both are independent)
Encoding.vo: Encoding.v
TaggedWord.vo: TaggedWord.v

%.vo: %.v
	$(COQC) $(COQFLAGS) $<

clean:
	rm -f $(VOFILES) $(VFILES:.v=.glob) $(VFILES:.v=.vok) $(VFILES:.v=.vos)

.PHONY: all clean
