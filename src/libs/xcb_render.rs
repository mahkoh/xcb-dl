use crate::ffi::*;
use crate::lazy::*;

/// A dynamically loaded `libxcb-render`.
pub struct XcbRender {
    pub(crate) lib: NamedLibrary,
    pub(crate) render: XcbRenderRender,
}

lib_entry!(XcbRender, "libxcb-render.so.0", "libxcb-render.so");
