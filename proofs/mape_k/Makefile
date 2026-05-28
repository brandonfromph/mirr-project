# Makefile for MIRR MAPE-K Proofs

.PHONY: all clean

VFILES := Equivalence.v
VOFILES := $(VFILES:.v=.vo)
GLOBFILES := $(VFILES:.v=.glob)

all: $(VOFILES)

%.vo: %.v
	coqc $<

clean:
	rm -f $(VOFILES) $(GLOBFILES) .*.aux
