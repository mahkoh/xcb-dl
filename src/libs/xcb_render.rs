use crate::ffi::*;
use crate::lazy::*;

pub struct XcbRender {
    pub(crate) lib: NamedLibrary,
    pub(crate) render: XcbRenderRender,
}

lib_entry!(XcbRender, "libxcb-render.so.0");
