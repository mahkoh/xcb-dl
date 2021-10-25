// This file was generated using generate.py. Do not edit.

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

pub type xcb_shm_seg_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shm_seg_iterator_t {
    pub data: *mut xcb_shm_seg_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_shm_seg_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_shm_completion.
pub const XCB_SHM_COMPLETION: u8 = 0i32 as u8;

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

impl Default for xcb_shm_completion_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_shm_bad_seg.
pub const XCB_SHM_BAD_SEG: u8 = 0i32 as u8;

pub type xcb_shm_bad_seg_error_t = xcb_value_error_t;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shm_query_version_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_shm_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_shm_query_version.
pub const XCB_SHM_QUERY_VERSION: u8 = 0i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shm_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_shm_query_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_shm_query_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_shm_attach.
pub const XCB_SHM_ATTACH: u8 = 1i32 as u8;

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

impl Default for xcb_shm_attach_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_shm_detach.
pub const XCB_SHM_DETACH: u8 = 2i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shm_detach_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub shmseg: xcb_shm_seg_t,
}

impl Default for xcb_shm_detach_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_shm_put_image.
pub const XCB_SHM_PUT_IMAGE: u8 = 3i32 as u8;

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

impl Default for xcb_shm_put_image_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shm_get_image_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_shm_get_image_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_shm_get_image.
pub const XCB_SHM_GET_IMAGE: u8 = 4i32 as u8;

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

impl Default for xcb_shm_get_image_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_shm_get_image_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_shm_create_pixmap.
pub const XCB_SHM_CREATE_PIXMAP: u8 = 5i32 as u8;

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

impl Default for xcb_shm_create_pixmap_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_shm_attach_fd.
pub const XCB_SHM_ATTACH_FD: u8 = 6i32 as u8;

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

impl Default for xcb_shm_attach_fd_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shm_create_segment_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_shm_create_segment_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_shm_create_segment.
pub const XCB_SHM_CREATE_SEGMENT: u8 = 7i32 as u8;

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

impl Default for xcb_shm_create_segment_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
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

impl Default for xcb_shm_create_segment_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[cfg(feature = "xcb_shm")]
pub(crate) struct XcbShmShm {
    xcb_shm_id: LazySymbol<*mut xcb_extension_t>,
    xcb_shm_seg_next: LazySymbol<unsafe fn(i: *mut xcb_shm_seg_iterator_t)>,
    xcb_shm_seg_end: LazySymbol<unsafe fn(i: xcb_shm_seg_iterator_t) -> xcb_generic_iterator_t>,
    xcb_shm_query_version:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_shm_query_version_cookie_t>,
    xcb_shm_query_version_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_shm_query_version_cookie_t>,
    xcb_shm_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_shm_query_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_shm_query_version_reply_t,
    >,
    xcb_shm_attach_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            shmseg: xcb_shm_seg_t,
            shmid: u32,
            read_only: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_shm_attach: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            shmseg: xcb_shm_seg_t,
            shmid: u32,
            read_only: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_shm_detach_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, shmseg: xcb_shm_seg_t) -> xcb_void_cookie_t>,
    xcb_shm_detach:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, shmseg: xcb_shm_seg_t) -> xcb_void_cookie_t>,
    xcb_shm_put_image_checked: LazySymbol<
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
    xcb_shm_put_image: LazySymbol<
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
    xcb_shm_get_image: LazySymbol<
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
    xcb_shm_get_image_unchecked: LazySymbol<
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
    xcb_shm_get_image_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_shm_get_image_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_shm_get_image_reply_t,
    >,
    xcb_shm_create_pixmap_checked: LazySymbol<
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
    xcb_shm_create_pixmap: LazySymbol<
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
    xcb_shm_attach_fd_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            shmseg: xcb_shm_seg_t,
            shm_fd: i32,
            read_only: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_shm_attach_fd: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            shmseg: xcb_shm_seg_t,
            shm_fd: i32,
            read_only: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_shm_create_segment: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            shmseg: xcb_shm_seg_t,
            size: u32,
            read_only: u8,
        ) -> xcb_shm_create_segment_cookie_t,
    >,
    xcb_shm_create_segment_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            shmseg: xcb_shm_seg_t,
            size: u32,
            read_only: u8,
        ) -> xcb_shm_create_segment_cookie_t,
    >,
    xcb_shm_create_segment_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_shm_create_segment_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_shm_create_segment_reply_t,
    >,
    xcb_shm_create_segment_reply_fds: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            reply: *mut xcb_shm_create_segment_reply_t,
        ) -> *mut c_int,
    >,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self.shm.$f.get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .shm
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

