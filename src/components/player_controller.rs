use crate::types::camera::Camera;
use nalgebra::Vector3;
use specs::{prelude::*, Component};

#[derive(Component)]
pub struct PlayerController {
	pub cam: Camera,
}
impl PlayerController {
	pub fn new() -> Self {
		let mut cam = Camera::new();
		cam.uniform.pos = Vector3::new(0.0, -5.0, 0.0);
		Self { cam }
	}
}