// This file was generated using generate.py. Do not edit.
#![allow(unused_macros)]

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

/// The name of the `ScreenSaver` extension.
pub const XCB_SCREENSAVER_NAME: &[u8] = b"MIT-SCREEN-SAVER";

/// The name of the `ScreenSaver` extension.
pub const XCB_SCREENSAVER_NAME_STR: &str = "MIT-SCREEN-SAVER";

/// The `ScreenSaver::Kind` enum.
///
/// This enum has the following variants:
///
/// - [`ScreenSaver::Kind::Blanked`](XCB_SCREENSAVER_KIND_BLANKED)
/// - [`ScreenSaver::Kind::Internal`](XCB_SCREENSAVER_KIND_INTERNAL)
/// - [`ScreenSaver::Kind::External`](XCB_SCREENSAVER_KIND_EXTERNAL)
pub type xcb_screensaver_kind_t = u32;
/// The `ScreenSaver::Kind::Blanked` enum variant.
///
/// This is a variant of [`xcb_screensaver_kind_t`].
pub const XCB_SCREENSAVER_KIND_BLANKED: xcb_screensaver_kind_t = 0;
/// The `ScreenSaver::Kind::Internal` enum variant.
///
/// This is a variant of [`xcb_screensaver_kind_t`].
pub const XCB_SCREENSAVER_KIND_INTERNAL: xcb_screensaver_kind_t = 1;
/// The `ScreenSaver::Kind::External` enum variant.
///
/// This is a variant of [`xcb_screensaver_kind_t`].
pub const XCB_SCREENSAVER_KIND_EXTERNAL: xcb_screensaver_kind_t = 2;

/// The `ScreenSaver::Event` enum.
///
/// This enum has the following variants:
///
/// - [`ScreenSaver::Event::NotifyMask`](XCB_SCREENSAVER_EVENT_NOTIFY_MASK)
/// - [`ScreenSaver::Event::CycleMask`](XCB_SCREENSAVER_EVENT_CYCLE_MASK)
pub type xcb_screensaver_event_t = u32;
/// The `ScreenSaver::Event::NotifyMask` enum variant.
///
/// This is a variant of [`xcb_screensaver_event_t`].
pub const XCB_SCREENSAVER_EVENT_NOTIFY_MASK: xcb_screensaver_event_t = 1;
/// The `ScreenSaver::Event::CycleMask` enum variant.
///
/// This is a variant of [`xcb_screensaver_event_t`].
pub const XCB_SCREENSAVER_EVENT_CYCLE_MASK: xcb_screensaver_event_t = 2;

/// The `ScreenSaver::State` enum.
///
/// This enum has the following variants:
///
/// - [`ScreenSaver::State::Off`](XCB_SCREENSAVER_STATE_OFF)
/// - [`ScreenSaver::State::On`](XCB_SCREENSAVER_STATE_ON)
/// - [`ScreenSaver::State::Cycle`](XCB_SCREENSAVER_STATE_CYCLE)
/// - [`ScreenSaver::State::Disabled`](XCB_SCREENSAVER_STATE_DISABLED)
pub type xcb_screensaver_state_t = u32;
/// The `ScreenSaver::State::Off` enum variant.
///
/// This is a variant of [`xcb_screensaver_state_t`].
pub const XCB_SCREENSAVER_STATE_OFF: xcb_screensaver_state_t = 0;
/// The `ScreenSaver::State::On` enum variant.
///
/// This is a variant of [`xcb_screensaver_state_t`].
pub const XCB_SCREENSAVER_STATE_ON: xcb_screensaver_state_t = 1;
/// The `ScreenSaver::State::Cycle` enum variant.
///
/// This is a variant of [`xcb_screensaver_state_t`].
pub const XCB_SCREENSAVER_STATE_CYCLE: xcb_screensaver_state_t = 2;
/// The `ScreenSaver::State::Disabled` enum variant.
///
/// This is a variant of [`xcb_screensaver_state_t`].
pub const XCB_SCREENSAVER_STATE_DISABLED: xcb_screensaver_state_t = 3;

