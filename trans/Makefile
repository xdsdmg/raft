build: main.tex
	xelatex main.tex
	# pdflatex main.tex
	bibtex main.aux   	
	xelatex main.tex
	xelatex main.tex
	# pdflatex main.tex
	# pdflatex main.tex

clean:
	rm -rf \
		*.aux \
		*.fdb_latexmk \
		*.log \
		*.out \
		*.fls \
		*.synctex.gz \
		*.bbl \
		*.bib \
		*.blg
