FILES = main.cs pyramid.cs
OUTFILES = $(subst cs,exe,$(FILES))

.PHONY: clean

all : $(FILES)
	mcs $(FILES)

clean :
	$(RM) $(OUTFILES)
