.PHONY: clean

CC=clang++

all : main
main : main.cpp
	#$(CC) -g -Wall -Werror -Wextra --std=c++11 main.cpp -o main
	@$(CC) -O3 -Wall -Werror -Wextra --std=c++11 main.cpp -o main
clean : 
	$(RM) main
