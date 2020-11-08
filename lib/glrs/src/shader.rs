use crate::{BufferSlice, Ctx};
use gl::{
	types::{GLenum, GLint, GLsizei, GLuint},
	Gl,
};
use std::{
	collections::HashMap,
	ffi::{CStr, CString},
	fs::File,
	io::prelude::*,
	iter::repeat,
	mem::size_of,
	path::Path,
	ptr,
	rc::Rc,
};

pub struct ShaderProgram {
	ctx: Rc<Ctx>,
	handle: GLuint,
	uniforms: HashMap<String, Uniform>,
	uniform_blocks: HashMap<String, UniformBlock>,
}
impl ShaderProgram {
	pub fn init(ctx: &Rc<Ctx>) -> ShaderBuilder {
		ShaderBuilder::new(ctx)
	}

	pub fn handle(&self) -> GLuint {
		self.handle
	}

	pub fn set_uniform_i32(&self, name: &str, val: i32) {
		let uniform = self.uniforms.get(name).unwrap();
		// assert_eq!(uniform.typ, gl::INT);
		assert_eq!(uniform.size, 1);
		unsafe { self.ctx.gl.ProgramUniform1i(self.handle, uniform.location, val) };
	}

	pub fn bind_buffer_range<T>(&self, name: &str, buffer: impl BufferSlice<T>) {
		let gl = &self.ctx.gl;
		let uniform_block = self.uniform_blocks.get(name).unwrap();
		unsafe {
			gl.BindBufferRange(
				gl::UNIFORM_BUFFER,
				uniform_block.index,
				buffer.handle(),
				buffer.offset() as _,
				(buffer.len() * size_of::<T>()) as _,
			)
		};
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

			let uniform_count = get_active_resources(gl, handle, gl::UNIFORM);
			let mut uniforms = HashMap::with_capacity(uniform_count as _);
			let mut uniform_blocks = HashMap::with_capacity(uniform_count as _);
			for idx in 0..uniform_count as _ {
				let [name_len, location, index, typ, size] = get_resource_props(gl, handle, gl::UNIFORM, idx, [
					gl::NAME_LENGTH,
					gl::LOCATION,
					gl::BLOCK_INDEX,
					gl::TYPE,
					gl::ARRAY_SIZE,
				]);
				let name = get_resource_name(gl, handle, gl::UNIFORM, idx, name_len);
				let uniform = Uniform { location, typ: typ as _, size };
				if location > -1 {
					uniforms.insert(name, uniform);
				} else if index > -1 {
					let name = name.split(".").next().unwrap().to_owned();
					uniform_blocks
						.entry(name)
						.or_insert(UniformBlock { index: index as _, props: vec![] })
						.props
						.push(uniform);
				}
			}

			ShaderProgram { ctx: self.ctx, handle, uniforms, uniform_blocks }
		}
	}

	fn new(ctx: &Rc<Ctx>) -> Self {
		Self { ctx: ctx.clone(), vertex: 0, fragment: 0 }
	}
}

unsafe fn get_active_resources(gl: &Gl, handle: GLuint, interface: GLenum) -> GLint {
	let mut count = 0;
	gl.GetProgramInterfaceiv(handle, interface, gl::ACTIVE_RESOURCES, &mut count);
	count
}

unsafe fn get_resource_props<const N: usize>(
	gl: &Gl,
	handle: GLuint,
	interface: GLenum,
	idx: GLuint,
	props: [GLenum; N],
) -> [GLint; N] {
	let mut values = [0; N];
	gl.GetProgramResourceiv(
		handle,
		interface,
		idx,
		props.len() as _,
		props.as_ptr(),
		values.len() as _,
		ptr::null_mut(),
		values.as_mut_ptr(),
	);
	values
}

unsafe fn get_resource_name(gl: &Gl, handle: GLuint, interface: GLenum, idx: GLuint, name_len: GLsizei) -> String {
	let mut name = Vec::<u8>::with_capacity(name_len as _);
	name.set_len(name_len as _);
	gl.GetProgramResourceName(handle, interface, idx, name_len, ptr::null_mut(), name.as_mut_ptr() as _);
	name.pop();
	String::from_utf8(name).unwrap()
}

#[derive(Debug)]
struct Uniform {
	location: GLint,
	typ: GLenum,
	size: GLint,
}
#[derive(Debug)]
struct UniformBlock {
	index: GLuint,
	props: Vec<Uniform>,
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
