use crate::ffi::*;
use crate::lazy::*;

pub struct XcbShape {
    pub(crate) lib: NamedLibrary,
    pub(crate) shape: XcbShapeShape,
}

lib_entry!(XcbShape, "libxcb-shape.so.0");
