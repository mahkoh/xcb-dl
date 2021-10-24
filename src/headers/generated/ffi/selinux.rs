use crate::*;
use std::os::raw::*;

pub const XCB_SELINUX_MAJOR_VERSION: u32 = 1;
pub const XCB_SELINUX_MINOR_VERSION: u32 = 0;

pub const XCB_SELINUX_QUERY_VERSION: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub client_major: u8,
    pub client_minor: u8,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_query_version_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub server_major: u16,
    pub server_minor: u16,
}

pub const XCB_SELINUX_SET_DEVICE_CREATE_CONTEXT: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_set_device_create_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_len: u32,
}

pub const XCB_SELINUX_GET_DEVICE_CREATE_CONTEXT: u8 = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_device_create_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_device_create_context_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_device_create_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_len: u32,
    pub pad1: [u8; 20],
}

pub const XCB_SELINUX_SET_DEVICE_CONTEXT: u8 = 3;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_set_device_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device: u32,
    pub context_len: u32,
}

pub const XCB_SELINUX_GET_DEVICE_CONTEXT: u8 = 4;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_device_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_device_context_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_device_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_len: u32,
    pub pad1: [u8; 20],
}

pub const XCB_SELINUX_SET_WINDOW_CREATE_CONTEXT: u8 = 5;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_set_window_create_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_len: u32,
}

pub const XCB_SELINUX_GET_WINDOW_CREATE_CONTEXT: u8 = 6;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_window_create_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_window_create_context_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_window_create_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_len: u32,
    pub pad1: [u8; 20],
}

pub const XCB_SELINUX_GET_WINDOW_CONTEXT: u8 = 7;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_window_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_window_context_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_window_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_len: u32,
    pub pad1: [u8; 20],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_list_item_t {
    pub name: xcb_atom_t,
    pub object_context_len: u32,
    pub data_context_len: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_list_item_iterator_t {
    pub data: *mut xcb_selinux_list_item_t,
    pub rem: c_int,
    pub index: c_int,
}

pub const XCB_SELINUX_SET_PROPERTY_CREATE_CONTEXT: u8 = 8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_set_property_create_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_len: u32,
}

pub const XCB_SELINUX_GET_PROPERTY_CREATE_CONTEXT: u8 = 9;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_property_create_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_property_create_context_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_property_create_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_len: u32,
    pub pad1: [u8; 20],
}

pub const XCB_SELINUX_SET_PROPERTY_USE_CONTEXT: u8 = 10;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_set_property_use_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_len: u32,
}

pub const XCB_SELINUX_GET_PROPERTY_USE_CONTEXT: u8 = 11;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_property_use_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_property_use_context_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_property_use_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_len: u32,
    pub pad1: [u8; 20],
}

pub const XCB_SELINUX_GET_PROPERTY_CONTEXT: u8 = 12;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_property_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub property: xcb_atom_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_property_context_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_property_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_len: u32,
    pub pad1: [u8; 20],
}

pub const XCB_SELINUX_GET_PROPERTY_DATA_CONTEXT: u8 = 13;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_property_data_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub property: xcb_atom_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_property_data_context_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_property_data_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_len: u32,
    pub pad1: [u8; 20],
}

pub const XCB_SELINUX_LIST_PROPERTIES: u8 = 14;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_list_properties_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_list_properties_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_list_properties_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub properties_len: u32,
    pub pad1: [u8; 20],
}

pub const XCB_SELINUX_SET_SELECTION_CREATE_CONTEXT: u8 = 15;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_set_selection_create_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_len: u32,
}

pub const XCB_SELINUX_GET_SELECTION_CREATE_CONTEXT: u8 = 16;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_selection_create_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_selection_create_context_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_selection_create_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_len: u32,
    pub pad1: [u8; 20],
}

pub const XCB_SELINUX_SET_SELECTION_USE_CONTEXT: u8 = 17;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_set_selection_use_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub context_len: u32,
}

pub const XCB_SELINUX_GET_SELECTION_USE_CONTEXT: u8 = 18;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_selection_use_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_selection_use_context_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_selection_use_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_len: u32,
    pub pad1: [u8; 20],
}

pub const XCB_SELINUX_GET_SELECTION_CONTEXT: u8 = 19;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_selection_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub selection: xcb_atom_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_selection_context_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_selection_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_len: u32,
    pub pad1: [u8; 20],
}

pub const XCB_SELINUX_GET_SELECTION_DATA_CONTEXT: u8 = 20;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_selection_data_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub selection: xcb_atom_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_selection_data_context_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_selection_data_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_len: u32,
    pub pad1: [u8; 20],
}

pub const XCB_SELINUX_LIST_SELECTIONS: u8 = 21;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_list_selections_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_list_selections_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_list_selections_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub selections_len: u32,
    pub pad1: [u8; 20],
}

pub const XCB_SELINUX_GET_CLIENT_CONTEXT: u8 = 22;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_client_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub resource: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_client_context_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selinux_get_client_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub context_len: u32,
    pub pad1: [u8; 20],
}

impl XcbXselinux {
    #[inline]
    pub fn xcb_selinux_id(&self) -> *mut xcb_extension_t {
        call!(self, xcb_selinux_id)
    }

