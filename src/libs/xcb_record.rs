use crate::ffi::*;
use crate::lazy::*;

pub struct XcbRecord {
    pub(crate) lib: NamedLibrary,
    pub(crate) record: XcbRecordRecord,
}

lib_entry!(XcbRecord, "libxcb-record.so.0");
