.PHONY: clean

Palindrome.class : Palindrome.java
	javac Palindrome.java

Combinator.class : Combinator.java
	javac Combinator.java
mergeSort.o : mergeSort.c
	gcc -Wall -g $< -o $@
reverseLinkedList.o : reverseLinkedList.c
	gcc -Wall -g $< -o $@

maxSubSum.o : maxSubSum.c
	gcc -Wall -g $< -o $@

treeToLinkedDList.o : treeToDLinkedList.c
	gcc -Wall -g $< -o $@

reverseDoublyLinkedList.o : reverseDoublyLinkedList.c
	gcc -Wall -g $< -o $@
clean : 
	$(RM) *.o *.class
