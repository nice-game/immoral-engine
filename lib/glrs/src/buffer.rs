use crate::Ctx;
use gl::{
	types::{GLsizeiptr, GLuint},
	Gl,
};
use std::{marker::PhantomData, mem::size_of, ptr, rc::Rc, slice};

pub struct Buffer<T: ?Sized> {
	ctx: Rc<Ctx>,
	handle: GLuint,
	len: usize,
	phantom: PhantomData<T>,
}
impl<T> Buffer<T> {
	pub fn map_mut(&self, cb: impl FnOnce(&mut T)) {
		let gl = &self.ctx.gl;
		let refr = unsafe {
			let ptr = gl.MapNamedBuffer(self.handle, gl::WRITE_ONLY);
			&mut *(ptr as *mut _)
		};
		cb(refr);
		unsafe { gl.UnmapNamedBuffer(self.handle) };
	}
}
impl<T: Copy + 'static> Buffer<T> {
	// TODO: add uninitialized() constructor

	pub fn from_val(ctx: &Rc<Ctx>, val: &T) -> Rc<Buffer<T>> {
		let gl = &ctx.gl;
		let size = size_of::<T>() as _;
		let handle;
		unsafe {
			handle = create_buffer_with_storage(gl, size);
			map_buf_as(gl, handle, |x| *x = val);
		}
		Rc::new(Self { ctx: ctx.clone(), handle, len: 1, phantom: PhantomData })
	}
}
impl<T> Buffer<[T]> {
	pub unsafe fn uninitialized_slice(ctx: &Rc<Ctx>, len: usize) -> Rc<Buffer<[T]>> {
		let gl = &ctx.gl;
		let handle = create_buffer_with_storage(gl, (len * size_of::<T>()) as _);
		Rc::new(Self { ctx: ctx.clone(), handle, len, phantom: PhantomData })
	}

	pub fn map_range_mut(&self, offset: usize, len: usize, cb: impl FnOnce(&mut [T])) {
		let gl = &self.ctx.gl;
		let offset = (offset * size_of::<T>()) as _;
		let size = (len * size_of::<T>()) as _;
		let refr = unsafe {
			let ptr = gl.MapNamedBufferRange(self.handle, offset, size, gl::MAP_WRITE_BIT);
			slice::from_raw_parts_mut(ptr as *mut T, len)
		};
		cb(refr);
		unsafe { gl.UnmapNamedBuffer(self.handle) };
	}
}
impl<T: Copy + 'static> Buffer<[T]> {
	pub fn from_slice(ctx: &Rc<Ctx>, val: &[T]) -> Rc<Buffer<[T]>> {
		let ret = unsafe { Self::uninitialized_slice(ctx, val.len()) };
		ret.map_range_mut(0, val.len(), |x| x.copy_from_slice(val));
		ret
	}
}
impl<T: Copy + 'static> Buffer<[T]> {
	pub fn len(&self) -> usize {
		self.len
	}
}
impl<T: ?Sized> Buffer<T> {
	pub fn ctx(&self) -> &Rc<Ctx> {
		&self.ctx
	}

	pub fn handle(&self) -> GLuint {
		self.handle
	}
}
impl<T: ?Sized> Drop for Buffer<T> {
	fn drop(&mut self) {
		unsafe { self.ctx.gl.DeleteBuffers(1, &self.handle) };
	}
}
impl<T> BufferSlice<T> for Rc<Buffer<[T]>> {
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
	fn handle(&self) -> GLuint;
	fn offset(&self) -> usize;
	fn len(&self) -> usize;
}

unsafe fn create_buffer_with_storage(gl: &Gl, size: GLsizeiptr) -> GLuint {
	let mut handle = 0;
	gl.CreateBuffers(1, &mut handle);
	gl.NamedBufferStorage(handle, size, ptr::null(), gl::MAP_WRITE_BIT);
	handle
}
unsafe fn map_buf_as<T>(gl: &Gl, handle: GLuint, cb: impl FnOnce(&mut T)) {
	let ptr = gl.MapNamedBufferRange(handle, 0, size_of::<T>() as _, gl::MAP_WRITE_BIT);
	let refr = &mut *(ptr as *mut T);
	cb(refr);
	gl.UnmapNamedBuffer(handle);
}
