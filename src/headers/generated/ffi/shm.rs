use crate::*;
use std::os::raw::*;

pub const XCB_SHM_MAJOR_VERSION: u32 = 1;
pub const XCB_SHM_MINOR_VERSION: u32 = 2;

pub type xcb_shm_seg_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shm_seg_iterator_t {
    pub data: *mut xcb_shm_seg_t,
    pub rem: c_int,
    pub index: c_int,
}

pub const XCB_SHM_COMPLETION: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shm_completion_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub drawable: xcb_drawable_t,
    pub minor_event: u16,
    pub major_event: u8,
    pub pad1: u8,
    pub shmseg: xcb_shm_seg_t,
    pub offset: u32,
}

pub const XCB_SHM_BAD_SEG: u8 = 0;

pub type xcb_shm_bad_seg_error_t = xcb_value_error_t;

pub const XCB_SHM_QUERY_VERSION: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shm_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shm_query_version_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shm_query_version_reply_t {
    pub response_type: u8,
    pub shared_pixmaps: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub uid: u16,
    pub gid: u16,
    pub pixmap_format: u8,
    pub pad0: [u8; 15],
}

pub const XCB_SHM_ATTACH: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shm_attach_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub shmseg: xcb_shm_seg_t,
    pub shmid: u32,
    pub read_only: u8,
    pub pad0: [u8; 3],
}

pub const XCB_SHM_DETACH: u8 = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shm_detach_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub shmseg: xcb_shm_seg_t,
}

pub const XCB_SHM_PUT_IMAGE: u8 = 3;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shm_put_image_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub total_width: u16,
    pub total_height: u16,
    pub src_x: u16,
    pub src_y: u16,
    pub src_width: u16,
    pub src_height: u16,
    pub dst_x: i16,
    pub dst_y: i16,
    pub depth: u8,
    pub format: u8,
    pub send_event: u8,
    pub pad0: u8,
    pub shmseg: xcb_shm_seg_t,
    pub offset: u32,
}

pub const XCB_SHM_GET_IMAGE: u8 = 4;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shm_get_image_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub plane_mask: u32,
    pub format: u8,
    pub pad0: [u8; 3],
    pub shmseg: xcb_shm_seg_t,
    pub offset: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shm_get_image_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shm_get_image_reply_t {
    pub response_type: u8,
    pub depth: u8,
    pub sequence: u16,
    pub length: u32,
    pub visual: xcb_visualid_t,
    pub size: u32,
}

pub const XCB_SHM_CREATE_PIXMAP: u8 = 5;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shm_create_pixmap_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub pid: xcb_pixmap_t,
    pub drawable: xcb_drawable_t,
    pub width: u16,
    pub height: u16,
    pub depth: u8,
    pub pad0: [u8; 3],
    pub shmseg: xcb_shm_seg_t,
    pub offset: u32,
}

pub const XCB_SHM_ATTACH_FD: u8 = 6;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shm_attach_fd_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub shmseg: xcb_shm_seg_t,
    pub read_only: u8,
    pub pad0: [u8; 3],
}

pub const XCB_SHM_CREATE_SEGMENT: u8 = 7;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shm_create_segment_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub shmseg: xcb_shm_seg_t,
    pub size: u32,
    pub read_only: u8,
    pub pad0: [u8; 3],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shm_create_segment_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shm_create_segment_reply_t {
    pub response_type: u8,
    pub nfd: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad0: [u8; 24],
}

impl XcbShm {
    #[inline]
    pub fn xcb_shm_id(&self) -> *mut xcb_extension_t {
        call!(self, xcb_shm_id)
    }

