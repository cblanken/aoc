CC=gcc
CFLAGS=-O2 -Wall -I/usr/local/include -I.

%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)

all: day1 day1_pt2 day2 day2_pt2 day3 day3_pt2 \
	day4 day4_pt2 day5
clean:
	rm -f *.o *~ day1 day1_pt2 day2 day2_pt2 day3 \
		day3_pt2 day4 day4_pt2 day5
day1: day1.o
	$(CC) -o day1 day1.o
day1_pt2: day1_pt2.o 
	$(CC) -o day1_pt2 day1_pt2.o 
day2: day2.o 
	$(CC) -o day2 day2.o 
day2_pt2: day2_pt2.o 
	$(CC) -o day2_pt2 day2_pt2.o
day3: day3.o
	$(CC) -o day3 day3.o
day3_pt2: day3_pt2.o
	$(CC) -o day3_pt2 day3_pt2.o
day4: day4.o
	$(CC) -o day4 day4.o
day4_pt2: day4_pt2.o
	$(CC) -o day4_pt2 day4_pt2.o
day5: day5.o
	$(CC) -o day5 day5.o

