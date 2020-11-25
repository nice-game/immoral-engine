use crate::{components::model::Instance, systems::render::Vertex};
use glrs::{
	alloc::{Allocation, Allocator, AllocatorAbstract},
	texture::{Filter, Texture2DArray, TextureAbstract},
	Ctx,
};
use std::{rc::Rc, slice, sync::atomic::AtomicI32};

pub struct RenderAllocs {
	pub vert_alloc: Rc<Allocator<Vertex>>,
	pub idx_alloc: Rc<Allocator<u16>>,
	pub instance_alloc: Rc<Allocator<Instance>>,
	pub tex: Texture2DArray,
	// TODO: make non-atomic, since this struct is !Sync
	pub tex_free: AtomicI32,
}
impl RenderAllocs {
	pub fn new(ctx: &Rc<Ctx>) -> Rc<Self> {
		let tex = Texture2DArray::new(ctx, [1024, 1024, 64].into());
		tex.min_filter(Filter::Linear);
		tex.mag_filter(Filter::Linear);

		let size = 32 * 1024 * 1024;

		Rc::new(Self {
			vert_alloc: Allocator::new(ctx, size),
			idx_alloc: Allocator::new(ctx, size),
			instance_alloc: Allocator::new(ctx, size),
			tex,
			tex_free: AtomicI32::default(),
		})
	}

	pub fn alloc_verts(&self, verts: &[Vertex]) -> Allocation<Vertex> {
		self.vert_alloc.alloc_slice(verts)
	}

	pub fn alloc_indices(&self, indices: &[u16]) -> Allocation<u16> {
		self.idx_alloc.alloc_slice(indices)
	}

	pub fn alloc_instance(&self, instance: &Instance) -> Allocation<Instance> {
		self.instance_alloc.alloc_slice(slice::from_ref(instance))
	}

	pub fn ctx(&self) -> &Rc<Ctx> {
		self.vert_alloc.ctx()
	}
}
