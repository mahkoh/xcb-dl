// This file was generated using generate.py. Do not edit.

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_test_get_version_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_test_get_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_test_get_version.
pub const XCB_TEST_GET_VERSION: u8 = 0i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_test_get_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub major_version: u8,
    pub pad0: u8,
    pub minor_version: u16,
}

impl Default for xcb_test_get_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_test_get_version_reply_t {
    pub response_type: u8,
    pub major_version: u8,
    pub sequence: u16,
    pub length: u32,
    pub minor_version: u16,
}

impl Default for xcb_test_get_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_test_cursor_t = u32;
pub const XCB_TEST_CURSOR_NONE: xcb_test_cursor_t = 0;
pub const XCB_TEST_CURSOR_CURRENT: xcb_test_cursor_t = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_test_compare_cursor_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_test_compare_cursor_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_test_compare_cursor.
pub const XCB_TEST_COMPARE_CURSOR: u8 = 1i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_test_compare_cursor_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub cursor: xcb_cursor_t,
}

impl Default for xcb_test_compare_cursor_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_test_compare_cursor_reply_t {
    pub response_type: u8,
    pub same: u8,
    pub sequence: u16,
    pub length: u32,
}

impl Default for xcb_test_compare_cursor_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_test_fake_input.
pub const XCB_TEST_FAKE_INPUT: u8 = 2i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_test_fake_input_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub type_: u8,
    pub detail: u8,
    pub pad0: [u8; 2],
    pub time: u32,
    pub root: xcb_window_t,
    pub pad1: [u8; 8],
    pub root_x: i16,
    pub root_y: i16,
    pub pad2: [u8; 7],
    pub deviceid: u8,
}

impl Default for xcb_test_fake_input_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_test_grab_control.
pub const XCB_TEST_GRAB_CONTROL: u8 = 3i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_test_grab_control_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub impervious: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_test_grab_control_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub(crate) struct XcbXtestXtest {
    xcb_test_id: LazySymbol<*mut xcb_extension_t>,
    xcb_test_get_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major_version: u8,
            minor_version: u16,
        ) -> xcb_test_get_version_cookie_t,
    >,
    xcb_test_get_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major_version: u8,
            minor_version: u16,
        ) -> xcb_test_get_version_cookie_t,
    >,
    xcb_test_get_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_test_get_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_test_get_version_reply_t,
    >,
    xcb_test_compare_cursor: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            cursor: xcb_cursor_t,
        ) -> xcb_test_compare_cursor_cookie_t,
    >,
    xcb_test_compare_cursor_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            cursor: xcb_cursor_t,
        ) -> xcb_test_compare_cursor_cookie_t,
    >,
    xcb_test_compare_cursor_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_test_compare_cursor_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_test_compare_cursor_reply_t,
    >,
    xcb_test_fake_input_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            type_: u8,
            detail: u8,
            time: u32,
            root: xcb_window_t,
            root_x: i16,
            root_y: i16,
            deviceid: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_test_fake_input: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            type_: u8,
            detail: u8,
            time: u32,
            root: xcb_window_t,
            root_x: i16,
            root_y: i16,
            deviceid: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_test_grab_control_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, impervious: u8) -> xcb_void_cookie_t>,
    xcb_test_grab_control:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, impervious: u8) -> xcb_void_cookie_t>,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self
            .xtest
            .$f
            .get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .xtest
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

