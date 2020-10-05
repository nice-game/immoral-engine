use crate::Ctx;
use gl::types::{GLenum, GLint, GLsizei, GLuint};
use std::sync::Arc;

pub struct Texture {
	ctx: Arc<Ctx>,
	handle: GLuint,
}
impl Texture {
	pub fn new_3d(ctx: &Arc<Ctx>, width: GLsizei, height: GLsizei, depth: GLsizei) -> Texture {
		let gl = &ctx.gl;
		let mut handle = 0;
		unsafe {
			gl.CreateTextures(gl::TEXTURE_2D_ARRAY, 1, &mut handle);
			gl.TextureStorage3D(handle, 1, gl::RGBA8, width, height, depth);
		}

		Texture { ctx: ctx.clone(), handle }
	}

	pub fn handle(&self) -> GLuint {
		self.handle
	}

	pub fn min_filter(&self, filter: Filter) {
		unsafe { self.param(gl::TEXTURE_MIN_FILTER, filter as _) };
	}

	pub fn mag_filter(&self, filter: Filter) {
		unsafe { self.param(gl::TEXTURE_MAG_FILTER, filter as _) };
	}

	unsafe fn param(&self, pname: GLenum, param: GLint) {
		self.ctx.gl.TextureParameteri(self.handle, pname, param);
	}
}
impl Drop for Texture {
	fn drop(&mut self) {
		unsafe { self.ctx.gl.DeleteTextures(1, &self.handle) };
	}
}

#[repr(u32)]
pub enum Filter {
	Linear = gl::LINEAR,
}
