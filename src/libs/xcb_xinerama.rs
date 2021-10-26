use crate::ffi::*;
use crate::lazy::*;

/// A dynamically loaded `libxcb-xinerama`.
pub struct XcbXinerama {
    pub(crate) lib: NamedLibrary,
    pub(crate) xinerama: XcbXineramaXinerama,
}

lib_entry!(XcbXinerama, "libxcb-xinerama.so.0", "libxcb-xinerama.so");
