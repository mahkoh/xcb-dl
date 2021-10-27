// This file was generated using generate.py. Do not edit.
#![allow(unused_macros)]

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

/// The name of the `Present` extension.
pub const XCB_PRESENT_NAME: &[u8] = b"Present";

/// The name of the `Present` extension.
pub const XCB_PRESENT_NAME_STR: &str = "Present";

/// The `Present::Event` enum.
///
/// This enum has the following variants:
///
/// - [`Present::Event::ConfigureNotify`](XCB_PRESENT_EVENT_CONFIGURE_NOTIFY)
/// - [`Present::Event::CompleteNotify`](XCB_PRESENT_EVENT_COMPLETE_NOTIFY)
/// - [`Present::Event::IdleNotify`](XCB_PRESENT_EVENT_IDLE_NOTIFY)
/// - [`Present::Event::RedirectNotify`](XCB_PRESENT_EVENT_REDIRECT_NOTIFY)
pub type xcb_present_event_enum_t = u32;
/// The `Present::Event::ConfigureNotify` enum variant.
///
/// This is a variant of [`xcb_present_event_enum_t`].
pub const XCB_PRESENT_EVENT_CONFIGURE_NOTIFY: xcb_present_event_enum_t = 0;
/// The `Present::Event::CompleteNotify` enum variant.
///
/// This is a variant of [`xcb_present_event_enum_t`].
pub const XCB_PRESENT_EVENT_COMPLETE_NOTIFY: xcb_present_event_enum_t = 1;
/// The `Present::Event::IdleNotify` enum variant.
///
/// This is a variant of [`xcb_present_event_enum_t`].
pub const XCB_PRESENT_EVENT_IDLE_NOTIFY: xcb_present_event_enum_t = 2;
/// The `Present::Event::RedirectNotify` enum variant.
///
/// This is a variant of [`xcb_present_event_enum_t`].
pub const XCB_PRESENT_EVENT_REDIRECT_NOTIFY: xcb_present_event_enum_t = 3;

/// The `Present::EventMask` enum.
///
/// This enum has the following variants:
///
/// - [`Present::EventMask::NoEvent`](XCB_PRESENT_EVENT_MASK_NO_EVENT)
/// - [`Present::EventMask::ConfigureNotify`](XCB_PRESENT_EVENT_MASK_CONFIGURE_NOTIFY)
/// - [`Present::EventMask::CompleteNotify`](XCB_PRESENT_EVENT_MASK_COMPLETE_NOTIFY)
/// - [`Present::EventMask::IdleNotify`](XCB_PRESENT_EVENT_MASK_IDLE_NOTIFY)
/// - [`Present::EventMask::RedirectNotify`](XCB_PRESENT_EVENT_MASK_REDIRECT_NOTIFY)
pub type xcb_present_event_mask_t = u32;
/// The `Present::EventMask::NoEvent` enum variant.
///
/// This is a variant of [`xcb_present_event_mask_t`].
pub const XCB_PRESENT_EVENT_MASK_NO_EVENT: xcb_present_event_mask_t = 0;
/// The `Present::EventMask::ConfigureNotify` enum variant.
///
/// This is a variant of [`xcb_present_event_mask_t`].
pub const XCB_PRESENT_EVENT_MASK_CONFIGURE_NOTIFY: xcb_present_event_mask_t = 1;
/// The `Present::EventMask::CompleteNotify` enum variant.
///
/// This is a variant of [`xcb_present_event_mask_t`].
pub const XCB_PRESENT_EVENT_MASK_COMPLETE_NOTIFY: xcb_present_event_mask_t = 2;
/// The `Present::EventMask::IdleNotify` enum variant.
///
/// This is a variant of [`xcb_present_event_mask_t`].
pub const XCB_PRESENT_EVENT_MASK_IDLE_NOTIFY: xcb_present_event_mask_t = 4;
/// The `Present::EventMask::RedirectNotify` enum variant.
///
/// This is a variant of [`xcb_present_event_mask_t`].
pub const XCB_PRESENT_EVENT_MASK_REDIRECT_NOTIFY: xcb_present_event_mask_t = 8;

