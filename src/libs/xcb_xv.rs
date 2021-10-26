use crate::ffi::*;
use crate::lazy::*;

/// A dynamically loaded `libxcb-xv`.
pub struct XcbXv {
    pub(crate) lib: NamedLibrary,
    pub(crate) xv: XcbXvXv,
}

lib_entry!(XcbXv, "libxcb-xv.so.0", "libxcb-xv.so");
