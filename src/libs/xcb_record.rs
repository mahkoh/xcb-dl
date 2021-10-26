use crate::ffi::*;
use crate::lazy::*;

/// A dynamically loaded `libxcb-record`.
pub struct XcbRecord {
    pub(crate) lib: NamedLibrary,
    pub(crate) record: XcbRecordRecord,
}

lib_entry!(XcbRecord, "libxcb-record.so.0", "libxcb-record.so");
