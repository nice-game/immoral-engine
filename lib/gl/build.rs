extern crate gl_generator;

use gl_generator::{Api, Fallbacks, GlobalGenerator, Profile, Registry};
use std::{env, fs::File, path::Path};

fn main() {
	let dest = env::var("OUT_DIR").unwrap();
	let mut file = File::create(&Path::new(&dest).join("gl_bindings.rs")).unwrap();

	Registry::new(Api::Gl, (4, 3), Profile::Core, Fallbacks::All, ["GL_ARB_direct_state_access"])
		.write_bindings(GlobalGenerator, &mut file)
		.unwrap();
}
