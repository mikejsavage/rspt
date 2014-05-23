![25000 samples](25000.png)

You need to build `gl-rs` with OpenGL 2.1 bindings. Full build
instructions look like:

	mv gl-rs/src/gl/lib.rs gl-rs/src/gl/gl43.rs
	cp gl-rs/src/gl/gl21.rs gl-rs/src/gl/lib.rs
	make -C gl-rs
	make -C glfw-rs
	make
