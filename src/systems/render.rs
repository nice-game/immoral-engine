pub mod allocs;

use crate::{
	components::{
		model::{Model, Vertex},
		player_controller::PlayerController,
	},
	glrs::buffer::Buffer,
	types::camera::CameraUniform,
	RenderAllocs,
};
use gl::{types::GLuint, Gl};
use specs::{prelude::*, System};
use std::{ffi::CString, iter::repeat, mem::size_of, ptr, sync::Arc};

pub struct RenderSys {
	allocs: Arc<RenderAllocs>,
	vao: GLuint,
	shader: GLuint,
	camidx: GLuint,
	cambuf: Buffer<CameraUniform>,
}
impl RenderSys {
	pub fn new(allocs: &Arc<RenderAllocs>) -> Self {
		let ctx = allocs.ctx();
		let gl = &ctx.gl;

		let mut vao = 0;
		unsafe {
			gl.CreateVertexArrays(1, &mut vao);
			gl.EnableVertexArrayAttrib(vao, 0);
			gl.VertexArrayAttribFormat(vao, 0, 2, gl::FLOAT, gl::FALSE, 0);
			gl.VertexArrayAttribBinding(vao, 0, 0);
			gl.VertexArrayVertexBuffer(vao, 0, allocs.vert_alloc.id, 0, size_of::<Vertex>() as _);
			gl.VertexArrayElementBuffer(vao, allocs.idx_alloc.id);

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

			let cambuf = allocs.alloc_other(&CameraUniform::default());

			Self { allocs: allocs.clone(), vao, shader, cambuf, camidx }
		}
	}
}
impl<'a> System<'a> for RenderSys {
	type SystemData = (ReadStorage<'a, PlayerController>, ReadStorage<'a, Model>);

	fn run(&mut self, (players, models): Self::SystemData) {
		let player = players.join().next().unwrap();

		self.cambuf.copy(&player.cam.uniform);

		let gl = &self.allocs.ctx().gl;
		unsafe {
			gl.UseProgram(self.shader);
			gl.BindVertexArray(self.vao);
			// does this need to be bound every frame?
			gl.BindBufferRange(
				gl::UNIFORM_BUFFER,
				self.camidx,
				self.allocs.other_alloc.id,
				self.cambuf.offset(),
				size_of::<CameraUniform>() as _,
			);
			for model in models.join() {
				for mesh in &model.meshes {
					gl.DrawElements(
						gl::TRIANGLES,
						mesh.index_count() as _,
						gl::UNSIGNED_SHORT,
						mesh.index_offset() as _,
					);
				}
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