/// The cookie for the reply to a `ScreenSaver::QueryVersion` request.
///
/// Pass this cookie to [`xcb_screensaver_query_version_reply`] to retrieve the reply.
///
/// [`xcb_screensaver_query_version_reply`]: XcbScreensaver::xcb_screensaver_query_version_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screensaver_query_version_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_screensaver_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ScreenSaver::QueryVersion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbScreensaver::xcb_screensaver_id()`], then the type of the request is
/// [`xcb_screensaver_query_version_request_t`].
pub const XCB_SCREENSAVER_QUERY_VERSION: u8 = 0i32 as u8;

/// The `ScreenSaver::QueryVersion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screensaver_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub client_major_version: u8,
    pub client_minor_version: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_screensaver_query_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `ScreenSaver::QueryVersion` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screensaver_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub server_major_version: u16,
    pub server_minor_version: u16,
    pub pad1: [u8; 20],
}

impl Default for xcb_screensaver_query_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `ScreenSaver::QueryInfo` request.
///
/// Pass this cookie to [`xcb_screensaver_query_info_reply`] to retrieve the reply.
///
/// [`xcb_screensaver_query_info_reply`]: XcbScreensaver::xcb_screensaver_query_info_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screensaver_query_info_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_screensaver_query_info_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ScreenSaver::QueryInfo` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbScreensaver::xcb_screensaver_id()`], then the type of the request is
/// [`xcb_screensaver_query_info_request_t`].
pub const XCB_SCREENSAVER_QUERY_INFO: u8 = 1i32 as u8;

/// The `ScreenSaver::QueryInfo` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screensaver_query_info_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
}

impl Default for xcb_screensaver_query_info_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `ScreenSaver::QueryInfo` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screensaver_query_info_reply_t {
    pub response_type: u8,
    pub state: u8,
    pub sequence: u16,
    pub length: u32,
    pub saver_window: xcb_window_t,
    pub ms_until_server: u32,
    pub ms_since_user_input: u32,
    pub event_mask: u32,
    pub kind: u8,
    pub pad0: [u8; 7],
}

impl Default for xcb_screensaver_query_info_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ScreenSaver::SelectInput` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbScreensaver::xcb_screensaver_id()`], then the type of the request is
/// [`xcb_screensaver_select_input_request_t`].
pub const XCB_SCREENSAVER_SELECT_INPUT: u8 = 2i32 as u8;

/// The `ScreenSaver::SelectInput` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screensaver_select_input_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub event_mask: u32,
}

impl Default for xcb_screensaver_select_input_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `ScreenSaver::value_list` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screensaver_set_attributes_value_list_t {
    pub background_pixmap: xcb_pixmap_t,
    pub background_pixel: u32,
    pub border_pixmap: xcb_pixmap_t,
    pub border_pixel: u32,
    pub bit_gravity: u32,
    pub win_gravity: u32,
    pub backing_store: u32,
    pub backing_planes: u32,
    pub backing_pixel: u32,
    pub override_redirect: xcb_bool32_t,
    pub save_under: xcb_bool32_t,
    pub event_mask: u32,
    pub do_not_propogate_mask: u32,
    pub colormap: xcb_colormap_t,
    pub cursor: xcb_cursor_t,
}

impl Default for xcb_screensaver_set_attributes_value_list_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ScreenSaver::SetAttributes` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbScreensaver::xcb_screensaver_id()`], then the type of the request is
/// [`xcb_screensaver_set_attributes_request_t`].
pub const XCB_SCREENSAVER_SET_ATTRIBUTES: u8 = 3i32 as u8;

/// The `ScreenSaver::SetAttributes` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `value_list`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screensaver_set_attributes_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub class: u8,
    pub depth: u8,
    pub visual: xcb_visualid_t,
    pub value_mask: u32,
}

impl Default for xcb_screensaver_set_attributes_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ScreenSaver::UnsetAttributes` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbScreensaver::xcb_screensaver_id()`], then the type of the request is
/// [`xcb_screensaver_unset_attributes_request_t`].
pub const XCB_SCREENSAVER_UNSET_ATTRIBUTES: u8 = 4i32 as u8;

/// The `ScreenSaver::UnsetAttributes` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screensaver_unset_attributes_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
}

impl Default for xcb_screensaver_unset_attributes_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ScreenSaver::Suspend` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbScreensaver::xcb_screensaver_id()`], then the type of the request is
/// [`xcb_screensaver_suspend_request_t`].
pub const XCB_SCREENSAVER_SUSPEND: u8 = 5i32 as u8;

