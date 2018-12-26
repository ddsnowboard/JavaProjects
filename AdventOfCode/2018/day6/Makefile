.PHONY: clean

all : main
main : main.o
	g++ --std=c++11 $? -o $@
main.o : main.cpp
	g++ --std=c++11 main.cpp -c -o main.o

clean : 
	$(RM) main.o main
