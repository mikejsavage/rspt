SRCS = $(shell find src -type f -name "*.rs" -print)
MAIN = src/main.rs

TARGET = rspt

RFLAGS = -O -W unused-qualifications -W unused-typecasts -W unused-results
RFLAGS += -L deps/gl-rs/ -L deps/gl-rs/src/gl_common/
RFLAGS += -L deps/glfw-rs/ -L deps/semver/
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
