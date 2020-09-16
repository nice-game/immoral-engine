use crate::{glrs::buffer::Buffer, Allocator};
use gl::types::GLuint;
use nalgebra::Vector2;
use specs::{prelude::*, Component};
use std::{mem::size_of, sync::Arc};

#[derive(Component)]
pub struct Mesh {
	_buf: Buffer<[Vertex]>,
	pub vao: GLuint,
}
impl Mesh {
	pub fn new(alloc: &Arc<Allocator>) -> Self {
		let vertices = [Vertex { pos: [-0.5, -0.5].into() }, Vertex { pos: [0.5, -0.5].into() }, Vertex {
			pos: [0.0, 0.5].into(),
		}];

		let buf = Buffer::init_slice(alloc, vertices.len()).copy_from_slice(&vertices);

		let mut vao = 0;
		unsafe {
			// let size = (size_of::<Vertex>() * vertices.len()) as _;

			// ctx.gl.CreateBuffers(1, &mut vbo);
			// ctx.gl.NamedBufferData(vbo, size, vertices.as_ptr() as _, gl::STATIC_DRAW);

			let gl = &alloc.ctx.gl;
			gl.CreateVertexArrays(1, &mut vao);
			gl.EnableVertexArrayAttrib(vao, 0);
			gl.VertexArrayAttribFormat(vao, 0, 2, gl::FLOAT, gl::FALSE, 0);
			gl.VertexArrayAttribBinding(vao, 0, 0);
			gl.VertexArrayVertexBuffer(vao, 0, buf.vbo(), buf.offset(), size_of::<Vertex>() as _);
		}

		Self { _buf: buf, vao }
	}
}

#[derive(Clone, Copy)]
pub struct Vertex {
	#[allow(unused)]
	pos: Vector2<f32>,
}
