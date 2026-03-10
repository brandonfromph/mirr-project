# Generated Makefile for R-SPU ISA proofs.
# Requires: coqc (Rocq/Coq compiler)
#
# Usage:
#   make        — compile all proofs
#   make clean  — remove compiled artifacts

COQPROJECT := _CoqProject

ifeq ($(wildcard $(COQPROJECT)),)
$(error _CoqProject not found. Run from proofs/rspu/ directory.)
endif

# Use coq_makefile if available; otherwise manual fallback.
COQMAKEFILE := $(shell command -v coq_makefile 2>/dev/null)

ifdef COQMAKEFILE
# Auto-generated rules via coq_makefile.
Makefile.coq: $(COQPROJECT)
	$(COQMAKEFILE) -f $(COQPROJECT) -o Makefile.coq

-include Makefile.coq

clean::
	$(if $(wildcard Makefile.coq),$(MAKE) -f Makefile.coq clean,)
	rm -f Makefile.coq Makefile.coq.conf
else
# Manual fallback: compile each .v file individually.
VFILES := $(shell grep '\.v$$' $(COQPROJECT))
VOFILES := $(VFILES:.v=.vo)

all: $(VOFILES)

%.vo: %.v
	coqc -R . MirrRspu $<

clean:
	rm -f $(VOFILES) $(VFILES:.v=.glob) $(VFILES:.v=.vok) $(VFILES:.v=.vos)
endif
