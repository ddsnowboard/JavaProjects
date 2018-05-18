.PHONY: runCollatz

runCollatz : Collatz.exe
	mono $<

Collatz.exe : Collatz.cs
	csc $<

main : main.c .objs/hashSet.o
	gcc -Wall -g -O3 -o main $^

mainThread : mainThread.c .objs/syncQueue.o
	gcc -Wall -g -pthread -O3 -o mainThread $^

clean : 
	$(RM) main mainThread .objs/* test

.objs/syncQueue.o : syncQueue/syncQueue.c syncQueue/syncQueue.h
	gcc -Wall -Werror -g -O3 -o $@ -c $<

.objs/hashMap.o : hashMap/hashMap.c hashMap/hashMap.h 
	gcc -Wall -Werror -g -O3 -o $@ -c $<

.objs/upTree.o : upTree/upTree.c upTree/upTree.h .objs/hashMap.o
	gcc -Wall -Werror -g -O3 -o $@ -c $<

test : .objs/hashMapTest.o .objs/hashMap.o
	gcc -o $@ $^

.objs/hashMapTest.o : hashMap/hashMapTest.c
	gcc -Wall -Werror -g -O3 -o $@ -c $<
