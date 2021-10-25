use crate::ffi::*;
use crate::lazy::*;

pub struct XcbSync {
    pub(crate) lib: NamedLibrary,
    pub(crate) sync: XcbSyncSync,
}

lib_entry!(XcbSync, "libxcb-sync.so.1");