#[cfg(feature = "xcb_shm")]
impl XcbShm {
    pub fn xcb_shm_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_shm_id) }
    }

    /// Returns `true` iff the symbol `xcb_shm_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_id(&self) -> bool {
        has_sym!(self, xcb_shm_id)
    }

    pub unsafe fn xcb_shm_seg_next(&self, i: *mut xcb_shm_seg_iterator_t) {
        sym!(self, xcb_shm_seg_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_shm_seg_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_seg_next(&self) -> bool {
        has_sym!(self, xcb_shm_seg_next)
    }

    pub unsafe fn xcb_shm_seg_end(&self, i: xcb_shm_seg_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_shm_seg_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_shm_seg_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_seg_end(&self) -> bool {
        has_sym!(self, xcb_shm_seg_end)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_shm_query_version(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_shm_query_version_cookie_t {
        sym!(self, xcb_shm_query_version)(c)
    }

    /// Returns `true` iff the symbol `xcb_shm_query_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_query_version(&self) -> bool {
        has_sym!(self, xcb_shm_query_version)
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
    pub unsafe fn xcb_shm_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_shm_query_version_cookie_t {
        sym!(self, xcb_shm_query_version_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_shm_query_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_query_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_shm_query_version_unchecked)
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
     * xcb_shm_query_version_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_shm_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_shm_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_shm_query_version_reply_t {
        sym!(self, xcb_shm_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_shm_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_shm_query_version_reply)
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
    pub unsafe fn xcb_shm_attach_checked(
        &self,
        c: *mut xcb_connection_t,
        shmseg: xcb_shm_seg_t,
        shmid: u32,
        read_only: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_shm_attach_checked)(c, shmseg, shmid, read_only)
    }

    /// Returns `true` iff the symbol `xcb_shm_attach_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_attach_checked(&self) -> bool {
        has_sym!(self, xcb_shm_attach_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_shm_attach(
        &self,
        c: *mut xcb_connection_t,
        shmseg: xcb_shm_seg_t,
        shmid: u32,
        read_only: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_shm_attach)(c, shmseg, shmid, read_only)
    }

    /// Returns `true` iff the symbol `xcb_shm_attach` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_attach(&self) -> bool {
        has_sym!(self, xcb_shm_attach)
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
    pub unsafe fn xcb_shm_detach_checked(
        &self,
        c: *mut xcb_connection_t,
        shmseg: xcb_shm_seg_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_shm_detach_checked)(c, shmseg)
    }

    /// Returns `true` iff the symbol `xcb_shm_detach_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_detach_checked(&self) -> bool {
        has_sym!(self, xcb_shm_detach_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_shm_detach(
        &self,
        c: *mut xcb_connection_t,
        shmseg: xcb_shm_seg_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_shm_detach)(c, shmseg)
    }

    /// Returns `true` iff the symbol `xcb_shm_detach` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_detach(&self) -> bool {
        has_sym!(self, xcb_shm_detach)
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
        sym!(self, xcb_shm_put_image_checked)(
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

    /// Returns `true` iff the symbol `xcb_shm_put_image_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_put_image_checked(&self) -> bool {
        has_sym!(self, xcb_shm_put_image_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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
        sym!(self, xcb_shm_put_image)(
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

    /// Returns `true` iff the symbol `xcb_shm_put_image` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_put_image(&self) -> bool {
        has_sym!(self, xcb_shm_put_image)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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
        sym!(self, xcb_shm_get_image)(
            c, drawable, x, y, width, height, plane_mask, format, shmseg, offset,
        )
    }

    /// Returns `true` iff the symbol `xcb_shm_get_image` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_get_image(&self) -> bool {
        has_sym!(self, xcb_shm_get_image)
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
        sym!(self, xcb_shm_get_image_unchecked)(
            c, drawable, x, y, width, height, plane_mask, format, shmseg, offset,
        )
    }

    /// Returns `true` iff the symbol `xcb_shm_get_image_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_get_image_unchecked(&self) -> bool {
        has_sym!(self, xcb_shm_get_image_unchecked)
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
     * xcb_shm_get_image_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_shm_get_image_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_shm_get_image_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_shm_get_image_reply_t {
        sym!(self, xcb_shm_get_image_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_shm_get_image_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_get_image_reply(&self) -> bool {
        has_sym!(self, xcb_shm_get_image_reply)
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
        sym!(self, xcb_shm_create_pixmap_checked)(
            c, pid, drawable, width, height, depth, shmseg, offset,
        )
    }

    /// Returns `true` iff the symbol `xcb_shm_create_pixmap_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_create_pixmap_checked(&self) -> bool {
        has_sym!(self, xcb_shm_create_pixmap_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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
        sym!(self, xcb_shm_create_pixmap)(c, pid, drawable, width, height, depth, shmseg, offset)
    }

    /// Returns `true` iff the symbol `xcb_shm_create_pixmap` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_create_pixmap(&self) -> bool {
        has_sym!(self, xcb_shm_create_pixmap)
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
    pub unsafe fn xcb_shm_attach_fd_checked(
        &self,
        c: *mut xcb_connection_t,
        shmseg: xcb_shm_seg_t,
        shm_fd: i32,
        read_only: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_shm_attach_fd_checked)(c, shmseg, shm_fd, read_only)
    }

    /// Returns `true` iff the symbol `xcb_shm_attach_fd_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_attach_fd_checked(&self) -> bool {
        has_sym!(self, xcb_shm_attach_fd_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_shm_attach_fd(
        &self,
        c: *mut xcb_connection_t,
        shmseg: xcb_shm_seg_t,
        shm_fd: i32,
        read_only: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_shm_attach_fd)(c, shmseg, shm_fd, read_only)
    }

    /// Returns `true` iff the symbol `xcb_shm_attach_fd` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_attach_fd(&self) -> bool {
        has_sym!(self, xcb_shm_attach_fd)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_shm_create_segment(
        &self,
        c: *mut xcb_connection_t,
        shmseg: xcb_shm_seg_t,
        size: u32,
        read_only: u8,
    ) -> xcb_shm_create_segment_cookie_t {
        sym!(self, xcb_shm_create_segment)(c, shmseg, size, read_only)
    }

    /// Returns `true` iff the symbol `xcb_shm_create_segment` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_create_segment(&self) -> bool {
        has_sym!(self, xcb_shm_create_segment)
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
    pub unsafe fn xcb_shm_create_segment_unchecked(
        &self,
        c: *mut xcb_connection_t,
        shmseg: xcb_shm_seg_t,
        size: u32,
        read_only: u8,
    ) -> xcb_shm_create_segment_cookie_t {
        sym!(self, xcb_shm_create_segment_unchecked)(c, shmseg, size, read_only)
    }

    /// Returns `true` iff the symbol `xcb_shm_create_segment_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_create_segment_unchecked(&self) -> bool {
        has_sym!(self, xcb_shm_create_segment_unchecked)
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
     * xcb_shm_create_segment_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_shm_create_segment_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_shm_create_segment_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_shm_create_segment_reply_t {
        sym!(self, xcb_shm_create_segment_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_shm_create_segment_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_create_segment_reply(&self) -> bool {
        has_sym!(self, xcb_shm_create_segment_reply)
    }
    pub unsafe fn xcb_shm_create_segment_reply_fds(
        &self,
        c: *mut xcb_connection_t,
        reply: *mut xcb_shm_create_segment_reply_t,
    ) -> *mut c_int {
        sym!(self, xcb_shm_create_segment_reply_fds)(c, reply)
    }

    /// Returns `true` iff the symbol `xcb_shm_create_segment_reply_fds` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_create_segment_reply_fds(&self) -> bool {
        has_sym!(self, xcb_shm_create_segment_reply_fds)
    }
}

#[cfg(feature = "xcb_shm")]
#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbShm::load().unwrap() };
        assert!(lib.has_xcb_shm_id());
        assert!(lib.has_xcb_shm_seg_next());
        assert!(lib.has_xcb_shm_seg_end());
        assert!(lib.has_xcb_shm_query_version());
        assert!(lib.has_xcb_shm_query_version_unchecked());
        assert!(lib.has_xcb_shm_query_version_reply());
        assert!(lib.has_xcb_shm_attach_checked());
        assert!(lib.has_xcb_shm_attach());
        assert!(lib.has_xcb_shm_detach_checked());
        assert!(lib.has_xcb_shm_detach());
        assert!(lib.has_xcb_shm_put_image_checked());
        assert!(lib.has_xcb_shm_put_image());
        assert!(lib.has_xcb_shm_get_image());
        assert!(lib.has_xcb_shm_get_image_unchecked());
        assert!(lib.has_xcb_shm_get_image_reply());
        assert!(lib.has_xcb_shm_create_pixmap_checked());
        assert!(lib.has_xcb_shm_create_pixmap());
        assert!(lib.has_xcb_shm_attach_fd_checked());
        assert!(lib.has_xcb_shm_attach_fd());
        assert!(lib.has_xcb_shm_create_segment());
        assert!(lib.has_xcb_shm_create_segment_unchecked());
        assert!(lib.has_xcb_shm_create_segment_reply());

        /**
         * Return the reply fds
         * @param c      The connection
         * @param reply  The reply
         *
         * Returns the array of reply fds of the request asked by
         *
         * The returned value must be freed by the caller using free().
         */
        assert!(lib.has_xcb_shm_create_segment_reply_fds());
    }
}
