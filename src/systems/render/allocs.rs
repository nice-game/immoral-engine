use crate::{components::model::Instance, systems::render::Vertex};
use glrs::{
	alloc::Allocator,
	buffer::Buffer,
	texture::{Filter, Texture, Texture3D},
	Ctx,
};
use std::{rc::Rc, sync::atomic::AtomicI32};

pub struct RenderAllocs {
	pub vert_alloc: Rc<Allocator>,
	pub idx_alloc: Rc<Allocator>,
	pub instance_alloc: Rc<Allocator>,
	pub other_alloc: Rc<Allocator>,
	pub tex: Texture3D,
	pub tex_free: AtomicI32,
}
impl RenderAllocs {
	pub fn new(ctx: &Rc<Ctx>) -> Rc<Self> {
		let tex = Texture3D::new(ctx, 1024, 1024, 64);
		tex.min_filter(Filter::Linear);
		tex.mag_filter(Filter::Linear);

		Rc::new(Self {
			vert_alloc: Allocator::new(ctx, true),
			idx_alloc: Allocator::new(ctx, true),
			instance_alloc: Allocator::new(ctx, true),
			other_alloc: Allocator::new(ctx, false),
			tex,
			tex_free: AtomicI32::default(),
		})
	}

	pub fn alloc_verts(&self, vertices: &[Vertex]) -> Buffer<[Vertex]> {
		Buffer::init_slice(&self.vert_alloc, vertices.len()).copy_from_slice(vertices)
	}

	pub fn alloc_indices(&self, indices: &[u16]) -> Buffer<[u16]> {
		Buffer::init_slice(&self.idx_alloc, indices.len()).copy_from_slice(indices)
	}

	pub fn alloc_instance(&self, instance: &Instance) -> Buffer<Instance> {
		Buffer::init(&self.instance_alloc).copy(instance)
	}

	pub fn alloc_other<T: Copy + 'static>(&self, data: &T) -> Buffer<T> {
		Buffer::init(&self.other_alloc).copy(&data)
	}

	pub fn alloc_other_slice<T: Copy + 'static>(&self, data: &[T]) -> Buffer<[T]> {
		Buffer::init_slice(&self.other_alloc, data.len()).copy_from_slice(data)
	}

	pub fn ctx(&self) -> &Rc<Ctx> {
		&self.vert_alloc.ctx
	}
}
