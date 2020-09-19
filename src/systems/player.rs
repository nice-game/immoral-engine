use crate::{Ctx, PlayerController};
use glutin::event::{DeviceEvent, VirtualKeyCode};
use specs::prelude::*;
use std::sync::{atomic::Ordering, Arc};

pub struct PlayerSys;
impl<'a> System<'a> for PlayerSys {
	type SystemData = (ReadExpect<'a, Arc<Ctx>>, Read<'a, Vec<DeviceEvent>>, WriteStorage<'a, PlayerController>);

	fn run(&mut self, (ctx, events, mut players): Self::SystemData) {
		// if !ctx.grab.load(Ordering::Relaxed) {
		// 	return;
		// }

		let player = (&mut players).join().next().unwrap();

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
}
