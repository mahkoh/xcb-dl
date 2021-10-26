use crate::ffi::*;
use crate::lazy::*;

/// A dynamically loaded `libxcb-xtest`.
pub struct XcbXtest {
    pub(crate) lib: NamedLibrary,
    pub(crate) xtest: XcbXtestXtest,
}

lib_entry!(XcbXtest, "libxcb-xtest.so.0", "libxcb-xtest.so");
