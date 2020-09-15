extern crate gl_generator;

use gl_generator::{generators, Api, Fallbacks, Generator, Profile, Registry};
use std::{env, fs::File, io, path::Path};

fn main() {
	let dest = env::var("OUT_DIR").unwrap();
	let mut file = File::create(&Path::new(&dest).join("gl_bindings.rs")).unwrap();

	Registry::new(Api::Gl, (4, 3), Profile::Core, Fallbacks::All, [
		"GL_ARB_buffer_storage",
		"GL_ARB_direct_state_access",
	])
	.write_bindings(DebugStructGenerator, &mut file)
	.unwrap();
}

// copyright notice applies to code below this point
// Copyright 2015 Brendan Zabarauskas and the gl-rs developers

#[allow(missing_copy_implementations)]
pub struct DebugStructGenerator;

impl Generator for DebugStructGenerator {
	fn write<W>(&self, registry: &Registry, dest: &mut W) -> io::Result<()>
	where
		W: io::Write,
	{
		write_header(dest)?;
		write_type_aliases(registry, dest)?;
		write_enums(registry, dest)?;
		write_fnptr_struct_def(dest)?;
		write_panicking_fns(registry, dest)?;
		write_struct(registry, dest)?;
		write_impl(registry, dest)?;
		Ok(())
	}
}

/// Creates a `__gl_imports` module which contains all the external symbols that we need for the
///  bindings.
fn write_header<W>(dest: &mut W) -> io::Result<()>
where
	W: io::Write,
{
	writeln!(
		dest,
		r#"
        mod __gl_imports {{
            pub use std::mem;
            pub use std::marker::Send;
            pub use std::os::raw;
        }}
    "#
	)
}

