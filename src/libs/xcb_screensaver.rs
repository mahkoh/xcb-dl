use crate::ffi::*;
use crate::lazy::*;

/// A dynamically loaded `libxcb-screensaver`.
pub struct XcbScreensaver {
    pub(crate) lib: NamedLibrary,
    pub(crate) screensaver: XcbScreensaverScreensaver,
}

lib_entry!(
    XcbScreensaver,
    "libxcb-screensaver.so.0",
    "libxcb-screensaver.so"
);
