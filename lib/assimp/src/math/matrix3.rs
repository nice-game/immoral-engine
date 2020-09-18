#[cfg(feature = "cgmath")]
use cgmath::Matrix3;
use ffi::AiMatrix3x3;

define_type! {
    /// Matrix3x3 docs
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Matrix3x3(AiMatrix3x3)
}

impl Matrix3x3 {
    pub fn new(c0r0: f32, c0r1: f32, c0r2: f32,
               c1r0: f32, c1r1: f32, c1r2: f32,
               c2r0: f32, c2r1: f32, c2r2: f32) -> Matrix3x3 {
        Matrix3x3(AiMatrix3x3 {
            a1: c0r0, a2: c0r1, a3: c0r2,
            b1: c1r0, b2: c1r1, b3: c1r2,
            c1: c2r0, c2: c2r1, c3: c2r2,
        })
    }
}

#[cfg(feature = "cgmath")]
impl From<Matrix3<f32>> for Matrix3x3 {
    fn from(mat: Matrix3<f32>) -> Matrix3x3 {
        Matrix3x3::new(mat[0][0], mat[1][0], mat[2][0],
                       mat[0][1], mat[1][1], mat[2][1],
                       mat[0][2], mat[1][2], mat[2][2])
    }
}

#[cfg(feature = "cgmath")]
impl From<Matrix3x3> for Matrix3<f32> {
    fn from(mat: Matrix3x3) -> Matrix3<f32> {
        Matrix3::new(mat.a1, mat.b1, mat.c1,
                     mat.a2, mat.b2, mat.c2,
                     mat.a3, mat.b3, mat.c3)
    }
}
