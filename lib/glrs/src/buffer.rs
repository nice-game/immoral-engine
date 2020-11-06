use crate::Ctx;
use gl::{
	types::{GLbitfield, GLsizeiptr, GLuint},
	Gl,
};
use std::{ffi::c_void, marker::PhantomData, mem::size_of, ptr, rc::Rc};

pub type DynamicBuffer<T> = Buffer<T, true>;
pub type ImmutableBuffer<T> = Buffer<T, false>;

pub struct Buffer<T: ?Sized, const DYN: bool> {
	ctx: Rc<Ctx>,
	handle: GLuint,
	len: usize,
	phantom: PhantomData<T>,
}
impl<T: Copy + 'static, const DYN: bool> Buffer<T, DYN> {
	// pub fn map_mut(&self, cb: impl FnOnce(&mut T)) {
	// 	let gl = &self.ctx.gl;
	// 	let refr = unsafe {
	// 		let ptr = gl.MapNamedBuffer(self.handle, gl::WRITE_ONLY);
	// 		&mut *(ptr as *mut _)
	// 	};
	// 	cb(refr);
	// 	unsafe { gl.UnmapNamedBuffer(self.handle) };
	// }

	pub unsafe fn uninitialized(ctx: &Rc<Ctx>) -> Rc<Self> {
		let gl = &ctx.gl;
		let flags = storage_flags(DYN);
		let handle = create_buffer_with_storage(gl, size_of::<T>() as _, ptr::null(), flags);
		Rc::new(Self { ctx: ctx.clone(), handle, len: 1, phantom: PhantomData })
	}

	pub fn from_val(ctx: &Rc<Ctx>, val: &T) -> Rc<Self> {
		let gl = &ctx.gl;
		let flags = storage_flags(DYN);
		let handle = unsafe { create_buffer_with_storage(gl, size_of::<T>() as _, val as *const _ as _, flags) };
		Rc::new(Self { ctx: ctx.clone(), handle, len: 1, phantom: PhantomData })
	}
}
impl<T: Copy + 'static> Buffer<T, true> {
	pub fn write(&self, val: &T) {
		unsafe { self.ctx.gl.NamedBufferSubData(self.handle, 0, size_of::<T>() as _, val as *const _ as _) };
	}
}
impl<T: Copy + 'static, const DYN: bool> Buffer<[T], DYN> {
	pub unsafe fn uninitialized_slice(ctx: &Rc<Ctx>, len: usize) -> Rc<Self> {
		let gl = &ctx.gl;
		let flags = storage_flags(DYN);
		let handle = create_buffer_with_storage(gl, (len * size_of::<T>()) as _, ptr::null(), flags);
		Rc::new(Self { ctx: ctx.clone(), handle, len, phantom: PhantomData })
	}

	// pub fn map_range_mut(&self, offset: usize, len: usize, cb: impl FnOnce(&mut [T])) {
	// 	let gl = &self.ctx.gl;
	// 	let offset = (offset * size_of::<T>()) as _;
	// 	let size = (len * size_of::<T>()) as _;
	// 	let refr = unsafe {
	// 		let ptr = gl.MapNamedBufferRange(self.handle, offset, size, gl::MAP_WRITE_BIT);
	// 		slice::from_raw_parts_mut(ptr as *mut T, len)
	// 	};
	// 	cb(refr);
	// 	unsafe { gl.UnmapNamedBuffer(self.handle) };
	// }

	pub fn from_slice(ctx: &Rc<Ctx>, val: &[T]) -> Rc<Self> {
		let gl = &ctx.gl;
		let len = val.len();
		let flags = storage_flags(DYN);
		let handle = unsafe { create_buffer_with_storage(gl, (len * size_of::<T>()) as _, val.as_ptr() as _, flags) };
		Rc::new(Self { ctx: ctx.clone(), handle, len, phantom: PhantomData })
	}

	pub fn len(&self) -> usize {
		self.len
	}
}
impl<T: Copy + Default + 'static, const DYN: bool> Buffer<[T], DYN> {
	pub fn default_slice(ctx: &Rc<Ctx>, len: usize) -> Rc<Self> {
		let data = vec![T::default(); len];
		Self::from_slice(ctx, &data[..])
	}
}
impl<T: ?Sized, const DYN: bool> Drop for Buffer<T, DYN> {
	fn drop(&mut self) {
		unsafe { self.ctx.gl.DeleteBuffers(1, &self.handle) };
	}
}
impl<T, const DYN: bool> BufferSlice<T> for Rc<Buffer<T, DYN>> {
	fn ctx(&self) -> &Rc<Ctx> {
		&self.ctx
	}

	fn handle(&self) -> GLuint {
		self.handle
	}

	fn offset(&self) -> usize {
		0
	}

	fn len(&self) -> usize {
		self.len
	}
}
impl<T, const DYN: bool> BufferSlice<T> for Rc<Buffer<[T], DYN>> {
	fn ctx(&self) -> &Rc<Ctx> {
		&self.ctx
	}

	fn handle(&self) -> GLuint {
		self.handle
	}

	fn offset(&self) -> usize {
		0
	}

	fn len(&self) -> usize {
		self.len
	}
}

pub trait BufferSlice<T> {
	fn ctx(&self) -> &Rc<Ctx>;
	fn handle(&self) -> GLuint;
	fn offset(&self) -> usize;
	fn len(&self) -> usize;

	fn write_range(&self, offset: usize, val: &[T]) {
		let offset = offset * size_of::<T>() + self.offset();
		let size = val.len() * size_of::<T>();
		unsafe { self.ctx().gl.NamedBufferSubData(self.handle(), offset as _, size as _, val.as_ptr() as _) };
	}
}

unsafe fn create_buffer_with_storage(gl: &Gl, size: GLsizeiptr, data: *const c_void, flags: GLbitfield) -> GLuint {
	let mut handle = 0;
	gl.CreateBuffers(1, &mut handle);
	gl.NamedBufferStorage(handle, size, data, flags);
	handle
}

const fn storage_flags(dynamic: bool) -> GLbitfield {
	let mut ret = 0;
	if dynamic {
		ret |= gl::DYNAMIC_STORAGE_BIT;
	}
	ret
}
