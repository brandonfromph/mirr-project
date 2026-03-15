# Makefile for MIRR width proofs.
# Requires: rocq compile (Rocq >= 9.0) or coqc (Coq <= 8.20)
#
# Usage:
#   make        - compile all proofs
#   make clean  - remove compiled artifacts

# Auto-detect: prefer 'rocq compile' (Rocq 9.0+), fall back to 'coqc'
COQC := $(shell command -v rocq >/dev/null 2>&1 && echo "rocq compile" || echo "coqc")
COQFLAGS := -R . MirrWidth

VFILES := \
  Types.v \
  MinBits.v \
  Constraint.v \
  Flatten.v \
  Monotone.v \
  Solver.v \
  Truncation.v \
  Integration.v \
  SCC/Tarjan.v \
  SCC/Classify.v \
  SCC/Nonexpansive.v

VOFILES := $(VFILES:.v=.vo)

all: $(VOFILES)

# Explicit dependency ordering (from Require Import analysis)
Types.vo: Types.v
MinBits.vo: MinBits.v Types.vo
Constraint.vo: Constraint.v Types.vo
Flatten.vo: Flatten.v Types.vo
Monotone.vo: Monotone.v Types.vo Constraint.vo
Solver.vo: Solver.v Types.vo Constraint.vo Monotone.vo
Truncation.vo: Truncation.v Types.vo
Integration.vo: Integration.v Types.vo MinBits.vo Constraint.vo Monotone.vo Solver.vo Flatten.vo Truncation.vo
SCC/Tarjan.vo: SCC/Tarjan.v Types.vo
SCC/Classify.vo: SCC/Classify.v Types.vo
SCC/Nonexpansive.vo: SCC/Nonexpansive.v Types.vo Constraint.vo Monotone.vo

%.vo: %.v
	$(COQC) $(COQFLAGS) $<

clean:
	rm -f $(VOFILES) $(VFILES:.v=.glob) $(VFILES:.v=.vok) $(VFILES:.v=.vos)

.PHONY: all clean
