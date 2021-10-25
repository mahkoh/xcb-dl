use crate::ffi::*;
use crate::lazy::*;

pub struct XcbGlx {
    pub(crate) lib: NamedLibrary,
    pub(crate) glx: XcbGlxGlx,
}

lib_entry!(XcbGlx, "libxcb-glx.so.0");
