use crate::ffi::*;
use crate::lazy::*;

/// A dynamically loaded `libxcb-xinput`.
pub struct XcbXinput {
    pub(crate) lib: NamedLibrary,
    pub(crate) xinput: XcbXinputXinput,
}

lib_entry!(XcbXinput, "libxcb-xinput.so.0", "libxcb-xinput.so");
