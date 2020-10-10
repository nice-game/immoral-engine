pub mod allocs;

use crate::{
	components::{
		model::{Instance, Model, Vertex},
		player_controller::PlayerController,
	},
	glrs::{buffer::Buffer, texture::Texture},
	types::camera::CameraUniform,
	RenderAllocs,
};
use gl::{types::GLuint, Gl};
use shipyard::{IntoIter, NonSendSync, UniqueView, UniqueViewMut, View};
use std::{ffi::CString, iter::repeat, mem::size_of, ptr, sync::Arc};

#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct RenderSysDrawCommand {
	count: u32,
	instance_count: u32,
	first_index: u32,
	base_vertex: u32,
	base_instance: u32,
}
pub struct RenderState {
	allocs: Arc<RenderAllocs>,
	vao: [GLuint; 3],
	shader: GLuint,
	cambuf: Buffer<CameraUniform>,
	cmd_a: Buffer<[RenderSysDrawCommand]>,
	cmd_b: Buffer<[RenderSysDrawCommand]>,
	cmd_a_length: usize,
	cmd_b_length: usize,
	cmd_phase: bool,
}
impl RenderState {
	pub fn new(allocs: &Arc<RenderAllocs>) -> Self {
		let ctx = allocs.ctx();
		let gl = &ctx.gl;

		let mut vao = [0, 0, 0];
		let vsize = [2, 3, 3];
		unsafe {
			gl.CreateVertexArrays(3, vao.as_mut_ptr());
			for i in 0..3 {
				// index buffer
				gl.VertexArrayElementBuffer(vao[i], allocs.idx_alloc.id);

				// instances
				gl.EnableVertexArrayAttrib(vao[i], 0);
				gl.VertexArrayAttribFormat(vao[i], 0, 1, gl::FLOAT, gl::FALSE, 0);
				gl.VertexArrayAttribBinding(vao[i], 0, 0);
				gl.VertexArrayBindingDivisor(vao[i], 0, 1);
				gl.VertexArrayVertexBuffer(vao[i], 0, allocs.instance_alloc.id, 0, size_of::<Instance>() as _);

				// positions
				gl.EnableVertexArrayAttrib(vao[i], 1);
				gl.VertexArrayAttribFormat(vao[i], 1, vsize[i], gl::FLOAT, gl::FALSE, 0);
				gl.VertexArrayAttribBinding(vao[i], 1, 1);
				gl.VertexArrayVertexBuffer(vao[i], 1, allocs.vert_alloc.id, 0, size_of::<Vertex>() as _);
				if i >= 1 {
					// tangent frames
					gl.EnableVertexArrayAttrib(vao[i], 2);
					gl.VertexArrayAttribFormat(vao[i], 2, 4, gl::FLOAT, gl::FALSE, 12);
					gl.VertexArrayAttribBinding(vao[i], 2, 1);
					gl.VertexArrayVertexBuffer(vao[i], 2, allocs.vert_alloc.id, 0, size_of::<Vertex>() as _);
					// texcoords
					gl.EnableVertexArrayAttrib(vao[i], 3);
					gl.VertexArrayAttribFormat(vao[i], 3, 4, gl::FLOAT, gl::FALSE, 28);
					gl.VertexArrayAttribBinding(vao[i], 3, 1);
					gl.VertexArrayVertexBuffer(vao[i], 3, allocs.vert_alloc.id, 0, size_of::<Vertex>() as _);
				}
				if i >= 2 {
					// bone ids
					gl.EnableVertexArrayAttrib(vao[i], 4);
					gl.VertexArrayAttribFormat(vao[i], 4, 4, gl::UNSIGNED_BYTE, gl::FALSE, 44);
					gl.VertexArrayAttribBinding(vao[i], 4, 1);
					gl.VertexArrayVertexBuffer(vao[i], 4, allocs.vert_alloc.id, 0, size_of::<Vertex>() as _);
					// bone weights
					gl.EnableVertexArrayAttrib(vao[i], 5);
					gl.VertexArrayAttribFormat(vao[i], 5, 4, gl::UNSIGNED_BYTE, gl::TRUE, 48);
					gl.VertexArrayAttribBinding(vao[i], 5, 1);
					gl.VertexArrayVertexBuffer(vao[i], 5, allocs.vert_alloc.id, 0, size_of::<Vertex>() as _);
				}
			}

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

			let cmd_a = allocs.alloc_other_slice(&[RenderSysDrawCommand::default(); 100]);
			let cmd_b = allocs.alloc_other_slice(&[RenderSysDrawCommand::default(); 100]);

			gl.BindBufferRange(
				gl::UNIFORM_BUFFER,
				camidx,
				allocs.other_alloc.id,
				cambuf.offset() as _,
				size_of::<CameraUniform>() as _,
			);

			gl.UseProgram(shader);
			gl.ActiveTexture(gl::TEXTURE0);
			gl.BindTexture(gl::TEXTURE_2D_ARRAY, allocs.tex.handle());
			gl.Uniform1i(gl.GetUniformLocation(shader, "tex\0".as_ptr() as _), 0);

			gl.BindBuffer(gl::DRAW_INDIRECT_BUFFER, allocs.other_alloc.id);

			Self {
				allocs: allocs.clone(),
				vao,
				shader,
				cambuf,
				cmd_a,
				cmd_b,
				cmd_a_length: 0,
				cmd_b_length: 0,
				cmd_phase: false,
			}
		}
	}
}

