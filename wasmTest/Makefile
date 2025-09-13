CFLAGS=-Wall

build/index.html : template.html canvas.c canvas.h build
	emcc --shell-file template.html main.c canvas.c -o $@

canvas.c : 
	wget https://raw.githubusercontent.com/alextyner/wasm-canvas/refs/heads/master/src/canvas.c

canvas.h : 
	wget https://raw.githubusercontent.com/alextyner/wasm-canvas/refs/heads/master/src/canvas.h

build :
	mkdir $@
