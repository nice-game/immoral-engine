use crate::Application;
use glutin::event::{ElementState, VirtualKeyCode, WindowEvent};
use shipyard::{UniqueView, UniqueViewMut};

pub fn update_gui(mut app: UniqueViewMut<Application>, events: UniqueView<Vec<WindowEvent>>) {
	for event in events.iter() {
		match event {
			WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
				Some(keycode) => match keycode {
					VirtualKeyCode::Escape => {
						// if ctx.grab.load(Ordering::Relaxed) {
						// ctx.set_grab(false);
						// } else {
						app.quit();
						// }
					},
					_ => (),
				},
				_ => (),
			},
			WindowEvent::MouseInput { state, .. } => match state {
				ElementState::Pressed => {
					// ctx.set_grab(true);
				},
				ElementState::Released => (),
			},
			_ => (),
		}
	}
}
