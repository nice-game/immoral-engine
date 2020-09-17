use crate::Ctx;
use gl::types::GLuint;
use std::{
	ptr, slice,
	sync::{
		atomic::{AtomicUsize, Ordering},
		Arc,
	},
};

pub struct Allocator {
	pub ctx: Arc<Ctx>,
	pub id: GLuint,
	buf: &'static [u8],
	align: usize,
	free: AtomicUsize,
}
impl Allocator {
	pub unsafe fn new(ctx: &Arc<Ctx>) -> Arc<Self> {
		let size = 32 * 1024 * 1024;

		let mut id = !0;
		ctx.gl.CreateBuffers(1, &mut id);
		ctx.gl.NamedBufferStorage(
			id,
			size,
			ptr::null(),
			gl::MAP_WRITE_BIT | gl::MAP_PERSISTENT_BIT | gl::MAP_COHERENT_BIT,
		);

		let buf =
			ctx.gl.MapNamedBufferRange(id, 0, size, gl::MAP_WRITE_BIT | gl::MAP_PERSISTENT_BIT | gl::MAP_COHERENT_BIT);
		let buf = slice::from_raw_parts(buf as *mut u8, size as _);

		let mut align = 0;
		ctx.gl.GetIntegerv(gl::UNIFORM_BUFFER_OFFSET_ALIGNMENT, &mut align);

		Arc::new(Self { ctx: ctx.clone(), id, buf, align: align as _, free: AtomicUsize::new(0) })
	}

	pub fn alloc(self: &Arc<Self>, size: usize) -> Allocation {
		let offset = self.free.fetch_add(self.round_up(size), Ordering::Relaxed);
		Allocation { alloc: self.clone(), offset, size }
	}

	fn round_up(&self, n: usize) -> usize {
		(n + self.align - 1) / self.align * self.align
	}
}

pub struct Allocation {
	pub alloc: Arc<Allocator>,
	pub offset: usize,
	size: usize,
}
impl Allocation {
	pub fn buf_mut(&mut self) -> &mut [u8] {
		unsafe { slice::from_raw_parts_mut(self.alloc.buf.as_ptr().add(self.offset as _) as _, self.size) }
	}
}