    #[inline]
    pub unsafe fn xcb_selinux_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_query_version_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_query_version_reply_t {
        call!(self, xcb_selinux_query_version_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_selinux_query_version(
        &self,
        c: *mut xcb_connection_t,
        client_major: u8,
        client_minor: u8,
    ) -> xcb_selinux_query_version_cookie_t {
        call!(self, xcb_selinux_query_version)(c, client_major, client_minor)
    }

    #[inline]
    pub unsafe fn xcb_selinux_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        client_major: u8,
        client_minor: u8,
    ) -> xcb_selinux_query_version_cookie_t {
        call!(self, xcb_selinux_query_version_unchecked)(c, client_major, client_minor)
    }

    #[inline]
    pub unsafe fn xcb_selinux_set_device_create_context(
        &self,
        c: *mut xcb_connection_t,
        context_len: u32,
        context: *const c_char,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_selinux_set_device_create_context)(c, context_len, context)
    }

    #[inline]
    pub unsafe fn xcb_selinux_set_device_create_context_checked(
        &self,
        c: *mut xcb_connection_t,
        context_len: u32,
        context: *const c_char,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_selinux_set_device_create_context_checked)(c, context_len, context)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_device_create_context_context(
        &self,
        R: *const xcb_selinux_get_device_create_context_reply_t,
    ) -> *mut c_char {
        call!(self, xcb_selinux_get_device_create_context_context)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_device_create_context_context_length(
        &self,
        R: *const xcb_selinux_get_device_create_context_reply_t,
    ) -> c_int {
        call!(self, xcb_selinux_get_device_create_context_context_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_device_create_context_context_end(
        &self,
        R: *const xcb_selinux_get_device_create_context_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_selinux_get_device_create_context_context_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_device_create_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_device_create_context_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_get_device_create_context_reply_t {
        call!(self, xcb_selinux_get_device_create_context_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_device_create_context(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_selinux_get_device_create_context_cookie_t {
        call!(self, xcb_selinux_get_device_create_context)(c)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_device_create_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_selinux_get_device_create_context_cookie_t {
        call!(self, xcb_selinux_get_device_create_context_unchecked)(c)
    }

    #[inline]
    pub unsafe fn xcb_selinux_set_device_context(
        &self,
        c: *mut xcb_connection_t,
        device: u32,
        context_len: u32,
        context: *const c_char,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_selinux_set_device_context)(c, device, context_len, context)
    }

    #[inline]
    pub unsafe fn xcb_selinux_set_device_context_checked(
        &self,
        c: *mut xcb_connection_t,
        device: u32,
        context_len: u32,
        context: *const c_char,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_selinux_set_device_context_checked)(c, device, context_len, context)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_device_context_context(
        &self,
        R: *const xcb_selinux_get_device_context_reply_t,
    ) -> *mut c_char {
        call!(self, xcb_selinux_get_device_context_context)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_device_context_context_length(
        &self,
        R: *const xcb_selinux_get_device_context_reply_t,
    ) -> c_int {
        call!(self, xcb_selinux_get_device_context_context_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_device_context_context_end(
        &self,
        R: *const xcb_selinux_get_device_context_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_selinux_get_device_context_context_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_device_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_device_context_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_get_device_context_reply_t {
        call!(self, xcb_selinux_get_device_context_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_device_context(
        &self,
        c: *mut xcb_connection_t,
        device: u32,
    ) -> xcb_selinux_get_device_context_cookie_t {
        call!(self, xcb_selinux_get_device_context)(c, device)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_device_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device: u32,
    ) -> xcb_selinux_get_device_context_cookie_t {
        call!(self, xcb_selinux_get_device_context_unchecked)(c, device)
    }

    #[inline]
    pub unsafe fn xcb_selinux_set_window_create_context(
        &self,
        c: *mut xcb_connection_t,
        context_len: u32,
        context: *const c_char,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_selinux_set_window_create_context)(c, context_len, context)
    }

    #[inline]
    pub unsafe fn xcb_selinux_set_window_create_context_checked(
        &self,
        c: *mut xcb_connection_t,
        context_len: u32,
        context: *const c_char,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_selinux_set_window_create_context_checked)(c, context_len, context)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_window_create_context_context(
        &self,
        R: *const xcb_selinux_get_window_create_context_reply_t,
    ) -> *mut c_char {
        call!(self, xcb_selinux_get_window_create_context_context)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_window_create_context_context_length(
        &self,
        R: *const xcb_selinux_get_window_create_context_reply_t,
    ) -> c_int {
        call!(self, xcb_selinux_get_window_create_context_context_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_window_create_context_context_end(
        &self,
        R: *const xcb_selinux_get_window_create_context_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_selinux_get_window_create_context_context_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_window_create_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_window_create_context_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_get_window_create_context_reply_t {
        call!(self, xcb_selinux_get_window_create_context_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_window_create_context(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_selinux_get_window_create_context_cookie_t {
        call!(self, xcb_selinux_get_window_create_context)(c)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_window_create_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_selinux_get_window_create_context_cookie_t {
        call!(self, xcb_selinux_get_window_create_context_unchecked)(c)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_window_context_context(
        &self,
        R: *const xcb_selinux_get_window_context_reply_t,
    ) -> *mut c_char {
        call!(self, xcb_selinux_get_window_context_context)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_window_context_context_length(
        &self,
        R: *const xcb_selinux_get_window_context_reply_t,
    ) -> c_int {
        call!(self, xcb_selinux_get_window_context_context_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_window_context_context_end(
        &self,
        R: *const xcb_selinux_get_window_context_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_selinux_get_window_context_context_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_window_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_window_context_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_get_window_context_reply_t {
        call!(self, xcb_selinux_get_window_context_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_window_context(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_selinux_get_window_context_cookie_t {
        call!(self, xcb_selinux_get_window_context)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_window_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_selinux_get_window_context_cookie_t {
        call!(self, xcb_selinux_get_window_context_unchecked)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_selinux_list_item_object_context(
        &self,
        R: *const xcb_selinux_list_item_t,
    ) -> *mut c_char {
        call!(self, xcb_selinux_list_item_object_context)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_list_item_object_context_length(
        &self,
        R: *const xcb_selinux_list_item_t,
    ) -> c_int {
        call!(self, xcb_selinux_list_item_object_context_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_list_item_object_context_end(
        &self,
        R: *const xcb_selinux_list_item_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_selinux_list_item_object_context_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_list_item_data_context(
        &self,
        R: *const xcb_selinux_list_item_t,
    ) -> *mut c_char {
        call!(self, xcb_selinux_list_item_data_context)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_list_item_data_context_length(
        &self,
        R: *const xcb_selinux_list_item_t,
    ) -> c_int {
        call!(self, xcb_selinux_list_item_data_context_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_list_item_data_context_end(
        &self,
        R: *const xcb_selinux_list_item_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_selinux_list_item_data_context_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_list_item_next(&self, i: *mut xcb_selinux_list_item_iterator_t) {
        call!(self, xcb_selinux_list_item_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_selinux_list_item_end(
        &self,
        i: *mut xcb_selinux_list_item_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_selinux_list_item_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_selinux_set_property_create_context(
        &self,
        c: *mut xcb_connection_t,
        context_len: u32,
        context: *const c_char,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_selinux_set_property_create_context)(c, context_len, context)
    }

    #[inline]
    pub unsafe fn xcb_selinux_set_property_create_context_checked(
        &self,
        c: *mut xcb_connection_t,
        context_len: u32,
        context: *const c_char,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_selinux_set_property_create_context_checked)(c, context_len, context)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_property_create_context_context(
        &self,
        R: *const xcb_selinux_get_property_create_context_reply_t,
    ) -> *mut c_char {
        call!(self, xcb_selinux_get_property_create_context_context)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_property_create_context_context_length(
        &self,
        R: *const xcb_selinux_get_property_create_context_reply_t,
    ) -> c_int {
        call!(self, xcb_selinux_get_property_create_context_context_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_property_create_context_context_end(
        &self,
        R: *const xcb_selinux_get_property_create_context_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_selinux_get_property_create_context_context_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_property_create_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_property_create_context_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_get_property_create_context_reply_t {
        call!(self, xcb_selinux_get_property_create_context_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_property_create_context(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_selinux_get_property_create_context_cookie_t {
        call!(self, xcb_selinux_get_property_create_context)(c)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_property_create_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_selinux_get_property_create_context_cookie_t {
        call!(self, xcb_selinux_get_property_create_context_unchecked)(c)
    }

    #[inline]
    pub unsafe fn xcb_selinux_set_property_use_context(
        &self,
        c: *mut xcb_connection_t,
        context_len: u32,
        context: *const c_char,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_selinux_set_property_use_context)(c, context_len, context)
    }

    #[inline]
    pub unsafe fn xcb_selinux_set_property_use_context_checked(
        &self,
        c: *mut xcb_connection_t,
        context_len: u32,
        context: *const c_char,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_selinux_set_property_use_context_checked)(c, context_len, context)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_property_use_context_context(
        &self,
        R: *const xcb_selinux_get_property_use_context_reply_t,
    ) -> *mut c_char {
        call!(self, xcb_selinux_get_property_use_context_context)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_property_use_context_context_length(
        &self,
        R: *const xcb_selinux_get_property_use_context_reply_t,
    ) -> c_int {
        call!(self, xcb_selinux_get_property_use_context_context_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_property_use_context_context_end(
        &self,
        R: *const xcb_selinux_get_property_use_context_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_selinux_get_property_use_context_context_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_property_use_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_property_use_context_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_get_property_use_context_reply_t {
        call!(self, xcb_selinux_get_property_use_context_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_property_use_context(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_selinux_get_property_use_context_cookie_t {
        call!(self, xcb_selinux_get_property_use_context)(c)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_property_use_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_selinux_get_property_use_context_cookie_t {
        call!(self, xcb_selinux_get_property_use_context_unchecked)(c)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_property_context_context(
        &self,
        R: *const xcb_selinux_get_property_context_reply_t,
    ) -> *mut c_char {
        call!(self, xcb_selinux_get_property_context_context)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_property_context_context_length(
        &self,
        R: *const xcb_selinux_get_property_context_reply_t,
    ) -> c_int {
        call!(self, xcb_selinux_get_property_context_context_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_property_context_context_end(
        &self,
        R: *const xcb_selinux_get_property_context_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_selinux_get_property_context_context_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_property_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_property_context_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_get_property_context_reply_t {
        call!(self, xcb_selinux_get_property_context_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_property_context(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        property: xcb_atom_t,
    ) -> xcb_selinux_get_property_context_cookie_t {
        call!(self, xcb_selinux_get_property_context)(c, window, property)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_property_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        property: xcb_atom_t,
    ) -> xcb_selinux_get_property_context_cookie_t {
        call!(self, xcb_selinux_get_property_context_unchecked)(c, window, property)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_property_data_context_context(
        &self,
        R: *const xcb_selinux_get_property_data_context_reply_t,
    ) -> *mut c_char {
        call!(self, xcb_selinux_get_property_data_context_context)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_property_data_context_context_length(
        &self,
        R: *const xcb_selinux_get_property_data_context_reply_t,
    ) -> c_int {
        call!(self, xcb_selinux_get_property_data_context_context_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_property_data_context_context_end(
        &self,
        R: *const xcb_selinux_get_property_data_context_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_selinux_get_property_data_context_context_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_property_data_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_property_data_context_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_get_property_data_context_reply_t {
        call!(self, xcb_selinux_get_property_data_context_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_property_data_context(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        property: xcb_atom_t,
    ) -> xcb_selinux_get_property_data_context_cookie_t {
        call!(self, xcb_selinux_get_property_data_context)(c, window, property)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_property_data_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        property: xcb_atom_t,
    ) -> xcb_selinux_get_property_data_context_cookie_t {
        call!(self, xcb_selinux_get_property_data_context_unchecked)(c, window, property)
    }

    #[inline]
    pub unsafe fn xcb_selinux_list_properties_properties_length(
        &self,
        R: *const xcb_selinux_list_properties_reply_t,
    ) -> c_int {
        call!(self, xcb_selinux_list_properties_properties_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_list_properties_properties_iterator(
        &self,
        R: *const xcb_selinux_list_properties_reply_t,
    ) -> xcb_selinux_list_item_iterator_t {
        call!(self, xcb_selinux_list_properties_properties_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_list_properties_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_list_properties_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_list_properties_reply_t {
        call!(self, xcb_selinux_list_properties_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_selinux_list_properties(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_selinux_list_properties_cookie_t {
        call!(self, xcb_selinux_list_properties)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_selinux_list_properties_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_selinux_list_properties_cookie_t {
        call!(self, xcb_selinux_list_properties_unchecked)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_selinux_set_selection_create_context(
        &self,
        c: *mut xcb_connection_t,
        context_len: u32,
        context: *const c_char,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_selinux_set_selection_create_context)(c, context_len, context)
    }

    #[inline]
    pub unsafe fn xcb_selinux_set_selection_create_context_checked(
        &self,
        c: *mut xcb_connection_t,
        context_len: u32,
        context: *const c_char,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_selinux_set_selection_create_context_checked)(c, context_len, context)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_selection_create_context_context(
        &self,
        R: *const xcb_selinux_get_selection_create_context_reply_t,
    ) -> *mut c_char {
        call!(self, xcb_selinux_get_selection_create_context_context)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_selection_create_context_context_length(
        &self,
        R: *const xcb_selinux_get_selection_create_context_reply_t,
    ) -> c_int {
        call!(
            self,
            xcb_selinux_get_selection_create_context_context_length
        )(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_selection_create_context_context_end(
        &self,
        R: *const xcb_selinux_get_selection_create_context_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_selinux_get_selection_create_context_context_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_selection_create_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_selection_create_context_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_get_selection_create_context_reply_t {
        call!(self, xcb_selinux_get_selection_create_context_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_selection_create_context(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_selinux_get_selection_create_context_cookie_t {
        call!(self, xcb_selinux_get_selection_create_context)(c)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_selection_create_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_selinux_get_selection_create_context_cookie_t {
        call!(self, xcb_selinux_get_selection_create_context_unchecked)(c)
    }

    #[inline]
    pub unsafe fn xcb_selinux_set_selection_use_context(
        &self,
        c: *mut xcb_connection_t,
        context_len: u32,
        context: *const c_char,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_selinux_set_selection_use_context)(c, context_len, context)
    }

    #[inline]
    pub unsafe fn xcb_selinux_set_selection_use_context_checked(
        &self,
        c: *mut xcb_connection_t,
        context_len: u32,
        context: *const c_char,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_selinux_set_selection_use_context_checked)(c, context_len, context)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_selection_use_context_context(
        &self,
        R: *const xcb_selinux_get_selection_use_context_reply_t,
    ) -> *mut c_char {
        call!(self, xcb_selinux_get_selection_use_context_context)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_selection_use_context_context_length(
        &self,
        R: *const xcb_selinux_get_selection_use_context_reply_t,
    ) -> c_int {
        call!(self, xcb_selinux_get_selection_use_context_context_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_selection_use_context_context_end(
        &self,
        R: *const xcb_selinux_get_selection_use_context_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_selinux_get_selection_use_context_context_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_selection_use_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_selection_use_context_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_get_selection_use_context_reply_t {
        call!(self, xcb_selinux_get_selection_use_context_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_selection_use_context(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_selinux_get_selection_use_context_cookie_t {
        call!(self, xcb_selinux_get_selection_use_context)(c)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_selection_use_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_selinux_get_selection_use_context_cookie_t {
        call!(self, xcb_selinux_get_selection_use_context_unchecked)(c)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_selection_context_context(
        &self,
        R: *const xcb_selinux_get_selection_context_reply_t,
    ) -> *mut c_char {
        call!(self, xcb_selinux_get_selection_context_context)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_selection_context_context_length(
        &self,
        R: *const xcb_selinux_get_selection_context_reply_t,
    ) -> c_int {
        call!(self, xcb_selinux_get_selection_context_context_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_selection_context_context_end(
        &self,
        R: *const xcb_selinux_get_selection_context_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_selinux_get_selection_context_context_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_selection_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_selection_context_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_get_selection_context_reply_t {
        call!(self, xcb_selinux_get_selection_context_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_selection_context(
        &self,
        c: *mut xcb_connection_t,
        selection: xcb_atom_t,
    ) -> xcb_selinux_get_selection_context_cookie_t {
        call!(self, xcb_selinux_get_selection_context)(c, selection)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_selection_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
        selection: xcb_atom_t,
    ) -> xcb_selinux_get_selection_context_cookie_t {
        call!(self, xcb_selinux_get_selection_context_unchecked)(c, selection)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_selection_data_context_context(
        &self,
        R: *const xcb_selinux_get_selection_data_context_reply_t,
    ) -> *mut c_char {
        call!(self, xcb_selinux_get_selection_data_context_context)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_selection_data_context_context_length(
        &self,
        R: *const xcb_selinux_get_selection_data_context_reply_t,
    ) -> c_int {
        call!(self, xcb_selinux_get_selection_data_context_context_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_selection_data_context_context_end(
        &self,
        R: *const xcb_selinux_get_selection_data_context_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_selinux_get_selection_data_context_context_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_selection_data_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_selection_data_context_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_get_selection_data_context_reply_t {
        call!(self, xcb_selinux_get_selection_data_context_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_selection_data_context(
        &self,
        c: *mut xcb_connection_t,
        selection: xcb_atom_t,
    ) -> xcb_selinux_get_selection_data_context_cookie_t {
        call!(self, xcb_selinux_get_selection_data_context)(c, selection)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_selection_data_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
        selection: xcb_atom_t,
    ) -> xcb_selinux_get_selection_data_context_cookie_t {
        call!(self, xcb_selinux_get_selection_data_context_unchecked)(c, selection)
    }

    #[inline]
    pub unsafe fn xcb_selinux_list_selections_selections_length(
        &self,
        R: *const xcb_selinux_list_selections_reply_t,
    ) -> c_int {
        call!(self, xcb_selinux_list_selections_selections_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_list_selections_selections_iterator(
        &self,
        R: *const xcb_selinux_list_selections_reply_t,
    ) -> xcb_selinux_list_item_iterator_t {
        call!(self, xcb_selinux_list_selections_selections_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_list_selections_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_list_selections_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_list_selections_reply_t {
        call!(self, xcb_selinux_list_selections_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_selinux_list_selections(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_selinux_list_selections_cookie_t {
        call!(self, xcb_selinux_list_selections)(c)
    }

    #[inline]
    pub unsafe fn xcb_selinux_list_selections_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_selinux_list_selections_cookie_t {
        call!(self, xcb_selinux_list_selections_unchecked)(c)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_client_context_context(
        &self,
        R: *const xcb_selinux_get_client_context_reply_t,
    ) -> *mut c_char {
        call!(self, xcb_selinux_get_client_context_context)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_client_context_context_length(
        &self,
        R: *const xcb_selinux_get_client_context_reply_t,
    ) -> c_int {
        call!(self, xcb_selinux_get_client_context_context_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_client_context_context_end(
        &self,
        R: *const xcb_selinux_get_client_context_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_selinux_get_client_context_context_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_client_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_selinux_get_client_context_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_selinux_get_client_context_reply_t {
        call!(self, xcb_selinux_get_client_context_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_client_context(
        &self,
        c: *mut xcb_connection_t,
        resource: u32,
    ) -> xcb_selinux_get_client_context_cookie_t {
        call!(self, xcb_selinux_get_client_context)(c, resource)
    }

    #[inline]
    pub unsafe fn xcb_selinux_get_client_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
        resource: u32,
    ) -> xcb_selinux_get_client_context_cookie_t {
        call!(self, xcb_selinux_get_client_context_unchecked)(c, resource)
    }
}

pub struct XcbXselinux {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb_selinux_id: LazySymbol<*mut xcb_extension_t>,
    pub(crate) xcb_selinux_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_query_version_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_query_version_reply_t,
    >,
    pub(crate) xcb_selinux_query_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major: u8,
            client_minor: u8,
        ) -> xcb_selinux_query_version_cookie_t,
    >,
    pub(crate) xcb_selinux_query_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major: u8,
            client_minor: u8,
        ) -> xcb_selinux_query_version_cookie_t,
    >,
    pub(crate) xcb_selinux_set_device_create_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_len: u32,
            context: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_selinux_set_device_create_context_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_len: u32,
            context: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_selinux_get_device_create_context_context: LazySymbol<
        unsafe fn(R: *const xcb_selinux_get_device_create_context_reply_t) -> *mut c_char,
    >,
    pub(crate) xcb_selinux_get_device_create_context_context_length:
        LazySymbol<unsafe fn(R: *const xcb_selinux_get_device_create_context_reply_t) -> c_int>,
    pub(crate) xcb_selinux_get_device_create_context_context_end: LazySymbol<
        unsafe fn(
            R: *const xcb_selinux_get_device_create_context_reply_t,
        ) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_selinux_get_device_create_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_get_device_create_context_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_get_device_create_context_reply_t,
    >,
    pub(crate) xcb_selinux_get_device_create_context: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_selinux_get_device_create_context_cookie_t,
    >,
    pub(crate) xcb_selinux_get_device_create_context_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_selinux_get_device_create_context_cookie_t,
    >,
    pub(crate) xcb_selinux_set_device_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device: u32,
            context_len: u32,
            context: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_selinux_set_device_context_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device: u32,
            context_len: u32,
            context: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_selinux_get_device_context_context:
        LazySymbol<unsafe fn(R: *const xcb_selinux_get_device_context_reply_t) -> *mut c_char>,
    pub(crate) xcb_selinux_get_device_context_context_length:
        LazySymbol<unsafe fn(R: *const xcb_selinux_get_device_context_reply_t) -> c_int>,
    pub(crate) xcb_selinux_get_device_context_context_end: LazySymbol<
        unsafe fn(R: *const xcb_selinux_get_device_context_reply_t) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_selinux_get_device_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_get_device_context_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_get_device_context_reply_t,
    >,
    pub(crate) xcb_selinux_get_device_context: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, device: u32) -> xcb_selinux_get_device_context_cookie_t,
    >,
    pub(crate) xcb_selinux_get_device_context_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, device: u32) -> xcb_selinux_get_device_context_cookie_t,
    >,
    pub(crate) xcb_selinux_set_window_create_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_len: u32,
            context: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_selinux_set_window_create_context_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_len: u32,
            context: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_selinux_get_window_create_context_context: LazySymbol<
        unsafe fn(R: *const xcb_selinux_get_window_create_context_reply_t) -> *mut c_char,
    >,
    pub(crate) xcb_selinux_get_window_create_context_context_length:
        LazySymbol<unsafe fn(R: *const xcb_selinux_get_window_create_context_reply_t) -> c_int>,
    pub(crate) xcb_selinux_get_window_create_context_context_end: LazySymbol<
        unsafe fn(
            R: *const xcb_selinux_get_window_create_context_reply_t,
        ) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_selinux_get_window_create_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_get_window_create_context_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_get_window_create_context_reply_t,
    >,
    pub(crate) xcb_selinux_get_window_create_context: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_selinux_get_window_create_context_cookie_t,
    >,
    pub(crate) xcb_selinux_get_window_create_context_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_selinux_get_window_create_context_cookie_t,
    >,
    pub(crate) xcb_selinux_get_window_context_context:
        LazySymbol<unsafe fn(R: *const xcb_selinux_get_window_context_reply_t) -> *mut c_char>,
    pub(crate) xcb_selinux_get_window_context_context_length:
        LazySymbol<unsafe fn(R: *const xcb_selinux_get_window_context_reply_t) -> c_int>,
    pub(crate) xcb_selinux_get_window_context_context_end: LazySymbol<
        unsafe fn(R: *const xcb_selinux_get_window_context_reply_t) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_selinux_get_window_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_get_window_context_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_get_window_context_reply_t,
    >,
    pub(crate) xcb_selinux_get_window_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_selinux_get_window_context_cookie_t,
    >,
    pub(crate) xcb_selinux_get_window_context_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_selinux_get_window_context_cookie_t,
    >,
    pub(crate) xcb_selinux_list_item_object_context:
        LazySymbol<unsafe fn(R: *const xcb_selinux_list_item_t) -> *mut c_char>,
    pub(crate) xcb_selinux_list_item_object_context_length:
        LazySymbol<unsafe fn(R: *const xcb_selinux_list_item_t) -> c_int>,
    pub(crate) xcb_selinux_list_item_object_context_end:
        LazySymbol<unsafe fn(R: *const xcb_selinux_list_item_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_selinux_list_item_data_context:
        LazySymbol<unsafe fn(R: *const xcb_selinux_list_item_t) -> *mut c_char>,
    pub(crate) xcb_selinux_list_item_data_context_length:
        LazySymbol<unsafe fn(R: *const xcb_selinux_list_item_t) -> c_int>,
    pub(crate) xcb_selinux_list_item_data_context_end:
        LazySymbol<unsafe fn(R: *const xcb_selinux_list_item_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_selinux_list_item_next:
        LazySymbol<unsafe fn(i: *mut xcb_selinux_list_item_iterator_t)>,
    pub(crate) xcb_selinux_list_item_end:
        LazySymbol<unsafe fn(i: *mut xcb_selinux_list_item_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_selinux_set_property_create_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_len: u32,
            context: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_selinux_set_property_create_context_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_len: u32,
            context: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_selinux_get_property_create_context_context: LazySymbol<
        unsafe fn(R: *const xcb_selinux_get_property_create_context_reply_t) -> *mut c_char,
    >,
    pub(crate) xcb_selinux_get_property_create_context_context_length:
        LazySymbol<unsafe fn(R: *const xcb_selinux_get_property_create_context_reply_t) -> c_int>,
    pub(crate) xcb_selinux_get_property_create_context_context_end: LazySymbol<
        unsafe fn(
            R: *const xcb_selinux_get_property_create_context_reply_t,
        ) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_selinux_get_property_create_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_get_property_create_context_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_get_property_create_context_reply_t,
    >,
    pub(crate) xcb_selinux_get_property_create_context: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_selinux_get_property_create_context_cookie_t,
    >,
    pub(crate) xcb_selinux_get_property_create_context_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_selinux_get_property_create_context_cookie_t,
    >,
    pub(crate) xcb_selinux_set_property_use_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_len: u32,
            context: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_selinux_set_property_use_context_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_len: u32,
            context: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_selinux_get_property_use_context_context: LazySymbol<
        unsafe fn(R: *const xcb_selinux_get_property_use_context_reply_t) -> *mut c_char,
    >,
    pub(crate) xcb_selinux_get_property_use_context_context_length:
        LazySymbol<unsafe fn(R: *const xcb_selinux_get_property_use_context_reply_t) -> c_int>,
    pub(crate) xcb_selinux_get_property_use_context_context_end: LazySymbol<
        unsafe fn(R: *const xcb_selinux_get_property_use_context_reply_t) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_selinux_get_property_use_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_get_property_use_context_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_get_property_use_context_reply_t,
    >,
    pub(crate) xcb_selinux_get_property_use_context: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_selinux_get_property_use_context_cookie_t,
    >,
    pub(crate) xcb_selinux_get_property_use_context_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_selinux_get_property_use_context_cookie_t,
    >,
    pub(crate) xcb_selinux_get_property_context_context:
        LazySymbol<unsafe fn(R: *const xcb_selinux_get_property_context_reply_t) -> *mut c_char>,
    pub(crate) xcb_selinux_get_property_context_context_length:
        LazySymbol<unsafe fn(R: *const xcb_selinux_get_property_context_reply_t) -> c_int>,
    pub(crate) xcb_selinux_get_property_context_context_end: LazySymbol<
        unsafe fn(R: *const xcb_selinux_get_property_context_reply_t) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_selinux_get_property_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_get_property_context_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_get_property_context_reply_t,
    >,
    pub(crate) xcb_selinux_get_property_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            property: xcb_atom_t,
        ) -> xcb_selinux_get_property_context_cookie_t,
    >,
    pub(crate) xcb_selinux_get_property_context_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            property: xcb_atom_t,
        ) -> xcb_selinux_get_property_context_cookie_t,
    >,
    pub(crate) xcb_selinux_get_property_data_context_context: LazySymbol<
        unsafe fn(R: *const xcb_selinux_get_property_data_context_reply_t) -> *mut c_char,
    >,
    pub(crate) xcb_selinux_get_property_data_context_context_length:
        LazySymbol<unsafe fn(R: *const xcb_selinux_get_property_data_context_reply_t) -> c_int>,
    pub(crate) xcb_selinux_get_property_data_context_context_end: LazySymbol<
        unsafe fn(
            R: *const xcb_selinux_get_property_data_context_reply_t,
        ) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_selinux_get_property_data_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_get_property_data_context_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_get_property_data_context_reply_t,
    >,
    pub(crate) xcb_selinux_get_property_data_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            property: xcb_atom_t,
        ) -> xcb_selinux_get_property_data_context_cookie_t,
    >,
    pub(crate) xcb_selinux_get_property_data_context_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            property: xcb_atom_t,
        ) -> xcb_selinux_get_property_data_context_cookie_t,
    >,
    pub(crate) xcb_selinux_list_properties_properties_length:
        LazySymbol<unsafe fn(R: *const xcb_selinux_list_properties_reply_t) -> c_int>,
    pub(crate) xcb_selinux_list_properties_properties_iterator: LazySymbol<
        unsafe fn(
            R: *const xcb_selinux_list_properties_reply_t,
        ) -> xcb_selinux_list_item_iterator_t,
    >,
    pub(crate) xcb_selinux_list_properties_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_list_properties_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_list_properties_reply_t,
    >,
    pub(crate) xcb_selinux_list_properties: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_selinux_list_properties_cookie_t,
    >,
    pub(crate) xcb_selinux_list_properties_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_selinux_list_properties_cookie_t,
    >,
    pub(crate) xcb_selinux_set_selection_create_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_len: u32,
            context: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_selinux_set_selection_create_context_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_len: u32,
            context: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_selinux_get_selection_create_context_context: LazySymbol<
        unsafe fn(R: *const xcb_selinux_get_selection_create_context_reply_t) -> *mut c_char,
    >,
    pub(crate) xcb_selinux_get_selection_create_context_context_length:
        LazySymbol<unsafe fn(R: *const xcb_selinux_get_selection_create_context_reply_t) -> c_int>,
    pub(crate) xcb_selinux_get_selection_create_context_context_end: LazySymbol<
        unsafe fn(
            R: *const xcb_selinux_get_selection_create_context_reply_t,
        ) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_selinux_get_selection_create_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_get_selection_create_context_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_get_selection_create_context_reply_t,
    >,
    pub(crate) xcb_selinux_get_selection_create_context: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_selinux_get_selection_create_context_cookie_t,
    >,
    pub(crate) xcb_selinux_get_selection_create_context_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_selinux_get_selection_create_context_cookie_t,
    >,
    pub(crate) xcb_selinux_set_selection_use_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_len: u32,
            context: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_selinux_set_selection_use_context_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            context_len: u32,
            context: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_selinux_get_selection_use_context_context: LazySymbol<
        unsafe fn(R: *const xcb_selinux_get_selection_use_context_reply_t) -> *mut c_char,
    >,
    pub(crate) xcb_selinux_get_selection_use_context_context_length:
        LazySymbol<unsafe fn(R: *const xcb_selinux_get_selection_use_context_reply_t) -> c_int>,
    pub(crate) xcb_selinux_get_selection_use_context_context_end: LazySymbol<
        unsafe fn(
            R: *const xcb_selinux_get_selection_use_context_reply_t,
        ) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_selinux_get_selection_use_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_get_selection_use_context_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_get_selection_use_context_reply_t,
    >,
    pub(crate) xcb_selinux_get_selection_use_context: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_selinux_get_selection_use_context_cookie_t,
    >,
    pub(crate) xcb_selinux_get_selection_use_context_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_selinux_get_selection_use_context_cookie_t,
    >,
    pub(crate) xcb_selinux_get_selection_context_context:
        LazySymbol<unsafe fn(R: *const xcb_selinux_get_selection_context_reply_t) -> *mut c_char>,
    pub(crate) xcb_selinux_get_selection_context_context_length:
        LazySymbol<unsafe fn(R: *const xcb_selinux_get_selection_context_reply_t) -> c_int>,
    pub(crate) xcb_selinux_get_selection_context_context_end: LazySymbol<
        unsafe fn(R: *const xcb_selinux_get_selection_context_reply_t) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_selinux_get_selection_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_get_selection_context_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_get_selection_context_reply_t,
    >,
    pub(crate) xcb_selinux_get_selection_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            selection: xcb_atom_t,
        ) -> xcb_selinux_get_selection_context_cookie_t,
    >,
    pub(crate) xcb_selinux_get_selection_context_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            selection: xcb_atom_t,
        ) -> xcb_selinux_get_selection_context_cookie_t,
    >,
    pub(crate) xcb_selinux_get_selection_data_context_context: LazySymbol<
        unsafe fn(R: *const xcb_selinux_get_selection_data_context_reply_t) -> *mut c_char,
    >,
    pub(crate) xcb_selinux_get_selection_data_context_context_length:
        LazySymbol<unsafe fn(R: *const xcb_selinux_get_selection_data_context_reply_t) -> c_int>,
    pub(crate) xcb_selinux_get_selection_data_context_context_end: LazySymbol<
        unsafe fn(
            R: *const xcb_selinux_get_selection_data_context_reply_t,
        ) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_selinux_get_selection_data_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_get_selection_data_context_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_get_selection_data_context_reply_t,
    >,
    pub(crate) xcb_selinux_get_selection_data_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            selection: xcb_atom_t,
        ) -> xcb_selinux_get_selection_data_context_cookie_t,
    >,
    pub(crate) xcb_selinux_get_selection_data_context_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            selection: xcb_atom_t,
        ) -> xcb_selinux_get_selection_data_context_cookie_t,
    >,
    pub(crate) xcb_selinux_list_selections_selections_length:
        LazySymbol<unsafe fn(R: *const xcb_selinux_list_selections_reply_t) -> c_int>,
    pub(crate) xcb_selinux_list_selections_selections_iterator: LazySymbol<
        unsafe fn(
            R: *const xcb_selinux_list_selections_reply_t,
        ) -> xcb_selinux_list_item_iterator_t,
    >,
    pub(crate) xcb_selinux_list_selections_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_list_selections_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_list_selections_reply_t,
    >,
    pub(crate) xcb_selinux_list_selections:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_selinux_list_selections_cookie_t>,
    pub(crate) xcb_selinux_list_selections_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_selinux_list_selections_cookie_t>,
    pub(crate) xcb_selinux_get_client_context_context:
        LazySymbol<unsafe fn(R: *const xcb_selinux_get_client_context_reply_t) -> *mut c_char>,
    pub(crate) xcb_selinux_get_client_context_context_length:
        LazySymbol<unsafe fn(R: *const xcb_selinux_get_client_context_reply_t) -> c_int>,
    pub(crate) xcb_selinux_get_client_context_context_end: LazySymbol<
        unsafe fn(R: *const xcb_selinux_get_client_context_reply_t) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_selinux_get_client_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_selinux_get_client_context_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_selinux_get_client_context_reply_t,
    >,
    pub(crate) xcb_selinux_get_client_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            resource: u32,
        ) -> xcb_selinux_get_client_context_cookie_t,
    >,
    pub(crate) xcb_selinux_get_client_context_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            resource: u32,
        ) -> xcb_selinux_get_client_context_cookie_t,
    >,
}
