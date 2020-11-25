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
	commands::CommandBuffer,
	framebuffer::Framebuffer,
	shader::ShaderProgram,
	texture::Texture2D,
	vertex::VertexArray,
};
use shipyard::{IntoIter, NonSendSync, UniqueView, View, World};
use std::rc::Rc;

pub fn render_init(world: &World, allocs: &Rc<RenderAllocs>) {
	world.add_unique_non_send_sync(RenderState::new(allocs));
}

pub fn render(
	state: NonSendSync<UniqueView<RenderState>>,
	player: UniqueView<PlayerController>,
	models: NonSendSync<View<Model>>,
) {
	state.cambuf.write(&player.cam.uniform);

	let mut cmds = CommandBuffer::new(&state.vao);
	for model in models.iter() {
		for mesh in &model.meshes {
			cmds.push(
				mesh.indices().len() as _,
				mesh.instance.len() as _,
				mesh.indices().offset() as _,
				mesh.buf.offset() as _,
				mesh.instance.offset() as _,
			);
		}
	}

	let ctx = state.allocs.ctx();
	ctx.use_program(&state.shader);
	ctx.multi_draw_elements_indirect(cmds);
}

pub struct RenderState {
	allocs: Rc<RenderAllocs>,
	vao: VertexArray,
	shader: ShaderProgram,
	cambuf: Rc<DynamicBuffer<CameraUniform>>,
}
impl RenderState {
	fn new(allocs: &Rc<RenderAllocs>) -> Self {
		let ctx = allocs.ctx();
		ctx.bind_texture(0, &allocs.tex);

		let mut vao = VertexArray::new(ctx);
		vao.enable_vertices::<Instance>(1);
		vao.enable_vertices::<Vertex>(0);
		vao.element_buffer(&allocs.idx_alloc);
		vao.vertex_buffer(0, &allocs.instance_alloc);
		vao.vertex_buffer(1, &allocs.vert_alloc);

		let cambuf = Buffer::from_val(ctx, &CameraUniform::default());

		let shader = ShaderProgram::init(ctx)
			.vertex_file("src/shaders/shader.vert")
			.fragment_file("src/shaders/shader.frag")
			.build();
		shader.set_uniform_i32("tex", 0);
		shader.bind_buffer_range("Camera", cambuf.clone());

		let [width, height]: [_; 2] = ctx.window().window().inner_size().into();
		let color = Texture2D::new(ctx, [width, height].into());

		let framebuffer = Framebuffer::new(ctx);
		framebuffer.color(0, &color);

		Self { allocs: allocs.clone(), vao, shader, cambuf }
	}
}
