use crate::ffi::*;
use crate::*;
use std::os::raw::*;

pub const XCB_DRI2_MAJOR_VERSION: u32 = 1;
pub const XCB_DRI2_MINOR_VERSION: u32 = 4;

pub type xcb_dri2_attachment_t = u32;
pub const XCB_DRI2_ATTACHMENT_BUFFER_FRONT_LEFT: xcb_dri2_attachment_t = 0x00;
pub const XCB_DRI2_ATTACHMENT_BUFFER_BACK_LEFT: xcb_dri2_attachment_t = 0x01;
pub const XCB_DRI2_ATTACHMENT_BUFFER_FRONT_RIGHT: xcb_dri2_attachment_t = 0x02;
pub const XCB_DRI2_ATTACHMENT_BUFFER_BACK_RIGHT: xcb_dri2_attachment_t = 0x03;
pub const XCB_DRI2_ATTACHMENT_BUFFER_DEPTH: xcb_dri2_attachment_t = 0x04;
pub const XCB_DRI2_ATTACHMENT_BUFFER_STENCIL: xcb_dri2_attachment_t = 0x05;
pub const XCB_DRI2_ATTACHMENT_BUFFER_ACCUM: xcb_dri2_attachment_t = 0x06;
pub const XCB_DRI2_ATTACHMENT_BUFFER_FAKE_FRONT_LEFT: xcb_dri2_attachment_t = 0x07;
pub const XCB_DRI2_ATTACHMENT_BUFFER_FAKE_FRONT_RIGHT: xcb_dri2_attachment_t = 0x08;
pub const XCB_DRI2_ATTACHMENT_BUFFER_DEPTH_STENCIL: xcb_dri2_attachment_t = 0x09;
pub const XCB_DRI2_ATTACHMENT_BUFFER_HIZ: xcb_dri2_attachment_t = 0x0a;

pub type xcb_dri2_driver_type_t = u32;
pub const XCB_DRI2_DRIVER_TYPE_DRI: xcb_dri2_driver_type_t = 0x00;
pub const XCB_DRI2_DRIVER_TYPE_VDPAU: xcb_dri2_driver_type_t = 0x01;

pub type xcb_dri2_event_type_t = u32;
pub const XCB_DRI2_EVENT_TYPE_EXCHANGE_COMPLETE: xcb_dri2_event_type_t = 0x01;
pub const XCB_DRI2_EVENT_TYPE_BLIT_COMPLETE: xcb_dri2_event_type_t = 0x02;
pub const XCB_DRI2_EVENT_TYPE_FLIP_COMPLETE: xcb_dri2_event_type_t = 0x03;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_dri2_buffer_t {
    pub attachment: u32,
    pub name: u32,
    pub pitch: u32,
    pub cpp: u32,
    pub flags: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_dri2_buffer_iterator_t {
    pub data: *mut xcb_dri2_dri2_buffer_t,
    pub rem: c_int,
    pub index: c_int,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_attach_format_t {
    pub attachment: u32,
    pub format: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_attach_format_iterator_t {
    pub data: *mut xcb_dri2_attach_format_t,
    pub rem: c_int,
    pub index: c_int,
}

pub const XCB_DRI2_QUERY_VERSION: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub major_version: u32,
    pub minor_version: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_query_version_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u32,
    pub minor_version: u32,
}

pub const XCB_DRI2_CONNECT: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_connect_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub driver_type: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_connect_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_connect_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub driver_name_length: u32,
    pub device_name_length: u32,
    pub pad1: [u8; 16],
}

pub const XCB_DRI2_AUTHENTICATE: u8 = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_authenticate_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub magic: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_authenticate_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_authenticate_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub authenticated: u32,
}

pub const XCB_DRI2_CREATE_DRAWABLE: u8 = 3;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_create_drawable_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
}

pub const XCB_DRI2_DESTROY_DRAWABLE: u8 = 4;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_destroy_drawable_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
}

pub const XCB_DRI2_GET_BUFFERS: u8 = 5;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_get_buffers_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub count: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_get_buffers_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_get_buffers_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: u32,
    pub height: u32,
    pub count: u32,
    pub pad1: [u8; 12],
}

pub const XCB_DRI2_COPY_REGION: u8 = 6;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_copy_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub region: u32,
    pub dest: u32,
    pub src: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_copy_region_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_copy_region_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
}

