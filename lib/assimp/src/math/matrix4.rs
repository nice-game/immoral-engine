#[cfg(feature = "cgmath")]
use cgmath::Matrix4;
use ffi::AiMatrix4x4;

define_type! {
    /// Matrix4x4 docs
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Matrix4x4(AiMatrix4x4)
}

impl Matrix4x4 {
    pub fn new(c0r0: f32, c0r1: f32, c0r2: f32, c0r3: f32,
               c1r0: f32, c1r1: f32, c1r2: f32, c1r3: f32,
               c2r0: f32, c2r1: f32, c2r2: f32, c2r3: f32,
               c3r0: f32, c3r1: f32, c3r2: f32, c3r3: f32) -> Matrix4x4 {
        Matrix4x4(AiMatrix4x4 {
            a1: c0r0, a2: c0r1, a3: c0r2, a4: c0r3,
            b1: c1r0, b2: c1r1, b3: c1r2, b4: c1r3,
            c1: c2r0, c2: c2r1, c3: c2r2, c4: c2r3,
            d1: c3r0, d2: c3r1, d3: c3r2, d4: c3r3,
        })
    }
}

#[cfg(feature = "cgmath")]
impl From<Matrix4<f32>> for Matrix4x4 {
    fn from(mat: Matrix4<f32>) -> Matrix4x4 {
        Matrix4x4::new(mat[0][0], mat[1][0], mat[2][0], mat[3][0],
                       mat[0][1], mat[1][1], mat[2][1], mat[3][1],
                       mat[0][2], mat[1][2], mat[2][2], mat[3][2],
                       mat[0][3], mat[1][3], mat[2][3], mat[3][3])
    }
}

#[cfg(feature = "cgmath")]
impl From<Matrix4x4> for Matrix4<f32> {
    fn from(mat: Matrix4x4) -> Matrix4<f32> {
        Matrix4::new(mat.a1, mat.b1, mat.c1, mat.d1,
                     mat.a2, mat.b2, mat.c2, mat.d2,
                     mat.a3, mat.b3, mat.c3, mat.d3,
                     mat.a4, mat.b4, mat.c4, mat.d4)
    }
}
