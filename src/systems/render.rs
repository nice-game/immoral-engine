pub mod allocs;

use crate::{
	components::{
		model::{Instance, Model, Vertex},
		player_controller::PlayerController,
	},
	types::camera::CameraUniform,
	RenderAllocs,
};
use glrs::{
	buffer::{Buffer, BufferSlice, DynamicBuffer},
	gl::{self, types::GLuint, Gl},
	shader::ShaderProgram,
	texture::Texture,
	vertex::VertexArray,
};
use shipyard::{IntoIter, NonSendSync, UniqueView, UniqueViewMut, View, World};
use std::{ffi::CString, iter::repeat, mem::size_of, ptr, rc::Rc};

pub fn render_init(world: &World, allocs: &Rc<RenderAllocs>) {
	world.add_unique_non_send_sync(RenderState::new(allocs));
}

pub fn render(
	state: NonSendSync<UniqueViewMut<RenderState>>,
	player: UniqueView<PlayerController>,
	models: NonSendSync<View<Model>>,
) {
	state.cambuf.write(&player.cam.uniform);

	// unsafe {
	// let mut cmd_counter = 0;
	// let cmd = if !state.cmd_phase { &mut state.cmd_a } else { &mut state.cmd_b };
	// for model in models.iter() {
	// 	for mesh in &model.meshes {
	// 		cmd[cmd_counter].count = mesh.index_count() as u32;
	// 		cmd[cmd_counter].instance_count = 1;
	// 		cmd[cmd_counter].first_index = mesh.index_offset() as u32;
	// 		cmd[cmd_counter].base_vertex = mesh.buf.offset() as u32;
	// 		cmd[cmd_counter].base_instance = mesh.instance.offset() as u32;
	// 		cmd_counter += 1;
	// 	}
	// }
	// if !state.cmd_phase {
	// 	state.cmd_a_length = cmd_counter;
	// 	state.cmd_phase = false;
	// } else {
	// 	state.cmd_b_length = cmd_counter;
	// 	state.cmd_phase = true;
	// }
	// let gl = &state.allocs.ctx().gl;
	// gl.BindVertexArray(state.vao[1]);
	// gl.MultiDrawElementsIndirect(
	// gl::TRIANGLES,
	// gl::UNSIGNED_SHORT,
	// if state.cmd_phase {state.cmd_a.offset()} else {state.cmd_b.offset()} as _,
	// if state.cmd_phase {state.cmd_a_length} else {state.cmd_b_length} as _,
	// 0,
	// );
	// }
	state.allocs.ctx().use_program(&state.shader);
	state.allocs.ctx().bind_vertex_array(&state.vao);
	for model in models.iter() {
		for mesh in &model.meshes {
			state.allocs.ctx().draw_elements_instanced(mesh.indices(), &mesh.buf, &mesh.instance);
		}
	}
}

#[derive(Clone, Copy, Default)]
#[repr(C)]
struct RenderSysDrawCommand {
	count: u32,
	instance_count: u32,
	first_index: u32,
	base_vertex: u32,
	base_instance: u32,
}

pub struct RenderState {
	allocs: Rc<RenderAllocs>,
	vao: VertexArray,
	shader: ShaderProgram,
	cambuf: Rc<DynamicBuffer<CameraUniform>>,
	/* cmd_a: Buffer<[RenderSysDrawCommand]>,
	 * cmd_b: Buffer<[RenderSysDrawCommand]>,
	 * cmd_a_length: usize,
	 * cmd_b_length: usize,
	 * cmd_phase: bool, */
}
impl RenderState {
	fn new(allocs: &Rc<RenderAllocs>) -> Self {
		let ctx = allocs.ctx();
		let gl = &ctx.gl;

		let mut vao = VertexArray::new(ctx);
		vao.enable_vertices::<Instance>(1);
		vao.enable_vertices::<Vertex>(0);
		vao.element_buffer(&allocs.idx_alloc);
		vao.vertex_buffer(0, &allocs.instance_alloc);
		vao.vertex_buffer(1, &allocs.vert_alloc);

		let shader = ShaderProgram::init(ctx)
			.vertex_file("src/shaders/shader.vert")
			.fragment_file("src/shaders/shader.frag")
			.build();

		unsafe {
			let camidx = gl.GetUniformBlockIndex(shader.handle(), "Camera\0".as_ptr() as _);

			let cambuf = Buffer::from_val(ctx, &CameraUniform::default());

			// let cmd_a = allocs.alloc_other_slice(&[RenderSysDrawCommand::default(); 100]);
			// let cmd_b = allocs.alloc_other_slice(&[RenderSysDrawCommand::default(); 100]);

			gl.BindBufferRange(gl::UNIFORM_BUFFER, camidx, cambuf.handle(), 0, size_of::<CameraUniform>() as _);

			gl.UseProgram(shader.handle());
			gl.ActiveTexture(gl::TEXTURE0);
			gl.BindTexture(gl::TEXTURE_2D_ARRAY, allocs.tex.handle());
			gl.Uniform1i(gl.GetUniformLocation(shader.handle(), "tex\0".as_ptr() as _), 0);

			// gl.BindBuffer(gl::DRAW_INDIRECT_BUFFER, allocs.other_alloc.id);

			Self {
				allocs: allocs.clone(),
				vao,
				shader,
				cambuf,
				/* cmd_a,
				 * cmd_b,
				 * cmd_a_length: 0,
				 * cmd_b_length: 0,
				 * cmd_phase: false, */
			}
		}
	}
}
