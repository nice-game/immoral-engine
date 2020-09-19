use nalgebra::{one, zero, UnitQuaternion, Vector3, Vector4};

pub struct Camera {
	pub uniform: CameraUniform,
	pub yaw: f32,
	pub pitch: f32,
	pub sensitivity: f32,
	pub z_near: f32,
	pub z_far: f32,
	pub fov: f32,
	pub aspect: f32, // aspect = screen_height / screen_width
}
impl Camera {
	pub fn new() -> Self {
		Self {
			uniform: CameraUniform {
				proj: zero(),
				rot: UnitQuaternion::identity(),
				pos: zero()
			},
			yaw: 0.0,
			pitch: 0.0,
			sensitivity: 1.0,
			z_near: 0.1,
			z_far: 1000.0,
			fov: 45.0,
			aspect: 1.0,
		}
	}

	pub fn look(&mut self, x: f32, y: f32) {
		self.yaw -= x * self.sensitivity;
		self.pitch -= y * self.sensitivity;
		self.update();
	}

	pub fn resize(&mut self, w: f32, h: f32) {
		self.aspect = h / w;
		self.update();
	}

	pub fn update(&mut self) {
		self.uniform.rot = UnitQuaternion::from_euler_angles(0.0, 0.0, self.yaw) * UnitQuaternion::from_euler_angles(self.pitch, 0.0, 0.0);

		let fov_tan_inv: f32 = 1.0 / (self.fov * (3.14159/180.0)).tan();
		self.uniform.proj[0] = fov_tan_inv * self.aspect;
		self.uniform.proj[1] = fov_tan_inv;
		self.uniform.proj[2] = (self.z_far + self.z_near) / (self.z_near - self.z_far);
		self.uniform.proj[3] = 2.0 * self.z_far * self.z_near / (self.z_near - self.z_far);
	}
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct CameraUniform {
	pub proj: Vector4<f32>,
	pub rot: UnitQuaternion<f32>,
	pub pos: Vector3<f32>,
}
