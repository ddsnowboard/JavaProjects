.PHONY: clean

FLAGS=-lpthread -g -O0
OBJS=server.o handler.o

all : server

server : $(OBJS)
	gcc $^ $(FLAGS) -o $@

server.o : server.c
	gcc $< -c $(FLAGS) -o $@

handler.o : handler.c
	gcc $< -c $(FLAGS) -o $@

clean :
	$(RM) $(OBJS) server
