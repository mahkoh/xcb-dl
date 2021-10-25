use crate::ffi::*;
use crate::lazy::*;

pub struct XcbXfixes {
    pub(crate) lib: NamedLibrary,
    pub(crate) xfixes: XcbXfixesXfixes,
}

lib_entry!(XcbXfixes, "libxcb-xfixes.so.0");
