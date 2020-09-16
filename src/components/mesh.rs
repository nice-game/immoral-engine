use crate::{glrs::buffer::Buffer, Allocator};
use gl::types::GLuint;
use nalgebra::Vector2;
use specs::{prelude::*, Component};
use std::{mem::size_of, sync::Arc};

#[derive(Component)]
pub struct Mesh {
	buf: Buffer<[Vertex]>,
}
impl Mesh {
	pub fn new(alloc: &Arc<Allocator>) -> Self {
		let vertices = [Vertex { pos: [-0.5, -0.5].into() }, Vertex { pos: [0.5, -0.5].into() }, Vertex {
			pos: [0.0, 0.5].into(),
		}];

		let buf = Buffer::init_slice(alloc, vertices.len()).copy_from_slice(&vertices);

		Self { buf }
	}

	pub unsafe fn bind(&self, vao: GLuint) {
		self.buf.mem.alloc.ctx.gl.VertexArrayVertexBuffer(
			vao,
			0,
			self.buf.vbo(),
			self.buf.offset(),
			size_of::<Vertex>() as _,
		);
	}
}

#[derive(Clone, Copy)]
pub struct Vertex {
	#[allow(unused)]
	pos: Vector2<f32>,
}