/// Creates a `types` module which contains all the type aliases.
///
/// See also `generators::gen_types`.
fn write_type_aliases<W>(registry: &Registry, dest: &mut W) -> io::Result<()>
where
	W: io::Write,
{
	writeln!(
		dest,
		r#"
        pub mod types {{
            #![allow(non_camel_case_types, non_snake_case, dead_code, missing_copy_implementations)]
    "#
	)?;

	generators::gen_types(registry.api, dest)?;

	writeln!(dest, "}}")
}

/// Creates all the `<enum>` elements at the root of the bindings.
fn write_enums<W>(registry: &Registry, dest: &mut W) -> io::Result<()>
where
	W: io::Write,
{
	for enm in &registry.enums {
		generators::gen_enum_item(enm, "types::", dest)?;
	}

	Ok(())
}

/// Creates a `FnPtr` structure which contains the store for a single binding.
fn write_fnptr_struct_def<W>(dest: &mut W) -> io::Result<()>
where
	W: io::Write,
{
	writeln!(
		dest,
		"
        #[allow(dead_code, missing_copy_implementations)]
        #[derive(Clone)]
        pub struct FnPtr {{
            /// The function pointer that will be used when calling the function.
            f: *const __gl_imports::raw::c_void,
            /// True if the pointer points to a real function, false if points to a `panic!` fn.
            is_loaded: bool,
        }}

        impl FnPtr {{
            /// Creates a `FnPtr` from a load attempt.
            fn new(ptr: *const __gl_imports::raw::c_void) -> FnPtr {{
                if ptr.is_null() {{
                    FnPtr {{
                        f: missing_fn_panic as *const __gl_imports::raw::c_void,
                        is_loaded: false
                    }}
                }} else {{
                    FnPtr {{ f: ptr, is_loaded: true }}
                }}
            }}

            /// Returns `true` if the function has been successfully loaded.
            ///
            /// If it returns `false`, calling the corresponding function will fail.
            #[inline]
            #[allow(dead_code)]
            pub fn is_loaded(&self) -> bool {{
                self.is_loaded
            }}
        }}
    "
	)
}

/// Creates a `panicking` module which contains one function per GL command.
///
/// These functions are the mocks that are called if the real function could not be loaded.
fn write_panicking_fns<W>(registry: &Registry, dest: &mut W) -> io::Result<()>
where
	W: io::Write,
{
	writeln!(
		dest,
		"#[inline(never)]
        fn missing_fn_panic() -> ! {{
            panic!(\"{api} function was not loaded\")
        }}",
		api = registry.api
	)
}

/// Creates a structure which stores all the `FnPtr` of the bindings.
///
/// The name of the struct corresponds to the namespace.
fn write_struct<W>(registry: &Registry, dest: &mut W) -> io::Result<()>
where
	W: io::Write,
{
	writeln!(
		dest,
		"
        #[allow(non_camel_case_types, non_snake_case, dead_code)]
        #[derive(Clone)]
        pub struct {api} {{",
		api = generators::gen_struct_name(registry.api)
	)?;

	for cmd in &registry.cmds {
		if let Some(v) = registry.aliases.get(&cmd.proto.ident) {
			writeln!(dest, "/// Fallbacks: {}", v.join(", "))?;
		}
		writeln!(dest, "pub {name}: FnPtr,", name = cmd.proto.ident)?;
	}
	writeln!(dest, "_priv: ()")?;

	writeln!(dest, "}}")
}

/// Creates the `impl` of the structure created by `write_struct`.
fn write_impl<W>(registry: &Registry, dest: &mut W) -> io::Result<()>
where
	W: io::Write,
{
	writeln!(
		dest,
		"impl {api} {{
            /// Load each OpenGL symbol using a custom load function. This allows for the
            /// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.
            ///
            /// ~~~ignore
            /// let gl = Gl::load_with(|s| glfw.get_proc_address(s));
            /// ~~~
            #[allow(dead_code, unused_variables)]
            pub fn load_with<F>(mut loadfn: F) -> {api} where F: FnMut(&'static str) -> *const \
		 __gl_imports::raw::c_void {{
                #[inline(never)]
                fn do_metaloadfn(loadfn: &mut dyn FnMut(&'static str) -> *const __gl_imports::raw::c_void,
                                 symbol: &'static str,
                                 symbols: &[&'static str])
                                 -> *const __gl_imports::raw::c_void {{
                    let mut ptr = loadfn(symbol);
                    if ptr.is_null() {{
                        for &sym in symbols {{
                            ptr = loadfn(sym);
                            if !ptr.is_null() {{ break; }}
                        }}
                    }}
                    ptr
                }}
                let mut metaloadfn = |symbol: &'static str, symbols: &[&'static str]| {{
                    do_metaloadfn(&mut loadfn, symbol, symbols)
                }};
                {api} {{",
		api = generators::gen_struct_name(registry.api)
	)?;

	for cmd in &registry.cmds {
		writeln!(
			dest,
			"{name}: FnPtr::new(metaloadfn(\"{symbol}\", &[{fallbacks}])),",
			name = cmd.proto.ident,
			symbol = generators::gen_symbol_name(registry.api, &cmd.proto.ident),
			fallbacks = match registry.aliases.get(&cmd.proto.ident) {
				Some(fbs) => fbs
					.iter()
					.map(|name| format!("\"{}\"", generators::gen_symbol_name(registry.api, &name)))
					.collect::<Vec<_>>()
					.join(", "),
				None => format!(""),
			},
		)?
	}
	writeln!(dest, "_priv: ()")?;

	writeln!(
		dest,
		"}}
        }}"
	)?;

	for cmd in &registry.cmds {
		let idents = generators::gen_parameters(cmd, true, false);
		let typed_params = generators::gen_parameters(cmd, false, true);
		// let println = format!(
		// 	"println!(\"[OpenGL] {}({})\" {});",
		// 	cmd.proto.ident,
		// 	(0..idents.len()).map(|_| "{:?}".to_string()).collect::<Vec<_>>().join(", "),
		// 	idents
		// 		.iter()
		// 		.zip(typed_params.iter())
		// 		.map(|(name, ty)| if ty.contains("GLDEBUGPROC") {
		// 			format!(", \"<callback>\"")
		// 		} else {
		// 			format!(", {}", name)
		// 		})
		// 		.collect::<Vec<_>>()
		// 		.concat()
		// );

		writeln!(
			dest,
			"#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn {name}(&self, {params}) -> {return_suffix} {{
                let r = __gl_imports::mem::transmute::<_, extern \"system\" fn({typed_params}) -> \
			 {return_suffix}>(self.{name}.f)({idents});
                {print_err}
                r
            }}",
			name = cmd.proto.ident,
			params = generators::gen_parameters(cmd, true, true).join(", "),
			typed_params = typed_params.join(", "),
			return_suffix = cmd.proto.ty,
			idents = idents.join(", "),
			print_err = if cmd.proto.ident != "GetError"
				&& registry.cmds.iter().find(|cmd| cmd.proto.ident == "GetError").is_some()
			{
				let mut map = std::collections::HashMap::new();
				map.insert(0x0500, "GL_INVALID_ENUM");
				map.insert(0x0502, "GL_INVALID_OPERATION");
				map.insert(0x0501, "GL_INVALID_VALUE");
				map.insert(0x0503, "GL_STACK_OVERFLOW");
				map.insert(0x0504, "GL_STACK_UNDERFLOW");
				map.insert(0x0505, "GL_OUT_OF_MEMORY");
				map.insert(0x0506, "GL_INVALID_FRAMEBUFFER_OPERATION");
				map.insert(0x0507, "GL_CONTEXT_LOST");
				map.insert(0x8031, "GL_TABLE_TOO_LARGE1");
				let mut stouf1 = std::string::String::new();
				let mut stouf2 = "".to_owned();
				for (asdf, fdsa) in &map {
					stouf1 += &format!("if r=={0} {{\"{1}\".to_owned()}} else {{", asdf, fdsa);
					stouf2 += "}";
				}
				format!(
					r#"match __gl_imports::mem::transmute::<_, extern "system" fn() -> u32>
                    (self.GetError.f)() {{ 0 => (), r => panic!("GL error triggered: {{}}", {0} format!("{{}}",r) {1}) }}"#,
					stouf1, stouf2
				)
			} else {
				format!("")
			}
		)?
	}

	writeln!(
		dest,
		"}}

        unsafe impl __gl_imports::Send for {api} {{}}",
		api = generators::gen_struct_name(registry.api)
	)
}
