SRCS = $(shell find src -type f -name "*.rs" -print)
MAIN = src/main.rs

TARGET = rspt

RFLAGS = -O -W unnecessary-qualification -W unnecessary-typecast -W unused-result
RFLAGS += -L gl-rs/lib
RFLAGS += -L glfw-rs/lib
RFLAGS += -L rust-stb-image
RFLAGS += -C link-args="-lglfw"

.PHONY: all debug test clean

all: $(TARGET)

lto: RFLAGS += -Z lto
lto: all

debug: RFLAGS += -Z debug-info
debug: all

test: RFLAGS += --test
test: debug

$(TARGET): $(SRCS)
	rustc $(RFLAGS) -o $(TARGET) $(MAIN)

wc:
	@wc -l $(SRCS)

clean:
	rm -f $(TARGET)
