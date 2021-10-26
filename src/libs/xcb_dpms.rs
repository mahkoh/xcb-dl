use crate::ffi::*;
use crate::lazy::*;

/// A dynamically loaded `libxcb-dpms`.
pub struct XcbDpms {
    pub(crate) lib: NamedLibrary,
    pub(crate) dpms: XcbDpmsDpms,
}

lib_entry!(XcbDpms, "libxcb-dpms.so.0", "libxcb-dpms.so");
