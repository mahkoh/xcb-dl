// This file was generated using generate.py. Do not edit.
#![allow(unused_macros)]

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

/// The `Composite::Redirect` enum.
///
/// This enum has the following variants:
///
/// - [`Composite::Redirect::Automatic`](XCB_COMPOSITE_REDIRECT_AUTOMATIC)
/// - [`Composite::Redirect::Manual`](XCB_COMPOSITE_REDIRECT_MANUAL)
pub type xcb_composite_redirect_t = u32;
/// The `Composite::Redirect::Automatic` enum variant.
///
/// This is a variant of [`xcb_composite_redirect_t`].
pub const XCB_COMPOSITE_REDIRECT_AUTOMATIC: xcb_composite_redirect_t = 0;
/// The `Composite::Redirect::Manual` enum variant.
///
/// This is a variant of [`xcb_composite_redirect_t`].
pub const XCB_COMPOSITE_REDIRECT_MANUAL: xcb_composite_redirect_t = 1;

/// The cookie for the reply to a `Composite::QueryVersion` request.
///
/// Pass this cookie to [`xcb_composite_query_version_reply`] to retrieve the reply.
///
/// [`xcb_composite_query_version_reply`]: XcbComposite::xcb_composite_query_version_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_composite_query_version_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_composite_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Composite::QueryVersion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbComposite::xcb_composite_id()`], then the type of the request is
/// [`xcb_composite_query_version_request_t`].
pub const XCB_COMPOSITE_QUERY_VERSION: u8 = 0i32 as u8;

/// The `Composite::QueryVersion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_composite_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub client_major_version: u32,
    pub client_minor_version: u32,
}

impl Default for xcb_composite_query_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Composite::QueryVersion` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_composite_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u32,
    pub minor_version: u32,
    pub pad1: [u8; 16],
}

impl Default for xcb_composite_query_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Composite::RedirectWindow` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbComposite::xcb_composite_id()`], then the type of the request is
/// [`xcb_composite_redirect_window_request_t`].
pub const XCB_COMPOSITE_REDIRECT_WINDOW: u8 = 1i32 as u8;

/// The `Composite::RedirectWindow` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_composite_redirect_window_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub update: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_composite_redirect_window_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Composite::RedirectSubwindows` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbComposite::xcb_composite_id()`], then the type of the request is
/// [`xcb_composite_redirect_subwindows_request_t`].
pub const XCB_COMPOSITE_REDIRECT_SUBWINDOWS: u8 = 2i32 as u8;

/// The `Composite::RedirectSubwindows` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_composite_redirect_subwindows_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub update: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_composite_redirect_subwindows_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Composite::UnredirectWindow` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbComposite::xcb_composite_id()`], then the type of the request is
/// [`xcb_composite_unredirect_window_request_t`].
pub const XCB_COMPOSITE_UNREDIRECT_WINDOW: u8 = 3i32 as u8;

/// The `Composite::UnredirectWindow` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_composite_unredirect_window_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub update: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_composite_unredirect_window_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Composite::UnredirectSubwindows` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbComposite::xcb_composite_id()`], then the type of the request is
/// [`xcb_composite_unredirect_subwindows_request_t`].
pub const XCB_COMPOSITE_UNREDIRECT_SUBWINDOWS: u8 = 4i32 as u8;

/// The `Composite::UnredirectSubwindows` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_composite_unredirect_subwindows_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub update: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_composite_unredirect_subwindows_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Composite::CreateRegionFromBorderClip` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbComposite::xcb_composite_id()`], then the type of the request is
/// [`xcb_composite_create_region_from_border_clip_request_t`].
pub const XCB_COMPOSITE_CREATE_REGION_FROM_BORDER_CLIP: u8 = 5i32 as u8;

/// The `Composite::CreateRegionFromBorderClip` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_composite_create_region_from_border_clip_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub region: xcb_xfixes_region_t,
    pub window: xcb_window_t,
}

impl Default for xcb_composite_create_region_from_border_clip_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Composite::NameWindowPixmap` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbComposite::xcb_composite_id()`], then the type of the request is
/// [`xcb_composite_name_window_pixmap_request_t`].
pub const XCB_COMPOSITE_NAME_WINDOW_PIXMAP: u8 = 6i32 as u8;

/// The `Composite::NameWindowPixmap` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_composite_name_window_pixmap_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub pixmap: xcb_pixmap_t,
}

impl Default for xcb_composite_name_window_pixmap_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Composite::GetOverlayWindow` request.
///
/// Pass this cookie to [`xcb_composite_get_overlay_window_reply`] to retrieve the reply.
///
/// [`xcb_composite_get_overlay_window_reply`]: XcbComposite::xcb_composite_get_overlay_window_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_composite_get_overlay_window_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_composite_get_overlay_window_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Composite::GetOverlayWindow` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbComposite::xcb_composite_id()`], then the type of the request is
/// [`xcb_composite_get_overlay_window_request_t`].
pub const XCB_COMPOSITE_GET_OVERLAY_WINDOW: u8 = 7i32 as u8;

