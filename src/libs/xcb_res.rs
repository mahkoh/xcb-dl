use crate::ffi::*;
use crate::lazy::*;

pub struct XcbRes {
    pub(crate) lib: NamedLibrary,
    pub(crate) res: XcbResRes,
}

lib_entry!(XcbRes, "libxcb-res.so.0");