/// The `Present::Option` enum.
///
/// This enum has the following variants:
///
/// - [`Present::Option::None`](XCB_PRESENT_OPTION_NONE)
/// - [`Present::Option::Async`](XCB_PRESENT_OPTION_ASYNC)
/// - [`Present::Option::Copy`](XCB_PRESENT_OPTION_COPY)
/// - [`Present::Option::UST`](XCB_PRESENT_OPTION_UST)
/// - [`Present::Option::Suboptimal`](XCB_PRESENT_OPTION_SUBOPTIMAL)
pub type xcb_present_option_t = u32;
/// The `Present::Option::None` enum variant.
///
/// This is a variant of [`xcb_present_option_t`].
pub const XCB_PRESENT_OPTION_NONE: xcb_present_option_t = 0;
/// The `Present::Option::Async` enum variant.
///
/// This is a variant of [`xcb_present_option_t`].
pub const XCB_PRESENT_OPTION_ASYNC: xcb_present_option_t = 1;
/// The `Present::Option::Copy` enum variant.
///
/// This is a variant of [`xcb_present_option_t`].
pub const XCB_PRESENT_OPTION_COPY: xcb_present_option_t = 2;
/// The `Present::Option::UST` enum variant.
///
/// This is a variant of [`xcb_present_option_t`].
pub const XCB_PRESENT_OPTION_UST: xcb_present_option_t = 4;
/// The `Present::Option::Suboptimal` enum variant.
///
/// This is a variant of [`xcb_present_option_t`].
pub const XCB_PRESENT_OPTION_SUBOPTIMAL: xcb_present_option_t = 8;

/// The `Present::Capability` enum.
///
/// This enum has the following variants:
///
/// - [`Present::Capability::None`](XCB_PRESENT_CAPABILITY_NONE)
/// - [`Present::Capability::Async`](XCB_PRESENT_CAPABILITY_ASYNC)
/// - [`Present::Capability::Fence`](XCB_PRESENT_CAPABILITY_FENCE)
/// - [`Present::Capability::UST`](XCB_PRESENT_CAPABILITY_UST)
pub type xcb_present_capability_t = u32;
/// The `Present::Capability::None` enum variant.
///
/// This is a variant of [`xcb_present_capability_t`].
pub const XCB_PRESENT_CAPABILITY_NONE: xcb_present_capability_t = 0;
/// The `Present::Capability::Async` enum variant.
///
/// This is a variant of [`xcb_present_capability_t`].
pub const XCB_PRESENT_CAPABILITY_ASYNC: xcb_present_capability_t = 1;
/// The `Present::Capability::Fence` enum variant.
///
/// This is a variant of [`xcb_present_capability_t`].
pub const XCB_PRESENT_CAPABILITY_FENCE: xcb_present_capability_t = 2;
/// The `Present::Capability::UST` enum variant.
///
/// This is a variant of [`xcb_present_capability_t`].
pub const XCB_PRESENT_CAPABILITY_UST: xcb_present_capability_t = 4;

/// The `Present::CompleteKind` enum.
///
/// This enum has the following variants:
///
/// - [`Present::CompleteKind::Pixmap`](XCB_PRESENT_COMPLETE_KIND_PIXMAP)
/// - [`Present::CompleteKind::NotifyMSC`](XCB_PRESENT_COMPLETE_KIND_NOTIFY_MSC)
pub type xcb_present_complete_kind_t = u32;
/// The `Present::CompleteKind::Pixmap` enum variant.
///
/// This is a variant of [`xcb_present_complete_kind_t`].
pub const XCB_PRESENT_COMPLETE_KIND_PIXMAP: xcb_present_complete_kind_t = 0;
/// The `Present::CompleteKind::NotifyMSC` enum variant.
///
/// This is a variant of [`xcb_present_complete_kind_t`].
pub const XCB_PRESENT_COMPLETE_KIND_NOTIFY_MSC: xcb_present_complete_kind_t = 1;

/// The `Present::CompleteMode` enum.
///
/// This enum has the following variants:
///
/// - [`Present::CompleteMode::Copy`](XCB_PRESENT_COMPLETE_MODE_COPY)
/// - [`Present::CompleteMode::Flip`](XCB_PRESENT_COMPLETE_MODE_FLIP)
/// - [`Present::CompleteMode::Skip`](XCB_PRESENT_COMPLETE_MODE_SKIP)
/// - [`Present::CompleteMode::SuboptimalCopy`](XCB_PRESENT_COMPLETE_MODE_SUBOPTIMAL_COPY)
pub type xcb_present_complete_mode_t = u32;
/// The `Present::CompleteMode::Copy` enum variant.
///
/// This is a variant of [`xcb_present_complete_mode_t`].
pub const XCB_PRESENT_COMPLETE_MODE_COPY: xcb_present_complete_mode_t = 0;
/// The `Present::CompleteMode::Flip` enum variant.
///
/// This is a variant of [`xcb_present_complete_mode_t`].
pub const XCB_PRESENT_COMPLETE_MODE_FLIP: xcb_present_complete_mode_t = 1;
/// The `Present::CompleteMode::Skip` enum variant.
///
/// This is a variant of [`xcb_present_complete_mode_t`].
pub const XCB_PRESENT_COMPLETE_MODE_SKIP: xcb_present_complete_mode_t = 2;
/// The `Present::CompleteMode::SuboptimalCopy` enum variant.
///
/// This is a variant of [`xcb_present_complete_mode_t`].
pub const XCB_PRESENT_COMPLETE_MODE_SUBOPTIMAL_COPY: xcb_present_complete_mode_t = 3;

