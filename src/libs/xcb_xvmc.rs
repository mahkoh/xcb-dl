use crate::ffi::*;
use crate::lazy::*;

pub struct XcbXvmc {
    pub(crate) lib: NamedLibrary,
    pub(crate) xvmc: XcbXvmcXvmc,
}

lib_entry!(XcbXvmc, "libxcb-xvmc.so.0");
