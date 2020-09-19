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

#[derive(Clone, Copy, Default)]
pub struct RenderSys_DrawCommand {
	count: u32,
	instanceCount: u32,
	firstIndex: u32,
	baseVertex: u32,
	baseInstance: u32,
}
pub struct RenderSys {
	allocs: Arc<RenderAllocs>,
	vao: [GLuint; 3],
	shader: GLuint,
	camidx: GLuint,
	cambuf: Buffer<CameraUniform>,
	cmdA: Buffer<RenderSys_DrawCommand>,
	cmdB: Buffer<RenderSys_DrawCommand>,
	cmd_phase: i32,
}
impl RenderSys {
	pub fn new(allocs: &Arc<RenderAllocs>) -> Self {
		let ctx = allocs.ctx();
		let gl = &ctx.gl;

		let mut vao = [0, 0, 0];
		let vsize = [2, 3, 3];
		unsafe {
			gl.CreateVertexArrays(3, vao.as_mut_ptr());
			for i in 0..3 {
				gl.EnableVertexArrayAttrib(vao[i], 0);
				gl.VertexArrayAttribFormat(vao[i], 0, vsize[i], gl::FLOAT, gl::FALSE, 0);
				gl.VertexArrayAttribBinding(vao[i], 0, 0);
				gl.VertexArrayVertexBuffer(vao[i], 0, allocs.vert_alloc.id, 0, size_of::<Vertex>() as _);
				gl.VertexArrayElementBuffer(vao[i], allocs.idx_alloc.id);

			}
			gl.BindVertexArray(self.vao);

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

			let cmdA = allocs.alloc_other(&RenderSys_DrawCommand::default());
			let cmdB = allocs.alloc_other(&RenderSys_DrawCommand::default());
			let cmd_phase = 0;
			
			gl.BindBufferRange(
				gl::UNIFORM_BUFFER,
				self.camidx,
				self.allocs.other_alloc.id,
				self.cambuf.offset(),
				size_of::<CameraUniform>() as _,
			);

			gl.BindBuffer(
				gl::DRAW_INDIRECT_BUFFER,
				self.allocs.other_alloc.id,
			);

			Self { allocs: allocs.clone(), vao, shader, camidx, cambuf, cmdA, cmdB, cmd_phase: 0 }
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
			let cmd = if self.cmd_phase == 0 {&mut self.cmdB} else {&mut self.cmdA};
			for model in models.join() {
				for mesh in &model.meshes {
					cmd[0].count = mesh.index_count();
					cmd[0].instanceCount = 0;
					cmd[0].firstIndex = ?;
					cmd[0].baseVertex = ?;
					cmd[0].baseInstance = 0;
				}
			}
			gl.MultiDrawElementsIndirect(
				gl::TRIANGLES,
				gl::UNSIGNED_SHORT,
				(if self.cmd_phase == 0 {self.cmdA.offset()} else {self.cmdB.offset()} as _),
				1,
				0,
			);
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
