files=$(subst .c,.o,$(wildcard *.c))
PROCESSORS=4
.PHONY: $(files) clean

fast :
	$(MAKE) all -j $(PROCESSORS)
all : $(files)

1-24.o:
	gcc 1-24.c hashTable.c -g -o $@ -Wall
# 1-24.o: 1-24.c hashTable.c
# 	# gcc 1-24.c hashTable.c -g -o $@ -Wall
# 	gcc -c $^
# 	gcc *.o -o main
# 	$(RM) $(subst .c,.o,$^)

$(files) : 
	gcc $(subst .o,.c,$@) -g -o $@ -Wall
clean :
	$(RM) *.o
