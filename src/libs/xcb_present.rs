use crate::ffi::*;
use crate::lazy::*;

/// A dynamically loaded `libxcb-present`.
pub struct XcbPresent {
    pub(crate) lib: NamedLibrary,
    pub(crate) present: XcbPresentPresent,
}

lib_entry!(XcbPresent, "libxcb-present.so.0", "libxcb-present.so");
