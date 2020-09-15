use crate::glrs::alloc::{Allocation, Allocator};
use gl::types::GLuint;
use std::{marker::PhantomData, mem::size_of, slice, sync::Arc};

pub struct Buffer<T: ?Sized> {
	mem: Allocation,
	phantom: PhantomData<T>,
}
impl<T> Buffer<[T]> {
	pub fn init_slice(alloc: &Arc<Allocator>, len: usize) -> BufferInit<[T]> {
		let size = size_of::<T>() * len;
		let mem = alloc.alloc(size);

		BufferInit { buf: Self { mem, phantom: PhantomData } }
	}

	pub fn vbo(&self) -> GLuint {
		self.mem.alloc.vbo
	}

	pub fn offset(&self) -> isize {
		self.mem.offset as _
	}
}

pub struct BufferInit<T: ?Sized> {
	buf: Buffer<T>,
}
impl<T: Copy + 'static> BufferInit<[T]> {
	pub fn copy_from_slice(mut self, data: &[T]) -> Arc<Buffer<[T]>> {
		let data = unsafe { slice::from_raw_parts(data.as_ptr() as _, size_of::<T>() * data.len()) };
		let buf = self.buf.mem.buf_mut();
		buf.copy_from_slice(data);
		Arc::new(self.buf)
	}
}
