// This file was generated using generate.py. Do not edit.

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_get_version_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_dpms_get_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_dpms_get_version.
pub const XCB_DPMS_GET_VERSION: u8 = 0i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_get_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub client_major_version: u16,
    pub client_minor_version: u16,
}

impl Default for xcb_dpms_get_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_get_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub server_major_version: u16,
    pub server_minor_version: u16,
}

impl Default for xcb_dpms_get_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_capable_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_dpms_capable_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_dpms_capable.
pub const XCB_DPMS_CAPABLE: u8 = 1i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_capable_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_dpms_capable_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_capable_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub capable: u8,
    pub pad1: [u8; 23],
}

impl Default for xcb_dpms_capable_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_get_timeouts_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_dpms_get_timeouts_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_dpms_get_timeouts.
pub const XCB_DPMS_GET_TIMEOUTS: u8 = 2i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_get_timeouts_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_dpms_get_timeouts_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_get_timeouts_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub standby_timeout: u16,
    pub suspend_timeout: u16,
    pub off_timeout: u16,
    pub pad1: [u8; 18],
}

impl Default for xcb_dpms_get_timeouts_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_dpms_set_timeouts.
pub const XCB_DPMS_SET_TIMEOUTS: u8 = 3i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_set_timeouts_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub standby_timeout: u16,
    pub suspend_timeout: u16,
    pub off_timeout: u16,
}

impl Default for xcb_dpms_set_timeouts_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_dpms_enable.
pub const XCB_DPMS_ENABLE: u8 = 4i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_enable_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_dpms_enable_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_dpms_disable.
pub const XCB_DPMS_DISABLE: u8 = 5i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_disable_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_dpms_disable_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_dpms_dpms_mode_t = u32;
pub const XCB_DPMS_DPMS_MODE_ON: xcb_dpms_dpms_mode_t = 0;
pub const XCB_DPMS_DPMS_MODE_STANDBY: xcb_dpms_dpms_mode_t = 1;
pub const XCB_DPMS_DPMS_MODE_SUSPEND: xcb_dpms_dpms_mode_t = 2;
pub const XCB_DPMS_DPMS_MODE_OFF: xcb_dpms_dpms_mode_t = 3;

/// Opcode for xcb_dpms_force_level.
pub const XCB_DPMS_FORCE_LEVEL: u8 = 6i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_force_level_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub power_level: u16,
}

impl Default for xcb_dpms_force_level_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_info_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_dpms_info_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_dpms_info.
pub const XCB_DPMS_INFO: u8 = 7i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_info_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_dpms_info_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_info_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub power_level: u16,
    pub state: u8,
    pub pad1: [u8; 21],
}

impl Default for xcb_dpms_info_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[cfg(feature = "xcb_dpms")]
pub(crate) struct XcbDpmsDpms {
    xcb_dpms_id: LazySymbol<*mut xcb_extension_t>,
    xcb_dpms_get_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u16,
            client_minor_version: u16,
        ) -> xcb_dpms_get_version_cookie_t,
    >,
    xcb_dpms_get_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u16,
            client_minor_version: u16,
        ) -> xcb_dpms_get_version_cookie_t,
    >,
    xcb_dpms_get_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dpms_get_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_dpms_get_version_reply_t,
    >,
    xcb_dpms_capable: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_dpms_capable_cookie_t>,
    xcb_dpms_capable_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_dpms_capable_cookie_t>,
    xcb_dpms_capable_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dpms_capable_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_dpms_capable_reply_t,
    >,
    xcb_dpms_get_timeouts:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_dpms_get_timeouts_cookie_t>,
    xcb_dpms_get_timeouts_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_dpms_get_timeouts_cookie_t>,
    xcb_dpms_get_timeouts_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dpms_get_timeouts_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_dpms_get_timeouts_reply_t,
    >,
    xcb_dpms_set_timeouts_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            standby_timeout: u16,
            suspend_timeout: u16,
            off_timeout: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_dpms_set_timeouts: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            standby_timeout: u16,
            suspend_timeout: u16,
            off_timeout: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_dpms_enable_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    xcb_dpms_enable: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    xcb_dpms_disable_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    xcb_dpms_disable: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    xcb_dpms_force_level_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, power_level: u16) -> xcb_void_cookie_t>,
    xcb_dpms_force_level:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, power_level: u16) -> xcb_void_cookie_t>,
    xcb_dpms_info: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_dpms_info_cookie_t>,
    xcb_dpms_info_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_dpms_info_cookie_t>,
    xcb_dpms_info_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dpms_info_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_dpms_info_reply_t,
    >,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self.dpms.$f.get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .dpms
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

