doc: FORCE
	ps2pdf doc/docs.ps doc/docs.pdf

FORCE: ;

.PHONY: clean

clean:
	find doc/docs.pdf -maxdepth 1 -type f -exec rm '{}' \;
