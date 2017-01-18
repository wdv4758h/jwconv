CC=clang

all: build run

build:
	cargo build
	$(CC) -Wall -O3 -I ../include/ -L ../../target/debug -l jwconv -o taiwan taiwan.c

run:
	LD_LIBRARY_PATH=../../target/debug ./taiwan
