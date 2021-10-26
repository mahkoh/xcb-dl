use crate::ffi::*;
use crate::lazy::*;

/// A dynamically loaded `libxcb-res`.
pub struct XcbRes {
    pub(crate) lib: NamedLibrary,
    pub(crate) res: XcbResRes,
}

lib_entry!(XcbRes, "libxcb-res.so.0", "libxcb-res.so");
