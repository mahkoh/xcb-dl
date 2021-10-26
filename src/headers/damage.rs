// This file was generated using generate.py. Do not edit.
#![allow(unused_macros)]

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

/// The `Damage::DAMAGE` type.
pub type xcb_damage_damage_t = u32;

/// An iterator over `Damage::DAMAGE` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_damage_damage_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_damage_damage_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_damage_damage_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Damage::ReportLevel` enum.
///
/// This enum has the following variants:
///
/// - [`Damage::ReportLevel::RawRectangles`](XCB_DAMAGE_REPORT_LEVEL_RAW_RECTANGLES)
/// - [`Damage::ReportLevel::DeltaRectangles`](XCB_DAMAGE_REPORT_LEVEL_DELTA_RECTANGLES)
/// - [`Damage::ReportLevel::BoundingBox`](XCB_DAMAGE_REPORT_LEVEL_BOUNDING_BOX)
/// - [`Damage::ReportLevel::NonEmpty`](XCB_DAMAGE_REPORT_LEVEL_NON_EMPTY)
pub type xcb_damage_report_level_t = u32;
/// The `Damage::ReportLevel::RawRectangles` enum variant.
///
/// This is a variant of [`xcb_damage_report_level_t`].
pub const XCB_DAMAGE_REPORT_LEVEL_RAW_RECTANGLES: xcb_damage_report_level_t = 0;
/// The `Damage::ReportLevel::DeltaRectangles` enum variant.
///
/// This is a variant of [`xcb_damage_report_level_t`].
pub const XCB_DAMAGE_REPORT_LEVEL_DELTA_RECTANGLES: xcb_damage_report_level_t = 1;
/// The `Damage::ReportLevel::BoundingBox` enum variant.
///
/// This is a variant of [`xcb_damage_report_level_t`].
pub const XCB_DAMAGE_REPORT_LEVEL_BOUNDING_BOX: xcb_damage_report_level_t = 2;
/// The `Damage::ReportLevel::NonEmpty` enum variant.
///
/// This is a variant of [`xcb_damage_report_level_t`].
pub const XCB_DAMAGE_REPORT_LEVEL_NON_EMPTY: xcb_damage_report_level_t = 3;

/// The opcode for `Damage::BadDamage` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_damage_bad_damage_error_t`].
pub const XCB_DAMAGE_BAD_DAMAGE: u8 = 0i32 as u8;

/// The `Damage::BadDamage` error.
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

/// The cookie for the reply to a `Damage::QueryVersion` request.
///
/// Pass this cookie to [`xcb_damage_query_version_reply`] to retrieve the reply.
///
/// [`xcb_damage_query_version_reply`]: XcbDamage::xcb_damage_query_version_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_damage_query_version_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_damage_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Damage::QueryVersion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDamage::xcb_damage_id()`], then the type of the request is
/// [`xcb_damage_query_version_request_t`].
pub const XCB_DAMAGE_QUERY_VERSION: u8 = 0i32 as u8;

/// The `Damage::QueryVersion` request.
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

/// The `Damage::QueryVersion` reply.
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

/// The opcode for `Damage::Create` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDamage::xcb_damage_id()`], then the type of the request is
/// [`xcb_damage_create_request_t`].
pub const XCB_DAMAGE_CREATE: u8 = 1i32 as u8;

/// The `Damage::Create` request.
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

/// The opcode for `Damage::Destroy` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDamage::xcb_damage_id()`], then the type of the request is
/// [`xcb_damage_destroy_request_t`].
pub const XCB_DAMAGE_DESTROY: u8 = 2i32 as u8;

/// The `Damage::Destroy` request.
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

/// The opcode for `Damage::Subtract` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDamage::xcb_damage_id()`], then the type of the request is
/// [`xcb_damage_subtract_request_t`].
pub const XCB_DAMAGE_SUBTRACT: u8 = 3i32 as u8;

/// The `Damage::Subtract` request.
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

/// The opcode for `Damage::Add` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbDamage::xcb_damage_id()`], then the type of the request is
/// [`xcb_damage_add_request_t`].
pub const XCB_DAMAGE_ADD: u8 = 4i32 as u8;

/// The `Damage::Add` request.
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

/// The opcode for `Damage::Notify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_damage_notify_event_t`].
pub const XCB_DAMAGE_NOTIFY: u8 = 0i32 as u8;

/// The `Damage::Notify` event.
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

#[cfg(feature = "xcb_damage")]
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
        ) -> *mut xcb_damage_query_version_reply_t,
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

#[cfg(feature = "xcb_damage")]
impl XcbDamage {
    /// The libxcb identifier of the `Damage` extension.
    #[inline]
    pub fn xcb_damage_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_damage_id) }
    }

    /// Returns `true` iff the symbol `xcb_damage_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_damage_id(&self) -> bool {
        has_sym!(self, xcb_damage_id)
    }

    /// Advances a `xcb_damage_damage_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_damage_damage_next(&self, i: *mut xcb_damage_damage_iterator_t) {
        sym!(self, xcb_damage_damage_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_damage_damage_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_damage_damage_next(&self) -> bool {
        has_sym!(self, xcb_damage_damage_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_damage_damage_iterator_t`.
    #[inline]
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

    /// Sends a `Damage::QueryVersion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_damage_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_damage_query_version_reply`]: Self::xcb_damage_query_version_reply
    #[inline]
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

    /// Sends a `Damage::QueryVersion` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_damage_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_damage_query_version_reply`]: Self::xcb_damage_query_version_reply
    #[inline]
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

    /// Waits for the reply to a `Damage::QueryVersion` request.
    #[inline]
    pub unsafe fn xcb_damage_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_damage_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_damage_query_version_reply_t {
        sym!(self, xcb_damage_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_damage_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_damage_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_damage_query_version_reply)
    }

    /// Sends a `Damage::Create` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
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

    /// Sends a `Damage::Create` request (unchecked).
    #[inline]
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

    /// Sends a `Damage::Destroy` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
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

    /// Sends a `Damage::Destroy` request (unchecked).
    #[inline]
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

    /// Sends a `Damage::Subtract` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
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

    /// Sends a `Damage::Subtract` request (unchecked).
    #[inline]
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

    /// Sends a `Damage::Add` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
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

    /// Sends a `Damage::Add` request (unchecked).
    #[inline]
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

#[cfg(feature = "xcb_damage")]
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
