use crate::ffi::*;
use crate::lazy::*;

pub struct XcbDamage {
    pub(crate) lib: NamedLibrary,
    pub(crate) damage: XcbDamageDamage,
}

lib_entry!(XcbDamage, "libxcb-damage.so.0");
