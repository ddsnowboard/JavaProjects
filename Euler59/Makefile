.PHONY: debug fast clean

debug : Euler59.c hashset.o
	gcc -Wall -o main -g $^ -lm

fast : Euler59.c hashset.o
	gcc -Wall -O3 -o main -g $^ -lm

tester : hashset.o tester.c
	gcc -Wall -g -o $@ $^ -lm

hashset.o : hashset.c hashset.h
	gcc -g -c -Wall -std=c99 $< -lm

clean : 
	$(RM) main *.o tester
