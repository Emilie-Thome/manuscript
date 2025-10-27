TEXC = latexmk
OUTDIR = ./out/
PUBLIC = ./public/
CFG = -cd ./src/main.tex -outdir=../$(OUTDIR) -Werror

.PHONY: clean cleanall display $(OUTDIR)./main.pdf
.DEFAULT_GOAL := $(PUBLIC)./main.pdf

$(PUBLIC)./main.pdf: $(OUTDIR)./main.pdf
	echo "copy PDF in $(PUBLIC)..."
	mkdir -p $(PUBLIC)
	cp $(OUTDIR)./main.pdf $(PUBLIC)./main.pdf
	echo "done!"

$(OUTDIR)./main.pdf: ./src/main.tex
	make cleanall
	echo "build in $(OUTDIR)..."
	$(TEXC) $(CFG) -pdf ./src/main.tex

display: $(PUBLIC)./main.pdf
	echo "display"
	xdg-open $(PUBLIC)./main.pdf

clean:
	echo "clean"
	rm -rf $(OUTDIR)

cleanall:
	echo "cleanall"
	rm -rf $(OUTDIR)
	rm -rf $(PUBLIC)

check:
	make clean
	make $(OUTDIR)./main.pdf
