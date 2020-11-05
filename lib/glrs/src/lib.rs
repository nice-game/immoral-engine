#![allow(incomplete_features)]
#![feature(const_generics)]

pub mod alloc;
pub mod buffer;
pub mod framebuffer;
pub mod shader;
pub mod texture;
pub mod vertex;

use crate::{
	buffer::BufferSlice,
	shader::ShaderProgram,
	vertex::{Vertex, VertexArray},
};
pub use gl;
pub use memoffset;
use std::mem::size_of;

use crate::framebuffer::DefaultFramebuffer;
use gl::Gl;
use glutin::{
	event_loop::EventLoop,
	window::{Window, WindowBuilder},
	ContextBuilder, ContextWrapper, GlProfile, PossiblyCurrent,
};
use std::rc::Rc;

pub struct Ctx {
	window: ContextWrapper<PossiblyCurrent, Window>,
	pub gl: Gl,
	uniform_align: i32,
}
impl Ctx {
	pub fn new(event_loop: &EventLoop<()>) -> Rc<Self> {
		let window = WindowBuilder::new();
		let window =
			ContextBuilder::new().with_gl_profile(GlProfile::Core).build_windowed(window, &event_loop).unwrap();
		let window = unsafe { window.make_current() }.unwrap();

		let gl = Gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);
		assert_eq!(unsafe { gl.GetError() }, 0);

		let mut uniform_align = 0;
		unsafe { gl.GetIntegerv(gl::UNIFORM_BUFFER_OFFSET_ALIGNMENT, &mut uniform_align) };

		Rc::new(Self { window, gl, uniform_align })
	}

	pub fn default_framebuffer(self: &Rc<Self>) -> DefaultFramebuffer {
		DefaultFramebuffer::new(self)
	}

	pub fn draw_elements_instanced<V: Vertex, I>(
		&self,
		indices: &dyn BufferSlice<u16>,
		vertices: &dyn BufferSlice<V>,
		instances: &dyn BufferSlice<I>,
	) {
		unsafe {
			self.gl.DrawElementsInstancedBaseVertexBaseInstance(
				gl::TRIANGLES,
				indices.len() as _,
				gl::UNSIGNED_SHORT,
				indices.offset() as _,
				1,
				(vertices.offset() as usize / size_of::<V>()) as _,
				(instances.offset() as usize / size_of::<I>()) as _,
			)
		};
	}

	pub fn flush(&self) {
		unsafe { self.gl.Flush() };
	}

	pub fn use_program(&self, program: &ShaderProgram) {
		unsafe { self.gl.UseProgram(program.handle()) };
	}

	pub fn bind_vertex_array(&self, vao: &VertexArray) {
		unsafe { self.gl.BindVertexArray(vao.handle()) };
	}

	pub fn window(&self) -> &ContextWrapper<PossiblyCurrent, Window> {
		&self.window
	}
}