/// The `Present::Notify` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_notify_t {
    pub window: xcb_window_t,
    pub serial: u32,
}

impl Default for xcb_present_notify_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Present::Notify` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_notify_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_present_notify_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_present_notify_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Present::QueryVersion` request.
///
/// Pass this cookie to [`xcb_present_query_version_reply`] to retrieve the reply.
///
/// [`xcb_present_query_version_reply`]: XcbPresent::xcb_present_query_version_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_query_version_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_present_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Present::QueryVersion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbPresent::xcb_present_id()`], then the type of the request is
/// [`xcb_present_query_version_request_t`].
pub const XCB_PRESENT_QUERY_VERSION: u8 = 0i32 as u8;

/// The `Present::QueryVersion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub major_version: u32,
    pub minor_version: u32,
}

impl Default for xcb_present_query_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Present::QueryVersion` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u32,
    pub minor_version: u32,
}

impl Default for xcb_present_query_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Present::Pixmap` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbPresent::xcb_present_id()`], then the type of the request is
/// [`xcb_present_pixmap_request_t`].
pub const XCB_PRESENT_PIXMAP: u8 = 1i32 as u8;

/// The `Present::Pixmap` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `notifies`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_pixmap_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub pixmap: xcb_pixmap_t,
    pub serial: u32,
    pub valid: xcb_xfixes_region_t,
    pub update: xcb_xfixes_region_t,
    pub x_off: i16,
    pub y_off: i16,
    pub target_crtc: xcb_randr_crtc_t,
    pub wait_fence: xcb_sync_fence_t,
    pub idle_fence: xcb_sync_fence_t,
    pub options: u32,
    pub pad0: [u8; 4],
    pub target_msc: u64,
    pub divisor: u64,
    pub remainder: u64,
}

impl Default for xcb_present_pixmap_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Present::NotifyMSC` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbPresent::xcb_present_id()`], then the type of the request is
/// [`xcb_present_notify_msc_request_t`].
pub const XCB_PRESENT_NOTIFY_MSC: u8 = 2i32 as u8;

/// The `Present::NotifyMSC` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_notify_msc_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub serial: u32,
    pub pad0: [u8; 4],
    pub target_msc: u64,
    pub divisor: u64,
    pub remainder: u64,
}

impl Default for xcb_present_notify_msc_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Present::EVENT` type.
pub type xcb_present_event_t = u32;

/// An iterator over `Present::EVENT` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_event_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_present_event_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_present_event_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Present::SelectInput` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbPresent::xcb_present_id()`], then the type of the request is
/// [`xcb_present_select_input_request_t`].
pub const XCB_PRESENT_SELECT_INPUT: u8 = 3i32 as u8;

/// The `Present::SelectInput` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_select_input_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub eid: xcb_present_event_t,
    pub window: xcb_window_t,
    pub event_mask: u32,
}

impl Default for xcb_present_select_input_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Present::QueryCapabilities` request.
///
/// Pass this cookie to [`xcb_present_query_capabilities_reply`] to retrieve the reply.
///
/// [`xcb_present_query_capabilities_reply`]: XcbPresent::xcb_present_query_capabilities_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_query_capabilities_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_present_query_capabilities_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Present::QueryCapabilities` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbPresent::xcb_present_id()`], then the type of the request is
/// [`xcb_present_query_capabilities_request_t`].
pub const XCB_PRESENT_QUERY_CAPABILITIES: u8 = 4i32 as u8;

/// The `Present::QueryCapabilities` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_query_capabilities_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub target: u32,
}

impl Default for xcb_present_query_capabilities_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Present::QueryCapabilities` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_query_capabilities_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub capabilities: u32,
}

