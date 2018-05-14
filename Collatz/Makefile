main : main.c
	gcc -Wall -g -o main main.c
mainThread : mainThread.c syncQueue.o
	gcc -Wall -g -pthread -o mainThread mainThread.c syncQueue.o
clean : 
	$(RM) main mainThread syncQueue.o
syncQueue.o : syncQueue.h syncQueue.c
	gcc -Wall -g -c syncQueue.c
