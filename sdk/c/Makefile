CC ?= gcc
CFLAGS ?= -Wall -Wextra -O2
INCLUDE = -I.
LDFLAGS = -lkcm

.PHONY: all clean example test

all: example

example: example.c kcm.h
	$(CC) $(CFLAGS) $(INCLUDE) -o kcm_example example.c $(LDFLAGS)

test: example
	./kcm_example

clean:
	rm -f kcm_example