impl Default for xcb_present_query_capabilities_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Present::Generic` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_present_generic_event_t`].
pub const XCB_PRESENT_GENERIC: u8 = 0i32 as u8;

/// The `Present::Generic` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_generic_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub evtype: u16,
    pub pad0: [u8; 2],
    pub event: xcb_present_event_t,
}

impl Default for xcb_present_generic_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Present::ConfigureNotify` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Present` extension,
/// then the type of the event is [`xcb_present_configure_notify_event_t`].
pub const XCB_PRESENT_CONFIGURE_NOTIFY: u16 = 0i32 as u16;

/// The `Present::ConfigureNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_configure_notify_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub pad0: [u8; 2],
    pub event: xcb_present_event_t,
    pub window: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub off_x: i16,
    pub off_y: i16,
    pub full_sequence: u32,
    pub pixmap_width: u16,
    pub pixmap_height: u16,
    pub pixmap_flags: u32,
}

impl Default for xcb_present_configure_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Present::CompleteNotify` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Present` extension,
/// then the type of the event is [`xcb_present_complete_notify_event_t`].
pub const XCB_PRESENT_COMPLETE_NOTIFY: u16 = 1i32 as u16;

/// The `Present::CompleteNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct xcb_present_complete_notify_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub kind: u8,
    pub mode: u8,
    pub event: xcb_present_event_t,
    pub window: xcb_window_t,
    pub serial: u32,
    pub ust: u64,
    pub full_sequence: u32,
    pub msc: u64,
}

impl Default for xcb_present_complete_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Present::IdleNotify` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Present` extension,
/// then the type of the event is [`xcb_present_idle_notify_event_t`].
pub const XCB_PRESENT_IDLE_NOTIFY: u16 = 2i32 as u16;

/// The `Present::IdleNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_idle_notify_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub pad0: [u8; 2],
    pub event: xcb_present_event_t,
    pub window: xcb_window_t,
    pub serial: u32,
    pub pixmap: xcb_pixmap_t,
    pub idle_fence: xcb_sync_fence_t,
    pub full_sequence: u32,
}

impl Default for xcb_present_idle_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Present::RedirectNotify` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Present` extension,
/// then the type of the event is [`xcb_present_redirect_notify_event_t`].
pub const XCB_PRESENT_REDIRECT_NOTIFY: u16 = 3i32 as u16;

/// The `Present::RedirectNotify` event.
///
/// The following fields can be accessed via accessor functions:
///
/// - `notifies`
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct xcb_present_redirect_notify_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub update_window: u8,
    pub pad0: u8,
    pub event: xcb_present_event_t,
    pub event_window: xcb_window_t,
    pub window: xcb_window_t,
    pub pixmap: xcb_pixmap_t,
    pub serial: u32,
    pub full_sequence: u32,
    pub valid_region: xcb_xfixes_region_t,
    pub update_region: xcb_xfixes_region_t,
    pub valid_rect: xcb_rectangle_t,
    pub update_rect: xcb_rectangle_t,
    pub x_off: i16,
    pub y_off: i16,
    pub target_crtc: xcb_randr_crtc_t,
    pub wait_fence: xcb_sync_fence_t,
    pub idle_fence: xcb_sync_fence_t,
    pub options: u32,
    pub pad1: [u8; 4],
    pub target_msc: u64,
    pub divisor: u64,
    pub remainder: u64,
}

