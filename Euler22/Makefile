.PHONY: clean
main : main.c
	gcc main.c -Wall -g -o main
clean :
	$(RM) main tags
tags :
	zsh -c "ctags -R ./fantasystocks/**/*.c(N) ./fantasystocks/**/*.py(N) ./fantasystocks/**/*.html(N) ./fantasystocks/**/*.js(N) ./fantasystocks/**/*.java(N)"
