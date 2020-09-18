#[cfg(feature = "cgmath")]
use cgmath::Vector3;
use ffi::AiColor3D;

define_type! {
    /// Color3D docs
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Color3D(AiColor3D)
}

impl Color3D {
    pub fn new(r: f32, g: f32, b: f32) -> Color3D {
        Color3D(AiColor3D { r: r, g: g, b: b })
    }
}

impl From<[f32; 3]> for Color3D {
    fn from(v: [f32; 3]) -> Color3D {
        Color3D::new(v[0], v[1], v[2])
    }
}

impl From<Color3D> for [f32; 3] {
    fn from(c: Color3D) -> [f32; 3] {
        [c.r, c.g, c.b]
    }
}

#[cfg(feature = "cgmath")]
impl From<Vector3<f32>> for Color3D {
    fn from(v: Vector3<f32>) -> Color3D {
        Color3D::new(v[0], v[1], v[2])
    }
}

#[cfg(feature = "cgmath")]
impl From<Color3D> for Vector3<f32> {
    fn from(c: Color3D) -> Vector3<f32> {
        Vector3::new(c.r, c.g, c.b)
    }
}
