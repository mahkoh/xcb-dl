use crate::ffi::*;
use crate::lazy::*;

pub struct XcbShm {
    pub(crate) lib: NamedLibrary,
    pub(crate) shm: XcbShmShm,
}

lib_entry!(XcbShm, "libxcb-shm.so.0");
