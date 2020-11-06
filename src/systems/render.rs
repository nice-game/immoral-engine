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
	alloc::{Allocation, Allocator},
	buffer::{Buffer, BufferSlice, DynamicBuffer},
	gl::{self},
	shader::ShaderProgram,
	texture::Texture,
	vertex::VertexArray,
	RenderSysDrawCommand,
};
use shipyard::{IntoIter, NonSendSync, Shiperator, UniqueView, UniqueViewMut, View, World};
use std::{mem::size_of, rc::Rc};

pub fn render_init(world: &World, allocs: &Rc<RenderAllocs>) {
	world.add_unique_non_send_sync(RenderState::new(allocs));
}

pub fn render(
	mut state: NonSendSync<UniqueViewMut<RenderState>>,
	player: UniqueView<PlayerController>,
	models: NonSendSync<View<Model>>,
) {
	state.cambuf.write(&player.cam.uniform);

	let cmdbuf = if !state.cmd_phase { &state.cmd_a } else { &state.cmd_b };

	let cmds: Vec<_> = models
		.iter()
		.map(|model| model.meshes.iter())
		.into_iterator()
		.flatten()
		.map(|mesh| RenderSysDrawCommand {
			count: mesh.indices().len() as _,
			instance_count: mesh.instance.len() as _,
			first_index: mesh.indices().offset() as _,
			base_vertex: mesh.buf.offset() as _,
			base_instance: mesh.instance.offset() as _,
		})
		.collect();
	cmdbuf.write_range(0, &cmds);

	let ctx = state.allocs.ctx();
	ctx.use_program(&state.shader);
	ctx.bind_vertex_array(&state.vao);
	ctx.multi_draw_elements_indirect(cmdbuf);

	state.cmd_phase = !state.cmd_phase;
}

pub struct RenderState {
	allocs: Rc<RenderAllocs>,
	vao: VertexArray,
	shader: ShaderProgram,
	cambuf: Rc<DynamicBuffer<CameraUniform>>,
	cmd_a: Allocation<RenderSysDrawCommand>,
	cmd_b: Allocation<RenderSysDrawCommand>,
	cmd_phase: bool,
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

			let cmdalloc = Allocator::<RenderSysDrawCommand>::new(ctx, 200);
			let cmd_a = cmdalloc.alloc_default_slice(100);
			let cmd_b = cmdalloc.alloc_default_slice(100);

			gl.BindBufferRange(gl::UNIFORM_BUFFER, camidx, cambuf.handle(), 0, size_of::<CameraUniform>() as _);

			ctx.use_program(&shader);
			gl.ActiveTexture(gl::TEXTURE0);
			gl.BindTexture(gl::TEXTURE_2D_ARRAY, allocs.tex.handle());
			gl.Uniform1i(gl.GetUniformLocation(shader.handle(), "tex\0".as_ptr() as _), 0);

			// gl.BindBuffer(gl::DRAW_INDIRECT_BUFFER, allocs.other_alloc.id);

			Self { allocs: allocs.clone(), vao, shader, cambuf, cmd_a, cmd_b, cmd_phase: false }
		}
	}
}
