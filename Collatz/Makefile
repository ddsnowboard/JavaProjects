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
	$(RM) main mainThread *.o test

syncQueue.o : syncQueue.h syncQueue.c
	gcc -Wall -Werror -g -O3 -c syncQueue.c

hashMap.o : hashMap.c hashMap.h 
	gcc -Wall -Werror -g -O3 -c $<

upTree.o : upTree.c upTree.h hashMap.o
	gcc -Wall -Werror -g -O3 -c $<

test : hashMapTest.o hashMap.o
	gcc -o $@ $^

hashMapTest.o : hashMapTest.c
	gcc -Wall -Werror -g -O3 -c $<
