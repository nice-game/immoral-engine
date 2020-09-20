extern crate libz_sys;

mod components;
mod glrs;
mod systems;
mod types;

use crate::{
	components::{model::Model, player_controller::PlayerController},
	glrs::alloc::Allocator,
	systems::{
		gui::GuiSys,
		player::PlayerSys,
		render::{allocs::RenderAllocs, RenderSys},
	},
};
use gl::Gl;
use glutin::{
	event::{DeviceEvent, Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::{Window, WindowBuilder},
	ContextBuilder, ContextWrapper, PossiblyCurrent,
};
use specs::prelude::*;
use std::sync::{
	atomic::{AtomicBool, Ordering},
	Arc,
};

fn main() {
	let event_loop = EventLoop::new();
	let ctx = Ctx::new(&event_loop);
	let allocs = RenderAllocs::new(&ctx);

	let mut world = World::new();
	world.insert(ctx.clone());
	world.insert(Vec::<DeviceEvent>::new());
	world.insert(Vec::<WindowEvent>::new());
	world.register::<Model>();
	world.register::<PlayerController>();
	world.create_entity().with(PlayerController::new()).build();
	world.create_entity().with(Model::from_file(&allocs, "assets/baldman.dae")).build();

	let mut dispatcher = DispatcherBuilder::new()
		.with(PlayerSys, "", &[])
		.with(GuiSys, "", &[])
		.with_thread_local(RenderSys::new(&allocs))
		.build();
	dispatcher.setup(&mut world);

	unsafe { ctx.gl.ClearColor(0.1, 0.1, 0.1, 1.0) };

	event_loop.run(move |event, _window, control| {
		*control = ControlFlow::Poll;

		unsafe { ctx.gl.Clear(gl::COLOR_BUFFER_BIT) };

		dispatcher.dispatch(&mut world);
		if ctx.quit.load(Ordering::Relaxed) {
			*control = ControlFlow::Exit;
			return;
		}
		world.maintain();

		unsafe { ctx.gl.Flush() };
		ctx.window.swap_buffers().unwrap();

		let device_events = world.get_mut::<Vec<DeviceEvent>>().unwrap();
		device_events.clear();
		let window_events = world.get_mut::<Vec<WindowEvent>>().unwrap();
		window_events.clear();

		match event {
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::CloseRequested => *control = ControlFlow::Exit,
				WindowEvent::Resized(physical_size) => ctx.window.resize(physical_size),
				_ => world.get_mut::<Vec<WindowEvent>>().unwrap().push(event.to_static().unwrap()),
			},
			Event::DeviceEvent { event, .. } => world.get_mut::<Vec<DeviceEvent>>().unwrap().push(event),
			_ => (),
		};
	});
}

pub struct Ctx {
	window: ContextWrapper<PossiblyCurrent, Window>,
	gl: Gl,
	grab: AtomicBool,
	quit: AtomicBool,
}
impl Ctx {
	fn new(event_loop: &EventLoop<()>) -> Arc<Self> {
		let window = WindowBuilder::new();
		let window = ContextBuilder::new().build_windowed(window, &event_loop).unwrap();
		let window = unsafe { window.make_current() }.unwrap();

		let gl = Gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);
		assert_eq!(unsafe { gl.GetError() }, 0);

		Arc::new(Self { window, gl, grab: AtomicBool::default(), quit: AtomicBool::default() })
	}

	fn quit(&self) {
		self.quit.store(true, Ordering::Relaxed);
	}

	fn set_grab(&self, grab: bool) {
		self.window.window().set_cursor_grab(grab).unwrap();
		self.grab.store(grab, Ordering::Relaxed);
	}
}
unsafe impl Send for Ctx {}
unsafe impl Sync for Ctx {}
