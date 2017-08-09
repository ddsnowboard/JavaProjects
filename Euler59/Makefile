.PHONY: debug fast clean

debug : Euler59.c hashset.o
	gcc -Wall -o main -g $^
fast : Euler59.c hashset.o
	gcc -Wall -O3 -o main -g $^
clean : 
	$(RM) main *.o
