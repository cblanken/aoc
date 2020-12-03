CC=gcc
CFLAGS=-O2 -Wall -I/usr/local/include -I.

%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)

day1: day1.o
	$(CC) -o day1 day1.o

da1_pt2: day1_pt2.o
	$(CC) -o day1_pt2 day1_pt2.o



