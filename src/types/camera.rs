use nalgebra::{one, zero, UnitQuaternion, Vector3};

pub struct Camera {
	pub uniform: CameraUniform,
	pub yaw: f32,
	pub pitch: f32,
	pub sensitivity: f32,
}
impl Camera {
	pub fn new() -> Self {
		Self { uniform: CameraUniform { rot: one(), pos: zero() }, yaw: 0.0, pitch: 0.0, sensitivity: 1.0 }
	}

	pub fn look(&mut self, x: f32, y: f32) {
		self.yaw -= x * self.sensitivity;
		self.pitch -= y * self.sensitivity;
		self.update();
	}

	pub fn update(&mut self) {
		self.uniform.rot = UnitQuaternion::from_euler_angles(0.0, 0.0, self.yaw)
			* UnitQuaternion::from_euler_angles(self.pitch, 0.0, 0.0);
	}
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct CameraUniform {
	pub rot: UnitQuaternion<f32>,
	pub pos: Vector3<f32>,
}