impl Default for xcb_present_redirect_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[cfg(feature = "xcb_present")]
pub(crate) struct XcbPresentPresent {
    xcb_present_id: LazySymbol<*mut xcb_extension_t>,
    xcb_present_notify_next: LazySymbol<unsafe fn(i: *mut xcb_present_notify_iterator_t)>,
    xcb_present_notify_end:
        LazySymbol<unsafe fn(i: xcb_present_notify_iterator_t) -> xcb_generic_iterator_t>,
    xcb_present_query_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major_version: u32,
            minor_version: u32,
        ) -> xcb_present_query_version_cookie_t,
    >,
    xcb_present_query_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major_version: u32,
            minor_version: u32,
        ) -> xcb_present_query_version_cookie_t,
    >,
    xcb_present_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_present_query_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_present_query_version_reply_t,
    >,
    xcb_present_pixmap_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, notifies_len: u32) -> c_int>,
    xcb_present_pixmap_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            pixmap: xcb_pixmap_t,
            serial: u32,
            valid: xcb_xfixes_region_t,
            update: xcb_xfixes_region_t,
            x_off: i16,
            y_off: i16,
            target_crtc: xcb_randr_crtc_t,
            wait_fence: xcb_sync_fence_t,
            idle_fence: xcb_sync_fence_t,
            options: u32,
            target_msc: u64,
            divisor: u64,
            remainder: u64,
            notifies_len: u32,
            notifies: *const xcb_present_notify_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_present_pixmap: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            pixmap: xcb_pixmap_t,
            serial: u32,
            valid: xcb_xfixes_region_t,
            update: xcb_xfixes_region_t,
            x_off: i16,
            y_off: i16,
            target_crtc: xcb_randr_crtc_t,
            wait_fence: xcb_sync_fence_t,
            idle_fence: xcb_sync_fence_t,
            options: u32,
            target_msc: u64,
            divisor: u64,
            remainder: u64,
            notifies_len: u32,
            notifies: *const xcb_present_notify_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_present_pixmap_notifies:
        LazySymbol<unsafe fn(r: *const xcb_present_pixmap_request_t) -> *mut xcb_present_notify_t>,
    xcb_present_pixmap_notifies_length:
        LazySymbol<unsafe fn(r: *const xcb_present_pixmap_request_t) -> c_int>,
    xcb_present_pixmap_notifies_iterator: LazySymbol<
        unsafe fn(r: *const xcb_present_pixmap_request_t) -> xcb_present_notify_iterator_t,
    >,
    xcb_present_notify_msc_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            serial: u32,
            target_msc: u64,
            divisor: u64,
            remainder: u64,
        ) -> xcb_void_cookie_t,
    >,
    xcb_present_notify_msc: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            serial: u32,
            target_msc: u64,
            divisor: u64,
            remainder: u64,
        ) -> xcb_void_cookie_t,
    >,
    xcb_present_event_next: LazySymbol<unsafe fn(i: *mut xcb_present_event_iterator_t)>,
    xcb_present_event_end:
        LazySymbol<unsafe fn(i: xcb_present_event_iterator_t) -> xcb_generic_iterator_t>,
    xcb_present_select_input_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            eid: xcb_present_event_t,
            window: xcb_window_t,
            event_mask: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_present_select_input: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            eid: xcb_present_event_t,
            window: xcb_window_t,
            event_mask: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_present_query_capabilities: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, target: u32) -> xcb_present_query_capabilities_cookie_t,
    >,
    xcb_present_query_capabilities_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, target: u32) -> xcb_present_query_capabilities_cookie_t,
    >,
    xcb_present_query_capabilities_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_present_query_capabilities_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_present_query_capabilities_reply_t,
    >,
    xcb_present_redirect_notify_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, notifies_len: u32) -> c_int>,
    xcb_present_redirect_notify_notifies: LazySymbol<
        unsafe fn(r: *const xcb_present_redirect_notify_event_t) -> *mut xcb_present_notify_t,
    >,
    xcb_present_redirect_notify_notifies_length:
        LazySymbol<unsafe fn(r: *const xcb_present_redirect_notify_event_t) -> c_int>,
    xcb_present_redirect_notify_notifies_iterator: LazySymbol<
        unsafe fn(r: *const xcb_present_redirect_notify_event_t) -> xcb_present_notify_iterator_t,
    >,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self
            .present
            .$f
            .get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .present
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

