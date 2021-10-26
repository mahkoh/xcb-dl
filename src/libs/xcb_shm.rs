use crate::ffi::*;
use crate::lazy::*;

/// A dynamically loaded `libxcb-shm`.
pub struct XcbShm {
    pub(crate) lib: NamedLibrary,
    pub(crate) shm: XcbShmShm,
}

lib_entry!(XcbShm, "libxcb-shm.so.0", "libxcb-shm.so");
