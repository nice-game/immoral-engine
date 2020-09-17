use crate::{glrs::buffer::Buffer, systems::render::allocs::RenderAllocs};
use gl::types::GLint;
use nalgebra::Vector2;
use specs::{prelude::*, Component};
use std::{mem::size_of, sync::Arc};

#[derive(Component)]
pub struct Mesh {
	_buf: Buffer<[Vertex]>,
	indices: Buffer<[u16]>,
}
impl Mesh {
	pub fn new(alloc: &Arc<RenderAllocs>) -> Self {
		let vertices = [Vertex { pos: [-0.5, -0.5].into() }, Vertex { pos: [0.5, -0.5].into() }, Vertex {
			pos: [0.0, 0.5].into(),
		}];
		let indices = [0, 1, 2];

		let buf = alloc.alloc_verts(&vertices);
		let indices = alloc.alloc_indices(&indices);

		Self { _buf: buf, indices }
	}

	pub fn first(&self) -> GLint {
		(self.indices.mem.offset / size_of::<u16>()) as _
	}
}

#[derive(Clone, Copy)]
pub struct Vertex {
	#[allow(unused)]
	pos: Vector2<f32>,
}
