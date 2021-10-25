use crate::ffi::*;
use crate::lazy::*;

pub struct XcbComposite {
    pub(crate) lib: NamedLibrary,
    pub(crate) composite: XcbCompositeComposite,
}

lib_entry!(XcbComposite, "libxcb-composite.so.0");
