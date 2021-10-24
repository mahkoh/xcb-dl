use crate::ffi::*;
use crate::*;
use std::os::raw::*;

pub const XCB_COMPOSITE_MAJOR_VERSION: u32 = 0;
pub const XCB_COMPOSITE_MINOR_VERSION: u32 = 4;

pub type xcb_composite_redirect_t = u32;
pub const XCB_COMPOSITE_REDIRECT_AUTOMATIC: xcb_composite_redirect_t = 0x00;
pub const XCB_COMPOSITE_REDIRECT_MANUAL: xcb_composite_redirect_t = 0x01;

pub const XCB_COMPOSITE_QUERY_VERSION: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_composite_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub client_major_version: u32,
    pub client_minor_version: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_composite_query_version_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_composite_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u32,
    pub minor_version: u32,
    pub pad1: [u8; 16],
}

pub const XCB_COMPOSITE_REDIRECT_WINDOW: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_composite_redirect_window_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub update: u8,
    pub pad0: [u8; 3],
}

pub const XCB_COMPOSITE_REDIRECT_SUBWINDOWS: u8 = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_composite_redirect_subwindows_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub update: u8,
    pub pad0: [u8; 3],
}

pub const XCB_COMPOSITE_UNREDIRECT_WINDOW: u8 = 3;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_composite_unredirect_window_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub update: u8,
    pub pad0: [u8; 3],
}

pub const XCB_COMPOSITE_UNREDIRECT_SUBWINDOWS: u8 = 4;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_composite_unredirect_subwindows_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub update: u8,
    pub pad0: [u8; 3],
}

pub const XCB_COMPOSITE_CREATE_REGION_FROM_BORDER_CLIP: u8 = 5;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_composite_create_region_from_border_clip_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub region: xcb_xfixes_region_t,
    pub window: xcb_window_t,
}

pub const XCB_COMPOSITE_NAME_WINDOW_PIXMAP: u8 = 6;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_composite_name_window_pixmap_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub pixmap: xcb_pixmap_t,
}

pub const XCB_COMPOSITE_GET_OVERLAY_WINDOW: u8 = 7;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_composite_get_overlay_window_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_composite_get_overlay_window_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_composite_get_overlay_window_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub overlay_win: xcb_window_t,
    pub pad1: [u8; 20],
}

pub const XCB_COMPOSITE_RELEASE_OVERLAY_WINDOW: u8 = 8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_composite_release_overlay_window_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl XcbComposite {
    #[inline]
    pub unsafe fn xcb_composite_id(&self) -> *mut xcb_extension_t {
        sym!(self, xcb_composite_id)
    }

    #[inline]
    pub unsafe fn xcb_composite_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_composite_query_version_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_composite_query_version_reply_t {
        sym!(self, xcb_composite_query_version_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_composite_query_version(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u32,
        client_minor_version: u32,
    ) -> xcb_composite_query_version_cookie_t {
        sym!(self, xcb_composite_query_version)(c, client_major_version, client_minor_version)
    }

    #[inline]
    pub unsafe fn xcb_composite_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u32,
        client_minor_version: u32,
    ) -> xcb_composite_query_version_cookie_t {
        sym!(self, xcb_composite_query_version_unchecked)(
            c,
            client_major_version,
            client_minor_version,
        )
    }

    #[inline]
    pub unsafe fn xcb_composite_redirect_window(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        update: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_composite_redirect_window)(c, window, update)
    }

    #[inline]
    pub unsafe fn xcb_composite_redirect_window_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        update: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_composite_redirect_window_checked)(c, window, update)
    }

    #[inline]
    pub unsafe fn xcb_composite_redirect_subwindows(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        update: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_composite_redirect_subwindows)(c, window, update)
    }

    #[inline]
    pub unsafe fn xcb_composite_redirect_subwindows_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        update: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_composite_redirect_subwindows_checked)(c, window, update)
    }

    #[inline]
    pub unsafe fn xcb_composite_unredirect_window(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        update: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_composite_unredirect_window)(c, window, update)
    }

    #[inline]
    pub unsafe fn xcb_composite_unredirect_window_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        update: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_composite_unredirect_window_checked)(c, window, update)
    }

    #[inline]
    pub unsafe fn xcb_composite_unredirect_subwindows(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        update: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_composite_unredirect_subwindows)(c, window, update)
    }

    #[inline]
    pub unsafe fn xcb_composite_unredirect_subwindows_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        update: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_composite_unredirect_subwindows_checked)(c, window, update)
    }

    #[inline]
    pub unsafe fn xcb_composite_create_region_from_border_clip(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_composite_create_region_from_border_clip)(c, region, window)
    }

    #[inline]
    pub unsafe fn xcb_composite_create_region_from_border_clip_checked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_composite_create_region_from_border_clip_checked)(c, region, window)
    }

    #[inline]
    pub unsafe fn xcb_composite_name_window_pixmap(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        pixmap: xcb_pixmap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_composite_name_window_pixmap)(c, window, pixmap)
    }

    #[inline]
    pub unsafe fn xcb_composite_name_window_pixmap_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        pixmap: xcb_pixmap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_composite_name_window_pixmap_checked)(c, window, pixmap)
    }

    #[inline]
    pub unsafe fn xcb_composite_get_overlay_window_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_composite_get_overlay_window_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_composite_get_overlay_window_reply_t {
        sym!(self, xcb_composite_get_overlay_window_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_composite_get_overlay_window(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_composite_get_overlay_window_cookie_t {
        sym!(self, xcb_composite_get_overlay_window)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_composite_get_overlay_window_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_composite_get_overlay_window_cookie_t {
        sym!(self, xcb_composite_get_overlay_window_unchecked)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_composite_release_overlay_window(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_composite_release_overlay_window)(c, window)
    }

    #[inline]
    pub unsafe fn xcb_composite_release_overlay_window_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_composite_release_overlay_window_checked)(c, window)
    }
}