/// The `ScreenSaver::Suspend` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screensaver_suspend_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub suspend: u32,
}

impl Default for xcb_screensaver_suspend_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ScreenSaver::Notify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_screensaver_notify_event_t`].
pub const XCB_SCREENSAVER_NOTIFY: u8 = 0i32 as u8;

/// The `ScreenSaver::Notify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screensaver_notify_event_t {
    pub response_type: u8,
    pub state: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub window: xcb_window_t,
    pub kind: u8,
    pub forced: u8,
    pub pad0: [u8; 14],
}

impl Default for xcb_screensaver_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[cfg(feature = "xcb_screensaver")]
pub(crate) struct XcbScreensaverScreensaver {
    xcb_screensaver_id: LazySymbol<*mut xcb_extension_t>,
    xcb_screensaver_query_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u8,
            client_minor_version: u8,
        ) -> xcb_screensaver_query_version_cookie_t,
    >,
    xcb_screensaver_query_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u8,
            client_minor_version: u8,
        ) -> xcb_screensaver_query_version_cookie_t,
    >,
    xcb_screensaver_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_screensaver_query_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_screensaver_query_version_reply_t,
    >,
    xcb_screensaver_query_info: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
        ) -> xcb_screensaver_query_info_cookie_t,
    >,
    xcb_screensaver_query_info_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
        ) -> xcb_screensaver_query_info_cookie_t,
    >,
    xcb_screensaver_query_info_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_screensaver_query_info_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_screensaver_query_info_reply_t,
    >,
    xcb_screensaver_select_input_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            event_mask: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_screensaver_select_input: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            event_mask: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_screensaver_set_attributes_value_list_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            value_mask: u32,
            _aux: *const xcb_screensaver_set_attributes_value_list_t,
        ) -> c_int,
    >,
    xcb_screensaver_set_attributes_value_list_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            value_mask: u32,
            _aux: *mut xcb_screensaver_set_attributes_value_list_t,
        ) -> c_int,
    >,
    xcb_screensaver_set_attributes_value_list_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, value_mask: u32) -> c_int>,
    xcb_screensaver_set_attributes_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_screensaver_set_attributes_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            x: i16,
            y: i16,
            width: u16,
            height: u16,
            border_width: u16,
            class: u8,
            depth: u8,
            visual: xcb_visualid_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_screensaver_set_attributes: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            x: i16,
            y: i16,
            width: u16,
            height: u16,
            border_width: u16,
            class: u8,
            depth: u8,
            visual: xcb_visualid_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_screensaver_set_attributes_aux_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            x: i16,
            y: i16,
            width: u16,
            height: u16,
            border_width: u16,
            class: u8,
            depth: u8,
            visual: xcb_visualid_t,
            value_mask: u32,
            value_list: *const xcb_screensaver_set_attributes_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_screensaver_set_attributes_aux: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            x: i16,
            y: i16,
            width: u16,
            height: u16,
            border_width: u16,
            class: u8,
            depth: u8,
            visual: xcb_visualid_t,
            value_mask: u32,
            value_list: *const xcb_screensaver_set_attributes_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_screensaver_set_attributes_value_list:
        LazySymbol<unsafe fn(r: *const xcb_screensaver_set_attributes_request_t) -> *mut c_void>,
    xcb_screensaver_unset_attributes_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t) -> xcb_void_cookie_t,
    >,
    xcb_screensaver_unset_attributes: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t) -> xcb_void_cookie_t,
    >,
    xcb_screensaver_suspend_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, suspend: u32) -> xcb_void_cookie_t>,
    xcb_screensaver_suspend:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, suspend: u32) -> xcb_void_cookie_t>,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self
            .screensaver
            .$f
            .get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .screensaver
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

