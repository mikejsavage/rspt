extern crate gl_generator;
extern crate khronos_api;

use std::os;
use std::io::File;
use std::io::BufferedWriter;

fn main() {
	let dest = Path::new(os::getenv("OUT_DIR").unwrap());

	let mut file = BufferedWriter::new(File::create(&dest.join("bindings.rs")).unwrap());
	gl_generator::generate_bindings(gl_generator::GlobalGenerator,
					gl_generator::registry::Ns::Gl,
					khronos_api::GL_XML, vec![], "1.5", "core",
					&mut file).unwrap();
}
