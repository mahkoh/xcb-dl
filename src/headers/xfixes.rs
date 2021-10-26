// This file was generated using generate.py. Do not edit.
#![allow(unused_macros)]

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

/// The cookie for the reply to a `XFixes::QueryVersion` request.
///
/// Pass this cookie to [`xcb_xfixes_query_version_reply`] to retrieve the reply.
///
/// [`xcb_xfixes_query_version_reply`]: XcbXfixes::xcb_xfixes_query_version_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_query_version_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xfixes_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::QueryVersion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_query_version_request_t`].
pub const XCB_XFIXES_QUERY_VERSION: u8 = 0i32 as u8;

/// The `XFixes::QueryVersion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub client_major_version: u32,
    pub client_minor_version: u32,
}

impl Default for xcb_xfixes_query_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XFixes::QueryVersion` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u32,
    pub minor_version: u32,
    pub pad1: [u8; 16],
}

impl Default for xcb_xfixes_query_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XFixes::SaveSetMode` enum.
///
/// This enum has the following variants:
///
/// - [`XFixes::SaveSetMode::Insert`](XCB_XFIXES_SAVE_SET_MODE_INSERT)
/// - [`XFixes::SaveSetMode::Delete`](XCB_XFIXES_SAVE_SET_MODE_DELETE)
pub type xcb_xfixes_save_set_mode_t = u32;
/// The `XFixes::SaveSetMode::Insert` enum variant.
///
/// This is a variant of [`xcb_xfixes_save_set_mode_t`].
pub const XCB_XFIXES_SAVE_SET_MODE_INSERT: xcb_xfixes_save_set_mode_t = 0;
/// The `XFixes::SaveSetMode::Delete` enum variant.
///
/// This is a variant of [`xcb_xfixes_save_set_mode_t`].
pub const XCB_XFIXES_SAVE_SET_MODE_DELETE: xcb_xfixes_save_set_mode_t = 1;

/// The `XFixes::SaveSetTarget` enum.
///
/// This enum has the following variants:
///
/// - [`XFixes::SaveSetTarget::Nearest`](XCB_XFIXES_SAVE_SET_TARGET_NEAREST)
/// - [`XFixes::SaveSetTarget::Root`](XCB_XFIXES_SAVE_SET_TARGET_ROOT)
pub type xcb_xfixes_save_set_target_t = u32;
/// The `XFixes::SaveSetTarget::Nearest` enum variant.
///
/// This is a variant of [`xcb_xfixes_save_set_target_t`].
pub const XCB_XFIXES_SAVE_SET_TARGET_NEAREST: xcb_xfixes_save_set_target_t = 0;
/// The `XFixes::SaveSetTarget::Root` enum variant.
///
/// This is a variant of [`xcb_xfixes_save_set_target_t`].
pub const XCB_XFIXES_SAVE_SET_TARGET_ROOT: xcb_xfixes_save_set_target_t = 1;

/// The `XFixes::SaveSetMapping` enum.
///
/// This enum has the following variants:
///
/// - [`XFixes::SaveSetMapping::Map`](XCB_XFIXES_SAVE_SET_MAPPING_MAP)
/// - [`XFixes::SaveSetMapping::Unmap`](XCB_XFIXES_SAVE_SET_MAPPING_UNMAP)
pub type xcb_xfixes_save_set_mapping_t = u32;
/// The `XFixes::SaveSetMapping::Map` enum variant.
///
/// This is a variant of [`xcb_xfixes_save_set_mapping_t`].
pub const XCB_XFIXES_SAVE_SET_MAPPING_MAP: xcb_xfixes_save_set_mapping_t = 0;
/// The `XFixes::SaveSetMapping::Unmap` enum variant.
///
/// This is a variant of [`xcb_xfixes_save_set_mapping_t`].
pub const XCB_XFIXES_SAVE_SET_MAPPING_UNMAP: xcb_xfixes_save_set_mapping_t = 1;

/// The opcode for `XFixes::ChangeSaveSet` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_change_save_set_request_t`].
pub const XCB_XFIXES_CHANGE_SAVE_SET: u8 = 1i32 as u8;

/// The `XFixes::ChangeSaveSet` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_change_save_set_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub mode: u8,
    pub target: u8,
    pub map: u8,
    pub pad0: u8,
    pub window: xcb_window_t,
}

impl Default for xcb_xfixes_change_save_set_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XFixes::SelectionEvent` enum.
///
/// This enum has the following variants:
///
/// - [`XFixes::SelectionEvent::SetSelectionOwner`](XCB_XFIXES_SELECTION_EVENT_SET_SELECTION_OWNER)
/// - [`XFixes::SelectionEvent::SelectionWindowDestroy`](XCB_XFIXES_SELECTION_EVENT_SELECTION_WINDOW_DESTROY)
/// - [`XFixes::SelectionEvent::SelectionClientClose`](XCB_XFIXES_SELECTION_EVENT_SELECTION_CLIENT_CLOSE)
pub type xcb_xfixes_selection_event_t = u32;
/// The `XFixes::SelectionEvent::SetSelectionOwner` enum variant.
///
/// This is a variant of [`xcb_xfixes_selection_event_t`].
pub const XCB_XFIXES_SELECTION_EVENT_SET_SELECTION_OWNER: xcb_xfixes_selection_event_t = 0;
/// The `XFixes::SelectionEvent::SelectionWindowDestroy` enum variant.
///
/// This is a variant of [`xcb_xfixes_selection_event_t`].
pub const XCB_XFIXES_SELECTION_EVENT_SELECTION_WINDOW_DESTROY: xcb_xfixes_selection_event_t = 1;
/// The `XFixes::SelectionEvent::SelectionClientClose` enum variant.
///
/// This is a variant of [`xcb_xfixes_selection_event_t`].
pub const XCB_XFIXES_SELECTION_EVENT_SELECTION_CLIENT_CLOSE: xcb_xfixes_selection_event_t = 2;

/// The `XFixes::SelectionEventMask` enum.
///
/// This enum has the following variants:
///
/// - [`XFixes::SelectionEventMask::SetSelectionOwner`](XCB_XFIXES_SELECTION_EVENT_MASK_SET_SELECTION_OWNER)
/// - [`XFixes::SelectionEventMask::SelectionWindowDestroy`](XCB_XFIXES_SELECTION_EVENT_MASK_SELECTION_WINDOW_DESTROY)
/// - [`XFixes::SelectionEventMask::SelectionClientClose`](XCB_XFIXES_SELECTION_EVENT_MASK_SELECTION_CLIENT_CLOSE)
pub type xcb_xfixes_selection_event_mask_t = u32;
/// The `XFixes::SelectionEventMask::SetSelectionOwner` enum variant.
///
/// This is a variant of [`xcb_xfixes_selection_event_mask_t`].
pub const XCB_XFIXES_SELECTION_EVENT_MASK_SET_SELECTION_OWNER: xcb_xfixes_selection_event_mask_t =
    1;
/// The `XFixes::SelectionEventMask::SelectionWindowDestroy` enum variant.
///
/// This is a variant of [`xcb_xfixes_selection_event_mask_t`].
pub const XCB_XFIXES_SELECTION_EVENT_MASK_SELECTION_WINDOW_DESTROY:
    xcb_xfixes_selection_event_mask_t = 2;
/// The `XFixes::SelectionEventMask::SelectionClientClose` enum variant.
///
/// This is a variant of [`xcb_xfixes_selection_event_mask_t`].
pub const XCB_XFIXES_SELECTION_EVENT_MASK_SELECTION_CLIENT_CLOSE:
    xcb_xfixes_selection_event_mask_t = 4;

/// The opcode for `XFixes::SelectionNotify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_xfixes_selection_notify_event_t`].
pub const XCB_XFIXES_SELECTION_NOTIFY: u8 = 0i32 as u8;

/// The `XFixes::SelectionNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_selection_notify_event_t {
    pub response_type: u8,
    pub subtype: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub owner: xcb_window_t,
    pub selection: xcb_atom_t,
    pub timestamp: xcb_timestamp_t,
    pub selection_timestamp: xcb_timestamp_t,
    pub pad0: [u8; 8],
}

impl Default for xcb_xfixes_selection_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::SelectSelectionInput` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_select_selection_input_request_t`].
pub const XCB_XFIXES_SELECT_SELECTION_INPUT: u8 = 2i32 as u8;

/// The `XFixes::SelectSelectionInput` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_select_selection_input_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub selection: xcb_atom_t,
    pub event_mask: u32,
}

impl Default for xcb_xfixes_select_selection_input_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XFixes::CursorNotify` enum.
///
/// This enum has the following variants:
///
/// - [`XFixes::CursorNotify::DisplayCursor`](XCB_XFIXES_CURSOR_NOTIFY_DISPLAY_CURSOR)
pub type xcb_xfixes_cursor_notify_t = u32;
/// The `XFixes::CursorNotify::DisplayCursor` enum variant.
///
/// This is a variant of [`xcb_xfixes_cursor_notify_t`].
pub const XCB_XFIXES_CURSOR_NOTIFY_DISPLAY_CURSOR: xcb_xfixes_cursor_notify_t = 0;

/// The `XFixes::CursorNotifyMask` enum.
///
/// This enum has the following variants:
///
/// - [`XFixes::CursorNotifyMask::DisplayCursor`](XCB_XFIXES_CURSOR_NOTIFY_MASK_DISPLAY_CURSOR)
pub type xcb_xfixes_cursor_notify_mask_t = u32;
/// The `XFixes::CursorNotifyMask::DisplayCursor` enum variant.
///
/// This is a variant of [`xcb_xfixes_cursor_notify_mask_t`].
pub const XCB_XFIXES_CURSOR_NOTIFY_MASK_DISPLAY_CURSOR: xcb_xfixes_cursor_notify_mask_t = 1;

/// The opcode for `XFixes::CursorNotify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_xfixes_cursor_notify_event_t`].
pub const XCB_XFIXES_CURSOR_NOTIFY: u8 = 1i32 as u8;

/// The `XFixes::CursorNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_cursor_notify_event_t {
    pub response_type: u8,
    pub subtype: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub cursor_serial: u32,
    pub timestamp: xcb_timestamp_t,
    pub name: xcb_atom_t,
    pub pad0: [u8; 12],
}

impl Default for xcb_xfixes_cursor_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::SelectCursorInput` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_select_cursor_input_request_t`].
pub const XCB_XFIXES_SELECT_CURSOR_INPUT: u8 = 3i32 as u8;

/// The `XFixes::SelectCursorInput` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_select_cursor_input_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub event_mask: u32,
}

impl Default for xcb_xfixes_select_cursor_input_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `XFixes::GetCursorImage` request.
///
/// Pass this cookie to [`xcb_xfixes_get_cursor_image_reply`] to retrieve the reply.
///
/// [`xcb_xfixes_get_cursor_image_reply`]: XcbXfixes::xcb_xfixes_get_cursor_image_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_get_cursor_image_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xfixes_get_cursor_image_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::GetCursorImage` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_get_cursor_image_request_t`].
pub const XCB_XFIXES_GET_CURSOR_IMAGE: u8 = 4i32 as u8;

