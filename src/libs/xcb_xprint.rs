use crate::ffi::*;
use crate::lazy::*;

pub struct XcbXprint {
    pub(crate) lib: NamedLibrary,
    pub(crate) xprint: XcbXprintXprint,
}

lib_entry!(XcbXprint, "libxcb-xprint.so.0");