#[cfg(feature = "xcb_screensaver")]
impl XcbScreensaver {
    /// The libxcb identifier of the `ScreenSaver` extension.
    #[inline]
    pub fn xcb_screensaver_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_screensaver_id) }
    }

    /// Returns `true` iff the symbol `xcb_screensaver_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screensaver_id(&self) -> bool {
        has_sym!(self, xcb_screensaver_id)
    }

    /// Sends a `ScreenSaver::QueryVersion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_screensaver_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_screensaver_query_version_reply`]: Self::xcb_screensaver_query_version_reply
    #[inline]
    pub unsafe fn xcb_screensaver_query_version(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u8,
        client_minor_version: u8,
    ) -> xcb_screensaver_query_version_cookie_t {
        sym!(self, xcb_screensaver_query_version)(c, client_major_version, client_minor_version)
    }

    /// Returns `true` iff the symbol `xcb_screensaver_query_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screensaver_query_version(&self) -> bool {
        has_sym!(self, xcb_screensaver_query_version)
    }

    /// Sends a `ScreenSaver::QueryVersion` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_screensaver_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_screensaver_query_version_reply`]: Self::xcb_screensaver_query_version_reply
    #[inline]
    pub unsafe fn xcb_screensaver_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u8,
        client_minor_version: u8,
    ) -> xcb_screensaver_query_version_cookie_t {
        sym!(self, xcb_screensaver_query_version_unchecked)(
            c,
            client_major_version,
            client_minor_version,
        )
    }

    /// Returns `true` iff the symbol `xcb_screensaver_query_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screensaver_query_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_screensaver_query_version_unchecked)
    }

    /// Waits for the reply to a `ScreenSaver::QueryVersion` request.
    #[inline]
    pub unsafe fn xcb_screensaver_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_screensaver_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_screensaver_query_version_reply_t {
        sym!(self, xcb_screensaver_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_screensaver_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screensaver_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_screensaver_query_version_reply)
    }

    /// Sends a `ScreenSaver::QueryInfo` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_screensaver_query_info_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_screensaver_query_info_reply`]: Self::xcb_screensaver_query_info_reply
    #[inline]
    pub unsafe fn xcb_screensaver_query_info(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_screensaver_query_info_cookie_t {
        sym!(self, xcb_screensaver_query_info)(c, drawable)
    }

    /// Returns `true` iff the symbol `xcb_screensaver_query_info` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screensaver_query_info(&self) -> bool {
        has_sym!(self, xcb_screensaver_query_info)
    }

    /// Sends a `ScreenSaver::QueryInfo` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_screensaver_query_info_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_screensaver_query_info_reply`]: Self::xcb_screensaver_query_info_reply
    #[inline]
    pub unsafe fn xcb_screensaver_query_info_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_screensaver_query_info_cookie_t {
        sym!(self, xcb_screensaver_query_info_unchecked)(c, drawable)
    }

    /// Returns `true` iff the symbol `xcb_screensaver_query_info_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screensaver_query_info_unchecked(&self) -> bool {
        has_sym!(self, xcb_screensaver_query_info_unchecked)
    }

    /// Waits for the reply to a `ScreenSaver::QueryInfo` request.
    #[inline]
    pub unsafe fn xcb_screensaver_query_info_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_screensaver_query_info_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_screensaver_query_info_reply_t {
        sym!(self, xcb_screensaver_query_info_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_screensaver_query_info_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screensaver_query_info_reply(&self) -> bool {
        has_sym!(self, xcb_screensaver_query_info_reply)
    }

    /// Sends a `ScreenSaver::SelectInput` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_screensaver_select_input_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        event_mask: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_screensaver_select_input_checked)(c, drawable, event_mask)
    }

    /// Returns `true` iff the symbol `xcb_screensaver_select_input_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screensaver_select_input_checked(&self) -> bool {
        has_sym!(self, xcb_screensaver_select_input_checked)
    }

    /// Sends a `ScreenSaver::SelectInput` request (unchecked).
    #[inline]
    pub unsafe fn xcb_screensaver_select_input(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        event_mask: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_screensaver_select_input)(c, drawable, event_mask)
    }

    /// Returns `true` iff the symbol `xcb_screensaver_select_input` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screensaver_select_input(&self) -> bool {
        has_sym!(self, xcb_screensaver_select_input)
    }

    /// Serializes a `xcb_screensaver_set_attributes_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_screensaver_set_attributes_value_list_serialize(
        &self,
        _buffer: *mut *mut c_void,
        value_mask: u32,
        _aux: *const xcb_screensaver_set_attributes_value_list_t,
    ) -> c_int {
        sym!(self, xcb_screensaver_set_attributes_value_list_serialize)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_screensaver_set_attributes_value_list_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screensaver_set_attributes_value_list_serialize(&self) -> bool {
        has_sym!(self, xcb_screensaver_set_attributes_value_list_serialize)
    }

    /// Unpacks a `xcb_screensaver_set_attributes_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_screensaver_set_attributes_value_list_unpack(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
        _aux: *mut xcb_screensaver_set_attributes_value_list_t,
    ) -> c_int {
        sym!(self, xcb_screensaver_set_attributes_value_list_unpack)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_screensaver_set_attributes_value_list_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screensaver_set_attributes_value_list_unpack(&self) -> bool {
        has_sym!(self, xcb_screensaver_set_attributes_value_list_unpack)
    }

    /// Computes the size of a `xcb_screensaver_set_attributes_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_screensaver_set_attributes_value_list_sizeof(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
    ) -> c_int {
        sym!(self, xcb_screensaver_set_attributes_value_list_sizeof)(_buffer, value_mask)
    }

    /// Returns `true` iff the symbol `xcb_screensaver_set_attributes_value_list_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screensaver_set_attributes_value_list_sizeof(&self) -> bool {
        has_sym!(self, xcb_screensaver_set_attributes_value_list_sizeof)
    }

    /// Computes the size of a `xcb_screensaver_set_attributes_request_t` object.
    #[inline]
    pub unsafe fn xcb_screensaver_set_attributes_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_screensaver_set_attributes_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_screensaver_set_attributes_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screensaver_set_attributes_sizeof(&self) -> bool {
        has_sym!(self, xcb_screensaver_set_attributes_sizeof)
    }

    /// Sends a `ScreenSaver::SetAttributes` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    ///
    /// There is an auxiliary version of this function: [`xcb_screensaver_set_attributes_aux_checked`].
    ///
    /// [`xcb_screensaver_set_attributes_aux_checked`]: Self::xcb_screensaver_set_attributes_aux_checked
    #[inline]
    pub unsafe fn xcb_screensaver_set_attributes_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        class: u8,
        depth: u8,
        visual: xcb_visualid_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_screensaver_set_attributes_checked)(
            c,
            drawable,
            x,
            y,
            width,
            height,
            border_width,
            class,
            depth,
            visual,
            value_mask,
            value_list,
        )
    }

    /// Returns `true` iff the symbol `xcb_screensaver_set_attributes_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screensaver_set_attributes_checked(&self) -> bool {
        has_sym!(self, xcb_screensaver_set_attributes_checked)
    }

    /// Sends a `ScreenSaver::SetAttributes` request (unchecked).
    ///
    /// There is an auxiliary version of this function: [`xcb_screensaver_set_attributes_aux`].
    ///
    /// [`xcb_screensaver_set_attributes_aux`]: Self::xcb_screensaver_set_attributes_aux
    #[inline]
    pub unsafe fn xcb_screensaver_set_attributes(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        class: u8,
        depth: u8,
        visual: xcb_visualid_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_screensaver_set_attributes)(
            c,
            drawable,
            x,
            y,
            width,
            height,
            border_width,
            class,
            depth,
            visual,
            value_mask,
            value_list,
        )
    }

    /// Returns `true` iff the symbol `xcb_screensaver_set_attributes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screensaver_set_attributes(&self) -> bool {
        has_sym!(self, xcb_screensaver_set_attributes)
    }

    /// Sends a `ScreenSaver::SetAttributes` request (checked) (aux).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_screensaver_set_attributes_aux_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        class: u8,
        depth: u8,
        visual: xcb_visualid_t,
        value_mask: u32,
        value_list: *const xcb_screensaver_set_attributes_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_screensaver_set_attributes_aux_checked)(
            c,
            drawable,
            x,
            y,
            width,
            height,
            border_width,
            class,
            depth,
            visual,
            value_mask,
            value_list,
        )
    }

    /// Returns `true` iff the symbol `xcb_screensaver_set_attributes_aux_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screensaver_set_attributes_aux_checked(&self) -> bool {
        has_sym!(self, xcb_screensaver_set_attributes_aux_checked)
    }

    /// Sends a `ScreenSaver::SetAttributes` request (unchecked) (aux).
    #[inline]
    pub unsafe fn xcb_screensaver_set_attributes_aux(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        class: u8,
        depth: u8,
        visual: xcb_visualid_t,
        value_mask: u32,
        value_list: *const xcb_screensaver_set_attributes_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_screensaver_set_attributes_aux)(
            c,
            drawable,
            x,
            y,
            width,
            height,
            border_width,
            class,
            depth,
            visual,
            value_mask,
            value_list,
        )
    }

    /// Returns `true` iff the symbol `xcb_screensaver_set_attributes_aux` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screensaver_set_attributes_aux(&self) -> bool {
        has_sym!(self, xcb_screensaver_set_attributes_aux)
    }

    /// Returns a pointer to the `value_list` field of a `xcb_screensaver_set_attributes_request_t` struct.
    #[inline]
    pub unsafe fn xcb_screensaver_set_attributes_value_list(
        &self,
        r: *const xcb_screensaver_set_attributes_request_t,
    ) -> *mut c_void {
        sym!(self, xcb_screensaver_set_attributes_value_list)(r)
    }

    /// Returns `true` iff the symbol `xcb_screensaver_set_attributes_value_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screensaver_set_attributes_value_list(&self) -> bool {
        has_sym!(self, xcb_screensaver_set_attributes_value_list)
    }

    /// Sends a `ScreenSaver::UnsetAttributes` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_screensaver_unset_attributes_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_screensaver_unset_attributes_checked)(c, drawable)
    }

    /// Returns `true` iff the symbol `xcb_screensaver_unset_attributes_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screensaver_unset_attributes_checked(&self) -> bool {
        has_sym!(self, xcb_screensaver_unset_attributes_checked)
    }

    /// Sends a `ScreenSaver::UnsetAttributes` request (unchecked).
    #[inline]
    pub unsafe fn xcb_screensaver_unset_attributes(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_screensaver_unset_attributes)(c, drawable)
    }

    /// Returns `true` iff the symbol `xcb_screensaver_unset_attributes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screensaver_unset_attributes(&self) -> bool {
        has_sym!(self, xcb_screensaver_unset_attributes)
    }

    /// Sends a `ScreenSaver::Suspend` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_screensaver_suspend_checked(
        &self,
        c: *mut xcb_connection_t,
        suspend: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_screensaver_suspend_checked)(c, suspend)
    }

    /// Returns `true` iff the symbol `xcb_screensaver_suspend_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screensaver_suspend_checked(&self) -> bool {
        has_sym!(self, xcb_screensaver_suspend_checked)
    }

    /// Sends a `ScreenSaver::Suspend` request (unchecked).
    #[inline]
    pub unsafe fn xcb_screensaver_suspend(
        &self,
        c: *mut xcb_connection_t,
        suspend: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_screensaver_suspend)(c, suspend)
    }

    /// Returns `true` iff the symbol `xcb_screensaver_suspend` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screensaver_suspend(&self) -> bool {
        has_sym!(self, xcb_screensaver_suspend)
    }
}

