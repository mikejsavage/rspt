#! /bin/sh

mkdir -p deps
cd deps

# xml-rs
if [[ ! -d xml-rs ]]; then
	git clone https://github.com/netvl/xml-rs
	cd xml-rs
	rustc --crate-name xml --crate-type=lib src/lib.rs
	cd ..
fi

# semver
if [[ ! -d semver ]]; then
	git clone https://github.com/rust-lang/semver
	cd semver
	rustc --crate-name semver --crate-type=lib src/lib.rs
	cd ..
fi

# glfw-rs
if [[ ! -d glfw-rs ]]; then
	git clone https://github.com/bjz/glfw-rs
	cd glfw-rs

	# comment out the -lglfw3 directives
	sed -i "s/#\[link.\+glfw3.\+\]/\/\/ \0/" src/ffi/link.rs

	rustc --crate-type=lib --crate-name glfw src/lib.rs -L ../semver/
	cd ..
fi

# gl-rs
if [[ ! -d gl-rs ]]; then
	git clone --recursive https://github.com/bjz/gl-rs
	cp ../build_gl.rs gl-rs/build.rs

	cd gl-rs/src/khronos_api/
	rustc --crate-name khronos_api --crate-type=lib src/lib.rs

	cd ../gl_common/
	rustc --crate-name gl_common --crate-type=lib src/lib.rs

	cd ../gl_generator/
	rustc --crate-name gl_generator --crate-type=lib -L ../khronos_api/ -L ../../../xml-rs/ lib.rs
	cd ../..

	rustc build.rs -L src/gl_generator/ -L src/khronos_api/ -L ../xml-rs/
	OUT_DIR=. ./build

	OUT_DIR=../.. rustc --crate-name gl --crate-type lib -L src/gl_common/ src/gl/lib.rs
fi
