# Makefile for MIRR PCC Proofs

.PHONY: all clean

VFILES := Verifier.v
VOFILES := $(VFILES:.v=.vo)
GLOBFILES := $(VFILES:.v=.glob)

all: $(VOFILES)

%.vo: %.v
	coqc $<

clean:
	rm -f $(VOFILES) $(GLOBFILES) .*.aux
