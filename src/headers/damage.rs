// This file was generated using generate.py. Do not edit.

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

pub type xcb_damage_damage_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_damage_damage_iterator_t {
    pub data: *mut xcb_damage_damage_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_damage_damage_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_damage_report_level_t = u32;
pub const XCB_DAMAGE_REPORT_LEVEL_RAW_RECTANGLES: xcb_damage_report_level_t = 0;
pub const XCB_DAMAGE_REPORT_LEVEL_DELTA_RECTANGLES: xcb_damage_report_level_t = 1;
pub const XCB_DAMAGE_REPORT_LEVEL_BOUNDING_BOX: xcb_damage_report_level_t = 2;
pub const XCB_DAMAGE_REPORT_LEVEL_NON_EMPTY: xcb_damage_report_level_t = 3;

/// Opcode for xcb_damage_bad_damage.
pub const XCB_DAMAGE_BAD_DAMAGE: u8 = 0i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_damage_bad_damage_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

impl Default for xcb_damage_bad_damage_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_damage_query_version_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_damage_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_damage_query_version.
pub const XCB_DAMAGE_QUERY_VERSION: u8 = 0i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_damage_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub client_major_version: u32,
    pub client_minor_version: u32,
}

impl Default for xcb_damage_query_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_damage_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u32,
    pub minor_version: u32,
    pub pad1: [u8; 16],
}

impl Default for xcb_damage_query_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_damage_create.
pub const XCB_DAMAGE_CREATE: u8 = 1i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_damage_create_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub damage: xcb_damage_damage_t,
    pub drawable: xcb_drawable_t,
    pub level: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_damage_create_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_damage_destroy.
pub const XCB_DAMAGE_DESTROY: u8 = 2i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_damage_destroy_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub damage: xcb_damage_damage_t,
}

impl Default for xcb_damage_destroy_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_damage_subtract.
pub const XCB_DAMAGE_SUBTRACT: u8 = 3i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_damage_subtract_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub damage: xcb_damage_damage_t,
    pub repair: xcb_xfixes_region_t,
    pub parts: xcb_xfixes_region_t,
}

impl Default for xcb_damage_subtract_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_damage_add.
pub const XCB_DAMAGE_ADD: u8 = 4i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_damage_add_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub region: xcb_xfixes_region_t,
}

impl Default for xcb_damage_add_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_damage_notify.
pub const XCB_DAMAGE_NOTIFY: u8 = 0i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_damage_notify_event_t {
    pub response_type: u8,
    pub level: u8,
    pub sequence: u16,
    pub drawable: xcb_drawable_t,
    pub damage: xcb_damage_damage_t,
    pub timestamp: xcb_timestamp_t,
    pub area: xcb_rectangle_t,
    pub geometry: xcb_rectangle_t,
}

impl Default for xcb_damage_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub(crate) struct XcbDamageDamage {
    xcb_damage_id: LazySymbol<*mut xcb_extension_t>,
    xcb_damage_damage_next: LazySymbol<unsafe fn(i: *mut xcb_damage_damage_iterator_t)>,
    xcb_damage_damage_end:
        LazySymbol<unsafe fn(i: xcb_damage_damage_iterator_t) -> xcb_generic_iterator_t>,
    xcb_damage_query_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u32,
            client_minor_version: u32,
        ) -> xcb_damage_query_version_cookie_t,
    >,
    xcb_damage_query_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u32,
            client_minor_version: u32,
        ) -> xcb_damage_query_version_cookie_t,
    >,
    xcb_damage_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_damage_query_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_damage_query_version_reply_t,
    >,
    xcb_damage_create_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            damage: xcb_damage_damage_t,
            drawable: xcb_drawable_t,
            level: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_damage_create: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            damage: xcb_damage_damage_t,
            drawable: xcb_drawable_t,
            level: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_damage_destroy_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, damage: xcb_damage_damage_t) -> xcb_void_cookie_t,
    >,
    xcb_damage_destroy: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, damage: xcb_damage_damage_t) -> xcb_void_cookie_t,
    >,
    xcb_damage_subtract_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            damage: xcb_damage_damage_t,
            repair: xcb_xfixes_region_t,
            parts: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_damage_subtract: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            damage: xcb_damage_damage_t,
            repair: xcb_xfixes_region_t,
            parts: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_damage_add_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            region: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_damage_add: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            region: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self
            .damage
            .$f
            .get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .damage
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

