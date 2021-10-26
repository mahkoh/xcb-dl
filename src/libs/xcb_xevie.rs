use crate::ffi::*;
use crate::lazy::*;

/// A dynamically loaded `libxcb-xevie`.
pub struct XcbXevie {
    pub(crate) lib: NamedLibrary,
    pub(crate) xevie: XcbXevieXevie,
}

lib_entry!(XcbXevie, "libxcb-xevie.so.0", "libxcb-xevie.so");