/// The `Composite::GetOverlayWindow` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_composite_get_overlay_window_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_composite_get_overlay_window_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Composite::GetOverlayWindow` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_composite_get_overlay_window_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub overlay_win: xcb_window_t,
    pub pad1: [u8; 20],
}

impl Default for xcb_composite_get_overlay_window_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Composite::ReleaseOverlayWindow` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbComposite::xcb_composite_id()`], then the type of the request is
/// [`xcb_composite_release_overlay_window_request_t`].
pub const XCB_COMPOSITE_RELEASE_OVERLAY_WINDOW: u8 = 8i32 as u8;

/// The `Composite::ReleaseOverlayWindow` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_composite_release_overlay_window_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_composite_release_overlay_window_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[cfg(feature = "xcb_composite")]
pub(crate) struct XcbCompositeComposite {
    xcb_composite_id: LazySymbol<*mut xcb_extension_t>,
    xcb_composite_query_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u32,
            client_minor_version: u32,
        ) -> xcb_composite_query_version_cookie_t,
    >,
    xcb_composite_query_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u32,
            client_minor_version: u32,
        ) -> xcb_composite_query_version_cookie_t,
    >,
    xcb_composite_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_composite_query_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_composite_query_version_reply_t,
    >,
    xcb_composite_redirect_window_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, update: u8) -> xcb_void_cookie_t,
    >,
    xcb_composite_redirect_window: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, update: u8) -> xcb_void_cookie_t,
    >,
    xcb_composite_redirect_subwindows_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, update: u8) -> xcb_void_cookie_t,
    >,
    xcb_composite_redirect_subwindows: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, update: u8) -> xcb_void_cookie_t,
    >,
    xcb_composite_unredirect_window_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, update: u8) -> xcb_void_cookie_t,
    >,
    xcb_composite_unredirect_window: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, update: u8) -> xcb_void_cookie_t,
    >,
    xcb_composite_unredirect_subwindows_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, update: u8) -> xcb_void_cookie_t,
    >,
    xcb_composite_unredirect_subwindows: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t, update: u8) -> xcb_void_cookie_t,
    >,
    xcb_composite_create_region_from_border_clip_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            window: xcb_window_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_composite_create_region_from_border_clip: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            window: xcb_window_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_composite_name_window_pixmap_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            pixmap: xcb_pixmap_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_composite_name_window_pixmap: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            pixmap: xcb_pixmap_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_composite_get_overlay_window: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_composite_get_overlay_window_cookie_t,
    >,
    xcb_composite_get_overlay_window_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_composite_get_overlay_window_cookie_t,
    >,
    xcb_composite_get_overlay_window_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_composite_get_overlay_window_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_composite_get_overlay_window_reply_t,
    >,
    xcb_composite_release_overlay_window_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_composite_release_overlay_window:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self
            .composite
            .$f
            .get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .composite
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

