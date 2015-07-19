#! /bin/sh

mkdir -p deps
cd deps

function rustlang() {
	if [[ ! -d "$1" ]]; then
		git clone "https://github.com/rust-lang/$1"
		cd "$1"
		rustc --crate-name "$1" --crate-type=lib src/lib.rs
		cd ..
	fi
}

if [[ ! -d libc ]]; then
	mkdir libc
	cd libc
	wget https://raw.githubusercontent.com/rust-lang/rust/master/src/liblibc/lib.rs
	rustc --crate-name libc --crate-type=lib --cfg feature=\"cargo-build\" lib.rs
	cd ..
fi

rustlang bitflags
rustlang semver

# rand
if [[ ! -d rust ]]; then
	git clone https://github.com/rust-lang/rand \
		--depth 1 # rand repo includes 90% of rust history...
	cd rand
	rustc --crate-name rand --crate-type=lib src/lib.rs --extern libc=../libc/liblibc.rlib
	cd ..
fi

# log
if [[ ! -d log ]]; then
	git clone https://github.com/rust-lang/log
	cd log
	rustc --crate-name log --crate-type=lib src/lib.rs --extern libc=../libc/liblibc.rlib
	cd ..
fi

# xml-rs
if [[ ! -d xml-rs ]]; then
	git clone https://github.com/netvl/xml-rs
	cd xml-rs
	rustc --crate-name xml --crate-type=lib src/lib.rs -L ../bitflags/
	cd ..
fi

# glfw-rs
if [[ ! -d glfw-rs ]]; then
	git clone https://github.com/bjz/glfw-rs
	cd glfw-rs

	# comment out the -lglfw3 directives
	sed -i "s/#\[link.\+glfw3.\+\]/\/\/ \0/" src/ffi/link.rs

	rustc --crate-type=lib --crate-name glfw --cfg feature=\"glfw-sys\" src/lib.rs \
		-L ../semver/ -L ../bitflags/ -L ../libc/ \
		--extern log=../log/liblog.rlib --extern libc=../libc/liblibc.rlib
	cd ..
fi

# gl-rs
if [[ ! -d gl-rs ]]; then
	git clone --recursive https://github.com/bjz/gl-rs
	cp ../build_gl_15.rs gl-rs/gl/build.rs

	cd gl-rs

	cd khronos_api
	echo khronos_api
	rustc --crate-name khronos_api --crate-type=lib src/lib.rs --extern libc=../../../libc/liblibc.rlib
	cd ..

	cd gl_common
	echo gl_common
	rustc --crate-name gl_common --crate-type=lib src/lib.rs --extern libc=../../libc/liblibc.rlib
	cd ..

	cd gl_generator
	echo gl_generator
	rustc --crate-name gl_generator --crate-type=lib lib.rs \
		-L ../khronos_api/ -L ../../xml-rs/ -L ../../bitflags -L ../../libc \
		--extern log=../../log/liblog.rlib
	cd ..

	echo build.rs
	rustc gl/build.rs -L gl_generator -L khronos_api \
		-L ../xml-rs -L ../log -L ../libc -L ../bitflags

	echo build
	OUT_DIR=. ./build

	echo gl
	OUT_DIR=../.. rustc --crate-name gl --crate-type lib -L gl_common gl/src/lib.rs -L ../libc -L ../libc -L ../libc
fi
