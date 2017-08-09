.PHONY: debug fast clean

debug : Euler59.c
	gcc -Wall -std=c99 -Wall -o main -g Euler59.c -lm -ggdb
fast : Euler59.c
	gcc -Wall -std=c99 -Wall -O3 -o main -g Euler59.c -lm -ggdb
clean : 
	$(RM) main 
