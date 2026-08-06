CC ?= gcc
CFLAGS ?= -Wall -Wextra -O2
INCLUDE = -Iinclude
LDFLAGS = -lkcm

BUILD_DIR = build

.PHONY: all clean test example

all: example test

example: $(BUILD_DIR)/examples/basic/basic

test: $(BUILD_DIR)/tests/test_kcm
	$(BUILD_DIR)/tests/test_kcm

$(BUILD_DIR)/examples/basic/basic: examples/basic/basic.c include/kcm.h
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) $(INCLUDE) -o $@ $< $(LDFLAGS)

$(BUILD_DIR)/tests/test_kcm: tests/test_kcm.c include/kcm.h
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) $(INCLUDE) -o $@ $< $(LDFLAGS)

clean:
	rm -rf $(BUILD_DIR)
