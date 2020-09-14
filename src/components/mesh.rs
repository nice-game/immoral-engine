use crate::Ctx;
use gl::types::GLuint;
use nalgebra::Vector2;
use specs::{prelude::*, Component};
use std::{mem::size_of, sync::Arc};

#[derive(Component)]
pub struct Mesh {
	_ctx: Arc<Ctx>,
	_vbo: GLuint,
	pub vao: GLuint,
}
impl Mesh {
	pub fn new(ctx: &Arc<Ctx>) -> Self {
		let vertices = [Vertex { pos: [-0.5, -0.5].into() }, Vertex { pos: [0.5, -0.5].into() }, Vertex {
			pos: [0.0, 0.5].into(),
		}];

		let mut vbo = 0;
		let mut vao = 0;
		unsafe {
			let size = (size_of::<Vertex>() * vertices.len()) as _;

			ctx.gl.CreateBuffers(1, &mut vbo);
			ctx.gl.NamedBufferData(vbo, size, [vertices.as_ptr()].as_ptr() as _, gl::STATIC_DRAW);

			ctx.gl.CreateVertexArrays(1, &mut vao);
			ctx.gl.EnableVertexArrayAttrib(vao, 0);
			ctx.gl.VertexArrayAttribFormat(vao, 0, 2, gl::FLOAT, gl::FALSE, 0);
			ctx.gl.VertexArrayAttribBinding(vao, 0, 0);
			ctx.gl.VertexArrayVertexBuffer(vao, 0, vbo, 0, size_of::<Vertex>() as _);
		}

		Self { _ctx: ctx.clone(), _vbo: vbo, vao }
	}
}

pub struct Vertex {
	#[allow(unused)]
	pos: Vector2<f32>,
}