#[cfg(feature = "xcb_dpms")]
impl XcbDpms {
    pub fn xcb_dpms_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_dpms_id) }
    }

    /// Returns `true` iff the symbol `xcb_dpms_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_id(&self) -> bool {
        has_sym!(self, xcb_dpms_id)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_dpms_get_version(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u16,
        client_minor_version: u16,
    ) -> xcb_dpms_get_version_cookie_t {
        sym!(self, xcb_dpms_get_version)(c, client_major_version, client_minor_version)
    }

    /// Returns `true` iff the symbol `xcb_dpms_get_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_get_version(&self) -> bool {
        has_sym!(self, xcb_dpms_get_version)
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
    pub unsafe fn xcb_dpms_get_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u16,
        client_minor_version: u16,
    ) -> xcb_dpms_get_version_cookie_t {
        sym!(self, xcb_dpms_get_version_unchecked)(c, client_major_version, client_minor_version)
    }

    /// Returns `true` iff the symbol `xcb_dpms_get_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_get_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_dpms_get_version_unchecked)
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
     * xcb_dpms_get_version_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_dpms_get_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dpms_get_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_dpms_get_version_reply_t {
        sym!(self, xcb_dpms_get_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dpms_get_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_get_version_reply(&self) -> bool {
        has_sym!(self, xcb_dpms_get_version_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_dpms_capable(&self, c: *mut xcb_connection_t) -> xcb_dpms_capable_cookie_t {
        sym!(self, xcb_dpms_capable)(c)
    }

    /// Returns `true` iff the symbol `xcb_dpms_capable` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_capable(&self) -> bool {
        has_sym!(self, xcb_dpms_capable)
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
    pub unsafe fn xcb_dpms_capable_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_dpms_capable_cookie_t {
        sym!(self, xcb_dpms_capable_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_dpms_capable_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_capable_unchecked(&self) -> bool {
        has_sym!(self, xcb_dpms_capable_unchecked)
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
     * xcb_dpms_capable_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_dpms_capable_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dpms_capable_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_dpms_capable_reply_t {
        sym!(self, xcb_dpms_capable_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dpms_capable_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_capable_reply(&self) -> bool {
        has_sym!(self, xcb_dpms_capable_reply)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_dpms_get_timeouts(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_dpms_get_timeouts_cookie_t {
        sym!(self, xcb_dpms_get_timeouts)(c)
    }

    /// Returns `true` iff the symbol `xcb_dpms_get_timeouts` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_get_timeouts(&self) -> bool {
        has_sym!(self, xcb_dpms_get_timeouts)
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
    pub unsafe fn xcb_dpms_get_timeouts_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_dpms_get_timeouts_cookie_t {
        sym!(self, xcb_dpms_get_timeouts_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_dpms_get_timeouts_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_get_timeouts_unchecked(&self) -> bool {
        has_sym!(self, xcb_dpms_get_timeouts_unchecked)
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
     * xcb_dpms_get_timeouts_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_dpms_get_timeouts_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dpms_get_timeouts_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_dpms_get_timeouts_reply_t {
        sym!(self, xcb_dpms_get_timeouts_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dpms_get_timeouts_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_get_timeouts_reply(&self) -> bool {
        has_sym!(self, xcb_dpms_get_timeouts_reply)
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
    pub unsafe fn xcb_dpms_set_timeouts_checked(
        &self,
        c: *mut xcb_connection_t,
        standby_timeout: u16,
        suspend_timeout: u16,
        off_timeout: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_dpms_set_timeouts_checked)(c, standby_timeout, suspend_timeout, off_timeout)
    }

    /// Returns `true` iff the symbol `xcb_dpms_set_timeouts_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_set_timeouts_checked(&self) -> bool {
        has_sym!(self, xcb_dpms_set_timeouts_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_dpms_set_timeouts(
        &self,
        c: *mut xcb_connection_t,
        standby_timeout: u16,
        suspend_timeout: u16,
        off_timeout: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_dpms_set_timeouts)(c, standby_timeout, suspend_timeout, off_timeout)
    }

    /// Returns `true` iff the symbol `xcb_dpms_set_timeouts` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_set_timeouts(&self) -> bool {
        has_sym!(self, xcb_dpms_set_timeouts)
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
    pub unsafe fn xcb_dpms_enable_checked(&self, c: *mut xcb_connection_t) -> xcb_void_cookie_t {
        sym!(self, xcb_dpms_enable_checked)(c)
    }

    /// Returns `true` iff the symbol `xcb_dpms_enable_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_enable_checked(&self) -> bool {
        has_sym!(self, xcb_dpms_enable_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_dpms_enable(&self, c: *mut xcb_connection_t) -> xcb_void_cookie_t {
        sym!(self, xcb_dpms_enable)(c)
    }

    /// Returns `true` iff the symbol `xcb_dpms_enable` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_enable(&self) -> bool {
        has_sym!(self, xcb_dpms_enable)
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
    pub unsafe fn xcb_dpms_disable_checked(&self, c: *mut xcb_connection_t) -> xcb_void_cookie_t {
        sym!(self, xcb_dpms_disable_checked)(c)
    }

    /// Returns `true` iff the symbol `xcb_dpms_disable_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_disable_checked(&self) -> bool {
        has_sym!(self, xcb_dpms_disable_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_dpms_disable(&self, c: *mut xcb_connection_t) -> xcb_void_cookie_t {
        sym!(self, xcb_dpms_disable)(c)
    }

    /// Returns `true` iff the symbol `xcb_dpms_disable` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_disable(&self) -> bool {
        has_sym!(self, xcb_dpms_disable)
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
    pub unsafe fn xcb_dpms_force_level_checked(
        &self,
        c: *mut xcb_connection_t,
        power_level: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_dpms_force_level_checked)(c, power_level)
    }

    /// Returns `true` iff the symbol `xcb_dpms_force_level_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_force_level_checked(&self) -> bool {
        has_sym!(self, xcb_dpms_force_level_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_dpms_force_level(
        &self,
        c: *mut xcb_connection_t,
        power_level: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_dpms_force_level)(c, power_level)
    }

    /// Returns `true` iff the symbol `xcb_dpms_force_level` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_force_level(&self) -> bool {
        has_sym!(self, xcb_dpms_force_level)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_dpms_info(&self, c: *mut xcb_connection_t) -> xcb_dpms_info_cookie_t {
        sym!(self, xcb_dpms_info)(c)
    }

    /// Returns `true` iff the symbol `xcb_dpms_info` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_info(&self) -> bool {
        has_sym!(self, xcb_dpms_info)
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
    pub unsafe fn xcb_dpms_info_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_dpms_info_cookie_t {
        sym!(self, xcb_dpms_info_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_dpms_info_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_info_unchecked(&self) -> bool {
        has_sym!(self, xcb_dpms_info_unchecked)
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
     * xcb_dpms_info_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_dpms_info_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dpms_info_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_dpms_info_reply_t {
        sym!(self, xcb_dpms_info_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_dpms_info_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_dpms_info_reply(&self) -> bool {
        has_sym!(self, xcb_dpms_info_reply)
    }
}

#[cfg(feature = "xcb_dpms")]
#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbDpms::load().unwrap() };
        assert!(lib.has_xcb_dpms_id());
        assert!(lib.has_xcb_dpms_get_version());
        assert!(lib.has_xcb_dpms_get_version_unchecked());
        assert!(lib.has_xcb_dpms_get_version_reply());
        assert!(lib.has_xcb_dpms_capable());
        assert!(lib.has_xcb_dpms_capable_unchecked());
        assert!(lib.has_xcb_dpms_capable_reply());
        assert!(lib.has_xcb_dpms_get_timeouts());
        assert!(lib.has_xcb_dpms_get_timeouts_unchecked());
        assert!(lib.has_xcb_dpms_get_timeouts_reply());
        assert!(lib.has_xcb_dpms_set_timeouts_checked());
        assert!(lib.has_xcb_dpms_set_timeouts());
        assert!(lib.has_xcb_dpms_enable_checked());
        assert!(lib.has_xcb_dpms_enable());
        assert!(lib.has_xcb_dpms_disable_checked());
        assert!(lib.has_xcb_dpms_disable());
        assert!(lib.has_xcb_dpms_force_level_checked());
        assert!(lib.has_xcb_dpms_force_level());
        assert!(lib.has_xcb_dpms_info());
        assert!(lib.has_xcb_dpms_info_unchecked());
        assert!(lib.has_xcb_dpms_info_reply());
    }
}
