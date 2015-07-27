CLASSES = Euler29.java 
CCLASSES = main.c
.SUFFIXES: .java .class

all : classes

.java.class : 
	    javac $*.java

classes : $(CLASSES:.java=.class)

vim : 
	    vim -p $(CLASSES)
clean : 
	    $(RM) *.class
	    $(RM) run
c : main.c
	gcc -Wall -g -std=c99 main.c -lm -o run
