use crate::ffi::*;
use crate::lazy::*;

pub struct XcbDpms {
    pub(crate) lib: NamedLibrary,
    pub(crate) dpms: XcbDpmsDpms,
}

lib_entry!(XcbDpms, "libxcb-dpms.so.0");
