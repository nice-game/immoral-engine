use crate::{buffer::BufferSlice, Ctx};
use gl::types::{GLenum, GLint, GLuint};
use nalgebra::{
	allocator::Allocator, base::dimension::DimAdd, DefaultAllocator, Dim, DimName, NamedDim, VectorN, U0, U1, U2, U3,
};
use std::{
	marker::PhantomData,
	ops::{Add, Mul},
	rc::Rc,
};
use typenum::Sum;

pub type Texture1D = Texture<U1, U0>;
pub type Texture2D = Texture<U2, U0>;
pub type Texture2DArray = Texture<U2, U1>;
pub type Texture3D = Texture<U3, U0>;

pub trait TextureAbstract {
	fn handle(&self) -> GLuint;
	fn target(&self) -> GLenum;
	fn min_filter(&self, filter: Filter);
	fn mag_filter(&self, filter: Filter);
}

pub struct Texture<D, A> {
	ctx: Rc<Ctx>,
	handle: GLuint,
	target: GLenum,
	phantom: PhantomData<(D, A)>,
}
impl<D: Dim + DimName, A: Dim + DimName> Texture<D, A>
where
	DefaultAllocator: Allocator<i32, <D as DimAdd<A>>::Output>,
	D::Value: Add<A::Value>,
	Sum<D::Value, A::Value>: NamedDim,
	<<D as DimAdd<A>>::Output as DimName>::Value: Mul<<U1 as DimName>::Value>,
{
	pub fn new(ctx: &Rc<Ctx>, size: VectorN<i32, <D as DimAdd<A>>::Output>) -> Self {
		let gl = &ctx.gl;

		let target = match D::dim() {
			1 => match A::dim() {
				1 => gl::TEXTURE_1D_ARRAY,
				0 => gl::TEXTURE_1D,
				_ => unimplemented!(),
			},
			2 => match A::dim() {
				1 => gl::TEXTURE_2D_ARRAY,
				0 => gl::TEXTURE_2D,
				_ => unimplemented!(),
			},
			3 => match A::dim() {
				1 => unimplemented!(),
				0 => gl::TEXTURE_3D,
				_ => unimplemented!(),
			},
			_ => unimplemented!(),
		};

		let mut handle = 0;
		unsafe {
			gl.CreateTextures(target, 1, &mut handle);
			match D::dim() + A::dim() {
				1 => gl.TextureStorage1D(handle, 1, gl::RGBA8, size[0]),
				2 => gl.TextureStorage2D(handle, 1, gl::RGBA8, size[0], size[1]),
				3 => gl.TextureStorage3D(handle, 1, gl::RGBA8, size[0], size[1], size[2]),
				_ => unimplemented!(),
			}
		}

		Self { ctx: ctx.clone(), handle, target, phantom: PhantomData }
	}

	// TODO: change subimage_* to specialized functions once generic specialization is stable
	pub fn subimage_u8(
		&self,
		offset: VectorN<i32, <D as DimAdd<A>>::Output>,
		area: VectorN<i32, <D as DimAdd<A>>::Output>,
		format: GLenum,
		data: &dyn BufferSlice<u8>,
	) {
		let area_size: i32 = area.iter().cloned().product();
		assert_eq!(data.len() as i32, format_len(format) * area_size);
		unsafe { self.subimage(offset, area, format, gl::UNSIGNED_BYTE, data.handle(), data.offset()) };
	}

	unsafe fn subimage(
		&self,
		offset: VectorN<i32, <D as DimAdd<A>>::Output>,
		area: VectorN<i32, <D as DimAdd<A>>::Output>,
		format: GLenum,
		typ: GLenum,
		buffer: GLuint,
		buf_offset: usize,
	) {
		let gl = &self.ctx.gl;
		gl.BindBuffer(gl::PIXEL_UNPACK_BUFFER, buffer);
		match D::dim() + A::dim() {
			1 => gl.TextureSubImage1D(self.handle, 0, offset[0], area[0], format, typ, buf_offset as _),
			2 => gl.TextureSubImage2D(
				self.handle,
				0,
				offset[0],
				offset[1],
				area[0],
				area[1],
				format,
				typ,
				buf_offset as _,
			),
			3 => gl.TextureSubImage3D(
				self.handle,
				0,
				offset[0],
				offset[1],
				offset[2],
				area[0],
				area[1],
				area[2],
				format,
				typ,
				buf_offset as _,
			),
			_ => unimplemented!(),
		}
	}

	unsafe fn param(&self, pname: GLenum, param: GLint) {
		self.ctx.gl.TextureParameteri(self.handle, pname, param);
	}
}
impl<D, A> Drop for Texture<D, A> {
	fn drop(&mut self) {
		unsafe { self.ctx.gl.DeleteTextures(1, &self.handle) };
	}
}
impl<D: Dim + DimName, A: Dim + DimName> TextureAbstract for Texture<D, A>
where
	DefaultAllocator: Allocator<i32, <D as DimAdd<A>>::Output>,
	D::Value: Add<A::Value>,
	Sum<D::Value, A::Value>: NamedDim,
	<<D as DimAdd<A>>::Output as DimName>::Value: Mul<<U1 as DimName>::Value>,
{
	fn handle(&self) -> GLuint {
		self.handle
	}

	fn target(&self) -> GLenum {
		self.target
	}

	fn min_filter(&self, filter: Filter) {
		unsafe { self.param(gl::TEXTURE_MIN_FILTER, filter as _) };
	}

	fn mag_filter(&self, filter: Filter) {
		unsafe { self.param(gl::TEXTURE_MAG_FILTER, filter as _) };
	}
}

#[repr(u32)]
pub enum Filter {
	Linear = gl::LINEAR,
}

fn format_len(format: GLenum) -> i32 {
	match format {
		gl::RGBA => 4,
		_ => unimplemented!(),
	}
}
