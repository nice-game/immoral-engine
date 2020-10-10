use crate::{glrs::buffer::Buffer, Ctx};
use gl::types::{GLenum, GLint, GLsizei, GLuint};
use std::rc::Rc;

pub trait Texture {
	fn handle(&self) -> GLuint;
	fn min_filter(&self, filter: Filter);
	fn mag_filter(&self, filter: Filter);
}

pub struct Texture3D {
	ctx: Rc<Ctx>,
	handle: GLuint,
}
impl Texture3D {
	pub fn new(ctx: &Rc<Ctx>, width: GLsizei, height: GLsizei, depth: GLsizei) -> Texture3D {
		let gl = &ctx.gl;
		let mut handle = 0;
		unsafe {
			gl.CreateTextures(gl::TEXTURE_2D_ARRAY, 1, &mut handle);
			gl.TextureStorage3D(handle, 1, gl::RGBA8, width, height, depth);
		}

		Texture3D { ctx: ctx.clone(), handle }
	}

	// TODO: change subimage_* to specialized functions once specialization is stable
	pub fn subimage_u8(&self, offset: [i32; 3], area: [i32; 3], format: GLenum, buffer: &Buffer<[u8]>) {
		println!("{:?}", area);
		assert_eq!(buffer.len() as i32, area[0] * area[1] * area[2] * format_len(format));
		unsafe { self.subimage(offset, area, format, gl::UNSIGNED_BYTE, buffer.mem.alloc.id, buffer.offset()) };
	}

	unsafe fn subimage(
		&self,
		offset: [i32; 3],
		area: [i32; 3],
		format: GLenum,
		typ: GLenum,
		buffer: GLuint,
		buf_offset: usize,
	) {
		let gl = &self.ctx.gl;
		gl.BindBuffer(gl::PIXEL_UNPACK_BUFFER, buffer);
		gl.TextureSubImage3D(
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
		);
	}

	unsafe fn param(&self, pname: GLenum, param: GLint) {
		self.ctx.gl.TextureParameteri(self.handle, pname, param);
	}
}
impl Drop for Texture3D {
	fn drop(&mut self) {
		unsafe { self.ctx.gl.DeleteTextures(1, &self.handle) };
	}
}
impl Texture for Texture3D {
	fn handle(&self) -> GLuint {
		self.handle
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
