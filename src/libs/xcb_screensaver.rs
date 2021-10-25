use crate::ffi::*;
use crate::lazy::*;

pub struct XcbScreensaver {
    pub(crate) lib: NamedLibrary,
    pub(crate) screensaver: XcbScreensaverScreensaver,
}

lib_entry!(XcbScreensaver, "libxcb-screensaver.so.0");