/// The `XFixes::GetCursorImage` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_get_cursor_image_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_xfixes_get_cursor_image_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XFixes::GetCursorImage` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `cursor_image`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_get_cursor_image_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub xhot: u16,
    pub yhot: u16,
    pub cursor_serial: u32,
    pub pad1: [u8; 8],
}

impl Default for xcb_xfixes_get_cursor_image_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XFixes::REGION` type.
pub type xcb_xfixes_region_t = u32;

/// An iterator over `XFixes::REGION` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_region_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xfixes_region_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xfixes_region_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::BadRegion` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_xfixes_bad_region_error_t`].
pub const XCB_XFIXES_BAD_REGION: u8 = 0i32 as u8;

/// The `XFixes::BadRegion` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_bad_region_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

impl Default for xcb_xfixes_bad_region_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XFixes::Region` enum.
///
/// This enum has the following variants:
///
/// - [`XFixes::Region::None`](XCB_XFIXES_REGION_NONE)
pub type xcb_xfixes_region_enum_t = u32;
/// The `XFixes::Region::None` enum variant.
///
/// This is a variant of [`xcb_xfixes_region_enum_t`].
pub const XCB_XFIXES_REGION_NONE: xcb_xfixes_region_enum_t = 0;

/// The opcode for `XFixes::CreateRegion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_create_region_request_t`].
pub const XCB_XFIXES_CREATE_REGION: u8 = 5i32 as u8;

/// The `XFixes::CreateRegion` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `rectangles`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_create_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub region: xcb_xfixes_region_t,
}

impl Default for xcb_xfixes_create_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::CreateRegionFromBitmap` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_create_region_from_bitmap_request_t`].
pub const XCB_XFIXES_CREATE_REGION_FROM_BITMAP: u8 = 6i32 as u8;

/// The `XFixes::CreateRegionFromBitmap` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_create_region_from_bitmap_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub region: xcb_xfixes_region_t,
    pub bitmap: xcb_pixmap_t,
}

impl Default for xcb_xfixes_create_region_from_bitmap_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::CreateRegionFromWindow` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_create_region_from_window_request_t`].
pub const XCB_XFIXES_CREATE_REGION_FROM_WINDOW: u8 = 7i32 as u8;

/// The `XFixes::CreateRegionFromWindow` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_create_region_from_window_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub region: xcb_xfixes_region_t,
    pub window: xcb_window_t,
    pub kind: xcb_shape_kind_t,
    pub pad0: [u8; 3],
}

impl Default for xcb_xfixes_create_region_from_window_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::CreateRegionFromGC` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_create_region_from_gc_request_t`].
pub const XCB_XFIXES_CREATE_REGION_FROM_GC: u8 = 8i32 as u8;

/// The `XFixes::CreateRegionFromGC` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_create_region_from_gc_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub region: xcb_xfixes_region_t,
    pub gc: xcb_gcontext_t,
}

impl Default for xcb_xfixes_create_region_from_gc_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::CreateRegionFromPicture` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_create_region_from_picture_request_t`].
pub const XCB_XFIXES_CREATE_REGION_FROM_PICTURE: u8 = 9i32 as u8;

/// The `XFixes::CreateRegionFromPicture` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_create_region_from_picture_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub region: xcb_xfixes_region_t,
    pub picture: xcb_render_picture_t,
}

impl Default for xcb_xfixes_create_region_from_picture_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::DestroyRegion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_destroy_region_request_t`].
pub const XCB_XFIXES_DESTROY_REGION: u8 = 10i32 as u8;

/// The `XFixes::DestroyRegion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_destroy_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub region: xcb_xfixes_region_t,
}

impl Default for xcb_xfixes_destroy_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::SetRegion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_set_region_request_t`].
pub const XCB_XFIXES_SET_REGION: u8 = 11i32 as u8;

/// The `XFixes::SetRegion` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `rectangles`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_set_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub region: xcb_xfixes_region_t,
}

impl Default for xcb_xfixes_set_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::CopyRegion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_copy_region_request_t`].
pub const XCB_XFIXES_COPY_REGION: u8 = 12i32 as u8;

/// The `XFixes::CopyRegion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_copy_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub source: xcb_xfixes_region_t,
    pub destination: xcb_xfixes_region_t,
}

impl Default for xcb_xfixes_copy_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::UnionRegion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_union_region_request_t`].
pub const XCB_XFIXES_UNION_REGION: u8 = 13i32 as u8;

/// The `XFixes::UnionRegion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_union_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub source1: xcb_xfixes_region_t,
    pub source2: xcb_xfixes_region_t,
    pub destination: xcb_xfixes_region_t,
}

impl Default for xcb_xfixes_union_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::IntersectRegion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_intersect_region_request_t`].
pub const XCB_XFIXES_INTERSECT_REGION: u8 = 14i32 as u8;

/// The `XFixes::IntersectRegion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_intersect_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub source1: xcb_xfixes_region_t,
    pub source2: xcb_xfixes_region_t,
    pub destination: xcb_xfixes_region_t,
}

impl Default for xcb_xfixes_intersect_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::SubtractRegion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_subtract_region_request_t`].
pub const XCB_XFIXES_SUBTRACT_REGION: u8 = 15i32 as u8;

/// The `XFixes::SubtractRegion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_subtract_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub source1: xcb_xfixes_region_t,
    pub source2: xcb_xfixes_region_t,
    pub destination: xcb_xfixes_region_t,
}

impl Default for xcb_xfixes_subtract_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::InvertRegion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_invert_region_request_t`].
pub const XCB_XFIXES_INVERT_REGION: u8 = 16i32 as u8;

/// The `XFixes::InvertRegion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_invert_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub source: xcb_xfixes_region_t,
    pub bounds: xcb_rectangle_t,
    pub destination: xcb_xfixes_region_t,
}

impl Default for xcb_xfixes_invert_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::TranslateRegion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_translate_region_request_t`].
pub const XCB_XFIXES_TRANSLATE_REGION: u8 = 17i32 as u8;

/// The `XFixes::TranslateRegion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_translate_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub region: xcb_xfixes_region_t,
    pub dx: i16,
    pub dy: i16,
}

impl Default for xcb_xfixes_translate_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::RegionExtents` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_region_extents_request_t`].
pub const XCB_XFIXES_REGION_EXTENTS: u8 = 18i32 as u8;

/// The `XFixes::RegionExtents` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_region_extents_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub source: xcb_xfixes_region_t,
    pub destination: xcb_xfixes_region_t,
}

impl Default for xcb_xfixes_region_extents_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `XFixes::FetchRegion` request.
///
/// Pass this cookie to [`xcb_xfixes_fetch_region_reply`] to retrieve the reply.
///
/// [`xcb_xfixes_fetch_region_reply`]: XcbXfixes::xcb_xfixes_fetch_region_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_fetch_region_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xfixes_fetch_region_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::FetchRegion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_fetch_region_request_t`].
pub const XCB_XFIXES_FETCH_REGION: u8 = 19i32 as u8;

/// The `XFixes::FetchRegion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_fetch_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub region: xcb_xfixes_region_t,
}

impl Default for xcb_xfixes_fetch_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XFixes::FetchRegion` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `rectangles`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_fetch_region_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub extents: xcb_rectangle_t,
    pub pad1: [u8; 16],
}

impl Default for xcb_xfixes_fetch_region_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::SetGCClipRegion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_set_gc_clip_region_request_t`].
pub const XCB_XFIXES_SET_GC_CLIP_REGION: u8 = 20i32 as u8;

/// The `XFixes::SetGCClipRegion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_set_gc_clip_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub gc: xcb_gcontext_t,
    pub region: xcb_xfixes_region_t,
    pub x_origin: i16,
    pub y_origin: i16,
}

impl Default for xcb_xfixes_set_gc_clip_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::SetWindowShapeRegion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_set_window_shape_region_request_t`].
pub const XCB_XFIXES_SET_WINDOW_SHAPE_REGION: u8 = 21i32 as u8;

/// The `XFixes::SetWindowShapeRegion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_set_window_shape_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub dest: xcb_window_t,
    pub dest_kind: xcb_shape_kind_t,
    pub pad0: [u8; 3],
    pub x_offset: i16,
    pub y_offset: i16,
    pub region: xcb_xfixes_region_t,
}

impl Default for xcb_xfixes_set_window_shape_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::SetPictureClipRegion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_set_picture_clip_region_request_t`].
pub const XCB_XFIXES_SET_PICTURE_CLIP_REGION: u8 = 22i32 as u8;

/// The `XFixes::SetPictureClipRegion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_set_picture_clip_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub picture: xcb_render_picture_t,
    pub region: xcb_xfixes_region_t,
    pub x_origin: i16,
    pub y_origin: i16,
}

impl Default for xcb_xfixes_set_picture_clip_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::SetCursorName` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_set_cursor_name_request_t`].
pub const XCB_XFIXES_SET_CURSOR_NAME: u8 = 23i32 as u8;

/// The `XFixes::SetCursorName` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `name`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_set_cursor_name_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub cursor: xcb_cursor_t,
    pub nbytes: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_xfixes_set_cursor_name_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `XFixes::GetCursorName` request.
///
/// Pass this cookie to [`xcb_xfixes_get_cursor_name_reply`] to retrieve the reply.
///
/// [`xcb_xfixes_get_cursor_name_reply`]: XcbXfixes::xcb_xfixes_get_cursor_name_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_get_cursor_name_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xfixes_get_cursor_name_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::GetCursorName` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_get_cursor_name_request_t`].
pub const XCB_XFIXES_GET_CURSOR_NAME: u8 = 24i32 as u8;

/// The `XFixes::GetCursorName` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_get_cursor_name_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub cursor: xcb_cursor_t,
}

impl Default for xcb_xfixes_get_cursor_name_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XFixes::GetCursorName` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `name`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_get_cursor_name_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub atom: xcb_atom_t,
    pub nbytes: u16,
    pub pad1: [u8; 18],
}

impl Default for xcb_xfixes_get_cursor_name_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `XFixes::GetCursorImageAndName` request.
///
/// Pass this cookie to [`xcb_xfixes_get_cursor_image_and_name_reply`] to retrieve the reply.
///
/// [`xcb_xfixes_get_cursor_image_and_name_reply`]: XcbXfixes::xcb_xfixes_get_cursor_image_and_name_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_get_cursor_image_and_name_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xfixes_get_cursor_image_and_name_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::GetCursorImageAndName` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_get_cursor_image_and_name_request_t`].
pub const XCB_XFIXES_GET_CURSOR_IMAGE_AND_NAME: u8 = 25i32 as u8;

/// The `XFixes::GetCursorImageAndName` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_get_cursor_image_and_name_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_xfixes_get_cursor_image_and_name_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XFixes::GetCursorImageAndName` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `cursor_image`
/// - `name`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_get_cursor_image_and_name_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub xhot: u16,
    pub yhot: u16,
    pub cursor_serial: u32,
    pub cursor_atom: xcb_atom_t,
    pub nbytes: u16,
    pub pad1: [u8; 2],
}

