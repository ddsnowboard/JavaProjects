files=$(subst .c,.o,$(wildcard *.c))
PROCESSORS=4
.PHONY: $(files) clean

fast :
	$(MAKE) all -j $(PROCESSORS)
all : $(files)

ex24 :
	gcc 1-24.c hashTable.c -g -o $@.o -Wall
$(files) : 
	gcc $(subst .o,.c,$@) -g -o $@ -Wall
clean :
	$(RM) *.o
