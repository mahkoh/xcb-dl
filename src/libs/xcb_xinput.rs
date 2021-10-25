use crate::ffi::*;
use crate::lazy::*;

pub struct XcbXinput {
    pub(crate) lib: NamedLibrary,
    pub(crate) xinput: XcbXinputXinput,
}

lib_entry!(XcbXinput, "libxcb-xinput.so.0");
