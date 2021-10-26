use crate::ffi::*;
use crate::lazy::*;

/// A dynamically loaded `libxcb-shape`.
pub struct XcbShape {
    pub(crate) lib: NamedLibrary,
    pub(crate) shape: XcbShapeShape,
}

lib_entry!(XcbShape, "libxcb-shape.so.0", "libxcb-shape.so");