pub const XCB_DRI2_GET_BUFFERS_WITH_FORMAT: u8 = 7;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_get_buffers_with_format_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub count: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_get_buffers_with_format_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_get_buffers_with_format_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: u32,
    pub height: u32,
    pub count: u32,
    pub pad1: [u8; 12],
}

pub const XCB_DRI2_SWAP_BUFFERS: u8 = 8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_swap_buffers_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub target_msc_hi: u32,
    pub target_msc_lo: u32,
    pub divisor_hi: u32,
    pub divisor_lo: u32,
    pub remainder_hi: u32,
    pub remainder_lo: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_swap_buffers_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_swap_buffers_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub swap_hi: u32,
    pub swap_lo: u32,
}

pub const XCB_DRI2_GET_MSC: u8 = 9;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_get_msc_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_get_msc_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_get_msc_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub ust_hi: u32,
    pub ust_lo: u32,
    pub msc_hi: u32,
    pub msc_lo: u32,
    pub sbc_hi: u32,
    pub sbc_lo: u32,
}

pub const XCB_DRI2_WAIT_MSC: u8 = 10;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_wait_msc_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub target_msc_hi: u32,
    pub target_msc_lo: u32,
    pub divisor_hi: u32,
    pub divisor_lo: u32,
    pub remainder_hi: u32,
    pub remainder_lo: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_wait_msc_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_wait_msc_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub ust_hi: u32,
    pub ust_lo: u32,
    pub msc_hi: u32,
    pub msc_lo: u32,
    pub sbc_hi: u32,
    pub sbc_lo: u32,
}

pub const XCB_DRI2_WAIT_SBC: u8 = 11;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_wait_sbc_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub target_sbc_hi: u32,
    pub target_sbc_lo: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_wait_sbc_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_wait_sbc_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub ust_hi: u32,
    pub ust_lo: u32,
    pub msc_hi: u32,
    pub msc_lo: u32,
    pub sbc_hi: u32,
    pub sbc_lo: u32,
}

pub const XCB_DRI2_SWAP_INTERVAL: u8 = 12;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_swap_interval_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub interval: u32,
}

pub const XCB_DRI2_GET_PARAM: u8 = 13;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_get_param_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub param: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_get_param_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_get_param_reply_t {
    pub response_type: u8,
    pub is_param_recognized: u8,
    pub sequence: u16,
    pub length: u32,
    pub value_hi: u32,
    pub value_lo: u32,
}

pub const XCB_DRI2_BUFFER_SWAP_COMPLETE: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_buffer_swap_complete_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event_type: u16,
    pub pad1: [u8; 2],
    pub drawable: xcb_drawable_t,
    pub ust_hi: u32,
    pub ust_lo: u32,
    pub msc_hi: u32,
    pub msc_lo: u32,
    pub sbc: u32,
}

pub const XCB_DRI2_INVALIDATE_BUFFERS: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri2_invalidate_buffers_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub drawable: xcb_drawable_t,
}

impl XcbDri2 {
    #[inline]
    pub unsafe fn xcb_dri2_id(&self) -> *mut xcb_extension_t {
        sym!(self, xcb_dri2_id)
    }

