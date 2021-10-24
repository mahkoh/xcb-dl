use crate::*;
use std::os::raw::*;

pub const XCB_DRI3_MAJOR_VERSION: u32 = 1;
pub const XCB_DRI3_MINOR_VERSION: u32 = 0;

pub const XCB_DRI3_QUERY_VERSION: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub major_version: u32,
    pub minor_version: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_query_version_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u32,
    pub minor_version: u32,
}

pub const XCB_DRI3_OPEN: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_open_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub provider: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_open_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_open_reply_t {
    pub response_type: u8,
    pub nfd: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad0: [u8; 24],
}

pub const XCB_DRI3_PIXMAP_FROM_BUFFER: u8 = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_pixmap_from_buffer_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub pixmap: xcb_pixmap_t,
    pub drawable: xcb_drawable_t,
    pub size: u32,
    pub width: u16,
    pub height: u16,
    pub stride: u16,
    pub depth: u8,
    pub bpp: u8,
}

pub const XCB_DRI3_BUFFER_FROM_PIXMAP: u8 = 3;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_buffer_from_pixmap_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub pixmap: xcb_pixmap_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_buffer_from_pixmap_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_buffer_from_pixmap_reply_t {
    pub response_type: u8,
    pub nfd: u8,
    pub sequence: u16,
    pub length: u32,
    pub size: u32,
    pub width: u16,
    pub height: u16,
    pub stride: u16,
    pub depth: u8,
    pub bpp: u8,
    pub pad0: [u8; 12],
}

pub const XCB_DRI3_FENCE_FROM_FD: u8 = 4;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_fence_from_fd_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub fence: u32,
    pub initially_triggered: u8,
    pub pad0: [u8; 3],
}

pub const XCB_DRI3_FD_FROM_FENCE: u8 = 5;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_fd_from_fence_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub fence: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_fd_from_fence_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_dri3_fd_from_fence_reply_t {
    pub response_type: u8,
    pub nfd: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad0: [u8; 24],
}

impl XcbDri3 {
    #[inline]
    pub fn xcb_dri3_id(&self) -> *mut xcb_extension_t {
        call!(self, xcb_dri3_id)
    }

    #[inline]
    pub unsafe fn xcb_dri3_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri3_query_version_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri3_query_version_reply_t {
        call!(self, xcb_dri3_query_version_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_dri3_query_version(
        &self,
        c: *mut xcb_connection_t,
        major_version: u32,
        minor_version: u32,
    ) -> xcb_dri3_query_version_cookie_t {
        call!(self, xcb_dri3_query_version)(c, major_version, minor_version)
    }

    #[inline]
    pub unsafe fn xcb_dri3_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        major_version: u32,
        minor_version: u32,
    ) -> xcb_dri3_query_version_cookie_t {
        call!(self, xcb_dri3_query_version_unchecked)(c, major_version, minor_version)
    }

    #[inline]
    pub unsafe fn xcb_dri3_open_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri3_open_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri3_open_reply_t {
        call!(self, xcb_dri3_open_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_dri3_open_reply_fds(
        &self,
        c: *mut xcb_connection_t,
        reply: *mut xcb_dri3_open_reply_t,
    ) -> *mut c_int {
        call!(self, xcb_dri3_open_reply_fds)(c, reply)
    }

    #[inline]
    pub unsafe fn xcb_dri3_open(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        provider: u32,
    ) -> xcb_dri3_open_cookie_t {
        call!(self, xcb_dri3_open)(c, drawable, provider)
    }

    #[inline]
    pub unsafe fn xcb_dri3_open_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        provider: u32,
    ) -> xcb_dri3_open_cookie_t {
        call!(self, xcb_dri3_open_unchecked)(c, drawable, provider)
    }

    #[inline]
    pub unsafe fn xcb_dri3_pixmap_from_buffer(
        &self,
        c: *mut xcb_connection_t,
        pixmap: xcb_pixmap_t,
        drawable: xcb_drawable_t,
        size: u32,
        width: u16,
        height: u16,
        stride: u16,
        depth: u8,
        bpp: u8,
        pixmap_fd: i32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_dri3_pixmap_from_buffer)(
            c, pixmap, drawable, size, width, height, stride, depth, bpp, pixmap_fd,
        )
    }

    #[inline]
    pub unsafe fn xcb_dri3_pixmap_from_buffer_checked(
        &self,
        c: *mut xcb_connection_t,
        pixmap: xcb_pixmap_t,
        drawable: xcb_drawable_t,
        size: u32,
        width: u16,
        height: u16,
        stride: u16,
        depth: u8,
        bpp: u8,
        pixmap_fd: i32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_dri3_pixmap_from_buffer_checked)(
            c, pixmap, drawable, size, width, height, stride, depth, bpp, pixmap_fd,
        )
    }

