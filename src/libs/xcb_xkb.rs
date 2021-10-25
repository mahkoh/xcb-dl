use crate::ffi::*;
use crate::lazy::*;

pub struct XcbXkb {
    pub(crate) lib: NamedLibrary,
    pub(crate) xkb: XcbXkbXkb,
}

lib_entry!(XcbXkb, "libxcb-xkb.so.1");
