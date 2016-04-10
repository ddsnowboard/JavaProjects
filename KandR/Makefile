files=$(subst .c,.o,$(wildcard *.c))
PROCESSORS=4
.PHONY: $(files) clean

fast :
	$(MAKE) all -j $(PROCESSORS)
all : $(files)

$(files) : 
	gcc $(subst .o,.c,$@) -g -o $@ -Wall
clean :
	$(RM) *.o