impl Default for xcb_xfixes_get_cursor_image_and_name_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::ChangeCursor` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_change_cursor_request_t`].
pub const XCB_XFIXES_CHANGE_CURSOR: u8 = 26i32 as u8;

/// The `XFixes::ChangeCursor` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_change_cursor_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub source: xcb_cursor_t,
    pub destination: xcb_cursor_t,
}

impl Default for xcb_xfixes_change_cursor_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::ChangeCursorByName` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_change_cursor_by_name_request_t`].
pub const XCB_XFIXES_CHANGE_CURSOR_BY_NAME: u8 = 27i32 as u8;

/// The `XFixes::ChangeCursorByName` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `name`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_change_cursor_by_name_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub src: xcb_cursor_t,
    pub nbytes: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_xfixes_change_cursor_by_name_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::ExpandRegion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_expand_region_request_t`].
pub const XCB_XFIXES_EXPAND_REGION: u8 = 28i32 as u8;

/// The `XFixes::ExpandRegion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_expand_region_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub source: xcb_xfixes_region_t,
    pub destination: xcb_xfixes_region_t,
    pub left: u16,
    pub right: u16,
    pub top: u16,
    pub bottom: u16,
}

impl Default for xcb_xfixes_expand_region_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::HideCursor` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_hide_cursor_request_t`].
pub const XCB_XFIXES_HIDE_CURSOR: u8 = 29i32 as u8;

/// The `XFixes::HideCursor` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_hide_cursor_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_xfixes_hide_cursor_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::ShowCursor` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_show_cursor_request_t`].
pub const XCB_XFIXES_SHOW_CURSOR: u8 = 30i32 as u8;

/// The `XFixes::ShowCursor` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_show_cursor_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_xfixes_show_cursor_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XFixes::BARRIER` type.
pub type xcb_xfixes_barrier_t = u32;

/// An iterator over `XFixes::BARRIER` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_barrier_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xfixes_barrier_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xfixes_barrier_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `XFixes::BarrierDirections` enum.
///
/// This enum has the following variants:
///
/// - [`XFixes::BarrierDirections::PositiveX`](XCB_XFIXES_BARRIER_DIRECTIONS_POSITIVE_X)
/// - [`XFixes::BarrierDirections::PositiveY`](XCB_XFIXES_BARRIER_DIRECTIONS_POSITIVE_Y)
/// - [`XFixes::BarrierDirections::NegativeX`](XCB_XFIXES_BARRIER_DIRECTIONS_NEGATIVE_X)
/// - [`XFixes::BarrierDirections::NegativeY`](XCB_XFIXES_BARRIER_DIRECTIONS_NEGATIVE_Y)
pub type xcb_xfixes_barrier_directions_t = u32;
/// The `XFixes::BarrierDirections::PositiveX` enum variant.
///
/// This is a variant of [`xcb_xfixes_barrier_directions_t`].
pub const XCB_XFIXES_BARRIER_DIRECTIONS_POSITIVE_X: xcb_xfixes_barrier_directions_t = 1;
/// The `XFixes::BarrierDirections::PositiveY` enum variant.
///
/// This is a variant of [`xcb_xfixes_barrier_directions_t`].
pub const XCB_XFIXES_BARRIER_DIRECTIONS_POSITIVE_Y: xcb_xfixes_barrier_directions_t = 2;
/// The `XFixes::BarrierDirections::NegativeX` enum variant.
///
/// This is a variant of [`xcb_xfixes_barrier_directions_t`].
pub const XCB_XFIXES_BARRIER_DIRECTIONS_NEGATIVE_X: xcb_xfixes_barrier_directions_t = 4;
/// The `XFixes::BarrierDirections::NegativeY` enum variant.
///
/// This is a variant of [`xcb_xfixes_barrier_directions_t`].
pub const XCB_XFIXES_BARRIER_DIRECTIONS_NEGATIVE_Y: xcb_xfixes_barrier_directions_t = 8;

/// The opcode for `XFixes::CreatePointerBarrier` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_create_pointer_barrier_request_t`].
pub const XCB_XFIXES_CREATE_POINTER_BARRIER: u8 = 31i32 as u8;

/// The `XFixes::CreatePointerBarrier` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `devices`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_create_pointer_barrier_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub barrier: xcb_xfixes_barrier_t,
    pub window: xcb_window_t,
    pub x1: u16,
    pub y1: u16,
    pub x2: u16,
    pub y2: u16,
    pub directions: u32,
    pub pad0: [u8; 2],
    pub num_devices: u16,
}

