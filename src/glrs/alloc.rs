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
	pub vbo: GLuint,
	buf: &'static [u8],
	free: AtomicUsize,
}
impl Allocator {
	pub unsafe fn new(ctx: &Arc<Ctx>) -> Arc<Self> {
		let size = 32 * 1024 * 1024;

		let mut vbo = !0;
		ctx.gl.CreateBuffers(1, &mut vbo);
		ctx.gl.NamedBufferStorage(
			vbo,
			size,
			ptr::null(),
			gl::MAP_WRITE_BIT | gl::MAP_PERSISTENT_BIT | gl::MAP_COHERENT_BIT,
		);

		let buf =
			ctx.gl.MapNamedBufferRange(vbo, 0, size, gl::MAP_WRITE_BIT | gl::MAP_PERSISTENT_BIT | gl::MAP_COHERENT_BIT);
		let buf = slice::from_raw_parts(buf as *mut u8, size as _);

		Arc::new(Self { ctx: ctx.clone(), vbo, buf, free: AtomicUsize::new(0) })
	}

	pub fn alloc(self: &Arc<Self>, size: usize) -> Allocation {
		let offset = self.free.fetch_add(size, Ordering::Relaxed);
		if offset + size > self.buf.len() {}
		Allocation { alloc: self.clone(), offset, size }
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
