use crate::components::mesh::{Mesh, Vertex};
use array_init::array_init;
use gl::types::GLuint;
use nalgebra::Vector2;
use specs::{prelude::*, System};
use std::{
	ffi::{CStr, CString},
	mem::size_of,
	ptr,
};

pub struct Render {
	shader: GLuint,
	vao: GLuint,
}
impl Render {
	pub fn new() -> Self {
		unsafe {
			let src = CString::new(include_str!("../shaders/shader.vert")).unwrap();
			let vshader = gl::CreateShader(gl::VERTEX_SHADER);
			gl::ShaderSource(vshader, 1, [src.as_ptr()].as_ptr(), ptr::null());
			gl::CompileShader(vshader);
			check_shader(vshader);

			let src = CString::new(include_str!("../shaders/shader.frag")).unwrap();
			let fshader = gl::CreateShader(gl::FRAGMENT_SHADER);
			gl::ShaderSource(fshader, 1, [src.as_ptr()].as_ptr(), ptr::null());
			gl::CompileShader(fshader);
			check_shader(fshader);

			let shader = gl::CreateProgram();
			gl::AttachShader(shader, vshader);
			gl::AttachShader(shader, fshader);
			gl::LinkProgram(shader);

			gl::DeleteShader(fshader);
			gl::DeleteShader(vshader);

			let mut vao = 0;
			gl::GenVertexArrays(1, &mut vao);
			gl::EnableVertexArrayAttrib(vao, 0);
			gl::VertexArrayAttribFormat(vao, 0, size_of::<Vector2<f32>>() as _, gl::FLOAT, gl::FALSE, 0);
			gl::VertexArrayAttribBinding(vao, 0, 0);

			println!("shader: {}", shader);
			Self { shader, vao }
		}
	}
}
impl<'a> System<'a> for Render {
	type SystemData = ReadStorage<'a, Mesh>;

	fn run(&mut self, meshes: Self::SystemData) {
		unsafe {
			gl::UseProgram(self.shader);
			gl::BindVertexArray(self.vao);
			for mesh in meshes.join() {
				gl::VertexArrayVertexBuffer(self.vao, 0, mesh.vbo, 0, size_of::<Vertex>() as _);
				gl::DrawArrays(gl::TRIANGLES, 0, 3);
			}
		}
	}
}

unsafe fn check_shader(shader: GLuint) {
	let mut success = -1;
	gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
	if success == 0 {
		let mut info_log: [i8; 512] = array_init(|_| 0);
		gl::GetShaderInfoLog(shader, 512, ptr::null_mut(), info_log.as_mut_ptr());
		let info_log = CStr::from_ptr(info_log.as_ptr());
		panic!("{:?}", info_log);
	}
}
