use crate::ffi::*;
use crate::lazy::*;

/// A dynamically loaded `libxcb-dri2`.
pub struct XcbDri2 {
    pub(crate) lib: NamedLibrary,
    pub(crate) dri2: XcbDri2Dri2,
}

lib_entry!(XcbDri2, "libxcb-dri2.so.0", "libxcb-dri2.so");
