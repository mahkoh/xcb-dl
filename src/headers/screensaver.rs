// This file was generated using generate.py. Do not edit.

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

pub type xcb_screensaver_kind_t = u32;
pub const XCB_SCREENSAVER_KIND_BLANKED: xcb_screensaver_kind_t = 0;
pub const XCB_SCREENSAVER_KIND_INTERNAL: xcb_screensaver_kind_t = 1;
pub const XCB_SCREENSAVER_KIND_EXTERNAL: xcb_screensaver_kind_t = 2;

pub type xcb_screensaver_event_t = u32;
pub const XCB_SCREENSAVER_EVENT_NOTIFY_MASK: xcb_screensaver_event_t = 1;
pub const XCB_SCREENSAVER_EVENT_CYCLE_MASK: xcb_screensaver_event_t = 2;

pub type xcb_screensaver_state_t = u32;
pub const XCB_SCREENSAVER_STATE_OFF: xcb_screensaver_state_t = 0;
pub const XCB_SCREENSAVER_STATE_ON: xcb_screensaver_state_t = 1;
pub const XCB_SCREENSAVER_STATE_CYCLE: xcb_screensaver_state_t = 2;
pub const XCB_SCREENSAVER_STATE_DISABLED: xcb_screensaver_state_t = 3;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screensaver_query_version_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_screensaver_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_screensaver_query_version.
pub const XCB_SCREENSAVER_QUERY_VERSION: u8 = 0i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screensaver_query_info_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_screensaver_query_info_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_screensaver_query_info.
pub const XCB_SCREENSAVER_QUERY_INFO: u8 = 1i32 as u8;

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

/// Opcode for xcb_screensaver_select_input.
pub const XCB_SCREENSAVER_SELECT_INPUT: u8 = 2i32 as u8;

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

/// Opcode for xcb_screensaver_set_attributes.
pub const XCB_SCREENSAVER_SET_ATTRIBUTES: u8 = 3i32 as u8;

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

/// Opcode for xcb_screensaver_unset_attributes.
pub const XCB_SCREENSAVER_UNSET_ATTRIBUTES: u8 = 4i32 as u8;

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

/// Opcode for xcb_screensaver_suspend.
pub const XCB_SCREENSAVER_SUSPEND: u8 = 5i32 as u8;

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

/// Opcode for xcb_screensaver_notify.
pub const XCB_SCREENSAVER_NOTIFY: u8 = 0i32 as u8;

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
        ) -> xcb_screensaver_query_version_reply_t,
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
        ) -> xcb_screensaver_query_info_reply_t,
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
    pub fn xcb_screensaver_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_screensaver_id) }
    }

    /// Returns `true` iff the symbol `xcb_screensaver_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screensaver_id(&self) -> bool {
        has_sym!(self, xcb_screensaver_id)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_screensaver_query_version_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_screensaver_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_screensaver_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_screensaver_query_version_reply_t {
        sym!(self, xcb_screensaver_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_screensaver_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screensaver_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_screensaver_query_version_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_screensaver_query_info_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_screensaver_query_info_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_screensaver_query_info_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_screensaver_query_info_reply_t {
        sym!(self, xcb_screensaver_query_info_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_screensaver_query_info_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screensaver_query_info_reply(&self) -> bool {
        has_sym!(self, xcb_screensaver_query_info_reply)
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    pub unsafe fn xcb_screensaver_set_attributes_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_screensaver_set_attributes_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_screensaver_set_attributes_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screensaver_set_attributes_sizeof(&self) -> bool {
        has_sym!(self, xcb_screensaver_set_attributes_sizeof)
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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
