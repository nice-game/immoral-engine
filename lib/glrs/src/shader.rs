use crate::Ctx;
use gl::{
	types::{GLenum, GLuint},
	Gl,
};
use std::{
	ffi::{CStr, CString},
	fs::File,
	io::prelude::*,
	iter::repeat,
	path::Path,
	ptr,
	rc::Rc,
};

pub struct ShaderProgram {
	ctx: Rc<Ctx>,
	handle: GLuint,
}
impl ShaderProgram {
	pub fn init(ctx: &Rc<Ctx>) -> ShaderBuilder {
		ShaderBuilder::new(ctx)
	}

	pub fn handle(&self) -> GLuint {
		self.handle
	}
}


pub struct ShaderBuilder {
	ctx: Rc<Ctx>,
	vertex: GLuint,
	fragment: GLuint,
}
impl ShaderBuilder {
	pub fn vertex_file(mut self, path: impl AsRef<Path>) -> Self {
		let src = load_src(path);
		self.vertex = create_shader(&self.ctx, gl::VERTEX_SHADER, &src);
		self
	}

	pub fn fragment_file(mut self, path: impl AsRef<Path>) -> Self {
		let src = load_src(path);
		self.fragment = create_shader(&self.ctx, gl::FRAGMENT_SHADER, &src);
		self
	}

	pub fn build(self) -> ShaderProgram {
		assert_ne!(self.vertex, 0);

		let gl = &self.ctx.gl;
		unsafe {
			let handle = gl.CreateProgram();
			gl.AttachShader(handle, self.vertex);
			if self.fragment > 0 {
				gl.AttachShader(handle, self.fragment);
			}
			gl.LinkProgram(handle);
			check_program(gl, handle);

			if self.fragment > 0 {
				gl.DeleteShader(self.fragment);
			}
			gl.DeleteShader(self.vertex);

			ShaderProgram { ctx: self.ctx, handle }
		}
	}

	fn new(ctx: &Rc<Ctx>) -> Self {
		Self { ctx: ctx.clone(), vertex: 0, fragment: 0 }
	}
}

fn load_src(path: impl AsRef<Path>) -> CString {
	let mut file = File::open(path).unwrap();
	let mut src = String::new();
	file.read_to_string(&mut src).unwrap();
	CString::new(src).unwrap()
}

fn create_shader(ctx: &Ctx, typ: GLenum, src: &CStr) -> GLuint {
	let gl = &ctx.gl;
	unsafe {
		let handle = gl.CreateShader(typ);
		gl.ShaderSource(handle, 1, [src.as_ptr()].as_ptr(), ptr::null());
		gl.CompileShader(handle);
		check_shader(gl, handle);
		handle
	}
}

unsafe fn check_shader(gl: &Gl, shader: GLuint) {
	let mut success = -1;
	gl.GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
	if success == 0 {
		let mut len = 0;
		gl.GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
		let mut info_log: String = repeat('\0').take(len as _).collect();
		gl.GetShaderInfoLog(shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as _);
		panic!("{:?}", info_log);
	}
}

unsafe fn check_program(gl: &Gl, program: GLuint) {
	let mut success = -1;
	gl.GetProgramiv(program, gl::LINK_STATUS, &mut success);
	if success == 0 {
		let mut len = 0;
		gl.GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
		let mut info_log: String = repeat('\0').take(len as _).collect();
		gl.GetProgramInfoLog(program, 512, ptr::null_mut(), info_log.as_mut_ptr() as _);
		panic!("{:?}", info_log);
	}
}
