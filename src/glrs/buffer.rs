use crate::glrs::alloc::{Allocation, Allocator};
use std::{marker::PhantomData, mem::size_of, slice, sync::Arc};

pub struct Buffer<T: ?Sized> {
	pub mem: Allocation,
	phantom: PhantomData<T>,
}
impl<T: Copy + 'static> Buffer<T> {
	pub fn init(alloc: &Arc<Allocator>) -> BufferInit<T> {
		let mem = alloc.alloc(size_of::<T>());
		BufferInit { buf: Self { mem, phantom: PhantomData } }
	}

	pub fn copy(&mut self, data: &T) {
		let data = unsafe { slice::from_raw_parts(data as *const _ as _, size_of::<T>()) };
		self.mem.buf_mut().copy_from_slice(data);
	}
}
impl<T: Copy + 'static> Buffer<[T]> {
	pub fn init_slice(alloc: &Arc<Allocator>, len: usize) -> BufferInit<[T]> {
		let mem = alloc.alloc(size_of::<T>() * len);
		BufferInit { buf: Self { mem, phantom: PhantomData } }
	}

	pub fn len(&self) -> usize {
		self.mem.size / size_of::<T>()
	}
}
impl<T: ?Sized> Buffer<T> {
	pub fn offset(&self) -> isize {
		self.mem.offset as _
	}
}

pub struct BufferInit<T: ?Sized> {
	buf: Buffer<T>,
}
impl<T: Copy + 'static> BufferInit<T> {
	pub fn copy(mut self, data: &T) -> Buffer<T> {
		self.buf.copy(data);
		self.buf
	}
}
impl<T: Copy + 'static> BufferInit<[T]> {
	pub fn copy_from_slice(mut self, data: &[T]) -> Buffer<[T]> {
		let data = unsafe { slice::from_raw_parts(data.as_ptr() as _, size_of::<T>() * data.len()) };
		self.buf.mem.buf_mut().copy_from_slice(data); // asserts that slices are the same length, so we don't have to
		self.buf
	}
}
