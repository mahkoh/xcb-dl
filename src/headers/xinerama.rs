// This file was generated using generate.py. Do not edit.

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_screen_info_t {
    pub x_org: i16,
    pub y_org: i16,
    pub width: u16,
    pub height: u16,
}

impl Default for xcb_xinerama_screen_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_screen_info_iterator_t {
    pub data: *mut xcb_xinerama_screen_info_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xinerama_screen_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_query_version_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xinerama_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xinerama_query_version.
pub const XCB_XINERAMA_QUERY_VERSION: u8 = 0i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub major: u8,
    pub minor: u8,
}

impl Default for xcb_xinerama_query_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major: u16,
    pub minor: u16,
}

impl Default for xcb_xinerama_query_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_get_state_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xinerama_get_state_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xinerama_get_state.
pub const XCB_XINERAMA_GET_STATE: u8 = 1i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_get_state_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_xinerama_get_state_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_get_state_reply_t {
    pub response_type: u8,
    pub state: u8,
    pub sequence: u16,
    pub length: u32,
    pub window: xcb_window_t,
}

impl Default for xcb_xinerama_get_state_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_get_screen_count_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xinerama_get_screen_count_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xinerama_get_screen_count.
pub const XCB_XINERAMA_GET_SCREEN_COUNT: u8 = 2i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_get_screen_count_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_xinerama_get_screen_count_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_get_screen_count_reply_t {
    pub response_type: u8,
    pub screen_count: u8,
    pub sequence: u16,
    pub length: u32,
    pub window: xcb_window_t,
}

impl Default for xcb_xinerama_get_screen_count_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_get_screen_size_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xinerama_get_screen_size_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xinerama_get_screen_size.
pub const XCB_XINERAMA_GET_SCREEN_SIZE: u8 = 3i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_get_screen_size_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub screen: u32,
}

impl Default for xcb_xinerama_get_screen_size_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_get_screen_size_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: u32,
    pub height: u32,
    pub window: xcb_window_t,
    pub screen: u32,
}

impl Default for xcb_xinerama_get_screen_size_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_is_active_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xinerama_is_active_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xinerama_is_active.
pub const XCB_XINERAMA_IS_ACTIVE: u8 = 4i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_is_active_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_xinerama_is_active_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_is_active_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub state: u32,
}

impl Default for xcb_xinerama_is_active_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_query_screens_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xinerama_query_screens_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xinerama_query_screens.
pub const XCB_XINERAMA_QUERY_SCREENS: u8 = 5i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_query_screens_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_xinerama_query_screens_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_query_screens_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub number: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_xinerama_query_screens_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub(crate) struct XcbXineramaXinerama {
    xcb_xinerama_id: LazySymbol<*mut xcb_extension_t>,
    xcb_xinerama_screen_info_next:
        LazySymbol<unsafe fn(i: *mut xcb_xinerama_screen_info_iterator_t)>,
    xcb_xinerama_screen_info_end:
        LazySymbol<unsafe fn(i: xcb_xinerama_screen_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xinerama_query_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major: u8,
            minor: u8,
        ) -> xcb_xinerama_query_version_cookie_t,
    >,
    xcb_xinerama_query_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major: u8,
            minor: u8,
        ) -> xcb_xinerama_query_version_cookie_t,
    >,
    xcb_xinerama_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xinerama_query_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_xinerama_query_version_reply_t,
    >,
    xcb_xinerama_get_state: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_xinerama_get_state_cookie_t,
    >,
    xcb_xinerama_get_state_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_xinerama_get_state_cookie_t,
    >,
    xcb_xinerama_get_state_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xinerama_get_state_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_xinerama_get_state_reply_t,
    >,
    xcb_xinerama_get_screen_count: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_xinerama_get_screen_count_cookie_t,
    >,
    xcb_xinerama_get_screen_count_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_xinerama_get_screen_count_cookie_t,
    >,
    xcb_xinerama_get_screen_count_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xinerama_get_screen_count_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_xinerama_get_screen_count_reply_t,
    >,
    xcb_xinerama_get_screen_size: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            screen: u32,
        ) -> xcb_xinerama_get_screen_size_cookie_t,
    >,
    xcb_xinerama_get_screen_size_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            screen: u32,
        ) -> xcb_xinerama_get_screen_size_cookie_t,
    >,
    xcb_xinerama_get_screen_size_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xinerama_get_screen_size_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_xinerama_get_screen_size_reply_t,
    >,
    xcb_xinerama_is_active:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_xinerama_is_active_cookie_t>,
    xcb_xinerama_is_active_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_xinerama_is_active_cookie_t>,
    xcb_xinerama_is_active_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xinerama_is_active_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_xinerama_is_active_reply_t,
    >,
    xcb_xinerama_query_screens_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xinerama_query_screens:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_xinerama_query_screens_cookie_t>,
    xcb_xinerama_query_screens_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_xinerama_query_screens_cookie_t>,
    xcb_xinerama_query_screens_screen_info: LazySymbol<
        unsafe fn(r: *const xcb_xinerama_query_screens_reply_t) -> *mut xcb_xinerama_screen_info_t,
    >,
    xcb_xinerama_query_screens_screen_info_length:
        LazySymbol<unsafe fn(r: *const xcb_xinerama_query_screens_reply_t) -> c_int>,
    xcb_xinerama_query_screens_screen_info_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xinerama_query_screens_reply_t,
        ) -> xcb_xinerama_screen_info_iterator_t,
    >,
    xcb_xinerama_query_screens_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xinerama_query_screens_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_xinerama_query_screens_reply_t,
    >,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self
            .xinerama
            .$f
            .get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .xinerama
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

