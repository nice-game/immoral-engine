use std::os::raw::{c_uint, c_void};

use types::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AiMetadataType {
    Bool = 0,
    Int32 = 1,
    Uint64 = 2,
    Float = 3,
    Double = 4,
    AiString = 5,
    AiVector3D = 6,
}

#[repr(C)]
pub struct AiMetadataEntry {
    pub data_type: AiMetadataType,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct AiMetadata {
    pub num_properties: c_uint,
    pub keys: *mut AiString,
    pub values: *mut AiMetadataEntry,
}
