CC=gcc
CFLAGS=-O2 -Wall -I/usr/local/include -I.

%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)

all: day1 day1_pt2 day2
clean:
	rm -f *.o *~ day1 day1_pt2 day2
day1: day1.o
	$(CC) -o day1 day1.o
day1_pt2: day1_pt2.o
	$(CC) -o day1_pt2 day1_pt2.o
day2: day2.o
	$(CC) -o day2 day2.o