pub fn render(
	mut state: NonSendSync<UniqueViewMut<RenderState>>,
	player: UniqueView<PlayerController>,
	models: NonSendSync<View<Model>>,
) {
	state.cambuf.copy(&player.cam.uniform);

	unsafe {
		let gl = &state.allocs.ctx().gl;
		gl.UseProgram(state.shader);

		let mut cmd_counter = 0;
		let cmd = if !state.cmd_phase { &mut state.cmd_a } else { &mut state.cmd_b };
		for model in models.iter() {
			for mesh in &model.meshes {
				cmd[cmd_counter].count = mesh.index_count() as u32;
				cmd[cmd_counter].instance_count = 1;
				cmd[cmd_counter].first_index = mesh.index_offset() as u32;
				cmd[cmd_counter].base_vertex = mesh.buf.offset() as u32;
				cmd[cmd_counter].base_instance = mesh.instance.offset() as u32;
				cmd_counter += 1;
			}
		}
		if !state.cmd_phase {
			state.cmd_a_length = cmd_counter;
			state.cmd_phase = false;
		} else {
			state.cmd_b_length = cmd_counter;
			state.cmd_phase = true;
		}
		let gl = &state.allocs.ctx().gl;
		// gl.BindVertexArray(state.vao[1]);
		// gl.MultiDrawElementsIndirect(
		// gl::TRIANGLES,
		// gl::UNSIGNED_SHORT,
		// if state.cmd_phase {state.cmd_a.offset()} else {state.cmd_b.offset()} as _,
		// if state.cmd_phase {state.cmd_a_length} else {state.cmd_b_length} as _,
		// 0,
		// );
		for model in models.iter() {
			for mesh in &model.meshes {
				gl.BindVertexArray(state.vao[1]);
				gl.DrawElementsInstancedBaseVertexBaseInstance(
					gl::TRIANGLES,
					mesh.index_count() as _,
					gl::UNSIGNED_SHORT,
					mesh.index_offset() as _,
					1,
					(mesh.buf.offset() as usize / size_of::<Vertex>()) as _,
					(mesh.instance.offset() as usize / size_of::<Instance>()) as _,
				);
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
		gl.GetProgramInfoLog(program, 512, ptr::null_mut(), info_log.as_mut_ptr() as _);
		panic!("{:?}", info_log);
	}
}
