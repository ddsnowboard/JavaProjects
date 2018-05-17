.PHONY: runCollatz

runCollatz : Collatz.exe
	mono $<

Collatz.exe : Collatz.cs
	csc $<

main : main.c hashSet.o
	gcc -Wall -g -O3 -o main main.c hashSet.o

mainThread : mainThread.c syncQueue.o
	gcc -Wall -g -pthread -O3 -o mainThread mainThread.c syncQueue.o

clean : 
	$(RM) main mainThread syncQueue.o hashSet.o

syncQueue.o : syncQueue.h syncQueue.c
	gcc -Wall -Werror -g -O3 -c syncQueue.c

hashSet.o : hashSet.c hashSet.h 
	gcc -Wall -Werror -g -O3 -c $<
