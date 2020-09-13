mod components;
mod systems;

use crate::{components::mesh::Mesh, systems::render::Render};
use glutin::{
	event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::WindowBuilder,
	ContextBuilder,
};
use specs::prelude::*;

fn main() {
	let event_loop = EventLoop::new();
	let wb = WindowBuilder::new();

	let window = ContextBuilder::new().build_windowed(wb, &event_loop).unwrap();
	let window = unsafe { window.make_current() }.unwrap();
	gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

	let mut world = World::new();
	world.register::<Mesh>();
	world.create_entity().with(Mesh::new()).build();

	let mut dispatcher = DispatcherBuilder::new().with_thread_local(Render::new()).build();
	dispatcher.setup(&mut world);

	unsafe { gl::ClearColor(0.1, 0.1, 0.1, 1.0) };

	event_loop.run(move |event, _window, control| {
		*control = ControlFlow::Poll;

		unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
		dispatcher.dispatch(&mut world);
		world.maintain();
		unsafe { gl::Flush() };
		window.swap_buffers().unwrap();

		match event {
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::CloseRequested => *control = ControlFlow::Exit,
				WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode, .. }, .. } => {
					match virtual_keycode {
						Some(VirtualKeyCode::Escape) => *control = ControlFlow::Exit,
						_ => (),
					}
				},
				WindowEvent::Resized(physical_size) => window.resize(physical_size),
				_ => (),
			},
			_ => (),
		};
	});
}
