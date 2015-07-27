CLASSES = Euler29.java 
.SUFFIXES: .java .class

all : classes

.java.class : 
	    javac $*.java

classes : $(CLASSES:.java=.class)

vim : 
	    vim -p $(CLASSES)
clean : 
	    $(RM) *.class

