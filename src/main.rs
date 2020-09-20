extern crate libz_sys;

mod components;
mod glrs;
mod systems;
mod types;

use crate::{
	components::{model::Model, player_controller::PlayerController},
	glrs::alloc::Allocator,
	systems::{
		gui::update_gui,
		player::update_player,
		render::{allocs::RenderAllocs, render, RenderState},
	},
};
use gl::Gl;
use glutin::{
	event::{DeviceEvent, Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::{Window, WindowBuilder},
	ContextBuilder, ContextWrapper, GlProfile, PossiblyCurrent,
};
use shipyard::{system, EntitiesViewMut, UniqueViewMut, ViewMut, World};
use std::sync::{
	atomic::{AtomicBool, Ordering},
	Arc,
};

fn main() {
	let event_loop = EventLoop::new();
	let ctx = Ctx::new(&event_loop);
	let allocs = RenderAllocs::new(&ctx);

	let world = World::new();
	world.add_unique(ctx.clone());
	world.add_unique(RenderState::new(&allocs));
	world.add_unique(PlayerController::new());
	world.run(|mut entities: EntitiesViewMut, mut models: ViewMut<Model>| {
		entities.add_entity(&mut models, Model::from_file(&allocs, "assets/baldman.dae"));
	});

	world.add_workload("").with_system(system!(update_gui)).with_system(system!(update_player)).build();

	unsafe { ctx.gl.ClearColor(0.1, 0.1, 0.1, 1.0) };

	let window_events: Vec<WindowEvent> = vec![];
	let device_events: Vec<DeviceEvent> = vec![];
	world.add_unique(window_events.clone());
	world.add_unique(device_events.clone());

	event_loop.run(move |event, _window, control| {
		*control = ControlFlow::Poll;

		match event {
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::CloseRequested => *control = ControlFlow::Exit,
				WindowEvent::Resized(physical_size) => ctx.window.resize(physical_size),
				_ => world.run(|events| push_window_event(event, events)),
			},
			Event::DeviceEvent { event, .. } => world.run(|events| push_device_event(event, events)),
			Event::MainEventsCleared => {
				unsafe { ctx.gl.Clear(gl::COLOR_BUFFER_BIT) };

				world.run_default();
				if ctx.quit.load(Ordering::Relaxed) {
					*control = ControlFlow::Exit;
					return;
				}
				world.run(render);

				unsafe { ctx.gl.Flush() };
				ctx.window.swap_buffers().unwrap();

				world.run(clear_events);
			},
			_ => (),
		};
	});
}

fn clear_events(
	mut window_events: UniqueViewMut<Vec<WindowEvent>>,
	mut device_events: UniqueViewMut<Vec<DeviceEvent>>,
) {
	window_events.clear();
	device_events.clear();
}

fn push_device_event(event: DeviceEvent, mut device_events: UniqueViewMut<Vec<DeviceEvent>>) {
	device_events.push(event);
}

fn push_window_event(event: WindowEvent, mut window_events: UniqueViewMut<Vec<WindowEvent>>) {
	window_events.push(event.to_static().unwrap());
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
		let window =
			ContextBuilder::new().with_gl_profile(GlProfile::Core).build_windowed(window, &event_loop).unwrap();
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
