use crate::{
	systems::render::{Buffer, Vertex},
	Allocator, Ctx,
};
use std::sync::Arc;

pub struct RenderAllocs {
	pub(super) vert_alloc: Arc<Allocator>,
	pub(super) idx_alloc: Arc<Allocator>,
	pub(super) other_alloc: Arc<Allocator>,
}
impl RenderAllocs {
	pub fn new(ctx: &Arc<Ctx>) -> Arc<Self> {
		Arc::new(Self {
			vert_alloc: Allocator::new(ctx, true),
			idx_alloc: Allocator::new(ctx, true),
			other_alloc: Allocator::new(ctx, false),
		})
	}

	pub fn alloc_verts(&self, vertices: &[Vertex]) -> Buffer<[Vertex]> {
		Buffer::init_slice(&self.vert_alloc, vertices.len()).copy_from_slice(&vertices)
	}

	pub fn alloc_indices(&self, indices: &[u16]) -> Buffer<[u16]> {
		Buffer::init_slice(&self.idx_alloc, indices.len()).copy_from_slice(&indices)
	}

	pub fn alloc_other<T: Copy + 'static>(&self, data: &T) -> Buffer<T> {
		Buffer::init(&self.other_alloc).copy(&data)
	}

	pub fn alloc_other_slice<T: Copy + 'static>(&self, data: &[T]) -> Buffer<[T]> {
		Buffer::init_slice(&self.other_alloc, data.len()).copy_from_slice(data)
	}

	pub fn ctx(&self) -> &Arc<Ctx> {
		&self.vert_alloc.ctx
	}
}
