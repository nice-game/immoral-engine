mod components;
mod systems;

use crate::{components::mesh::Mesh, systems::render::Render};
use gl::Gl;
use glutin::{
	event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::{Window, WindowBuilder},
	ContextBuilder, ContextWrapper, PossiblyCurrent,
};
use specs::prelude::*;
use std::sync::Arc;

fn main() {
	let event_loop = EventLoop::new();
	let ctx = Ctx::new(&event_loop);

	let mut world = World::new();
	world.register::<Mesh>();
	world.create_entity().with(Mesh::new(&ctx)).build();

	let mut dispatcher = DispatcherBuilder::new().with_thread_local(Render::new(&ctx)).build();
	dispatcher.setup(&mut world);

	unsafe { ctx.gl.ClearColor(0.1, 0.1, 0.1, 1.0) };

	event_loop.run(move |event, _window, control| {
		*control = ControlFlow::Poll;

		unsafe { ctx.gl.Clear(gl::COLOR_BUFFER_BIT) };
		dispatcher.dispatch(&mut world);
		world.maintain();
		unsafe { ctx.gl.Flush() };
		ctx.window.swap_buffers().unwrap();

		match event {
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::CloseRequested => *control = ControlFlow::Exit,
				WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode, .. }, .. } => {
					match virtual_keycode {
						Some(VirtualKeyCode::Escape) => *control = ControlFlow::Exit,
						_ => (),
					}
				},
				WindowEvent::Resized(physical_size) => ctx.window.resize(physical_size),
				_ => (),
			},
			_ => (),
		};
	});
}

pub struct Ctx {
	window: ContextWrapper<PossiblyCurrent, Window>,
	gl: Gl,
}
impl Ctx {
	fn new(event_loop: &EventLoop<()>) -> Arc<Self> {
		let window = WindowBuilder::new();
		let window = ContextBuilder::new().build_windowed(window, &event_loop).unwrap();
		let window = unsafe { window.make_current() }.unwrap();

		let gl = Gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

		Arc::new(Self { window, gl })
	}
}
unsafe impl Send for Ctx {}
unsafe impl Sync for Ctx {}
