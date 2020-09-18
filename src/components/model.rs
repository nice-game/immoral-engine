use crate::{glrs::buffer::Buffer, systems::render::allocs::RenderAllocs};
use assimp::{Importer, Mesh as AssimpMesh};
use gl::types::GLint;
use nalgebra::Vector3;
use specs::{prelude::*, Component};
use std::{mem::size_of, sync::Arc};

#[derive(Component)]
pub struct Model {
	pub meshes: Vec<Mesh>,
}
impl Model {
	pub fn from_file(alloc: &Arc<RenderAllocs>, file: &str) -> Self {
		let scene = Importer::new().read_file(file).unwrap();
		let meshes = scene.mesh_iter().map(|mesh| Mesh::from_assimp(alloc, &mesh)).collect();
		Self { meshes }
	}
}

pub struct Mesh {
	_buf: Buffer<[Vertex]>,
	indices: Buffer<[u16]>,
}
impl Mesh {
	fn from_assimp(alloc: &Arc<RenderAllocs>, mesh: &AssimpMesh) -> Self {
		let vertices: Vec<_> = mesh.vertex_iter().map(|v| Vertex { pos: [v.x, v.y, v.z].into() }).collect();
		let indices: Vec<_> =
			mesh.face_iter().map(|f| (0..f.num_indices).map(move |i| f[i as _] as _)).flatten().collect();

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
	pos: Vector3<f32>,
}
