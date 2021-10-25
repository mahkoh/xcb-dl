use crate::ffi::*;
use crate::lazy::*;

pub struct XcbRandr {
    pub(crate) lib: NamedLibrary,
    pub(crate) randr: XcbRandrRandr,
}

lib_entry!(XcbRandr, "libxcb-randr.so.0");
