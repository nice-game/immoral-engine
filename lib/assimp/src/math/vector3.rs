#[cfg(feature = "cgmath")]
use cgmath::{Point3, Vector3};
use ffi::AiVector3D;

define_type_and_iterator! {
    /// Vector3D docs
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Vector3D(AiVector3D)
    /// Vector3DIter docs
    struct Vector3DIter
}

impl Vector3D {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3D {
        Vector3D(AiVector3D { x: x, y: y, z: z })
    }
}

impl From<[f32; 3]> for Vector3D {
    fn from(v: [f32; 3]) -> Vector3D {
        Vector3D::new(v[0], v[1], v[2])
    }
}

impl From<Vector3D> for [f32; 3] {
    fn from(v: Vector3D) -> [f32; 3] {
        [v.x, v.y, v.z]
    }
}

#[cfg(feature = "cgmath")]
impl From<Point3<f32>> for Vector3D {
    fn from(p: Point3<f32>) -> Vector3D {
        Vector3D::new(p[0], p[1], p[2])
    }
}

#[cfg(feature = "cgmath")]
impl From<Vector3D> for Point3<f32> {
    fn from(v: Vector3D) -> Point3<f32> {
        Point3::new(v.x, v.y, v.z)
    }
}

#[cfg(feature = "cgmath")]
impl From<Vector3<f32>> for Vector3D {
    fn from(v: Vector3<f32>) -> Vector3D {
        Vector3D::new(v[0], v[1], v[2])
    }
}

#[cfg(feature = "cgmath")]
impl From<Vector3D> for Vector3<f32> {
    fn from(v: Vector3D) -> Vector3<f32> {
        Vector3::new(v.x, v.y, v.z)
    }
}
