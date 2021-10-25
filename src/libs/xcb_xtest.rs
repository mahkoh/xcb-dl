use crate::ffi::*;
use crate::lazy::*;

pub struct XcbXtest {
    pub(crate) lib: NamedLibrary,
    pub(crate) xtest: XcbXtestXtest,
}

lib_entry!(XcbXtest, "libxcb-xtest.so.0");
