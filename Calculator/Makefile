.PHONY:clean
CC=g++
CFLAGS=-Wall -Wextra -Werror -g --std=c++17

main : parser.o
	$(CC) $^ -o $@

parser.o : parser.cpp parser.h
	$(CC) -c $(CFLAGS) $< -o $@

clean : 
	$(RM) *.o main
