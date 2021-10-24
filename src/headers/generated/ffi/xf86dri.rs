use crate::*;
use std::os::raw::*;

pub const XCB_XF86DRI_MAJOR_VERSION: u32 = 4;
pub const XCB_XF86DRI_MINOR_VERSION: u32 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_drm_clip_rect_t {
    pub x1: i16,
    pub y1: i16,
    pub x2: i16,
    pub x3: i16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_drm_clip_rect_iterator_t {
    pub data: *mut xcb_xf86dri_drm_clip_rect_t,
    pub rem: c_int,
    pub index: c_int,
}

pub const XCB_XF86DRI_QUERY_VERSION: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_query_version_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub dri_major_version: u16,
    pub dri_minor_version: u16,
    pub dri_minor_patch: u32,
}

pub const XCB_XF86DRI_QUERY_DIRECT_RENDERING_CAPABLE: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_query_direct_rendering_capable_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_query_direct_rendering_capable_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_query_direct_rendering_capable_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub is_capable: u8,
}

pub const XCB_XF86DRI_OPEN_CONNECTION: u8 = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_open_connection_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_open_connection_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_open_connection_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub sarea_handle_low: u32,
    pub sarea_handle_high: u32,
    pub bus_id_len: u32,
    pub pad1: [u8; 12],
}

pub const XCB_XF86DRI_CLOSE_CONNECTION: u8 = 3;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_close_connection_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
}

pub const XCB_XF86DRI_GET_CLIENT_DRIVER_NAME: u8 = 4;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_get_client_driver_name_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_get_client_driver_name_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_get_client_driver_name_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub client_driver_major_version: u32,
    pub client_driver_minor_version: u32,
    pub client_driver_patch_version: u32,
    pub client_driver_name_len: u32,
    pub pad1: [u8; 8],
}

pub const XCB_XF86DRI_CREATE_CONTEXT: u8 = 5;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_create_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
    pub visual: u32,
    pub context: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_create_context_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_create_context_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub hw_context: u32,
}

pub const XCB_XF86DRI_DESTROY_CONTEXT: u8 = 6;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_destroy_context_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
    pub context: u32,
}

pub const XCB_XF86DRI_CREATE_DRAWABLE: u8 = 7;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_create_drawable_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
    pub drawable: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_create_drawable_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_create_drawable_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub hw_drawable_handle: u32,
}

pub const XCB_XF86DRI_DESTROY_DRAWABLE: u8 = 8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_destroy_drawable_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
    pub drawable: u32,
}

pub const XCB_XF86DRI_GET_DRAWABLE_INFO: u8 = 9;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_get_drawable_info_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
    pub drawable: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_get_drawable_info_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_get_drawable_info_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub drawable_table_index: u32,
    pub drawable_table_stamp: u32,
    pub drawable_origin_x: i16,
    pub drawable_origin_y: i16,
    pub drawable_size_w: i16,
    pub drawable_size_h: i16,
    pub num_clip_rects: u32,
    pub back_x: i16,
    pub back_y: i16,
    pub num_back_clip_rects: u32,
}

pub const XCB_XF86DRI_GET_DEVICE_INFO: u8 = 10;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_get_device_info_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_get_device_info_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_get_device_info_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub framebuffer_handle_low: u32,
    pub framebuffer_handle_high: u32,
    pub framebuffer_origin_offset: u32,
    pub framebuffer_size: u32,
    pub framebuffer_stride: u32,
    pub device_private_size: u32,
}

pub const XCB_XF86DRI_AUTH_CONNECTION: u8 = 11;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_auth_connection_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub screen: u32,
    pub magic: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_auth_connection_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xf86dri_auth_connection_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub authenticated: u32,
}

