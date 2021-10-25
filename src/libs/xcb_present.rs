use crate::ffi::*;
use crate::lazy::*;

pub struct XcbPresent {
    pub(crate) lib: NamedLibrary,
    pub(crate) present: XcbPresentPresent,
}

lib_entry!(XcbPresent, "libxcb-present.so.0");
