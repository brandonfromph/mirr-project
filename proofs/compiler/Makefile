# Makefile for MIRR Verified Compiler Pass Proofs

.PHONY: all clean

VFILES := ConstFold.v Simplify.v
VOFILES := $(VFILES:.v=.vo)
GLOBFILES := $(VFILES:.v=.glob)

all: $(VOFILES)

%.vo: %.v
	coqc $<

clean:
	rm -f $(VOFILES) $(GLOBFILES) .*.aux
