use std::mem;
use std::ops::Index;
use std::os::raw::c_uint;

use ffi::AiFace;

define_type_and_iterator! {
    /// Face type (not yet implemented)
    struct Face(&AiFace)
    /// Face iterator type.
    struct FaceIter
}

impl<'a> Index<isize> for Face<'a> {
    type Output = c_uint;
    fn index(&self, index: isize) -> &c_uint {
        unsafe {
            assert!(index < self.num_indices as isize);
            mem::transmute(self.indices.offset(index))
        }
    }
}
