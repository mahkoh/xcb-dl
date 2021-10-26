use crate::ffi::*;
use crate::lazy::*;

/// A dynamically loaded `libxcb-ge`.
pub struct XcbGe {
    pub(crate) lib: NamedLibrary,
    pub(crate) ge: XcbGeGe,
}

lib_entry!(XcbGe, "libxcb-ge.so.0", "libxcb-ge.so");