impl XcbXinerama {
    pub fn xcb_xinerama_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_xinerama_id) }
    }

    /// Returns `true` iff the symbol `xcb_xinerama_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_id(&self) -> bool {
        has_sym!(self, xcb_xinerama_id)
    }

    pub unsafe fn xcb_xinerama_screen_info_next(
        &self,
        i: *mut xcb_xinerama_screen_info_iterator_t,
    ) {
        sym!(self, xcb_xinerama_screen_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_screen_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_screen_info_next(&self) -> bool {
        has_sym!(self, xcb_xinerama_screen_info_next)
    }

    pub unsafe fn xcb_xinerama_screen_info_end(
        &self,
        i: xcb_xinerama_screen_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xinerama_screen_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_screen_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_screen_info_end(&self) -> bool {
        has_sym!(self, xcb_xinerama_screen_info_end)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xinerama_query_version(
        &self,
        c: *mut xcb_connection_t,
        major: u8,
        minor: u8,
    ) -> xcb_xinerama_query_version_cookie_t {
        sym!(self, xcb_xinerama_query_version)(c, major, minor)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_query_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_query_version(&self) -> bool {
        has_sym!(self, xcb_xinerama_query_version)
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
    pub unsafe fn xcb_xinerama_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        major: u8,
        minor: u8,
    ) -> xcb_xinerama_query_version_cookie_t {
        sym!(self, xcb_xinerama_query_version_unchecked)(c, major, minor)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_query_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_query_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_xinerama_query_version_unchecked)
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
     * xcb_xinerama_query_version_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xinerama_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xinerama_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xinerama_query_version_reply_t {
        sym!(self, xcb_xinerama_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_xinerama_query_version_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xinerama_get_state(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_xinerama_get_state_cookie_t {
        sym!(self, xcb_xinerama_get_state)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_get_state` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_get_state(&self) -> bool {
        has_sym!(self, xcb_xinerama_get_state)
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
    pub unsafe fn xcb_xinerama_get_state_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_xinerama_get_state_cookie_t {
        sym!(self, xcb_xinerama_get_state_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_get_state_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_get_state_unchecked(&self) -> bool {
        has_sym!(self, xcb_xinerama_get_state_unchecked)
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
     * xcb_xinerama_get_state_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xinerama_get_state_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xinerama_get_state_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xinerama_get_state_reply_t {
        sym!(self, xcb_xinerama_get_state_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_get_state_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_get_state_reply(&self) -> bool {
        has_sym!(self, xcb_xinerama_get_state_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xinerama_get_screen_count(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_xinerama_get_screen_count_cookie_t {
        sym!(self, xcb_xinerama_get_screen_count)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_get_screen_count` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_get_screen_count(&self) -> bool {
        has_sym!(self, xcb_xinerama_get_screen_count)
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
    pub unsafe fn xcb_xinerama_get_screen_count_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_xinerama_get_screen_count_cookie_t {
        sym!(self, xcb_xinerama_get_screen_count_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_get_screen_count_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_get_screen_count_unchecked(&self) -> bool {
        has_sym!(self, xcb_xinerama_get_screen_count_unchecked)
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
     * xcb_xinerama_get_screen_count_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xinerama_get_screen_count_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xinerama_get_screen_count_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xinerama_get_screen_count_reply_t {
        sym!(self, xcb_xinerama_get_screen_count_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_get_screen_count_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_get_screen_count_reply(&self) -> bool {
        has_sym!(self, xcb_xinerama_get_screen_count_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xinerama_get_screen_size(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        screen: u32,
    ) -> xcb_xinerama_get_screen_size_cookie_t {
        sym!(self, xcb_xinerama_get_screen_size)(c, window, screen)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_get_screen_size` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_get_screen_size(&self) -> bool {
        has_sym!(self, xcb_xinerama_get_screen_size)
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
    pub unsafe fn xcb_xinerama_get_screen_size_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        screen: u32,
    ) -> xcb_xinerama_get_screen_size_cookie_t {
        sym!(self, xcb_xinerama_get_screen_size_unchecked)(c, window, screen)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_get_screen_size_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_get_screen_size_unchecked(&self) -> bool {
        has_sym!(self, xcb_xinerama_get_screen_size_unchecked)
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
     * xcb_xinerama_get_screen_size_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xinerama_get_screen_size_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xinerama_get_screen_size_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xinerama_get_screen_size_reply_t {
        sym!(self, xcb_xinerama_get_screen_size_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_get_screen_size_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_get_screen_size_reply(&self) -> bool {
        has_sym!(self, xcb_xinerama_get_screen_size_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xinerama_is_active(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xinerama_is_active_cookie_t {
        sym!(self, xcb_xinerama_is_active)(c)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_is_active` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_is_active(&self) -> bool {
        has_sym!(self, xcb_xinerama_is_active)
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
    pub unsafe fn xcb_xinerama_is_active_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xinerama_is_active_cookie_t {
        sym!(self, xcb_xinerama_is_active_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_is_active_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_is_active_unchecked(&self) -> bool {
        has_sym!(self, xcb_xinerama_is_active_unchecked)
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
     * xcb_xinerama_is_active_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xinerama_is_active_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xinerama_is_active_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xinerama_is_active_reply_t {
        sym!(self, xcb_xinerama_is_active_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_is_active_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_is_active_reply(&self) -> bool {
        has_sym!(self, xcb_xinerama_is_active_reply)
    }

    pub unsafe fn xcb_xinerama_query_screens_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xinerama_query_screens_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_query_screens_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_query_screens_sizeof(&self) -> bool {
        has_sym!(self, xcb_xinerama_query_screens_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xinerama_query_screens(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xinerama_query_screens_cookie_t {
        sym!(self, xcb_xinerama_query_screens)(c)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_query_screens` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_query_screens(&self) -> bool {
        has_sym!(self, xcb_xinerama_query_screens)
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
    pub unsafe fn xcb_xinerama_query_screens_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xinerama_query_screens_cookie_t {
        sym!(self, xcb_xinerama_query_screens_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_query_screens_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_query_screens_unchecked(&self) -> bool {
        has_sym!(self, xcb_xinerama_query_screens_unchecked)
    }

    pub unsafe fn xcb_xinerama_query_screens_screen_info(
        &self,
        r: *const xcb_xinerama_query_screens_reply_t,
    ) -> *mut xcb_xinerama_screen_info_t {
        sym!(self, xcb_xinerama_query_screens_screen_info)(r)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_query_screens_screen_info` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_query_screens_screen_info(&self) -> bool {
        has_sym!(self, xcb_xinerama_query_screens_screen_info)
    }

    pub unsafe fn xcb_xinerama_query_screens_screen_info_length(
        &self,
        r: *const xcb_xinerama_query_screens_reply_t,
    ) -> c_int {
        sym!(self, xcb_xinerama_query_screens_screen_info_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_query_screens_screen_info_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_query_screens_screen_info_length(&self) -> bool {
        has_sym!(self, xcb_xinerama_query_screens_screen_info_length)
    }

    pub unsafe fn xcb_xinerama_query_screens_screen_info_iterator(
        &self,
        r: *const xcb_xinerama_query_screens_reply_t,
    ) -> xcb_xinerama_screen_info_iterator_t {
        sym!(self, xcb_xinerama_query_screens_screen_info_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_query_screens_screen_info_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_query_screens_screen_info_iterator(&self) -> bool {
        has_sym!(self, xcb_xinerama_query_screens_screen_info_iterator)
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
     * xcb_xinerama_query_screens_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xinerama_query_screens_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xinerama_query_screens_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xinerama_query_screens_reply_t {
        sym!(self, xcb_xinerama_query_screens_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xinerama_query_screens_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xinerama_query_screens_reply(&self) -> bool {
        has_sym!(self, xcb_xinerama_query_screens_reply)
    }
}

#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbXinerama::load().unwrap() };
        assert!(lib.has_xcb_xinerama_id());
        assert!(lib.has_xcb_xinerama_screen_info_next());
        assert!(lib.has_xcb_xinerama_screen_info_end());
        assert!(lib.has_xcb_xinerama_query_version());
        assert!(lib.has_xcb_xinerama_query_version_unchecked());
        assert!(lib.has_xcb_xinerama_query_version_reply());
        assert!(lib.has_xcb_xinerama_get_state());
        assert!(lib.has_xcb_xinerama_get_state_unchecked());
        assert!(lib.has_xcb_xinerama_get_state_reply());
        assert!(lib.has_xcb_xinerama_get_screen_count());
        assert!(lib.has_xcb_xinerama_get_screen_count_unchecked());
        assert!(lib.has_xcb_xinerama_get_screen_count_reply());
        assert!(lib.has_xcb_xinerama_get_screen_size());
        assert!(lib.has_xcb_xinerama_get_screen_size_unchecked());
        assert!(lib.has_xcb_xinerama_get_screen_size_reply());
        assert!(lib.has_xcb_xinerama_is_active());
        assert!(lib.has_xcb_xinerama_is_active_unchecked());
        assert!(lib.has_xcb_xinerama_is_active_reply());
        assert!(lib.has_xcb_xinerama_query_screens_sizeof());
        assert!(lib.has_xcb_xinerama_query_screens());
        assert!(lib.has_xcb_xinerama_query_screens_unchecked());
        assert!(lib.has_xcb_xinerama_query_screens_screen_info());
        assert!(lib.has_xcb_xinerama_query_screens_screen_info_length());
        assert!(lib.has_xcb_xinerama_query_screens_screen_info_iterator());
        assert!(lib.has_xcb_xinerama_query_screens_reply());
    }
}