impl XcbXtest {
    pub fn xcb_test_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_test_id) }
    }

    /// Returns `true` iff the symbol `xcb_test_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_test_id(&self) -> bool {
        has_sym!(self, xcb_test_id)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_test_get_version(
        &self,
        c: *mut xcb_connection_t,
        major_version: u8,
        minor_version: u16,
    ) -> xcb_test_get_version_cookie_t {
        sym!(self, xcb_test_get_version)(c, major_version, minor_version)
    }

    /// Returns `true` iff the symbol `xcb_test_get_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_test_get_version(&self) -> bool {
        has_sym!(self, xcb_test_get_version)
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
    pub unsafe fn xcb_test_get_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        major_version: u8,
        minor_version: u16,
    ) -> xcb_test_get_version_cookie_t {
        sym!(self, xcb_test_get_version_unchecked)(c, major_version, minor_version)
    }

    /// Returns `true` iff the symbol `xcb_test_get_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_test_get_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_test_get_version_unchecked)
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
     * xcb_test_get_version_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_test_get_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_test_get_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_test_get_version_reply_t {
        sym!(self, xcb_test_get_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_test_get_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_test_get_version_reply(&self) -> bool {
        has_sym!(self, xcb_test_get_version_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_test_compare_cursor(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        cursor: xcb_cursor_t,
    ) -> xcb_test_compare_cursor_cookie_t {
        sym!(self, xcb_test_compare_cursor)(c, window, cursor)
    }

    /// Returns `true` iff the symbol `xcb_test_compare_cursor` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_test_compare_cursor(&self) -> bool {
        has_sym!(self, xcb_test_compare_cursor)
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
    pub unsafe fn xcb_test_compare_cursor_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        cursor: xcb_cursor_t,
    ) -> xcb_test_compare_cursor_cookie_t {
        sym!(self, xcb_test_compare_cursor_unchecked)(c, window, cursor)
    }

    /// Returns `true` iff the symbol `xcb_test_compare_cursor_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_test_compare_cursor_unchecked(&self) -> bool {
        has_sym!(self, xcb_test_compare_cursor_unchecked)
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
     * xcb_test_compare_cursor_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_test_compare_cursor_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_test_compare_cursor_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_test_compare_cursor_reply_t {
        sym!(self, xcb_test_compare_cursor_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_test_compare_cursor_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_test_compare_cursor_reply(&self) -> bool {
        has_sym!(self, xcb_test_compare_cursor_reply)
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
    pub unsafe fn xcb_test_fake_input_checked(
        &self,
        c: *mut xcb_connection_t,
        type_: u8,
        detail: u8,
        time: u32,
        root: xcb_window_t,
        root_x: i16,
        root_y: i16,
        deviceid: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_test_fake_input_checked)(
            c, type_, detail, time, root, root_x, root_y, deviceid,
        )
    }

    /// Returns `true` iff the symbol `xcb_test_fake_input_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_test_fake_input_checked(&self) -> bool {
        has_sym!(self, xcb_test_fake_input_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_test_fake_input(
        &self,
        c: *mut xcb_connection_t,
        type_: u8,
        detail: u8,
        time: u32,
        root: xcb_window_t,
        root_x: i16,
        root_y: i16,
        deviceid: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_test_fake_input)(c, type_, detail, time, root, root_x, root_y, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_test_fake_input` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_test_fake_input(&self) -> bool {
        has_sym!(self, xcb_test_fake_input)
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
    pub unsafe fn xcb_test_grab_control_checked(
        &self,
        c: *mut xcb_connection_t,
        impervious: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_test_grab_control_checked)(c, impervious)
    }

    /// Returns `true` iff the symbol `xcb_test_grab_control_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_test_grab_control_checked(&self) -> bool {
        has_sym!(self, xcb_test_grab_control_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_test_grab_control(
        &self,
        c: *mut xcb_connection_t,
        impervious: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_test_grab_control)(c, impervious)
    }

    /// Returns `true` iff the symbol `xcb_test_grab_control` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_test_grab_control(&self) -> bool {
        has_sym!(self, xcb_test_grab_control)
    }
}

#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbXtest::load().unwrap() };
        assert!(lib.has_xcb_test_id());
        assert!(lib.has_xcb_test_get_version());
        assert!(lib.has_xcb_test_get_version_unchecked());
        assert!(lib.has_xcb_test_get_version_reply());
        assert!(lib.has_xcb_test_compare_cursor());
        assert!(lib.has_xcb_test_compare_cursor_unchecked());
        assert!(lib.has_xcb_test_compare_cursor_reply());
        assert!(lib.has_xcb_test_fake_input_checked());
        assert!(lib.has_xcb_test_fake_input());
        assert!(lib.has_xcb_test_grab_control_checked());
        assert!(lib.has_xcb_test_grab_control());
    }
}
