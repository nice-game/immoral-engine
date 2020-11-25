use crate::{texture::TextureAbstract, Ctx};
use gl::types::GLuint;
use std::rc::Rc;

pub trait FramebufferAbstract {
	fn ctx(&self) -> &Rc<Ctx>;
	fn handle(&self) -> GLuint;

	fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
		let color = [red, green, blue, alpha];
		unsafe { self.ctx().gl.ClearNamedFramebufferfv(self.handle(), gl::COLOR, 0, color.as_ptr()) };
	}
}

pub struct DefaultFramebuffer {
	ctx: Rc<Ctx>,
}
impl DefaultFramebuffer {
	pub(super) fn new(ctx: &Rc<Ctx>) -> Self {
		Self { ctx: ctx.clone() }
	}
}
impl FramebufferAbstract for DefaultFramebuffer {
	fn ctx(&self) -> &Rc<Ctx> {
		&self.ctx
	}

	fn handle(&self) -> GLuint {
		0
	}
}

pub struct Framebuffer {
	ctx: Rc<Ctx>,
	handle: GLuint,
}
impl Framebuffer {
	pub fn new(ctx: &Rc<Ctx>) -> Self {
		let mut handle = 0;
		unsafe { ctx.gl.CreateFramebuffers(1, &mut handle) };
		Self { ctx: ctx.clone(), handle }
	}

	pub fn color(&self, idx: u32, texture: &dyn TextureAbstract) {
		unsafe { self.ctx.gl.NamedFramebufferTexture(self.handle, gl::COLOR_ATTACHMENT0 + idx, texture.handle(), 0) };
	}
}
impl Drop for Framebuffer {
	fn drop(&mut self) {
		unsafe { self.ctx.gl.DeleteFramebuffers(1, &self.handle) };
	}
}
