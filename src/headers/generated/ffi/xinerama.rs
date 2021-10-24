use crate::ffi::*;
use crate::*;
use std::os::raw::*;

pub const XCB_XINERAMA_MAJOR_VERSION: u32 = 1;
pub const XCB_XINERAMA_MINOR_VERSION: u32 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_screen_info_t {
    pub x_org: i16,
    pub y_org: i16,
    pub width: u16,
    pub height: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_screen_info_iterator_t {
    pub data: *mut xcb_xinerama_screen_info_t,
    pub rem: c_int,
    pub index: c_int,
}

pub const XCB_XINERAMA_QUERY_VERSION: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub major: u8,
    pub minor: u8,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_query_version_cookie_t {
    pub sequence: c_uint,
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

pub const XCB_XINERAMA_GET_STATE: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_get_state_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_get_state_cookie_t {
    pub sequence: c_uint,
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

pub const XCB_XINERAMA_GET_SCREEN_COUNT: u8 = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_get_screen_count_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_get_screen_count_cookie_t {
    pub sequence: c_uint,
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

pub const XCB_XINERAMA_GET_SCREEN_SIZE: u8 = 3;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_get_screen_size_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub screen: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_get_screen_size_cookie_t {
    pub sequence: c_uint,
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

pub const XCB_XINERAMA_IS_ACTIVE: u8 = 4;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_is_active_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_is_active_cookie_t {
    pub sequence: c_uint,
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

pub const XCB_XINERAMA_QUERY_SCREENS: u8 = 5;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_query_screens_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xinerama_query_screens_cookie_t {
    pub sequence: c_uint,
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

impl XcbXinerama {
    #[inline]
    pub unsafe fn xcb_xinerama_id(&self) -> *mut xcb_extension_t {
        sym!(self, xcb_xinerama_id)
    }

    #[inline]
    pub unsafe fn xcb_xinerama_screen_info_next(
        &self,
        i: *mut xcb_xinerama_screen_info_iterator_t,
    ) {
        sym!(self, xcb_xinerama_screen_info_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_xinerama_screen_info_end(
        &self,
        i: *mut xcb_xinerama_screen_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xinerama_screen_info_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_xinerama_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xinerama_query_version_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xinerama_query_version_reply_t {
        sym!(self, xcb_xinerama_query_version_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xinerama_query_version(
        &self,
        c: *mut xcb_connection_t,
        major: u8,
        minor: u8,
    ) -> xcb_xinerama_query_version_cookie_t {
        sym!(self, xcb_xinerama_query_version)(c, major, minor)
    }

    #[inline]
    pub unsafe fn xcb_xinerama_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        major: u8,
        minor: u8,
    ) -> xcb_xinerama_query_version_cookie_t {
        sym!(self, xcb_xinerama_query_version_unchecked)(c, major, minor)
    }

    #[inline]
    pub unsafe fn xcb_xinerama_get_state_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xinerama_get_state_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xinerama_get_state_reply_t {
        sym!(self, xcb_xinerama_get_state_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xinerama_get_state(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_xinerama_get_state_cookie_t {
        sym!(self, xcb_xinerama_get_state)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_xinerama_get_state_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_xinerama_get_state_cookie_t {
        sym!(self, xcb_xinerama_get_state_unchecked)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_xinerama_get_screen_count_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xinerama_get_screen_count_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xinerama_get_screen_count_reply_t {
        sym!(self, xcb_xinerama_get_screen_count_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xinerama_get_screen_count(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_xinerama_get_screen_count_cookie_t {
        sym!(self, xcb_xinerama_get_screen_count)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_xinerama_get_screen_count_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_xinerama_get_screen_count_cookie_t {
        sym!(self, xcb_xinerama_get_screen_count_unchecked)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_xinerama_get_screen_size_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xinerama_get_screen_size_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xinerama_get_screen_size_reply_t {
        sym!(self, xcb_xinerama_get_screen_size_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xinerama_get_screen_size(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        screen: u32,
    ) -> xcb_xinerama_get_screen_size_cookie_t {
        sym!(self, xcb_xinerama_get_screen_size)(c, window, screen)
    }

    #[inline]
    pub unsafe fn xcb_xinerama_get_screen_size_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        screen: u32,
    ) -> xcb_xinerama_get_screen_size_cookie_t {
        sym!(self, xcb_xinerama_get_screen_size_unchecked)(c, window, screen)
    }

    #[inline]
    pub unsafe fn xcb_xinerama_is_active_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xinerama_is_active_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xinerama_is_active_reply_t {
        sym!(self, xcb_xinerama_is_active_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xinerama_is_active(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xinerama_is_active_cookie_t {
        sym!(self, xcb_xinerama_is_active)(c)
    }

    #[inline]
    pub unsafe fn xcb_xinerama_is_active_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xinerama_is_active_cookie_t {
        sym!(self, xcb_xinerama_is_active_unchecked)(c)
    }

    #[inline]
    pub unsafe fn xcb_xinerama_query_screens_screen_info(
        &self,
        R: *const xcb_xinerama_query_screens_reply_t,
    ) -> *mut xcb_xinerama_screen_info_t {
        sym!(self, xcb_xinerama_query_screens_screen_info)(R)
    }

    #[inline]
    pub unsafe fn xcb_xinerama_query_screens_screen_info_length(
        &self,
        R: *const xcb_xinerama_query_screens_reply_t,
    ) -> c_int {
        sym!(self, xcb_xinerama_query_screens_screen_info_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_xinerama_query_screens_screen_info_iterator(
        &self,
        R: *const xcb_xinerama_query_screens_reply_t,
    ) -> xcb_xinerama_screen_info_iterator_t {
        sym!(self, xcb_xinerama_query_screens_screen_info_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_xinerama_query_screens_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xinerama_query_screens_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xinerama_query_screens_reply_t {
        sym!(self, xcb_xinerama_query_screens_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xinerama_query_screens(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xinerama_query_screens_cookie_t {
        sym!(self, xcb_xinerama_query_screens)(c)
    }

    #[inline]
    pub unsafe fn xcb_xinerama_query_screens_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xinerama_query_screens_cookie_t {
        sym!(self, xcb_xinerama_query_screens_unchecked)(c)
    }
}
