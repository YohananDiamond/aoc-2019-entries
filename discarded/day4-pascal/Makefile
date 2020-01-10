build: clean
	fpc main.pas

run: build
	./main

clean:
	if [ -f ./main ]; then rm ./main; fi
	if [ -f ./main.o ]; then rm ./main.o; fi
