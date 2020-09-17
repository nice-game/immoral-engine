use crate::{glrs::buffer::Buffer, systems::render::allocs::RenderAllocs};
use gl::types::GLint;
use nalgebra::Vector2;
use specs::{prelude::*, Component};
use std::{mem::size_of, sync::Arc};

#[derive(Component)]
pub struct Mesh {
	buf: Buffer<[Vertex]>,
}
impl Mesh {
	pub fn new(alloc: &Arc<RenderAllocs>) -> Self {
		let vertices = [Vertex { pos: [-0.5, -0.5].into() }, Vertex { pos: [0.5, -0.5].into() }, Vertex {
			pos: [0.0, 0.5].into(),
		}];

		let buf = alloc.alloc_verts(&vertices);

		Self { buf }
	}

	pub fn first(&self) -> GLint {
		(self.buf.mem.offset / size_of::<Vertex>()) as _
	}
}

#[derive(Clone, Copy)]
pub struct Vertex {
	#[allow(unused)]
	pos: Vector2<f32>,
}