impl Default for xcb_xfixes_create_pointer_barrier_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `XFixes::DeletePointerBarrier` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXfixes::xcb_xfixes_id()`], then the type of the request is
/// [`xcb_xfixes_delete_pointer_barrier_request_t`].
pub const XCB_XFIXES_DELETE_POINTER_BARRIER: u8 = 32i32 as u8;

/// The `XFixes::DeletePointerBarrier` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xfixes_delete_pointer_barrier_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub barrier: xcb_xfixes_barrier_t,
}

impl Default for xcb_xfixes_delete_pointer_barrier_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[cfg(feature = "xcb_xfixes")]
pub(crate) struct XcbXfixesXfixes {
    xcb_xfixes_id: LazySymbol<*mut xcb_extension_t>,
    xcb_xfixes_query_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u32,
            client_minor_version: u32,
        ) -> xcb_xfixes_query_version_cookie_t,
    >,
    xcb_xfixes_query_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u32,
            client_minor_version: u32,
        ) -> xcb_xfixes_query_version_cookie_t,
    >,
    xcb_xfixes_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xfixes_query_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xfixes_query_version_reply_t,
    >,
    xcb_xfixes_change_save_set_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            mode: u8,
            target: u8,
            map: u8,
            window: xcb_window_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_change_save_set: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            mode: u8,
            target: u8,
            map: u8,
            window: xcb_window_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_select_selection_input_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            selection: xcb_atom_t,
            event_mask: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_select_selection_input: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            selection: xcb_atom_t,
            event_mask: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_select_cursor_input_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            event_mask: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_select_cursor_input: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            event_mask: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_get_cursor_image_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xfixes_get_cursor_image:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_xfixes_get_cursor_image_cookie_t>,
    xcb_xfixes_get_cursor_image_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_xfixes_get_cursor_image_cookie_t>,
    xcb_xfixes_get_cursor_image_cursor_image:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_get_cursor_image_reply_t) -> *mut u32>,
    xcb_xfixes_get_cursor_image_cursor_image_length:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_get_cursor_image_reply_t) -> c_int>,
    xcb_xfixes_get_cursor_image_cursor_image_end: LazySymbol<
        unsafe fn(r: *const xcb_xfixes_get_cursor_image_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_xfixes_get_cursor_image_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xfixes_get_cursor_image_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xfixes_get_cursor_image_reply_t,
    >,
    xcb_xfixes_region_next: LazySymbol<unsafe fn(i: *mut xcb_xfixes_region_iterator_t)>,
    xcb_xfixes_region_end:
        LazySymbol<unsafe fn(i: xcb_xfixes_region_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xfixes_create_region_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, rectangles_len: u32) -> c_int>,
    xcb_xfixes_create_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_create_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_create_region_rectangles:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_create_region_request_t) -> *mut xcb_rectangle_t>,
    xcb_xfixes_create_region_rectangles_length:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_create_region_request_t) -> c_int>,
    xcb_xfixes_create_region_rectangles_iterator: LazySymbol<
        unsafe fn(r: *const xcb_xfixes_create_region_request_t) -> xcb_rectangle_iterator_t,
    >,
    xcb_xfixes_create_region_from_bitmap_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            bitmap: xcb_pixmap_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_create_region_from_bitmap: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            bitmap: xcb_pixmap_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_create_region_from_window_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            window: xcb_window_t,
            kind: xcb_shape_kind_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_create_region_from_window: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            window: xcb_window_t,
            kind: xcb_shape_kind_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_create_region_from_gc_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            gc: xcb_gcontext_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_create_region_from_gc: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            gc: xcb_gcontext_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_create_region_from_picture_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            picture: xcb_render_picture_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_create_region_from_picture: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            picture: xcb_render_picture_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_destroy_region_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, region: xcb_xfixes_region_t) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_destroy_region: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, region: xcb_xfixes_region_t) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_set_region_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, rectangles_len: u32) -> c_int>,
    xcb_xfixes_set_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_set_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_set_region_rectangles:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_set_region_request_t) -> *mut xcb_rectangle_t>,
    xcb_xfixes_set_region_rectangles_length:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_set_region_request_t) -> c_int>,
    xcb_xfixes_set_region_rectangles_iterator: LazySymbol<
        unsafe fn(r: *const xcb_xfixes_set_region_request_t) -> xcb_rectangle_iterator_t,
    >,
    xcb_xfixes_copy_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_copy_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_union_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source1: xcb_xfixes_region_t,
            source2: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_union_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source1: xcb_xfixes_region_t,
            source2: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_intersect_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source1: xcb_xfixes_region_t,
            source2: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_intersect_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source1: xcb_xfixes_region_t,
            source2: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_subtract_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source1: xcb_xfixes_region_t,
            source2: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_subtract_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source1: xcb_xfixes_region_t,
            source2: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_invert_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source: xcb_xfixes_region_t,
            bounds: xcb_rectangle_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_invert_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source: xcb_xfixes_region_t,
            bounds: xcb_rectangle_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_translate_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            dx: i16,
            dy: i16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_translate_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
            dx: i16,
            dy: i16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_region_extents_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_region_extents: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_fetch_region_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xfixes_fetch_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
        ) -> xcb_xfixes_fetch_region_cookie_t,
    >,
    xcb_xfixes_fetch_region_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            region: xcb_xfixes_region_t,
        ) -> xcb_xfixes_fetch_region_cookie_t,
    >,
    xcb_xfixes_fetch_region_rectangles:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_fetch_region_reply_t) -> *mut xcb_rectangle_t>,
    xcb_xfixes_fetch_region_rectangles_length:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_fetch_region_reply_t) -> c_int>,
    xcb_xfixes_fetch_region_rectangles_iterator: LazySymbol<
        unsafe fn(r: *const xcb_xfixes_fetch_region_reply_t) -> xcb_rectangle_iterator_t,
    >,
    xcb_xfixes_fetch_region_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xfixes_fetch_region_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xfixes_fetch_region_reply_t,
    >,
    xcb_xfixes_set_gc_clip_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            gc: xcb_gcontext_t,
            region: xcb_xfixes_region_t,
            x_origin: i16,
            y_origin: i16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_set_gc_clip_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            gc: xcb_gcontext_t,
            region: xcb_xfixes_region_t,
            x_origin: i16,
            y_origin: i16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_set_window_shape_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            dest: xcb_window_t,
            dest_kind: xcb_shape_kind_t,
            x_offset: i16,
            y_offset: i16,
            region: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_set_window_shape_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            dest: xcb_window_t,
            dest_kind: xcb_shape_kind_t,
            x_offset: i16,
            y_offset: i16,
            region: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_set_picture_clip_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            region: xcb_xfixes_region_t,
            x_origin: i16,
            y_origin: i16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_set_picture_clip_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            picture: xcb_render_picture_t,
            region: xcb_xfixes_region_t,
            x_origin: i16,
            y_origin: i16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_set_cursor_name_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xfixes_set_cursor_name_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cursor: xcb_cursor_t,
            nbytes: u16,
            name: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_set_cursor_name: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cursor: xcb_cursor_t,
            nbytes: u16,
            name: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_set_cursor_name_name:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_set_cursor_name_request_t) -> *mut c_char>,
    xcb_xfixes_set_cursor_name_name_length:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_set_cursor_name_request_t) -> c_int>,
    xcb_xfixes_set_cursor_name_name_end: LazySymbol<
        unsafe fn(r: *const xcb_xfixes_set_cursor_name_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_xfixes_get_cursor_name_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xfixes_get_cursor_name: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cursor: xcb_cursor_t,
        ) -> xcb_xfixes_get_cursor_name_cookie_t,
    >,
    xcb_xfixes_get_cursor_name_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cursor: xcb_cursor_t,
        ) -> xcb_xfixes_get_cursor_name_cookie_t,
    >,
    xcb_xfixes_get_cursor_name_name:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_get_cursor_name_reply_t) -> *mut c_char>,
    xcb_xfixes_get_cursor_name_name_length:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_get_cursor_name_reply_t) -> c_int>,
    xcb_xfixes_get_cursor_name_name_end: LazySymbol<
        unsafe fn(r: *const xcb_xfixes_get_cursor_name_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_xfixes_get_cursor_name_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xfixes_get_cursor_name_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xfixes_get_cursor_name_reply_t,
    >,
    xcb_xfixes_get_cursor_image_and_name_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xfixes_get_cursor_image_and_name: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_xfixes_get_cursor_image_and_name_cookie_t,
    >,
    xcb_xfixes_get_cursor_image_and_name_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t) -> xcb_xfixes_get_cursor_image_and_name_cookie_t,
    >,
    xcb_xfixes_get_cursor_image_and_name_cursor_image:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_get_cursor_image_and_name_reply_t) -> *mut u32>,
    xcb_xfixes_get_cursor_image_and_name_cursor_image_length:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_get_cursor_image_and_name_reply_t) -> c_int>,
    xcb_xfixes_get_cursor_image_and_name_cursor_image_end: LazySymbol<
        unsafe fn(r: *const xcb_xfixes_get_cursor_image_and_name_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_xfixes_get_cursor_image_and_name_name: LazySymbol<
        unsafe fn(r: *const xcb_xfixes_get_cursor_image_and_name_reply_t) -> *mut c_char,
    >,
    xcb_xfixes_get_cursor_image_and_name_name_length:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_get_cursor_image_and_name_reply_t) -> c_int>,
    xcb_xfixes_get_cursor_image_and_name_name_end: LazySymbol<
        unsafe fn(r: *const xcb_xfixes_get_cursor_image_and_name_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_xfixes_get_cursor_image_and_name_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xfixes_get_cursor_image_and_name_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xfixes_get_cursor_image_and_name_reply_t,
    >,
    xcb_xfixes_change_cursor_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source: xcb_cursor_t,
            destination: xcb_cursor_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_change_cursor: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source: xcb_cursor_t,
            destination: xcb_cursor_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_change_cursor_by_name_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xfixes_change_cursor_by_name_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src: xcb_cursor_t,
            nbytes: u16,
            name: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_change_cursor_by_name: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src: xcb_cursor_t,
            nbytes: u16,
            name: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_change_cursor_by_name_name:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_change_cursor_by_name_request_t) -> *mut c_char>,
    xcb_xfixes_change_cursor_by_name_name_length:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_change_cursor_by_name_request_t) -> c_int>,
    xcb_xfixes_change_cursor_by_name_name_end: LazySymbol<
        unsafe fn(r: *const xcb_xfixes_change_cursor_by_name_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_xfixes_expand_region_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
            left: u16,
            right: u16,
            top: u16,
            bottom: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_expand_region: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            source: xcb_xfixes_region_t,
            destination: xcb_xfixes_region_t,
            left: u16,
            right: u16,
            top: u16,
            bottom: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_hide_cursor_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_xfixes_hide_cursor:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_xfixes_show_cursor_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_xfixes_show_cursor:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_xfixes_barrier_next: LazySymbol<unsafe fn(i: *mut xcb_xfixes_barrier_iterator_t)>,
    xcb_xfixes_barrier_end:
        LazySymbol<unsafe fn(i: xcb_xfixes_barrier_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xfixes_create_pointer_barrier_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xfixes_create_pointer_barrier_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            barrier: xcb_xfixes_barrier_t,
            window: xcb_window_t,
            x1: u16,
            y1: u16,
            x2: u16,
            y2: u16,
            directions: u32,
            num_devices: u16,
            devices: *const u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_create_pointer_barrier: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            barrier: xcb_xfixes_barrier_t,
            window: xcb_window_t,
            x1: u16,
            y1: u16,
            x2: u16,
            y2: u16,
            directions: u32,
            num_devices: u16,
            devices: *const u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_create_pointer_barrier_devices:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_create_pointer_barrier_request_t) -> *mut u16>,
    xcb_xfixes_create_pointer_barrier_devices_length:
        LazySymbol<unsafe fn(r: *const xcb_xfixes_create_pointer_barrier_request_t) -> c_int>,
    xcb_xfixes_create_pointer_barrier_devices_end: LazySymbol<
        unsafe fn(r: *const xcb_xfixes_create_pointer_barrier_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_xfixes_delete_pointer_barrier_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, barrier: xcb_xfixes_barrier_t) -> xcb_void_cookie_t,
    >,
    xcb_xfixes_delete_pointer_barrier: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, barrier: xcb_xfixes_barrier_t) -> xcb_void_cookie_t,
    >,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self
            .xfixes
            .$f
            .get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .xfixes
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

#[cfg(feature = "xcb_xfixes")]
impl XcbXfixes {
    /// The libxcb identifier of the `XFixes` extension.
    #[inline]
    pub fn xcb_xfixes_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_xfixes_id) }
    }

    /// Returns `true` iff the symbol `xcb_xfixes_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_id(&self) -> bool {
        has_sym!(self, xcb_xfixes_id)
    }

    /// Sends a `XFixes::QueryVersion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xfixes_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xfixes_query_version_reply`]: Self::xcb_xfixes_query_version_reply
    #[inline]
    pub unsafe fn xcb_xfixes_query_version(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u32,
        client_minor_version: u32,
    ) -> xcb_xfixes_query_version_cookie_t {
        sym!(self, xcb_xfixes_query_version)(c, client_major_version, client_minor_version)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_query_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_query_version(&self) -> bool {
        has_sym!(self, xcb_xfixes_query_version)
    }

    /// Sends a `XFixes::QueryVersion` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xfixes_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xfixes_query_version_reply`]: Self::xcb_xfixes_query_version_reply
    #[inline]
    pub unsafe fn xcb_xfixes_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u32,
        client_minor_version: u32,
    ) -> xcb_xfixes_query_version_cookie_t {
        sym!(self, xcb_xfixes_query_version_unchecked)(
            c,
            client_major_version,
            client_minor_version,
        )
    }

    /// Returns `true` iff the symbol `xcb_xfixes_query_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_query_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_xfixes_query_version_unchecked)
    }

    /// Waits for the reply to a `XFixes::QueryVersion` request.
    #[inline]
    pub unsafe fn xcb_xfixes_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xfixes_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xfixes_query_version_reply_t {
        sym!(self, xcb_xfixes_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_xfixes_query_version_reply)
    }

    /// Sends a `XFixes::ChangeSaveSet` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xfixes_change_save_set_checked(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
        target: u8,
        map: u8,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_change_save_set_checked)(c, mode, target, map, window)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_change_save_set_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_change_save_set_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_change_save_set_checked)
    }

    /// Sends a `XFixes::ChangeSaveSet` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xfixes_change_save_set(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
        target: u8,
        map: u8,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_change_save_set)(c, mode, target, map, window)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_change_save_set` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_change_save_set(&self) -> bool {
        has_sym!(self, xcb_xfixes_change_save_set)
    }

    /// Sends a `XFixes::SelectSelectionInput` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xfixes_select_selection_input_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        selection: xcb_atom_t,
        event_mask: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_select_selection_input_checked)(c, window, selection, event_mask)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_select_selection_input_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_select_selection_input_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_select_selection_input_checked)
    }

    /// Sends a `XFixes::SelectSelectionInput` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xfixes_select_selection_input(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        selection: xcb_atom_t,
        event_mask: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_select_selection_input)(c, window, selection, event_mask)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_select_selection_input` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_select_selection_input(&self) -> bool {
        has_sym!(self, xcb_xfixes_select_selection_input)
    }

    /// Sends a `XFixes::SelectCursorInput` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xfixes_select_cursor_input_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        event_mask: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_select_cursor_input_checked)(c, window, event_mask)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_select_cursor_input_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_select_cursor_input_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_select_cursor_input_checked)
    }

    /// Sends a `XFixes::SelectCursorInput` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xfixes_select_cursor_input(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        event_mask: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_select_cursor_input)(c, window, event_mask)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_select_cursor_input` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_select_cursor_input(&self) -> bool {
        has_sym!(self, xcb_xfixes_select_cursor_input)
    }

    /// Computes the size of a `xcb_xfixes_get_cursor_image_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xfixes_get_cursor_image_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_sizeof(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_sizeof)
    }

    /// Sends a `XFixes::GetCursorImage` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xfixes_get_cursor_image_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xfixes_get_cursor_image_reply`]: Self::xcb_xfixes_get_cursor_image_reply
    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xfixes_get_cursor_image_cookie_t {
        sym!(self, xcb_xfixes_get_cursor_image)(c)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image)
    }

    /// Sends a `XFixes::GetCursorImage` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xfixes_get_cursor_image_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xfixes_get_cursor_image_reply`]: Self::xcb_xfixes_get_cursor_image_reply
    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xfixes_get_cursor_image_cookie_t {
        sym!(self, xcb_xfixes_get_cursor_image_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_unchecked(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_unchecked)
    }

    /// Returns a pointer to the `cursor_image` field of a `xcb_xfixes_get_cursor_image_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_cursor_image(
        &self,
        r: *const xcb_xfixes_get_cursor_image_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_xfixes_get_cursor_image_cursor_image)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_cursor_image` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_cursor_image(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_cursor_image)
    }

    /// Returns the number of elements of the `cursor_image` field of a `xcb_xfixes_get_cursor_image_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_cursor_image_length(
        &self,
        r: *const xcb_xfixes_get_cursor_image_reply_t,
    ) -> c_int {
        sym!(self, xcb_xfixes_get_cursor_image_cursor_image_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_cursor_image_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_cursor_image_length(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_cursor_image_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `cursor_image` field of a `xcb_xfixes_get_cursor_image_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_cursor_image_end(
        &self,
        r: *const xcb_xfixes_get_cursor_image_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xfixes_get_cursor_image_cursor_image_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_cursor_image_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_cursor_image_end(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_cursor_image_end)
    }

    /// Waits for the reply to a `XFixes::GetCursorImage` request.
    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xfixes_get_cursor_image_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xfixes_get_cursor_image_reply_t {
        sym!(self, xcb_xfixes_get_cursor_image_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_reply(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_reply)
    }

    /// Advances a `xcb_xfixes_region_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xfixes_region_next(&self, i: *mut xcb_xfixes_region_iterator_t) {
        sym!(self, xcb_xfixes_region_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_region_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_region_next(&self) -> bool {
        has_sym!(self, xcb_xfixes_region_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xfixes_region_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xfixes_region_end(
        &self,
        i: xcb_xfixes_region_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xfixes_region_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_region_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_region_end(&self) -> bool {
        has_sym!(self, xcb_xfixes_region_end)
    }

    /// Computes the size of a `xcb_xfixes_create_region_request_t` object.
    #[inline]
    pub unsafe fn xcb_xfixes_create_region_sizeof(
        &self,
        _buffer: *const c_void,
        rectangles_len: u32,
    ) -> c_int {
        sym!(self, xcb_xfixes_create_region_sizeof)(_buffer, rectangles_len)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_region_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_region_sizeof(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_region_sizeof)
    }

    /// Sends a `XFixes::CreateRegion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xfixes_create_region_checked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_create_region_checked)(c, region, rectangles_len, rectangles)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_region_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_region_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_region_checked)
    }

    /// Sends a `XFixes::CreateRegion` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xfixes_create_region(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_create_region)(c, region, rectangles_len, rectangles)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_region(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_region)
    }

    /// Returns a pointer to the `rectangles` field of a `xcb_xfixes_create_region_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_create_region_rectangles(
        &self,
        r: *const xcb_xfixes_create_region_request_t,
    ) -> *mut xcb_rectangle_t {
        sym!(self, xcb_xfixes_create_region_rectangles)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_region_rectangles` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_region_rectangles(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_region_rectangles)
    }

    /// Returns the number of elements of the `rectangles` field of a `xcb_xfixes_create_region_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_create_region_rectangles_length(
        &self,
        r: *const xcb_xfixes_create_region_request_t,
    ) -> c_int {
        sym!(self, xcb_xfixes_create_region_rectangles_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_region_rectangles_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_region_rectangles_length(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_region_rectangles_length)
    }

    /// Returns an iterator over the elements of the
    /// `rectangles` field of a `xcb_xfixes_create_region_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_create_region_rectangles_iterator(
        &self,
        r: *const xcb_xfixes_create_region_request_t,
    ) -> xcb_rectangle_iterator_t {
        sym!(self, xcb_xfixes_create_region_rectangles_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_region_rectangles_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_region_rectangles_iterator(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_region_rectangles_iterator)
    }

    /// Sends a `XFixes::CreateRegionFromBitmap` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xfixes_create_region_from_bitmap_checked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        bitmap: xcb_pixmap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_create_region_from_bitmap_checked)(c, region, bitmap)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_region_from_bitmap_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_region_from_bitmap_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_region_from_bitmap_checked)
    }

    /// Sends a `XFixes::CreateRegionFromBitmap` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xfixes_create_region_from_bitmap(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        bitmap: xcb_pixmap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_create_region_from_bitmap)(c, region, bitmap)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_region_from_bitmap` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_region_from_bitmap(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_region_from_bitmap)
    }

    /// Sends a `XFixes::CreateRegionFromWindow` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xfixes_create_region_from_window_checked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        window: xcb_window_t,
        kind: xcb_shape_kind_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_create_region_from_window_checked)(c, region, window, kind)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_region_from_window_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_region_from_window_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_region_from_window_checked)
    }

    /// Sends a `XFixes::CreateRegionFromWindow` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xfixes_create_region_from_window(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        window: xcb_window_t,
        kind: xcb_shape_kind_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_create_region_from_window)(c, region, window, kind)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_region_from_window` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_region_from_window(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_region_from_window)
    }

    /// Sends a `XFixes::CreateRegionFromGC` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xfixes_create_region_from_gc_checked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        gc: xcb_gcontext_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_create_region_from_gc_checked)(c, region, gc)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_region_from_gc_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_region_from_gc_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_region_from_gc_checked)
    }

    /// Sends a `XFixes::CreateRegionFromGC` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xfixes_create_region_from_gc(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        gc: xcb_gcontext_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_create_region_from_gc)(c, region, gc)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_region_from_gc` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_region_from_gc(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_region_from_gc)
    }

    /// Sends a `XFixes::CreateRegionFromPicture` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xfixes_create_region_from_picture_checked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        picture: xcb_render_picture_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_create_region_from_picture_checked)(c, region, picture)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_region_from_picture_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_region_from_picture_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_region_from_picture_checked)
    }

    /// Sends a `XFixes::CreateRegionFromPicture` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xfixes_create_region_from_picture(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        picture: xcb_render_picture_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_create_region_from_picture)(c, region, picture)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_region_from_picture` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_region_from_picture(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_region_from_picture)
    }

    /// Sends a `XFixes::DestroyRegion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xfixes_destroy_region_checked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_destroy_region_checked)(c, region)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_destroy_region_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_destroy_region_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_destroy_region_checked)
    }

    /// Sends a `XFixes::DestroyRegion` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xfixes_destroy_region(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_destroy_region)(c, region)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_destroy_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_destroy_region(&self) -> bool {
        has_sym!(self, xcb_xfixes_destroy_region)
    }

    /// Computes the size of a `xcb_xfixes_set_region_request_t` object.
    #[inline]
    pub unsafe fn xcb_xfixes_set_region_sizeof(
        &self,
        _buffer: *const c_void,
        rectangles_len: u32,
    ) -> c_int {
        sym!(self, xcb_xfixes_set_region_sizeof)(_buffer, rectangles_len)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_region_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_region_sizeof(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_region_sizeof)
    }

    /// Sends a `XFixes::SetRegion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xfixes_set_region_checked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_set_region_checked)(c, region, rectangles_len, rectangles)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_region_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_region_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_region_checked)
    }

    /// Sends a `XFixes::SetRegion` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xfixes_set_region(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_set_region)(c, region, rectangles_len, rectangles)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_region(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_region)
    }

    /// Returns a pointer to the `rectangles` field of a `xcb_xfixes_set_region_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_set_region_rectangles(
        &self,
        r: *const xcb_xfixes_set_region_request_t,
    ) -> *mut xcb_rectangle_t {
        sym!(self, xcb_xfixes_set_region_rectangles)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_region_rectangles` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_region_rectangles(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_region_rectangles)
    }

    /// Returns the number of elements of the `rectangles` field of a `xcb_xfixes_set_region_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_set_region_rectangles_length(
        &self,
        r: *const xcb_xfixes_set_region_request_t,
    ) -> c_int {
        sym!(self, xcb_xfixes_set_region_rectangles_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_region_rectangles_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_region_rectangles_length(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_region_rectangles_length)
    }

    /// Returns an iterator over the elements of the
    /// `rectangles` field of a `xcb_xfixes_set_region_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_set_region_rectangles_iterator(
        &self,
        r: *const xcb_xfixes_set_region_request_t,
    ) -> xcb_rectangle_iterator_t {
        sym!(self, xcb_xfixes_set_region_rectangles_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_region_rectangles_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_region_rectangles_iterator(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_region_rectangles_iterator)
    }

    /// Sends a `XFixes::CopyRegion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xfixes_copy_region_checked(
        &self,
        c: *mut xcb_connection_t,
        source: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_copy_region_checked)(c, source, destination)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_copy_region_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_copy_region_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_copy_region_checked)
    }

    /// Sends a `XFixes::CopyRegion` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xfixes_copy_region(
        &self,
        c: *mut xcb_connection_t,
        source: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_copy_region)(c, source, destination)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_copy_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_copy_region(&self) -> bool {
        has_sym!(self, xcb_xfixes_copy_region)
    }

    /// Sends a `XFixes::UnionRegion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xfixes_union_region_checked(
        &self,
        c: *mut xcb_connection_t,
        source1: xcb_xfixes_region_t,
        source2: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_union_region_checked)(c, source1, source2, destination)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_union_region_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_union_region_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_union_region_checked)
    }

    /// Sends a `XFixes::UnionRegion` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xfixes_union_region(
        &self,
        c: *mut xcb_connection_t,
        source1: xcb_xfixes_region_t,
        source2: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_union_region)(c, source1, source2, destination)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_union_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_union_region(&self) -> bool {
        has_sym!(self, xcb_xfixes_union_region)
    }

    /// Sends a `XFixes::IntersectRegion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xfixes_intersect_region_checked(
        &self,
        c: *mut xcb_connection_t,
        source1: xcb_xfixes_region_t,
        source2: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_intersect_region_checked)(c, source1, source2, destination)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_intersect_region_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_intersect_region_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_intersect_region_checked)
    }

    /// Sends a `XFixes::IntersectRegion` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xfixes_intersect_region(
        &self,
        c: *mut xcb_connection_t,
        source1: xcb_xfixes_region_t,
        source2: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_intersect_region)(c, source1, source2, destination)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_intersect_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_intersect_region(&self) -> bool {
        has_sym!(self, xcb_xfixes_intersect_region)
    }

    /// Sends a `XFixes::SubtractRegion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xfixes_subtract_region_checked(
        &self,
        c: *mut xcb_connection_t,
        source1: xcb_xfixes_region_t,
        source2: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_subtract_region_checked)(c, source1, source2, destination)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_subtract_region_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_subtract_region_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_subtract_region_checked)
    }

    /// Sends a `XFixes::SubtractRegion` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xfixes_subtract_region(
        &self,
        c: *mut xcb_connection_t,
        source1: xcb_xfixes_region_t,
        source2: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_subtract_region)(c, source1, source2, destination)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_subtract_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_subtract_region(&self) -> bool {
        has_sym!(self, xcb_xfixes_subtract_region)
    }

    /// Sends a `XFixes::InvertRegion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xfixes_invert_region_checked(
        &self,
        c: *mut xcb_connection_t,
        source: xcb_xfixes_region_t,
        bounds: xcb_rectangle_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_invert_region_checked)(c, source, bounds, destination)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_invert_region_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_invert_region_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_invert_region_checked)
    }

    /// Sends a `XFixes::InvertRegion` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xfixes_invert_region(
        &self,
        c: *mut xcb_connection_t,
        source: xcb_xfixes_region_t,
        bounds: xcb_rectangle_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_invert_region)(c, source, bounds, destination)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_invert_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_invert_region(&self) -> bool {
        has_sym!(self, xcb_xfixes_invert_region)
    }

    /// Sends a `XFixes::TranslateRegion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xfixes_translate_region_checked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        dx: i16,
        dy: i16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_translate_region_checked)(c, region, dx, dy)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_translate_region_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_translate_region_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_translate_region_checked)
    }

    /// Sends a `XFixes::TranslateRegion` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xfixes_translate_region(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
        dx: i16,
        dy: i16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_translate_region)(c, region, dx, dy)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_translate_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_translate_region(&self) -> bool {
        has_sym!(self, xcb_xfixes_translate_region)
    }

    /// Sends a `XFixes::RegionExtents` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xfixes_region_extents_checked(
        &self,
        c: *mut xcb_connection_t,
        source: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_region_extents_checked)(c, source, destination)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_region_extents_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_region_extents_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_region_extents_checked)
    }

    /// Sends a `XFixes::RegionExtents` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xfixes_region_extents(
        &self,
        c: *mut xcb_connection_t,
        source: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_region_extents)(c, source, destination)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_region_extents` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_region_extents(&self) -> bool {
        has_sym!(self, xcb_xfixes_region_extents)
    }

    /// Computes the size of a `xcb_xfixes_fetch_region_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xfixes_fetch_region_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xfixes_fetch_region_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_fetch_region_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_fetch_region_sizeof(&self) -> bool {
        has_sym!(self, xcb_xfixes_fetch_region_sizeof)
    }

    /// Sends a `XFixes::FetchRegion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xfixes_fetch_region_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xfixes_fetch_region_reply`]: Self::xcb_xfixes_fetch_region_reply
    #[inline]
    pub unsafe fn xcb_xfixes_fetch_region(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
    ) -> xcb_xfixes_fetch_region_cookie_t {
        sym!(self, xcb_xfixes_fetch_region)(c, region)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_fetch_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_fetch_region(&self) -> bool {
        has_sym!(self, xcb_xfixes_fetch_region)
    }

    /// Sends a `XFixes::FetchRegion` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xfixes_fetch_region_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xfixes_fetch_region_reply`]: Self::xcb_xfixes_fetch_region_reply
    #[inline]
    pub unsafe fn xcb_xfixes_fetch_region_unchecked(
        &self,
        c: *mut xcb_connection_t,
        region: xcb_xfixes_region_t,
    ) -> xcb_xfixes_fetch_region_cookie_t {
        sym!(self, xcb_xfixes_fetch_region_unchecked)(c, region)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_fetch_region_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_fetch_region_unchecked(&self) -> bool {
        has_sym!(self, xcb_xfixes_fetch_region_unchecked)
    }

    /// Returns a pointer to the `rectangles` field of a `xcb_xfixes_fetch_region_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_fetch_region_rectangles(
        &self,
        r: *const xcb_xfixes_fetch_region_reply_t,
    ) -> *mut xcb_rectangle_t {
        sym!(self, xcb_xfixes_fetch_region_rectangles)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_fetch_region_rectangles` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_fetch_region_rectangles(&self) -> bool {
        has_sym!(self, xcb_xfixes_fetch_region_rectangles)
    }

    /// Returns the number of elements of the `rectangles` field of a `xcb_xfixes_fetch_region_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_fetch_region_rectangles_length(
        &self,
        r: *const xcb_xfixes_fetch_region_reply_t,
    ) -> c_int {
        sym!(self, xcb_xfixes_fetch_region_rectangles_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_fetch_region_rectangles_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_fetch_region_rectangles_length(&self) -> bool {
        has_sym!(self, xcb_xfixes_fetch_region_rectangles_length)
    }

    /// Returns an iterator over the elements of the
    /// `rectangles` field of a `xcb_xfixes_fetch_region_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_fetch_region_rectangles_iterator(
        &self,
        r: *const xcb_xfixes_fetch_region_reply_t,
    ) -> xcb_rectangle_iterator_t {
        sym!(self, xcb_xfixes_fetch_region_rectangles_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_fetch_region_rectangles_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_fetch_region_rectangles_iterator(&self) -> bool {
        has_sym!(self, xcb_xfixes_fetch_region_rectangles_iterator)
    }

    /// Waits for the reply to a `XFixes::FetchRegion` request.
    #[inline]
    pub unsafe fn xcb_xfixes_fetch_region_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xfixes_fetch_region_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xfixes_fetch_region_reply_t {
        sym!(self, xcb_xfixes_fetch_region_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_fetch_region_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_fetch_region_reply(&self) -> bool {
        has_sym!(self, xcb_xfixes_fetch_region_reply)
    }

    /// Sends a `XFixes::SetGCClipRegion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xfixes_set_gc_clip_region_checked(
        &self,
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        region: xcb_xfixes_region_t,
        x_origin: i16,
        y_origin: i16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_set_gc_clip_region_checked)(c, gc, region, x_origin, y_origin)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_gc_clip_region_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_gc_clip_region_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_gc_clip_region_checked)
    }

    /// Sends a `XFixes::SetGCClipRegion` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xfixes_set_gc_clip_region(
        &self,
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        region: xcb_xfixes_region_t,
        x_origin: i16,
        y_origin: i16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_set_gc_clip_region)(c, gc, region, x_origin, y_origin)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_gc_clip_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_gc_clip_region(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_gc_clip_region)
    }

    /// Sends a `XFixes::SetWindowShapeRegion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xfixes_set_window_shape_region_checked(
        &self,
        c: *mut xcb_connection_t,
        dest: xcb_window_t,
        dest_kind: xcb_shape_kind_t,
        x_offset: i16,
        y_offset: i16,
        region: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_set_window_shape_region_checked)(
            c, dest, dest_kind, x_offset, y_offset, region,
        )
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_window_shape_region_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_window_shape_region_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_window_shape_region_checked)
    }

    /// Sends a `XFixes::SetWindowShapeRegion` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xfixes_set_window_shape_region(
        &self,
        c: *mut xcb_connection_t,
        dest: xcb_window_t,
        dest_kind: xcb_shape_kind_t,
        x_offset: i16,
        y_offset: i16,
        region: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_set_window_shape_region)(
            c, dest, dest_kind, x_offset, y_offset, region,
        )
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_window_shape_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_window_shape_region(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_window_shape_region)
    }

    /// Sends a `XFixes::SetPictureClipRegion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xfixes_set_picture_clip_region_checked(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        region: xcb_xfixes_region_t,
        x_origin: i16,
        y_origin: i16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_set_picture_clip_region_checked)(
            c, picture, region, x_origin, y_origin,
        )
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_picture_clip_region_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_picture_clip_region_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_picture_clip_region_checked)
    }

    /// Sends a `XFixes::SetPictureClipRegion` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xfixes_set_picture_clip_region(
        &self,
        c: *mut xcb_connection_t,
        picture: xcb_render_picture_t,
        region: xcb_xfixes_region_t,
        x_origin: i16,
        y_origin: i16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_set_picture_clip_region)(c, picture, region, x_origin, y_origin)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_picture_clip_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_picture_clip_region(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_picture_clip_region)
    }

    /// Computes the size of a `xcb_xfixes_set_cursor_name_request_t` object.
    #[inline]
    pub unsafe fn xcb_xfixes_set_cursor_name_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xfixes_set_cursor_name_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_cursor_name_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_cursor_name_sizeof(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_cursor_name_sizeof)
    }

    /// Sends a `XFixes::SetCursorName` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xfixes_set_cursor_name_checked(
        &self,
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
        nbytes: u16,
        name: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_set_cursor_name_checked)(c, cursor, nbytes, name)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_cursor_name_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_cursor_name_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_cursor_name_checked)
    }

    /// Sends a `XFixes::SetCursorName` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xfixes_set_cursor_name(
        &self,
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
        nbytes: u16,
        name: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_set_cursor_name)(c, cursor, nbytes, name)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_cursor_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_cursor_name(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_cursor_name)
    }

    /// Returns a pointer to the `name` field of a `xcb_xfixes_set_cursor_name_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_set_cursor_name_name(
        &self,
        r: *const xcb_xfixes_set_cursor_name_request_t,
    ) -> *mut c_char {
        sym!(self, xcb_xfixes_set_cursor_name_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_cursor_name_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_cursor_name_name(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_cursor_name_name)
    }

    /// Returns the number of elements of the `name` field of a `xcb_xfixes_set_cursor_name_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_set_cursor_name_name_length(
        &self,
        r: *const xcb_xfixes_set_cursor_name_request_t,
    ) -> c_int {
        sym!(self, xcb_xfixes_set_cursor_name_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_cursor_name_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_cursor_name_name_length(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_cursor_name_name_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `name` field of a `xcb_xfixes_set_cursor_name_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_set_cursor_name_name_end(
        &self,
        r: *const xcb_xfixes_set_cursor_name_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xfixes_set_cursor_name_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_set_cursor_name_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_set_cursor_name_name_end(&self) -> bool {
        has_sym!(self, xcb_xfixes_set_cursor_name_name_end)
    }

    /// Computes the size of a `xcb_xfixes_get_cursor_name_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_name_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xfixes_get_cursor_name_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_name_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_name_sizeof(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_name_sizeof)
    }

    /// Sends a `XFixes::GetCursorName` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xfixes_get_cursor_name_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xfixes_get_cursor_name_reply`]: Self::xcb_xfixes_get_cursor_name_reply
    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_name(
        &self,
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
    ) -> xcb_xfixes_get_cursor_name_cookie_t {
        sym!(self, xcb_xfixes_get_cursor_name)(c, cursor)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_name(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_name)
    }

    /// Sends a `XFixes::GetCursorName` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xfixes_get_cursor_name_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xfixes_get_cursor_name_reply`]: Self::xcb_xfixes_get_cursor_name_reply
    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_name_unchecked(
        &self,
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
    ) -> xcb_xfixes_get_cursor_name_cookie_t {
        sym!(self, xcb_xfixes_get_cursor_name_unchecked)(c, cursor)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_name_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_name_unchecked(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_name_unchecked)
    }

    /// Returns a pointer to the `name` field of a `xcb_xfixes_get_cursor_name_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_name_name(
        &self,
        r: *const xcb_xfixes_get_cursor_name_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_xfixes_get_cursor_name_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_name_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_name_name(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_name_name)
    }

    /// Returns the number of elements of the `name` field of a `xcb_xfixes_get_cursor_name_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_name_name_length(
        &self,
        r: *const xcb_xfixes_get_cursor_name_reply_t,
    ) -> c_int {
        sym!(self, xcb_xfixes_get_cursor_name_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_name_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_name_name_length(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_name_name_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `name` field of a `xcb_xfixes_get_cursor_name_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_name_name_end(
        &self,
        r: *const xcb_xfixes_get_cursor_name_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xfixes_get_cursor_name_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_name_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_name_name_end(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_name_name_end)
    }

    /// Waits for the reply to a `XFixes::GetCursorName` request.
    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_name_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xfixes_get_cursor_name_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xfixes_get_cursor_name_reply_t {
        sym!(self, xcb_xfixes_get_cursor_name_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_name_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_name_reply(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_name_reply)
    }

    /// Computes the size of a `xcb_xfixes_get_cursor_image_and_name_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_and_name_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_xfixes_get_cursor_image_and_name_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_and_name_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_and_name_sizeof(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_and_name_sizeof)
    }

    /// Sends a `XFixes::GetCursorImageAndName` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xfixes_get_cursor_image_and_name_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xfixes_get_cursor_image_and_name_reply`]: Self::xcb_xfixes_get_cursor_image_and_name_reply
    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_and_name(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xfixes_get_cursor_image_and_name_cookie_t {
        sym!(self, xcb_xfixes_get_cursor_image_and_name)(c)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_and_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_and_name(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_and_name)
    }

    /// Sends a `XFixes::GetCursorImageAndName` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xfixes_get_cursor_image_and_name_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xfixes_get_cursor_image_and_name_reply`]: Self::xcb_xfixes_get_cursor_image_and_name_reply
    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_and_name_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_xfixes_get_cursor_image_and_name_cookie_t {
        sym!(self, xcb_xfixes_get_cursor_image_and_name_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_and_name_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_and_name_unchecked(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_and_name_unchecked)
    }

    /// Returns a pointer to the `cursor_image` field of a `xcb_xfixes_get_cursor_image_and_name_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_and_name_cursor_image(
        &self,
        r: *const xcb_xfixes_get_cursor_image_and_name_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_xfixes_get_cursor_image_and_name_cursor_image)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_and_name_cursor_image` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_and_name_cursor_image(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_and_name_cursor_image)
    }

    /// Returns the number of elements of the `cursor_image` field of a `xcb_xfixes_get_cursor_image_and_name_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_and_name_cursor_image_length(
        &self,
        r: *const xcb_xfixes_get_cursor_image_and_name_reply_t,
    ) -> c_int {
        sym!(
            self,
            xcb_xfixes_get_cursor_image_and_name_cursor_image_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_and_name_cursor_image_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_and_name_cursor_image_length(&self) -> bool {
        has_sym!(
            self,
            xcb_xfixes_get_cursor_image_and_name_cursor_image_length
        )
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `cursor_image` field of a `xcb_xfixes_get_cursor_image_and_name_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_and_name_cursor_image_end(
        &self,
        r: *const xcb_xfixes_get_cursor_image_and_name_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xfixes_get_cursor_image_and_name_cursor_image_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_and_name_cursor_image_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_and_name_cursor_image_end(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_and_name_cursor_image_end)
    }

    /// Returns a pointer to the `name` field of a `xcb_xfixes_get_cursor_image_and_name_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_and_name_name(
        &self,
        r: *const xcb_xfixes_get_cursor_image_and_name_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_xfixes_get_cursor_image_and_name_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_and_name_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_and_name_name(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_and_name_name)
    }

    /// Returns the number of elements of the `name` field of a `xcb_xfixes_get_cursor_image_and_name_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_and_name_name_length(
        &self,
        r: *const xcb_xfixes_get_cursor_image_and_name_reply_t,
    ) -> c_int {
        sym!(self, xcb_xfixes_get_cursor_image_and_name_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_and_name_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_and_name_name_length(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_and_name_name_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `name` field of a `xcb_xfixes_get_cursor_image_and_name_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_and_name_name_end(
        &self,
        r: *const xcb_xfixes_get_cursor_image_and_name_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xfixes_get_cursor_image_and_name_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_and_name_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_and_name_name_end(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_and_name_name_end)
    }

    /// Waits for the reply to a `XFixes::GetCursorImageAndName` request.
    #[inline]
    pub unsafe fn xcb_xfixes_get_cursor_image_and_name_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xfixes_get_cursor_image_and_name_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xfixes_get_cursor_image_and_name_reply_t {
        sym!(self, xcb_xfixes_get_cursor_image_and_name_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_get_cursor_image_and_name_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_get_cursor_image_and_name_reply(&self) -> bool {
        has_sym!(self, xcb_xfixes_get_cursor_image_and_name_reply)
    }

    /// Sends a `XFixes::ChangeCursor` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xfixes_change_cursor_checked(
        &self,
        c: *mut xcb_connection_t,
        source: xcb_cursor_t,
        destination: xcb_cursor_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_change_cursor_checked)(c, source, destination)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_change_cursor_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_change_cursor_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_change_cursor_checked)
    }

    /// Sends a `XFixes::ChangeCursor` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xfixes_change_cursor(
        &self,
        c: *mut xcb_connection_t,
        source: xcb_cursor_t,
        destination: xcb_cursor_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_change_cursor)(c, source, destination)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_change_cursor` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_change_cursor(&self) -> bool {
        has_sym!(self, xcb_xfixes_change_cursor)
    }

    /// Computes the size of a `xcb_xfixes_change_cursor_by_name_request_t` object.
    #[inline]
    pub unsafe fn xcb_xfixes_change_cursor_by_name_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xfixes_change_cursor_by_name_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_change_cursor_by_name_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_change_cursor_by_name_sizeof(&self) -> bool {
        has_sym!(self, xcb_xfixes_change_cursor_by_name_sizeof)
    }

    /// Sends a `XFixes::ChangeCursorByName` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xfixes_change_cursor_by_name_checked(
        &self,
        c: *mut xcb_connection_t,
        src: xcb_cursor_t,
        nbytes: u16,
        name: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_change_cursor_by_name_checked)(c, src, nbytes, name)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_change_cursor_by_name_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_change_cursor_by_name_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_change_cursor_by_name_checked)
    }

    /// Sends a `XFixes::ChangeCursorByName` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xfixes_change_cursor_by_name(
        &self,
        c: *mut xcb_connection_t,
        src: xcb_cursor_t,
        nbytes: u16,
        name: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_change_cursor_by_name)(c, src, nbytes, name)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_change_cursor_by_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_change_cursor_by_name(&self) -> bool {
        has_sym!(self, xcb_xfixes_change_cursor_by_name)
    }

    /// Returns a pointer to the `name` field of a `xcb_xfixes_change_cursor_by_name_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_change_cursor_by_name_name(
        &self,
        r: *const xcb_xfixes_change_cursor_by_name_request_t,
    ) -> *mut c_char {
        sym!(self, xcb_xfixes_change_cursor_by_name_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_change_cursor_by_name_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_change_cursor_by_name_name(&self) -> bool {
        has_sym!(self, xcb_xfixes_change_cursor_by_name_name)
    }

    /// Returns the number of elements of the `name` field of a `xcb_xfixes_change_cursor_by_name_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_change_cursor_by_name_name_length(
        &self,
        r: *const xcb_xfixes_change_cursor_by_name_request_t,
    ) -> c_int {
        sym!(self, xcb_xfixes_change_cursor_by_name_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_change_cursor_by_name_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_change_cursor_by_name_name_length(&self) -> bool {
        has_sym!(self, xcb_xfixes_change_cursor_by_name_name_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `name` field of a `xcb_xfixes_change_cursor_by_name_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_change_cursor_by_name_name_end(
        &self,
        r: *const xcb_xfixes_change_cursor_by_name_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xfixes_change_cursor_by_name_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_change_cursor_by_name_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_change_cursor_by_name_name_end(&self) -> bool {
        has_sym!(self, xcb_xfixes_change_cursor_by_name_name_end)
    }

    /// Sends a `XFixes::ExpandRegion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xfixes_expand_region_checked(
        &self,
        c: *mut xcb_connection_t,
        source: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
        left: u16,
        right: u16,
        top: u16,
        bottom: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_expand_region_checked)(
            c,
            source,
            destination,
            left,
            right,
            top,
            bottom,
        )
    }

    /// Returns `true` iff the symbol `xcb_xfixes_expand_region_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_expand_region_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_expand_region_checked)
    }

    /// Sends a `XFixes::ExpandRegion` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xfixes_expand_region(
        &self,
        c: *mut xcb_connection_t,
        source: xcb_xfixes_region_t,
        destination: xcb_xfixes_region_t,
        left: u16,
        right: u16,
        top: u16,
        bottom: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_expand_region)(c, source, destination, left, right, top, bottom)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_expand_region` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_expand_region(&self) -> bool {
        has_sym!(self, xcb_xfixes_expand_region)
    }

    /// Sends a `XFixes::HideCursor` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xfixes_hide_cursor_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_hide_cursor_checked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_hide_cursor_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_hide_cursor_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_hide_cursor_checked)
    }

    /// Sends a `XFixes::HideCursor` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xfixes_hide_cursor(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_hide_cursor)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_hide_cursor` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_hide_cursor(&self) -> bool {
        has_sym!(self, xcb_xfixes_hide_cursor)
    }

    /// Sends a `XFixes::ShowCursor` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xfixes_show_cursor_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_show_cursor_checked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_show_cursor_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_show_cursor_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_show_cursor_checked)
    }

    /// Sends a `XFixes::ShowCursor` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xfixes_show_cursor(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_show_cursor)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_show_cursor` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_show_cursor(&self) -> bool {
        has_sym!(self, xcb_xfixes_show_cursor)
    }

    /// Advances a `xcb_xfixes_barrier_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xfixes_barrier_next(&self, i: *mut xcb_xfixes_barrier_iterator_t) {
        sym!(self, xcb_xfixes_barrier_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_barrier_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_barrier_next(&self) -> bool {
        has_sym!(self, xcb_xfixes_barrier_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xfixes_barrier_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xfixes_barrier_end(
        &self,
        i: xcb_xfixes_barrier_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xfixes_barrier_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_barrier_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_barrier_end(&self) -> bool {
        has_sym!(self, xcb_xfixes_barrier_end)
    }

    /// Computes the size of a `xcb_xfixes_create_pointer_barrier_request_t` object.
    #[inline]
    pub unsafe fn xcb_xfixes_create_pointer_barrier_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xfixes_create_pointer_barrier_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_pointer_barrier_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_pointer_barrier_sizeof(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_pointer_barrier_sizeof)
    }

    /// Sends a `XFixes::CreatePointerBarrier` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xfixes_create_pointer_barrier_checked(
        &self,
        c: *mut xcb_connection_t,
        barrier: xcb_xfixes_barrier_t,
        window: xcb_window_t,
        x1: u16,
        y1: u16,
        x2: u16,
        y2: u16,
        directions: u32,
        num_devices: u16,
        devices: *const u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_create_pointer_barrier_checked)(
            c,
            barrier,
            window,
            x1,
            y1,
            x2,
            y2,
            directions,
            num_devices,
            devices,
        )
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_pointer_barrier_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_pointer_barrier_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_pointer_barrier_checked)
    }

    /// Sends a `XFixes::CreatePointerBarrier` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xfixes_create_pointer_barrier(
        &self,
        c: *mut xcb_connection_t,
        barrier: xcb_xfixes_barrier_t,
        window: xcb_window_t,
        x1: u16,
        y1: u16,
        x2: u16,
        y2: u16,
        directions: u32,
        num_devices: u16,
        devices: *const u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_create_pointer_barrier)(
            c,
            barrier,
            window,
            x1,
            y1,
            x2,
            y2,
            directions,
            num_devices,
            devices,
        )
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_pointer_barrier` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_pointer_barrier(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_pointer_barrier)
    }

    /// Returns a pointer to the `devices` field of a `xcb_xfixes_create_pointer_barrier_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_create_pointer_barrier_devices(
        &self,
        r: *const xcb_xfixes_create_pointer_barrier_request_t,
    ) -> *mut u16 {
        sym!(self, xcb_xfixes_create_pointer_barrier_devices)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_pointer_barrier_devices` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_pointer_barrier_devices(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_pointer_barrier_devices)
    }

    /// Returns the number of elements of the `devices` field of a `xcb_xfixes_create_pointer_barrier_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_create_pointer_barrier_devices_length(
        &self,
        r: *const xcb_xfixes_create_pointer_barrier_request_t,
    ) -> c_int {
        sym!(self, xcb_xfixes_create_pointer_barrier_devices_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_pointer_barrier_devices_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_pointer_barrier_devices_length(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_pointer_barrier_devices_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `devices` field of a `xcb_xfixes_create_pointer_barrier_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xfixes_create_pointer_barrier_devices_end(
        &self,
        r: *const xcb_xfixes_create_pointer_barrier_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xfixes_create_pointer_barrier_devices_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_create_pointer_barrier_devices_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_create_pointer_barrier_devices_end(&self) -> bool {
        has_sym!(self, xcb_xfixes_create_pointer_barrier_devices_end)
    }

    /// Sends a `XFixes::DeletePointerBarrier` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xfixes_delete_pointer_barrier_checked(
        &self,
        c: *mut xcb_connection_t,
        barrier: xcb_xfixes_barrier_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_delete_pointer_barrier_checked)(c, barrier)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_delete_pointer_barrier_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_delete_pointer_barrier_checked(&self) -> bool {
        has_sym!(self, xcb_xfixes_delete_pointer_barrier_checked)
    }

    /// Sends a `XFixes::DeletePointerBarrier` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xfixes_delete_pointer_barrier(
        &self,
        c: *mut xcb_connection_t,
        barrier: xcb_xfixes_barrier_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xfixes_delete_pointer_barrier)(c, barrier)
    }

    /// Returns `true` iff the symbol `xcb_xfixes_delete_pointer_barrier` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xfixes_delete_pointer_barrier(&self) -> bool {
        has_sym!(self, xcb_xfixes_delete_pointer_barrier)
    }
}

#[cfg(feature = "xcb_xfixes")]
#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbXfixes::load().unwrap() };
        assert!(lib.has_xcb_xfixes_id());
        assert!(lib.has_xcb_xfixes_query_version());
        assert!(lib.has_xcb_xfixes_query_version_unchecked());
        assert!(lib.has_xcb_xfixes_query_version_reply());
        assert!(lib.has_xcb_xfixes_change_save_set_checked());
        assert!(lib.has_xcb_xfixes_change_save_set());
        assert!(lib.has_xcb_xfixes_select_selection_input_checked());
        assert!(lib.has_xcb_xfixes_select_selection_input());
        assert!(lib.has_xcb_xfixes_select_cursor_input_checked());
        assert!(lib.has_xcb_xfixes_select_cursor_input());
        assert!(lib.has_xcb_xfixes_get_cursor_image_sizeof());
        assert!(lib.has_xcb_xfixes_get_cursor_image());
        assert!(lib.has_xcb_xfixes_get_cursor_image_unchecked());
        assert!(lib.has_xcb_xfixes_get_cursor_image_cursor_image());
        assert!(lib.has_xcb_xfixes_get_cursor_image_cursor_image_length());
        assert!(lib.has_xcb_xfixes_get_cursor_image_cursor_image_end());
        assert!(lib.has_xcb_xfixes_get_cursor_image_reply());
        assert!(lib.has_xcb_xfixes_region_next());
        assert!(lib.has_xcb_xfixes_region_end());
        assert!(lib.has_xcb_xfixes_create_region_sizeof());
        assert!(lib.has_xcb_xfixes_create_region_checked());
        assert!(lib.has_xcb_xfixes_create_region());
        assert!(lib.has_xcb_xfixes_create_region_rectangles());
        assert!(lib.has_xcb_xfixes_create_region_rectangles_length());
        assert!(lib.has_xcb_xfixes_create_region_rectangles_iterator());
        assert!(lib.has_xcb_xfixes_create_region_from_bitmap_checked());
        assert!(lib.has_xcb_xfixes_create_region_from_bitmap());
        assert!(lib.has_xcb_xfixes_create_region_from_window_checked());
        assert!(lib.has_xcb_xfixes_create_region_from_window());
        assert!(lib.has_xcb_xfixes_create_region_from_gc_checked());
        assert!(lib.has_xcb_xfixes_create_region_from_gc());
        assert!(lib.has_xcb_xfixes_create_region_from_picture_checked());
        assert!(lib.has_xcb_xfixes_create_region_from_picture());
        assert!(lib.has_xcb_xfixes_destroy_region_checked());
        assert!(lib.has_xcb_xfixes_destroy_region());
        assert!(lib.has_xcb_xfixes_set_region_sizeof());
        assert!(lib.has_xcb_xfixes_set_region_checked());
        assert!(lib.has_xcb_xfixes_set_region());
        assert!(lib.has_xcb_xfixes_set_region_rectangles());
        assert!(lib.has_xcb_xfixes_set_region_rectangles_length());
        assert!(lib.has_xcb_xfixes_set_region_rectangles_iterator());
        assert!(lib.has_xcb_xfixes_copy_region_checked());
        assert!(lib.has_xcb_xfixes_copy_region());
        assert!(lib.has_xcb_xfixes_union_region_checked());
        assert!(lib.has_xcb_xfixes_union_region());
        assert!(lib.has_xcb_xfixes_intersect_region_checked());
        assert!(lib.has_xcb_xfixes_intersect_region());
        assert!(lib.has_xcb_xfixes_subtract_region_checked());
        assert!(lib.has_xcb_xfixes_subtract_region());
        assert!(lib.has_xcb_xfixes_invert_region_checked());
        assert!(lib.has_xcb_xfixes_invert_region());
        assert!(lib.has_xcb_xfixes_translate_region_checked());
        assert!(lib.has_xcb_xfixes_translate_region());
        assert!(lib.has_xcb_xfixes_region_extents_checked());
        assert!(lib.has_xcb_xfixes_region_extents());
        assert!(lib.has_xcb_xfixes_fetch_region_sizeof());
        assert!(lib.has_xcb_xfixes_fetch_region());
        assert!(lib.has_xcb_xfixes_fetch_region_unchecked());
        assert!(lib.has_xcb_xfixes_fetch_region_rectangles());
        assert!(lib.has_xcb_xfixes_fetch_region_rectangles_length());
        assert!(lib.has_xcb_xfixes_fetch_region_rectangles_iterator());
        assert!(lib.has_xcb_xfixes_fetch_region_reply());
        assert!(lib.has_xcb_xfixes_set_gc_clip_region_checked());
        assert!(lib.has_xcb_xfixes_set_gc_clip_region());
        assert!(lib.has_xcb_xfixes_set_window_shape_region_checked());
        assert!(lib.has_xcb_xfixes_set_window_shape_region());
        assert!(lib.has_xcb_xfixes_set_picture_clip_region_checked());
        assert!(lib.has_xcb_xfixes_set_picture_clip_region());
        assert!(lib.has_xcb_xfixes_set_cursor_name_sizeof());
        assert!(lib.has_xcb_xfixes_set_cursor_name_checked());
        assert!(lib.has_xcb_xfixes_set_cursor_name());
        assert!(lib.has_xcb_xfixes_set_cursor_name_name());
        assert!(lib.has_xcb_xfixes_set_cursor_name_name_length());
        assert!(lib.has_xcb_xfixes_set_cursor_name_name_end());
        assert!(lib.has_xcb_xfixes_get_cursor_name_sizeof());
        assert!(lib.has_xcb_xfixes_get_cursor_name());
        assert!(lib.has_xcb_xfixes_get_cursor_name_unchecked());
        assert!(lib.has_xcb_xfixes_get_cursor_name_name());
        assert!(lib.has_xcb_xfixes_get_cursor_name_name_length());
        assert!(lib.has_xcb_xfixes_get_cursor_name_name_end());
        assert!(lib.has_xcb_xfixes_get_cursor_name_reply());
        assert!(lib.has_xcb_xfixes_get_cursor_image_and_name_sizeof());
        assert!(lib.has_xcb_xfixes_get_cursor_image_and_name());
        assert!(lib.has_xcb_xfixes_get_cursor_image_and_name_unchecked());
        assert!(lib.has_xcb_xfixes_get_cursor_image_and_name_cursor_image());
        assert!(lib.has_xcb_xfixes_get_cursor_image_and_name_cursor_image_length());
        assert!(lib.has_xcb_xfixes_get_cursor_image_and_name_cursor_image_end());
        assert!(lib.has_xcb_xfixes_get_cursor_image_and_name_name());
        assert!(lib.has_xcb_xfixes_get_cursor_image_and_name_name_length());
        assert!(lib.has_xcb_xfixes_get_cursor_image_and_name_name_end());
        assert!(lib.has_xcb_xfixes_get_cursor_image_and_name_reply());
        assert!(lib.has_xcb_xfixes_change_cursor_checked());
        assert!(lib.has_xcb_xfixes_change_cursor());
        assert!(lib.has_xcb_xfixes_change_cursor_by_name_sizeof());
        assert!(lib.has_xcb_xfixes_change_cursor_by_name_checked());
        assert!(lib.has_xcb_xfixes_change_cursor_by_name());
        assert!(lib.has_xcb_xfixes_change_cursor_by_name_name());
        assert!(lib.has_xcb_xfixes_change_cursor_by_name_name_length());
        assert!(lib.has_xcb_xfixes_change_cursor_by_name_name_end());
        assert!(lib.has_xcb_xfixes_expand_region_checked());
        assert!(lib.has_xcb_xfixes_expand_region());
        assert!(lib.has_xcb_xfixes_hide_cursor_checked());
        assert!(lib.has_xcb_xfixes_hide_cursor());
        assert!(lib.has_xcb_xfixes_show_cursor_checked());
        assert!(lib.has_xcb_xfixes_show_cursor());
        assert!(lib.has_xcb_xfixes_barrier_next());
        assert!(lib.has_xcb_xfixes_barrier_end());
        assert!(lib.has_xcb_xfixes_create_pointer_barrier_sizeof());
        assert!(lib.has_xcb_xfixes_create_pointer_barrier_checked());
        assert!(lib.has_xcb_xfixes_create_pointer_barrier());
        assert!(lib.has_xcb_xfixes_create_pointer_barrier_devices());
        assert!(lib.has_xcb_xfixes_create_pointer_barrier_devices_length());
        assert!(lib.has_xcb_xfixes_create_pointer_barrier_devices_end());
        assert!(lib.has_xcb_xfixes_delete_pointer_barrier_checked());
        assert!(lib.has_xcb_xfixes_delete_pointer_barrier());
    }
}
