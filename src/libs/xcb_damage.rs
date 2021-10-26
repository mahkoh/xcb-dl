use crate::ffi::*;
use crate::lazy::*;

/// A dynamically loaded `libxcb-damage`.
pub struct XcbDamage {
    pub(crate) lib: NamedLibrary,
    pub(crate) damage: XcbDamageDamage,
}

lib_entry!(XcbDamage, "libxcb-damage.so.0", "libxcb-damage.so");
