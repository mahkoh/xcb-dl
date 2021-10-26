use crate::ffi::*;
use crate::lazy::*;

/// A dynamically loaded `libxcb-xselinux`.
pub struct XcbXselinux {
    pub(crate) lib: NamedLibrary,
    pub(crate) xselinux: XcbXselinuxXselinux,
}

lib_entry!(XcbXselinux, "libxcb-xselinux.so.0", "libxcb-xselinux.so");
