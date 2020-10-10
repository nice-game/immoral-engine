use crate::glrs::ctx::Ctx;
use gl::types::GLuint;
use std::{
	ptr,
	rc::Rc,
	slice,
	sync::atomic::{AtomicUsize, Ordering},
};

pub struct Allocator {
	pub ctx: Rc<Ctx>,
	pub id: GLuint,
	buf: &'static [u8],
	align: usize,
	free: AtomicUsize,
}
impl Allocator {
	pub fn new(ctx: &Rc<Ctx>, pack: bool) -> Rc<Self> {
		let size = 32 * 1024 * 1024;

		let mut id = !0;
		let mut align = 1;
		let buf;
		unsafe {
			ctx.gl.CreateBuffers(1, &mut id);
			ctx.gl.NamedBufferStorage(
				id,
				size,
				ptr::null(),
				gl::MAP_WRITE_BIT | gl::MAP_PERSISTENT_BIT | gl::MAP_COHERENT_BIT,
			);

			let bufptr = ctx.gl.MapNamedBufferRange(
				id,
				0,
				size,
				gl::MAP_WRITE_BIT | gl::MAP_PERSISTENT_BIT | gl::MAP_COHERENT_BIT,
			);
			buf = slice::from_raw_parts(bufptr as *mut u8, size as _);

			if !pack {
				ctx.gl.GetIntegerv(gl::UNIFORM_BUFFER_OFFSET_ALIGNMENT, &mut align);
			}
		}

		Rc::new(Self { ctx: ctx.clone(), id, buf, align: align as _, free: AtomicUsize::new(0) })
	}

	pub fn alloc(self: &Rc<Self>, size: usize) -> Allocation {
		let offset = self.free.fetch_add(self.round_up(size), Ordering::Relaxed);
		Allocation { alloc: self.clone(), offset, size }
	}

	fn round_up(&self, n: usize) -> usize {
		(n + self.align - 1) / self.align * self.align
	}
}
impl Drop for Allocator {
	fn drop(&mut self) {
		unsafe { self.ctx.gl.DeleteBuffers(1, &self.id) };
	}
}

pub struct Allocation {
	pub alloc: Rc<Allocator>,
	pub offset: usize,
	pub size: usize,
}
impl Allocation {
	pub fn buf(&self) -> &[u8] {
		unsafe { slice::from_raw_parts_mut(self.alloc.buf.as_ptr().add(self.offset as _) as _, self.size) }
	}

	pub fn buf_mut(&mut self) -> &mut [u8] {
		unsafe { slice::from_raw_parts_mut(self.alloc.buf.as_ptr().add(self.offset as _) as _, self.size) }
	}
}
