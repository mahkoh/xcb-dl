use crate::ffi::*;
use crate::lazy::*;

/// A dynamically loaded `libxcb-composite`.
pub struct XcbComposite {
    pub(crate) lib: NamedLibrary,
    pub(crate) composite: XcbCompositeComposite,
}

lib_entry!(XcbComposite, "libxcb-composite.so.0", "libxcb-composite.so");
