use crate::*;
use std::os::raw::*;

pub const XCB_DPMS_MAJOR_VERSION: u32 = 0;
pub const XCB_DPMS_MINOR_VERSION: u32 = 0;

pub const XCB_DPMS_GET_VERSION: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_get_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub client_major_version: u16,
    pub client_minor_version: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_get_version_cookie_t {
    pub sequence: c_uint,
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

pub const XCB_DPMS_CAPABLE: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_capable_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_capable_cookie_t {
    pub sequence: c_uint,
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

pub const XCB_DPMS_GET_TIMEOUTS: u8 = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_get_timeouts_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_get_timeouts_cookie_t {
    pub sequence: c_uint,
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

pub const XCB_DPMS_SET_TIMEOUTS: u8 = 3;

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

pub const XCB_DPMS_ENABLE: u8 = 4;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_enable_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

pub const XCB_DPMS_DISABLE: u8 = 5;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_disable_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

pub type xcb_dpms_dpms_mode_t = u32;
pub const XCB_DPMS_DPMS_MODE_ON: xcb_dpms_dpms_mode_t = 0x00;
pub const XCB_DPMS_DPMS_MODE_STANDBY: xcb_dpms_dpms_mode_t = 0x01;
pub const XCB_DPMS_DPMS_MODE_SUSPEND: xcb_dpms_dpms_mode_t = 0x02;
pub const XCB_DPMS_DPMS_MODE_OFF: xcb_dpms_dpms_mode_t = 0x03;

pub const XCB_DPMS_FORCE_LEVEL: u8 = 6;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_force_level_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub power_level: u16,
}

pub const XCB_DPMS_INFO: u8 = 7;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_info_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dpms_info_cookie_t {
    pub sequence: c_uint,
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

impl XcbDpms {
    #[inline]
    pub fn xcb_dpms_id(&self) -> *mut xcb_extension_t {
        call!(self, xcb_dpms_id)
    }

    #[inline]
    pub unsafe fn xcb_dpms_get_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dpms_get_version_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dpms_get_version_reply_t {
        call!(self, xcb_dpms_get_version_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_dpms_get_version(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u16,
        client_minor_version: u16,
    ) -> xcb_dpms_get_version_cookie_t {
        call!(self, xcb_dpms_get_version)(c, client_major_version, client_minor_version)
    }

    #[inline]
    pub unsafe fn xcb_dpms_get_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u16,
        client_minor_version: u16,
    ) -> xcb_dpms_get_version_cookie_t {
        call!(self, xcb_dpms_get_version_unchecked)(c, client_major_version, client_minor_version)
    }

    #[inline]
    pub unsafe fn xcb_dpms_capable_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dpms_capable_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dpms_capable_reply_t {
        call!(self, xcb_dpms_capable_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_dpms_capable(&self, c: *mut xcb_connection_t) -> xcb_dpms_capable_cookie_t {
        call!(self, xcb_dpms_capable)(c)
    }

    #[inline]
    pub unsafe fn xcb_dpms_capable_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_dpms_capable_cookie_t {
        call!(self, xcb_dpms_capable_unchecked)(c)
    }

    #[inline]
    pub unsafe fn xcb_dpms_get_timeouts_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dpms_get_timeouts_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dpms_get_timeouts_reply_t {
        call!(self, xcb_dpms_get_timeouts_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_dpms_get_timeouts(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_dpms_get_timeouts_cookie_t {
        call!(self, xcb_dpms_get_timeouts)(c)
    }

    #[inline]
    pub unsafe fn xcb_dpms_get_timeouts_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_dpms_get_timeouts_cookie_t {
        call!(self, xcb_dpms_get_timeouts_unchecked)(c)
    }

    #[inline]
    pub unsafe fn xcb_dpms_set_timeouts(
        &self,
        c: *mut xcb_connection_t,
        standby_timeout: u16,
        suspend_timeout: u16,
        off_timeout: u16,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_dpms_set_timeouts)(c, standby_timeout, suspend_timeout, off_timeout)
    }

    #[inline]
    pub unsafe fn xcb_dpms_set_timeouts_checked(
        &self,
        c: *mut xcb_connection_t,
        standby_timeout: u16,
        suspend_timeout: u16,
        off_timeout: u16,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_dpms_set_timeouts_checked)(c, standby_timeout, suspend_timeout, off_timeout)
    }

    #[inline]
    pub unsafe fn xcb_dpms_enable(&self, c: *mut xcb_connection_t) -> xcb_void_cookie_t {
        call!(self, xcb_dpms_enable)(c)
    }

    #[inline]
    pub unsafe fn xcb_dpms_enable_checked(&self, c: *mut xcb_connection_t) -> xcb_void_cookie_t {
        call!(self, xcb_dpms_enable_checked)(c)
    }

    #[inline]
    pub unsafe fn xcb_dpms_disable(&self, c: *mut xcb_connection_t) -> xcb_void_cookie_t {
        call!(self, xcb_dpms_disable)(c)
    }

    #[inline]
    pub unsafe fn xcb_dpms_disable_checked(&self, c: *mut xcb_connection_t) -> xcb_void_cookie_t {
        call!(self, xcb_dpms_disable_checked)(c)
    }

    #[inline]
    pub unsafe fn xcb_dpms_force_level(
        &self,
        c: *mut xcb_connection_t,
        power_level: u16,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_dpms_force_level)(c, power_level)
    }

    #[inline]
    pub unsafe fn xcb_dpms_force_level_checked(
        &self,
        c: *mut xcb_connection_t,
        power_level: u16,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_dpms_force_level_checked)(c, power_level)
    }

    #[inline]
    pub unsafe fn xcb_dpms_info_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dpms_info_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dpms_info_reply_t {
        call!(self, xcb_dpms_info_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_dpms_info(&self, c: *mut xcb_connection_t) -> xcb_dpms_info_cookie_t {
        call!(self, xcb_dpms_info)(c)
    }

    #[inline]
    pub unsafe fn xcb_dpms_info_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_dpms_info_cookie_t {
        call!(self, xcb_dpms_info_unchecked)(c)
    }
}

pub struct XcbDpms {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb_dpms_id: LazySymbol<*mut xcb_extension_t>,
    pub(crate) xcb_dpms_get_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dpms_get_version_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_dpms_get_version_reply_t,
    >,
    pub(crate) xcb_dpms_get_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u16,
            client_minor_version: u16,
        ) -> xcb_dpms_get_version_cookie_t,
    >,
    pub(crate) xcb_dpms_get_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u16,
            client_minor_version: u16,
        ) -> xcb_dpms_get_version_cookie_t,
    >,
    pub(crate) xcb_dpms_capable_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dpms_capable_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_dpms_capable_reply_t,
    >,
    pub(crate) xcb_dpms_capable:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_dpms_capable_cookie_t>,
    pub(crate) xcb_dpms_capable_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_dpms_capable_cookie_t>,
    pub(crate) xcb_dpms_get_timeouts_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dpms_get_timeouts_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_dpms_get_timeouts_reply_t,
    >,
    pub(crate) xcb_dpms_get_timeouts:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_dpms_get_timeouts_cookie_t>,
    pub(crate) xcb_dpms_get_timeouts_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_dpms_get_timeouts_cookie_t>,
    pub(crate) xcb_dpms_set_timeouts: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            standby_timeout: u16,
            suspend_timeout: u16,
            off_timeout: u16,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_dpms_set_timeouts_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            standby_timeout: u16,
            suspend_timeout: u16,
            off_timeout: u16,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_dpms_enable:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_dpms_enable_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_dpms_disable:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_dpms_disable_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_dpms_force_level:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, power_level: u16) -> xcb_void_cookie_t>,
    pub(crate) xcb_dpms_force_level_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, power_level: u16) -> xcb_void_cookie_t>,
    pub(crate) xcb_dpms_info_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dpms_info_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_dpms_info_reply_t,
    >,
    pub(crate) xcb_dpms_info:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_dpms_info_cookie_t>,
    pub(crate) xcb_dpms_info_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_dpms_info_cookie_t>,
}
