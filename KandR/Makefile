files=$(subst .c,.o,$(wildcard *.c))
.PHONY: $(files) clean

all : $(files)

$(files) : 
	gcc $(subst .o,.c,$@) -g -o $@ -Wall
clean :
	$(RM) *.o
