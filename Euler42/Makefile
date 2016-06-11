.PHONY: clean tags
all : main.c
	gcc -Wall -g main.c -o run
run2 : main2.c
	gcc -Wall -g main2.c -lm -o run2
clean : 
	$(RM) run run2
tags : 
	zsh -c "ctags -R ./**/*.c(N) ./**/*.py(N) ./**/*.html(N) ./**/*.js(N) ./**/*.java(N)"
