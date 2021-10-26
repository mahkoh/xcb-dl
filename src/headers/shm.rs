// This file was generated using generate.py. Do not edit.
#![allow(unused_macros)]

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

/// The `Shm::SEG` type.
pub type xcb_shm_seg_t = u32;

/// An iterator over `Shm::SEG` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shm_seg_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_shm_seg_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_shm_seg_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Shm::Completion` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_shm_completion_event_t`].
pub const XCB_SHM_COMPLETION: u8 = 0i32 as u8;

/// The `Shm::Completion` event.
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

/// The opcode for `Shm::BadSeg` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_shm_bad_seg_error_t`].
pub const XCB_SHM_BAD_SEG: u8 = 0i32 as u8;

/// The `Shm::BadSeg` error.
pub type xcb_shm_bad_seg_error_t = xcb_value_error_t;

/// The cookie for the reply to a `Shm::QueryVersion` request.
///
/// Pass this cookie to [`xcb_shm_query_version_reply`] to retrieve the reply.
///
/// [`xcb_shm_query_version_reply`]: XcbShm::xcb_shm_query_version_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shm_query_version_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_shm_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Shm::QueryVersion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbShm::xcb_shm_id()`], then the type of the request is
/// [`xcb_shm_query_version_request_t`].
pub const XCB_SHM_QUERY_VERSION: u8 = 0i32 as u8;

/// The `Shm::QueryVersion` request.
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

/// The `Shm::QueryVersion` reply.
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

/// The opcode for `Shm::Attach` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbShm::xcb_shm_id()`], then the type of the request is
/// [`xcb_shm_attach_request_t`].
pub const XCB_SHM_ATTACH: u8 = 1i32 as u8;

/// The `Shm::Attach` request.
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

/// The opcode for `Shm::Detach` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbShm::xcb_shm_id()`], then the type of the request is
/// [`xcb_shm_detach_request_t`].
pub const XCB_SHM_DETACH: u8 = 2i32 as u8;

/// The `Shm::Detach` request.
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

/// The opcode for `Shm::PutImage` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbShm::xcb_shm_id()`], then the type of the request is
/// [`xcb_shm_put_image_request_t`].
pub const XCB_SHM_PUT_IMAGE: u8 = 3i32 as u8;

/// The `Shm::PutImage` request.
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

/// The cookie for the reply to a `Shm::GetImage` request.
///
/// Pass this cookie to [`xcb_shm_get_image_reply`] to retrieve the reply.
///
/// [`xcb_shm_get_image_reply`]: XcbShm::xcb_shm_get_image_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shm_get_image_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_shm_get_image_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Shm::GetImage` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbShm::xcb_shm_id()`], then the type of the request is
/// [`xcb_shm_get_image_request_t`].
pub const XCB_SHM_GET_IMAGE: u8 = 4i32 as u8;

/// The `Shm::GetImage` request.
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

/// The `Shm::GetImage` reply.
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

/// The opcode for `Shm::CreatePixmap` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbShm::xcb_shm_id()`], then the type of the request is
/// [`xcb_shm_create_pixmap_request_t`].
pub const XCB_SHM_CREATE_PIXMAP: u8 = 5i32 as u8;

/// The `Shm::CreatePixmap` request.
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

/// The opcode for `Shm::AttachFd` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbShm::xcb_shm_id()`], then the type of the request is
/// [`xcb_shm_attach_fd_request_t`].
pub const XCB_SHM_ATTACH_FD: u8 = 6i32 as u8;

/// The `Shm::AttachFd` request.
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

/// The cookie for the reply to a `Shm::CreateSegment` request.
///
/// Pass this cookie to [`xcb_shm_create_segment_reply`] to retrieve the reply.
///
/// [`xcb_shm_create_segment_reply`]: XcbShm::xcb_shm_create_segment_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_shm_create_segment_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_shm_create_segment_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Shm::CreateSegment` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbShm::xcb_shm_id()`], then the type of the request is
/// [`xcb_shm_create_segment_request_t`].
pub const XCB_SHM_CREATE_SEGMENT: u8 = 7i32 as u8;

/// The `Shm::CreateSegment` request.
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

