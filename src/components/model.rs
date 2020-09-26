use crate::{glrs::buffer::Buffer, systems::render::allocs::RenderAllocs};
use assimp::{Importer, Mesh as AssimpMesh, Scene, Vector3D};
use assimp_sys::{aiGetMaterialTexture, AiString, AiTextureType};
use gl::types::GLint;
use nalgebra::{UnitQuaternion, Vector3, Vector4};
use std::{
	iter::repeat,
	mem::size_of,
	path::Path,
	ptr, slice, str,
	sync::{atomic::Ordering, Arc},
};

pub struct Model {
	pub meshes: Vec<Mesh>,
}
impl Model {
	pub fn from_file(alloc: &Arc<RenderAllocs>, file: &str) -> Self {
		let mut importer = Importer::new();
		importer.triangulate(true);
		let scene = importer.read_file(file).unwrap();

		let dir = Path::new(file).parent().unwrap();
		let texidxs = get_textures(&dir, &scene, alloc);

		let meshes = scene.mesh_iter().map(|mesh| Mesh::from_assimp(alloc, &mesh, &texidxs)).collect();

		Self { meshes }
	}
}

fn get_textures(file: &Path, scene: &Scene, alloc: &RenderAllocs) -> Vec<i32> {
	let gl = &alloc.ctx().gl;
	unsafe { gl.BindBuffer(gl::PIXEL_UNPACK_BUFFER, alloc.other_alloc.id) };

	scene
		.material_iter()
		.map(|m| unsafe {
			let mut path = AiString::default();
			aiGetMaterialTexture(
				&*m,
				AiTextureType::Diffuse,
				0,
				&mut path,
				ptr::null(),
				ptr::null_mut(),
				ptr::null_mut(),
				ptr::null_mut(),
				ptr::null_mut(),
				ptr::null_mut(),
			);
			str::from_utf8_unchecked(slice::from_raw_parts(path.data.as_ptr(), path.length)).to_owned()
		})
		.map(|p| {
			if p.len() > 0 {
				let path = file.join(p);
				println!("{}", path.display());
				let img = image::open(path).unwrap().to_rgba();
				let (w, h) = img.dimensions();
				let buf = alloc.alloc_other_slice(&img.into_raw());
				let idx = alloc.tex_free.fetch_add(1, Ordering::Relaxed);
				unsafe {
					gl.TextureSubImage3D(
						alloc.tex,
						0,
						0,
						0,
						idx,
						w as _,
						h as _,
						1,
						gl::RGBA,
						gl::UNSIGNED_BYTE,
						buf.offset() as _,
					)
				};
				idx
			} else {
				-1
			}
		})
		.collect()
}

pub struct Mesh {
	pub buf: Buffer<[Vertex]>,
	indices: Buffer<[u16]>,
	// -1 if no texture
	pub texi: i32,
}
impl Mesh {
	fn from_assimp(alloc: &Arc<RenderAllocs>, mesh: &AssimpMesh, texidxs: &[i32]) -> Self {
		let texcoords = if mesh.get_num_uv_channels() > 1 {
			Box::new(mesh.texture_coords_iter(1)) as Box<dyn Iterator<Item = Vector3D>>
		} else {
			Box::new(repeat(Vector3D::new(0.0, 0.0, 0.0)))
		};

		let vertices: Vec<_> = mesh
			.vertex_iter()
			.zip(mesh.normal_iter())
			.zip(mesh.texture_coords_iter(0))
			.zip(texcoords)
			.map(|(((v, n), u), l)| Vertex {
				pos: [v.x, v.y, v.z].into(),
				rot: UnitQuaternion::rotation_between(&[0.0, 0.0, 1.0].into(), &[n.x, n.y, n.z].into()).unwrap(),
				uvw: [u.x, u.y, l.x, l.y].into(),
			})
			.collect();
		let indices: Vec<_> = mesh
			.face_iter()
			.map(|f| {
				assert_eq!(f.num_indices, 3);
				(0..f.num_indices).map(move |i| f[i as _] as _)
			})
			.flatten()
			.collect();

		let buf = alloc.alloc_verts(&vertices);
		let indices = alloc.alloc_indices(&indices);

		Self { buf, indices, texi: texidxs[mesh.material_index as usize] }
	}

	pub fn index_offset(&self) -> GLint {
		(self.indices.mem.offset / size_of::<u16>()) as _
	}

	pub fn index_count(&self) -> usize {
		self.indices.len()
	}
}

#[allow(unused)]
#[derive(Clone, Copy)]
pub struct Vertex {
	pos: Vector3<f32>,
	rot: UnitQuaternion<f32>,
	uvw: Vector4<f32>,
}

#[allow(unused)]
#[derive(Clone, Copy)]
pub struct VertexRigged {
	pos: Vector3<f32>,
	rot: UnitQuaternion<f32>,
	uvw: Vector4<f32>,
	bone_id: Vector4<u8>,
	bone_wt: Vector4<u8>,
}