impl XcbXf86dri {
    #[inline]
    pub fn xcb_xf86dri_id(&self) -> *mut xcb_extension_t {
        call!(self, xcb_xf86dri_id)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_drm_clip_rect_next(
        &self,
        i: *mut xcb_xf86dri_drm_clip_rect_iterator_t,
    ) {
        call!(self, xcb_xf86dri_drm_clip_rect_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_drm_clip_rect_end(
        &self,
        i: *mut xcb_xf86dri_drm_clip_rect_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_xf86dri_drm_clip_rect_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xf86dri_query_version_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xf86dri_query_version_reply_t {
        call!(self, xcb_xf86dri_query_version_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_query_version(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xf86dri_query_version_cookie_t {
        call!(self, xcb_xf86dri_query_version)(c)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xf86dri_query_version_cookie_t {
        call!(self, xcb_xf86dri_query_version_unchecked)(c)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_query_direct_rendering_capable_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xf86dri_query_direct_rendering_capable_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xf86dri_query_direct_rendering_capable_reply_t {
        call!(self, xcb_xf86dri_query_direct_rendering_capable_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_query_direct_rendering_capable(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_xf86dri_query_direct_rendering_capable_cookie_t {
        call!(self, xcb_xf86dri_query_direct_rendering_capable)(c, screen)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_query_direct_rendering_capable_unchecked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_xf86dri_query_direct_rendering_capable_cookie_t {
        call!(self, xcb_xf86dri_query_direct_rendering_capable_unchecked)(c, screen)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_open_connection_bus_id(
        &self,
        R: *const xcb_xf86dri_open_connection_reply_t,
    ) -> *mut c_char {
        call!(self, xcb_xf86dri_open_connection_bus_id)(R)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_open_connection_bus_id_length(
        &self,
        R: *const xcb_xf86dri_open_connection_reply_t,
    ) -> c_int {
        call!(self, xcb_xf86dri_open_connection_bus_id_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_open_connection_bus_id_end(
        &self,
        R: *const xcb_xf86dri_open_connection_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_xf86dri_open_connection_bus_id_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_open_connection_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xf86dri_open_connection_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xf86dri_open_connection_reply_t {
        call!(self, xcb_xf86dri_open_connection_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_open_connection(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_xf86dri_open_connection_cookie_t {
        call!(self, xcb_xf86dri_open_connection)(c, screen)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_open_connection_unchecked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_xf86dri_open_connection_cookie_t {
        call!(self, xcb_xf86dri_open_connection_unchecked)(c, screen)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_close_connection(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xf86dri_close_connection)(c, screen)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_close_connection_checked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xf86dri_close_connection_checked)(c, screen)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_get_client_driver_name_client_driver_name(
        &self,
        R: *const xcb_xf86dri_get_client_driver_name_reply_t,
    ) -> *mut c_char {
        call!(self, xcb_xf86dri_get_client_driver_name_client_driver_name)(R)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_get_client_driver_name_client_driver_name_length(
        &self,
        R: *const xcb_xf86dri_get_client_driver_name_reply_t,
    ) -> c_int {
        call!(
            self,
            xcb_xf86dri_get_client_driver_name_client_driver_name_length
        )(R)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_get_client_driver_name_client_driver_name_end(
        &self,
        R: *const xcb_xf86dri_get_client_driver_name_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(
            self,
            xcb_xf86dri_get_client_driver_name_client_driver_name_end
        )(R)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_get_client_driver_name_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xf86dri_get_client_driver_name_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xf86dri_get_client_driver_name_reply_t {
        call!(self, xcb_xf86dri_get_client_driver_name_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_get_client_driver_name(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_xf86dri_get_client_driver_name_cookie_t {
        call!(self, xcb_xf86dri_get_client_driver_name)(c, screen)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_get_client_driver_name_unchecked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_xf86dri_get_client_driver_name_cookie_t {
        call!(self, xcb_xf86dri_get_client_driver_name_unchecked)(c, screen)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_create_context_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xf86dri_create_context_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xf86dri_create_context_reply_t {
        call!(self, xcb_xf86dri_create_context_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_create_context(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        visual: u32,
        context: u32,
    ) -> xcb_xf86dri_create_context_cookie_t {
        call!(self, xcb_xf86dri_create_context)(c, screen, visual, context)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_create_context_unchecked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        visual: u32,
        context: u32,
    ) -> xcb_xf86dri_create_context_cookie_t {
        call!(self, xcb_xf86dri_create_context_unchecked)(c, screen, visual, context)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_destroy_context(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        context: u32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xf86dri_destroy_context)(c, screen, context)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_destroy_context_checked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        context: u32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xf86dri_destroy_context_checked)(c, screen, context)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_create_drawable_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xf86dri_create_drawable_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xf86dri_create_drawable_reply_t {
        call!(self, xcb_xf86dri_create_drawable_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_create_drawable(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        drawable: u32,
    ) -> xcb_xf86dri_create_drawable_cookie_t {
        call!(self, xcb_xf86dri_create_drawable)(c, screen, drawable)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_create_drawable_unchecked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        drawable: u32,
    ) -> xcb_xf86dri_create_drawable_cookie_t {
        call!(self, xcb_xf86dri_create_drawable_unchecked)(c, screen, drawable)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_destroy_drawable(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        drawable: u32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xf86dri_destroy_drawable)(c, screen, drawable)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_destroy_drawable_checked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        drawable: u32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_xf86dri_destroy_drawable_checked)(c, screen, drawable)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_get_drawable_info_clip_rects(
        &self,
        R: *const xcb_xf86dri_get_drawable_info_reply_t,
    ) -> *mut xcb_xf86dri_drm_clip_rect_t {
        call!(self, xcb_xf86dri_get_drawable_info_clip_rects)(R)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_get_drawable_info_clip_rects_length(
        &self,
        R: *const xcb_xf86dri_get_drawable_info_reply_t,
    ) -> c_int {
        call!(self, xcb_xf86dri_get_drawable_info_clip_rects_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_get_drawable_info_clip_rects_iterator(
        &self,
        R: *const xcb_xf86dri_get_drawable_info_reply_t,
    ) -> xcb_xf86dri_drm_clip_rect_iterator_t {
        call!(self, xcb_xf86dri_get_drawable_info_clip_rects_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_get_drawable_info_back_clip_rects(
        &self,
        R: *const xcb_xf86dri_get_drawable_info_reply_t,
    ) -> *mut xcb_xf86dri_drm_clip_rect_t {
        call!(self, xcb_xf86dri_get_drawable_info_back_clip_rects)(R)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_get_drawable_info_back_clip_rects_length(
        &self,
        R: *const xcb_xf86dri_get_drawable_info_reply_t,
    ) -> c_int {
        call!(self, xcb_xf86dri_get_drawable_info_back_clip_rects_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_get_drawable_info_back_clip_rects_iterator(
        &self,
        R: *const xcb_xf86dri_get_drawable_info_reply_t,
    ) -> xcb_xf86dri_drm_clip_rect_iterator_t {
        call!(self, xcb_xf86dri_get_drawable_info_back_clip_rects_iterator)(R)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_get_drawable_info_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xf86dri_get_drawable_info_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xf86dri_get_drawable_info_reply_t {
        call!(self, xcb_xf86dri_get_drawable_info_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_get_drawable_info(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        drawable: u32,
    ) -> xcb_xf86dri_get_drawable_info_cookie_t {
        call!(self, xcb_xf86dri_get_drawable_info)(c, screen, drawable)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_get_drawable_info_unchecked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        drawable: u32,
    ) -> xcb_xf86dri_get_drawable_info_cookie_t {
        call!(self, xcb_xf86dri_get_drawable_info_unchecked)(c, screen, drawable)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_get_device_info_device_private(
        &self,
        R: *const xcb_xf86dri_get_device_info_reply_t,
    ) -> *mut u32 {
        call!(self, xcb_xf86dri_get_device_info_device_private)(R)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_get_device_info_device_private_length(
        &self,
        R: *const xcb_xf86dri_get_device_info_reply_t,
    ) -> c_int {
        call!(self, xcb_xf86dri_get_device_info_device_private_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_get_device_info_device_private_end(
        &self,
        R: *const xcb_xf86dri_get_device_info_reply_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_xf86dri_get_device_info_device_private_end)(R)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_get_device_info_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xf86dri_get_device_info_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xf86dri_get_device_info_reply_t {
        call!(self, xcb_xf86dri_get_device_info_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_get_device_info(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_xf86dri_get_device_info_cookie_t {
        call!(self, xcb_xf86dri_get_device_info)(c, screen)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_get_device_info_unchecked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
    ) -> xcb_xf86dri_get_device_info_cookie_t {
        call!(self, xcb_xf86dri_get_device_info_unchecked)(c, screen)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_auth_connection_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xf86dri_auth_connection_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xf86dri_auth_connection_reply_t {
        call!(self, xcb_xf86dri_auth_connection_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_auth_connection(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        magic: u32,
    ) -> xcb_xf86dri_auth_connection_cookie_t {
        call!(self, xcb_xf86dri_auth_connection)(c, screen, magic)
    }

    #[inline]
    pub unsafe fn xcb_xf86dri_auth_connection_unchecked(
        &self,
        c: *mut xcb_connection_t,
        screen: u32,
        magic: u32,
    ) -> xcb_xf86dri_auth_connection_cookie_t {
        call!(self, xcb_xf86dri_auth_connection_unchecked)(c, screen, magic)
    }
}

pub struct XcbXf86dri {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb_xf86dri_id: LazySymbol<*mut xcb_extension_t>,
    pub(crate) xcb_xf86dri_drm_clip_rect_next:
        LazySymbol<unsafe fn(i: *mut xcb_xf86dri_drm_clip_rect_iterator_t)>,
    pub(crate) xcb_xf86dri_drm_clip_rect_end: LazySymbol<
        unsafe fn(i: *mut xcb_xf86dri_drm_clip_rect_iterator_t) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_xf86dri_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xf86dri_query_version_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xf86dri_query_version_reply_t,
    >,
    pub(crate) xcb_xf86dri_query_version:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_xf86dri_query_version_cookie_t>,
    pub(crate) xcb_xf86dri_query_version_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_xf86dri_query_version_cookie_t>,
    pub(crate) xcb_xf86dri_query_direct_rendering_capable_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xf86dri_query_direct_rendering_capable_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xf86dri_query_direct_rendering_capable_reply_t,
    >,
    pub(crate) xcb_xf86dri_query_direct_rendering_capable: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
        ) -> xcb_xf86dri_query_direct_rendering_capable_cookie_t,
    >,
    pub(crate) xcb_xf86dri_query_direct_rendering_capable_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
        ) -> xcb_xf86dri_query_direct_rendering_capable_cookie_t,
    >,
    pub(crate) xcb_xf86dri_open_connection_bus_id:
        LazySymbol<unsafe fn(R: *const xcb_xf86dri_open_connection_reply_t) -> *mut c_char>,
    pub(crate) xcb_xf86dri_open_connection_bus_id_length:
        LazySymbol<unsafe fn(R: *const xcb_xf86dri_open_connection_reply_t) -> c_int>,
    pub(crate) xcb_xf86dri_open_connection_bus_id_end: LazySymbol<
        unsafe fn(R: *const xcb_xf86dri_open_connection_reply_t) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_xf86dri_open_connection_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xf86dri_open_connection_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xf86dri_open_connection_reply_t,
    >,
    pub(crate) xcb_xf86dri_open_connection: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, screen: u32) -> xcb_xf86dri_open_connection_cookie_t,
    >,
    pub(crate) xcb_xf86dri_open_connection_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, screen: u32) -> xcb_xf86dri_open_connection_cookie_t,
    >,
    pub(crate) xcb_xf86dri_close_connection:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, screen: u32) -> xcb_void_cookie_t>,
    pub(crate) xcb_xf86dri_close_connection_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, screen: u32) -> xcb_void_cookie_t>,
    pub(crate) xcb_xf86dri_get_client_driver_name_client_driver_name:
        LazySymbol<unsafe fn(R: *const xcb_xf86dri_get_client_driver_name_reply_t) -> *mut c_char>,
    pub(crate) xcb_xf86dri_get_client_driver_name_client_driver_name_length:
        LazySymbol<unsafe fn(R: *const xcb_xf86dri_get_client_driver_name_reply_t) -> c_int>,
    pub(crate) xcb_xf86dri_get_client_driver_name_client_driver_name_end: LazySymbol<
        unsafe fn(R: *const xcb_xf86dri_get_client_driver_name_reply_t) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_xf86dri_get_client_driver_name_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xf86dri_get_client_driver_name_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xf86dri_get_client_driver_name_reply_t,
    >,
    pub(crate) xcb_xf86dri_get_client_driver_name: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
        ) -> xcb_xf86dri_get_client_driver_name_cookie_t,
    >,
    pub(crate) xcb_xf86dri_get_client_driver_name_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
        ) -> xcb_xf86dri_get_client_driver_name_cookie_t,
    >,
    pub(crate) xcb_xf86dri_create_context_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xf86dri_create_context_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xf86dri_create_context_reply_t,
    >,
    pub(crate) xcb_xf86dri_create_context: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
            visual: u32,
            context: u32,
        ) -> xcb_xf86dri_create_context_cookie_t,
    >,
    pub(crate) xcb_xf86dri_create_context_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
            visual: u32,
            context: u32,
        ) -> xcb_xf86dri_create_context_cookie_t,
    >,
    pub(crate) xcb_xf86dri_destroy_context: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, screen: u32, context: u32) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xf86dri_destroy_context_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, screen: u32, context: u32) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xf86dri_create_drawable_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xf86dri_create_drawable_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xf86dri_create_drawable_reply_t,
    >,
    pub(crate) xcb_xf86dri_create_drawable: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
            drawable: u32,
        ) -> xcb_xf86dri_create_drawable_cookie_t,
    >,
    pub(crate) xcb_xf86dri_create_drawable_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
            drawable: u32,
        ) -> xcb_xf86dri_create_drawable_cookie_t,
    >,
    pub(crate) xcb_xf86dri_destroy_drawable: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, screen: u32, drawable: u32) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xf86dri_destroy_drawable_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, screen: u32, drawable: u32) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_xf86dri_get_drawable_info_clip_rects: LazySymbol<
        unsafe fn(
            R: *const xcb_xf86dri_get_drawable_info_reply_t,
        ) -> *mut xcb_xf86dri_drm_clip_rect_t,
    >,
    pub(crate) xcb_xf86dri_get_drawable_info_clip_rects_length:
        LazySymbol<unsafe fn(R: *const xcb_xf86dri_get_drawable_info_reply_t) -> c_int>,
    pub(crate) xcb_xf86dri_get_drawable_info_clip_rects_iterator: LazySymbol<
        unsafe fn(
            R: *const xcb_xf86dri_get_drawable_info_reply_t,
        ) -> xcb_xf86dri_drm_clip_rect_iterator_t,
    >,
    pub(crate) xcb_xf86dri_get_drawable_info_back_clip_rects: LazySymbol<
        unsafe fn(
            R: *const xcb_xf86dri_get_drawable_info_reply_t,
        ) -> *mut xcb_xf86dri_drm_clip_rect_t,
    >,
    pub(crate) xcb_xf86dri_get_drawable_info_back_clip_rects_length:
        LazySymbol<unsafe fn(R: *const xcb_xf86dri_get_drawable_info_reply_t) -> c_int>,
    pub(crate) xcb_xf86dri_get_drawable_info_back_clip_rects_iterator: LazySymbol<
        unsafe fn(
            R: *const xcb_xf86dri_get_drawable_info_reply_t,
        ) -> xcb_xf86dri_drm_clip_rect_iterator_t,
    >,
    pub(crate) xcb_xf86dri_get_drawable_info_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xf86dri_get_drawable_info_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xf86dri_get_drawable_info_reply_t,
    >,
    pub(crate) xcb_xf86dri_get_drawable_info: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
            drawable: u32,
        ) -> xcb_xf86dri_get_drawable_info_cookie_t,
    >,
    pub(crate) xcb_xf86dri_get_drawable_info_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
            drawable: u32,
        ) -> xcb_xf86dri_get_drawable_info_cookie_t,
    >,
    pub(crate) xcb_xf86dri_get_device_info_device_private:
        LazySymbol<unsafe fn(R: *const xcb_xf86dri_get_device_info_reply_t) -> *mut u32>,
    pub(crate) xcb_xf86dri_get_device_info_device_private_length:
        LazySymbol<unsafe fn(R: *const xcb_xf86dri_get_device_info_reply_t) -> c_int>,
    pub(crate) xcb_xf86dri_get_device_info_device_private_end: LazySymbol<
        unsafe fn(R: *const xcb_xf86dri_get_device_info_reply_t) -> xcb_generic_iterator_t,
    >,
    pub(crate) xcb_xf86dri_get_device_info_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xf86dri_get_device_info_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xf86dri_get_device_info_reply_t,
    >,
    pub(crate) xcb_xf86dri_get_device_info: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, screen: u32) -> xcb_xf86dri_get_device_info_cookie_t,
    >,
    pub(crate) xcb_xf86dri_get_device_info_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, screen: u32) -> xcb_xf86dri_get_device_info_cookie_t,
    >,
    pub(crate) xcb_xf86dri_auth_connection_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xf86dri_auth_connection_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xf86dri_auth_connection_reply_t,
    >,
    pub(crate) xcb_xf86dri_auth_connection: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
            magic: u32,
        ) -> xcb_xf86dri_auth_connection_cookie_t,
    >,
    pub(crate) xcb_xf86dri_auth_connection_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            screen: u32,
            magic: u32,
        ) -> xcb_xf86dri_auth_connection_cookie_t,
    >,
}
