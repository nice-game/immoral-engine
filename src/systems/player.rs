use crate::PlayerController;
use glutin::event::{DeviceEvent, VirtualKeyCode};
use specs::prelude::*;
use std::collections::VecDeque;

pub struct PlayerSys;
impl<'a> System<'a> for PlayerSys {
	type SystemData = (Read<'a, VecDeque<DeviceEvent>>, WriteStorage<'a, PlayerController>);

	fn run(&mut self, (events, mut players): Self::SystemData) {
		let player = (&mut players).join().next().unwrap();

		for event in &*events {
			match event {
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
