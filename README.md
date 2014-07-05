![25000 samples](25000.png)

You need to build `gl-rs` with OpenGL 2.1 bindings. Full build
instructions look like:

	sed -i "s/4\.3/2.1" gl-rs/Makefile
	make -C gl-rs
	make -C glfw-rs
	make
