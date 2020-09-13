use crate::Ctx;
use gl::types::GLuint;
use nalgebra::Vector2;
use specs::{prelude::*, Component};
use std::{mem::size_of, sync::Arc};

#[derive(Component)]
pub struct Mesh {
	_ctx: Arc<Ctx>,
	pub vbo: GLuint,
}
impl Mesh {
	pub fn new(ctx: &Arc<Ctx>) -> Self {
		let vertices = [Vertex { pos: [-0.5, -0.5].into() }, Vertex { pos: [0.5, -0.5].into() }, Vertex {
			pos: [0.0, 0.5].into(),
		}];

		let mut vbo = 0;
		unsafe {
			ctx.gl.GenBuffers(1, &mut vbo);
			ctx.gl.NamedBufferData(
				vbo,
				(size_of::<Vertex>() * vertices.len()) as _,
				&vertices as *const _ as _,
				gl::STATIC_DRAW,
			);
		}

		Self { _ctx: ctx.clone(), vbo }
	}
}

pub struct Vertex {
	#[allow(unused)]
	pos: Vector2<f32>,
}
