.PHONY: clean

CC=clang++

all : main
main : main.cpp Graph.cpp
	$(CC) -g -Wall -Werror -Wextra --std=c++11 $^ -o $@
	#@$(CC) -O3 -Wall -Werror -Wextra --std=c++11 $^ -o $@
clean : 
	$(RM) main
