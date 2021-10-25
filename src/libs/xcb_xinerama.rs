use crate::ffi::*;
use crate::lazy::*;

pub struct XcbXinerama {
    pub(crate) lib: NamedLibrary,
    pub(crate) xinerama: XcbXineramaXinerama,
}

lib_entry!(XcbXinerama, "libxcb-xinerama.so.0");
