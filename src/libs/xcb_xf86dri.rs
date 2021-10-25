use crate::ffi::*;
use crate::lazy::*;

pub struct XcbXf86dri {
    pub(crate) lib: NamedLibrary,
    pub(crate) xf86dri: XcbXf86driXf86Dri,
}

lib_entry!(XcbXf86dri, "libxcb-xf86dri.so.0");
