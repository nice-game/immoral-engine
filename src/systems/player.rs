use crate::{Ctx, PlayerController};
use glutin::event::{DeviceEvent, ElementState, VirtualKeyCode};
use shipyard::{UniqueView, UniqueViewMut};
use std::{sync::Arc, time::Duration};

pub fn update_player(
	_ctx: UniqueView<Arc<Ctx>>,
	events: UniqueView<Vec<DeviceEvent>>,
	delta: UniqueView<Duration>,
	mut player: UniqueViewMut<PlayerController>,
) {
	// if !ctx.grab.load(Ordering::Relaxed) {
	// 	return;
	// }

	let PlayerController { movement, cam, .. } = &mut *player;

	for event in &*events {
		match event {
			DeviceEvent::MouseMotion { delta: (x, y) } => cam.look((x * 0.01) as _, (y * 0.01) as _),
			DeviceEvent::Key(key) => {
				let val = if key.state == ElementState::Pressed { 1.0 } else { 0.0 };
				match key.virtual_keycode {
					Some(keycode) => match keycode {
						VirtualKeyCode::W => movement.y = val,
						VirtualKeyCode::A => movement.x = -val,
						VirtualKeyCode::S => movement.y = -val,
						VirtualKeyCode::D => movement.x = val,
						VirtualKeyCode::Space => movement.z = val,
						VirtualKeyCode::LShift => movement.z = -val,
						_ => (),
					},
					_ => (),
				}
			},
			_ => (),
		}
	}

	cam.uniform.pos += cam.uniform.rot * *movement * delta.as_secs_f32();
}
