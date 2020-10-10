extern crate libz_sys;

mod components;
mod systems;
mod types;

use crate::{
	components::{model::Model, player_controller::PlayerController},
	systems::{
		gui::update_gui,
		player::update_player,
		render::{allocs::RenderAllocs, render, render_init},
	},
};
use glrs::{framebuffer::Framebuffer, Ctx};
use glutin::{
	event::{DeviceEvent, Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
};
use shipyard::{system, EntitiesViewMut, NonSendSync, UniqueView, UniqueViewMut, ViewMut, World};
use std::time::{Duration, Instant};

fn main() {
	let event_loop = EventLoop::new();
	let ctx = Ctx::new(&event_loop);
	let allocs = RenderAllocs::new(&ctx);

	let world = World::new();
	world.add_unique(Application::default());
	world.add_unique(PlayerController::new());
	world.run(|mut entities: EntitiesViewMut, mut models: NonSendSync<ViewMut<Model>>| {
		entities.add_entity(&mut *models, Model::from_file(&allocs, "assets/baldman.dae"));
	});

	render_init(&world, &allocs);

	world
		.add_workload("")
		.with_system(system!(update_gui))
		.with_system(system!(update_player))
		.with_system(system!(render))
		.build();

	let mut last_instant = Instant::now();

	world.add_unique(Vec::<WindowEvent>::new());
	world.add_unique(Vec::<DeviceEvent>::new());
	world.add_unique(Duration::new(0, 0));

	event_loop.run(move |event, _window, control| {
		*control = ControlFlow::Poll;

		match event {
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::CloseRequested => *control = ControlFlow::Exit,
				WindowEvent::Resized(physical_size) => ctx.window().resize(physical_size),
				_ => world.run(|events| push_window_event(event, events)),
			},
			Event::DeviceEvent { event, .. } => world.run(|events| push_device_event(event, events)),
			Event::MainEventsCleared => {
				world.run(|mut delta: UniqueViewMut<Duration>| {
					let now = Instant::now();
					*delta = now - last_instant;
					last_instant = now;
				});
				world.run_default();
				world.run(|app: UniqueView<Application>| {
					if app.quit {
						*control = ControlFlow::Exit;
						return;
					}
				});

				ctx.flush();
				ctx.window().swap_buffers().unwrap();
				ctx.default_framebuffer().clear_color(0.1, 0.1, 0.1, 1.0);

				world.run(clear_events);
			},
			_ => (),
		};
	});
}

#[derive(Default)]
pub struct Application {
	pub quit: bool,
}
impl Application {
	pub fn quit(&mut self) {
		self.quit = true;
	}
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