    #[inline]
    pub unsafe fn xcb_shm_seg_next(&self, i: *mut xcb_shm_seg_iterator_t) {
        call!(self, xcb_shm_seg_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_shm_seg_end(&self, i: *mut xcb_shm_seg_iterator_t) -> xcb_generic_iterator_t {
        call!(self, xcb_shm_seg_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_shm_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_shm_query_version_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_shm_query_version_reply_t {
        call!(self, xcb_shm_query_version_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_shm_query_version(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_shm_query_version_cookie_t {
        call!(self, xcb_shm_query_version)(c)
    }

    #[inline]
    pub unsafe fn xcb_shm_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_shm_query_version_cookie_t {
        call!(self, xcb_shm_query_version_unchecked)(c)
    }

    #[inline]
    pub unsafe fn xcb_shm_attach(
        &self,
        c: *mut xcb_connection_t,
        shmseg: xcb_shm_seg_t,
        shmid: u32,
        read_only: u8,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_shm_attach)(c, shmseg, shmid, read_only)
    }

    #[inline]
    pub unsafe fn xcb_shm_attach_checked(
        &self,
        c: *mut xcb_connection_t,
        shmseg: xcb_shm_seg_t,
        shmid: u32,
        read_only: u8,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_shm_attach_checked)(c, shmseg, shmid, read_only)
    }

    #[inline]
    pub unsafe fn xcb_shm_detach(
        &self,
        c: *mut xcb_connection_t,
        shmseg: xcb_shm_seg_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_shm_detach)(c, shmseg)
    }

    #[inline]
    pub unsafe fn xcb_shm_detach_checked(
        &self,
        c: *mut xcb_connection_t,
        shmseg: xcb_shm_seg_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_shm_detach_checked)(c, shmseg)
    }

    #[inline]
    pub unsafe fn xcb_shm_put_image(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        total_width: u16,
        total_height: u16,
        src_x: u16,
        src_y: u16,
        src_width: u16,
        src_height: u16,
        dst_x: i16,
        dst_y: i16,
        depth: u8,
        format: u8,
        send_event: u8,
        shmseg: xcb_shm_seg_t,
        offset: u32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_shm_put_image)(
            c,
            drawable,
            gc,
            total_width,
            total_height,
            src_x,
            src_y,
            src_width,
            src_height,
            dst_x,
            dst_y,
            depth,
            format,
            send_event,
            shmseg,
            offset,
        )
    }

    #[inline]
    pub unsafe fn xcb_shm_put_image_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        total_width: u16,
        total_height: u16,
        src_x: u16,
        src_y: u16,
        src_width: u16,
        src_height: u16,
        dst_x: i16,
        dst_y: i16,
        depth: u8,
        format: u8,
        send_event: u8,
        shmseg: xcb_shm_seg_t,
        offset: u32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_shm_put_image_checked)(
            c,
            drawable,
            gc,
            total_width,
            total_height,
            src_x,
            src_y,
            src_width,
            src_height,
            dst_x,
            dst_y,
            depth,
            format,
            send_event,
            shmseg,
            offset,
        )
    }

    #[inline]
    pub unsafe fn xcb_shm_get_image_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_shm_get_image_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_shm_get_image_reply_t {
        call!(self, xcb_shm_get_image_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_shm_get_image(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        plane_mask: u32,
        format: u8,
        shmseg: xcb_shm_seg_t,
        offset: u32,
    ) -> xcb_shm_get_image_cookie_t {
        call!(self, xcb_shm_get_image)(
            c, drawable, x, y, width, height, plane_mask, format, shmseg, offset,
        )
    }

    #[inline]
    pub unsafe fn xcb_shm_get_image_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        plane_mask: u32,
        format: u8,
        shmseg: xcb_shm_seg_t,
        offset: u32,
    ) -> xcb_shm_get_image_cookie_t {
        call!(self, xcb_shm_get_image_unchecked)(
            c, drawable, x, y, width, height, plane_mask, format, shmseg, offset,
        )
    }

    #[inline]
    pub unsafe fn xcb_shm_create_pixmap(
        &self,
        c: *mut xcb_connection_t,
        pid: xcb_pixmap_t,
        drawable: xcb_drawable_t,
        width: u16,
        height: u16,
        depth: u8,
        shmseg: xcb_shm_seg_t,
        offset: u32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_shm_create_pixmap)(c, pid, drawable, width, height, depth, shmseg, offset)
    }

