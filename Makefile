doc:
	latexmk -pdf report/report.tex --outdir=report

.PHONY: clean

clean:
	find report -maxdepth 1 -type f | grep -v report.tex | tee | grep report | xargs rm
