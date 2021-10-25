// This file was generated using generate.py. Do not edit.

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

pub type xcb_present_event_enum_t = u32;
pub const XCB_PRESENT_EVENT_CONFIGURE_NOTIFY: xcb_present_event_enum_t = 0;
pub const XCB_PRESENT_EVENT_COMPLETE_NOTIFY: xcb_present_event_enum_t = 1;
pub const XCB_PRESENT_EVENT_IDLE_NOTIFY: xcb_present_event_enum_t = 2;
pub const XCB_PRESENT_EVENT_REDIRECT_NOTIFY: xcb_present_event_enum_t = 3;

pub type xcb_present_event_mask_t = u32;
pub const XCB_PRESENT_EVENT_MASK_NO_EVENT: xcb_present_event_mask_t = 0;
pub const XCB_PRESENT_EVENT_MASK_CONFIGURE_NOTIFY: xcb_present_event_mask_t = 1;
pub const XCB_PRESENT_EVENT_MASK_COMPLETE_NOTIFY: xcb_present_event_mask_t = 2;
pub const XCB_PRESENT_EVENT_MASK_IDLE_NOTIFY: xcb_present_event_mask_t = 4;
pub const XCB_PRESENT_EVENT_MASK_REDIRECT_NOTIFY: xcb_present_event_mask_t = 8;

pub type xcb_present_option_t = u32;
pub const XCB_PRESENT_OPTION_NONE: xcb_present_option_t = 0;
pub const XCB_PRESENT_OPTION_ASYNC: xcb_present_option_t = 1;
pub const XCB_PRESENT_OPTION_COPY: xcb_present_option_t = 2;
pub const XCB_PRESENT_OPTION_UST: xcb_present_option_t = 4;
pub const XCB_PRESENT_OPTION_SUBOPTIMAL: xcb_present_option_t = 8;

pub type xcb_present_capability_t = u32;
pub const XCB_PRESENT_CAPABILITY_NONE: xcb_present_capability_t = 0;
pub const XCB_PRESENT_CAPABILITY_ASYNC: xcb_present_capability_t = 1;
pub const XCB_PRESENT_CAPABILITY_FENCE: xcb_present_capability_t = 2;
pub const XCB_PRESENT_CAPABILITY_UST: xcb_present_capability_t = 4;

pub type xcb_present_complete_kind_t = u32;
pub const XCB_PRESENT_COMPLETE_KIND_PIXMAP: xcb_present_complete_kind_t = 0;
pub const XCB_PRESENT_COMPLETE_KIND_NOTIFY_MSC: xcb_present_complete_kind_t = 1;

pub type xcb_present_complete_mode_t = u32;
pub const XCB_PRESENT_COMPLETE_MODE_COPY: xcb_present_complete_mode_t = 0;
pub const XCB_PRESENT_COMPLETE_MODE_FLIP: xcb_present_complete_mode_t = 1;
pub const XCB_PRESENT_COMPLETE_MODE_SKIP: xcb_present_complete_mode_t = 2;
pub const XCB_PRESENT_COMPLETE_MODE_SUBOPTIMAL_COPY: xcb_present_complete_mode_t = 3;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_notify_iterator_t {
    pub data: *mut xcb_present_notify_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_present_notify_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_query_version_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_present_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_present_query_version.
pub const XCB_PRESENT_QUERY_VERSION: u8 = 0i32 as u8;

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

/// Opcode for xcb_present_pixmap.
pub const XCB_PRESENT_PIXMAP: u8 = 1i32 as u8;

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

/// Opcode for xcb_present_notify_msc.
pub const XCB_PRESENT_NOTIFY_MSC: u8 = 2i32 as u8;

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

pub type xcb_present_event_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_event_iterator_t {
    pub data: *mut xcb_present_event_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_present_event_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_present_select_input.
pub const XCB_PRESENT_SELECT_INPUT: u8 = 3i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_query_capabilities_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_present_query_capabilities_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_present_query_capabilities.
pub const XCB_PRESENT_QUERY_CAPABILITIES: u8 = 4i32 as u8;

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

/// Opcode for xcb_present_generic.
pub const XCB_PRESENT_GENERIC: u8 = 0i32 as u8;

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

/// Opcode for xcb_present_configure_notify.
pub const XCB_PRESENT_CONFIGURE_NOTIFY: u8 = 0i32 as u8;

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

/// Opcode for xcb_present_complete_notify.
pub const XCB_PRESENT_COMPLETE_NOTIFY: u8 = 1i32 as u8;

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

/// Opcode for xcb_present_idle_notify.
pub const XCB_PRESENT_IDLE_NOTIFY: u8 = 2i32 as u8;

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

/// Opcode for xcb_present_redirect_notify.
pub const XCB_PRESENT_REDIRECT_NOTIFY: u8 = 3i32 as u8;

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
        ) -> xcb_present_query_version_reply_t,
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
        ) -> xcb_present_query_capabilities_reply_t,
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
    pub fn xcb_present_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_present_id) }
    }

    /// Returns `true` iff the symbol `xcb_present_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_id(&self) -> bool {
        has_sym!(self, xcb_present_id)
    }

    pub unsafe fn xcb_present_notify_next(&self, i: *mut xcb_present_notify_iterator_t) {
        sym!(self, xcb_present_notify_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_present_notify_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_notify_next(&self) -> bool {
        has_sym!(self, xcb_present_notify_next)
    }

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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_present_query_version_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_present_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_present_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_present_query_version_reply_t {
        sym!(self, xcb_present_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_present_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_present_query_version_reply)
    }

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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    pub unsafe fn xcb_present_event_next(&self, i: *mut xcb_present_event_iterator_t) {
        sym!(self, xcb_present_event_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_present_event_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_event_next(&self) -> bool {
        has_sym!(self, xcb_present_event_next)
    }

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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_present_query_capabilities_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_present_query_capabilities_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_present_query_capabilities_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_present_query_capabilities_reply_t {
        sym!(self, xcb_present_query_capabilities_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_present_query_capabilities_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_present_query_capabilities_reply(&self) -> bool {
        has_sym!(self, xcb_present_query_capabilities_reply)
    }

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
