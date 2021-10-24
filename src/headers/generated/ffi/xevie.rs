use crate::*;
use std::os::raw::*;

pub const XCB_XEVIE_MAJOR_VERSION: u32 = 1;
pub const XCB_XEVIE_MINOR_VERSION: u32 = 0;

pub const XCB_XEVIE_QUERY_VERSION: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub client_major_version: u16,
    pub client_minor_version: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_query_version_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub server_major_version: u16,
    pub server_minor_version: u16,
    pub pad1: [u8; 20],
}

pub const XCB_XEVIE_START: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_start_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_start_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_start_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 24],
}

pub const XCB_XEVIE_END: u8 = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_end_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub cmap: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_end_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_end_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 24],
}

pub type xcb_xevie_datatype_t = u32;
pub const XCB_XEVIE_DATATYPE_UNMODIFIED: xcb_xevie_datatype_t = 0x00;
pub const XCB_XEVIE_DATATYPE_MODIFIED: xcb_xevie_datatype_t = 0x01;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_event_t {
    pub pad0: [u8; 32],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_event_iterator_t {
    pub data: *mut xcb_xevie_event_t,
    pub rem: c_int,
    pub index: c_int,
}

pub const XCB_XEVIE_SEND: u8 = 3;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_send_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub event: xcb_xevie_event_t,
    pub data_type: u32,
    pub pad0: [u8; 64],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_send_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_send_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 24],
}

pub const XCB_XEVIE_SELECT_INPUT: u8 = 4;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_select_input_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub event_mask: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_select_input_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xevie_select_input_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad1: [u8; 24],
}

impl XcbXevie {
    #[inline]
    pub fn xcb_xevie_id(&self) -> *mut xcb_extension_t {
        call!(self, xcb_xevie_id)
    }

    #[inline]
    pub unsafe fn xcb_xevie_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xevie_query_version_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xevie_query_version_reply_t {
        call!(self, xcb_xevie_query_version_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xevie_query_version(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u16,
        client_minor_version: u16,
    ) -> xcb_xevie_query_version_cookie_t {
        call!(self, xcb_xevie_query_version)(c, client_major_version, client_minor_version)
    }

    #[inline]
    pub unsafe fn xcb_xevie_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u16,
        client_minor_version: u16,
    ) -> xcb_xevie_query_version_cookie_t {
        call!(self, xcb_xevie_query_version_unchecked)(
            c,
            client_major_version,
            client_minor_version,
        )
    }

    #[inline]
    pub unsafe fn xcb_xevie_start_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xevie_start_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xevie_start_reply_t {
        call!(self, xcb_xevie_start_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xevie_start(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_xevie_start_cookie_t {
        call!(self, xcb_xevie_start)(c, screen)
    }

    #[inline]
    pub unsafe fn xcb_xevie_start_unchecked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_xevie_start_cookie_t {
        call!(self, xcb_xevie_start_unchecked)(c, screen)
    }

    #[inline]
    pub unsafe fn xcb_xevie_end_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xevie_end_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xevie_end_reply_t {
        call!(self, xcb_xevie_end_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xevie_end(
        &self,
        c: *mut xcb_connection_t,
        cmap: u32,
    ) -> xcb_xevie_end_cookie_t {
        call!(self, xcb_xevie_end)(c, cmap)
    }

    #[inline]
    pub unsafe fn xcb_xevie_end_unchecked(
        &self,
        c: *mut xcb_connection_t,
        cmap: u32,
    ) -> xcb_xevie_end_cookie_t {
        call!(self, xcb_xevie_end_unchecked)(c, cmap)
    }

    #[inline]
    pub unsafe fn xcb_xevie_event_next(&self, i: *mut xcb_xevie_event_iterator_t) {
        call!(self, xcb_xevie_event_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_xevie_event_end(
        &self,
        i: *mut xcb_xevie_event_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_xevie_event_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_xevie_send_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xevie_send_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xevie_send_reply_t {
        call!(self, xcb_xevie_send_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xevie_send(
        &self,
        c: *mut xcb_connection_t,
        event: xcb_xevie_event_t,
        data_type: u32,
    ) -> xcb_xevie_send_cookie_t {
        call!(self, xcb_xevie_send)(c, event, data_type)
    }

    #[inline]
    pub unsafe fn xcb_xevie_send_unchecked(
        &self,
        c: *mut xcb_connection_t,
        event: xcb_xevie_event_t,
        data_type: u32,
    ) -> xcb_xevie_send_cookie_t {
        call!(self, xcb_xevie_send_unchecked)(c, event, data_type)
    }

    #[inline]
    pub unsafe fn xcb_xevie_select_input_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xevie_select_input_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xevie_select_input_reply_t {
        call!(self, xcb_xevie_select_input_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xevie_select_input(
        &self,
        c: *mut xcb_connection_t,
        event_mask: u32,
    ) -> xcb_xevie_select_input_cookie_t {
        call!(self, xcb_xevie_select_input)(c, event_mask)
    }

    #[inline]
    pub unsafe fn xcb_xevie_select_input_unchecked(
        &self,
        c: *mut xcb_connection_t,
        event_mask: u32,
    ) -> xcb_xevie_select_input_cookie_t {
        call!(self, xcb_xevie_select_input_unchecked)(c, event_mask)
    }
}

pub struct XcbXevie {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb_xevie_id: LazySymbol<*mut xcb_extension_t>,
    pub(crate) xcb_xevie_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xevie_query_version_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xevie_query_version_reply_t,
    >,
    pub(crate) xcb_xevie_query_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u16,
            client_minor_version: u16,
        ) -> xcb_xevie_query_version_cookie_t,
    >,
    pub(crate) xcb_xevie_query_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u16,
            client_minor_version: u16,
        ) -> xcb_xevie_query_version_cookie_t,
    >,
    pub(crate) xcb_xevie_start_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xevie_start_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xevie_start_reply_t,
    >,
    pub(crate) xcb_xevie_start:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, screen: u32) -> xcb_xevie_start_cookie_t>,
    pub(crate) xcb_xevie_start_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, screen: u32) -> xcb_xevie_start_cookie_t>,
    pub(crate) xcb_xevie_end_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xevie_end_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xevie_end_reply_t,
    >,
    pub(crate) xcb_xevie_end:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cmap: u32) -> xcb_xevie_end_cookie_t>,
    pub(crate) xcb_xevie_end_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cmap: u32) -> xcb_xevie_end_cookie_t>,
    pub(crate) xcb_xevie_event_next: LazySymbol<unsafe fn(i: *mut xcb_xevie_event_iterator_t)>,
    pub(crate) xcb_xevie_event_end:
        LazySymbol<unsafe fn(i: *mut xcb_xevie_event_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_xevie_send_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xevie_send_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xevie_send_reply_t,
    >,
    pub(crate) xcb_xevie_send: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            event: xcb_xevie_event_t,
            data_type: u32,
        ) -> xcb_xevie_send_cookie_t,
    >,
    pub(crate) xcb_xevie_send_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            event: xcb_xevie_event_t,
            data_type: u32,
        ) -> xcb_xevie_send_cookie_t,
    >,
    pub(crate) xcb_xevie_select_input_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xevie_select_input_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xevie_select_input_reply_t,
    >,
    pub(crate) xcb_xevie_select_input: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, event_mask: u32) -> xcb_xevie_select_input_cookie_t,
    >,
    pub(crate) xcb_xevie_select_input_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, event_mask: u32) -> xcb_xevie_select_input_cookie_t,
    >,
}
