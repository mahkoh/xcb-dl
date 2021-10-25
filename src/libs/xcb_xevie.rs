use crate::ffi::*;
use crate::lazy::*;

pub struct XcbXevie {
    pub(crate) lib: NamedLibrary,
    pub(crate) xevie: XcbXevieXevie,
}

lib_entry!(XcbXevie, "libxcb-xevie.so.0");
