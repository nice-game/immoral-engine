extern crate cmake;
extern crate pkg_config;

use cmake::Config;
use std::env;

fn main() {
	// Use system libassimp if it exists
	if let Ok(..) = pkg_config::Config::new().atleast_version("4.0.0").find("assimp") {
		return;
	}

	// Compile assimp from source
	// Disable unnecessary stuff, it takes long enough to compile already
	let dst = Config::new("assimp")
		.define("ASSIMP_BUILD_ASSIMP_TOOLS", "OFF")
		.define("ASSIMP_BUILD_TESTS", "OFF")
		.define("ASSIMP_INSTALL_PDB", "OFF")
		.define("BUILD_SHARED_LIBS", "OFF")
		.define("CMAKE_SUPPRESS_DEVELOPER_WARNINGS", "ON")
		.define("LIBRARY_SUFFIX", "")
		.build();
	println!("cargo:rustc-link-search=native={}", dst.join("lib").display());

	// Link to assimp and its dependencies
	let debug_postfix = if env::var("DEBUG").unwrap() == "true" { "d" } else { "" };
	println!("cargo:rustc-link-lib=static=assimp{}", debug_postfix);
	println!("cargo:rustc-link-lib=static=IrrXML{}", debug_postfix);
	// if !pkg_config::find_library("zlib").is_ok() {
	//     println!("cargo:rustc-link-lib=static=zlibstatic{}", debug_postfix);
	// }

	// Link to libstdc++ on GNU
	let target = env::var("TARGET").unwrap();
	if target.contains("gnu") {
		println!("cargo:rustc-link-lib=stdc++");
	} else if target.contains("apple") {
		println!("cargo:rustc-link-lib=c++");
	}

	println!("cargo:rerun-if-changed=build.rs");
}
