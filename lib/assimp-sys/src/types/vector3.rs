use std::os::raw::c_float;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AiVector3D {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
}
