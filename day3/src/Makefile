build: clean tools
	go build

run: build
	./src

tools:
	go generate

clean:
	if [ -f src ]; then rm src; fi