#[cfg(feature = "xcb_present")]
impl XcbPresent {
    /// The libxcb identifier of the `Present` extension.
    #[inline]
    pub fn xcb_present_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_present_id) }
    }

    /// Returns `true` iff the symbol `xcb_present_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_id(&self) -> bool {
        has_sym!(self, xcb_present_id)
    }

    /// Advances a `xcb_present_notify_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_present_notify_next(&self, i: *mut xcb_present_notify_iterator_t) {
        sym!(self, xcb_present_notify_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_present_notify_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_notify_next(&self) -> bool {
        has_sym!(self, xcb_present_notify_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_present_notify_iterator_t`.
    #[inline]
    pub unsafe fn xcb_present_notify_end(
        &self,
        i: xcb_present_notify_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_present_notify_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_present_notify_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_notify_end(&self) -> bool {
        has_sym!(self, xcb_present_notify_end)
    }

    /// Sends a `Present::QueryVersion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_present_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_present_query_version_reply`]: Self::xcb_present_query_version_reply
    #[inline]
    pub unsafe fn xcb_present_query_version(
        &self,
        c: *mut xcb_connection_t,
        major_version: u32,
        minor_version: u32,
    ) -> xcb_present_query_version_cookie_t {
        sym!(self, xcb_present_query_version)(c, major_version, minor_version)
    }

    /// Returns `true` iff the symbol `xcb_present_query_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_query_version(&self) -> bool {
        has_sym!(self, xcb_present_query_version)
    }

    /// Sends a `Present::QueryVersion` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_present_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_present_query_version_reply`]: Self::xcb_present_query_version_reply
    #[inline]
    pub unsafe fn xcb_present_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        major_version: u32,
        minor_version: u32,
    ) -> xcb_present_query_version_cookie_t {
        sym!(self, xcb_present_query_version_unchecked)(c, major_version, minor_version)
    }

    /// Returns `true` iff the symbol `xcb_present_query_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_query_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_present_query_version_unchecked)
    }

    /// Waits for the reply to a `Present::QueryVersion` request.
    #[inline]
    pub unsafe fn xcb_present_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_present_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_present_query_version_reply_t {
        sym!(self, xcb_present_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_present_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_present_query_version_reply)
    }

    /// Computes the size of a `xcb_present_pixmap_request_t` object.
    #[inline]
    pub unsafe fn xcb_present_pixmap_sizeof(
        &self,
        _buffer: *const c_void,
        notifies_len: u32,
    ) -> c_int {
        sym!(self, xcb_present_pixmap_sizeof)(_buffer, notifies_len)
    }

    /// Returns `true` iff the symbol `xcb_present_pixmap_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_pixmap_sizeof(&self) -> bool {
        has_sym!(self, xcb_present_pixmap_sizeof)
    }

    /// Sends a `Present::Pixmap` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_present_pixmap_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        pixmap: xcb_pixmap_t,
        serial: u32,
        valid: xcb_xfixes_region_t,
        update: xcb_xfixes_region_t,
        x_off: i16,
        y_off: i16,
        target_crtc: xcb_randr_crtc_t,
        wait_fence: xcb_sync_fence_t,
        idle_fence: xcb_sync_fence_t,
        options: u32,
        target_msc: u64,
        divisor: u64,
        remainder: u64,
        notifies_len: u32,
        notifies: *const xcb_present_notify_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_present_pixmap_checked)(
            c,
            window,
            pixmap,
            serial,
            valid,
            update,
            x_off,
            y_off,
            target_crtc,
            wait_fence,
            idle_fence,
            options,
            target_msc,
            divisor,
            remainder,
            notifies_len,
            notifies,
        )
    }

    /// Returns `true` iff the symbol `xcb_present_pixmap_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_pixmap_checked(&self) -> bool {
        has_sym!(self, xcb_present_pixmap_checked)
    }

    /// Sends a `Present::Pixmap` request (unchecked).
    #[inline]
    pub unsafe fn xcb_present_pixmap(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        pixmap: xcb_pixmap_t,
        serial: u32,
        valid: xcb_xfixes_region_t,
        update: xcb_xfixes_region_t,
        x_off: i16,
        y_off: i16,
        target_crtc: xcb_randr_crtc_t,
        wait_fence: xcb_sync_fence_t,
        idle_fence: xcb_sync_fence_t,
        options: u32,
        target_msc: u64,
        divisor: u64,
        remainder: u64,
        notifies_len: u32,
        notifies: *const xcb_present_notify_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_present_pixmap)(
            c,
            window,
            pixmap,
            serial,
            valid,
            update,
            x_off,
            y_off,
            target_crtc,
            wait_fence,
            idle_fence,
            options,
            target_msc,
            divisor,
            remainder,
            notifies_len,
            notifies,
        )
    }

    /// Returns `true` iff the symbol `xcb_present_pixmap` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_pixmap(&self) -> bool {
        has_sym!(self, xcb_present_pixmap)
    }

    /// Returns a pointer to the `notifies` field of a `xcb_present_pixmap_request_t` struct.
    #[inline]
    pub unsafe fn xcb_present_pixmap_notifies(
        &self,
        r: *const xcb_present_pixmap_request_t,
    ) -> *mut xcb_present_notify_t {
        sym!(self, xcb_present_pixmap_notifies)(r)
    }

    /// Returns `true` iff the symbol `xcb_present_pixmap_notifies` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_pixmap_notifies(&self) -> bool {
        has_sym!(self, xcb_present_pixmap_notifies)
    }

    /// Returns the number of elements of the `notifies` field of a `xcb_present_pixmap_request_t` struct.
    #[inline]
    pub unsafe fn xcb_present_pixmap_notifies_length(
        &self,
        r: *const xcb_present_pixmap_request_t,
    ) -> c_int {
        sym!(self, xcb_present_pixmap_notifies_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_present_pixmap_notifies_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_pixmap_notifies_length(&self) -> bool {
        has_sym!(self, xcb_present_pixmap_notifies_length)
    }

    /// Returns an iterator over the elements of the
    /// `notifies` field of a `xcb_present_pixmap_request_t` struct.
    #[inline]
    pub unsafe fn xcb_present_pixmap_notifies_iterator(
        &self,
        r: *const xcb_present_pixmap_request_t,
    ) -> xcb_present_notify_iterator_t {
        sym!(self, xcb_present_pixmap_notifies_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_present_pixmap_notifies_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_pixmap_notifies_iterator(&self) -> bool {
        has_sym!(self, xcb_present_pixmap_notifies_iterator)
    }

    /// Sends a `Present::NotifyMSC` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_present_notify_msc_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        serial: u32,
        target_msc: u64,
        divisor: u64,
        remainder: u64,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_present_notify_msc_checked)(
            c, window, serial, target_msc, divisor, remainder,
        )
    }

    /// Returns `true` iff the symbol `xcb_present_notify_msc_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_notify_msc_checked(&self) -> bool {
        has_sym!(self, xcb_present_notify_msc_checked)
    }

    /// Sends a `Present::NotifyMSC` request (unchecked).
    #[inline]
    pub unsafe fn xcb_present_notify_msc(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        serial: u32,
        target_msc: u64,
        divisor: u64,
        remainder: u64,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_present_notify_msc)(c, window, serial, target_msc, divisor, remainder)
    }

    /// Returns `true` iff the symbol `xcb_present_notify_msc` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_notify_msc(&self) -> bool {
        has_sym!(self, xcb_present_notify_msc)
    }

    /// Advances a `xcb_present_event_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_present_event_next(&self, i: *mut xcb_present_event_iterator_t) {
        sym!(self, xcb_present_event_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_present_event_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_event_next(&self) -> bool {
        has_sym!(self, xcb_present_event_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_present_event_iterator_t`.
    #[inline]
    pub unsafe fn xcb_present_event_end(
        &self,
        i: xcb_present_event_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_present_event_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_present_event_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_event_end(&self) -> bool {
        has_sym!(self, xcb_present_event_end)
    }

    /// Sends a `Present::SelectInput` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_present_select_input_checked(
        &self,
        c: *mut xcb_connection_t,
        eid: xcb_present_event_t,
        window: xcb_window_t,
        event_mask: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_present_select_input_checked)(c, eid, window, event_mask)
    }

    /// Returns `true` iff the symbol `xcb_present_select_input_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_select_input_checked(&self) -> bool {
        has_sym!(self, xcb_present_select_input_checked)
    }

    /// Sends a `Present::SelectInput` request (unchecked).
    #[inline]
    pub unsafe fn xcb_present_select_input(
        &self,
        c: *mut xcb_connection_t,
        eid: xcb_present_event_t,
        window: xcb_window_t,
        event_mask: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_present_select_input)(c, eid, window, event_mask)
    }

    /// Returns `true` iff the symbol `xcb_present_select_input` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_select_input(&self) -> bool {
        has_sym!(self, xcb_present_select_input)
    }

    /// Sends a `Present::QueryCapabilities` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_present_query_capabilities_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_present_query_capabilities_reply`]: Self::xcb_present_query_capabilities_reply
    #[inline]
    pub unsafe fn xcb_present_query_capabilities(
        &self,
        c: *mut xcb_connection_t,
        target: u32,
    ) -> xcb_present_query_capabilities_cookie_t {
        sym!(self, xcb_present_query_capabilities)(c, target)
    }

    /// Returns `true` iff the symbol `xcb_present_query_capabilities` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_query_capabilities(&self) -> bool {
        has_sym!(self, xcb_present_query_capabilities)
    }

    /// Sends a `Present::QueryCapabilities` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_present_query_capabilities_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_present_query_capabilities_reply`]: Self::xcb_present_query_capabilities_reply
    #[inline]
    pub unsafe fn xcb_present_query_capabilities_unchecked(
        &self,
        c: *mut xcb_connection_t,
        target: u32,
    ) -> xcb_present_query_capabilities_cookie_t {
        sym!(self, xcb_present_query_capabilities_unchecked)(c, target)
    }

    /// Returns `true` iff the symbol `xcb_present_query_capabilities_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_query_capabilities_unchecked(&self) -> bool {
        has_sym!(self, xcb_present_query_capabilities_unchecked)
    }

    /// Waits for the reply to a `Present::QueryCapabilities` request.
    #[inline]
    pub unsafe fn xcb_present_query_capabilities_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_present_query_capabilities_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_present_query_capabilities_reply_t {
        sym!(self, xcb_present_query_capabilities_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_present_query_capabilities_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_query_capabilities_reply(&self) -> bool {
        has_sym!(self, xcb_present_query_capabilities_reply)
    }

    /// Computes the size of a `xcb_present_redirect_notify_event_t` object.
    #[inline]
    pub unsafe fn xcb_present_redirect_notify_sizeof(
        &self,
        _buffer: *const c_void,
        notifies_len: u32,
    ) -> c_int {
        sym!(self, xcb_present_redirect_notify_sizeof)(_buffer, notifies_len)
    }

    /// Returns `true` iff the symbol `xcb_present_redirect_notify_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_redirect_notify_sizeof(&self) -> bool {
        has_sym!(self, xcb_present_redirect_notify_sizeof)
    }

    /// Returns a pointer to the `notifies` field of a `xcb_present_redirect_notify_event_t` struct.
    #[inline]
    pub unsafe fn xcb_present_redirect_notify_notifies(
        &self,
        r: *const xcb_present_redirect_notify_event_t,
    ) -> *mut xcb_present_notify_t {
        sym!(self, xcb_present_redirect_notify_notifies)(r)
    }

    /// Returns `true` iff the symbol `xcb_present_redirect_notify_notifies` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_redirect_notify_notifies(&self) -> bool {
        has_sym!(self, xcb_present_redirect_notify_notifies)
    }

    /// Returns the number of elements of the `notifies` field of a `xcb_present_redirect_notify_event_t` struct.
    #[inline]
    pub unsafe fn xcb_present_redirect_notify_notifies_length(
        &self,
        r: *const xcb_present_redirect_notify_event_t,
    ) -> c_int {
        sym!(self, xcb_present_redirect_notify_notifies_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_present_redirect_notify_notifies_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_redirect_notify_notifies_length(&self) -> bool {
        has_sym!(self, xcb_present_redirect_notify_notifies_length)
    }

    /// Returns an iterator over the elements of the
    /// `notifies` field of a `xcb_present_redirect_notify_event_t` struct.
    #[inline]
    pub unsafe fn xcb_present_redirect_notify_notifies_iterator(
        &self,
        r: *const xcb_present_redirect_notify_event_t,
    ) -> xcb_present_notify_iterator_t {
        sym!(self, xcb_present_redirect_notify_notifies_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_present_redirect_notify_notifies_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_redirect_notify_notifies_iterator(&self) -> bool {
        has_sym!(self, xcb_present_redirect_notify_notifies_iterator)
    }
}

#[cfg(feature = "xcb_present")]
#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbPresent::load().unwrap() };
        assert!(lib.has_xcb_present_id());
        assert!(lib.has_xcb_present_notify_next());
        assert!(lib.has_xcb_present_notify_end());
        assert!(lib.has_xcb_present_query_version());
        assert!(lib.has_xcb_present_query_version_unchecked());
        assert!(lib.has_xcb_present_query_version_reply());
        assert!(lib.has_xcb_present_pixmap_sizeof());
        assert!(lib.has_xcb_present_pixmap_checked());
        assert!(lib.has_xcb_present_pixmap());
        assert!(lib.has_xcb_present_pixmap_notifies());
        assert!(lib.has_xcb_present_pixmap_notifies_length());
        assert!(lib.has_xcb_present_pixmap_notifies_iterator());
        assert!(lib.has_xcb_present_notify_msc_checked());
        assert!(lib.has_xcb_present_notify_msc());
        assert!(lib.has_xcb_present_event_next());
        assert!(lib.has_xcb_present_event_end());
        assert!(lib.has_xcb_present_select_input_checked());
        assert!(lib.has_xcb_present_select_input());
        assert!(lib.has_xcb_present_query_capabilities());
        assert!(lib.has_xcb_present_query_capabilities_unchecked());
        assert!(lib.has_xcb_present_query_capabilities_reply());
        assert!(lib.has_xcb_present_redirect_notify_sizeof());
        assert!(lib.has_xcb_present_redirect_notify_notifies());
        assert!(lib.has_xcb_present_redirect_notify_notifies_length());
        assert!(lib.has_xcb_present_redirect_notify_notifies_iterator());
    }
}