    #[inline]
    pub unsafe fn xcb_dri3_buffer_from_pixmap_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri3_buffer_from_pixmap_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri3_buffer_from_pixmap_reply_t {
        call!(self, xcb_dri3_buffer_from_pixmap_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_dri3_buffer_from_pixmap_reply_fds(
        &self,
        c: *mut xcb_connection_t,
        reply: *mut xcb_dri3_buffer_from_pixmap_reply_t,
    ) -> *mut c_int {
        call!(self, xcb_dri3_buffer_from_pixmap_reply_fds)(c, reply)
    }

    #[inline]
    pub unsafe fn xcb_dri3_buffer_from_pixmap(
        &self,
        c: *mut xcb_connection_t,
        pixmap: xcb_pixmap_t,
    ) -> xcb_dri3_buffer_from_pixmap_cookie_t {
        call!(self, xcb_dri3_buffer_from_pixmap)(c, pixmap)
    }

    #[inline]
    pub unsafe fn xcb_dri3_buffer_from_pixmap_unchecked(
        &self,
        c: *mut xcb_connection_t,
        pixmap: xcb_pixmap_t,
    ) -> xcb_dri3_buffer_from_pixmap_cookie_t {
        call!(self, xcb_dri3_buffer_from_pixmap_unchecked)(c, pixmap)
    }

    #[inline]
    pub unsafe fn xcb_dri3_fence_from_fd(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        fence: u32,
        initially_triggered: u8,
        fence_fd: i32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_dri3_fence_from_fd)(c, drawable, fence, initially_triggered, fence_fd)
    }

    #[inline]
    pub unsafe fn xcb_dri3_fence_from_fd_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        fence: u32,
        initially_triggered: u8,
        fence_fd: i32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_dri3_fence_from_fd_checked)(
            c,
            drawable,
            fence,
            initially_triggered,
            fence_fd,
        )
    }

    #[inline]
    pub unsafe fn xcb_dri3_fd_from_fence_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_dri3_fd_from_fence_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_dri3_fd_from_fence_reply_t {
        call!(self, xcb_dri3_fd_from_fence_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_dri3_fd_from_fence_reply_fds(
        &self,
        c: *mut xcb_connection_t,
        reply: *mut xcb_dri3_fd_from_fence_reply_t,
    ) -> *mut c_int {
        call!(self, xcb_dri3_fd_from_fence_reply_fds)(c, reply)
    }

    #[inline]
    pub unsafe fn xcb_dri3_fd_from_fence(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        fence: u32,
    ) -> xcb_dri3_fd_from_fence_cookie_t {
        call!(self, xcb_dri3_fd_from_fence)(c, drawable, fence)
    }

    #[inline]
    pub unsafe fn xcb_dri3_fd_from_fence_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        fence: u32,
    ) -> xcb_dri3_fd_from_fence_cookie_t {
        call!(self, xcb_dri3_fd_from_fence_unchecked)(c, drawable, fence)
    }
}

pub struct XcbDri3 {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb_dri3_id: LazySymbol<*mut xcb_extension_t>,
    pub(crate) xcb_dri3_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dri3_query_version_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_dri3_query_version_reply_t,
    >,
    pub(crate) xcb_dri3_query_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major_version: u32,
            minor_version: u32,
        ) -> xcb_dri3_query_version_cookie_t,
    >,
    pub(crate) xcb_dri3_query_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major_version: u32,
            minor_version: u32,
        ) -> xcb_dri3_query_version_cookie_t,
    >,
    pub(crate) xcb_dri3_open_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dri3_open_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_dri3_open_reply_t,
    >,
    pub(crate) xcb_dri3_open_reply_fds: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, reply: *mut xcb_dri3_open_reply_t) -> *mut c_int,
    >,
    pub(crate) xcb_dri3_open: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            provider: u32,
        ) -> xcb_dri3_open_cookie_t,
    >,
    pub(crate) xcb_dri3_open_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            provider: u32,
        ) -> xcb_dri3_open_cookie_t,
    >,
    pub(crate) xcb_dri3_pixmap_from_buffer: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            pixmap: xcb_pixmap_t,
            drawable: xcb_drawable_t,
            size: u32,
            width: u16,
            height: u16,
            stride: u16,
            depth: u8,
            bpp: u8,
            pixmap_fd: i32,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_dri3_pixmap_from_buffer_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            pixmap: xcb_pixmap_t,
            drawable: xcb_drawable_t,
            size: u32,
            width: u16,
            height: u16,
            stride: u16,
            depth: u8,
            bpp: u8,
            pixmap_fd: i32,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_dri3_buffer_from_pixmap_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dri3_buffer_from_pixmap_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_dri3_buffer_from_pixmap_reply_t,
    >,
    pub(crate) xcb_dri3_buffer_from_pixmap_reply_fds: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            reply: *mut xcb_dri3_buffer_from_pixmap_reply_t,
        ) -> *mut c_int,
    >,
    pub(crate) xcb_dri3_buffer_from_pixmap: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            pixmap: xcb_pixmap_t,
        ) -> xcb_dri3_buffer_from_pixmap_cookie_t,
    >,
    pub(crate) xcb_dri3_buffer_from_pixmap_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            pixmap: xcb_pixmap_t,
        ) -> xcb_dri3_buffer_from_pixmap_cookie_t,
    >,
    pub(crate) xcb_dri3_fence_from_fd: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            fence: u32,
            initially_triggered: u8,
            fence_fd: i32,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_dri3_fence_from_fd_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            fence: u32,
            initially_triggered: u8,
            fence_fd: i32,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_dri3_fd_from_fence_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_dri3_fd_from_fence_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_dri3_fd_from_fence_reply_t,
    >,
    pub(crate) xcb_dri3_fd_from_fence_reply_fds: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            reply: *mut xcb_dri3_fd_from_fence_reply_t,
        ) -> *mut c_int,
    >,
    pub(crate) xcb_dri3_fd_from_fence: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            fence: u32,
        ) -> xcb_dri3_fd_from_fence_cookie_t,
    >,
    pub(crate) xcb_dri3_fd_from_fence_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            fence: u32,
        ) -> xcb_dri3_fd_from_fence_cookie_t,
    >,
}
