.PHONY=run
index.html : main.elm
	elm-make main.elm
run : index.html
	go run main.go
clean : 
	$(RM) index.html