/// The `Shm::CreateSegment` reply.
///
/// This reply contains file descriptors that can be accessed with [`xcb_shm_create_segment_reply_fds`].
///
/// [`xcb_shm_create_segment_reply_fds`]: XcbShm::xcb_shm_create_segment_reply_fds
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
        ) -> *mut xcb_shm_query_version_reply_t,
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
        ) -> *mut xcb_shm_get_image_reply_t,
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
        ) -> *mut xcb_shm_create_segment_reply_t,
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
    /// The libxcb identifier of the `Shm` extension.
    #[inline]
    pub fn xcb_shm_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_shm_id) }
    }

    /// Returns `true` iff the symbol `xcb_shm_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_id(&self) -> bool {
        has_sym!(self, xcb_shm_id)
    }

    /// Advances a `xcb_shm_seg_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_shm_seg_next(&self, i: *mut xcb_shm_seg_iterator_t) {
        sym!(self, xcb_shm_seg_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_shm_seg_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_seg_next(&self) -> bool {
        has_sym!(self, xcb_shm_seg_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_shm_seg_iterator_t`.
    #[inline]
    pub unsafe fn xcb_shm_seg_end(&self, i: xcb_shm_seg_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_shm_seg_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_shm_seg_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_seg_end(&self) -> bool {
        has_sym!(self, xcb_shm_seg_end)
    }

    /// Sends a `Shm::QueryVersion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_shm_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_shm_query_version_reply`]: Self::xcb_shm_query_version_reply
    #[inline]
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

    /// Sends a `Shm::QueryVersion` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_shm_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_shm_query_version_reply`]: Self::xcb_shm_query_version_reply
    #[inline]
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

    /// Waits for the reply to a `Shm::QueryVersion` request.
    #[inline]
    pub unsafe fn xcb_shm_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_shm_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_shm_query_version_reply_t {
        sym!(self, xcb_shm_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_shm_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_shm_query_version_reply)
    }

    /// Sends a `Shm::Attach` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
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

    /// Sends a `Shm::Attach` request (unchecked).
    #[inline]
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

    /// Sends a `Shm::Detach` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
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

    /// Sends a `Shm::Detach` request (unchecked).
    #[inline]
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

    /// Sends a `Shm::PutImage` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
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

    /// Sends a `Shm::PutImage` request (unchecked).
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

    /// Sends a `Shm::GetImage` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_shm_get_image_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_shm_get_image_reply`]: Self::xcb_shm_get_image_reply
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
        sym!(self, xcb_shm_get_image)(
            c, drawable, x, y, width, height, plane_mask, format, shmseg, offset,
        )
    }

    /// Returns `true` iff the symbol `xcb_shm_get_image` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_get_image(&self) -> bool {
        has_sym!(self, xcb_shm_get_image)
    }

    /// Sends a `Shm::GetImage` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_shm_get_image_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_shm_get_image_reply`]: Self::xcb_shm_get_image_reply
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
        sym!(self, xcb_shm_get_image_unchecked)(
            c, drawable, x, y, width, height, plane_mask, format, shmseg, offset,
        )
    }

    /// Returns `true` iff the symbol `xcb_shm_get_image_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_get_image_unchecked(&self) -> bool {
        has_sym!(self, xcb_shm_get_image_unchecked)
    }

    /// Waits for the reply to a `Shm::GetImage` request.
    #[inline]
    pub unsafe fn xcb_shm_get_image_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_shm_get_image_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_shm_get_image_reply_t {
        sym!(self, xcb_shm_get_image_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_shm_get_image_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_get_image_reply(&self) -> bool {
        has_sym!(self, xcb_shm_get_image_reply)
    }

    /// Sends a `Shm::CreatePixmap` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
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
        sym!(self, xcb_shm_create_pixmap_checked)(
            c, pid, drawable, width, height, depth, shmseg, offset,
        )
    }

    /// Returns `true` iff the symbol `xcb_shm_create_pixmap_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_create_pixmap_checked(&self) -> bool {
        has_sym!(self, xcb_shm_create_pixmap_checked)
    }

    /// Sends a `Shm::CreatePixmap` request (unchecked).
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
        sym!(self, xcb_shm_create_pixmap)(c, pid, drawable, width, height, depth, shmseg, offset)
    }

    /// Returns `true` iff the symbol `xcb_shm_create_pixmap` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_create_pixmap(&self) -> bool {
        has_sym!(self, xcb_shm_create_pixmap)
    }

    /// Sends a `Shm::AttachFd` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
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

    /// Sends a `Shm::AttachFd` request (unchecked).
    #[inline]
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

    /// Sends a `Shm::CreateSegment` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_shm_create_segment_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_shm_create_segment_reply`]: Self::xcb_shm_create_segment_reply
    #[inline]
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

    /// Sends a `Shm::CreateSegment` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_shm_create_segment_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_shm_create_segment_reply`]: Self::xcb_shm_create_segment_reply
    #[inline]
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

    /// Waits for the reply to a `Shm::CreateSegment` request.
    #[inline]
    pub unsafe fn xcb_shm_create_segment_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_shm_create_segment_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_shm_create_segment_reply_t {
        sym!(self, xcb_shm_create_segment_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_shm_create_segment_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_shm_create_segment_reply(&self) -> bool {
        has_sym!(self, xcb_shm_create_segment_reply)
    }

    /// Retrieves the file descriptors from the reply to a `Shm::CreateSegment` request.
    ///
    /// The returned pointer must be freed with `libc::free`.
    #[inline]
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
        assert!(lib.has_xcb_shm_create_segment_reply_fds());
    }
}
