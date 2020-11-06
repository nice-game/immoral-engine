use crate::{
	buffer::{BufferSlice, DynamicBuffer},
	Ctx,
};
use gl::types::GLuint;
use std::{cell::Cell, marker::PhantomData, mem::size_of, rc::Rc, slice};

pub trait AllocatorAbstract {
	fn ctx(&self) -> &Rc<Ctx>;
	fn buffer(&self) -> &Rc<DynamicBuffer<[u8]>>;
}

pub struct Allocator<T, S = PackedStrategy> {
	pub buffer: Rc<DynamicBuffer<[u8]>>,
	free: Cell<usize>,
	strategy: PhantomData<(T, S)>,
}
impl<T: Copy + 'static, S: AllocatorStrategy> Allocator<T, S> {
	pub fn new(ctx: &Rc<Ctx>, size: usize) -> Rc<Self> {
		let buffer = unsafe { DynamicBuffer::uninitialized_slice(ctx, size * size_of::<T>()) };
		Rc::new(Self { buffer, free: Cell::new(0), strategy: PhantomData })
	}

	pub fn alloc_slice(self: &Rc<Self>, val: &[T]) -> Allocation<T> {
		let offset = self.free.get();
		let len = val.len();
		let size = len * size_of::<T>();
		self.free.set(offset + self.align(size));
		self.buffer.write_range(offset, unsafe { slice_as_bytes(val) });
		Allocation { alloc: self.clone(), offset, len, phantom: PhantomData }
	}

	fn align(&self, n: usize) -> usize {
		let align = S::align(self.buffer.ctx());
		(n + align - 1) / align * align
	}
}
impl<T: Copy + Default + 'static, S: AllocatorStrategy> Allocator<T, S> {
	pub fn alloc_default_slice(self: &Rc<Self>, len: usize) -> Allocation<T> {
		let data = vec![T::default(); len];
		self.alloc_slice(&data[..])
	}
}
impl<T, S> AllocatorAbstract for Allocator<T, S> {
	fn ctx(&self) -> &Rc<Ctx> {
		self.buffer.ctx()
	}

	fn buffer(&self) -> &Rc<DynamicBuffer<[u8]>> {
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
impl<T: Copy + 'static> BufferSlice<T> for Allocation<T> {
	fn ctx(&self) -> &Rc<Ctx> {
		self.alloc.ctx()
	}

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
