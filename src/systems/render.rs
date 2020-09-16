use crate::{
	components::mesh::Mesh,
	glrs::{alloc::Allocator, buffer::Buffer},
	types::camera::{Camera, CameraUniform},
};
use gl::{types::GLuint, Gl};
use nalgebra::Vector3;
use specs::{prelude::*, System};
use std::{ffi::CString, iter::repeat, mem::size_of, ptr, sync::Arc};

pub struct Render {
	alloc: Arc<Allocator>,
	vao: GLuint,
	shader: GLuint,
	cam: Camera,
	cambuf: Buffer<CameraUniform>,
	camidx: GLuint,
}
impl Render {
	pub fn new(alloc: &Arc<Allocator>) -> Self {
		let ctx = &alloc.ctx;
		let gl = &ctx.gl;

		let mut vao = 0;
		unsafe {
			gl.CreateVertexArrays(1, &mut vao);
			gl.EnableVertexArrayAttrib(vao, 0);
			gl.VertexArrayAttribFormat(vao, 0, 2, gl::FLOAT, gl::FALSE, 0);
			gl.VertexArrayAttribBinding(vao, 0, 0);

			let src = CString::new(include_str!("../shaders/shader.vert")).unwrap();
			let vshader = gl.CreateShader(gl::VERTEX_SHADER);
			gl.ShaderSource(vshader, 1, [src.as_ptr()].as_ptr(), ptr::null());
			gl.CompileShader(vshader);
			check_shader(gl, vshader);

			let src = CString::new(include_str!("../shaders/shader.frag")).unwrap();
			let fshader = gl.CreateShader(gl::FRAGMENT_SHADER);
			gl.ShaderSource(fshader, 1, [src.as_ptr()].as_ptr(), ptr::null());
			gl.CompileShader(fshader);
			check_shader(gl, fshader);

			let shader = gl.CreateProgram();
			gl.AttachShader(shader, vshader);
			gl.AttachShader(shader, fshader);
			gl.LinkProgram(shader);
			check_program(gl, shader);

			gl.DeleteShader(fshader);
			gl.DeleteShader(vshader);

			let camidx = gl.GetUniformBlockIndex(shader, "Camera\0".as_ptr() as _);

			let mut cam = Camera::new();
			cam.uniform.pos = Vector3::new(0.0, -5.0, 0.0);
			let cambuf = Buffer::init(alloc).copy(&cam.uniform);

			Self { alloc: alloc.clone(), vao, shader, cam, cambuf, camidx }
		}
	}
}
impl<'a> System<'a> for Render {
	type SystemData = ReadStorage<'a, Mesh>;

	fn run(&mut self, meshes: Self::SystemData) {
		self.cambuf.copy(&self.cam.uniform);

		let gl = &self.alloc.ctx.gl;
		unsafe {
			gl.UseProgram(self.shader);
			gl.BindBufferRange(
				gl::UNIFORM_BUFFER,
				self.camidx,
				self.alloc.vbo,
				self.cambuf.offset(),
				size_of::<CameraUniform>() as _,
			);
			gl.BindVertexArray(self.vao);
			for mesh in meshes.join() {
				mesh.bind(self.vao);
				gl.DrawArrays(gl::TRIANGLES, 0, 3);
			}
		}
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
		gl.GetShaderInfoLog(program, 512, ptr::null_mut(), info_log.as_mut_ptr() as _);
		panic!("{:?}", info_log);
	}
}
