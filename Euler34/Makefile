.PHONY: clean
all : main.c
	gcc -Wall -g main.c -o run -lm -lpthread
clean : 
	$(RM) run
old : mainOld.c
	gcc -Wall -g mainOld.c -fsanitize=address -o run -lm 
