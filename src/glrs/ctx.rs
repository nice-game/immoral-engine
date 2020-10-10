use crate::glrs::framebuffer::DefaultFramebuffer;
use gl::Gl;
use glutin::{
	event_loop::EventLoop,
	window::{Window, WindowBuilder},
	ContextBuilder, ContextWrapper, GlProfile, PossiblyCurrent,
};
use std::rc::Rc;

pub struct Ctx {
	window: ContextWrapper<PossiblyCurrent, Window>,
	pub gl: Gl,
}
impl Ctx {
	pub fn new(event_loop: &EventLoop<()>) -> Rc<Self> {
		let window = WindowBuilder::new();
		let window =
			ContextBuilder::new().with_gl_profile(GlProfile::Core).build_windowed(window, &event_loop).unwrap();
		let window = unsafe { window.make_current() }.unwrap();

		let gl = Gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);
		assert_eq!(unsafe { gl.GetError() }, 0);

		Rc::new(Self { window, gl })
	}

	pub fn default_framebuffer(self: &Rc<Self>) -> DefaultFramebuffer {
		DefaultFramebuffer::new(self)
	}

	pub fn window(&self) -> &ContextWrapper<PossiblyCurrent, Window> {
		&self.window
	}

	pub fn flush(&self) {
		unsafe { self.gl.Flush() };
	}
}
