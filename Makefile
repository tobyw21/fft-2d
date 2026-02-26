doc:
	ps2pdf doc/docs.ps

.PHONY: clean

clean:
	find doc/docs.pdf -maxdepth 1 -type f -exec rm '{}' \;