    #[inline]
    pub unsafe fn xcb_dri2_dri2_buffer_next(&self, i: *mut xcb_dri2_dri2_buffer_iterator_t) {
        sym!(self, xcb_dri2_dri2_buffer_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_dri2_dri2_buffer_end(
        &self,
        i: *mut xcb_dri2_dri2_buffer_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_dri2_dri2_buffer_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_dri2_attach_format_next(&self, i: *mut xcb_dri2_attach_format_iterator_t) {
        sym!(self, xcb_dri2_attach_format_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_dri2_attach_format_end(
        &self,
        i: *mut xcb_dri2_attach_format_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_dri2_attach_format_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_dri2_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri2_query_version_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri2_query_version_reply_t {
        sym!(self, xcb_dri2_query_version_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_dri2_query_version(
        &self,
        c: *mut xcb_connection_t,
        major_version: u32,
        minor_version: u32,
    ) -> xcb_dri2_query_version_cookie_t {
        sym!(self, xcb_dri2_query_version)(c, major_version, minor_version)
    }

    #[inline]
    pub unsafe fn xcb_dri2_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        major_version: u32,
        minor_version: u32,
    ) -> xcb_dri2_query_version_cookie_t {
        sym!(self, xcb_dri2_query_version_unchecked)(c, major_version, minor_version)
    }

    #[inline]
    pub unsafe fn xcb_dri2_connect_driver_name(
        &self,
        R: *const xcb_dri2_connect_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_dri2_connect_driver_name)(R)
    }

    #[inline]
    pub unsafe fn xcb_dri2_connect_driver_name_length(
        &self,
        R: *const xcb_dri2_connect_reply_t,
    ) -> c_int {
        sym!(self, xcb_dri2_connect_driver_name_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_dri2_connect_driver_name_end(
        &self,
        R: *const xcb_dri2_connect_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_dri2_connect_driver_name_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_dri2_connect_alignment_pad(
        &self,
        R: *const xcb_dri2_connect_reply_t,
    ) -> *mut c_void {
        sym!(self, xcb_dri2_connect_alignment_pad)(R)
    }

    #[inline]
    pub unsafe fn xcb_dri2_connect_alignment_pad_length(
        &self,
        R: *const xcb_dri2_connect_reply_t,
    ) -> c_int {
        sym!(self, xcb_dri2_connect_alignment_pad_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_dri2_connect_alignment_pad_end(
        &self,
        R: *const xcb_dri2_connect_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_dri2_connect_alignment_pad_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_dri2_connect_device_name(
        &self,
        R: *const xcb_dri2_connect_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_dri2_connect_device_name)(R)
    }

    #[inline]
    pub unsafe fn xcb_dri2_connect_device_name_length(
        &self,
        R: *const xcb_dri2_connect_reply_t,
    ) -> c_int {
        sym!(self, xcb_dri2_connect_device_name_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_dri2_connect_device_name_end(
        &self,
        R: *const xcb_dri2_connect_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_dri2_connect_device_name_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_dri2_connect_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri2_connect_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri2_connect_reply_t {
        sym!(self, xcb_dri2_connect_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_dri2_connect(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        driver_type: u32,
    ) -> xcb_dri2_connect_cookie_t {
        sym!(self, xcb_dri2_connect)(c, window, driver_type)
    }

    #[inline]
    pub unsafe fn xcb_dri2_connect_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        driver_type: u32,
    ) -> xcb_dri2_connect_cookie_t {
        sym!(self, xcb_dri2_connect_unchecked)(c, window, driver_type)
    }

    #[inline]
    pub unsafe fn xcb_dri2_authenticate_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri2_authenticate_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri2_authenticate_reply_t {
        sym!(self, xcb_dri2_authenticate_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_dri2_authenticate(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        magic: u32,
    ) -> xcb_dri2_authenticate_cookie_t {
        sym!(self, xcb_dri2_authenticate)(c, window, magic)
    }

    #[inline]
    pub unsafe fn xcb_dri2_authenticate_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        magic: u32,
    ) -> xcb_dri2_authenticate_cookie_t {
        sym!(self, xcb_dri2_authenticate_unchecked)(c, window, magic)
    }

    #[inline]
    pub unsafe fn xcb_dri2_create_drawable(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_dri2_create_drawable)(c, drawable)
    }

    #[inline]
    pub unsafe fn xcb_dri2_create_drawable_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_dri2_create_drawable_checked)(c, drawable)
    }

    #[inline]
    pub unsafe fn xcb_dri2_destroy_drawable(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_dri2_destroy_drawable)(c, drawable)
    }

    #[inline]
    pub unsafe fn xcb_dri2_destroy_drawable_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_dri2_destroy_drawable_checked)(c, drawable)
    }

    #[inline]
    pub unsafe fn xcb_dri2_get_buffers_buffers(
        &self,
        R: *const xcb_dri2_get_buffers_reply_t,
    ) -> *mut xcb_dri2_dri2_buffer_t {
        sym!(self, xcb_dri2_get_buffers_buffers)(R)
    }

    #[inline]
    pub unsafe fn xcb_dri2_get_buffers_buffers_length(
        &self,
        R: *const xcb_dri2_get_buffers_reply_t,
    ) -> c_int {
        sym!(self, xcb_dri2_get_buffers_buffers_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_dri2_get_buffers_buffers_iterator(
        &self,
        R: *const xcb_dri2_get_buffers_reply_t,
    ) -> xcb_dri2_dri2_buffer_iterator_t {
        sym!(self, xcb_dri2_get_buffers_buffers_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_dri2_get_buffers_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri2_get_buffers_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri2_get_buffers_reply_t {
        sym!(self, xcb_dri2_get_buffers_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_dri2_get_buffers(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        count: u32,
        attachments_len: u32,
        attachments: *const u32,
    ) -> xcb_dri2_get_buffers_cookie_t {
        sym!(self, xcb_dri2_get_buffers)(c, drawable, count, attachments_len, attachments)
    }

    #[inline]
    pub unsafe fn xcb_dri2_get_buffers_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        count: u32,
        attachments_len: u32,
        attachments: *const u32,
    ) -> xcb_dri2_get_buffers_cookie_t {
        sym!(self, xcb_dri2_get_buffers_unchecked)(c, drawable, count, attachments_len, attachments)
    }

    #[inline]
    pub unsafe fn xcb_dri2_copy_region_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri2_copy_region_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri2_copy_region_reply_t {
        sym!(self, xcb_dri2_copy_region_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_dri2_copy_region(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        region: u32,
        dest: u32,
        src: u32,
    ) -> xcb_dri2_copy_region_cookie_t {
        sym!(self, xcb_dri2_copy_region)(c, drawable, region, dest, src)
    }

    #[inline]
    pub unsafe fn xcb_dri2_copy_region_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        region: u32,
        dest: u32,
        src: u32,
    ) -> xcb_dri2_copy_region_cookie_t {
        sym!(self, xcb_dri2_copy_region_unchecked)(c, drawable, region, dest, src)
    }

    #[inline]
    pub unsafe fn xcb_dri2_get_buffers_with_format_buffers(
        &self,
        R: *const xcb_dri2_get_buffers_with_format_reply_t,
    ) -> *mut xcb_dri2_dri2_buffer_t {
        sym!(self, xcb_dri2_get_buffers_with_format_buffers)(R)
    }

    #[inline]
    pub unsafe fn xcb_dri2_get_buffers_with_format_buffers_length(
        &self,
        R: *const xcb_dri2_get_buffers_with_format_reply_t,
    ) -> c_int {
        sym!(self, xcb_dri2_get_buffers_with_format_buffers_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_dri2_get_buffers_with_format_buffers_iterator(
        &self,
        R: *const xcb_dri2_get_buffers_with_format_reply_t,
    ) -> xcb_dri2_dri2_buffer_iterator_t {
        sym!(self, xcb_dri2_get_buffers_with_format_buffers_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_dri2_get_buffers_with_format_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri2_get_buffers_with_format_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri2_get_buffers_with_format_reply_t {
        sym!(self, xcb_dri2_get_buffers_with_format_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_dri2_get_buffers_with_format(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        count: u32,
        attachments_len: u32,
        attachments: *const xcb_dri2_attach_format_t,
    ) -> xcb_dri2_get_buffers_with_format_cookie_t {
        sym!(self, xcb_dri2_get_buffers_with_format)(
            c,
            drawable,
            count,
            attachments_len,
            attachments,
        )
    }

    #[inline]
    pub unsafe fn xcb_dri2_get_buffers_with_format_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        count: u32,
        attachments_len: u32,
        attachments: *const xcb_dri2_attach_format_t,
    ) -> xcb_dri2_get_buffers_with_format_cookie_t {
        sym!(self, xcb_dri2_get_buffers_with_format_unchecked)(
            c,
            drawable,
            count,
            attachments_len,
            attachments,
        )
    }

    #[inline]
    pub unsafe fn xcb_dri2_swap_buffers_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri2_swap_buffers_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri2_swap_buffers_reply_t {
        sym!(self, xcb_dri2_swap_buffers_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_dri2_swap_buffers(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        target_msc_hi: u32,
        target_msc_lo: u32,
        divisor_hi: u32,
        divisor_lo: u32,
        remainder_hi: u32,
        remainder_lo: u32,
    ) -> xcb_dri2_swap_buffers_cookie_t {
        sym!(self, xcb_dri2_swap_buffers)(
            c,
            drawable,
            target_msc_hi,
            target_msc_lo,
            divisor_hi,
            divisor_lo,
            remainder_hi,
            remainder_lo,
        )
    }

    #[inline]
    pub unsafe fn xcb_dri2_swap_buffers_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        target_msc_hi: u32,
        target_msc_lo: u32,
        divisor_hi: u32,
        divisor_lo: u32,
        remainder_hi: u32,
        remainder_lo: u32,
    ) -> xcb_dri2_swap_buffers_cookie_t {
        sym!(self, xcb_dri2_swap_buffers_unchecked)(
            c,
            drawable,
            target_msc_hi,
            target_msc_lo,
            divisor_hi,
            divisor_lo,
            remainder_hi,
            remainder_lo,
        )
    }

    #[inline]
    pub unsafe fn xcb_dri2_get_msc_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri2_get_msc_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri2_get_msc_reply_t {
        sym!(self, xcb_dri2_get_msc_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_dri2_get_msc(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_dri2_get_msc_cookie_t {
        sym!(self, xcb_dri2_get_msc)(c, drawable)
    }

    #[inline]
    pub unsafe fn xcb_dri2_get_msc_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_dri2_get_msc_cookie_t {
        sym!(self, xcb_dri2_get_msc_unchecked)(c, drawable)
    }

    #[inline]
    pub unsafe fn xcb_dri2_wait_msc_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri2_wait_msc_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri2_wait_msc_reply_t {
        sym!(self, xcb_dri2_wait_msc_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_dri2_wait_msc(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        target_msc_hi: u32,
        target_msc_lo: u32,
        divisor_hi: u32,
        divisor_lo: u32,
        remainder_hi: u32,
        remainder_lo: u32,
    ) -> xcb_dri2_wait_msc_cookie_t {
        sym!(self, xcb_dri2_wait_msc)(
            c,
            drawable,
            target_msc_hi,
            target_msc_lo,
            divisor_hi,
            divisor_lo,
            remainder_hi,
            remainder_lo,
        )
    }

    #[inline]
    pub unsafe fn xcb_dri2_wait_msc_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        target_msc_hi: u32,
        target_msc_lo: u32,
        divisor_hi: u32,
        divisor_lo: u32,
        remainder_hi: u32,
        remainder_lo: u32,
    ) -> xcb_dri2_wait_msc_cookie_t {
        sym!(self, xcb_dri2_wait_msc_unchecked)(
            c,
            drawable,
            target_msc_hi,
            target_msc_lo,
            divisor_hi,
            divisor_lo,
            remainder_hi,
            remainder_lo,
        )
    }

    #[inline]
    pub unsafe fn xcb_dri2_wait_sbc_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri2_wait_sbc_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri2_wait_sbc_reply_t {
        sym!(self, xcb_dri2_wait_sbc_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_dri2_wait_sbc(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        target_sbc_hi: u32,
        target_sbc_lo: u32,
    ) -> xcb_dri2_wait_sbc_cookie_t {
        sym!(self, xcb_dri2_wait_sbc)(c, drawable, target_sbc_hi, target_sbc_lo)
    }

    #[inline]
    pub unsafe fn xcb_dri2_wait_sbc_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        target_sbc_hi: u32,
        target_sbc_lo: u32,
    ) -> xcb_dri2_wait_sbc_cookie_t {
        sym!(self, xcb_dri2_wait_sbc_unchecked)(c, drawable, target_sbc_hi, target_sbc_lo)
    }

    #[inline]
    pub unsafe fn xcb_dri2_swap_interval(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        interval: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_dri2_swap_interval)(c, drawable, interval)
    }

    #[inline]
    pub unsafe fn xcb_dri2_swap_interval_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        interval: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_dri2_swap_interval_checked)(c, drawable, interval)
    }

    #[inline]
    pub unsafe fn xcb_dri2_get_param_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri2_get_param_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri2_get_param_reply_t {
        sym!(self, xcb_dri2_get_param_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_dri2_get_param(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        param: u32,
    ) -> xcb_dri2_get_param_cookie_t {
        sym!(self, xcb_dri2_get_param)(c, drawable, param)
    }

    #[inline]
    pub unsafe fn xcb_dri2_get_param_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        param: u32,
    ) -> xcb_dri2_get_param_cookie_t {
        sym!(self, xcb_dri2_get_param_unchecked)(c, drawable, param)
    }
}
