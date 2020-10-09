extern crate libz_sys;

mod components;
mod glrs;
mod systems;
mod types;

use crate::{
	components::{model::Model, player_controller::PlayerController},
	glrs::{alloc::Allocator, ctx::Ctx, framebuffer::Framebuffer},
	systems::{
		gui::update_gui,
		player::update_player,
		render::{allocs::RenderAllocs, render, RenderState},
	},
};
use glutin::{
	event::{DeviceEvent, Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
};
use shipyard::{system, EntitiesViewMut, UniqueViewMut, ViewMut, World};
use std::{
	sync::atomic::Ordering,
	time::{Duration, Instant},
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

	let mut last_instant = Instant::now();

	let window_events: Vec<WindowEvent> = vec![];
	let device_events: Vec<DeviceEvent> = vec![];
	world.add_unique(window_events.clone());
	world.add_unique(device_events.clone());
	world.add_unique(Instant::now() - last_instant);

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
				ctx.default_framebuffer().clear_color(0.1, 0.1, 0.1, 1.0);

				world.run(|mut delta: UniqueViewMut<Duration>| {
					let now = Instant::now();
					*delta = now - last_instant;
					last_instant = now;
				});
				world.run_default();
				if ctx.quit.load(Ordering::Relaxed) {
					*control = ControlFlow::Exit;
					return;
				}
				world.run(render);

				ctx.flush();
				ctx.window().swap_buffers().unwrap();

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
