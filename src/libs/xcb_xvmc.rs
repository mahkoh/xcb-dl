use crate::ffi::*;
use crate::lazy::*;

/// A dynamically loaded `libxcb-xvmc`.
pub struct XcbXvmc {
    pub(crate) lib: NamedLibrary,
    pub(crate) xvmc: XcbXvmcXvmc,
}

lib_entry!(XcbXvmc, "libxcb-xvmc.so.0", "libxcb-xvmc.so");
