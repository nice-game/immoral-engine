use crate::Ctx;
use glutin::event::{ElementState, VirtualKeyCode, WindowEvent};
use shipyard::UniqueView;
use std::sync::Arc;

pub fn update_gui(ctx: UniqueView<Arc<Ctx>>, events: UniqueView<Vec<WindowEvent>>) {
	for event in events.iter() {
		match event {
			WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
				Some(keycode) => match keycode {
					VirtualKeyCode::Escape => {
						// if ctx.grab.load(Ordering::Relaxed) {
						// ctx.set_grab(false);
						// } else {
						ctx.quit();
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