#[cfg(feature = "xcb_screensaver")]
#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbScreensaver::load().unwrap() };
        assert!(lib.has_xcb_screensaver_id());
        assert!(lib.has_xcb_screensaver_query_version());
        assert!(lib.has_xcb_screensaver_query_version_unchecked());
        assert!(lib.has_xcb_screensaver_query_version_reply());
        assert!(lib.has_xcb_screensaver_query_info());
        assert!(lib.has_xcb_screensaver_query_info_unchecked());
        assert!(lib.has_xcb_screensaver_query_info_reply());
        assert!(lib.has_xcb_screensaver_select_input_checked());
        assert!(lib.has_xcb_screensaver_select_input());
        assert!(lib.has_xcb_screensaver_set_attributes_value_list_serialize());
        assert!(lib.has_xcb_screensaver_set_attributes_value_list_unpack());
        assert!(lib.has_xcb_screensaver_set_attributes_value_list_sizeof());
        assert!(lib.has_xcb_screensaver_set_attributes_sizeof());
        assert!(lib.has_xcb_screensaver_set_attributes_checked());
        assert!(lib.has_xcb_screensaver_set_attributes());
        assert!(lib.has_xcb_screensaver_set_attributes_aux_checked());
        assert!(lib.has_xcb_screensaver_set_attributes_aux());
        assert!(lib.has_xcb_screensaver_set_attributes_value_list());
        assert!(lib.has_xcb_screensaver_unset_attributes_checked());
        assert!(lib.has_xcb_screensaver_unset_attributes());
        assert!(lib.has_xcb_screensaver_suspend_checked());
        assert!(lib.has_xcb_screensaver_suspend());
    }
}
