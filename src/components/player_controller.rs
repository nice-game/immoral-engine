use crate::types::camera::Camera;
use nalgebra::{zero, Vector3};

pub struct PlayerController {
	pub cam: Camera,
	pub movement: Vector3<f32>,
}
impl PlayerController {
	pub fn new() -> Self {
		let mut cam = Camera::new();
		cam.uniform.pos = Vector3::new(0.0, 0.0, 0.0);
		cam.update();
		Self { cam, movement: zero() }
	}
}
