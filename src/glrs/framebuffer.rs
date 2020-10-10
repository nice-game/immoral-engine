use crate::Ctx;
use gl::types::GLuint;
use std::rc::Rc;

pub trait Framebuffer {
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
impl Framebuffer for DefaultFramebuffer {
	fn ctx(&self) -> &Rc<Ctx> {
		&self.ctx
	}

	fn handle(&self) -> GLuint {
		0
	}
}
