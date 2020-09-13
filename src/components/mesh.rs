use gl::types::GLuint;
use nalgebra::Vector2;
use specs::{prelude::*, Component};
use std::mem::size_of;

#[derive(Component)]
pub struct Mesh {
	pub vbo: GLuint,
}
impl Mesh {
	pub fn new() -> Self {
		let vertices = [Vertex { pos: [-0.5, -0.5].into() }, Vertex { pos: [0.5, -0.5].into() }, Vertex {
			pos: [0.0, 0.5].into(),
		}];

		let mut vbo = 0;
		unsafe {
			gl::GenBuffers(1, &mut vbo);
			gl::NamedBufferData(
				vbo,
				(size_of::<Vertex>() * vertices.len()) as _,
				&vertices as *const _ as _,
				gl::STATIC_DRAW,
			);
		}

		Self { vbo }
	}
}

pub struct Vertex {
	#[allow(unused)]
	pos: Vector2<f32>,
}
