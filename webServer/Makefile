.PHONY: clean

FLAGS=-lpthread -g -O0
OBJS=server.o handler.o stack.o

all : server

stackTest : stackTest.c stack.o
	gcc $^ $(FLAGS) -fsanitize=address -O0 -o $@

server : $(OBJS)
	gcc $^ $(FLAGS) -o $@

server.o : server.c
	gcc $< -c $(FLAGS) -o $@

handler.o : handler.c
	gcc $< -c $(FLAGS) -o $@

stack.o : stack.c
	gcc $< -c $(FLAGS) -o $@

clean :
	$(RM) $(OBJS) server stackTest
