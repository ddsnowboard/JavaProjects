.PHONY: clean tags
all : main.c
	gcc -Wall -g main.c -lm -o run
clean : 
	$(RM) run run2
tags : 
	zsh -c "ctags -R ./**/*.c(N) ./**/*.py(N) ./**/*.html(N) ./**/*.js(N) ./**/*.java(N)"