#[cfg(feature = "xcb_composite")]
impl XcbComposite {
    /// The libxcb identifier of the `Composite` extension.
    #[inline]
    pub fn xcb_composite_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_composite_id) }
    }

    /// Returns `true` iff the symbol `xcb_composite_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_composite_id(&self) -> bool {
        has_sym!(self, xcb_composite_id)
    }

    /// Sends a `Composite::QueryVersion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_composite_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_composite_query_version_reply`]: Self::xcb_composite_query_version_reply
    #[inline]
    pub unsafe fn xcb_composite_query_version(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u32,
        client_minor_version: u32,
    ) -> xcb_composite_query_version_cookie_t {
        sym!(self, xcb_composite_query_version)(c, client_major_version, client_minor_version)
    }

    /// Returns `true` iff the symbol `xcb_composite_query_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_composite_query_version(&self) -> bool {
        has_sym!(self, xcb_composite_query_version)
    }

    /// Sends a `Composite::QueryVersion` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_composite_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_composite_query_version_reply`]: Self::xcb_composite_query_version_reply
    #[inline]
    pub unsafe fn xcb_composite_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u32,
        client_minor_version: u32,
    ) -> xcb_composite_query_version_cookie_t {
        sym!(self, xcb_composite_query_version_unchecked)(
            c,
            client_major_version,
            client_minor_version,
        )
    }

    /// Returns `true` iff the symbol `xcb_composite_query_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_composite_query_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_composite_query_version_unchecked)
    }

    /// Waits for the reply to a `Composite::QueryVersion` request.
    #[inline]
    pub unsafe fn xcb_composite_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_composite_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_composite_query_version_reply_t {
        sym!(self, xcb_composite_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_composite_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_composite_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_composite_query_version_reply)
    }

    /// Sends a `Composite::RedirectWindow` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_composite_redirect_window_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        update: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_composite_redirect_window_checked)(c, window, update)
    }

    /// Returns `true` iff the symbol `xcb_composite_redirect_window_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_composite_redirect_window_checked(&self) -> bool {
        has_sym!(self, xcb_composite_redirect_window_checked)
    }

    /// Sends a `Composite::RedirectWindow` request (unchecked).
    #[inline]
    pub unsafe fn xcb_composite_redirect_window(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        update: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_composite_redirect_window)(c, window, update)
    }

    /// Returns `true` iff the symbol `xcb_composite_redirect_window` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_composite_redirect_window(&self) -> bool {
        has_sym!(self, xcb_composite_redirect_window)
    }

    /// Sends a `Composite::RedirectSubwindows` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_composite_redirect_subwindows_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        update: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_composite_redirect_subwindows_checked)(c, window, update)
    }

    /// Returns `true` iff the symbol `xcb_composite_redirect_subwindows_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_composite_redirect_subwindows_checked(&self) -> bool {
        has_sym!(self, xcb_composite_redirect_subwindows_checked)
    }

    /// Sends a `Composite::RedirectSubwindows` request (unchecked).
    #[inline]
    pub unsafe fn xcb_composite_redirect_subwindows(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        update: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_composite_redirect_subwindows)(c, window, update)
    }

    /// Returns `true` iff the symbol `xcb_composite_redirect_subwindows` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_composite_redirect_subwindows(&self) -> bool {
        has_sym!(self, xcb_composite_redirect_subwindows)
    }

    /// Sends a `Composite::UnredirectWindow` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_composite_unredirect_window_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        update: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_composite_unredirect_window_checked)(c, window, update)
    }

    /// Returns `true` iff the symbol `xcb_composite_unredirect_window_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_composite_unredirect_window_checked(&self) -> bool {
        has_sym!(self, xcb_composite_unredirect_window_checked)
    }

    /// Sends a `Composite::UnredirectWindow` request (unchecked).
    #[inline]
    pub unsafe fn xcb_composite_unredirect_window(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        update: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_composite_unredirect_window)(c, window, update)
    }

    /// Returns `true` iff the symbol `xcb_composite_unredirect_window` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_composite_unredirect_window(&self) -> bool {
        has_sym!(self, xcb_composite_unredirect_window)
    }

    /// Sends a `Composite::UnredirectSubwindows` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_composite_unredirect_subwindows_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        update: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_composite_unredirect_subwindows_checked)(c, window, update)
    }

    /// Returns `true` iff the symbol `xcb_composite_unredirect_subwindows_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_composite_unredirect_subwindows_checked(&self) -> bool {
        has_sym!(self, xcb_composite_unredirect_subwindows_checked)
    }

    /// Sends a `Composite::UnredirectSubwindows` request (unchecked).
    #[inline]
    pub unsafe fn xcb_composite_unredirect_subwindows(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        update: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_composite_unredirect_subwindows)(c, window, update)
    }

    /// Returns `true` iff the symbol `xcb_composite_unredirect_subwindows` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_composite_unredirect_subwindows(&self) -> bool {
        has_sym!(self, xcb_composite_unredirect_subwindows)
    }

    /// Sends a `Composite::CreateRegionFromBorderClip` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_composite_create_region_from_border_clip_checked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_composite_create_region_from_border_clip_checked)(c, region, window)
    }

    /// Returns `true` iff the symbol `xcb_composite_create_region_from_border_clip_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_composite_create_region_from_border_clip_checked(&self) -> bool {
        has_sym!(self, xcb_composite_create_region_from_border_clip_checked)
    }

    /// Sends a `Composite::CreateRegionFromBorderClip` request (unchecked).
    #[inline]
    pub unsafe fn xcb_composite_create_region_from_border_clip(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_composite_create_region_from_border_clip)(c, region, window)
    }

    /// Returns `true` iff the symbol `xcb_composite_create_region_from_border_clip` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_composite_create_region_from_border_clip(&self) -> bool {
        has_sym!(self, xcb_composite_create_region_from_border_clip)
    }

    /// Sends a `Composite::NameWindowPixmap` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_composite_name_window_pixmap_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        pixmap: xcb_pixmap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_composite_name_window_pixmap_checked)(c, window, pixmap)
    }

    /// Returns `true` iff the symbol `xcb_composite_name_window_pixmap_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_composite_name_window_pixmap_checked(&self) -> bool {
        has_sym!(self, xcb_composite_name_window_pixmap_checked)
    }

    /// Sends a `Composite::NameWindowPixmap` request (unchecked).
    #[inline]
    pub unsafe fn xcb_composite_name_window_pixmap(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        pixmap: xcb_pixmap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_composite_name_window_pixmap)(c, window, pixmap)
    }

    /// Returns `true` iff the symbol `xcb_composite_name_window_pixmap` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_composite_name_window_pixmap(&self) -> bool {
        has_sym!(self, xcb_composite_name_window_pixmap)
    }

    /// Sends a `Composite::GetOverlayWindow` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_composite_get_overlay_window_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_composite_get_overlay_window_reply`]: Self::xcb_composite_get_overlay_window_reply
    #[inline]
    pub unsafe fn xcb_composite_get_overlay_window(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_composite_get_overlay_window_cookie_t {
        sym!(self, xcb_composite_get_overlay_window)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_composite_get_overlay_window` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_composite_get_overlay_window(&self) -> bool {
        has_sym!(self, xcb_composite_get_overlay_window)
    }

    /// Sends a `Composite::GetOverlayWindow` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_composite_get_overlay_window_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_composite_get_overlay_window_reply`]: Self::xcb_composite_get_overlay_window_reply
    #[inline]
    pub unsafe fn xcb_composite_get_overlay_window_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_composite_get_overlay_window_cookie_t {
        sym!(self, xcb_composite_get_overlay_window_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_composite_get_overlay_window_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_composite_get_overlay_window_unchecked(&self) -> bool {
        has_sym!(self, xcb_composite_get_overlay_window_unchecked)
    }

    /// Waits for the reply to a `Composite::GetOverlayWindow` request.
    #[inline]
    pub unsafe fn xcb_composite_get_overlay_window_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_composite_get_overlay_window_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_composite_get_overlay_window_reply_t {
        sym!(self, xcb_composite_get_overlay_window_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_composite_get_overlay_window_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_composite_get_overlay_window_reply(&self) -> bool {
        has_sym!(self, xcb_composite_get_overlay_window_reply)
    }

    /// Sends a `Composite::ReleaseOverlayWindow` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_composite_release_overlay_window_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_composite_release_overlay_window_checked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_composite_release_overlay_window_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_composite_release_overlay_window_checked(&self) -> bool {
        has_sym!(self, xcb_composite_release_overlay_window_checked)
    }

    /// Sends a `Composite::ReleaseOverlayWindow` request (unchecked).
    #[inline]
    pub unsafe fn xcb_composite_release_overlay_window(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_composite_release_overlay_window)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_composite_release_overlay_window` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_composite_release_overlay_window(&self) -> bool {
        has_sym!(self, xcb_composite_release_overlay_window)
    }
}

#[cfg(feature = "xcb_composite")]
#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbComposite::load().unwrap() };
        assert!(lib.has_xcb_composite_id());
        assert!(lib.has_xcb_composite_query_version());
        assert!(lib.has_xcb_composite_query_version_unchecked());
        assert!(lib.has_xcb_composite_query_version_reply());
        assert!(lib.has_xcb_composite_redirect_window_checked());
        assert!(lib.has_xcb_composite_redirect_window());
        assert!(lib.has_xcb_composite_redirect_subwindows_checked());
        assert!(lib.has_xcb_composite_redirect_subwindows());
        assert!(lib.has_xcb_composite_unredirect_window_checked());
        assert!(lib.has_xcb_composite_unredirect_window());
        assert!(lib.has_xcb_composite_unredirect_subwindows_checked());
        assert!(lib.has_xcb_composite_unredirect_subwindows());
        assert!(lib.has_xcb_composite_create_region_from_border_clip_checked());
        assert!(lib.has_xcb_composite_create_region_from_border_clip());
        assert!(lib.has_xcb_composite_name_window_pixmap_checked());
        assert!(lib.has_xcb_composite_name_window_pixmap());
        assert!(lib.has_xcb_composite_get_overlay_window());
        assert!(lib.has_xcb_composite_get_overlay_window_unchecked());
        assert!(lib.has_xcb_composite_get_overlay_window_reply());
        assert!(lib.has_xcb_composite_release_overlay_window_checked());
        assert!(lib.has_xcb_composite_release_overlay_window());
    }
}
