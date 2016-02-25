.PHONY: clean tags
all : main.c
	gcc -Wall -g -O3 main.c -o run
clean : 
	$(RM) run
tags : 
	zsh -c "ctags -R ./**/*.c(N) ./**/*.py(N) ./**/*.html(N) ./**/*.js(N) ./**/*.java(N)"