impl XcbDamage {
    pub fn xcb_damage_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_damage_id) }
    }

    /// Returns `true` iff the symbol `xcb_damage_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_damage_id(&self) -> bool {
        has_sym!(self, xcb_damage_id)
    }

    pub unsafe fn xcb_damage_damage_next(&self, i: *mut xcb_damage_damage_iterator_t) {
        sym!(self, xcb_damage_damage_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_damage_damage_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_damage_damage_next(&self) -> bool {
        has_sym!(self, xcb_damage_damage_next)
    }

    pub unsafe fn xcb_damage_damage_end(
        &self,
        i: xcb_damage_damage_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_damage_damage_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_damage_damage_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_damage_damage_end(&self) -> bool {
        has_sym!(self, xcb_damage_damage_end)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_damage_query_version(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u32,
        client_minor_version: u32,
    ) -> xcb_damage_query_version_cookie_t {
        sym!(self, xcb_damage_query_version)(c, client_major_version, client_minor_version)
    }

    /// Returns `true` iff the symbol `xcb_damage_query_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_damage_query_version(&self) -> bool {
        has_sym!(self, xcb_damage_query_version)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will cause
     * a reply to be generated. Any returned error will be
     * placed in the event queue.
     */
    pub unsafe fn xcb_damage_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u32,
        client_minor_version: u32,
    ) -> xcb_damage_query_version_cookie_t {
        sym!(self, xcb_damage_query_version_unchecked)(
            c,
            client_major_version,
            client_minor_version,
        )
    }

    /// Returns `true` iff the symbol `xcb_damage_query_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_damage_query_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_damage_query_version_unchecked)
    }

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_damage_query_version_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_damage_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_damage_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_damage_query_version_reply_t {
        sym!(self, xcb_damage_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_damage_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_damage_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_damage_query_version_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_damage_create_checked(
        &self,
        c: *mut xcb_connection_t,
        damage: xcb_damage_damage_t,
        drawable: xcb_drawable_t,
        level: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_damage_create_checked)(c, damage, drawable, level)
    }

    /// Returns `true` iff the symbol `xcb_damage_create_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_damage_create_checked(&self) -> bool {
        has_sym!(self, xcb_damage_create_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_damage_create(
        &self,
        c: *mut xcb_connection_t,
        damage: xcb_damage_damage_t,
        drawable: xcb_drawable_t,
        level: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_damage_create)(c, damage, drawable, level)
    }

    /// Returns `true` iff the symbol `xcb_damage_create` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_damage_create(&self) -> bool {
        has_sym!(self, xcb_damage_create)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_damage_destroy_checked(
        &self,
        c: *mut xcb_connection_t,
        damage: xcb_damage_damage_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_damage_destroy_checked)(c, damage)
    }

    /// Returns `true` iff the symbol `xcb_damage_destroy_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_damage_destroy_checked(&self) -> bool {
        has_sym!(self, xcb_damage_destroy_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_damage_destroy(
        &self,
        c: *mut xcb_connection_t,
        damage: xcb_damage_damage_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_damage_destroy)(c, damage)
    }

    /// Returns `true` iff the symbol `xcb_damage_destroy` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_damage_destroy(&self) -> bool {
        has_sym!(self, xcb_damage_destroy)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_damage_subtract_checked(
        &self,
        c: *mut xcb_connection_t,
        damage: xcb_damage_damage_t,
        repair: xcb_xfixes_region_t,
        parts: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_damage_subtract_checked)(c, damage, repair, parts)
    }

    /// Returns `true` iff the symbol `xcb_damage_subtract_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_damage_subtract_checked(&self) -> bool {
        has_sym!(self, xcb_damage_subtract_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_damage_subtract(
        &self,
        c: *mut xcb_connection_t,
        damage: xcb_damage_damage_t,
        repair: xcb_xfixes_region_t,
        parts: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_damage_subtract)(c, damage, repair, parts)
    }

    /// Returns `true` iff the symbol `xcb_damage_subtract` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_damage_subtract(&self) -> bool {
        has_sym!(self, xcb_damage_subtract)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     * This form can be used only if the request will not cause
     * a reply to be generated. Any returned error will be
     * saved for handling by xcb_request_check().
     */
    pub unsafe fn xcb_damage_add_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        region: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_damage_add_checked)(c, drawable, region)
    }

    /// Returns `true` iff the symbol `xcb_damage_add_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_damage_add_checked(&self) -> bool {
        has_sym!(self, xcb_damage_add_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_damage_add(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        region: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_damage_add)(c, drawable, region)
    }

    /// Returns `true` iff the symbol `xcb_damage_add` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_damage_add(&self) -> bool {
        has_sym!(self, xcb_damage_add)
    }
}

#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbDamage::load().unwrap() };
        assert!(lib.has_xcb_damage_id());
        assert!(lib.has_xcb_damage_damage_next());
        assert!(lib.has_xcb_damage_damage_end());
        assert!(lib.has_xcb_damage_query_version());
        assert!(lib.has_xcb_damage_query_version_unchecked());
        assert!(lib.has_xcb_damage_query_version_reply());
        assert!(lib.has_xcb_damage_create_checked());
        assert!(lib.has_xcb_damage_create());
        assert!(lib.has_xcb_damage_destroy_checked());
        assert!(lib.has_xcb_damage_destroy());
        assert!(lib.has_xcb_damage_subtract_checked());
        assert!(lib.has_xcb_damage_subtract());
        assert!(lib.has_xcb_damage_add_checked());
        assert!(lib.has_xcb_damage_add());
    }
}
