.PHONY: runCollatz

CCARGS=-Wall -Werror -g3 -pg

main : main.c .objs/upTree.o hashMap/hashMap.o
	gcc $(CCARGS) -o main $^

runCollatz : Collatz.exe
	mono $<

Collatz.exe : Collatz.cs
	csc $<

mainThread : mainThread.c .objs/syncQueue.o
	gcc -Wall -g3 -pthread -O3 -o mainThread $^

clean : 
	$(RM) main mainThread .objs/* test

.objs/hashMap.o : hashMap/hashMap.c hashMap/hashMap.h 
	gcc $(CCARGS) -o $@ -c $<

.objs/upTree.o : upTree/upTree.c upTree/upTree.h .objs/hashMap.o
	gcc $(CCARGS) -o $@ -c $<

test : .objs/hashMapTest.o .objs/hashMap.o
	gcc $(CCARGS) -o $@ $^

.objs/hashMapTest.o : hashMap/hashMapTest.c
	gcc $(CCARGS) -o $@ -c $<
