use crate::ffi::*;
use crate::lazy::*;

/// A dynamically loaded `libxcb-dri3`.
pub struct XcbDri3 {
    pub(crate) lib: NamedLibrary,
    pub(crate) dri3: XcbDri3Dri3,
}

lib_entry!(XcbDri3, "libxcb-dri3.so.0", "libxcb-dri3.so");
