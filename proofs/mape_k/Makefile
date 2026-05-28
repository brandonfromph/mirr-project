# Makefile for MIRR MAPE-K Proofs

.PHONY: all clean

COQC := $(shell command -v rocq >/dev/null 2>&1 && echo "rocq compile" || (command -v coqc >/dev/null 2>&1 && echo "coqc"))

VFILES := Equivalence.v
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
