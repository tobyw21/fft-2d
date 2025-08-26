
.PHONY: clean

clean:
	ls | grep -v report.tex | tee | grep report | xargs rm