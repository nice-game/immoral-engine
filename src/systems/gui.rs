use crate::Ctx;
use glutin::event::{ElementState, VirtualKeyCode, WindowEvent};
use specs::prelude::*;
use std::sync::Arc;

pub struct GuiSys;
impl<'a> System<'a> for GuiSys {
	type SystemData = (ReadExpect<'a, Arc<Ctx>>, Read<'a, Vec<WindowEvent<'static>>>);

	fn run(&mut self, (ctx, events): Self::SystemData) {
		for event in &*events {
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
}
