SRCS = $(shell find src -type f -name "*.rs" -print)
MAIN = src/main.rs

TARGET = rspt

RFLAGS = -O -W unused-qualifications -W unused-results
RFLAGS += -L deps/gl-rs/ -L deps/gl-rs/gl_common/
RFLAGS += -L deps/glfw-rs/ -L deps/semver/
RFLAGS += -L deps/libc/ -L deps/rand/ -L deps/log/ -L deps/bitflags/
RFLAGS += --extern rand=deps/rand/librand.rlib
RFLAGS += --cfg feature=\"duration\"
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
