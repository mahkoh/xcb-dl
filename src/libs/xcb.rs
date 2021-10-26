use crate::ffi::*;
use crate::lazy::*;

pub struct Xcb {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb: XcbXcb,
    pub(crate) xcbext: XcbXcbext,
    pub(crate) xproto: XcbXproto,
    pub(crate) bigreq: XcbBigreq,
    pub(crate) xc_misc: XcbXc_Misc,
}

lib_entry!(Xcb, "libxcb.so.1");
