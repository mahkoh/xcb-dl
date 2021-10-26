use crate::ffi::*;
use crate::lazy::*;

/// A dynamically loaded `libxcb-xprint`.
pub struct XcbXprint {
    pub(crate) lib: NamedLibrary,
    pub(crate) xprint: XcbXprintXprint,
}

lib_entry!(XcbXprint, "libxcb-xprint.so.0", "libxcb-xprint.so");
