.PHONY: all
all : Node.o Euler31.o

clean :
	rm *.o

Node.o : Node.cpp Node.h
	g++ -c -std=c++11 -Wall -o Node.o Node.cpp
Euler31.o : Euler31.cpp
	g++ -std=c++11 -Wall -o Euler31.o Euler31.cpp
