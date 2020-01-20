CC := g++

build: clean
	mkdir build
	$(CC) -o build/main.o -c main.cpp
	$(CC) -o build/utils.o utils.h
	$(CC) -o build/orbit.o orbit.h
	$(CC) -o build/main build/orbit.o build/main.o build/utils.o
	chmod +x build/main

clean:
	if [ -r build ]; then rm -r build; fi

run: build
	./build/main
