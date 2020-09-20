use crate::{Ctx, PlayerController};
use glutin::event::{DeviceEvent, VirtualKeyCode};
use shipyard::{UniqueView, UniqueViewMut};
use std::sync::Arc;

pub fn update_player(
	_ctx: UniqueView<Arc<Ctx>>,
	events: UniqueView<Vec<DeviceEvent>>,
	mut player: UniqueViewMut<PlayerController>,
) {
	// if !ctx.grab.load(Ordering::Relaxed) {
	// 	return;
	// }

	for event in &*events {
		match event {
			DeviceEvent::MouseMotion { delta: (x, y) } => player.cam.look((x * 0.01) as _, (y * 0.01) as _),
			DeviceEvent::Key(key) => match key.virtual_keycode {
				Some(keycode) => match keycode {
					VirtualKeyCode::W => player.cam.uniform.pos.y += 0.1,
					VirtualKeyCode::A => player.cam.uniform.pos.x -= 0.1,
					VirtualKeyCode::S => player.cam.uniform.pos.y -= 0.1,
					VirtualKeyCode::D => player.cam.uniform.pos.x += 0.1,
					_ => (),
				},
				_ => (),
			},
			_ => (),
		}
	}
}
