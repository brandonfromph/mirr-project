# Makefile for R-SPU ISA proofs.
# Requires: coqc (Rocq/Coq compiler)
#
# Usage:
#   make        - compile all proofs
#   make clean  - remove compiled artifacts

COQC := coqc
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
