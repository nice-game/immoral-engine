use gl::{types::GLenum, Gl};
use glutin::{
	event_loop::EventLoop,
	window::{Window, WindowBuilder},
	ContextBuilder, ContextWrapper, GlProfile, PossiblyCurrent,
};
use std::sync::{
	atomic::{AtomicBool, Ordering},
	Arc,
};

pub struct Ctx {
	window: ContextWrapper<PossiblyCurrent, Window>,
	pub gl: Gl,
	grab: AtomicBool,
	pub quit: AtomicBool,
}
impl Ctx {
	pub fn new(event_loop: &EventLoop<()>) -> Arc<Self> {
		let window = WindowBuilder::new();
		let window =
			ContextBuilder::new().with_gl_profile(GlProfile::Core).build_windowed(window, &event_loop).unwrap();
		let window = unsafe { window.make_current() }.unwrap();

		let gl = Gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);
		assert_eq!(unsafe { gl.GetError() }, 0);

		Arc::new(Self { window, gl, grab: AtomicBool::default(), quit: AtomicBool::default() })
	}

	pub fn window(&self) -> &ContextWrapper<PossiblyCurrent, Window> {
		&self.window
	}

	pub fn clear(&self, target: GLenum) {
		unsafe { self.gl.Clear(target) };
	}

	pub fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
		unsafe { self.gl.ClearColor(0.1, 0.1, 0.1, 1.0) };
	}

	pub fn flush(&self) {
		unsafe { self.gl.Flush() };
	}

	pub fn quit(&self) {
		self.quit.store(true, Ordering::Relaxed);
	}

	pub fn set_grab(&self, grab: bool) {
		self.window.window().set_cursor_grab(grab).unwrap();
		self.grab.store(grab, Ordering::Relaxed);
	}
}
unsafe impl Send for Ctx {}
unsafe impl Sync for Ctx {}
