use crate::ffi::*;
use crate::lazy::*;

/// A dynamically loaded `libxcb-xkb`.
pub struct XcbXkb {
    pub(crate) lib: NamedLibrary,
    pub(crate) xkb: XcbXkbXkb,
}

lib_entry!(XcbXkb, "libxcb-xkb.so.1", "libxcb-xkb.so");