    #[inline]
    pub unsafe fn xcb_shm_create_pixmap_checked(
        &self,
        c: *mut xcb_connection_t,
        pid: xcb_pixmap_t,
        drawable: xcb_drawable_t,
        width: u16,
        height: u16,
        depth: u8,
        shmseg: xcb_shm_seg_t,
        offset: u32,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_shm_create_pixmap_checked)(
            c, pid, drawable, width, height, depth, shmseg, offset,
        )
    }

    #[inline]
    pub unsafe fn xcb_shm_attach_fd(
        &self,
        c: *mut xcb_connection_t,
        shmseg: xcb_shm_seg_t,
        shm_fd: i32,
        read_only: u8,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_shm_attach_fd)(c, shmseg, shm_fd, read_only)
    }

    #[inline]
    pub unsafe fn xcb_shm_attach_fd_checked(
        &self,
        c: *mut xcb_connection_t,
        shmseg: xcb_shm_seg_t,
        shm_fd: i32,
        read_only: u8,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_shm_attach_fd_checked)(c, shmseg, shm_fd, read_only)
    }

    #[inline]
    pub unsafe fn xcb_shm_create_segment_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_shm_create_segment_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_shm_create_segment_reply_t {
        call!(self, xcb_shm_create_segment_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_shm_create_segment_reply_fds(
        &self,
        c: *mut xcb_connection_t,
        reply: *mut xcb_shm_create_segment_reply_t,
    ) -> *mut c_int {
        call!(self, xcb_shm_create_segment_reply_fds)(c, reply)
    }

    #[inline]
    pub unsafe fn xcb_shm_create_segment(
        &self,
        c: *mut xcb_connection_t,
        shmseg: xcb_shm_seg_t,
        size: u32,
        read_only: u8,
    ) -> xcb_shm_create_segment_cookie_t {
        call!(self, xcb_shm_create_segment)(c, shmseg, size, read_only)
    }

    #[inline]
    pub unsafe fn xcb_shm_create_segment_unchecked(
        &self,
        c: *mut xcb_connection_t,
        shmseg: xcb_shm_seg_t,
        size: u32,
        read_only: u8,
    ) -> xcb_shm_create_segment_cookie_t {
        call!(self, xcb_shm_create_segment_unchecked)(c, shmseg, size, read_only)
    }
}

pub struct XcbShm {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb_shm_id: LazySymbol<*mut xcb_extension_t>,
    pub(crate) xcb_shm_seg_next: LazySymbol<unsafe fn(i: *mut xcb_shm_seg_iterator_t)>,
    pub(crate) xcb_shm_seg_end:
        LazySymbol<unsafe fn(i: *mut xcb_shm_seg_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_shm_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_shm_query_version_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_shm_query_version_reply_t,
    >,
    pub(crate) xcb_shm_query_version:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_shm_query_version_cookie_t>,
    pub(crate) xcb_shm_query_version_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_shm_query_version_cookie_t>,
    pub(crate) xcb_shm_attach: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            shmseg: xcb_shm_seg_t,
            shmid: u32,
            read_only: u8,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_shm_attach_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            shmseg: xcb_shm_seg_t,
            shmid: u32,
            read_only: u8,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_shm_detach:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, shmseg: xcb_shm_seg_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_shm_detach_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, shmseg: xcb_shm_seg_t) -> xcb_void_cookie_t>,
    pub(crate) xcb_shm_put_image: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            total_width: u16,
            total_height: u16,
            src_x: u16,
            src_y: u16,
            src_width: u16,
            src_height: u16,
            dst_x: i16,
            dst_y: i16,
            depth: u8,
            format: u8,
            send_event: u8,
            shmseg: xcb_shm_seg_t,
            offset: u32,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_shm_put_image_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            total_width: u16,
            total_height: u16,
            src_x: u16,
            src_y: u16,
            src_width: u16,
            src_height: u16,
            dst_x: i16,
            dst_y: i16,
            depth: u8,
            format: u8,
            send_event: u8,
            shmseg: xcb_shm_seg_t,
            offset: u32,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_shm_get_image_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_shm_get_image_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_shm_get_image_reply_t,
    >,
    pub(crate) xcb_shm_get_image: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            x: i16,
            y: i16,
            width: u16,
            height: u16,
            plane_mask: u32,
            format: u8,
            shmseg: xcb_shm_seg_t,
            offset: u32,
        ) -> xcb_shm_get_image_cookie_t,
    >,
    pub(crate) xcb_shm_get_image_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            x: i16,
            y: i16,
            width: u16,
            height: u16,
            plane_mask: u32,
            format: u8,
            shmseg: xcb_shm_seg_t,
            offset: u32,
        ) -> xcb_shm_get_image_cookie_t,
    >,
    pub(crate) xcb_shm_create_pixmap: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            pid: xcb_pixmap_t,
            drawable: xcb_drawable_t,
            width: u16,
            height: u16,
            depth: u8,
            shmseg: xcb_shm_seg_t,
            offset: u32,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_shm_create_pixmap_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            pid: xcb_pixmap_t,
            drawable: xcb_drawable_t,
            width: u16,
            height: u16,
            depth: u8,
            shmseg: xcb_shm_seg_t,
            offset: u32,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_shm_attach_fd: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            shmseg: xcb_shm_seg_t,
            shm_fd: i32,
            read_only: u8,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_shm_attach_fd_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            shmseg: xcb_shm_seg_t,
            shm_fd: i32,
            read_only: u8,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_shm_create_segment_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_shm_create_segment_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_shm_create_segment_reply_t,
    >,
    pub(crate) xcb_shm_create_segment_reply_fds: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            reply: *mut xcb_shm_create_segment_reply_t,
        ) -> *mut c_int,
    >,
    pub(crate) xcb_shm_create_segment: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            shmseg: xcb_shm_seg_t,
            size: u32,
            read_only: u8,
        ) -> xcb_shm_create_segment_cookie_t,
    >,
    pub(crate) xcb_shm_create_segment_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            shmseg: xcb_shm_seg_t,
            size: u32,
            read_only: u8,
        ) -> xcb_shm_create_segment_cookie_t,
    >,
}
