CC = rustc

.PHONY = all, test
all: 
	$(CC) -o main src/controler/trafficlight.rs src/main.rs src/controler.rs
	./main
test:
	$(CC) -o test test.rs
	./test