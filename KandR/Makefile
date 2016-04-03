files=$(subst .c,.o,$(wildcard *.c))
.PHONY: $(files) clean

$(files) : 
	gcc $(subst .o,.c,$@) -g -o $@ -Wall
clean :
	$(RM) *.o
