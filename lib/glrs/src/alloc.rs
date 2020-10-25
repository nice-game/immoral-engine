use crate::{
	buffer::{Buffer, BufferSlice},
	Ctx,
};
use gl::types::GLuint;
use std::{cell::Cell, marker::PhantomData, mem::size_of, rc::Rc, slice};

pub trait AllocatorAbstract {
	fn buffer(&self) -> &Buffer<[u8]>;
}

pub struct Allocator<T, S = PackedStrategy> {
	pub buffer: Rc<Buffer<[u8]>>,
	free: Cell<usize>,
	strategy: PhantomData<(T, S)>,
}
impl<T: 'static, S: AllocatorStrategy> Allocator<T, S> {
	pub fn new(ctx: &Rc<Ctx>, size: usize) -> Rc<Self> {
		let buffer = unsafe { Buffer::uninitialized_slice(ctx, size) };
		Rc::new(Self { buffer, free: Cell::new(0), strategy: PhantomData })
	}

	pub fn alloc_slice(self: &Rc<Self>, val: &[T]) -> Allocation<T> {
		let offset = self.free.get();
		let len = val.len();
		let size = len * size_of::<T>();
		self.free.set(offset + self.align(size));
		self.buffer.map_range_mut(offset, size, |x| x.copy_from_slice(unsafe { slice_as_bytes(val) }));
		Allocation { alloc: self.clone(), offset, len, phantom: PhantomData }
	}

	pub fn handle(&self) -> GLuint {
		self.buffer.handle()
	}

	pub fn ctx(&self) -> &Rc<Ctx> {
		self.buffer.ctx()
	}

	fn align(&self, n: usize) -> usize {
		let align = S::align(self.buffer.ctx());
		(n + align - 1) / align * align
	}
}
impl<T, S> AllocatorAbstract for Allocator<T, S> {
	fn buffer(&self) -> &Buffer<[u8]> {
		&self.buffer
	}
}

pub trait AllocatorStrategy: 'static {
	fn align(ctx: &Ctx) -> usize;
}
pub struct PackedStrategy;
impl AllocatorStrategy for PackedStrategy {
	fn align(_ctx: &Ctx) -> usize {
		1
	}
}
pub struct UniformStrategy;
impl AllocatorStrategy for UniformStrategy {
	fn align(ctx: &Ctx) -> usize {
		ctx.uniform_align as _
	}
}

pub struct Allocation<T> {
	pub alloc: Rc<dyn AllocatorAbstract>,
	pub offset: usize,
	pub len: usize,
	phantom: PhantomData<T>,
}
impl<T> Allocation<T> {
	// pub fn buf(&self) -> &[u8] {
	// 	unsafe { slice::from_raw_parts_mut(self.alloc.buf.as_ptr().add(self.offset as _) as _, self.size) }
	// }

	// pub fn buf_mut(&mut self) -> &mut [u8] {
	// 	unsafe { slice::from_raw_parts_mut(self.alloc.buf.as_ptr().add(self.offset as _) as _, self.size) }
	// }
}
impl<T: Copy + 'static> BufferSlice<T> for Allocation<T> {
	fn handle(&self) -> GLuint {
		self.alloc.buffer().handle()
	}

	fn offset(&self) -> usize {
		self.offset
	}

	fn len(&self) -> usize {
		self.len
	}
}

unsafe fn slice_as_bytes<T>(p: &[T]) -> &[u8] {
	slice::from_raw_parts(p as *const _ as *const u8, p.len() * size_of::<T>())
}
