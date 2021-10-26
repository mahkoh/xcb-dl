// This file was generated using generate.py. Do not edit.
#![allow(unused_macros)]

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

/// The `CHAR2B` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_char2b_t {
    pub byte1: u8,
    pub byte2: u8,
}

impl Default for xcb_char2b_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `CHAR2B` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_char2b_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_char2b_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_char2b_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `WINDOW` type.
pub type xcb_window_t = u32;

/// An iterator over `WINDOW` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_window_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_window_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_window_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `PIXMAP` type.
pub type xcb_pixmap_t = u32;

/// An iterator over `PIXMAP` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_pixmap_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_pixmap_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_pixmap_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `CURSOR` type.
pub type xcb_cursor_t = u32;

/// An iterator over `CURSOR` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_cursor_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_cursor_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_cursor_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `FONT` type.
pub type xcb_font_t = u32;

/// An iterator over `FONT` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_font_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_font_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_font_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `GCONTEXT` type.
pub type xcb_gcontext_t = u32;

/// An iterator over `GCONTEXT` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_gcontext_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_gcontext_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_gcontext_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `COLORMAP` type.
pub type xcb_colormap_t = u32;

/// An iterator over `COLORMAP` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_colormap_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_colormap_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_colormap_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `ATOM` type.
pub type xcb_atom_t = u32;

/// An iterator over `ATOM` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_atom_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_atom_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_atom_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `DRAWABLE` type.
pub type xcb_drawable_t = u32;

/// An iterator over `DRAWABLE` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_drawable_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_drawable_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_drawable_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `FONTABLE` type.
pub type xcb_fontable_t = u32;

/// An iterator over `FONTABLE` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_fontable_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_fontable_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_fontable_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `BOOL32` type.
pub type xcb_bool32_t = u32;

/// An iterator over `BOOL32` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_bool32_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_bool32_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_bool32_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `VISUALID` type.
pub type xcb_visualid_t = u32;

/// An iterator over `VISUALID` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_visualid_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_visualid_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_visualid_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `TIMESTAMP` type.
pub type xcb_timestamp_t = u32;

/// An iterator over `TIMESTAMP` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_timestamp_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_timestamp_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_timestamp_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `KEYSYM` type.
pub type xcb_keysym_t = u32;

/// An iterator over `KEYSYM` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_keysym_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_keysym_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_keysym_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `KEYCODE` type.
pub type xcb_keycode_t = u8;

/// An iterator over `KEYCODE` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_keycode_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_keycode_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_keycode_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `KEYCODE32` type.
pub type xcb_keycode32_t = u32;

/// An iterator over `KEYCODE32` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_keycode32_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_keycode32_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_keycode32_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `BUTTON` type.
pub type xcb_button_t = u8;

/// An iterator over `BUTTON` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_button_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_button_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_button_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `POINT` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_point_t {
    pub x: i16,
    pub y: i16,
}

impl Default for xcb_point_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `POINT` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_point_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_point_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_point_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RECTANGLE` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_rectangle_t {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}

impl Default for xcb_rectangle_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `RECTANGLE` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_rectangle_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_rectangle_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_rectangle_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `ARC` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_arc_t {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub angle1: i16,
    pub angle2: i16,
}

impl Default for xcb_arc_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `ARC` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_arc_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_arc_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_arc_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `FORMAT` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_format_t {
    pub depth: u8,
    pub bits_per_pixel: u8,
    pub scanline_pad: u8,
    pub pad0: [u8; 5],
}

impl Default for xcb_format_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `FORMAT` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_format_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_format_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_format_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `VisualClass` enum.
///
/// This enum has the following variants:
///
/// - [`VisualClass::StaticGray`](XCB_VISUAL_CLASS_STATIC_GRAY)
/// - [`VisualClass::GrayScale`](XCB_VISUAL_CLASS_GRAY_SCALE)
/// - [`VisualClass::StaticColor`](XCB_VISUAL_CLASS_STATIC_COLOR)
/// - [`VisualClass::PseudoColor`](XCB_VISUAL_CLASS_PSEUDO_COLOR)
/// - [`VisualClass::TrueColor`](XCB_VISUAL_CLASS_TRUE_COLOR)
/// - [`VisualClass::DirectColor`](XCB_VISUAL_CLASS_DIRECT_COLOR)
pub type xcb_visual_class_t = u32;
/// The `VisualClass::StaticGray` enum variant.
///
/// This is a variant of [`xcb_visual_class_t`].
pub const XCB_VISUAL_CLASS_STATIC_GRAY: xcb_visual_class_t = 0;
/// The `VisualClass::GrayScale` enum variant.
///
/// This is a variant of [`xcb_visual_class_t`].
pub const XCB_VISUAL_CLASS_GRAY_SCALE: xcb_visual_class_t = 1;
/// The `VisualClass::StaticColor` enum variant.
///
/// This is a variant of [`xcb_visual_class_t`].
pub const XCB_VISUAL_CLASS_STATIC_COLOR: xcb_visual_class_t = 2;
/// The `VisualClass::PseudoColor` enum variant.
///
/// This is a variant of [`xcb_visual_class_t`].
pub const XCB_VISUAL_CLASS_PSEUDO_COLOR: xcb_visual_class_t = 3;
/// The `VisualClass::TrueColor` enum variant.
///
/// This is a variant of [`xcb_visual_class_t`].
pub const XCB_VISUAL_CLASS_TRUE_COLOR: xcb_visual_class_t = 4;
/// The `VisualClass::DirectColor` enum variant.
///
/// This is a variant of [`xcb_visual_class_t`].
pub const XCB_VISUAL_CLASS_DIRECT_COLOR: xcb_visual_class_t = 5;

/// The `VISUALTYPE` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_visualtype_t {
    pub visual_id: xcb_visualid_t,
    pub class: u8,
    pub bits_per_rgb_value: u8,
    pub colormap_entries: u16,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub pad0: [u8; 4],
}

impl Default for xcb_visualtype_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `VISUALTYPE` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_visualtype_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_visualtype_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_visualtype_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `DEPTH` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `visuals`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_depth_t {
    pub depth: u8,
    pub pad0: u8,
    pub visuals_len: u16,
    pub pad1: [u8; 4],
}

impl Default for xcb_depth_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `DEPTH` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_depth_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_depth_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_depth_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `EventMask` enum.
///
/// This enum has the following variants:
///
/// - [`EventMask::NoEvent`](XCB_EVENT_MASK_NO_EVENT)
/// - [`EventMask::KeyPress`](XCB_EVENT_MASK_KEY_PRESS)
/// - [`EventMask::KeyRelease`](XCB_EVENT_MASK_KEY_RELEASE)
/// - [`EventMask::ButtonPress`](XCB_EVENT_MASK_BUTTON_PRESS)
/// - [`EventMask::ButtonRelease`](XCB_EVENT_MASK_BUTTON_RELEASE)
/// - [`EventMask::EnterWindow`](XCB_EVENT_MASK_ENTER_WINDOW)
/// - [`EventMask::LeaveWindow`](XCB_EVENT_MASK_LEAVE_WINDOW)
/// - [`EventMask::PointerMotion`](XCB_EVENT_MASK_POINTER_MOTION)
/// - [`EventMask::PointerMotionHint`](XCB_EVENT_MASK_POINTER_MOTION_HINT)
/// - [`EventMask::Button1Motion`](XCB_EVENT_MASK_BUTTON_1_MOTION)
/// - [`EventMask::Button2Motion`](XCB_EVENT_MASK_BUTTON_2_MOTION)
/// - [`EventMask::Button3Motion`](XCB_EVENT_MASK_BUTTON_3_MOTION)
/// - [`EventMask::Button4Motion`](XCB_EVENT_MASK_BUTTON_4_MOTION)
/// - [`EventMask::Button5Motion`](XCB_EVENT_MASK_BUTTON_5_MOTION)
/// - [`EventMask::ButtonMotion`](XCB_EVENT_MASK_BUTTON_MOTION)
/// - [`EventMask::KeymapState`](XCB_EVENT_MASK_KEYMAP_STATE)
/// - [`EventMask::Exposure`](XCB_EVENT_MASK_EXPOSURE)
/// - [`EventMask::VisibilityChange`](XCB_EVENT_MASK_VISIBILITY_CHANGE)
/// - [`EventMask::StructureNotify`](XCB_EVENT_MASK_STRUCTURE_NOTIFY)
/// - [`EventMask::ResizeRedirect`](XCB_EVENT_MASK_RESIZE_REDIRECT)
/// - [`EventMask::SubstructureNotify`](XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY)
/// - [`EventMask::SubstructureRedirect`](XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT)
/// - [`EventMask::FocusChange`](XCB_EVENT_MASK_FOCUS_CHANGE)
/// - [`EventMask::PropertyChange`](XCB_EVENT_MASK_PROPERTY_CHANGE)
/// - [`EventMask::ColorMapChange`](XCB_EVENT_MASK_COLOR_MAP_CHANGE)
/// - [`EventMask::OwnerGrabButton`](XCB_EVENT_MASK_OWNER_GRAB_BUTTON)
pub type xcb_event_mask_t = u32;
/// The `EventMask::NoEvent` enum variant.
///
/// This is a variant of [`xcb_event_mask_t`].
pub const XCB_EVENT_MASK_NO_EVENT: xcb_event_mask_t = 0;
/// The `EventMask::KeyPress` enum variant.
///
/// This is a variant of [`xcb_event_mask_t`].
pub const XCB_EVENT_MASK_KEY_PRESS: xcb_event_mask_t = 1;
/// The `EventMask::KeyRelease` enum variant.
///
/// This is a variant of [`xcb_event_mask_t`].
pub const XCB_EVENT_MASK_KEY_RELEASE: xcb_event_mask_t = 2;
/// The `EventMask::ButtonPress` enum variant.
///
/// This is a variant of [`xcb_event_mask_t`].
pub const XCB_EVENT_MASK_BUTTON_PRESS: xcb_event_mask_t = 4;
/// The `EventMask::ButtonRelease` enum variant.
///
/// This is a variant of [`xcb_event_mask_t`].
pub const XCB_EVENT_MASK_BUTTON_RELEASE: xcb_event_mask_t = 8;
/// The `EventMask::EnterWindow` enum variant.
///
/// This is a variant of [`xcb_event_mask_t`].
pub const XCB_EVENT_MASK_ENTER_WINDOW: xcb_event_mask_t = 16;
/// The `EventMask::LeaveWindow` enum variant.
///
/// This is a variant of [`xcb_event_mask_t`].
pub const XCB_EVENT_MASK_LEAVE_WINDOW: xcb_event_mask_t = 32;
/// The `EventMask::PointerMotion` enum variant.
///
/// This is a variant of [`xcb_event_mask_t`].
pub const XCB_EVENT_MASK_POINTER_MOTION: xcb_event_mask_t = 64;
/// The `EventMask::PointerMotionHint` enum variant.
///
/// This is a variant of [`xcb_event_mask_t`].
pub const XCB_EVENT_MASK_POINTER_MOTION_HINT: xcb_event_mask_t = 128;
/// The `EventMask::Button1Motion` enum variant.
///
/// This is a variant of [`xcb_event_mask_t`].
pub const XCB_EVENT_MASK_BUTTON_1_MOTION: xcb_event_mask_t = 256;
/// The `EventMask::Button2Motion` enum variant.
///
/// This is a variant of [`xcb_event_mask_t`].
pub const XCB_EVENT_MASK_BUTTON_2_MOTION: xcb_event_mask_t = 512;
/// The `EventMask::Button3Motion` enum variant.
///
/// This is a variant of [`xcb_event_mask_t`].
pub const XCB_EVENT_MASK_BUTTON_3_MOTION: xcb_event_mask_t = 1024;
/// The `EventMask::Button4Motion` enum variant.
///
/// This is a variant of [`xcb_event_mask_t`].
pub const XCB_EVENT_MASK_BUTTON_4_MOTION: xcb_event_mask_t = 2048;
/// The `EventMask::Button5Motion` enum variant.
///
/// This is a variant of [`xcb_event_mask_t`].
pub const XCB_EVENT_MASK_BUTTON_5_MOTION: xcb_event_mask_t = 4096;
/// The `EventMask::ButtonMotion` enum variant.
///
/// This is a variant of [`xcb_event_mask_t`].
pub const XCB_EVENT_MASK_BUTTON_MOTION: xcb_event_mask_t = 8192;
/// The `EventMask::KeymapState` enum variant.
///
/// This is a variant of [`xcb_event_mask_t`].
pub const XCB_EVENT_MASK_KEYMAP_STATE: xcb_event_mask_t = 16384;
/// The `EventMask::Exposure` enum variant.
///
/// This is a variant of [`xcb_event_mask_t`].
pub const XCB_EVENT_MASK_EXPOSURE: xcb_event_mask_t = 32768;
/// The `EventMask::VisibilityChange` enum variant.
///
/// This is a variant of [`xcb_event_mask_t`].
pub const XCB_EVENT_MASK_VISIBILITY_CHANGE: xcb_event_mask_t = 65536;
/// The `EventMask::StructureNotify` enum variant.
///
/// This is a variant of [`xcb_event_mask_t`].
pub const XCB_EVENT_MASK_STRUCTURE_NOTIFY: xcb_event_mask_t = 131072;
/// The `EventMask::ResizeRedirect` enum variant.
///
/// This is a variant of [`xcb_event_mask_t`].
pub const XCB_EVENT_MASK_RESIZE_REDIRECT: xcb_event_mask_t = 262144;
/// The `EventMask::SubstructureNotify` enum variant.
///
/// This is a variant of [`xcb_event_mask_t`].
pub const XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY: xcb_event_mask_t = 524288;
/// The `EventMask::SubstructureRedirect` enum variant.
///
/// This is a variant of [`xcb_event_mask_t`].
pub const XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT: xcb_event_mask_t = 1048576;
/// The `EventMask::FocusChange` enum variant.
///
/// This is a variant of [`xcb_event_mask_t`].
pub const XCB_EVENT_MASK_FOCUS_CHANGE: xcb_event_mask_t = 2097152;
/// The `EventMask::PropertyChange` enum variant.
///
/// This is a variant of [`xcb_event_mask_t`].
pub const XCB_EVENT_MASK_PROPERTY_CHANGE: xcb_event_mask_t = 4194304;
/// The `EventMask::ColorMapChange` enum variant.
///
/// This is a variant of [`xcb_event_mask_t`].
pub const XCB_EVENT_MASK_COLOR_MAP_CHANGE: xcb_event_mask_t = 8388608;
/// The `EventMask::OwnerGrabButton` enum variant.
///
/// This is a variant of [`xcb_event_mask_t`].
pub const XCB_EVENT_MASK_OWNER_GRAB_BUTTON: xcb_event_mask_t = 16777216;

/// The `BackingStore` enum.
///
/// This enum has the following variants:
///
/// - [`BackingStore::NotUseful`](XCB_BACKING_STORE_NOT_USEFUL)
/// - [`BackingStore::WhenMapped`](XCB_BACKING_STORE_WHEN_MAPPED)
/// - [`BackingStore::Always`](XCB_BACKING_STORE_ALWAYS)
pub type xcb_backing_store_t = u32;
/// The `BackingStore::NotUseful` enum variant.
///
/// This is a variant of [`xcb_backing_store_t`].
pub const XCB_BACKING_STORE_NOT_USEFUL: xcb_backing_store_t = 0;
/// The `BackingStore::WhenMapped` enum variant.
///
/// This is a variant of [`xcb_backing_store_t`].
pub const XCB_BACKING_STORE_WHEN_MAPPED: xcb_backing_store_t = 1;
/// The `BackingStore::Always` enum variant.
///
/// This is a variant of [`xcb_backing_store_t`].
pub const XCB_BACKING_STORE_ALWAYS: xcb_backing_store_t = 2;

/// The `SCREEN` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `allowed_depths`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screen_t {
    pub root: xcb_window_t,
    pub default_colormap: xcb_colormap_t,
    pub white_pixel: u32,
    pub black_pixel: u32,
    pub current_input_masks: u32,
    pub width_in_pixels: u16,
    pub height_in_pixels: u16,
    pub width_in_millimeters: u16,
    pub height_in_millimeters: u16,
    pub min_installed_maps: u16,
    pub max_installed_maps: u16,
    pub root_visual: xcb_visualid_t,
    pub backing_stores: u8,
    pub save_unders: u8,
    pub root_depth: u8,
    pub allowed_depths_len: u8,
}

impl Default for xcb_screen_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `SCREEN` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screen_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_screen_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_screen_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `SetupRequest` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `authorization_protocol_name`
/// - `authorization_protocol_data`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_setup_request_t {
    pub byte_order: u8,
    pub pad0: u8,
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub authorization_protocol_name_len: u16,
    pub authorization_protocol_data_len: u16,
    pub pad1: [u8; 2],
}

impl Default for xcb_setup_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `SetupRequest` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_setup_request_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_setup_request_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_setup_request_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `SetupFailed` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `reason`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_setup_failed_t {
    pub status: u8,
    pub reason_len: u8,
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub length: u16,
}

impl Default for xcb_setup_failed_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `SetupFailed` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_setup_failed_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_setup_failed_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_setup_failed_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `SetupAuthenticate` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `reason`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_setup_authenticate_t {
    pub status: u8,
    pub pad0: [u8; 5],
    pub length: u16,
}

impl Default for xcb_setup_authenticate_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `SetupAuthenticate` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_setup_authenticate_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_setup_authenticate_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_setup_authenticate_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `ImageOrder` enum.
///
/// This enum has the following variants:
///
/// - [`ImageOrder::LSBFirst`](XCB_IMAGE_ORDER_LSB_FIRST)
/// - [`ImageOrder::MSBFirst`](XCB_IMAGE_ORDER_MSB_FIRST)
pub type xcb_image_order_t = u32;
/// The `ImageOrder::LSBFirst` enum variant.
///
/// This is a variant of [`xcb_image_order_t`].
pub const XCB_IMAGE_ORDER_LSB_FIRST: xcb_image_order_t = 0;
/// The `ImageOrder::MSBFirst` enum variant.
///
/// This is a variant of [`xcb_image_order_t`].
pub const XCB_IMAGE_ORDER_MSB_FIRST: xcb_image_order_t = 1;

/// The `Setup` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `vendor`
/// - `pixmap_formats`
/// - `roots`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_setup_t {
    pub status: u8,
    pub pad0: u8,
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub length: u16,
    pub release_number: u32,
    pub resource_id_base: u32,
    pub resource_id_mask: u32,
    pub motion_buffer_size: u32,
    pub vendor_len: u16,
    pub maximum_request_length: u16,
    pub roots_len: u8,
    pub pixmap_formats_len: u8,
    pub image_byte_order: u8,
    pub bitmap_format_bit_order: u8,
    pub bitmap_format_scanline_unit: u8,
    pub bitmap_format_scanline_pad: u8,
    pub min_keycode: xcb_keycode_t,
    pub max_keycode: xcb_keycode_t,
    pub pad1: [u8; 4],
}

impl Default for xcb_setup_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Setup` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_setup_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_setup_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_setup_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `ModMask` enum.
///
/// This enum has the following variants:
///
/// - [`ModMask::Shift`](XCB_MOD_MASK_SHIFT)
/// - [`ModMask::Lock`](XCB_MOD_MASK_LOCK)
/// - [`ModMask::Control`](XCB_MOD_MASK_CONTROL)
/// - [`ModMask::1`](XCB_MOD_MASK_1)
/// - [`ModMask::2`](XCB_MOD_MASK_2)
/// - [`ModMask::3`](XCB_MOD_MASK_3)
/// - [`ModMask::4`](XCB_MOD_MASK_4)
/// - [`ModMask::5`](XCB_MOD_MASK_5)
/// - [`ModMask::Any`](XCB_MOD_MASK_ANY)
pub type xcb_mod_mask_t = u32;
/// The `ModMask::Shift` enum variant.
///
/// This is a variant of [`xcb_mod_mask_t`].
pub const XCB_MOD_MASK_SHIFT: xcb_mod_mask_t = 1;
/// The `ModMask::Lock` enum variant.
///
/// This is a variant of [`xcb_mod_mask_t`].
pub const XCB_MOD_MASK_LOCK: xcb_mod_mask_t = 2;
/// The `ModMask::Control` enum variant.
///
/// This is a variant of [`xcb_mod_mask_t`].
pub const XCB_MOD_MASK_CONTROL: xcb_mod_mask_t = 4;
/// The `ModMask::1` enum variant.
///
/// This is a variant of [`xcb_mod_mask_t`].
pub const XCB_MOD_MASK_1: xcb_mod_mask_t = 8;
/// The `ModMask::2` enum variant.
///
/// This is a variant of [`xcb_mod_mask_t`].
pub const XCB_MOD_MASK_2: xcb_mod_mask_t = 16;
/// The `ModMask::3` enum variant.
///
/// This is a variant of [`xcb_mod_mask_t`].
pub const XCB_MOD_MASK_3: xcb_mod_mask_t = 32;
/// The `ModMask::4` enum variant.
///
/// This is a variant of [`xcb_mod_mask_t`].
pub const XCB_MOD_MASK_4: xcb_mod_mask_t = 64;
/// The `ModMask::5` enum variant.
///
/// This is a variant of [`xcb_mod_mask_t`].
pub const XCB_MOD_MASK_5: xcb_mod_mask_t = 128;
/// The `ModMask::Any` enum variant.
///
/// This is a variant of [`xcb_mod_mask_t`].
pub const XCB_MOD_MASK_ANY: xcb_mod_mask_t = 32768;

/// The `KeyButMask` enum.
///
/// This enum has the following variants:
///
/// - [`KeyButMask::Shift`](XCB_KEY_BUT_MASK_SHIFT)
/// - [`KeyButMask::Lock`](XCB_KEY_BUT_MASK_LOCK)
/// - [`KeyButMask::Control`](XCB_KEY_BUT_MASK_CONTROL)
/// - [`KeyButMask::Mod1`](XCB_KEY_BUT_MASK_MOD_1)
/// - [`KeyButMask::Mod2`](XCB_KEY_BUT_MASK_MOD_2)
/// - [`KeyButMask::Mod3`](XCB_KEY_BUT_MASK_MOD_3)
/// - [`KeyButMask::Mod4`](XCB_KEY_BUT_MASK_MOD_4)
/// - [`KeyButMask::Mod5`](XCB_KEY_BUT_MASK_MOD_5)
/// - [`KeyButMask::Button1`](XCB_KEY_BUT_MASK_BUTTON_1)
/// - [`KeyButMask::Button2`](XCB_KEY_BUT_MASK_BUTTON_2)
/// - [`KeyButMask::Button3`](XCB_KEY_BUT_MASK_BUTTON_3)
/// - [`KeyButMask::Button4`](XCB_KEY_BUT_MASK_BUTTON_4)
/// - [`KeyButMask::Button5`](XCB_KEY_BUT_MASK_BUTTON_5)
pub type xcb_key_but_mask_t = u32;
/// The `KeyButMask::Shift` enum variant.
///
/// This is a variant of [`xcb_key_but_mask_t`].
pub const XCB_KEY_BUT_MASK_SHIFT: xcb_key_but_mask_t = 1;
/// The `KeyButMask::Lock` enum variant.
///
/// This is a variant of [`xcb_key_but_mask_t`].
pub const XCB_KEY_BUT_MASK_LOCK: xcb_key_but_mask_t = 2;
/// The `KeyButMask::Control` enum variant.
///
/// This is a variant of [`xcb_key_but_mask_t`].
pub const XCB_KEY_BUT_MASK_CONTROL: xcb_key_but_mask_t = 4;
/// The `KeyButMask::Mod1` enum variant.
///
/// This is a variant of [`xcb_key_but_mask_t`].
pub const XCB_KEY_BUT_MASK_MOD_1: xcb_key_but_mask_t = 8;
/// The `KeyButMask::Mod2` enum variant.
///
/// This is a variant of [`xcb_key_but_mask_t`].
pub const XCB_KEY_BUT_MASK_MOD_2: xcb_key_but_mask_t = 16;
/// The `KeyButMask::Mod3` enum variant.
///
/// This is a variant of [`xcb_key_but_mask_t`].
pub const XCB_KEY_BUT_MASK_MOD_3: xcb_key_but_mask_t = 32;
/// The `KeyButMask::Mod4` enum variant.
///
/// This is a variant of [`xcb_key_but_mask_t`].
pub const XCB_KEY_BUT_MASK_MOD_4: xcb_key_but_mask_t = 64;
/// The `KeyButMask::Mod5` enum variant.
///
/// This is a variant of [`xcb_key_but_mask_t`].
pub const XCB_KEY_BUT_MASK_MOD_5: xcb_key_but_mask_t = 128;
/// The `KeyButMask::Button1` enum variant.
///
/// This is a variant of [`xcb_key_but_mask_t`].
pub const XCB_KEY_BUT_MASK_BUTTON_1: xcb_key_but_mask_t = 256;
/// The `KeyButMask::Button2` enum variant.
///
/// This is a variant of [`xcb_key_but_mask_t`].
pub const XCB_KEY_BUT_MASK_BUTTON_2: xcb_key_but_mask_t = 512;
/// The `KeyButMask::Button3` enum variant.
///
/// This is a variant of [`xcb_key_but_mask_t`].
pub const XCB_KEY_BUT_MASK_BUTTON_3: xcb_key_but_mask_t = 1024;
/// The `KeyButMask::Button4` enum variant.
///
/// This is a variant of [`xcb_key_but_mask_t`].
pub const XCB_KEY_BUT_MASK_BUTTON_4: xcb_key_but_mask_t = 2048;
/// The `KeyButMask::Button5` enum variant.
///
/// This is a variant of [`xcb_key_but_mask_t`].
pub const XCB_KEY_BUT_MASK_BUTTON_5: xcb_key_but_mask_t = 4096;

/// The `Window` enum.
///
/// This enum has the following variants:
///
/// - [`Window::None`](XCB_WINDOW_NONE)
pub type xcb_window_enum_t = u32;
/// The `Window::None` enum variant.
///
/// This is a variant of [`xcb_window_enum_t`].
pub const XCB_WINDOW_NONE: xcb_window_enum_t = 0;

/// The opcode for `KeyPress` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_key_press_event_t`].
pub const XCB_KEY_PRESS: u8 = 2i32 as u8;

/// The `KeyPress` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_key_press_event_t {
    pub response_type: u8,
    pub detail: xcb_keycode_t,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: u8,
    pub pad0: u8,
}

impl Default for xcb_key_press_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `KeyRelease` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_key_release_event_t`].
pub const XCB_KEY_RELEASE: u8 = 3i32 as u8;

/// The `KeyRelease` event.
pub type xcb_key_release_event_t = xcb_key_press_event_t;

/// The `ButtonMask` enum.
///
/// This enum has the following variants:
///
/// - [`ButtonMask::1`](XCB_BUTTON_MASK_1)
/// - [`ButtonMask::2`](XCB_BUTTON_MASK_2)
/// - [`ButtonMask::3`](XCB_BUTTON_MASK_3)
/// - [`ButtonMask::4`](XCB_BUTTON_MASK_4)
/// - [`ButtonMask::5`](XCB_BUTTON_MASK_5)
/// - [`ButtonMask::Any`](XCB_BUTTON_MASK_ANY)
pub type xcb_button_mask_t = u32;
/// The `ButtonMask::1` enum variant.
///
/// This is a variant of [`xcb_button_mask_t`].
pub const XCB_BUTTON_MASK_1: xcb_button_mask_t = 256;
/// The `ButtonMask::2` enum variant.
///
/// This is a variant of [`xcb_button_mask_t`].
pub const XCB_BUTTON_MASK_2: xcb_button_mask_t = 512;
/// The `ButtonMask::3` enum variant.
///
/// This is a variant of [`xcb_button_mask_t`].
pub const XCB_BUTTON_MASK_3: xcb_button_mask_t = 1024;
/// The `ButtonMask::4` enum variant.
///
/// This is a variant of [`xcb_button_mask_t`].
pub const XCB_BUTTON_MASK_4: xcb_button_mask_t = 2048;
/// The `ButtonMask::5` enum variant.
///
/// This is a variant of [`xcb_button_mask_t`].
pub const XCB_BUTTON_MASK_5: xcb_button_mask_t = 4096;
/// The `ButtonMask::Any` enum variant.
///
/// This is a variant of [`xcb_button_mask_t`].
pub const XCB_BUTTON_MASK_ANY: xcb_button_mask_t = 32768;

/// The opcode for `ButtonPress` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_button_press_event_t`].
pub const XCB_BUTTON_PRESS: u8 = 4i32 as u8;

/// The `ButtonPress` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_button_press_event_t {
    pub response_type: u8,
    pub detail: xcb_button_t,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: u8,
    pub pad0: u8,
}

impl Default for xcb_button_press_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ButtonRelease` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_button_release_event_t`].
pub const XCB_BUTTON_RELEASE: u8 = 5i32 as u8;

/// The `ButtonRelease` event.
pub type xcb_button_release_event_t = xcb_button_press_event_t;

/// The `Motion` enum.
///
/// This enum has the following variants:
///
/// - [`Motion::Normal`](XCB_MOTION_NORMAL)
/// - [`Motion::Hint`](XCB_MOTION_HINT)
pub type xcb_motion_t = u32;
/// The `Motion::Normal` enum variant.
///
/// This is a variant of [`xcb_motion_t`].
pub const XCB_MOTION_NORMAL: xcb_motion_t = 0;
/// The `Motion::Hint` enum variant.
///
/// This is a variant of [`xcb_motion_t`].
pub const XCB_MOTION_HINT: xcb_motion_t = 1;

/// The opcode for `MotionNotify` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_motion_notify_event_t`].
pub const XCB_MOTION_NOTIFY: u8 = 6i32 as u8;

/// The `MotionNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_motion_notify_event_t {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: u8,
    pub pad0: u8,
}

impl Default for xcb_motion_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `NotifyDetail` enum.
///
/// This enum has the following variants:
///
/// - [`NotifyDetail::Ancestor`](XCB_NOTIFY_DETAIL_ANCESTOR)
/// - [`NotifyDetail::Virtual`](XCB_NOTIFY_DETAIL_VIRTUAL)
/// - [`NotifyDetail::Inferior`](XCB_NOTIFY_DETAIL_INFERIOR)
/// - [`NotifyDetail::Nonlinear`](XCB_NOTIFY_DETAIL_NONLINEAR)
/// - [`NotifyDetail::NonlinearVirtual`](XCB_NOTIFY_DETAIL_NONLINEAR_VIRTUAL)
/// - [`NotifyDetail::Pointer`](XCB_NOTIFY_DETAIL_POINTER)
/// - [`NotifyDetail::PointerRoot`](XCB_NOTIFY_DETAIL_POINTER_ROOT)
/// - [`NotifyDetail::None`](XCB_NOTIFY_DETAIL_NONE)
pub type xcb_notify_detail_t = u32;
/// The `NotifyDetail::Ancestor` enum variant.
///
/// This is a variant of [`xcb_notify_detail_t`].
pub const XCB_NOTIFY_DETAIL_ANCESTOR: xcb_notify_detail_t = 0;
/// The `NotifyDetail::Virtual` enum variant.
///
/// This is a variant of [`xcb_notify_detail_t`].
pub const XCB_NOTIFY_DETAIL_VIRTUAL: xcb_notify_detail_t = 1;
/// The `NotifyDetail::Inferior` enum variant.
///
/// This is a variant of [`xcb_notify_detail_t`].
pub const XCB_NOTIFY_DETAIL_INFERIOR: xcb_notify_detail_t = 2;
/// The `NotifyDetail::Nonlinear` enum variant.
///
/// This is a variant of [`xcb_notify_detail_t`].
pub const XCB_NOTIFY_DETAIL_NONLINEAR: xcb_notify_detail_t = 3;
/// The `NotifyDetail::NonlinearVirtual` enum variant.
///
/// This is a variant of [`xcb_notify_detail_t`].
pub const XCB_NOTIFY_DETAIL_NONLINEAR_VIRTUAL: xcb_notify_detail_t = 4;
/// The `NotifyDetail::Pointer` enum variant.
///
/// This is a variant of [`xcb_notify_detail_t`].
pub const XCB_NOTIFY_DETAIL_POINTER: xcb_notify_detail_t = 5;
/// The `NotifyDetail::PointerRoot` enum variant.
///
/// This is a variant of [`xcb_notify_detail_t`].
pub const XCB_NOTIFY_DETAIL_POINTER_ROOT: xcb_notify_detail_t = 6;
/// The `NotifyDetail::None` enum variant.
///
/// This is a variant of [`xcb_notify_detail_t`].
pub const XCB_NOTIFY_DETAIL_NONE: xcb_notify_detail_t = 7;

/// The `NotifyMode` enum.
///
/// This enum has the following variants:
///
/// - [`NotifyMode::Normal`](XCB_NOTIFY_MODE_NORMAL)
/// - [`NotifyMode::Grab`](XCB_NOTIFY_MODE_GRAB)
/// - [`NotifyMode::Ungrab`](XCB_NOTIFY_MODE_UNGRAB)
/// - [`NotifyMode::WhileGrabbed`](XCB_NOTIFY_MODE_WHILE_GRABBED)
pub type xcb_notify_mode_t = u32;
/// The `NotifyMode::Normal` enum variant.
///
/// This is a variant of [`xcb_notify_mode_t`].
pub const XCB_NOTIFY_MODE_NORMAL: xcb_notify_mode_t = 0;
/// The `NotifyMode::Grab` enum variant.
///
/// This is a variant of [`xcb_notify_mode_t`].
pub const XCB_NOTIFY_MODE_GRAB: xcb_notify_mode_t = 1;
/// The `NotifyMode::Ungrab` enum variant.
///
/// This is a variant of [`xcb_notify_mode_t`].
pub const XCB_NOTIFY_MODE_UNGRAB: xcb_notify_mode_t = 2;
/// The `NotifyMode::WhileGrabbed` enum variant.
///
/// This is a variant of [`xcb_notify_mode_t`].
pub const XCB_NOTIFY_MODE_WHILE_GRABBED: xcb_notify_mode_t = 3;

/// The opcode for `EnterNotify` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_enter_notify_event_t`].
pub const XCB_ENTER_NOTIFY: u8 = 7i32 as u8;

/// The `EnterNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_enter_notify_event_t {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub mode: u8,
    pub same_screen_focus: u8,
}

impl Default for xcb_enter_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `LeaveNotify` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_leave_notify_event_t`].
pub const XCB_LEAVE_NOTIFY: u8 = 8i32 as u8;

/// The `LeaveNotify` event.
pub type xcb_leave_notify_event_t = xcb_enter_notify_event_t;

/// The opcode for `FocusIn` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_focus_in_event_t`].
pub const XCB_FOCUS_IN: u8 = 9i32 as u8;

/// The `FocusIn` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_focus_in_event_t {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub mode: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_focus_in_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `FocusOut` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_focus_out_event_t`].
pub const XCB_FOCUS_OUT: u8 = 10i32 as u8;

/// The `FocusOut` event.
pub type xcb_focus_out_event_t = xcb_focus_in_event_t;

/// The opcode for `KeymapNotify` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_keymap_notify_event_t`].
pub const XCB_KEYMAP_NOTIFY: u8 = 11i32 as u8;

/// The `KeymapNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_keymap_notify_event_t {
    pub response_type: u8,
    pub keys: [u8; 31],
}

impl Default for xcb_keymap_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Expose` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_expose_event_t`].
pub const XCB_EXPOSE: u8 = 12i32 as u8;

/// The `Expose` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_expose_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub count: u16,
    pub pad1: [u8; 2],
}

impl Default for xcb_expose_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `GraphicsExposure` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_graphics_exposure_event_t`].
pub const XCB_GRAPHICS_EXPOSURE: u8 = 13i32 as u8;

/// The `GraphicsExposure` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_graphics_exposure_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub drawable: xcb_drawable_t,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub minor_opcode: u16,
    pub count: u16,
    pub major_opcode: u8,
    pub pad1: [u8; 3],
}

impl Default for xcb_graphics_exposure_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `NoExposure` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_no_exposure_event_t`].
pub const XCB_NO_EXPOSURE: u8 = 14i32 as u8;

/// The `NoExposure` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_no_exposure_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub drawable: xcb_drawable_t,
    pub minor_opcode: u16,
    pub major_opcode: u8,
    pub pad1: u8,
}

impl Default for xcb_no_exposure_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Visibility` enum.
///
/// This enum has the following variants:
///
/// - [`Visibility::Unobscured`](XCB_VISIBILITY_UNOBSCURED)
/// - [`Visibility::PartiallyObscured`](XCB_VISIBILITY_PARTIALLY_OBSCURED)
/// - [`Visibility::FullyObscured`](XCB_VISIBILITY_FULLY_OBSCURED)
pub type xcb_visibility_t = u32;
/// The `Visibility::Unobscured` enum variant.
///
/// This is a variant of [`xcb_visibility_t`].
pub const XCB_VISIBILITY_UNOBSCURED: xcb_visibility_t = 0;
/// The `Visibility::PartiallyObscured` enum variant.
///
/// This is a variant of [`xcb_visibility_t`].
pub const XCB_VISIBILITY_PARTIALLY_OBSCURED: xcb_visibility_t = 1;
/// The `Visibility::FullyObscured` enum variant.
///
/// This is a variant of [`xcb_visibility_t`].
pub const XCB_VISIBILITY_FULLY_OBSCURED: xcb_visibility_t = 2;

/// The opcode for `VisibilityNotify` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_visibility_notify_event_t`].
pub const XCB_VISIBILITY_NOTIFY: u8 = 15i32 as u8;

/// The `VisibilityNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_visibility_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub state: u8,
    pub pad1: [u8; 3],
}

impl Default for xcb_visibility_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `CreateNotify` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_create_notify_event_t`].
pub const XCB_CREATE_NOTIFY: u8 = 16i32 as u8;

/// The `CreateNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_create_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub parent: xcb_window_t,
    pub window: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub override_redirect: u8,
    pub pad1: u8,
}

impl Default for xcb_create_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DestroyNotify` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_destroy_notify_event_t`].
pub const XCB_DESTROY_NOTIFY: u8 = 17i32 as u8;

/// The `DestroyNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_destroy_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
}

impl Default for xcb_destroy_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `UnmapNotify` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_unmap_notify_event_t`].
pub const XCB_UNMAP_NOTIFY: u8 = 18i32 as u8;

/// The `UnmapNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_unmap_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub from_configure: u8,
    pub pad1: [u8; 3],
}

impl Default for xcb_unmap_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `MapNotify` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_map_notify_event_t`].
pub const XCB_MAP_NOTIFY: u8 = 19i32 as u8;

/// The `MapNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_map_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub override_redirect: u8,
    pub pad1: [u8; 3],
}

impl Default for xcb_map_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `MapRequest` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_map_request_event_t`].
pub const XCB_MAP_REQUEST: u8 = 20i32 as u8;

/// The `MapRequest` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_map_request_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub parent: xcb_window_t,
    pub window: xcb_window_t,
}

impl Default for xcb_map_request_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ReparentNotify` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_reparent_notify_event_t`].
pub const XCB_REPARENT_NOTIFY: u8 = 21i32 as u8;

/// The `ReparentNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_reparent_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub parent: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub override_redirect: u8,
    pub pad1: [u8; 3],
}

impl Default for xcb_reparent_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ConfigureNotify` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_configure_notify_event_t`].
pub const XCB_CONFIGURE_NOTIFY: u8 = 22i32 as u8;

/// The `ConfigureNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_configure_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub above_sibling: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub override_redirect: u8,
    pub pad1: u8,
}

impl Default for xcb_configure_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ConfigureRequest` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_configure_request_event_t`].
pub const XCB_CONFIGURE_REQUEST: u8 = 23i32 as u8;

/// The `ConfigureRequest` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_configure_request_event_t {
    pub response_type: u8,
    pub stack_mode: u8,
    pub sequence: u16,
    pub parent: xcb_window_t,
    pub window: xcb_window_t,
    pub sibling: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub value_mask: u16,
}

impl Default for xcb_configure_request_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `GravityNotify` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_gravity_notify_event_t`].
pub const XCB_GRAVITY_NOTIFY: u8 = 24i32 as u8;

/// The `GravityNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_gravity_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub x: i16,
    pub y: i16,
}

impl Default for xcb_gravity_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ResizeRequest` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_resize_request_event_t`].
pub const XCB_RESIZE_REQUEST: u8 = 25i32 as u8;

/// The `ResizeRequest` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_resize_request_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub width: u16,
    pub height: u16,
}

impl Default for xcb_resize_request_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Place` enum.
///
/// This enum has the following variants:
///
/// - [`Place::OnTop`](XCB_PLACE_ON_TOP)
/// - [`Place::OnBottom`](XCB_PLACE_ON_BOTTOM)
pub type xcb_place_t = u32;
/// The `Place::OnTop` enum variant.
///
/// This is a variant of [`xcb_place_t`].
pub const XCB_PLACE_ON_TOP: xcb_place_t = 0;
/// The `Place::OnBottom` enum variant.
///
/// This is a variant of [`xcb_place_t`].
pub const XCB_PLACE_ON_BOTTOM: xcb_place_t = 1;

/// The opcode for `CirculateNotify` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_circulate_notify_event_t`].
pub const XCB_CIRCULATE_NOTIFY: u8 = 26i32 as u8;

/// The `CirculateNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_circulate_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub pad1: [u8; 4],
    pub place: u8,
    pub pad2: [u8; 3],
}

impl Default for xcb_circulate_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `CirculateRequest` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_circulate_request_event_t`].
pub const XCB_CIRCULATE_REQUEST: u8 = 27i32 as u8;

/// The `CirculateRequest` event.
pub type xcb_circulate_request_event_t = xcb_circulate_notify_event_t;

/// The `Property` enum.
///
/// This enum has the following variants:
///
/// - [`Property::NewValue`](XCB_PROPERTY_NEW_VALUE)
/// - [`Property::Delete`](XCB_PROPERTY_DELETE)
pub type xcb_property_t = u32;
/// The `Property::NewValue` enum variant.
///
/// This is a variant of [`xcb_property_t`].
pub const XCB_PROPERTY_NEW_VALUE: xcb_property_t = 0;
/// The `Property::Delete` enum variant.
///
/// This is a variant of [`xcb_property_t`].
pub const XCB_PROPERTY_DELETE: xcb_property_t = 1;

/// The opcode for `PropertyNotify` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_property_notify_event_t`].
pub const XCB_PROPERTY_NOTIFY: u8 = 28i32 as u8;

/// The `PropertyNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_property_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub atom: xcb_atom_t,
    pub time: xcb_timestamp_t,
    pub state: u8,
    pub pad1: [u8; 3],
}

impl Default for xcb_property_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SelectionClear` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_selection_clear_event_t`].
pub const XCB_SELECTION_CLEAR: u8 = 29i32 as u8;

/// The `SelectionClear` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selection_clear_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub owner: xcb_window_t,
    pub selection: xcb_atom_t,
}

impl Default for xcb_selection_clear_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Time` enum.
///
/// This enum has the following variants:
///
/// - [`Time::CurrentTime`](XCB_TIME_CURRENT_TIME)
pub type xcb_time_t = u32;
/// The `Time::CurrentTime` enum variant.
///
/// This is a variant of [`xcb_time_t`].
pub const XCB_TIME_CURRENT_TIME: xcb_time_t = 0;

/// The `Atom` enum.
///
/// This enum has the following variants:
///
/// - [`Atom::None`](XCB_ATOM_NONE)
/// - [`Atom::Any`](XCB_ATOM_ANY)
/// - [`Atom::PRIMARY`](XCB_ATOM_PRIMARY)
/// - [`Atom::SECONDARY`](XCB_ATOM_SECONDARY)
/// - [`Atom::ARC`](XCB_ATOM_ARC)
/// - [`Atom::ATOM`](XCB_ATOM_ATOM)
/// - [`Atom::BITMAP`](XCB_ATOM_BITMAP)
/// - [`Atom::CARDINAL`](XCB_ATOM_CARDINAL)
/// - [`Atom::COLORMAP`](XCB_ATOM_COLORMAP)
/// - [`Atom::CURSOR`](XCB_ATOM_CURSOR)
/// - [`Atom::CUT_BUFFER0`](XCB_ATOM_CUT_BUFFER0)
/// - [`Atom::CUT_BUFFER1`](XCB_ATOM_CUT_BUFFER1)
/// - [`Atom::CUT_BUFFER2`](XCB_ATOM_CUT_BUFFER2)
/// - [`Atom::CUT_BUFFER3`](XCB_ATOM_CUT_BUFFER3)
/// - [`Atom::CUT_BUFFER4`](XCB_ATOM_CUT_BUFFER4)
/// - [`Atom::CUT_BUFFER5`](XCB_ATOM_CUT_BUFFER5)
/// - [`Atom::CUT_BUFFER6`](XCB_ATOM_CUT_BUFFER6)
/// - [`Atom::CUT_BUFFER7`](XCB_ATOM_CUT_BUFFER7)
/// - [`Atom::DRAWABLE`](XCB_ATOM_DRAWABLE)
/// - [`Atom::FONT`](XCB_ATOM_FONT)
/// - [`Atom::INTEGER`](XCB_ATOM_INTEGER)
/// - [`Atom::PIXMAP`](XCB_ATOM_PIXMAP)
/// - [`Atom::POINT`](XCB_ATOM_POINT)
/// - [`Atom::RECTANGLE`](XCB_ATOM_RECTANGLE)
/// - [`Atom::RESOURCE_MANAGER`](XCB_ATOM_RESOURCE_MANAGER)
/// - [`Atom::RGB_COLOR_MAP`](XCB_ATOM_RGB_COLOR_MAP)
/// - [`Atom::RGB_BEST_MAP`](XCB_ATOM_RGB_BEST_MAP)
/// - [`Atom::RGB_BLUE_MAP`](XCB_ATOM_RGB_BLUE_MAP)
/// - [`Atom::RGB_DEFAULT_MAP`](XCB_ATOM_RGB_DEFAULT_MAP)
/// - [`Atom::RGB_GRAY_MAP`](XCB_ATOM_RGB_GRAY_MAP)
/// - [`Atom::RGB_GREEN_MAP`](XCB_ATOM_RGB_GREEN_MAP)
/// - [`Atom::RGB_RED_MAP`](XCB_ATOM_RGB_RED_MAP)
/// - [`Atom::STRING`](XCB_ATOM_STRING)
/// - [`Atom::VISUALID`](XCB_ATOM_VISUALID)
/// - [`Atom::WINDOW`](XCB_ATOM_WINDOW)
/// - [`Atom::WM_COMMAND`](XCB_ATOM_WM_COMMAND)
/// - [`Atom::WM_HINTS`](XCB_ATOM_WM_HINTS)
/// - [`Atom::WM_CLIENT_MACHINE`](XCB_ATOM_WM_CLIENT_MACHINE)
/// - [`Atom::WM_ICON_NAME`](XCB_ATOM_WM_ICON_NAME)
/// - [`Atom::WM_ICON_SIZE`](XCB_ATOM_WM_ICON_SIZE)
/// - [`Atom::WM_NAME`](XCB_ATOM_WM_NAME)
/// - [`Atom::WM_NORMAL_HINTS`](XCB_ATOM_WM_NORMAL_HINTS)
/// - [`Atom::WM_SIZE_HINTS`](XCB_ATOM_WM_SIZE_HINTS)
/// - [`Atom::WM_ZOOM_HINTS`](XCB_ATOM_WM_ZOOM_HINTS)
/// - [`Atom::MIN_SPACE`](XCB_ATOM_MIN_SPACE)
/// - [`Atom::NORM_SPACE`](XCB_ATOM_NORM_SPACE)
/// - [`Atom::MAX_SPACE`](XCB_ATOM_MAX_SPACE)
/// - [`Atom::END_SPACE`](XCB_ATOM_END_SPACE)
/// - [`Atom::SUPERSCRIPT_X`](XCB_ATOM_SUPERSCRIPT_X)
/// - [`Atom::SUPERSCRIPT_Y`](XCB_ATOM_SUPERSCRIPT_Y)
/// - [`Atom::SUBSCRIPT_X`](XCB_ATOM_SUBSCRIPT_X)
/// - [`Atom::SUBSCRIPT_Y`](XCB_ATOM_SUBSCRIPT_Y)
/// - [`Atom::UNDERLINE_POSITION`](XCB_ATOM_UNDERLINE_POSITION)
/// - [`Atom::UNDERLINE_THICKNESS`](XCB_ATOM_UNDERLINE_THICKNESS)
/// - [`Atom::STRIKEOUT_ASCENT`](XCB_ATOM_STRIKEOUT_ASCENT)
/// - [`Atom::STRIKEOUT_DESCENT`](XCB_ATOM_STRIKEOUT_DESCENT)
/// - [`Atom::ITALIC_ANGLE`](XCB_ATOM_ITALIC_ANGLE)
/// - [`Atom::X_HEIGHT`](XCB_ATOM_X_HEIGHT)
/// - [`Atom::QUAD_WIDTH`](XCB_ATOM_QUAD_WIDTH)
/// - [`Atom::WEIGHT`](XCB_ATOM_WEIGHT)
/// - [`Atom::POINT_SIZE`](XCB_ATOM_POINT_SIZE)
/// - [`Atom::RESOLUTION`](XCB_ATOM_RESOLUTION)
/// - [`Atom::COPYRIGHT`](XCB_ATOM_COPYRIGHT)
/// - [`Atom::NOTICE`](XCB_ATOM_NOTICE)
/// - [`Atom::FONT_NAME`](XCB_ATOM_FONT_NAME)
/// - [`Atom::FAMILY_NAME`](XCB_ATOM_FAMILY_NAME)
/// - [`Atom::FULL_NAME`](XCB_ATOM_FULL_NAME)
/// - [`Atom::CAP_HEIGHT`](XCB_ATOM_CAP_HEIGHT)
/// - [`Atom::WM_CLASS`](XCB_ATOM_WM_CLASS)
/// - [`Atom::WM_TRANSIENT_FOR`](XCB_ATOM_WM_TRANSIENT_FOR)
pub type xcb_atom_enum_t = u32;
/// The `Atom::None` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_NONE: xcb_atom_enum_t = 0;
/// The `Atom::Any` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_ANY: xcb_atom_enum_t = 0;
/// The `Atom::PRIMARY` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_PRIMARY: xcb_atom_enum_t = 1;
/// The `Atom::SECONDARY` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_SECONDARY: xcb_atom_enum_t = 2;
/// The `Atom::ARC` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_ARC: xcb_atom_enum_t = 3;
/// The `Atom::ATOM` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_ATOM: xcb_atom_enum_t = 4;
/// The `Atom::BITMAP` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_BITMAP: xcb_atom_enum_t = 5;
/// The `Atom::CARDINAL` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_CARDINAL: xcb_atom_enum_t = 6;
/// The `Atom::COLORMAP` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_COLORMAP: xcb_atom_enum_t = 7;
/// The `Atom::CURSOR` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_CURSOR: xcb_atom_enum_t = 8;
/// The `Atom::CUT_BUFFER0` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_CUT_BUFFER0: xcb_atom_enum_t = 9;
/// The `Atom::CUT_BUFFER1` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_CUT_BUFFER1: xcb_atom_enum_t = 10;
/// The `Atom::CUT_BUFFER2` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_CUT_BUFFER2: xcb_atom_enum_t = 11;
/// The `Atom::CUT_BUFFER3` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_CUT_BUFFER3: xcb_atom_enum_t = 12;
/// The `Atom::CUT_BUFFER4` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_CUT_BUFFER4: xcb_atom_enum_t = 13;
/// The `Atom::CUT_BUFFER5` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_CUT_BUFFER5: xcb_atom_enum_t = 14;
/// The `Atom::CUT_BUFFER6` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_CUT_BUFFER6: xcb_atom_enum_t = 15;
/// The `Atom::CUT_BUFFER7` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_CUT_BUFFER7: xcb_atom_enum_t = 16;
/// The `Atom::DRAWABLE` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_DRAWABLE: xcb_atom_enum_t = 17;
/// The `Atom::FONT` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_FONT: xcb_atom_enum_t = 18;
/// The `Atom::INTEGER` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_INTEGER: xcb_atom_enum_t = 19;
/// The `Atom::PIXMAP` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_PIXMAP: xcb_atom_enum_t = 20;
/// The `Atom::POINT` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_POINT: xcb_atom_enum_t = 21;
/// The `Atom::RECTANGLE` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_RECTANGLE: xcb_atom_enum_t = 22;
/// The `Atom::RESOURCE_MANAGER` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_RESOURCE_MANAGER: xcb_atom_enum_t = 23;
/// The `Atom::RGB_COLOR_MAP` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_RGB_COLOR_MAP: xcb_atom_enum_t = 24;
/// The `Atom::RGB_BEST_MAP` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_RGB_BEST_MAP: xcb_atom_enum_t = 25;
/// The `Atom::RGB_BLUE_MAP` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_RGB_BLUE_MAP: xcb_atom_enum_t = 26;
/// The `Atom::RGB_DEFAULT_MAP` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_RGB_DEFAULT_MAP: xcb_atom_enum_t = 27;
/// The `Atom::RGB_GRAY_MAP` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_RGB_GRAY_MAP: xcb_atom_enum_t = 28;
/// The `Atom::RGB_GREEN_MAP` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_RGB_GREEN_MAP: xcb_atom_enum_t = 29;
/// The `Atom::RGB_RED_MAP` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_RGB_RED_MAP: xcb_atom_enum_t = 30;
/// The `Atom::STRING` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_STRING: xcb_atom_enum_t = 31;
/// The `Atom::VISUALID` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_VISUALID: xcb_atom_enum_t = 32;
/// The `Atom::WINDOW` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_WINDOW: xcb_atom_enum_t = 33;
/// The `Atom::WM_COMMAND` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_WM_COMMAND: xcb_atom_enum_t = 34;
/// The `Atom::WM_HINTS` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_WM_HINTS: xcb_atom_enum_t = 35;
/// The `Atom::WM_CLIENT_MACHINE` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_WM_CLIENT_MACHINE: xcb_atom_enum_t = 36;
/// The `Atom::WM_ICON_NAME` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_WM_ICON_NAME: xcb_atom_enum_t = 37;
/// The `Atom::WM_ICON_SIZE` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_WM_ICON_SIZE: xcb_atom_enum_t = 38;
/// The `Atom::WM_NAME` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_WM_NAME: xcb_atom_enum_t = 39;
/// The `Atom::WM_NORMAL_HINTS` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_WM_NORMAL_HINTS: xcb_atom_enum_t = 40;
/// The `Atom::WM_SIZE_HINTS` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_WM_SIZE_HINTS: xcb_atom_enum_t = 41;
/// The `Atom::WM_ZOOM_HINTS` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_WM_ZOOM_HINTS: xcb_atom_enum_t = 42;
/// The `Atom::MIN_SPACE` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_MIN_SPACE: xcb_atom_enum_t = 43;
/// The `Atom::NORM_SPACE` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_NORM_SPACE: xcb_atom_enum_t = 44;
/// The `Atom::MAX_SPACE` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_MAX_SPACE: xcb_atom_enum_t = 45;
/// The `Atom::END_SPACE` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_END_SPACE: xcb_atom_enum_t = 46;
/// The `Atom::SUPERSCRIPT_X` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_SUPERSCRIPT_X: xcb_atom_enum_t = 47;
/// The `Atom::SUPERSCRIPT_Y` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_SUPERSCRIPT_Y: xcb_atom_enum_t = 48;
/// The `Atom::SUBSCRIPT_X` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_SUBSCRIPT_X: xcb_atom_enum_t = 49;
/// The `Atom::SUBSCRIPT_Y` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_SUBSCRIPT_Y: xcb_atom_enum_t = 50;
/// The `Atom::UNDERLINE_POSITION` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_UNDERLINE_POSITION: xcb_atom_enum_t = 51;
/// The `Atom::UNDERLINE_THICKNESS` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_UNDERLINE_THICKNESS: xcb_atom_enum_t = 52;
/// The `Atom::STRIKEOUT_ASCENT` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_STRIKEOUT_ASCENT: xcb_atom_enum_t = 53;
/// The `Atom::STRIKEOUT_DESCENT` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_STRIKEOUT_DESCENT: xcb_atom_enum_t = 54;
/// The `Atom::ITALIC_ANGLE` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_ITALIC_ANGLE: xcb_atom_enum_t = 55;
/// The `Atom::X_HEIGHT` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_X_HEIGHT: xcb_atom_enum_t = 56;
/// The `Atom::QUAD_WIDTH` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_QUAD_WIDTH: xcb_atom_enum_t = 57;
/// The `Atom::WEIGHT` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_WEIGHT: xcb_atom_enum_t = 58;
/// The `Atom::POINT_SIZE` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_POINT_SIZE: xcb_atom_enum_t = 59;
/// The `Atom::RESOLUTION` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_RESOLUTION: xcb_atom_enum_t = 60;
/// The `Atom::COPYRIGHT` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_COPYRIGHT: xcb_atom_enum_t = 61;
/// The `Atom::NOTICE` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_NOTICE: xcb_atom_enum_t = 62;
/// The `Atom::FONT_NAME` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_FONT_NAME: xcb_atom_enum_t = 63;
/// The `Atom::FAMILY_NAME` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_FAMILY_NAME: xcb_atom_enum_t = 64;
/// The `Atom::FULL_NAME` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_FULL_NAME: xcb_atom_enum_t = 65;
/// The `Atom::CAP_HEIGHT` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_CAP_HEIGHT: xcb_atom_enum_t = 66;
/// The `Atom::WM_CLASS` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_WM_CLASS: xcb_atom_enum_t = 67;
/// The `Atom::WM_TRANSIENT_FOR` enum variant.
///
/// This is a variant of [`xcb_atom_enum_t`].
pub const XCB_ATOM_WM_TRANSIENT_FOR: xcb_atom_enum_t = 68;

/// The opcode for `SelectionRequest` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_selection_request_event_t`].
pub const XCB_SELECTION_REQUEST: u8 = 30i32 as u8;

/// The `SelectionRequest` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selection_request_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub owner: xcb_window_t,
    pub requestor: xcb_window_t,
    pub selection: xcb_atom_t,
    pub target: xcb_atom_t,
    pub property: xcb_atom_t,
}

impl Default for xcb_selection_request_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SelectionNotify` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_selection_notify_event_t`].
pub const XCB_SELECTION_NOTIFY: u8 = 31i32 as u8;

/// The `SelectionNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_selection_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub requestor: xcb_window_t,
    pub selection: xcb_atom_t,
    pub target: xcb_atom_t,
    pub property: xcb_atom_t,
}

impl Default for xcb_selection_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `ColormapState` enum.
///
/// This enum has the following variants:
///
/// - [`ColormapState::Uninstalled`](XCB_COLORMAP_STATE_UNINSTALLED)
/// - [`ColormapState::Installed`](XCB_COLORMAP_STATE_INSTALLED)
pub type xcb_colormap_state_t = u32;
/// The `ColormapState::Uninstalled` enum variant.
///
/// This is a variant of [`xcb_colormap_state_t`].
pub const XCB_COLORMAP_STATE_UNINSTALLED: xcb_colormap_state_t = 0;
/// The `ColormapState::Installed` enum variant.
///
/// This is a variant of [`xcb_colormap_state_t`].
pub const XCB_COLORMAP_STATE_INSTALLED: xcb_colormap_state_t = 1;

/// The `Colormap` enum.
///
/// This enum has the following variants:
///
/// - [`Colormap::None`](XCB_COLORMAP_NONE)
pub type xcb_colormap_enum_t = u32;
/// The `Colormap::None` enum variant.
///
/// This is a variant of [`xcb_colormap_enum_t`].
pub const XCB_COLORMAP_NONE: xcb_colormap_enum_t = 0;

/// The opcode for `ColormapNotify` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_colormap_notify_event_t`].
pub const XCB_COLORMAP_NOTIFY: u8 = 32i32 as u8;

/// The `ColormapNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_colormap_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub colormap: xcb_colormap_t,
    pub new: u8,
    pub state: u8,
    pub pad1: [u8; 2],
}

impl Default for xcb_colormap_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `ClientMessageData` union.
#[derive(Copy, Clone)]
#[repr(C)]
pub union xcb_client_message_data_t {
    pub data8: [u8; 20],
    pub data16: [u16; 10],
    pub data32: [u32; 5],
}

impl Default for xcb_client_message_data_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `ClientMessageData` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_client_message_data_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_client_message_data_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_client_message_data_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ClientMessage` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_client_message_event_t`].
pub const XCB_CLIENT_MESSAGE: u8 = 33i32 as u8;

/// The `ClientMessage` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_client_message_event_t {
    pub response_type: u8,
    pub format: u8,
    pub sequence: u16,
    pub window: xcb_window_t,
    pub type_: xcb_atom_t,
    pub data: xcb_client_message_data_t,
}

impl Default for xcb_client_message_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Mapping` enum.
///
/// This enum has the following variants:
///
/// - [`Mapping::Modifier`](XCB_MAPPING_MODIFIER)
/// - [`Mapping::Keyboard`](XCB_MAPPING_KEYBOARD)
/// - [`Mapping::Pointer`](XCB_MAPPING_POINTER)
pub type xcb_mapping_t = u32;
/// The `Mapping::Modifier` enum variant.
///
/// This is a variant of [`xcb_mapping_t`].
pub const XCB_MAPPING_MODIFIER: xcb_mapping_t = 0;
/// The `Mapping::Keyboard` enum variant.
///
/// This is a variant of [`xcb_mapping_t`].
pub const XCB_MAPPING_KEYBOARD: xcb_mapping_t = 1;
/// The `Mapping::Pointer` enum variant.
///
/// This is a variant of [`xcb_mapping_t`].
pub const XCB_MAPPING_POINTER: xcb_mapping_t = 2;

/// The opcode for `MappingNotify` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_mapping_notify_event_t`].
pub const XCB_MAPPING_NOTIFY: u8 = 34i32 as u8;

/// The `MappingNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_mapping_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub request: u8,
    pub first_keycode: xcb_keycode_t,
    pub count: u8,
    pub pad1: u8,
}

impl Default for xcb_mapping_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `GeGeneric` events.
///
/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the
/// event is [`xcb_ge_generic_event_t`].
pub const XCB_GE_GENERIC: u8 = 35i32 as u8;

/// The `GeGeneric` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_ge_generic_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub pad0: [u8; 22],
    pub full_sequence: u32,
}

impl Default for xcb_ge_generic_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Request` errors.
///
/// If this value appears in [`xcb_generic_error_t::error_code`], then the type of the
/// error is [`xcb_request_error_t`].
pub const XCB_REQUEST: u8 = 1i32 as u8;

/// The `Request` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_request_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
    pub pad0: u8,
}

impl Default for xcb_request_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Value` errors.
///
/// If this value appears in [`xcb_generic_error_t::error_code`], then the type of the
/// error is [`xcb_value_error_t`].
pub const XCB_VALUE: u8 = 2i32 as u8;

/// The `Value` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_value_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
    pub pad0: u8,
}

impl Default for xcb_value_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Window` errors.
///
/// If this value appears in [`xcb_generic_error_t::error_code`], then the type of the
/// error is [`xcb_window_error_t`].
pub const XCB_WINDOW: u8 = 3i32 as u8;

/// The `Window` error.
pub type xcb_window_error_t = xcb_value_error_t;

/// The opcode for `Pixmap` errors.
///
/// If this value appears in [`xcb_generic_error_t::error_code`], then the type of the
/// error is [`xcb_pixmap_error_t`].
pub const XCB_PIXMAP: u8 = 4i32 as u8;

/// The `Pixmap` error.
pub type xcb_pixmap_error_t = xcb_value_error_t;

/// The opcode for `Atom` errors.
///
/// If this value appears in [`xcb_generic_error_t::error_code`], then the type of the
/// error is [`xcb_atom_error_t`].
pub const XCB_ATOM: u8 = 5i32 as u8;

/// The `Atom` error.
pub type xcb_atom_error_t = xcb_value_error_t;

/// The opcode for `Cursor` errors.
///
/// If this value appears in [`xcb_generic_error_t::error_code`], then the type of the
/// error is [`xcb_cursor_error_t`].
pub const XCB_CURSOR: u8 = 6i32 as u8;

/// The `Cursor` error.
pub type xcb_cursor_error_t = xcb_value_error_t;

/// The opcode for `Font` errors.
///
/// If this value appears in [`xcb_generic_error_t::error_code`], then the type of the
/// error is [`xcb_font_error_t`].
pub const XCB_FONT: u8 = 7i32 as u8;

/// The `Font` error.
pub type xcb_font_error_t = xcb_value_error_t;

/// The opcode for `Match` errors.
///
/// If this value appears in [`xcb_generic_error_t::error_code`], then the type of the
/// error is [`xcb_match_error_t`].
pub const XCB_MATCH: u8 = 8i32 as u8;

/// The `Match` error.
pub type xcb_match_error_t = xcb_request_error_t;

/// The opcode for `Drawable` errors.
///
/// If this value appears in [`xcb_generic_error_t::error_code`], then the type of the
/// error is [`xcb_drawable_error_t`].
pub const XCB_DRAWABLE: u8 = 9i32 as u8;

/// The `Drawable` error.
pub type xcb_drawable_error_t = xcb_value_error_t;

/// The opcode for `Access` errors.
///
/// If this value appears in [`xcb_generic_error_t::error_code`], then the type of the
/// error is [`xcb_access_error_t`].
pub const XCB_ACCESS: u8 = 10i32 as u8;

/// The `Access` error.
pub type xcb_access_error_t = xcb_request_error_t;

/// The opcode for `Alloc` errors.
///
/// If this value appears in [`xcb_generic_error_t::error_code`], then the type of the
/// error is [`xcb_alloc_error_t`].
pub const XCB_ALLOC: u8 = 11i32 as u8;

/// The `Alloc` error.
pub type xcb_alloc_error_t = xcb_request_error_t;

/// The opcode for `Colormap` errors.
///
/// If this value appears in [`xcb_generic_error_t::error_code`], then the type of the
/// error is [`xcb_colormap_error_t`].
pub const XCB_COLORMAP: u8 = 12i32 as u8;

/// The `Colormap` error.
pub type xcb_colormap_error_t = xcb_value_error_t;

/// The opcode for `GContext` errors.
///
/// If this value appears in [`xcb_generic_error_t::error_code`], then the type of the
/// error is [`xcb_g_context_error_t`].
pub const XCB_G_CONTEXT: u8 = 13i32 as u8;

/// The `GContext` error.
pub type xcb_g_context_error_t = xcb_value_error_t;

/// The opcode for `IDChoice` errors.
///
/// If this value appears in [`xcb_generic_error_t::error_code`], then the type of the
/// error is [`xcb_id_choice_error_t`].
pub const XCB_ID_CHOICE: u8 = 14i32 as u8;

/// The `IDChoice` error.
pub type xcb_id_choice_error_t = xcb_value_error_t;

/// The opcode for `Name` errors.
///
/// If this value appears in [`xcb_generic_error_t::error_code`], then the type of the
/// error is [`xcb_name_error_t`].
pub const XCB_NAME: u8 = 15i32 as u8;

/// The `Name` error.
pub type xcb_name_error_t = xcb_request_error_t;

/// The opcode for `Length` errors.
///
/// If this value appears in [`xcb_generic_error_t::error_code`], then the type of the
/// error is [`xcb_length_error_t`].
pub const XCB_LENGTH: u8 = 16i32 as u8;

/// The `Length` error.
pub type xcb_length_error_t = xcb_request_error_t;

/// The opcode for `Implementation` errors.
///
/// If this value appears in [`xcb_generic_error_t::error_code`], then the type of the
/// error is [`xcb_implementation_error_t`].
pub const XCB_IMPLEMENTATION: u8 = 17i32 as u8;

/// The `Implementation` error.
pub type xcb_implementation_error_t = xcb_request_error_t;

/// The `WindowClass` enum.
///
/// This enum has the following variants:
///
/// - [`WindowClass::CopyFromParent`](XCB_WINDOW_CLASS_COPY_FROM_PARENT)
/// - [`WindowClass::InputOutput`](XCB_WINDOW_CLASS_INPUT_OUTPUT)
/// - [`WindowClass::InputOnly`](XCB_WINDOW_CLASS_INPUT_ONLY)
pub type xcb_window_class_t = u32;
/// The `WindowClass::CopyFromParent` enum variant.
///
/// This is a variant of [`xcb_window_class_t`].
pub const XCB_WINDOW_CLASS_COPY_FROM_PARENT: xcb_window_class_t = 0;
/// The `WindowClass::InputOutput` enum variant.
///
/// This is a variant of [`xcb_window_class_t`].
pub const XCB_WINDOW_CLASS_INPUT_OUTPUT: xcb_window_class_t = 1;
/// The `WindowClass::InputOnly` enum variant.
///
/// This is a variant of [`xcb_window_class_t`].
pub const XCB_WINDOW_CLASS_INPUT_ONLY: xcb_window_class_t = 2;

/// The `CW` enum.
///
/// This enum has the following variants:
///
/// - [`CW::BackPixmap`](XCB_CW_BACK_PIXMAP)
/// - [`CW::BackPixel`](XCB_CW_BACK_PIXEL)
/// - [`CW::BorderPixmap`](XCB_CW_BORDER_PIXMAP)
/// - [`CW::BorderPixel`](XCB_CW_BORDER_PIXEL)
/// - [`CW::BitGravity`](XCB_CW_BIT_GRAVITY)
/// - [`CW::WinGravity`](XCB_CW_WIN_GRAVITY)
/// - [`CW::BackingStore`](XCB_CW_BACKING_STORE)
/// - [`CW::BackingPlanes`](XCB_CW_BACKING_PLANES)
/// - [`CW::BackingPixel`](XCB_CW_BACKING_PIXEL)
/// - [`CW::OverrideRedirect`](XCB_CW_OVERRIDE_REDIRECT)
/// - [`CW::SaveUnder`](XCB_CW_SAVE_UNDER)
/// - [`CW::EventMask`](XCB_CW_EVENT_MASK)
/// - [`CW::DontPropagate`](XCB_CW_DONT_PROPAGATE)
/// - [`CW::Colormap`](XCB_CW_COLORMAP)
/// - [`CW::Cursor`](XCB_CW_CURSOR)
pub type xcb_cw_t = u32;
/// The `CW::BackPixmap` enum variant.
///
/// This is a variant of [`xcb_cw_t`].
pub const XCB_CW_BACK_PIXMAP: xcb_cw_t = 1;
/// The `CW::BackPixel` enum variant.
///
/// This is a variant of [`xcb_cw_t`].
pub const XCB_CW_BACK_PIXEL: xcb_cw_t = 2;
/// The `CW::BorderPixmap` enum variant.
///
/// This is a variant of [`xcb_cw_t`].
pub const XCB_CW_BORDER_PIXMAP: xcb_cw_t = 4;
/// The `CW::BorderPixel` enum variant.
///
/// This is a variant of [`xcb_cw_t`].
pub const XCB_CW_BORDER_PIXEL: xcb_cw_t = 8;
/// The `CW::BitGravity` enum variant.
///
/// This is a variant of [`xcb_cw_t`].
pub const XCB_CW_BIT_GRAVITY: xcb_cw_t = 16;
/// The `CW::WinGravity` enum variant.
///
/// This is a variant of [`xcb_cw_t`].
pub const XCB_CW_WIN_GRAVITY: xcb_cw_t = 32;
/// The `CW::BackingStore` enum variant.
///
/// This is a variant of [`xcb_cw_t`].
pub const XCB_CW_BACKING_STORE: xcb_cw_t = 64;
/// The `CW::BackingPlanes` enum variant.
///
/// This is a variant of [`xcb_cw_t`].
pub const XCB_CW_BACKING_PLANES: xcb_cw_t = 128;
/// The `CW::BackingPixel` enum variant.
///
/// This is a variant of [`xcb_cw_t`].
pub const XCB_CW_BACKING_PIXEL: xcb_cw_t = 256;
/// The `CW::OverrideRedirect` enum variant.
///
/// This is a variant of [`xcb_cw_t`].
pub const XCB_CW_OVERRIDE_REDIRECT: xcb_cw_t = 512;
/// The `CW::SaveUnder` enum variant.
///
/// This is a variant of [`xcb_cw_t`].
pub const XCB_CW_SAVE_UNDER: xcb_cw_t = 1024;
/// The `CW::EventMask` enum variant.
///
/// This is a variant of [`xcb_cw_t`].
pub const XCB_CW_EVENT_MASK: xcb_cw_t = 2048;
/// The `CW::DontPropagate` enum variant.
///
/// This is a variant of [`xcb_cw_t`].
pub const XCB_CW_DONT_PROPAGATE: xcb_cw_t = 4096;
/// The `CW::Colormap` enum variant.
///
/// This is a variant of [`xcb_cw_t`].
pub const XCB_CW_COLORMAP: xcb_cw_t = 8192;
/// The `CW::Cursor` enum variant.
///
/// This is a variant of [`xcb_cw_t`].
pub const XCB_CW_CURSOR: xcb_cw_t = 16384;

/// The `BackPixmap` enum.
///
/// This enum has the following variants:
///
/// - [`BackPixmap::None`](XCB_BACK_PIXMAP_NONE)
/// - [`BackPixmap::ParentRelative`](XCB_BACK_PIXMAP_PARENT_RELATIVE)
pub type xcb_back_pixmap_t = u32;
/// The `BackPixmap::None` enum variant.
///
/// This is a variant of [`xcb_back_pixmap_t`].
pub const XCB_BACK_PIXMAP_NONE: xcb_back_pixmap_t = 0;
/// The `BackPixmap::ParentRelative` enum variant.
///
/// This is a variant of [`xcb_back_pixmap_t`].
pub const XCB_BACK_PIXMAP_PARENT_RELATIVE: xcb_back_pixmap_t = 1;

/// The `Gravity` enum.
///
/// This enum has the following variants:
///
/// - [`Gravity::BitForget`](XCB_GRAVITY_BIT_FORGET)
/// - [`Gravity::WinUnmap`](XCB_GRAVITY_WIN_UNMAP)
/// - [`Gravity::NorthWest`](XCB_GRAVITY_NORTH_WEST)
/// - [`Gravity::North`](XCB_GRAVITY_NORTH)
/// - [`Gravity::NorthEast`](XCB_GRAVITY_NORTH_EAST)
/// - [`Gravity::West`](XCB_GRAVITY_WEST)
/// - [`Gravity::Center`](XCB_GRAVITY_CENTER)
/// - [`Gravity::East`](XCB_GRAVITY_EAST)
/// - [`Gravity::SouthWest`](XCB_GRAVITY_SOUTH_WEST)
/// - [`Gravity::South`](XCB_GRAVITY_SOUTH)
/// - [`Gravity::SouthEast`](XCB_GRAVITY_SOUTH_EAST)
/// - [`Gravity::Static`](XCB_GRAVITY_STATIC)
pub type xcb_gravity_t = u32;
/// The `Gravity::BitForget` enum variant.
///
/// This is a variant of [`xcb_gravity_t`].
pub const XCB_GRAVITY_BIT_FORGET: xcb_gravity_t = 0;
/// The `Gravity::WinUnmap` enum variant.
///
/// This is a variant of [`xcb_gravity_t`].
pub const XCB_GRAVITY_WIN_UNMAP: xcb_gravity_t = 0;
/// The `Gravity::NorthWest` enum variant.
///
/// This is a variant of [`xcb_gravity_t`].
pub const XCB_GRAVITY_NORTH_WEST: xcb_gravity_t = 1;
/// The `Gravity::North` enum variant.
///
/// This is a variant of [`xcb_gravity_t`].
pub const XCB_GRAVITY_NORTH: xcb_gravity_t = 2;
/// The `Gravity::NorthEast` enum variant.
///
/// This is a variant of [`xcb_gravity_t`].
pub const XCB_GRAVITY_NORTH_EAST: xcb_gravity_t = 3;
/// The `Gravity::West` enum variant.
///
/// This is a variant of [`xcb_gravity_t`].
pub const XCB_GRAVITY_WEST: xcb_gravity_t = 4;
/// The `Gravity::Center` enum variant.
///
/// This is a variant of [`xcb_gravity_t`].
pub const XCB_GRAVITY_CENTER: xcb_gravity_t = 5;
/// The `Gravity::East` enum variant.
///
/// This is a variant of [`xcb_gravity_t`].
pub const XCB_GRAVITY_EAST: xcb_gravity_t = 6;
/// The `Gravity::SouthWest` enum variant.
///
/// This is a variant of [`xcb_gravity_t`].
pub const XCB_GRAVITY_SOUTH_WEST: xcb_gravity_t = 7;
/// The `Gravity::South` enum variant.
///
/// This is a variant of [`xcb_gravity_t`].
pub const XCB_GRAVITY_SOUTH: xcb_gravity_t = 8;
/// The `Gravity::SouthEast` enum variant.
///
/// This is a variant of [`xcb_gravity_t`].
pub const XCB_GRAVITY_SOUTH_EAST: xcb_gravity_t = 9;
/// The `Gravity::Static` enum variant.
///
/// This is a variant of [`xcb_gravity_t`].
pub const XCB_GRAVITY_STATIC: xcb_gravity_t = 10;

/// The `value_list` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_create_window_value_list_t {
    pub background_pixmap: xcb_pixmap_t,
    pub background_pixel: u32,
    pub border_pixmap: xcb_pixmap_t,
    pub border_pixel: u32,
    pub bit_gravity: u32,
    pub win_gravity: u32,
    pub backing_store: u32,
    pub backing_planes: u32,
    pub backing_pixel: u32,
    pub override_redirect: xcb_bool32_t,
    pub save_under: xcb_bool32_t,
    pub event_mask: u32,
    pub do_not_propogate_mask: u32,
    pub colormap: xcb_colormap_t,
    pub cursor: xcb_cursor_t,
}

impl Default for xcb_create_window_value_list_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `CreateWindow` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_create_window_request_t`].
pub const XCB_CREATE_WINDOW: u8 = 1i32 as u8;

/// The `CreateWindow` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `value_list`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_create_window_request_t {
    pub major_opcode: u8,
    pub depth: u8,
    pub length: u16,
    pub wid: xcb_window_t,
    pub parent: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub class: u16,
    pub visual: xcb_visualid_t,
    pub value_mask: u32,
}

impl Default for xcb_create_window_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `value_list` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_change_window_attributes_value_list_t {
    pub background_pixmap: xcb_pixmap_t,
    pub background_pixel: u32,
    pub border_pixmap: xcb_pixmap_t,
    pub border_pixel: u32,
    pub bit_gravity: u32,
    pub win_gravity: u32,
    pub backing_store: u32,
    pub backing_planes: u32,
    pub backing_pixel: u32,
    pub override_redirect: xcb_bool32_t,
    pub save_under: xcb_bool32_t,
    pub event_mask: u32,
    pub do_not_propogate_mask: u32,
    pub colormap: xcb_colormap_t,
    pub cursor: xcb_cursor_t,
}

impl Default for xcb_change_window_attributes_value_list_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ChangeWindowAttributes` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_change_window_attributes_request_t`].
pub const XCB_CHANGE_WINDOW_ATTRIBUTES: u8 = 2i32 as u8;

/// The `ChangeWindowAttributes` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `value_list`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_change_window_attributes_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub value_mask: u32,
}

impl Default for xcb_change_window_attributes_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `MapState` enum.
///
/// This enum has the following variants:
///
/// - [`MapState::Unmapped`](XCB_MAP_STATE_UNMAPPED)
/// - [`MapState::Unviewable`](XCB_MAP_STATE_UNVIEWABLE)
/// - [`MapState::Viewable`](XCB_MAP_STATE_VIEWABLE)
pub type xcb_map_state_t = u32;
/// The `MapState::Unmapped` enum variant.
///
/// This is a variant of [`xcb_map_state_t`].
pub const XCB_MAP_STATE_UNMAPPED: xcb_map_state_t = 0;
/// The `MapState::Unviewable` enum variant.
///
/// This is a variant of [`xcb_map_state_t`].
pub const XCB_MAP_STATE_UNVIEWABLE: xcb_map_state_t = 1;
/// The `MapState::Viewable` enum variant.
///
/// This is a variant of [`xcb_map_state_t`].
pub const XCB_MAP_STATE_VIEWABLE: xcb_map_state_t = 2;

/// The cookie for the reply to a `GetWindowAttributes` request.
///
/// Pass this cookie to [`xcb_get_window_attributes_reply`] to retrieve the reply.
///
/// [`xcb_get_window_attributes_reply`]: Xcb::xcb_get_window_attributes_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_window_attributes_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_get_window_attributes_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `GetWindowAttributes` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_get_window_attributes_request_t`].
pub const XCB_GET_WINDOW_ATTRIBUTES: u8 = 3i32 as u8;

/// The `GetWindowAttributes` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_window_attributes_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_get_window_attributes_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `GetWindowAttributes` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_window_attributes_reply_t {
    pub response_type: u8,
    pub backing_store: u8,
    pub sequence: u16,
    pub length: u32,
    pub visual: xcb_visualid_t,
    pub class: u16,
    pub bit_gravity: u8,
    pub win_gravity: u8,
    pub backing_planes: u32,
    pub backing_pixel: u32,
    pub save_under: u8,
    pub map_is_installed: u8,
    pub map_state: u8,
    pub override_redirect: u8,
    pub colormap: xcb_colormap_t,
    pub all_event_masks: u32,
    pub your_event_mask: u32,
    pub do_not_propagate_mask: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_get_window_attributes_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DestroyWindow` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_destroy_window_request_t`].
pub const XCB_DESTROY_WINDOW: u8 = 4i32 as u8;

/// The `DestroyWindow` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_destroy_window_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_destroy_window_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DestroySubwindows` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_destroy_subwindows_request_t`].
pub const XCB_DESTROY_SUBWINDOWS: u8 = 5i32 as u8;

/// The `DestroySubwindows` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_destroy_subwindows_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_destroy_subwindows_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `SetMode` enum.
///
/// This enum has the following variants:
///
/// - [`SetMode::Insert`](XCB_SET_MODE_INSERT)
/// - [`SetMode::Delete`](XCB_SET_MODE_DELETE)
pub type xcb_set_mode_t = u32;
/// The `SetMode::Insert` enum variant.
///
/// This is a variant of [`xcb_set_mode_t`].
pub const XCB_SET_MODE_INSERT: xcb_set_mode_t = 0;
/// The `SetMode::Delete` enum variant.
///
/// This is a variant of [`xcb_set_mode_t`].
pub const XCB_SET_MODE_DELETE: xcb_set_mode_t = 1;

/// The opcode for `ChangeSaveSet` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_change_save_set_request_t`].
pub const XCB_CHANGE_SAVE_SET: u8 = 6i32 as u8;

/// The `ChangeSaveSet` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_change_save_set_request_t {
    pub major_opcode: u8,
    pub mode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_change_save_set_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ReparentWindow` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_reparent_window_request_t`].
pub const XCB_REPARENT_WINDOW: u8 = 7i32 as u8;

/// The `ReparentWindow` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_reparent_window_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub parent: xcb_window_t,
    pub x: i16,
    pub y: i16,
}

impl Default for xcb_reparent_window_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `MapWindow` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_map_window_request_t`].
pub const XCB_MAP_WINDOW: u8 = 8i32 as u8;

/// The `MapWindow` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_map_window_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_map_window_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `MapSubwindows` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_map_subwindows_request_t`].
pub const XCB_MAP_SUBWINDOWS: u8 = 9i32 as u8;

/// The `MapSubwindows` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_map_subwindows_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_map_subwindows_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `UnmapWindow` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_unmap_window_request_t`].
pub const XCB_UNMAP_WINDOW: u8 = 10i32 as u8;

/// The `UnmapWindow` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_unmap_window_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_unmap_window_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `UnmapSubwindows` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_unmap_subwindows_request_t`].
pub const XCB_UNMAP_SUBWINDOWS: u8 = 11i32 as u8;

/// The `UnmapSubwindows` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_unmap_subwindows_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_unmap_subwindows_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `ConfigWindow` enum.
///
/// This enum has the following variants:
///
/// - [`ConfigWindow::X`](XCB_CONFIG_WINDOW_X)
/// - [`ConfigWindow::Y`](XCB_CONFIG_WINDOW_Y)
/// - [`ConfigWindow::Width`](XCB_CONFIG_WINDOW_WIDTH)
/// - [`ConfigWindow::Height`](XCB_CONFIG_WINDOW_HEIGHT)
/// - [`ConfigWindow::BorderWidth`](XCB_CONFIG_WINDOW_BORDER_WIDTH)
/// - [`ConfigWindow::Sibling`](XCB_CONFIG_WINDOW_SIBLING)
/// - [`ConfigWindow::StackMode`](XCB_CONFIG_WINDOW_STACK_MODE)
pub type xcb_config_window_t = u32;
/// The `ConfigWindow::X` enum variant.
///
/// This is a variant of [`xcb_config_window_t`].
pub const XCB_CONFIG_WINDOW_X: xcb_config_window_t = 1;
/// The `ConfigWindow::Y` enum variant.
///
/// This is a variant of [`xcb_config_window_t`].
pub const XCB_CONFIG_WINDOW_Y: xcb_config_window_t = 2;
/// The `ConfigWindow::Width` enum variant.
///
/// This is a variant of [`xcb_config_window_t`].
pub const XCB_CONFIG_WINDOW_WIDTH: xcb_config_window_t = 4;
/// The `ConfigWindow::Height` enum variant.
///
/// This is a variant of [`xcb_config_window_t`].
pub const XCB_CONFIG_WINDOW_HEIGHT: xcb_config_window_t = 8;
/// The `ConfigWindow::BorderWidth` enum variant.
///
/// This is a variant of [`xcb_config_window_t`].
pub const XCB_CONFIG_WINDOW_BORDER_WIDTH: xcb_config_window_t = 16;
/// The `ConfigWindow::Sibling` enum variant.
///
/// This is a variant of [`xcb_config_window_t`].
pub const XCB_CONFIG_WINDOW_SIBLING: xcb_config_window_t = 32;
/// The `ConfigWindow::StackMode` enum variant.
///
/// This is a variant of [`xcb_config_window_t`].
pub const XCB_CONFIG_WINDOW_STACK_MODE: xcb_config_window_t = 64;

/// The `StackMode` enum.
///
/// This enum has the following variants:
///
/// - [`StackMode::Above`](XCB_STACK_MODE_ABOVE)
/// - [`StackMode::Below`](XCB_STACK_MODE_BELOW)
/// - [`StackMode::TopIf`](XCB_STACK_MODE_TOP_IF)
/// - [`StackMode::BottomIf`](XCB_STACK_MODE_BOTTOM_IF)
/// - [`StackMode::Opposite`](XCB_STACK_MODE_OPPOSITE)
pub type xcb_stack_mode_t = u32;
/// The `StackMode::Above` enum variant.
///
/// This is a variant of [`xcb_stack_mode_t`].
pub const XCB_STACK_MODE_ABOVE: xcb_stack_mode_t = 0;
/// The `StackMode::Below` enum variant.
///
/// This is a variant of [`xcb_stack_mode_t`].
pub const XCB_STACK_MODE_BELOW: xcb_stack_mode_t = 1;
/// The `StackMode::TopIf` enum variant.
///
/// This is a variant of [`xcb_stack_mode_t`].
pub const XCB_STACK_MODE_TOP_IF: xcb_stack_mode_t = 2;
/// The `StackMode::BottomIf` enum variant.
///
/// This is a variant of [`xcb_stack_mode_t`].
pub const XCB_STACK_MODE_BOTTOM_IF: xcb_stack_mode_t = 3;
/// The `StackMode::Opposite` enum variant.
///
/// This is a variant of [`xcb_stack_mode_t`].
pub const XCB_STACK_MODE_OPPOSITE: xcb_stack_mode_t = 4;

/// The `value_list` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_configure_window_value_list_t {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub border_width: u32,
    pub sibling: xcb_window_t,
    pub stack_mode: u32,
}

impl Default for xcb_configure_window_value_list_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ConfigureWindow` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_configure_window_request_t`].
pub const XCB_CONFIGURE_WINDOW: u8 = 12i32 as u8;

/// The `ConfigureWindow` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `value_list`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_configure_window_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub value_mask: u16,
    pub pad1: [u8; 2],
}

impl Default for xcb_configure_window_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Circulate` enum.
///
/// This enum has the following variants:
///
/// - [`Circulate::RaiseLowest`](XCB_CIRCULATE_RAISE_LOWEST)
/// - [`Circulate::LowerHighest`](XCB_CIRCULATE_LOWER_HIGHEST)
pub type xcb_circulate_t = u32;
/// The `Circulate::RaiseLowest` enum variant.
///
/// This is a variant of [`xcb_circulate_t`].
pub const XCB_CIRCULATE_RAISE_LOWEST: xcb_circulate_t = 0;
/// The `Circulate::LowerHighest` enum variant.
///
/// This is a variant of [`xcb_circulate_t`].
pub const XCB_CIRCULATE_LOWER_HIGHEST: xcb_circulate_t = 1;

/// The opcode for `CirculateWindow` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_circulate_window_request_t`].
pub const XCB_CIRCULATE_WINDOW: u8 = 13i32 as u8;

/// The `CirculateWindow` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_circulate_window_request_t {
    pub major_opcode: u8,
    pub direction: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_circulate_window_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `GetGeometry` request.
///
/// Pass this cookie to [`xcb_get_geometry_reply`] to retrieve the reply.
///
/// [`xcb_get_geometry_reply`]: Xcb::xcb_get_geometry_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_geometry_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_get_geometry_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `GetGeometry` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_get_geometry_request_t`].
pub const XCB_GET_GEOMETRY: u8 = 14i32 as u8;

/// The `GetGeometry` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_geometry_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
}

impl Default for xcb_get_geometry_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `GetGeometry` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_geometry_reply_t {
    pub response_type: u8,
    pub depth: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_get_geometry_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `QueryTree` request.
///
/// Pass this cookie to [`xcb_query_tree_reply`] to retrieve the reply.
///
/// [`xcb_query_tree_reply`]: Xcb::xcb_query_tree_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_tree_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_query_tree_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `QueryTree` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_query_tree_request_t`].
pub const XCB_QUERY_TREE: u8 = 15i32 as u8;

/// The `QueryTree` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_tree_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_query_tree_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `QueryTree` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `children`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_tree_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: xcb_window_t,
    pub parent: xcb_window_t,
    pub children_len: u16,
    pub pad1: [u8; 14],
}

impl Default for xcb_query_tree_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `InternAtom` request.
///
/// Pass this cookie to [`xcb_intern_atom_reply`] to retrieve the reply.
///
/// [`xcb_intern_atom_reply`]: Xcb::xcb_intern_atom_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_intern_atom_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_intern_atom_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `InternAtom` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_intern_atom_request_t`].
pub const XCB_INTERN_ATOM: u8 = 16i32 as u8;

/// The `InternAtom` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `name`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_intern_atom_request_t {
    pub major_opcode: u8,
    pub only_if_exists: u8,
    pub length: u16,
    pub name_len: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_intern_atom_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `InternAtom` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_intern_atom_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub atom: xcb_atom_t,
}

impl Default for xcb_intern_atom_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `GetAtomName` request.
///
/// Pass this cookie to [`xcb_get_atom_name_reply`] to retrieve the reply.
///
/// [`xcb_get_atom_name_reply`]: Xcb::xcb_get_atom_name_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_atom_name_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_get_atom_name_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `GetAtomName` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_get_atom_name_request_t`].
pub const XCB_GET_ATOM_NAME: u8 = 17i32 as u8;

/// The `GetAtomName` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_atom_name_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub atom: xcb_atom_t,
}

impl Default for xcb_get_atom_name_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `GetAtomName` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `name`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_atom_name_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub name_len: u16,
    pub pad1: [u8; 22],
}

impl Default for xcb_get_atom_name_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `PropMode` enum.
///
/// This enum has the following variants:
///
/// - [`PropMode::Replace`](XCB_PROP_MODE_REPLACE)
/// - [`PropMode::Prepend`](XCB_PROP_MODE_PREPEND)
/// - [`PropMode::Append`](XCB_PROP_MODE_APPEND)
pub type xcb_prop_mode_t = u32;
/// The `PropMode::Replace` enum variant.
///
/// This is a variant of [`xcb_prop_mode_t`].
pub const XCB_PROP_MODE_REPLACE: xcb_prop_mode_t = 0;
/// The `PropMode::Prepend` enum variant.
///
/// This is a variant of [`xcb_prop_mode_t`].
pub const XCB_PROP_MODE_PREPEND: xcb_prop_mode_t = 1;
/// The `PropMode::Append` enum variant.
///
/// This is a variant of [`xcb_prop_mode_t`].
pub const XCB_PROP_MODE_APPEND: xcb_prop_mode_t = 2;

/// The opcode for `ChangeProperty` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_change_property_request_t`].
pub const XCB_CHANGE_PROPERTY: u8 = 18i32 as u8;

/// The `ChangeProperty` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `data`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_change_property_request_t {
    pub major_opcode: u8,
    pub mode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub property: xcb_atom_t,
    pub type_: xcb_atom_t,
    pub format: u8,
    pub pad0: [u8; 3],
    pub data_len: u32,
}

impl Default for xcb_change_property_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `DeleteProperty` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_delete_property_request_t`].
pub const XCB_DELETE_PROPERTY: u8 = 19i32 as u8;

/// The `DeleteProperty` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_delete_property_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub property: xcb_atom_t,
}

impl Default for xcb_delete_property_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `GetPropertyType` enum.
///
/// This enum has the following variants:
///
/// - [`GetPropertyType::Any`](XCB_GET_PROPERTY_TYPE_ANY)
pub type xcb_get_property_type_t = u32;
/// The `GetPropertyType::Any` enum variant.
///
/// This is a variant of [`xcb_get_property_type_t`].
pub const XCB_GET_PROPERTY_TYPE_ANY: xcb_get_property_type_t = 0;

/// The cookie for the reply to a `GetProperty` request.
///
/// Pass this cookie to [`xcb_get_property_reply`] to retrieve the reply.
///
/// [`xcb_get_property_reply`]: Xcb::xcb_get_property_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_property_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_get_property_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `GetProperty` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_get_property_request_t`].
pub const XCB_GET_PROPERTY: u8 = 20i32 as u8;

/// The `GetProperty` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_property_request_t {
    pub major_opcode: u8,
    pub delete: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub property: xcb_atom_t,
    pub type_: xcb_atom_t,
    pub long_offset: u32,
    pub long_length: u32,
}

impl Default for xcb_get_property_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `GetProperty` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `value`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_property_reply_t {
    pub response_type: u8,
    pub format: u8,
    pub sequence: u16,
    pub length: u32,
    pub type_: xcb_atom_t,
    pub bytes_after: u32,
    pub value_len: u32,
    pub pad0: [u8; 12],
}

impl Default for xcb_get_property_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `ListProperties` request.
///
/// Pass this cookie to [`xcb_list_properties_reply`] to retrieve the reply.
///
/// [`xcb_list_properties_reply`]: Xcb::xcb_list_properties_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_properties_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_list_properties_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ListProperties` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_list_properties_request_t`].
pub const XCB_LIST_PROPERTIES: u8 = 21i32 as u8;

/// The `ListProperties` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_properties_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_list_properties_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `ListProperties` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `atoms`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_properties_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub atoms_len: u16,
    pub pad1: [u8; 22],
}

impl Default for xcb_list_properties_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SetSelectionOwner` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_set_selection_owner_request_t`].
pub const XCB_SET_SELECTION_OWNER: u8 = 22i32 as u8;

/// The `SetSelectionOwner` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_set_selection_owner_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub owner: xcb_window_t,
    pub selection: xcb_atom_t,
    pub time: xcb_timestamp_t,
}

impl Default for xcb_set_selection_owner_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `GetSelectionOwner` request.
///
/// Pass this cookie to [`xcb_get_selection_owner_reply`] to retrieve the reply.
///
/// [`xcb_get_selection_owner_reply`]: Xcb::xcb_get_selection_owner_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_selection_owner_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_get_selection_owner_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `GetSelectionOwner` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_get_selection_owner_request_t`].
pub const XCB_GET_SELECTION_OWNER: u8 = 23i32 as u8;

/// The `GetSelectionOwner` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_selection_owner_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub selection: xcb_atom_t,
}

impl Default for xcb_get_selection_owner_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `GetSelectionOwner` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_selection_owner_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub owner: xcb_window_t,
}

impl Default for xcb_get_selection_owner_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ConvertSelection` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_convert_selection_request_t`].
pub const XCB_CONVERT_SELECTION: u8 = 24i32 as u8;

/// The `ConvertSelection` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_convert_selection_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub requestor: xcb_window_t,
    pub selection: xcb_atom_t,
    pub target: xcb_atom_t,
    pub property: xcb_atom_t,
    pub time: xcb_timestamp_t,
}

impl Default for xcb_convert_selection_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `SendEventDest` enum.
///
/// This enum has the following variants:
///
/// - [`SendEventDest::PointerWindow`](XCB_SEND_EVENT_DEST_POINTER_WINDOW)
/// - [`SendEventDest::ItemFocus`](XCB_SEND_EVENT_DEST_ITEM_FOCUS)
pub type xcb_send_event_dest_t = u32;
/// The `SendEventDest::PointerWindow` enum variant.
///
/// This is a variant of [`xcb_send_event_dest_t`].
pub const XCB_SEND_EVENT_DEST_POINTER_WINDOW: xcb_send_event_dest_t = 0;
/// The `SendEventDest::ItemFocus` enum variant.
///
/// This is a variant of [`xcb_send_event_dest_t`].
pub const XCB_SEND_EVENT_DEST_ITEM_FOCUS: xcb_send_event_dest_t = 1;

/// The opcode for `SendEvent` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_send_event_request_t`].
pub const XCB_SEND_EVENT: u8 = 25i32 as u8;

/// The `SendEvent` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_send_event_request_t {
    pub major_opcode: u8,
    pub propagate: u8,
    pub length: u16,
    pub destination: xcb_window_t,
    pub event_mask: u32,
    pub event: [c_char; 32],
}

impl Default for xcb_send_event_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `GrabMode` enum.
///
/// This enum has the following variants:
///
/// - [`GrabMode::Sync`](XCB_GRAB_MODE_SYNC)
/// - [`GrabMode::Async`](XCB_GRAB_MODE_ASYNC)
pub type xcb_grab_mode_t = u32;
/// The `GrabMode::Sync` enum variant.
///
/// This is a variant of [`xcb_grab_mode_t`].
pub const XCB_GRAB_MODE_SYNC: xcb_grab_mode_t = 0;
/// The `GrabMode::Async` enum variant.
///
/// This is a variant of [`xcb_grab_mode_t`].
pub const XCB_GRAB_MODE_ASYNC: xcb_grab_mode_t = 1;

/// The `GrabStatus` enum.
///
/// This enum has the following variants:
///
/// - [`GrabStatus::Success`](XCB_GRAB_STATUS_SUCCESS)
/// - [`GrabStatus::AlreadyGrabbed`](XCB_GRAB_STATUS_ALREADY_GRABBED)
/// - [`GrabStatus::InvalidTime`](XCB_GRAB_STATUS_INVALID_TIME)
/// - [`GrabStatus::NotViewable`](XCB_GRAB_STATUS_NOT_VIEWABLE)
/// - [`GrabStatus::Frozen`](XCB_GRAB_STATUS_FROZEN)
pub type xcb_grab_status_t = u32;
/// The `GrabStatus::Success` enum variant.
///
/// This is a variant of [`xcb_grab_status_t`].
pub const XCB_GRAB_STATUS_SUCCESS: xcb_grab_status_t = 0;
/// The `GrabStatus::AlreadyGrabbed` enum variant.
///
/// This is a variant of [`xcb_grab_status_t`].
pub const XCB_GRAB_STATUS_ALREADY_GRABBED: xcb_grab_status_t = 1;
/// The `GrabStatus::InvalidTime` enum variant.
///
/// This is a variant of [`xcb_grab_status_t`].
pub const XCB_GRAB_STATUS_INVALID_TIME: xcb_grab_status_t = 2;
/// The `GrabStatus::NotViewable` enum variant.
///
/// This is a variant of [`xcb_grab_status_t`].
pub const XCB_GRAB_STATUS_NOT_VIEWABLE: xcb_grab_status_t = 3;
/// The `GrabStatus::Frozen` enum variant.
///
/// This is a variant of [`xcb_grab_status_t`].
pub const XCB_GRAB_STATUS_FROZEN: xcb_grab_status_t = 4;

/// The `Cursor` enum.
///
/// This enum has the following variants:
///
/// - [`Cursor::None`](XCB_CURSOR_NONE)
pub type xcb_cursor_enum_t = u32;
/// The `Cursor::None` enum variant.
///
/// This is a variant of [`xcb_cursor_enum_t`].
pub const XCB_CURSOR_NONE: xcb_cursor_enum_t = 0;

/// The cookie for the reply to a `GrabPointer` request.
///
/// Pass this cookie to [`xcb_grab_pointer_reply`] to retrieve the reply.
///
/// [`xcb_grab_pointer_reply`]: Xcb::xcb_grab_pointer_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_grab_pointer_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_grab_pointer_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `GrabPointer` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_grab_pointer_request_t`].
pub const XCB_GRAB_POINTER: u8 = 26i32 as u8;

/// The `GrabPointer` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_grab_pointer_request_t {
    pub major_opcode: u8,
    pub owner_events: u8,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub event_mask: u16,
    pub pointer_mode: u8,
    pub keyboard_mode: u8,
    pub confine_to: xcb_window_t,
    pub cursor: xcb_cursor_t,
    pub time: xcb_timestamp_t,
}

impl Default for xcb_grab_pointer_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `GrabPointer` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_grab_pointer_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
}

impl Default for xcb_grab_pointer_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `UngrabPointer` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_ungrab_pointer_request_t`].
pub const XCB_UNGRAB_POINTER: u8 = 27i32 as u8;

/// The `UngrabPointer` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_ungrab_pointer_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub time: xcb_timestamp_t,
}

impl Default for xcb_ungrab_pointer_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `ButtonIndex` enum.
///
/// This enum has the following variants:
///
/// - [`ButtonIndex::Any`](XCB_BUTTON_INDEX_ANY)
/// - [`ButtonIndex::1`](XCB_BUTTON_INDEX_1)
/// - [`ButtonIndex::2`](XCB_BUTTON_INDEX_2)
/// - [`ButtonIndex::3`](XCB_BUTTON_INDEX_3)
/// - [`ButtonIndex::4`](XCB_BUTTON_INDEX_4)
/// - [`ButtonIndex::5`](XCB_BUTTON_INDEX_5)
pub type xcb_button_index_t = u32;
/// The `ButtonIndex::Any` enum variant.
///
/// This is a variant of [`xcb_button_index_t`].
pub const XCB_BUTTON_INDEX_ANY: xcb_button_index_t = 0;
/// The `ButtonIndex::1` enum variant.
///
/// This is a variant of [`xcb_button_index_t`].
pub const XCB_BUTTON_INDEX_1: xcb_button_index_t = 1;
/// The `ButtonIndex::2` enum variant.
///
/// This is a variant of [`xcb_button_index_t`].
pub const XCB_BUTTON_INDEX_2: xcb_button_index_t = 2;
/// The `ButtonIndex::3` enum variant.
///
/// This is a variant of [`xcb_button_index_t`].
pub const XCB_BUTTON_INDEX_3: xcb_button_index_t = 3;
/// The `ButtonIndex::4` enum variant.
///
/// This is a variant of [`xcb_button_index_t`].
pub const XCB_BUTTON_INDEX_4: xcb_button_index_t = 4;
/// The `ButtonIndex::5` enum variant.
///
/// This is a variant of [`xcb_button_index_t`].
pub const XCB_BUTTON_INDEX_5: xcb_button_index_t = 5;

/// The opcode for `GrabButton` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_grab_button_request_t`].
pub const XCB_GRAB_BUTTON: u8 = 28i32 as u8;

/// The `GrabButton` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_grab_button_request_t {
    pub major_opcode: u8,
    pub owner_events: u8,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub event_mask: u16,
    pub pointer_mode: u8,
    pub keyboard_mode: u8,
    pub confine_to: xcb_window_t,
    pub cursor: xcb_cursor_t,
    pub button: u8,
    pub pad0: u8,
    pub modifiers: u16,
}

impl Default for xcb_grab_button_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `UngrabButton` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_ungrab_button_request_t`].
pub const XCB_UNGRAB_BUTTON: u8 = 29i32 as u8;

/// The `UngrabButton` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_ungrab_button_request_t {
    pub major_opcode: u8,
    pub button: u8,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub modifiers: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_ungrab_button_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ChangeActivePointerGrab` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_change_active_pointer_grab_request_t`].
pub const XCB_CHANGE_ACTIVE_POINTER_GRAB: u8 = 30i32 as u8;

/// The `ChangeActivePointerGrab` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_change_active_pointer_grab_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cursor: xcb_cursor_t,
    pub time: xcb_timestamp_t,
    pub event_mask: u16,
    pub pad1: [u8; 2],
}

impl Default for xcb_change_active_pointer_grab_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `GrabKeyboard` request.
///
/// Pass this cookie to [`xcb_grab_keyboard_reply`] to retrieve the reply.
///
/// [`xcb_grab_keyboard_reply`]: Xcb::xcb_grab_keyboard_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_grab_keyboard_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_grab_keyboard_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `GrabKeyboard` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_grab_keyboard_request_t`].
pub const XCB_GRAB_KEYBOARD: u8 = 31i32 as u8;

/// The `GrabKeyboard` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_grab_keyboard_request_t {
    pub major_opcode: u8,
    pub owner_events: u8,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub time: xcb_timestamp_t,
    pub pointer_mode: u8,
    pub keyboard_mode: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_grab_keyboard_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `GrabKeyboard` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_grab_keyboard_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
}

impl Default for xcb_grab_keyboard_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `UngrabKeyboard` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_ungrab_keyboard_request_t`].
pub const XCB_UNGRAB_KEYBOARD: u8 = 32i32 as u8;

/// The `UngrabKeyboard` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_ungrab_keyboard_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub time: xcb_timestamp_t,
}

impl Default for xcb_ungrab_keyboard_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Grab` enum.
///
/// This enum has the following variants:
///
/// - [`Grab::Any`](XCB_GRAB_ANY)
pub type xcb_grab_t = u32;
/// The `Grab::Any` enum variant.
///
/// This is a variant of [`xcb_grab_t`].
pub const XCB_GRAB_ANY: xcb_grab_t = 0;

/// The opcode for `GrabKey` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_grab_key_request_t`].
pub const XCB_GRAB_KEY: u8 = 33i32 as u8;

/// The `GrabKey` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_grab_key_request_t {
    pub major_opcode: u8,
    pub owner_events: u8,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub modifiers: u16,
    pub key: xcb_keycode_t,
    pub pointer_mode: u8,
    pub keyboard_mode: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_grab_key_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `UngrabKey` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_ungrab_key_request_t`].
pub const XCB_UNGRAB_KEY: u8 = 34i32 as u8;

/// The `UngrabKey` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_ungrab_key_request_t {
    pub major_opcode: u8,
    pub key: xcb_keycode_t,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub modifiers: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_ungrab_key_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Allow` enum.
///
/// This enum has the following variants:
///
/// - [`Allow::AsyncPointer`](XCB_ALLOW_ASYNC_POINTER)
/// - [`Allow::SyncPointer`](XCB_ALLOW_SYNC_POINTER)
/// - [`Allow::ReplayPointer`](XCB_ALLOW_REPLAY_POINTER)
/// - [`Allow::AsyncKeyboard`](XCB_ALLOW_ASYNC_KEYBOARD)
/// - [`Allow::SyncKeyboard`](XCB_ALLOW_SYNC_KEYBOARD)
/// - [`Allow::ReplayKeyboard`](XCB_ALLOW_REPLAY_KEYBOARD)
/// - [`Allow::AsyncBoth`](XCB_ALLOW_ASYNC_BOTH)
/// - [`Allow::SyncBoth`](XCB_ALLOW_SYNC_BOTH)
pub type xcb_allow_t = u32;
/// The `Allow::AsyncPointer` enum variant.
///
/// This is a variant of [`xcb_allow_t`].
pub const XCB_ALLOW_ASYNC_POINTER: xcb_allow_t = 0;
/// The `Allow::SyncPointer` enum variant.
///
/// This is a variant of [`xcb_allow_t`].
pub const XCB_ALLOW_SYNC_POINTER: xcb_allow_t = 1;
/// The `Allow::ReplayPointer` enum variant.
///
/// This is a variant of [`xcb_allow_t`].
pub const XCB_ALLOW_REPLAY_POINTER: xcb_allow_t = 2;
/// The `Allow::AsyncKeyboard` enum variant.
///
/// This is a variant of [`xcb_allow_t`].
pub const XCB_ALLOW_ASYNC_KEYBOARD: xcb_allow_t = 3;
/// The `Allow::SyncKeyboard` enum variant.
///
/// This is a variant of [`xcb_allow_t`].
pub const XCB_ALLOW_SYNC_KEYBOARD: xcb_allow_t = 4;
/// The `Allow::ReplayKeyboard` enum variant.
///
/// This is a variant of [`xcb_allow_t`].
pub const XCB_ALLOW_REPLAY_KEYBOARD: xcb_allow_t = 5;
/// The `Allow::AsyncBoth` enum variant.
///
/// This is a variant of [`xcb_allow_t`].
pub const XCB_ALLOW_ASYNC_BOTH: xcb_allow_t = 6;
/// The `Allow::SyncBoth` enum variant.
///
/// This is a variant of [`xcb_allow_t`].
pub const XCB_ALLOW_SYNC_BOTH: xcb_allow_t = 7;

/// The opcode for `AllowEvents` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_allow_events_request_t`].
pub const XCB_ALLOW_EVENTS: u8 = 35i32 as u8;

/// The `AllowEvents` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_allow_events_request_t {
    pub major_opcode: u8,
    pub mode: u8,
    pub length: u16,
    pub time: xcb_timestamp_t,
}

impl Default for xcb_allow_events_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `GrabServer` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_grab_server_request_t`].
pub const XCB_GRAB_SERVER: u8 = 36i32 as u8;

/// The `GrabServer` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_grab_server_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}

impl Default for xcb_grab_server_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `UngrabServer` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_ungrab_server_request_t`].
pub const XCB_UNGRAB_SERVER: u8 = 37i32 as u8;

/// The `UngrabServer` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_ungrab_server_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}

impl Default for xcb_ungrab_server_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `QueryPointer` request.
///
/// Pass this cookie to [`xcb_query_pointer_reply`] to retrieve the reply.
///
/// [`xcb_query_pointer_reply`]: Xcb::xcb_query_pointer_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_pointer_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_query_pointer_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `QueryPointer` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_query_pointer_request_t`].
pub const XCB_QUERY_POINTER: u8 = 38i32 as u8;

/// The `QueryPointer` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_pointer_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_query_pointer_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `QueryPointer` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_pointer_reply_t {
    pub response_type: u8,
    pub same_screen: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: i16,
    pub root_y: i16,
    pub win_x: i16,
    pub win_y: i16,
    pub mask: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_query_pointer_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `TIMECOORD` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_timecoord_t {
    pub time: xcb_timestamp_t,
    pub x: i16,
    pub y: i16,
}

impl Default for xcb_timecoord_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `TIMECOORD` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_timecoord_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_timecoord_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_timecoord_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `GetMotionEvents` request.
///
/// Pass this cookie to [`xcb_get_motion_events_reply`] to retrieve the reply.
///
/// [`xcb_get_motion_events_reply`]: Xcb::xcb_get_motion_events_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_motion_events_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_get_motion_events_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `GetMotionEvents` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_get_motion_events_request_t`].
pub const XCB_GET_MOTION_EVENTS: u8 = 39i32 as u8;

/// The `GetMotionEvents` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_motion_events_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub start: xcb_timestamp_t,
    pub stop: xcb_timestamp_t,
}

impl Default for xcb_get_motion_events_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `GetMotionEvents` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `events`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_motion_events_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub events_len: u32,
    pub pad1: [u8; 20],
}

impl Default for xcb_get_motion_events_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `TranslateCoordinates` request.
///
/// Pass this cookie to [`xcb_translate_coordinates_reply`] to retrieve the reply.
///
/// [`xcb_translate_coordinates_reply`]: Xcb::xcb_translate_coordinates_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_translate_coordinates_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_translate_coordinates_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `TranslateCoordinates` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_translate_coordinates_request_t`].
pub const XCB_TRANSLATE_COORDINATES: u8 = 40i32 as u8;

/// The `TranslateCoordinates` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_translate_coordinates_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub src_window: xcb_window_t,
    pub dst_window: xcb_window_t,
    pub src_x: i16,
    pub src_y: i16,
}

impl Default for xcb_translate_coordinates_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `TranslateCoordinates` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_translate_coordinates_reply_t {
    pub response_type: u8,
    pub same_screen: u8,
    pub sequence: u16,
    pub length: u32,
    pub child: xcb_window_t,
    pub dst_x: i16,
    pub dst_y: i16,
}

impl Default for xcb_translate_coordinates_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `WarpPointer` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_warp_pointer_request_t`].
pub const XCB_WARP_POINTER: u8 = 41i32 as u8;

/// The `WarpPointer` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_warp_pointer_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub src_window: xcb_window_t,
    pub dst_window: xcb_window_t,
    pub src_x: i16,
    pub src_y: i16,
    pub src_width: u16,
    pub src_height: u16,
    pub dst_x: i16,
    pub dst_y: i16,
}

impl Default for xcb_warp_pointer_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `InputFocus` enum.
///
/// This enum has the following variants:
///
/// - [`InputFocus::None`](XCB_INPUT_FOCUS_NONE)
/// - [`InputFocus::PointerRoot`](XCB_INPUT_FOCUS_POINTER_ROOT)
/// - [`InputFocus::Parent`](XCB_INPUT_FOCUS_PARENT)
/// - [`InputFocus::FollowKeyboard`](XCB_INPUT_FOCUS_FOLLOW_KEYBOARD)
pub type xcb_input_focus_t = u32;
/// The `InputFocus::None` enum variant.
///
/// This is a variant of [`xcb_input_focus_t`].
pub const XCB_INPUT_FOCUS_NONE: xcb_input_focus_t = 0;
/// The `InputFocus::PointerRoot` enum variant.
///
/// This is a variant of [`xcb_input_focus_t`].
pub const XCB_INPUT_FOCUS_POINTER_ROOT: xcb_input_focus_t = 1;
/// The `InputFocus::Parent` enum variant.
///
/// This is a variant of [`xcb_input_focus_t`].
pub const XCB_INPUT_FOCUS_PARENT: xcb_input_focus_t = 2;
/// The `InputFocus::FollowKeyboard` enum variant.
///
/// This is a variant of [`xcb_input_focus_t`].
pub const XCB_INPUT_FOCUS_FOLLOW_KEYBOARD: xcb_input_focus_t = 3;

/// The opcode for `SetInputFocus` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_set_input_focus_request_t`].
pub const XCB_SET_INPUT_FOCUS: u8 = 42i32 as u8;

/// The `SetInputFocus` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_set_input_focus_request_t {
    pub major_opcode: u8,
    pub revert_to: u8,
    pub length: u16,
    pub focus: xcb_window_t,
    pub time: xcb_timestamp_t,
}

impl Default for xcb_set_input_focus_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `GetInputFocus` request.
///
/// Pass this cookie to [`xcb_get_input_focus_reply`] to retrieve the reply.
///
/// [`xcb_get_input_focus_reply`]: Xcb::xcb_get_input_focus_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_input_focus_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_get_input_focus_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `GetInputFocus` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_get_input_focus_request_t`].
pub const XCB_GET_INPUT_FOCUS: u8 = 43i32 as u8;

/// The `GetInputFocus` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_input_focus_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}

impl Default for xcb_get_input_focus_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `GetInputFocus` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_input_focus_reply_t {
    pub response_type: u8,
    pub revert_to: u8,
    pub sequence: u16,
    pub length: u32,
    pub focus: xcb_window_t,
}

impl Default for xcb_get_input_focus_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `QueryKeymap` request.
///
/// Pass this cookie to [`xcb_query_keymap_reply`] to retrieve the reply.
///
/// [`xcb_query_keymap_reply`]: Xcb::xcb_query_keymap_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_keymap_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_query_keymap_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `QueryKeymap` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_query_keymap_request_t`].
pub const XCB_QUERY_KEYMAP: u8 = 44i32 as u8;

/// The `QueryKeymap` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_keymap_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}

impl Default for xcb_query_keymap_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `QueryKeymap` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_keymap_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub keys: [u8; 32],
}

impl Default for xcb_query_keymap_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `OpenFont` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_open_font_request_t`].
pub const XCB_OPEN_FONT: u8 = 45i32 as u8;

/// The `OpenFont` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `name`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_open_font_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub fid: xcb_font_t,
    pub name_len: u16,
    pub pad1: [u8; 2],
}

impl Default for xcb_open_font_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `CloseFont` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_close_font_request_t`].
pub const XCB_CLOSE_FONT: u8 = 46i32 as u8;

/// The `CloseFont` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_close_font_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub font: xcb_font_t,
}

impl Default for xcb_close_font_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `FontDraw` enum.
///
/// This enum has the following variants:
///
/// - [`FontDraw::LeftToRight`](XCB_FONT_DRAW_LEFT_TO_RIGHT)
/// - [`FontDraw::RightToLeft`](XCB_FONT_DRAW_RIGHT_TO_LEFT)
pub type xcb_font_draw_t = u32;
/// The `FontDraw::LeftToRight` enum variant.
///
/// This is a variant of [`xcb_font_draw_t`].
pub const XCB_FONT_DRAW_LEFT_TO_RIGHT: xcb_font_draw_t = 0;
/// The `FontDraw::RightToLeft` enum variant.
///
/// This is a variant of [`xcb_font_draw_t`].
pub const XCB_FONT_DRAW_RIGHT_TO_LEFT: xcb_font_draw_t = 1;

/// The `FONTPROP` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_fontprop_t {
    pub name: xcb_atom_t,
    pub value: u32,
}

impl Default for xcb_fontprop_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `FONTPROP` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_fontprop_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_fontprop_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_fontprop_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `CHARINFO` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_charinfo_t {
    pub left_side_bearing: i16,
    pub right_side_bearing: i16,
    pub character_width: i16,
    pub ascent: i16,
    pub descent: i16,
    pub attributes: u16,
}

impl Default for xcb_charinfo_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `CHARINFO` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_charinfo_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_charinfo_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_charinfo_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `QueryFont` request.
///
/// Pass this cookie to [`xcb_query_font_reply`] to retrieve the reply.
///
/// [`xcb_query_font_reply`]: Xcb::xcb_query_font_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_font_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_query_font_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `QueryFont` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_query_font_request_t`].
pub const XCB_QUERY_FONT: u8 = 47i32 as u8;

/// The `QueryFont` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_font_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub font: xcb_fontable_t,
}

impl Default for xcb_query_font_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `QueryFont` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `properties`
/// - `char_infos`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_font_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub min_bounds: xcb_charinfo_t,
    pub pad1: [u8; 4],
    pub max_bounds: xcb_charinfo_t,
    pub pad2: [u8; 4],
    pub min_char_or_byte2: u16,
    pub max_char_or_byte2: u16,
    pub default_char: u16,
    pub properties_len: u16,
    pub draw_direction: u8,
    pub min_byte1: u8,
    pub max_byte1: u8,
    pub all_chars_exist: u8,
    pub font_ascent: i16,
    pub font_descent: i16,
    pub char_infos_len: u32,
}

impl Default for xcb_query_font_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `QueryTextExtents` request.
///
/// Pass this cookie to [`xcb_query_text_extents_reply`] to retrieve the reply.
///
/// [`xcb_query_text_extents_reply`]: Xcb::xcb_query_text_extents_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_text_extents_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_query_text_extents_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `QueryTextExtents` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_query_text_extents_request_t`].
pub const XCB_QUERY_TEXT_EXTENTS: u8 = 48i32 as u8;

/// The `QueryTextExtents` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `string`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_text_extents_request_t {
    pub major_opcode: u8,
    pub odd_length: u8,
    pub length: u16,
    pub font: xcb_fontable_t,
}

impl Default for xcb_query_text_extents_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `QueryTextExtents` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_text_extents_reply_t {
    pub response_type: u8,
    pub draw_direction: u8,
    pub sequence: u16,
    pub length: u32,
    pub font_ascent: i16,
    pub font_descent: i16,
    pub overall_ascent: i16,
    pub overall_descent: i16,
    pub overall_width: i32,
    pub overall_left: i32,
    pub overall_right: i32,
}

impl Default for xcb_query_text_extents_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `STR` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `name`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_str_t {
    pub name_len: u8,
}

impl Default for xcb_str_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `STR` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_str_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_str_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_str_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `ListFonts` request.
///
/// Pass this cookie to [`xcb_list_fonts_reply`] to retrieve the reply.
///
/// [`xcb_list_fonts_reply`]: Xcb::xcb_list_fonts_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_fonts_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_list_fonts_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ListFonts` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_list_fonts_request_t`].
pub const XCB_LIST_FONTS: u8 = 49i32 as u8;

/// The `ListFonts` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `pattern`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_fonts_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub max_names: u16,
    pub pattern_len: u16,
}

impl Default for xcb_list_fonts_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `ListFonts` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `names`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_fonts_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub names_len: u16,
    pub pad1: [u8; 22],
}

impl Default for xcb_list_fonts_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `ListFontsWithInfo` request.
///
/// Pass this cookie to [`xcb_list_fonts_with_info_reply`] to retrieve the reply.
///
/// [`xcb_list_fonts_with_info_reply`]: Xcb::xcb_list_fonts_with_info_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_fonts_with_info_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_list_fonts_with_info_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ListFontsWithInfo` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_list_fonts_with_info_request_t`].
pub const XCB_LIST_FONTS_WITH_INFO: u8 = 50i32 as u8;

/// The `ListFontsWithInfo` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `pattern`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_fonts_with_info_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub max_names: u16,
    pub pattern_len: u16,
}

impl Default for xcb_list_fonts_with_info_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `ListFontsWithInfo` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `properties`
/// - `name`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_fonts_with_info_reply_t {
    pub response_type: u8,
    pub name_len: u8,
    pub sequence: u16,
    pub length: u32,
    pub min_bounds: xcb_charinfo_t,
    pub pad0: [u8; 4],
    pub max_bounds: xcb_charinfo_t,
    pub pad1: [u8; 4],
    pub min_char_or_byte2: u16,
    pub max_char_or_byte2: u16,
    pub default_char: u16,
    pub properties_len: u16,
    pub draw_direction: u8,
    pub min_byte1: u8,
    pub max_byte1: u8,
    pub all_chars_exist: u8,
    pub font_ascent: i16,
    pub font_descent: i16,
    pub replies_hint: u32,
}

impl Default for xcb_list_fonts_with_info_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SetFontPath` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_set_font_path_request_t`].
pub const XCB_SET_FONT_PATH: u8 = 51i32 as u8;

/// The `SetFontPath` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `font`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_set_font_path_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub font_qty: u16,
    pub pad1: [u8; 2],
}

impl Default for xcb_set_font_path_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `GetFontPath` request.
///
/// Pass this cookie to [`xcb_get_font_path_reply`] to retrieve the reply.
///
/// [`xcb_get_font_path_reply`]: Xcb::xcb_get_font_path_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_font_path_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_get_font_path_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `GetFontPath` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_get_font_path_request_t`].
pub const XCB_GET_FONT_PATH: u8 = 52i32 as u8;

/// The `GetFontPath` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_font_path_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}

impl Default for xcb_get_font_path_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `GetFontPath` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `path`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_font_path_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub path_len: u16,
    pub pad1: [u8; 22],
}

impl Default for xcb_get_font_path_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `CreatePixmap` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_create_pixmap_request_t`].
pub const XCB_CREATE_PIXMAP: u8 = 53i32 as u8;

/// The `CreatePixmap` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_create_pixmap_request_t {
    pub major_opcode: u8,
    pub depth: u8,
    pub length: u16,
    pub pid: xcb_pixmap_t,
    pub drawable: xcb_drawable_t,
    pub width: u16,
    pub height: u16,
}

impl Default for xcb_create_pixmap_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `FreePixmap` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_free_pixmap_request_t`].
pub const XCB_FREE_PIXMAP: u8 = 54i32 as u8;

/// The `FreePixmap` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_free_pixmap_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub pixmap: xcb_pixmap_t,
}

impl Default for xcb_free_pixmap_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `GC` enum.
///
/// This enum has the following variants:
///
/// - [`GC::Function`](XCB_GC_FUNCTION)
/// - [`GC::PlaneMask`](XCB_GC_PLANE_MASK)
/// - [`GC::Foreground`](XCB_GC_FOREGROUND)
/// - [`GC::Background`](XCB_GC_BACKGROUND)
/// - [`GC::LineWidth`](XCB_GC_LINE_WIDTH)
/// - [`GC::LineStyle`](XCB_GC_LINE_STYLE)
/// - [`GC::CapStyle`](XCB_GC_CAP_STYLE)
/// - [`GC::JoinStyle`](XCB_GC_JOIN_STYLE)
/// - [`GC::FillStyle`](XCB_GC_FILL_STYLE)
/// - [`GC::FillRule`](XCB_GC_FILL_RULE)
/// - [`GC::Tile`](XCB_GC_TILE)
/// - [`GC::Stipple`](XCB_GC_STIPPLE)
/// - [`GC::TileStippleOriginX`](XCB_GC_TILE_STIPPLE_ORIGIN_X)
/// - [`GC::TileStippleOriginY`](XCB_GC_TILE_STIPPLE_ORIGIN_Y)
/// - [`GC::Font`](XCB_GC_FONT)
/// - [`GC::SubwindowMode`](XCB_GC_SUBWINDOW_MODE)
/// - [`GC::GraphicsExposures`](XCB_GC_GRAPHICS_EXPOSURES)
/// - [`GC::ClipOriginX`](XCB_GC_CLIP_ORIGIN_X)
/// - [`GC::ClipOriginY`](XCB_GC_CLIP_ORIGIN_Y)
/// - [`GC::ClipMask`](XCB_GC_CLIP_MASK)
/// - [`GC::DashOffset`](XCB_GC_DASH_OFFSET)
/// - [`GC::DashList`](XCB_GC_DASH_LIST)
/// - [`GC::ArcMode`](XCB_GC_ARC_MODE)
pub type xcb_gc_t = u32;
/// The `GC::Function` enum variant.
///
/// This is a variant of [`xcb_gc_t`].
pub const XCB_GC_FUNCTION: xcb_gc_t = 1;
/// The `GC::PlaneMask` enum variant.
///
/// This is a variant of [`xcb_gc_t`].
pub const XCB_GC_PLANE_MASK: xcb_gc_t = 2;
/// The `GC::Foreground` enum variant.
///
/// This is a variant of [`xcb_gc_t`].
pub const XCB_GC_FOREGROUND: xcb_gc_t = 4;
/// The `GC::Background` enum variant.
///
/// This is a variant of [`xcb_gc_t`].
pub const XCB_GC_BACKGROUND: xcb_gc_t = 8;
/// The `GC::LineWidth` enum variant.
///
/// This is a variant of [`xcb_gc_t`].
pub const XCB_GC_LINE_WIDTH: xcb_gc_t = 16;
/// The `GC::LineStyle` enum variant.
///
/// This is a variant of [`xcb_gc_t`].
pub const XCB_GC_LINE_STYLE: xcb_gc_t = 32;
/// The `GC::CapStyle` enum variant.
///
/// This is a variant of [`xcb_gc_t`].
pub const XCB_GC_CAP_STYLE: xcb_gc_t = 64;
/// The `GC::JoinStyle` enum variant.
///
/// This is a variant of [`xcb_gc_t`].
pub const XCB_GC_JOIN_STYLE: xcb_gc_t = 128;
/// The `GC::FillStyle` enum variant.
///
/// This is a variant of [`xcb_gc_t`].
pub const XCB_GC_FILL_STYLE: xcb_gc_t = 256;
/// The `GC::FillRule` enum variant.
///
/// This is a variant of [`xcb_gc_t`].
pub const XCB_GC_FILL_RULE: xcb_gc_t = 512;
/// The `GC::Tile` enum variant.
///
/// This is a variant of [`xcb_gc_t`].
pub const XCB_GC_TILE: xcb_gc_t = 1024;
/// The `GC::Stipple` enum variant.
///
/// This is a variant of [`xcb_gc_t`].
pub const XCB_GC_STIPPLE: xcb_gc_t = 2048;
/// The `GC::TileStippleOriginX` enum variant.
///
/// This is a variant of [`xcb_gc_t`].
pub const XCB_GC_TILE_STIPPLE_ORIGIN_X: xcb_gc_t = 4096;
/// The `GC::TileStippleOriginY` enum variant.
///
/// This is a variant of [`xcb_gc_t`].
pub const XCB_GC_TILE_STIPPLE_ORIGIN_Y: xcb_gc_t = 8192;
/// The `GC::Font` enum variant.
///
/// This is a variant of [`xcb_gc_t`].
pub const XCB_GC_FONT: xcb_gc_t = 16384;
/// The `GC::SubwindowMode` enum variant.
///
/// This is a variant of [`xcb_gc_t`].
pub const XCB_GC_SUBWINDOW_MODE: xcb_gc_t = 32768;
/// The `GC::GraphicsExposures` enum variant.
///
/// This is a variant of [`xcb_gc_t`].
pub const XCB_GC_GRAPHICS_EXPOSURES: xcb_gc_t = 65536;
/// The `GC::ClipOriginX` enum variant.
///
/// This is a variant of [`xcb_gc_t`].
pub const XCB_GC_CLIP_ORIGIN_X: xcb_gc_t = 131072;
/// The `GC::ClipOriginY` enum variant.
///
/// This is a variant of [`xcb_gc_t`].
pub const XCB_GC_CLIP_ORIGIN_Y: xcb_gc_t = 262144;
/// The `GC::ClipMask` enum variant.
///
/// This is a variant of [`xcb_gc_t`].
pub const XCB_GC_CLIP_MASK: xcb_gc_t = 524288;
/// The `GC::DashOffset` enum variant.
///
/// This is a variant of [`xcb_gc_t`].
pub const XCB_GC_DASH_OFFSET: xcb_gc_t = 1048576;
/// The `GC::DashList` enum variant.
///
/// This is a variant of [`xcb_gc_t`].
pub const XCB_GC_DASH_LIST: xcb_gc_t = 2097152;
/// The `GC::ArcMode` enum variant.
///
/// This is a variant of [`xcb_gc_t`].
pub const XCB_GC_ARC_MODE: xcb_gc_t = 4194304;

/// The `GX` enum.
///
/// This enum has the following variants:
///
/// - [`GX::clear`](XCB_GX_CLEAR)
/// - [`GX::and`](XCB_GX_AND)
/// - [`GX::andReverse`](XCB_GX_AND_REVERSE)
/// - [`GX::copy`](XCB_GX_COPY)
/// - [`GX::andInverted`](XCB_GX_AND_INVERTED)
/// - [`GX::noop`](XCB_GX_NOOP)
/// - [`GX::xor`](XCB_GX_XOR)
/// - [`GX::or`](XCB_GX_OR)
/// - [`GX::nor`](XCB_GX_NOR)
/// - [`GX::equiv`](XCB_GX_EQUIV)
/// - [`GX::invert`](XCB_GX_INVERT)
/// - [`GX::orReverse`](XCB_GX_OR_REVERSE)
/// - [`GX::copyInverted`](XCB_GX_COPY_INVERTED)
/// - [`GX::orInverted`](XCB_GX_OR_INVERTED)
/// - [`GX::nand`](XCB_GX_NAND)
/// - [`GX::set`](XCB_GX_SET)
pub type xcb_gx_t = u32;
/// The `GX::clear` enum variant.
///
/// This is a variant of [`xcb_gx_t`].
pub const XCB_GX_CLEAR: xcb_gx_t = 0;
/// The `GX::and` enum variant.
///
/// This is a variant of [`xcb_gx_t`].
pub const XCB_GX_AND: xcb_gx_t = 1;
/// The `GX::andReverse` enum variant.
///
/// This is a variant of [`xcb_gx_t`].
pub const XCB_GX_AND_REVERSE: xcb_gx_t = 2;
/// The `GX::copy` enum variant.
///
/// This is a variant of [`xcb_gx_t`].
pub const XCB_GX_COPY: xcb_gx_t = 3;
/// The `GX::andInverted` enum variant.
///
/// This is a variant of [`xcb_gx_t`].
pub const XCB_GX_AND_INVERTED: xcb_gx_t = 4;
/// The `GX::noop` enum variant.
///
/// This is a variant of [`xcb_gx_t`].
pub const XCB_GX_NOOP: xcb_gx_t = 5;
/// The `GX::xor` enum variant.
///
/// This is a variant of [`xcb_gx_t`].
pub const XCB_GX_XOR: xcb_gx_t = 6;
/// The `GX::or` enum variant.
///
/// This is a variant of [`xcb_gx_t`].
pub const XCB_GX_OR: xcb_gx_t = 7;
/// The `GX::nor` enum variant.
///
/// This is a variant of [`xcb_gx_t`].
pub const XCB_GX_NOR: xcb_gx_t = 8;
/// The `GX::equiv` enum variant.
///
/// This is a variant of [`xcb_gx_t`].
pub const XCB_GX_EQUIV: xcb_gx_t = 9;
/// The `GX::invert` enum variant.
///
/// This is a variant of [`xcb_gx_t`].
pub const XCB_GX_INVERT: xcb_gx_t = 10;
/// The `GX::orReverse` enum variant.
///
/// This is a variant of [`xcb_gx_t`].
pub const XCB_GX_OR_REVERSE: xcb_gx_t = 11;
/// The `GX::copyInverted` enum variant.
///
/// This is a variant of [`xcb_gx_t`].
pub const XCB_GX_COPY_INVERTED: xcb_gx_t = 12;
/// The `GX::orInverted` enum variant.
///
/// This is a variant of [`xcb_gx_t`].
pub const XCB_GX_OR_INVERTED: xcb_gx_t = 13;
/// The `GX::nand` enum variant.
///
/// This is a variant of [`xcb_gx_t`].
pub const XCB_GX_NAND: xcb_gx_t = 14;
/// The `GX::set` enum variant.
///
/// This is a variant of [`xcb_gx_t`].
pub const XCB_GX_SET: xcb_gx_t = 15;

/// The `LineStyle` enum.
///
/// This enum has the following variants:
///
/// - [`LineStyle::Solid`](XCB_LINE_STYLE_SOLID)
/// - [`LineStyle::OnOffDash`](XCB_LINE_STYLE_ON_OFF_DASH)
/// - [`LineStyle::DoubleDash`](XCB_LINE_STYLE_DOUBLE_DASH)
pub type xcb_line_style_t = u32;
/// The `LineStyle::Solid` enum variant.
///
/// This is a variant of [`xcb_line_style_t`].
pub const XCB_LINE_STYLE_SOLID: xcb_line_style_t = 0;
/// The `LineStyle::OnOffDash` enum variant.
///
/// This is a variant of [`xcb_line_style_t`].
pub const XCB_LINE_STYLE_ON_OFF_DASH: xcb_line_style_t = 1;
/// The `LineStyle::DoubleDash` enum variant.
///
/// This is a variant of [`xcb_line_style_t`].
pub const XCB_LINE_STYLE_DOUBLE_DASH: xcb_line_style_t = 2;

/// The `CapStyle` enum.
///
/// This enum has the following variants:
///
/// - [`CapStyle::NotLast`](XCB_CAP_STYLE_NOT_LAST)
/// - [`CapStyle::Butt`](XCB_CAP_STYLE_BUTT)
/// - [`CapStyle::Round`](XCB_CAP_STYLE_ROUND)
/// - [`CapStyle::Projecting`](XCB_CAP_STYLE_PROJECTING)
pub type xcb_cap_style_t = u32;
/// The `CapStyle::NotLast` enum variant.
///
/// This is a variant of [`xcb_cap_style_t`].
pub const XCB_CAP_STYLE_NOT_LAST: xcb_cap_style_t = 0;
/// The `CapStyle::Butt` enum variant.
///
/// This is a variant of [`xcb_cap_style_t`].
pub const XCB_CAP_STYLE_BUTT: xcb_cap_style_t = 1;
/// The `CapStyle::Round` enum variant.
///
/// This is a variant of [`xcb_cap_style_t`].
pub const XCB_CAP_STYLE_ROUND: xcb_cap_style_t = 2;
/// The `CapStyle::Projecting` enum variant.
///
/// This is a variant of [`xcb_cap_style_t`].
pub const XCB_CAP_STYLE_PROJECTING: xcb_cap_style_t = 3;

/// The `JoinStyle` enum.
///
/// This enum has the following variants:
///
/// - [`JoinStyle::Miter`](XCB_JOIN_STYLE_MITER)
/// - [`JoinStyle::Round`](XCB_JOIN_STYLE_ROUND)
/// - [`JoinStyle::Bevel`](XCB_JOIN_STYLE_BEVEL)
pub type xcb_join_style_t = u32;
/// The `JoinStyle::Miter` enum variant.
///
/// This is a variant of [`xcb_join_style_t`].
pub const XCB_JOIN_STYLE_MITER: xcb_join_style_t = 0;
/// The `JoinStyle::Round` enum variant.
///
/// This is a variant of [`xcb_join_style_t`].
pub const XCB_JOIN_STYLE_ROUND: xcb_join_style_t = 1;
/// The `JoinStyle::Bevel` enum variant.
///
/// This is a variant of [`xcb_join_style_t`].
pub const XCB_JOIN_STYLE_BEVEL: xcb_join_style_t = 2;

/// The `FillStyle` enum.
///
/// This enum has the following variants:
///
/// - [`FillStyle::Solid`](XCB_FILL_STYLE_SOLID)
/// - [`FillStyle::Tiled`](XCB_FILL_STYLE_TILED)
/// - [`FillStyle::Stippled`](XCB_FILL_STYLE_STIPPLED)
/// - [`FillStyle::OpaqueStippled`](XCB_FILL_STYLE_OPAQUE_STIPPLED)
pub type xcb_fill_style_t = u32;
/// The `FillStyle::Solid` enum variant.
///
/// This is a variant of [`xcb_fill_style_t`].
pub const XCB_FILL_STYLE_SOLID: xcb_fill_style_t = 0;
/// The `FillStyle::Tiled` enum variant.
///
/// This is a variant of [`xcb_fill_style_t`].
pub const XCB_FILL_STYLE_TILED: xcb_fill_style_t = 1;
/// The `FillStyle::Stippled` enum variant.
///
/// This is a variant of [`xcb_fill_style_t`].
pub const XCB_FILL_STYLE_STIPPLED: xcb_fill_style_t = 2;
/// The `FillStyle::OpaqueStippled` enum variant.
///
/// This is a variant of [`xcb_fill_style_t`].
pub const XCB_FILL_STYLE_OPAQUE_STIPPLED: xcb_fill_style_t = 3;

/// The `FillRule` enum.
///
/// This enum has the following variants:
///
/// - [`FillRule::EvenOdd`](XCB_FILL_RULE_EVEN_ODD)
/// - [`FillRule::Winding`](XCB_FILL_RULE_WINDING)
pub type xcb_fill_rule_t = u32;
/// The `FillRule::EvenOdd` enum variant.
///
/// This is a variant of [`xcb_fill_rule_t`].
pub const XCB_FILL_RULE_EVEN_ODD: xcb_fill_rule_t = 0;
/// The `FillRule::Winding` enum variant.
///
/// This is a variant of [`xcb_fill_rule_t`].
pub const XCB_FILL_RULE_WINDING: xcb_fill_rule_t = 1;

/// The `SubwindowMode` enum.
///
/// This enum has the following variants:
///
/// - [`SubwindowMode::ClipByChildren`](XCB_SUBWINDOW_MODE_CLIP_BY_CHILDREN)
/// - [`SubwindowMode::IncludeInferiors`](XCB_SUBWINDOW_MODE_INCLUDE_INFERIORS)
pub type xcb_subwindow_mode_t = u32;
/// The `SubwindowMode::ClipByChildren` enum variant.
///
/// This is a variant of [`xcb_subwindow_mode_t`].
pub const XCB_SUBWINDOW_MODE_CLIP_BY_CHILDREN: xcb_subwindow_mode_t = 0;
/// The `SubwindowMode::IncludeInferiors` enum variant.
///
/// This is a variant of [`xcb_subwindow_mode_t`].
pub const XCB_SUBWINDOW_MODE_INCLUDE_INFERIORS: xcb_subwindow_mode_t = 1;

/// The `ArcMode` enum.
///
/// This enum has the following variants:
///
/// - [`ArcMode::Chord`](XCB_ARC_MODE_CHORD)
/// - [`ArcMode::PieSlice`](XCB_ARC_MODE_PIE_SLICE)
pub type xcb_arc_mode_t = u32;
/// The `ArcMode::Chord` enum variant.
///
/// This is a variant of [`xcb_arc_mode_t`].
pub const XCB_ARC_MODE_CHORD: xcb_arc_mode_t = 0;
/// The `ArcMode::PieSlice` enum variant.
///
/// This is a variant of [`xcb_arc_mode_t`].
pub const XCB_ARC_MODE_PIE_SLICE: xcb_arc_mode_t = 1;

/// The `value_list` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_create_gc_value_list_t {
    pub function: u32,
    pub plane_mask: u32,
    pub foreground: u32,
    pub background: u32,
    pub line_width: u32,
    pub line_style: u32,
    pub cap_style: u32,
    pub join_style: u32,
    pub fill_style: u32,
    pub fill_rule: u32,
    pub tile: xcb_pixmap_t,
    pub stipple: xcb_pixmap_t,
    pub tile_stipple_x_origin: i32,
    pub tile_stipple_y_origin: i32,
    pub font: xcb_font_t,
    pub subwindow_mode: u32,
    pub graphics_exposures: xcb_bool32_t,
    pub clip_x_origin: i32,
    pub clip_y_origin: i32,
    pub clip_mask: xcb_pixmap_t,
    pub dash_offset: u32,
    pub dashes: u32,
    pub arc_mode: u32,
}

impl Default for xcb_create_gc_value_list_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `CreateGC` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_create_gc_request_t`].
pub const XCB_CREATE_GC: u8 = 55i32 as u8;

/// The `CreateGC` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `value_list`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_create_gc_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cid: xcb_gcontext_t,
    pub drawable: xcb_drawable_t,
    pub value_mask: u32,
}

impl Default for xcb_create_gc_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `value_list` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_change_gc_value_list_t {
    pub function: u32,
    pub plane_mask: u32,
    pub foreground: u32,
    pub background: u32,
    pub line_width: u32,
    pub line_style: u32,
    pub cap_style: u32,
    pub join_style: u32,
    pub fill_style: u32,
    pub fill_rule: u32,
    pub tile: xcb_pixmap_t,
    pub stipple: xcb_pixmap_t,
    pub tile_stipple_x_origin: i32,
    pub tile_stipple_y_origin: i32,
    pub font: xcb_font_t,
    pub subwindow_mode: u32,
    pub graphics_exposures: xcb_bool32_t,
    pub clip_x_origin: i32,
    pub clip_y_origin: i32,
    pub clip_mask: xcb_pixmap_t,
    pub dash_offset: u32,
    pub dashes: u32,
    pub arc_mode: u32,
}

impl Default for xcb_change_gc_value_list_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ChangeGC` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_change_gc_request_t`].
pub const XCB_CHANGE_GC: u8 = 56i32 as u8;

/// The `ChangeGC` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `value_list`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_change_gc_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub gc: xcb_gcontext_t,
    pub value_mask: u32,
}

impl Default for xcb_change_gc_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `CopyGC` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_copy_gc_request_t`].
pub const XCB_COPY_GC: u8 = 57i32 as u8;

/// The `CopyGC` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_copy_gc_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub src_gc: xcb_gcontext_t,
    pub dst_gc: xcb_gcontext_t,
    pub value_mask: u32,
}

impl Default for xcb_copy_gc_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SetDashes` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_set_dashes_request_t`].
pub const XCB_SET_DASHES: u8 = 58i32 as u8;

/// The `SetDashes` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `dashes`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_set_dashes_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub gc: xcb_gcontext_t,
    pub dash_offset: u16,
    pub dashes_len: u16,
}

impl Default for xcb_set_dashes_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `ClipOrdering` enum.
///
/// This enum has the following variants:
///
/// - [`ClipOrdering::Unsorted`](XCB_CLIP_ORDERING_UNSORTED)
/// - [`ClipOrdering::YSorted`](XCB_CLIP_ORDERING_Y_SORTED)
/// - [`ClipOrdering::YXSorted`](XCB_CLIP_ORDERING_YX_SORTED)
/// - [`ClipOrdering::YXBanded`](XCB_CLIP_ORDERING_YX_BANDED)
pub type xcb_clip_ordering_t = u32;
/// The `ClipOrdering::Unsorted` enum variant.
///
/// This is a variant of [`xcb_clip_ordering_t`].
pub const XCB_CLIP_ORDERING_UNSORTED: xcb_clip_ordering_t = 0;
/// The `ClipOrdering::YSorted` enum variant.
///
/// This is a variant of [`xcb_clip_ordering_t`].
pub const XCB_CLIP_ORDERING_Y_SORTED: xcb_clip_ordering_t = 1;
/// The `ClipOrdering::YXSorted` enum variant.
///
/// This is a variant of [`xcb_clip_ordering_t`].
pub const XCB_CLIP_ORDERING_YX_SORTED: xcb_clip_ordering_t = 2;
/// The `ClipOrdering::YXBanded` enum variant.
///
/// This is a variant of [`xcb_clip_ordering_t`].
pub const XCB_CLIP_ORDERING_YX_BANDED: xcb_clip_ordering_t = 3;

/// The opcode for `SetClipRectangles` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_set_clip_rectangles_request_t`].
pub const XCB_SET_CLIP_RECTANGLES: u8 = 59i32 as u8;

/// The `SetClipRectangles` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `rectangles`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_set_clip_rectangles_request_t {
    pub major_opcode: u8,
    pub ordering: u8,
    pub length: u16,
    pub gc: xcb_gcontext_t,
    pub clip_x_origin: i16,
    pub clip_y_origin: i16,
}

impl Default for xcb_set_clip_rectangles_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `FreeGC` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_free_gc_request_t`].
pub const XCB_FREE_GC: u8 = 60i32 as u8;

/// The `FreeGC` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_free_gc_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub gc: xcb_gcontext_t,
}

impl Default for xcb_free_gc_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ClearArea` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_clear_area_request_t`].
pub const XCB_CLEAR_AREA: u8 = 61i32 as u8;

/// The `ClearArea` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_clear_area_request_t {
    pub major_opcode: u8,
    pub exposures: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}

impl Default for xcb_clear_area_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `CopyArea` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_copy_area_request_t`].
pub const XCB_COPY_AREA: u8 = 62i32 as u8;

/// The `CopyArea` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_copy_area_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub src_drawable: xcb_drawable_t,
    pub dst_drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub src_x: i16,
    pub src_y: i16,
    pub dst_x: i16,
    pub dst_y: i16,
    pub width: u16,
    pub height: u16,
}

impl Default for xcb_copy_area_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `CopyPlane` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_copy_plane_request_t`].
pub const XCB_COPY_PLANE: u8 = 63i32 as u8;

/// The `CopyPlane` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_copy_plane_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub src_drawable: xcb_drawable_t,
    pub dst_drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub src_x: i16,
    pub src_y: i16,
    pub dst_x: i16,
    pub dst_y: i16,
    pub width: u16,
    pub height: u16,
    pub bit_plane: u32,
}

impl Default for xcb_copy_plane_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `CoordMode` enum.
///
/// This enum has the following variants:
///
/// - [`CoordMode::Origin`](XCB_COORD_MODE_ORIGIN)
/// - [`CoordMode::Previous`](XCB_COORD_MODE_PREVIOUS)
pub type xcb_coord_mode_t = u32;
/// The `CoordMode::Origin` enum variant.
///
/// This is a variant of [`xcb_coord_mode_t`].
pub const XCB_COORD_MODE_ORIGIN: xcb_coord_mode_t = 0;
/// The `CoordMode::Previous` enum variant.
///
/// This is a variant of [`xcb_coord_mode_t`].
pub const XCB_COORD_MODE_PREVIOUS: xcb_coord_mode_t = 1;

/// The opcode for `PolyPoint` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_poly_point_request_t`].
pub const XCB_POLY_POINT: u8 = 64i32 as u8;

/// The `PolyPoint` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `points`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_poly_point_request_t {
    pub major_opcode: u8,
    pub coordinate_mode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
}

impl Default for xcb_poly_point_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `PolyLine` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_poly_line_request_t`].
pub const XCB_POLY_LINE: u8 = 65i32 as u8;

/// The `PolyLine` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `points`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_poly_line_request_t {
    pub major_opcode: u8,
    pub coordinate_mode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
}

impl Default for xcb_poly_line_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `SEGMENT` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_segment_t {
    pub x1: i16,
    pub y1: i16,
    pub x2: i16,
    pub y2: i16,
}

impl Default for xcb_segment_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `SEGMENT` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_segment_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_segment_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_segment_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `PolySegment` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_poly_segment_request_t`].
pub const XCB_POLY_SEGMENT: u8 = 66i32 as u8;

/// The `PolySegment` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `segments`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_poly_segment_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
}

impl Default for xcb_poly_segment_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `PolyRectangle` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_poly_rectangle_request_t`].
pub const XCB_POLY_RECTANGLE: u8 = 67i32 as u8;

/// The `PolyRectangle` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `rectangles`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_poly_rectangle_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
}

impl Default for xcb_poly_rectangle_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `PolyArc` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_poly_arc_request_t`].
pub const XCB_POLY_ARC: u8 = 68i32 as u8;

/// The `PolyArc` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `arcs`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_poly_arc_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
}

impl Default for xcb_poly_arc_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `PolyShape` enum.
///
/// This enum has the following variants:
///
/// - [`PolyShape::Complex`](XCB_POLY_SHAPE_COMPLEX)
/// - [`PolyShape::Nonconvex`](XCB_POLY_SHAPE_NONCONVEX)
/// - [`PolyShape::Convex`](XCB_POLY_SHAPE_CONVEX)
pub type xcb_poly_shape_t = u32;
/// The `PolyShape::Complex` enum variant.
///
/// This is a variant of [`xcb_poly_shape_t`].
pub const XCB_POLY_SHAPE_COMPLEX: xcb_poly_shape_t = 0;
/// The `PolyShape::Nonconvex` enum variant.
///
/// This is a variant of [`xcb_poly_shape_t`].
pub const XCB_POLY_SHAPE_NONCONVEX: xcb_poly_shape_t = 1;
/// The `PolyShape::Convex` enum variant.
///
/// This is a variant of [`xcb_poly_shape_t`].
pub const XCB_POLY_SHAPE_CONVEX: xcb_poly_shape_t = 2;

/// The opcode for `FillPoly` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_fill_poly_request_t`].
pub const XCB_FILL_POLY: u8 = 69i32 as u8;

/// The `FillPoly` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `points`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_fill_poly_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub shape: u8,
    pub coordinate_mode: u8,
    pub pad1: [u8; 2],
}

impl Default for xcb_fill_poly_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `PolyFillRectangle` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_poly_fill_rectangle_request_t`].
pub const XCB_POLY_FILL_RECTANGLE: u8 = 70i32 as u8;

/// The `PolyFillRectangle` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `rectangles`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_poly_fill_rectangle_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
}

impl Default for xcb_poly_fill_rectangle_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `PolyFillArc` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_poly_fill_arc_request_t`].
pub const XCB_POLY_FILL_ARC: u8 = 71i32 as u8;

/// The `PolyFillArc` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `arcs`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_poly_fill_arc_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
}

impl Default for xcb_poly_fill_arc_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `ImageFormat` enum.
///
/// This enum has the following variants:
///
/// - [`ImageFormat::XYBitmap`](XCB_IMAGE_FORMAT_XY_BITMAP)
/// - [`ImageFormat::XYPixmap`](XCB_IMAGE_FORMAT_XY_PIXMAP)
/// - [`ImageFormat::ZPixmap`](XCB_IMAGE_FORMAT_Z_PIXMAP)
pub type xcb_image_format_t = u32;
/// The `ImageFormat::XYBitmap` enum variant.
///
/// This is a variant of [`xcb_image_format_t`].
pub const XCB_IMAGE_FORMAT_XY_BITMAP: xcb_image_format_t = 0;
/// The `ImageFormat::XYPixmap` enum variant.
///
/// This is a variant of [`xcb_image_format_t`].
pub const XCB_IMAGE_FORMAT_XY_PIXMAP: xcb_image_format_t = 1;
/// The `ImageFormat::ZPixmap` enum variant.
///
/// This is a variant of [`xcb_image_format_t`].
pub const XCB_IMAGE_FORMAT_Z_PIXMAP: xcb_image_format_t = 2;

/// The opcode for `PutImage` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_put_image_request_t`].
pub const XCB_PUT_IMAGE: u8 = 72i32 as u8;

/// The `PutImage` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `data`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_put_image_request_t {
    pub major_opcode: u8,
    pub format: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub width: u16,
    pub height: u16,
    pub dst_x: i16,
    pub dst_y: i16,
    pub left_pad: u8,
    pub depth: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_put_image_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `GetImage` request.
///
/// Pass this cookie to [`xcb_get_image_reply`] to retrieve the reply.
///
/// [`xcb_get_image_reply`]: Xcb::xcb_get_image_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_image_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_get_image_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `GetImage` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_get_image_request_t`].
pub const XCB_GET_IMAGE: u8 = 73i32 as u8;

/// The `GetImage` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_image_request_t {
    pub major_opcode: u8,
    pub format: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub plane_mask: u32,
}

impl Default for xcb_get_image_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `GetImage` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `data`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_image_reply_t {
    pub response_type: u8,
    pub depth: u8,
    pub sequence: u16,
    pub length: u32,
    pub visual: xcb_visualid_t,
    pub pad0: [u8; 20],
}

impl Default for xcb_get_image_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `PolyText8` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_poly_text_8_request_t`].
pub const XCB_POLY_TEXT_8: u8 = 74i32 as u8;

/// The `PolyText8` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `items`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_poly_text_8_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub x: i16,
    pub y: i16,
}

impl Default for xcb_poly_text_8_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `PolyText16` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_poly_text_16_request_t`].
pub const XCB_POLY_TEXT_16: u8 = 75i32 as u8;

/// The `PolyText16` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `items`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_poly_text_16_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub x: i16,
    pub y: i16,
}

impl Default for xcb_poly_text_16_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ImageText8` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_image_text_8_request_t`].
pub const XCB_IMAGE_TEXT_8: u8 = 76i32 as u8;

/// The `ImageText8` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `string`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_image_text_8_request_t {
    pub major_opcode: u8,
    pub string_len: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub x: i16,
    pub y: i16,
}

impl Default for xcb_image_text_8_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ImageText16` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_image_text_16_request_t`].
pub const XCB_IMAGE_TEXT_16: u8 = 77i32 as u8;

/// The `ImageText16` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `string`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_image_text_16_request_t {
    pub major_opcode: u8,
    pub string_len: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub gc: xcb_gcontext_t,
    pub x: i16,
    pub y: i16,
}

impl Default for xcb_image_text_16_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `ColormapAlloc` enum.
///
/// This enum has the following variants:
///
/// - [`ColormapAlloc::None`](XCB_COLORMAP_ALLOC_NONE)
/// - [`ColormapAlloc::All`](XCB_COLORMAP_ALLOC_ALL)
pub type xcb_colormap_alloc_t = u32;
/// The `ColormapAlloc::None` enum variant.
///
/// This is a variant of [`xcb_colormap_alloc_t`].
pub const XCB_COLORMAP_ALLOC_NONE: xcb_colormap_alloc_t = 0;
/// The `ColormapAlloc::All` enum variant.
///
/// This is a variant of [`xcb_colormap_alloc_t`].
pub const XCB_COLORMAP_ALLOC_ALL: xcb_colormap_alloc_t = 1;

/// The opcode for `CreateColormap` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_create_colormap_request_t`].
pub const XCB_CREATE_COLORMAP: u8 = 78i32 as u8;

/// The `CreateColormap` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_create_colormap_request_t {
    pub major_opcode: u8,
    pub alloc: u8,
    pub length: u16,
    pub mid: xcb_colormap_t,
    pub window: xcb_window_t,
    pub visual: xcb_visualid_t,
}

impl Default for xcb_create_colormap_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `FreeColormap` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_free_colormap_request_t`].
pub const XCB_FREE_COLORMAP: u8 = 79i32 as u8;

/// The `FreeColormap` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_free_colormap_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
}

impl Default for xcb_free_colormap_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `CopyColormapAndFree` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_copy_colormap_and_free_request_t`].
pub const XCB_COPY_COLORMAP_AND_FREE: u8 = 80i32 as u8;

/// The `CopyColormapAndFree` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_copy_colormap_and_free_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub mid: xcb_colormap_t,
    pub src_cmap: xcb_colormap_t,
}

impl Default for xcb_copy_colormap_and_free_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `InstallColormap` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_install_colormap_request_t`].
pub const XCB_INSTALL_COLORMAP: u8 = 81i32 as u8;

/// The `InstallColormap` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_install_colormap_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
}

impl Default for xcb_install_colormap_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `UninstallColormap` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_uninstall_colormap_request_t`].
pub const XCB_UNINSTALL_COLORMAP: u8 = 82i32 as u8;

/// The `UninstallColormap` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_uninstall_colormap_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
}

impl Default for xcb_uninstall_colormap_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `ListInstalledColormaps` request.
///
/// Pass this cookie to [`xcb_list_installed_colormaps_reply`] to retrieve the reply.
///
/// [`xcb_list_installed_colormaps_reply`]: Xcb::xcb_list_installed_colormaps_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_installed_colormaps_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_list_installed_colormaps_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ListInstalledColormaps` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_list_installed_colormaps_request_t`].
pub const XCB_LIST_INSTALLED_COLORMAPS: u8 = 83i32 as u8;

/// The `ListInstalledColormaps` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_installed_colormaps_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_list_installed_colormaps_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `ListInstalledColormaps` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `cmaps`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_installed_colormaps_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub cmaps_len: u16,
    pub pad1: [u8; 22],
}

impl Default for xcb_list_installed_colormaps_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `AllocColor` request.
///
/// Pass this cookie to [`xcb_alloc_color_reply`] to retrieve the reply.
///
/// [`xcb_alloc_color_reply`]: Xcb::xcb_alloc_color_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_alloc_color_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_alloc_color_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `AllocColor` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_alloc_color_request_t`].
pub const XCB_ALLOC_COLOR: u8 = 84i32 as u8;

/// The `AllocColor` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_alloc_color_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub pad1: [u8; 2],
}

impl Default for xcb_alloc_color_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `AllocColor` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_alloc_color_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub pad1: [u8; 2],
    pub pixel: u32,
}

impl Default for xcb_alloc_color_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `AllocNamedColor` request.
///
/// Pass this cookie to [`xcb_alloc_named_color_reply`] to retrieve the reply.
///
/// [`xcb_alloc_named_color_reply`]: Xcb::xcb_alloc_named_color_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_alloc_named_color_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_alloc_named_color_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `AllocNamedColor` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_alloc_named_color_request_t`].
pub const XCB_ALLOC_NAMED_COLOR: u8 = 85i32 as u8;

/// The `AllocNamedColor` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `name`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_alloc_named_color_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
    pub name_len: u16,
    pub pad1: [u8; 2],
}

impl Default for xcb_alloc_named_color_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `AllocNamedColor` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_alloc_named_color_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pixel: u32,
    pub exact_red: u16,
    pub exact_green: u16,
    pub exact_blue: u16,
    pub visual_red: u16,
    pub visual_green: u16,
    pub visual_blue: u16,
}

impl Default for xcb_alloc_named_color_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `AllocColorCells` request.
///
/// Pass this cookie to [`xcb_alloc_color_cells_reply`] to retrieve the reply.
///
/// [`xcb_alloc_color_cells_reply`]: Xcb::xcb_alloc_color_cells_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_alloc_color_cells_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_alloc_color_cells_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `AllocColorCells` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_alloc_color_cells_request_t`].
pub const XCB_ALLOC_COLOR_CELLS: u8 = 86i32 as u8;

/// The `AllocColorCells` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_alloc_color_cells_request_t {
    pub major_opcode: u8,
    pub contiguous: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
    pub colors: u16,
    pub planes: u16,
}

impl Default for xcb_alloc_color_cells_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `AllocColorCells` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `pixels`
/// - `masks`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_alloc_color_cells_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pixels_len: u16,
    pub masks_len: u16,
    pub pad1: [u8; 20],
}

impl Default for xcb_alloc_color_cells_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `AllocColorPlanes` request.
///
/// Pass this cookie to [`xcb_alloc_color_planes_reply`] to retrieve the reply.
///
/// [`xcb_alloc_color_planes_reply`]: Xcb::xcb_alloc_color_planes_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_alloc_color_planes_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_alloc_color_planes_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `AllocColorPlanes` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_alloc_color_planes_request_t`].
pub const XCB_ALLOC_COLOR_PLANES: u8 = 87i32 as u8;

/// The `AllocColorPlanes` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_alloc_color_planes_request_t {
    pub major_opcode: u8,
    pub contiguous: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
    pub colors: u16,
    pub reds: u16,
    pub greens: u16,
    pub blues: u16,
}

impl Default for xcb_alloc_color_planes_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `AllocColorPlanes` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `pixels`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_alloc_color_planes_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub pixels_len: u16,
    pub pad1: [u8; 2],
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub pad2: [u8; 8],
}

impl Default for xcb_alloc_color_planes_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `FreeColors` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_free_colors_request_t`].
pub const XCB_FREE_COLORS: u8 = 88i32 as u8;

/// The `FreeColors` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `pixels`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_free_colors_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
    pub plane_mask: u32,
}

impl Default for xcb_free_colors_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `ColorFlag` enum.
///
/// This enum has the following variants:
///
/// - [`ColorFlag::Red`](XCB_COLOR_FLAG_RED)
/// - [`ColorFlag::Green`](XCB_COLOR_FLAG_GREEN)
/// - [`ColorFlag::Blue`](XCB_COLOR_FLAG_BLUE)
pub type xcb_color_flag_t = u32;
/// The `ColorFlag::Red` enum variant.
///
/// This is a variant of [`xcb_color_flag_t`].
pub const XCB_COLOR_FLAG_RED: xcb_color_flag_t = 1;
/// The `ColorFlag::Green` enum variant.
///
/// This is a variant of [`xcb_color_flag_t`].
pub const XCB_COLOR_FLAG_GREEN: xcb_color_flag_t = 2;
/// The `ColorFlag::Blue` enum variant.
///
/// This is a variant of [`xcb_color_flag_t`].
pub const XCB_COLOR_FLAG_BLUE: xcb_color_flag_t = 4;

/// The `COLORITEM` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_coloritem_t {
    pub pixel: u32,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub flags: u8,
    pub pad0: u8,
}

impl Default for xcb_coloritem_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `COLORITEM` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_coloritem_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_coloritem_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_coloritem_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `StoreColors` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_store_colors_request_t`].
pub const XCB_STORE_COLORS: u8 = 89i32 as u8;

/// The `StoreColors` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `items`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_store_colors_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
}

impl Default for xcb_store_colors_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `StoreNamedColor` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_store_named_color_request_t`].
pub const XCB_STORE_NAMED_COLOR: u8 = 90i32 as u8;

/// The `StoreNamedColor` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `name`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_store_named_color_request_t {
    pub major_opcode: u8,
    pub flags: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
    pub pixel: u32,
    pub name_len: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_store_named_color_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `RGB` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_rgb_t {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_rgb_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `RGB` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_rgb_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_rgb_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_rgb_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `QueryColors` request.
///
/// Pass this cookie to [`xcb_query_colors_reply`] to retrieve the reply.
///
/// [`xcb_query_colors_reply`]: Xcb::xcb_query_colors_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_colors_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_query_colors_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `QueryColors` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_query_colors_request_t`].
pub const XCB_QUERY_COLORS: u8 = 91i32 as u8;

/// The `QueryColors` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `pixels`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_colors_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
}

impl Default for xcb_query_colors_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `QueryColors` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `colors`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_colors_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub colors_len: u16,
    pub pad1: [u8; 22],
}

impl Default for xcb_query_colors_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `LookupColor` request.
///
/// Pass this cookie to [`xcb_lookup_color_reply`] to retrieve the reply.
///
/// [`xcb_lookup_color_reply`]: Xcb::xcb_lookup_color_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_lookup_color_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_lookup_color_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `LookupColor` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_lookup_color_request_t`].
pub const XCB_LOOKUP_COLOR: u8 = 92i32 as u8;

/// The `LookupColor` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `name`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_lookup_color_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cmap: xcb_colormap_t,
    pub name_len: u16,
    pub pad1: [u8; 2],
}

impl Default for xcb_lookup_color_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `LookupColor` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_lookup_color_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub exact_red: u16,
    pub exact_green: u16,
    pub exact_blue: u16,
    pub visual_red: u16,
    pub visual_green: u16,
    pub visual_blue: u16,
}

impl Default for xcb_lookup_color_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Pixmap` enum.
///
/// This enum has the following variants:
///
/// - [`Pixmap::None`](XCB_PIXMAP_NONE)
pub type xcb_pixmap_enum_t = u32;
/// The `Pixmap::None` enum variant.
///
/// This is a variant of [`xcb_pixmap_enum_t`].
pub const XCB_PIXMAP_NONE: xcb_pixmap_enum_t = 0;

/// The opcode for `CreateCursor` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_create_cursor_request_t`].
pub const XCB_CREATE_CURSOR: u8 = 93i32 as u8;

/// The `CreateCursor` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_create_cursor_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cid: xcb_cursor_t,
    pub source: xcb_pixmap_t,
    pub mask: xcb_pixmap_t,
    pub fore_red: u16,
    pub fore_green: u16,
    pub fore_blue: u16,
    pub back_red: u16,
    pub back_green: u16,
    pub back_blue: u16,
    pub x: u16,
    pub y: u16,
}

impl Default for xcb_create_cursor_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Font` enum.
///
/// This enum has the following variants:
///
/// - [`Font::None`](XCB_FONT_NONE)
pub type xcb_font_enum_t = u32;
/// The `Font::None` enum variant.
///
/// This is a variant of [`xcb_font_enum_t`].
pub const XCB_FONT_NONE: xcb_font_enum_t = 0;

/// The opcode for `CreateGlyphCursor` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_create_glyph_cursor_request_t`].
pub const XCB_CREATE_GLYPH_CURSOR: u8 = 94i32 as u8;

/// The `CreateGlyphCursor` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_create_glyph_cursor_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cid: xcb_cursor_t,
    pub source_font: xcb_font_t,
    pub mask_font: xcb_font_t,
    pub source_char: u16,
    pub mask_char: u16,
    pub fore_red: u16,
    pub fore_green: u16,
    pub fore_blue: u16,
    pub back_red: u16,
    pub back_green: u16,
    pub back_blue: u16,
}

impl Default for xcb_create_glyph_cursor_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `FreeCursor` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_free_cursor_request_t`].
pub const XCB_FREE_CURSOR: u8 = 95i32 as u8;

/// The `FreeCursor` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_free_cursor_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cursor: xcb_cursor_t,
}

impl Default for xcb_free_cursor_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RecolorCursor` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_recolor_cursor_request_t`].
pub const XCB_RECOLOR_CURSOR: u8 = 96i32 as u8;

/// The `RecolorCursor` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_recolor_cursor_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub cursor: xcb_cursor_t,
    pub fore_red: u16,
    pub fore_green: u16,
    pub fore_blue: u16,
    pub back_red: u16,
    pub back_green: u16,
    pub back_blue: u16,
}

impl Default for xcb_recolor_cursor_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `QueryShapeOf` enum.
///
/// This enum has the following variants:
///
/// - [`QueryShapeOf::LargestCursor`](XCB_QUERY_SHAPE_OF_LARGEST_CURSOR)
/// - [`QueryShapeOf::FastestTile`](XCB_QUERY_SHAPE_OF_FASTEST_TILE)
/// - [`QueryShapeOf::FastestStipple`](XCB_QUERY_SHAPE_OF_FASTEST_STIPPLE)
pub type xcb_query_shape_of_t = u32;
/// The `QueryShapeOf::LargestCursor` enum variant.
///
/// This is a variant of [`xcb_query_shape_of_t`].
pub const XCB_QUERY_SHAPE_OF_LARGEST_CURSOR: xcb_query_shape_of_t = 0;
/// The `QueryShapeOf::FastestTile` enum variant.
///
/// This is a variant of [`xcb_query_shape_of_t`].
pub const XCB_QUERY_SHAPE_OF_FASTEST_TILE: xcb_query_shape_of_t = 1;
/// The `QueryShapeOf::FastestStipple` enum variant.
///
/// This is a variant of [`xcb_query_shape_of_t`].
pub const XCB_QUERY_SHAPE_OF_FASTEST_STIPPLE: xcb_query_shape_of_t = 2;

/// The cookie for the reply to a `QueryBestSize` request.
///
/// Pass this cookie to [`xcb_query_best_size_reply`] to retrieve the reply.
///
/// [`xcb_query_best_size_reply`]: Xcb::xcb_query_best_size_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_best_size_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_query_best_size_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `QueryBestSize` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_query_best_size_request_t`].
pub const XCB_QUERY_BEST_SIZE: u8 = 97i32 as u8;

/// The `QueryBestSize` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_best_size_request_t {
    pub major_opcode: u8,
    pub class: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub width: u16,
    pub height: u16,
}

impl Default for xcb_query_best_size_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `QueryBestSize` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_best_size_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: u16,
    pub height: u16,
}

impl Default for xcb_query_best_size_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `QueryExtension` request.
///
/// Pass this cookie to [`xcb_query_extension_reply`] to retrieve the reply.
///
/// [`xcb_query_extension_reply`]: Xcb::xcb_query_extension_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_extension_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_query_extension_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `QueryExtension` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_query_extension_request_t`].
pub const XCB_QUERY_EXTENSION: u8 = 98i32 as u8;

/// The `QueryExtension` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `name`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_extension_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub name_len: u16,
    pub pad1: [u8; 2],
}

impl Default for xcb_query_extension_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `QueryExtension` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_query_extension_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub present: u8,
    pub major_opcode: u8,
    pub first_event: u8,
    pub first_error: u8,
}

impl Default for xcb_query_extension_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `ListExtensions` request.
///
/// Pass this cookie to [`xcb_list_extensions_reply`] to retrieve the reply.
///
/// [`xcb_list_extensions_reply`]: Xcb::xcb_list_extensions_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_extensions_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_list_extensions_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ListExtensions` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_list_extensions_request_t`].
pub const XCB_LIST_EXTENSIONS: u8 = 99i32 as u8;

/// The `ListExtensions` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_extensions_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}

impl Default for xcb_list_extensions_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `ListExtensions` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `names`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_extensions_reply_t {
    pub response_type: u8,
    pub names_len: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad0: [u8; 24],
}

impl Default for xcb_list_extensions_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ChangeKeyboardMapping` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_change_keyboard_mapping_request_t`].
pub const XCB_CHANGE_KEYBOARD_MAPPING: u8 = 100i32 as u8;

/// The `ChangeKeyboardMapping` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `keysyms`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_change_keyboard_mapping_request_t {
    pub major_opcode: u8,
    pub keycode_count: u8,
    pub length: u16,
    pub first_keycode: xcb_keycode_t,
    pub keysyms_per_keycode: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_change_keyboard_mapping_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `GetKeyboardMapping` request.
///
/// Pass this cookie to [`xcb_get_keyboard_mapping_reply`] to retrieve the reply.
///
/// [`xcb_get_keyboard_mapping_reply`]: Xcb::xcb_get_keyboard_mapping_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_keyboard_mapping_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_get_keyboard_mapping_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `GetKeyboardMapping` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_get_keyboard_mapping_request_t`].
pub const XCB_GET_KEYBOARD_MAPPING: u8 = 101i32 as u8;

/// The `GetKeyboardMapping` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_keyboard_mapping_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub first_keycode: xcb_keycode_t,
    pub count: u8,
}

impl Default for xcb_get_keyboard_mapping_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `GetKeyboardMapping` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `keysyms`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_keyboard_mapping_reply_t {
    pub response_type: u8,
    pub keysyms_per_keycode: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad0: [u8; 24],
}

impl Default for xcb_get_keyboard_mapping_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `KB` enum.
///
/// This enum has the following variants:
///
/// - [`KB::KeyClickPercent`](XCB_KB_KEY_CLICK_PERCENT)
/// - [`KB::BellPercent`](XCB_KB_BELL_PERCENT)
/// - [`KB::BellPitch`](XCB_KB_BELL_PITCH)
/// - [`KB::BellDuration`](XCB_KB_BELL_DURATION)
/// - [`KB::Led`](XCB_KB_LED)
/// - [`KB::LedMode`](XCB_KB_LED_MODE)
/// - [`KB::Key`](XCB_KB_KEY)
/// - [`KB::AutoRepeatMode`](XCB_KB_AUTO_REPEAT_MODE)
pub type xcb_kb_t = u32;
/// The `KB::KeyClickPercent` enum variant.
///
/// This is a variant of [`xcb_kb_t`].
pub const XCB_KB_KEY_CLICK_PERCENT: xcb_kb_t = 1;
/// The `KB::BellPercent` enum variant.
///
/// This is a variant of [`xcb_kb_t`].
pub const XCB_KB_BELL_PERCENT: xcb_kb_t = 2;
/// The `KB::BellPitch` enum variant.
///
/// This is a variant of [`xcb_kb_t`].
pub const XCB_KB_BELL_PITCH: xcb_kb_t = 4;
/// The `KB::BellDuration` enum variant.
///
/// This is a variant of [`xcb_kb_t`].
pub const XCB_KB_BELL_DURATION: xcb_kb_t = 8;
/// The `KB::Led` enum variant.
///
/// This is a variant of [`xcb_kb_t`].
pub const XCB_KB_LED: xcb_kb_t = 16;
/// The `KB::LedMode` enum variant.
///
/// This is a variant of [`xcb_kb_t`].
pub const XCB_KB_LED_MODE: xcb_kb_t = 32;
/// The `KB::Key` enum variant.
///
/// This is a variant of [`xcb_kb_t`].
pub const XCB_KB_KEY: xcb_kb_t = 64;
/// The `KB::AutoRepeatMode` enum variant.
///
/// This is a variant of [`xcb_kb_t`].
pub const XCB_KB_AUTO_REPEAT_MODE: xcb_kb_t = 128;

/// The `LedMode` enum.
///
/// This enum has the following variants:
///
/// - [`LedMode::Off`](XCB_LED_MODE_OFF)
/// - [`LedMode::On`](XCB_LED_MODE_ON)
pub type xcb_led_mode_t = u32;
/// The `LedMode::Off` enum variant.
///
/// This is a variant of [`xcb_led_mode_t`].
pub const XCB_LED_MODE_OFF: xcb_led_mode_t = 0;
/// The `LedMode::On` enum variant.
///
/// This is a variant of [`xcb_led_mode_t`].
pub const XCB_LED_MODE_ON: xcb_led_mode_t = 1;

/// The `AutoRepeatMode` enum.
///
/// This enum has the following variants:
///
/// - [`AutoRepeatMode::Off`](XCB_AUTO_REPEAT_MODE_OFF)
/// - [`AutoRepeatMode::On`](XCB_AUTO_REPEAT_MODE_ON)
/// - [`AutoRepeatMode::Default`](XCB_AUTO_REPEAT_MODE_DEFAULT)
pub type xcb_auto_repeat_mode_t = u32;
/// The `AutoRepeatMode::Off` enum variant.
///
/// This is a variant of [`xcb_auto_repeat_mode_t`].
pub const XCB_AUTO_REPEAT_MODE_OFF: xcb_auto_repeat_mode_t = 0;
/// The `AutoRepeatMode::On` enum variant.
///
/// This is a variant of [`xcb_auto_repeat_mode_t`].
pub const XCB_AUTO_REPEAT_MODE_ON: xcb_auto_repeat_mode_t = 1;
/// The `AutoRepeatMode::Default` enum variant.
///
/// This is a variant of [`xcb_auto_repeat_mode_t`].
pub const XCB_AUTO_REPEAT_MODE_DEFAULT: xcb_auto_repeat_mode_t = 2;

/// The `value_list` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_change_keyboard_control_value_list_t {
    pub key_click_percent: i32,
    pub bell_percent: i32,
    pub bell_pitch: i32,
    pub bell_duration: i32,
    pub led: u32,
    pub led_mode: u32,
    pub key: xcb_keycode32_t,
    pub auto_repeat_mode: u32,
}

impl Default for xcb_change_keyboard_control_value_list_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ChangeKeyboardControl` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_change_keyboard_control_request_t`].
pub const XCB_CHANGE_KEYBOARD_CONTROL: u8 = 102i32 as u8;

/// The `ChangeKeyboardControl` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `value_list`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_change_keyboard_control_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub value_mask: u32,
}

impl Default for xcb_change_keyboard_control_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `GetKeyboardControl` request.
///
/// Pass this cookie to [`xcb_get_keyboard_control_reply`] to retrieve the reply.
///
/// [`xcb_get_keyboard_control_reply`]: Xcb::xcb_get_keyboard_control_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_keyboard_control_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_get_keyboard_control_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `GetKeyboardControl` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_get_keyboard_control_request_t`].
pub const XCB_GET_KEYBOARD_CONTROL: u8 = 103i32 as u8;

/// The `GetKeyboardControl` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_keyboard_control_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}

impl Default for xcb_get_keyboard_control_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `GetKeyboardControl` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_keyboard_control_reply_t {
    pub response_type: u8,
    pub global_auto_repeat: u8,
    pub sequence: u16,
    pub length: u32,
    pub led_mask: u32,
    pub key_click_percent: u8,
    pub bell_percent: u8,
    pub bell_pitch: u16,
    pub bell_duration: u16,
    pub pad0: [u8; 2],
    pub auto_repeats: [u8; 32],
}

impl Default for xcb_get_keyboard_control_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Bell` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_bell_request_t`].
pub const XCB_BELL: u8 = 104i32 as u8;

/// The `Bell` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_bell_request_t {
    pub major_opcode: u8,
    pub percent: i8,
    pub length: u16,
}

impl Default for xcb_bell_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ChangePointerControl` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_change_pointer_control_request_t`].
pub const XCB_CHANGE_POINTER_CONTROL: u8 = 105i32 as u8;

/// The `ChangePointerControl` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_change_pointer_control_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub acceleration_numerator: i16,
    pub acceleration_denominator: i16,
    pub threshold: i16,
    pub do_acceleration: u8,
    pub do_threshold: u8,
}

impl Default for xcb_change_pointer_control_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `GetPointerControl` request.
///
/// Pass this cookie to [`xcb_get_pointer_control_reply`] to retrieve the reply.
///
/// [`xcb_get_pointer_control_reply`]: Xcb::xcb_get_pointer_control_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_pointer_control_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_get_pointer_control_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `GetPointerControl` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_get_pointer_control_request_t`].
pub const XCB_GET_POINTER_CONTROL: u8 = 106i32 as u8;

/// The `GetPointerControl` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_pointer_control_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}

impl Default for xcb_get_pointer_control_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `GetPointerControl` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_pointer_control_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub acceleration_numerator: u16,
    pub acceleration_denominator: u16,
    pub threshold: u16,
    pub pad1: [u8; 18],
}

impl Default for xcb_get_pointer_control_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Blanking` enum.
///
/// This enum has the following variants:
///
/// - [`Blanking::NotPreferred`](XCB_BLANKING_NOT_PREFERRED)
/// - [`Blanking::Preferred`](XCB_BLANKING_PREFERRED)
/// - [`Blanking::Default`](XCB_BLANKING_DEFAULT)
pub type xcb_blanking_t = u32;
/// The `Blanking::NotPreferred` enum variant.
///
/// This is a variant of [`xcb_blanking_t`].
pub const XCB_BLANKING_NOT_PREFERRED: xcb_blanking_t = 0;
/// The `Blanking::Preferred` enum variant.
///
/// This is a variant of [`xcb_blanking_t`].
pub const XCB_BLANKING_PREFERRED: xcb_blanking_t = 1;
/// The `Blanking::Default` enum variant.
///
/// This is a variant of [`xcb_blanking_t`].
pub const XCB_BLANKING_DEFAULT: xcb_blanking_t = 2;

/// The `Exposures` enum.
///
/// This enum has the following variants:
///
/// - [`Exposures::NotAllowed`](XCB_EXPOSURES_NOT_ALLOWED)
/// - [`Exposures::Allowed`](XCB_EXPOSURES_ALLOWED)
/// - [`Exposures::Default`](XCB_EXPOSURES_DEFAULT)
pub type xcb_exposures_t = u32;
/// The `Exposures::NotAllowed` enum variant.
///
/// This is a variant of [`xcb_exposures_t`].
pub const XCB_EXPOSURES_NOT_ALLOWED: xcb_exposures_t = 0;
/// The `Exposures::Allowed` enum variant.
///
/// This is a variant of [`xcb_exposures_t`].
pub const XCB_EXPOSURES_ALLOWED: xcb_exposures_t = 1;
/// The `Exposures::Default` enum variant.
///
/// This is a variant of [`xcb_exposures_t`].
pub const XCB_EXPOSURES_DEFAULT: xcb_exposures_t = 2;

/// The opcode for `SetScreenSaver` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_set_screen_saver_request_t`].
pub const XCB_SET_SCREEN_SAVER: u8 = 107i32 as u8;

/// The `SetScreenSaver` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_set_screen_saver_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub timeout: i16,
    pub interval: i16,
    pub prefer_blanking: u8,
    pub allow_exposures: u8,
}

impl Default for xcb_set_screen_saver_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `GetScreenSaver` request.
///
/// Pass this cookie to [`xcb_get_screen_saver_reply`] to retrieve the reply.
///
/// [`xcb_get_screen_saver_reply`]: Xcb::xcb_get_screen_saver_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_screen_saver_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_get_screen_saver_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `GetScreenSaver` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_get_screen_saver_request_t`].
pub const XCB_GET_SCREEN_SAVER: u8 = 108i32 as u8;

/// The `GetScreenSaver` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_screen_saver_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}

impl Default for xcb_get_screen_saver_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `GetScreenSaver` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_screen_saver_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub timeout: u16,
    pub interval: u16,
    pub prefer_blanking: u8,
    pub allow_exposures: u8,
    pub pad1: [u8; 18],
}

impl Default for xcb_get_screen_saver_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `HostMode` enum.
///
/// This enum has the following variants:
///
/// - [`HostMode::Insert`](XCB_HOST_MODE_INSERT)
/// - [`HostMode::Delete`](XCB_HOST_MODE_DELETE)
pub type xcb_host_mode_t = u32;
/// The `HostMode::Insert` enum variant.
///
/// This is a variant of [`xcb_host_mode_t`].
pub const XCB_HOST_MODE_INSERT: xcb_host_mode_t = 0;
/// The `HostMode::Delete` enum variant.
///
/// This is a variant of [`xcb_host_mode_t`].
pub const XCB_HOST_MODE_DELETE: xcb_host_mode_t = 1;

/// The `Family` enum.
///
/// This enum has the following variants:
///
/// - [`Family::Internet`](XCB_FAMILY_INTERNET)
/// - [`Family::DECnet`](XCB_FAMILY_DECNET)
/// - [`Family::Chaos`](XCB_FAMILY_CHAOS)
/// - [`Family::ServerInterpreted`](XCB_FAMILY_SERVER_INTERPRETED)
/// - [`Family::Internet6`](XCB_FAMILY_INTERNET_6)
pub type xcb_family_t = u32;
/// The `Family::Internet` enum variant.
///
/// This is a variant of [`xcb_family_t`].
pub const XCB_FAMILY_INTERNET: xcb_family_t = 0;
/// The `Family::DECnet` enum variant.
///
/// This is a variant of [`xcb_family_t`].
pub const XCB_FAMILY_DECNET: xcb_family_t = 1;
/// The `Family::Chaos` enum variant.
///
/// This is a variant of [`xcb_family_t`].
pub const XCB_FAMILY_CHAOS: xcb_family_t = 2;
/// The `Family::ServerInterpreted` enum variant.
///
/// This is a variant of [`xcb_family_t`].
pub const XCB_FAMILY_SERVER_INTERPRETED: xcb_family_t = 5;
/// The `Family::Internet6` enum variant.
///
/// This is a variant of [`xcb_family_t`].
pub const XCB_FAMILY_INTERNET_6: xcb_family_t = 6;

/// The opcode for `ChangeHosts` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_change_hosts_request_t`].
pub const XCB_CHANGE_HOSTS: u8 = 109i32 as u8;

/// The `ChangeHosts` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `address`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_change_hosts_request_t {
    pub major_opcode: u8,
    pub mode: u8,
    pub length: u16,
    pub family: u8,
    pub pad0: u8,
    pub address_len: u16,
}

impl Default for xcb_change_hosts_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `HOST` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `address`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_host_t {
    pub family: u8,
    pub pad0: u8,
    pub address_len: u16,
}

impl Default for xcb_host_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `HOST` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_host_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_host_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_host_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `ListHosts` request.
///
/// Pass this cookie to [`xcb_list_hosts_reply`] to retrieve the reply.
///
/// [`xcb_list_hosts_reply`]: Xcb::xcb_list_hosts_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_hosts_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_list_hosts_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `ListHosts` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_list_hosts_request_t`].
pub const XCB_LIST_HOSTS: u8 = 110i32 as u8;

/// The `ListHosts` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_hosts_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}

impl Default for xcb_list_hosts_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `ListHosts` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `hosts`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_list_hosts_reply_t {
    pub response_type: u8,
    pub mode: u8,
    pub sequence: u16,
    pub length: u32,
    pub hosts_len: u16,
    pub pad0: [u8; 22],
}

impl Default for xcb_list_hosts_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `AccessControl` enum.
///
/// This enum has the following variants:
///
/// - [`AccessControl::Disable`](XCB_ACCESS_CONTROL_DISABLE)
/// - [`AccessControl::Enable`](XCB_ACCESS_CONTROL_ENABLE)
pub type xcb_access_control_t = u32;
/// The `AccessControl::Disable` enum variant.
///
/// This is a variant of [`xcb_access_control_t`].
pub const XCB_ACCESS_CONTROL_DISABLE: xcb_access_control_t = 0;
/// The `AccessControl::Enable` enum variant.
///
/// This is a variant of [`xcb_access_control_t`].
pub const XCB_ACCESS_CONTROL_ENABLE: xcb_access_control_t = 1;

/// The opcode for `SetAccessControl` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_set_access_control_request_t`].
pub const XCB_SET_ACCESS_CONTROL: u8 = 111i32 as u8;

/// The `SetAccessControl` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_set_access_control_request_t {
    pub major_opcode: u8,
    pub mode: u8,
    pub length: u16,
}

impl Default for xcb_set_access_control_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `CloseDown` enum.
///
/// This enum has the following variants:
///
/// - [`CloseDown::DestroyAll`](XCB_CLOSE_DOWN_DESTROY_ALL)
/// - [`CloseDown::RetainPermanent`](XCB_CLOSE_DOWN_RETAIN_PERMANENT)
/// - [`CloseDown::RetainTemporary`](XCB_CLOSE_DOWN_RETAIN_TEMPORARY)
pub type xcb_close_down_t = u32;
/// The `CloseDown::DestroyAll` enum variant.
///
/// This is a variant of [`xcb_close_down_t`].
pub const XCB_CLOSE_DOWN_DESTROY_ALL: xcb_close_down_t = 0;
/// The `CloseDown::RetainPermanent` enum variant.
///
/// This is a variant of [`xcb_close_down_t`].
pub const XCB_CLOSE_DOWN_RETAIN_PERMANENT: xcb_close_down_t = 1;
/// The `CloseDown::RetainTemporary` enum variant.
///
/// This is a variant of [`xcb_close_down_t`].
pub const XCB_CLOSE_DOWN_RETAIN_TEMPORARY: xcb_close_down_t = 2;

/// The opcode for `SetCloseDownMode` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_set_close_down_mode_request_t`].
pub const XCB_SET_CLOSE_DOWN_MODE: u8 = 112i32 as u8;

/// The `SetCloseDownMode` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_set_close_down_mode_request_t {
    pub major_opcode: u8,
    pub mode: u8,
    pub length: u16,
}

impl Default for xcb_set_close_down_mode_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Kill` enum.
///
/// This enum has the following variants:
///
/// - [`Kill::AllTemporary`](XCB_KILL_ALL_TEMPORARY)
pub type xcb_kill_t = u32;
/// The `Kill::AllTemporary` enum variant.
///
/// This is a variant of [`xcb_kill_t`].
pub const XCB_KILL_ALL_TEMPORARY: xcb_kill_t = 0;

/// The opcode for `KillClient` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_kill_client_request_t`].
pub const XCB_KILL_CLIENT: u8 = 113i32 as u8;

/// The `KillClient` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_kill_client_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub resource: u32,
}

impl Default for xcb_kill_client_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `RotateProperties` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_rotate_properties_request_t`].
pub const XCB_ROTATE_PROPERTIES: u8 = 114i32 as u8;

/// The `RotateProperties` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `atoms`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_rotate_properties_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub atoms_len: u16,
    pub delta: i16,
}

impl Default for xcb_rotate_properties_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `ScreenSaver` enum.
///
/// This enum has the following variants:
///
/// - [`ScreenSaver::Reset`](XCB_SCREEN_SAVER_RESET)
/// - [`ScreenSaver::Active`](XCB_SCREEN_SAVER_ACTIVE)
pub type xcb_screen_saver_t = u32;
/// The `ScreenSaver::Reset` enum variant.
///
/// This is a variant of [`xcb_screen_saver_t`].
pub const XCB_SCREEN_SAVER_RESET: xcb_screen_saver_t = 0;
/// The `ScreenSaver::Active` enum variant.
///
/// This is a variant of [`xcb_screen_saver_t`].
pub const XCB_SCREEN_SAVER_ACTIVE: xcb_screen_saver_t = 1;

/// The opcode for `ForceScreenSaver` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_force_screen_saver_request_t`].
pub const XCB_FORCE_SCREEN_SAVER: u8 = 115i32 as u8;

/// The `ForceScreenSaver` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_force_screen_saver_request_t {
    pub major_opcode: u8,
    pub mode: u8,
    pub length: u16,
}

impl Default for xcb_force_screen_saver_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `MappingStatus` enum.
///
/// This enum has the following variants:
///
/// - [`MappingStatus::Success`](XCB_MAPPING_STATUS_SUCCESS)
/// - [`MappingStatus::Busy`](XCB_MAPPING_STATUS_BUSY)
/// - [`MappingStatus::Failure`](XCB_MAPPING_STATUS_FAILURE)
pub type xcb_mapping_status_t = u32;
/// The `MappingStatus::Success` enum variant.
///
/// This is a variant of [`xcb_mapping_status_t`].
pub const XCB_MAPPING_STATUS_SUCCESS: xcb_mapping_status_t = 0;
/// The `MappingStatus::Busy` enum variant.
///
/// This is a variant of [`xcb_mapping_status_t`].
pub const XCB_MAPPING_STATUS_BUSY: xcb_mapping_status_t = 1;
/// The `MappingStatus::Failure` enum variant.
///
/// This is a variant of [`xcb_mapping_status_t`].
pub const XCB_MAPPING_STATUS_FAILURE: xcb_mapping_status_t = 2;

/// The cookie for the reply to a `SetPointerMapping` request.
///
/// Pass this cookie to [`xcb_set_pointer_mapping_reply`] to retrieve the reply.
///
/// [`xcb_set_pointer_mapping_reply`]: Xcb::xcb_set_pointer_mapping_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_set_pointer_mapping_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_set_pointer_mapping_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SetPointerMapping` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_set_pointer_mapping_request_t`].
pub const XCB_SET_POINTER_MAPPING: u8 = 116i32 as u8;

/// The `SetPointerMapping` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `map`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_set_pointer_mapping_request_t {
    pub major_opcode: u8,
    pub map_len: u8,
    pub length: u16,
}

impl Default for xcb_set_pointer_mapping_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `SetPointerMapping` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_set_pointer_mapping_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
}

impl Default for xcb_set_pointer_mapping_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `GetPointerMapping` request.
///
/// Pass this cookie to [`xcb_get_pointer_mapping_reply`] to retrieve the reply.
///
/// [`xcb_get_pointer_mapping_reply`]: Xcb::xcb_get_pointer_mapping_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_pointer_mapping_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_get_pointer_mapping_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `GetPointerMapping` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_get_pointer_mapping_request_t`].
pub const XCB_GET_POINTER_MAPPING: u8 = 117i32 as u8;

/// The `GetPointerMapping` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_pointer_mapping_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}

impl Default for xcb_get_pointer_mapping_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `GetPointerMapping` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `map`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_pointer_mapping_reply_t {
    pub response_type: u8,
    pub map_len: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad0: [u8; 24],
}

impl Default for xcb_get_pointer_mapping_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `MapIndex` enum.
///
/// This enum has the following variants:
///
/// - [`MapIndex::Shift`](XCB_MAP_INDEX_SHIFT)
/// - [`MapIndex::Lock`](XCB_MAP_INDEX_LOCK)
/// - [`MapIndex::Control`](XCB_MAP_INDEX_CONTROL)
/// - [`MapIndex::1`](XCB_MAP_INDEX_1)
/// - [`MapIndex::2`](XCB_MAP_INDEX_2)
/// - [`MapIndex::3`](XCB_MAP_INDEX_3)
/// - [`MapIndex::4`](XCB_MAP_INDEX_4)
/// - [`MapIndex::5`](XCB_MAP_INDEX_5)
pub type xcb_map_index_t = u32;
/// The `MapIndex::Shift` enum variant.
///
/// This is a variant of [`xcb_map_index_t`].
pub const XCB_MAP_INDEX_SHIFT: xcb_map_index_t = 0;
/// The `MapIndex::Lock` enum variant.
///
/// This is a variant of [`xcb_map_index_t`].
pub const XCB_MAP_INDEX_LOCK: xcb_map_index_t = 1;
/// The `MapIndex::Control` enum variant.
///
/// This is a variant of [`xcb_map_index_t`].
pub const XCB_MAP_INDEX_CONTROL: xcb_map_index_t = 2;
/// The `MapIndex::1` enum variant.
///
/// This is a variant of [`xcb_map_index_t`].
pub const XCB_MAP_INDEX_1: xcb_map_index_t = 3;
/// The `MapIndex::2` enum variant.
///
/// This is a variant of [`xcb_map_index_t`].
pub const XCB_MAP_INDEX_2: xcb_map_index_t = 4;
/// The `MapIndex::3` enum variant.
///
/// This is a variant of [`xcb_map_index_t`].
pub const XCB_MAP_INDEX_3: xcb_map_index_t = 5;
/// The `MapIndex::4` enum variant.
///
/// This is a variant of [`xcb_map_index_t`].
pub const XCB_MAP_INDEX_4: xcb_map_index_t = 6;
/// The `MapIndex::5` enum variant.
///
/// This is a variant of [`xcb_map_index_t`].
pub const XCB_MAP_INDEX_5: xcb_map_index_t = 7;

/// The cookie for the reply to a `SetModifierMapping` request.
///
/// Pass this cookie to [`xcb_set_modifier_mapping_reply`] to retrieve the reply.
///
/// [`xcb_set_modifier_mapping_reply`]: Xcb::xcb_set_modifier_mapping_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_set_modifier_mapping_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_set_modifier_mapping_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `SetModifierMapping` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_set_modifier_mapping_request_t`].
pub const XCB_SET_MODIFIER_MAPPING: u8 = 118i32 as u8;

/// The `SetModifierMapping` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `keycodes`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_set_modifier_mapping_request_t {
    pub major_opcode: u8,
    pub keycodes_per_modifier: u8,
    pub length: u16,
}

impl Default for xcb_set_modifier_mapping_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `SetModifierMapping` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_set_modifier_mapping_reply_t {
    pub response_type: u8,
    pub status: u8,
    pub sequence: u16,
    pub length: u32,
}

impl Default for xcb_set_modifier_mapping_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `GetModifierMapping` request.
///
/// Pass this cookie to [`xcb_get_modifier_mapping_reply`] to retrieve the reply.
///
/// [`xcb_get_modifier_mapping_reply`]: Xcb::xcb_get_modifier_mapping_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_modifier_mapping_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_get_modifier_mapping_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `GetModifierMapping` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_get_modifier_mapping_request_t`].
pub const XCB_GET_MODIFIER_MAPPING: u8 = 119i32 as u8;

/// The `GetModifierMapping` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_modifier_mapping_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}

impl Default for xcb_get_modifier_mapping_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `GetModifierMapping` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `keycodes`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_get_modifier_mapping_reply_t {
    pub response_type: u8,
    pub keycodes_per_modifier: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad0: [u8; 24],
}

impl Default for xcb_get_modifier_mapping_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `NoOperation` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`ptr::null_mut()`](std::ptr::null_mut()), then the type of the request is
/// [`xcb_no_operation_request_t`].
pub const XCB_NO_OPERATION: u8 = 127i32 as u8;

/// The `NoOperation` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_no_operation_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
}

impl Default for xcb_no_operation_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub(crate) struct XcbXproto {
    xcb_char2b_next: LazySymbol<unsafe fn(i: *mut xcb_char2b_iterator_t)>,
    xcb_char2b_end: LazySymbol<unsafe fn(i: xcb_char2b_iterator_t) -> xcb_generic_iterator_t>,
    xcb_window_next: LazySymbol<unsafe fn(i: *mut xcb_window_iterator_t)>,
    xcb_window_end: LazySymbol<unsafe fn(i: xcb_window_iterator_t) -> xcb_generic_iterator_t>,
    xcb_pixmap_next: LazySymbol<unsafe fn(i: *mut xcb_pixmap_iterator_t)>,
    xcb_pixmap_end: LazySymbol<unsafe fn(i: xcb_pixmap_iterator_t) -> xcb_generic_iterator_t>,
    xcb_cursor_next: LazySymbol<unsafe fn(i: *mut xcb_cursor_iterator_t)>,
    xcb_cursor_end: LazySymbol<unsafe fn(i: xcb_cursor_iterator_t) -> xcb_generic_iterator_t>,
    xcb_font_next: LazySymbol<unsafe fn(i: *mut xcb_font_iterator_t)>,
    xcb_font_end: LazySymbol<unsafe fn(i: xcb_font_iterator_t) -> xcb_generic_iterator_t>,
    xcb_gcontext_next: LazySymbol<unsafe fn(i: *mut xcb_gcontext_iterator_t)>,
    xcb_gcontext_end: LazySymbol<unsafe fn(i: xcb_gcontext_iterator_t) -> xcb_generic_iterator_t>,
    xcb_colormap_next: LazySymbol<unsafe fn(i: *mut xcb_colormap_iterator_t)>,
    xcb_colormap_end: LazySymbol<unsafe fn(i: xcb_colormap_iterator_t) -> xcb_generic_iterator_t>,
    xcb_atom_next: LazySymbol<unsafe fn(i: *mut xcb_atom_iterator_t)>,
    xcb_atom_end: LazySymbol<unsafe fn(i: xcb_atom_iterator_t) -> xcb_generic_iterator_t>,
    xcb_drawable_next: LazySymbol<unsafe fn(i: *mut xcb_drawable_iterator_t)>,
    xcb_drawable_end: LazySymbol<unsafe fn(i: xcb_drawable_iterator_t) -> xcb_generic_iterator_t>,
    xcb_fontable_next: LazySymbol<unsafe fn(i: *mut xcb_fontable_iterator_t)>,
    xcb_fontable_end: LazySymbol<unsafe fn(i: xcb_fontable_iterator_t) -> xcb_generic_iterator_t>,
    xcb_bool32_next: LazySymbol<unsafe fn(i: *mut xcb_bool32_iterator_t)>,
    xcb_bool32_end: LazySymbol<unsafe fn(i: xcb_bool32_iterator_t) -> xcb_generic_iterator_t>,
    xcb_visualid_next: LazySymbol<unsafe fn(i: *mut xcb_visualid_iterator_t)>,
    xcb_visualid_end: LazySymbol<unsafe fn(i: xcb_visualid_iterator_t) -> xcb_generic_iterator_t>,
    xcb_timestamp_next: LazySymbol<unsafe fn(i: *mut xcb_timestamp_iterator_t)>,
    xcb_timestamp_end: LazySymbol<unsafe fn(i: xcb_timestamp_iterator_t) -> xcb_generic_iterator_t>,
    xcb_keysym_next: LazySymbol<unsafe fn(i: *mut xcb_keysym_iterator_t)>,
    xcb_keysym_end: LazySymbol<unsafe fn(i: xcb_keysym_iterator_t) -> xcb_generic_iterator_t>,
    xcb_keycode_next: LazySymbol<unsafe fn(i: *mut xcb_keycode_iterator_t)>,
    xcb_keycode_end: LazySymbol<unsafe fn(i: xcb_keycode_iterator_t) -> xcb_generic_iterator_t>,
    xcb_keycode32_next: LazySymbol<unsafe fn(i: *mut xcb_keycode32_iterator_t)>,
    xcb_keycode32_end: LazySymbol<unsafe fn(i: xcb_keycode32_iterator_t) -> xcb_generic_iterator_t>,
    xcb_button_next: LazySymbol<unsafe fn(i: *mut xcb_button_iterator_t)>,
    xcb_button_end: LazySymbol<unsafe fn(i: xcb_button_iterator_t) -> xcb_generic_iterator_t>,
    xcb_point_next: LazySymbol<unsafe fn(i: *mut xcb_point_iterator_t)>,
    xcb_point_end: LazySymbol<unsafe fn(i: xcb_point_iterator_t) -> xcb_generic_iterator_t>,
    xcb_rectangle_next: LazySymbol<unsafe fn(i: *mut xcb_rectangle_iterator_t)>,
    xcb_rectangle_end: LazySymbol<unsafe fn(i: xcb_rectangle_iterator_t) -> xcb_generic_iterator_t>,
    xcb_arc_next: LazySymbol<unsafe fn(i: *mut xcb_arc_iterator_t)>,
    xcb_arc_end: LazySymbol<unsafe fn(i: xcb_arc_iterator_t) -> xcb_generic_iterator_t>,
    xcb_format_next: LazySymbol<unsafe fn(i: *mut xcb_format_iterator_t)>,
    xcb_format_end: LazySymbol<unsafe fn(i: xcb_format_iterator_t) -> xcb_generic_iterator_t>,
    xcb_visualtype_next: LazySymbol<unsafe fn(i: *mut xcb_visualtype_iterator_t)>,
    xcb_visualtype_end:
        LazySymbol<unsafe fn(i: xcb_visualtype_iterator_t) -> xcb_generic_iterator_t>,
    xcb_depth_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_depth_visuals: LazySymbol<unsafe fn(r: *const xcb_depth_t) -> *mut xcb_visualtype_t>,
    xcb_depth_visuals_length: LazySymbol<unsafe fn(r: *const xcb_depth_t) -> c_int>,
    xcb_depth_visuals_iterator:
        LazySymbol<unsafe fn(r: *const xcb_depth_t) -> xcb_visualtype_iterator_t>,
    xcb_depth_next: LazySymbol<unsafe fn(i: *mut xcb_depth_iterator_t)>,
    xcb_depth_end: LazySymbol<unsafe fn(i: xcb_depth_iterator_t) -> xcb_generic_iterator_t>,
    xcb_screen_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_screen_allowed_depths_length: LazySymbol<unsafe fn(r: *const xcb_screen_t) -> c_int>,
    xcb_screen_allowed_depths_iterator:
        LazySymbol<unsafe fn(r: *const xcb_screen_t) -> xcb_depth_iterator_t>,
    xcb_screen_next: LazySymbol<unsafe fn(i: *mut xcb_screen_iterator_t)>,
    xcb_screen_end: LazySymbol<unsafe fn(i: xcb_screen_iterator_t) -> xcb_generic_iterator_t>,
    xcb_setup_request_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_setup_request_authorization_protocol_name:
        LazySymbol<unsafe fn(r: *const xcb_setup_request_t) -> *mut c_char>,
    xcb_setup_request_authorization_protocol_name_length:
        LazySymbol<unsafe fn(r: *const xcb_setup_request_t) -> c_int>,
    xcb_setup_request_authorization_protocol_name_end:
        LazySymbol<unsafe fn(r: *const xcb_setup_request_t) -> xcb_generic_iterator_t>,
    xcb_setup_request_authorization_protocol_data:
        LazySymbol<unsafe fn(r: *const xcb_setup_request_t) -> *mut c_char>,
    xcb_setup_request_authorization_protocol_data_length:
        LazySymbol<unsafe fn(r: *const xcb_setup_request_t) -> c_int>,
    xcb_setup_request_authorization_protocol_data_end:
        LazySymbol<unsafe fn(r: *const xcb_setup_request_t) -> xcb_generic_iterator_t>,
    xcb_setup_request_next: LazySymbol<unsafe fn(i: *mut xcb_setup_request_iterator_t)>,
    xcb_setup_request_end:
        LazySymbol<unsafe fn(i: xcb_setup_request_iterator_t) -> xcb_generic_iterator_t>,
    xcb_setup_failed_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_setup_failed_reason: LazySymbol<unsafe fn(r: *const xcb_setup_failed_t) -> *mut c_char>,
    xcb_setup_failed_reason_length: LazySymbol<unsafe fn(r: *const xcb_setup_failed_t) -> c_int>,
    xcb_setup_failed_reason_end:
        LazySymbol<unsafe fn(r: *const xcb_setup_failed_t) -> xcb_generic_iterator_t>,
    xcb_setup_failed_next: LazySymbol<unsafe fn(i: *mut xcb_setup_failed_iterator_t)>,
    xcb_setup_failed_end:
        LazySymbol<unsafe fn(i: xcb_setup_failed_iterator_t) -> xcb_generic_iterator_t>,
    xcb_setup_authenticate_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_setup_authenticate_reason:
        LazySymbol<unsafe fn(r: *const xcb_setup_authenticate_t) -> *mut c_char>,
    xcb_setup_authenticate_reason_length:
        LazySymbol<unsafe fn(r: *const xcb_setup_authenticate_t) -> c_int>,
    xcb_setup_authenticate_reason_end:
        LazySymbol<unsafe fn(r: *const xcb_setup_authenticate_t) -> xcb_generic_iterator_t>,
    xcb_setup_authenticate_next: LazySymbol<unsafe fn(i: *mut xcb_setup_authenticate_iterator_t)>,
    xcb_setup_authenticate_end:
        LazySymbol<unsafe fn(i: xcb_setup_authenticate_iterator_t) -> xcb_generic_iterator_t>,
    xcb_setup_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_setup_vendor: LazySymbol<unsafe fn(r: *const xcb_setup_t) -> *mut c_char>,
    xcb_setup_vendor_length: LazySymbol<unsafe fn(r: *const xcb_setup_t) -> c_int>,
    xcb_setup_vendor_end: LazySymbol<unsafe fn(r: *const xcb_setup_t) -> xcb_generic_iterator_t>,
    xcb_setup_pixmap_formats: LazySymbol<unsafe fn(r: *const xcb_setup_t) -> *mut xcb_format_t>,
    xcb_setup_pixmap_formats_length: LazySymbol<unsafe fn(r: *const xcb_setup_t) -> c_int>,
    xcb_setup_pixmap_formats_iterator:
        LazySymbol<unsafe fn(r: *const xcb_setup_t) -> xcb_format_iterator_t>,
    xcb_setup_roots_length: LazySymbol<unsafe fn(r: *const xcb_setup_t) -> c_int>,
    xcb_setup_roots_iterator: LazySymbol<unsafe fn(r: *const xcb_setup_t) -> xcb_screen_iterator_t>,
    xcb_setup_next: LazySymbol<unsafe fn(i: *mut xcb_setup_iterator_t)>,
    xcb_setup_end: LazySymbol<unsafe fn(i: xcb_setup_iterator_t) -> xcb_generic_iterator_t>,
    xcb_client_message_data_next: LazySymbol<unsafe fn(i: *mut xcb_client_message_data_iterator_t)>,
    xcb_client_message_data_end:
        LazySymbol<unsafe fn(i: xcb_client_message_data_iterator_t) -> xcb_generic_iterator_t>,
    xcb_create_window_value_list_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            value_mask: u32,
            _aux: *const xcb_create_window_value_list_t,
        ) -> c_int,
    >,
    xcb_create_window_value_list_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            value_mask: u32,
            _aux: *mut xcb_create_window_value_list_t,
        ) -> c_int,
    >,
    xcb_create_window_value_list_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, value_mask: u32) -> c_int>,
    xcb_create_window_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_create_window_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            depth: u8,
            wid: xcb_window_t,
            parent: xcb_window_t,
            x: i16,
            y: i16,
            width: u16,
            height: u16,
            border_width: u16,
            class: u16,
            visual: xcb_visualid_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_create_window: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            depth: u8,
            wid: xcb_window_t,
            parent: xcb_window_t,
            x: i16,
            y: i16,
            width: u16,
            height: u16,
            border_width: u16,
            class: u16,
            visual: xcb_visualid_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_create_window_aux_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            depth: u8,
            wid: xcb_window_t,
            parent: xcb_window_t,
            x: i16,
            y: i16,
            width: u16,
            height: u16,
            border_width: u16,
            class: u16,
            visual: xcb_visualid_t,
            value_mask: u32,
            value_list: *const xcb_create_window_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_create_window_aux: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            depth: u8,
            wid: xcb_window_t,
            parent: xcb_window_t,
            x: i16,
            y: i16,
            width: u16,
            height: u16,
            border_width: u16,
            class: u16,
            visual: xcb_visualid_t,
            value_mask: u32,
            value_list: *const xcb_create_window_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_create_window_value_list:
        LazySymbol<unsafe fn(r: *const xcb_create_window_request_t) -> *mut c_void>,
    xcb_change_window_attributes_value_list_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            value_mask: u32,
            _aux: *const xcb_change_window_attributes_value_list_t,
        ) -> c_int,
    >,
    xcb_change_window_attributes_value_list_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            value_mask: u32,
            _aux: *mut xcb_change_window_attributes_value_list_t,
        ) -> c_int,
    >,
    xcb_change_window_attributes_value_list_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, value_mask: u32) -> c_int>,
    xcb_change_window_attributes_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_change_window_attributes_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_window_attributes: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_window_attributes_aux_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            value_mask: u32,
            value_list: *const xcb_change_window_attributes_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_window_attributes_aux: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            value_mask: u32,
            value_list: *const xcb_change_window_attributes_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_window_attributes_value_list:
        LazySymbol<unsafe fn(r: *const xcb_change_window_attributes_request_t) -> *mut c_void>,
    xcb_get_window_attributes: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_get_window_attributes_cookie_t,
    >,
    xcb_get_window_attributes_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_get_window_attributes_cookie_t,
    >,
    xcb_get_window_attributes_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_window_attributes_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_get_window_attributes_reply_t,
    >,
    xcb_destroy_window_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_destroy_window:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_destroy_subwindows_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_destroy_subwindows:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_change_save_set_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, mode: u8, window: xcb_window_t) -> xcb_void_cookie_t,
    >,
    xcb_change_save_set: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, mode: u8, window: xcb_window_t) -> xcb_void_cookie_t,
    >,
    xcb_reparent_window_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            parent: xcb_window_t,
            x: i16,
            y: i16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_reparent_window: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            parent: xcb_window_t,
            x: i16,
            y: i16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_map_window_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_map_window:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_map_subwindows_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_map_subwindows:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_unmap_window_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_unmap_window:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_unmap_subwindows_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_unmap_subwindows:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t>,
    xcb_configure_window_value_list_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            value_mask: u16,
            _aux: *const xcb_configure_window_value_list_t,
        ) -> c_int,
    >,
    xcb_configure_window_value_list_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            value_mask: u16,
            _aux: *mut xcb_configure_window_value_list_t,
        ) -> c_int,
    >,
    xcb_configure_window_value_list_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, value_mask: u16) -> c_int>,
    xcb_configure_window_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_configure_window_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            value_mask: u16,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_configure_window: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            value_mask: u16,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_configure_window_aux_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            value_mask: u16,
            value_list: *const xcb_configure_window_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_configure_window_aux: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            value_mask: u16,
            value_list: *const xcb_configure_window_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_configure_window_value_list:
        LazySymbol<unsafe fn(r: *const xcb_configure_window_request_t) -> *mut c_void>,
    xcb_circulate_window_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            direction: u8,
            window: xcb_window_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_circulate_window: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            direction: u8,
            window: xcb_window_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_get_geometry: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t) -> xcb_get_geometry_cookie_t,
    >,
    xcb_get_geometry_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, drawable: xcb_drawable_t) -> xcb_get_geometry_cookie_t,
    >,
    xcb_get_geometry_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_geometry_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_get_geometry_reply_t,
    >,
    xcb_query_tree_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_query_tree: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_query_tree_cookie_t,
    >,
    xcb_query_tree_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_query_tree_cookie_t,
    >,
    xcb_query_tree_children:
        LazySymbol<unsafe fn(r: *const xcb_query_tree_reply_t) -> *mut xcb_window_t>,
    xcb_query_tree_children_length:
        LazySymbol<unsafe fn(r: *const xcb_query_tree_reply_t) -> c_int>,
    xcb_query_tree_children_end:
        LazySymbol<unsafe fn(r: *const xcb_query_tree_reply_t) -> xcb_generic_iterator_t>,
    xcb_query_tree_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_query_tree_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_query_tree_reply_t,
    >,
    xcb_intern_atom_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_intern_atom: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            only_if_exists: u8,
            name_len: u16,
            name: *const c_char,
        ) -> xcb_intern_atom_cookie_t,
    >,
    xcb_intern_atom_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            only_if_exists: u8,
            name_len: u16,
            name: *const c_char,
        ) -> xcb_intern_atom_cookie_t,
    >,
    xcb_intern_atom_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_intern_atom_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_intern_atom_reply_t,
    >,
    xcb_get_atom_name_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_get_atom_name: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, atom: xcb_atom_t) -> xcb_get_atom_name_cookie_t,
    >,
    xcb_get_atom_name_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, atom: xcb_atom_t) -> xcb_get_atom_name_cookie_t,
    >,
    xcb_get_atom_name_name:
        LazySymbol<unsafe fn(r: *const xcb_get_atom_name_reply_t) -> *mut c_char>,
    xcb_get_atom_name_name_length:
        LazySymbol<unsafe fn(r: *const xcb_get_atom_name_reply_t) -> c_int>,
    xcb_get_atom_name_name_end:
        LazySymbol<unsafe fn(r: *const xcb_get_atom_name_reply_t) -> xcb_generic_iterator_t>,
    xcb_get_atom_name_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_atom_name_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_get_atom_name_reply_t,
    >,
    xcb_change_property_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_change_property_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            mode: u8,
            window: xcb_window_t,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            format: u8,
            data_len: u32,
            data: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_property: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            mode: u8,
            window: xcb_window_t,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            format: u8,
            data_len: u32,
            data: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_property_data:
        LazySymbol<unsafe fn(r: *const xcb_change_property_request_t) -> *mut c_void>,
    xcb_change_property_data_length:
        LazySymbol<unsafe fn(r: *const xcb_change_property_request_t) -> c_int>,
    xcb_change_property_data_end:
        LazySymbol<unsafe fn(r: *const xcb_change_property_request_t) -> xcb_generic_iterator_t>,
    xcb_delete_property_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            property: xcb_atom_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_delete_property: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            property: xcb_atom_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_get_property_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_get_property: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            delete: u8,
            window: xcb_window_t,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            long_offset: u32,
            long_length: u32,
        ) -> xcb_get_property_cookie_t,
    >,
    xcb_get_property_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            delete: u8,
            window: xcb_window_t,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            long_offset: u32,
            long_length: u32,
        ) -> xcb_get_property_cookie_t,
    >,
    xcb_get_property_value:
        LazySymbol<unsafe fn(r: *const xcb_get_property_reply_t) -> *mut c_void>,
    xcb_get_property_value_length:
        LazySymbol<unsafe fn(r: *const xcb_get_property_reply_t) -> c_int>,
    xcb_get_property_value_end:
        LazySymbol<unsafe fn(r: *const xcb_get_property_reply_t) -> xcb_generic_iterator_t>,
    xcb_get_property_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_property_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_get_property_reply_t,
    >,
    xcb_list_properties_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_list_properties: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_list_properties_cookie_t,
    >,
    xcb_list_properties_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_list_properties_cookie_t,
    >,
    xcb_list_properties_atoms:
        LazySymbol<unsafe fn(r: *const xcb_list_properties_reply_t) -> *mut xcb_atom_t>,
    xcb_list_properties_atoms_length:
        LazySymbol<unsafe fn(r: *const xcb_list_properties_reply_t) -> c_int>,
    xcb_list_properties_atoms_end:
        LazySymbol<unsafe fn(r: *const xcb_list_properties_reply_t) -> xcb_generic_iterator_t>,
    xcb_list_properties_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_list_properties_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_list_properties_reply_t,
    >,
    xcb_set_selection_owner_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            owner: xcb_window_t,
            selection: xcb_atom_t,
            time: xcb_timestamp_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_set_selection_owner: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            owner: xcb_window_t,
            selection: xcb_atom_t,
            time: xcb_timestamp_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_get_selection_owner: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            selection: xcb_atom_t,
        ) -> xcb_get_selection_owner_cookie_t,
    >,
    xcb_get_selection_owner_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            selection: xcb_atom_t,
        ) -> xcb_get_selection_owner_cookie_t,
    >,
    xcb_get_selection_owner_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_selection_owner_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_get_selection_owner_reply_t,
    >,
    xcb_convert_selection_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            requestor: xcb_window_t,
            selection: xcb_atom_t,
            target: xcb_atom_t,
            property: xcb_atom_t,
            time: xcb_timestamp_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_convert_selection: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            requestor: xcb_window_t,
            selection: xcb_atom_t,
            target: xcb_atom_t,
            property: xcb_atom_t,
            time: xcb_timestamp_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_send_event_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            propagate: u8,
            destination: xcb_window_t,
            event_mask: u32,
            event: *const [c_char; 32],
        ) -> xcb_void_cookie_t,
    >,
    xcb_send_event: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            propagate: u8,
            destination: xcb_window_t,
            event_mask: u32,
            event: *const [c_char; 32],
        ) -> xcb_void_cookie_t,
    >,
    xcb_grab_pointer: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            owner_events: u8,
            grab_window: xcb_window_t,
            event_mask: u16,
            pointer_mode: u8,
            keyboard_mode: u8,
            confine_to: xcb_window_t,
            cursor: xcb_cursor_t,
            time: xcb_timestamp_t,
        ) -> xcb_grab_pointer_cookie_t,
    >,
    xcb_grab_pointer_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            owner_events: u8,
            grab_window: xcb_window_t,
            event_mask: u16,
            pointer_mode: u8,
            keyboard_mode: u8,
            confine_to: xcb_window_t,
            cursor: xcb_cursor_t,
            time: xcb_timestamp_t,
        ) -> xcb_grab_pointer_cookie_t,
    >,
    xcb_grab_pointer_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_grab_pointer_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_grab_pointer_reply_t,
    >,
    xcb_ungrab_pointer_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, time: xcb_timestamp_t) -> xcb_void_cookie_t>,
    xcb_ungrab_pointer:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, time: xcb_timestamp_t) -> xcb_void_cookie_t>,
    xcb_grab_button_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            owner_events: u8,
            grab_window: xcb_window_t,
            event_mask: u16,
            pointer_mode: u8,
            keyboard_mode: u8,
            confine_to: xcb_window_t,
            cursor: xcb_cursor_t,
            button: u8,
            modifiers: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_grab_button: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            owner_events: u8,
            grab_window: xcb_window_t,
            event_mask: u16,
            pointer_mode: u8,
            keyboard_mode: u8,
            confine_to: xcb_window_t,
            cursor: xcb_cursor_t,
            button: u8,
            modifiers: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_ungrab_button_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            button: u8,
            grab_window: xcb_window_t,
            modifiers: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_ungrab_button: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            button: u8,
            grab_window: xcb_window_t,
            modifiers: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_active_pointer_grab_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cursor: xcb_cursor_t,
            time: xcb_timestamp_t,
            event_mask: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_active_pointer_grab: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cursor: xcb_cursor_t,
            time: xcb_timestamp_t,
            event_mask: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_grab_keyboard: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            owner_events: u8,
            grab_window: xcb_window_t,
            time: xcb_timestamp_t,
            pointer_mode: u8,
            keyboard_mode: u8,
        ) -> xcb_grab_keyboard_cookie_t,
    >,
    xcb_grab_keyboard_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            owner_events: u8,
            grab_window: xcb_window_t,
            time: xcb_timestamp_t,
            pointer_mode: u8,
            keyboard_mode: u8,
        ) -> xcb_grab_keyboard_cookie_t,
    >,
    xcb_grab_keyboard_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_grab_keyboard_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_grab_keyboard_reply_t,
    >,
    xcb_ungrab_keyboard_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, time: xcb_timestamp_t) -> xcb_void_cookie_t>,
    xcb_ungrab_keyboard:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, time: xcb_timestamp_t) -> xcb_void_cookie_t>,
    xcb_grab_key_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            owner_events: u8,
            grab_window: xcb_window_t,
            modifiers: u16,
            key: xcb_keycode_t,
            pointer_mode: u8,
            keyboard_mode: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_grab_key: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            owner_events: u8,
            grab_window: xcb_window_t,
            modifiers: u16,
            key: xcb_keycode_t,
            pointer_mode: u8,
            keyboard_mode: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_ungrab_key_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            key: xcb_keycode_t,
            grab_window: xcb_window_t,
            modifiers: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_ungrab_key: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            key: xcb_keycode_t,
            grab_window: xcb_window_t,
            modifiers: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_allow_events_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, mode: u8, time: xcb_timestamp_t) -> xcb_void_cookie_t,
    >,
    xcb_allow_events: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, mode: u8, time: xcb_timestamp_t) -> xcb_void_cookie_t,
    >,
    xcb_grab_server_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    xcb_grab_server: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    xcb_ungrab_server_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    xcb_ungrab_server: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    xcb_query_pointer: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_query_pointer_cookie_t,
    >,
    xcb_query_pointer_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, window: xcb_window_t) -> xcb_query_pointer_cookie_t,
    >,
    xcb_query_pointer_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_query_pointer_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_query_pointer_reply_t,
    >,
    xcb_timecoord_next: LazySymbol<unsafe fn(i: *mut xcb_timecoord_iterator_t)>,
    xcb_timecoord_end: LazySymbol<unsafe fn(i: xcb_timecoord_iterator_t) -> xcb_generic_iterator_t>,
    xcb_get_motion_events_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_get_motion_events: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            start: xcb_timestamp_t,
            stop: xcb_timestamp_t,
        ) -> xcb_get_motion_events_cookie_t,
    >,
    xcb_get_motion_events_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            start: xcb_timestamp_t,
            stop: xcb_timestamp_t,
        ) -> xcb_get_motion_events_cookie_t,
    >,
    xcb_get_motion_events_events:
        LazySymbol<unsafe fn(r: *const xcb_get_motion_events_reply_t) -> *mut xcb_timecoord_t>,
    xcb_get_motion_events_events_length:
        LazySymbol<unsafe fn(r: *const xcb_get_motion_events_reply_t) -> c_int>,
    xcb_get_motion_events_events_iterator:
        LazySymbol<unsafe fn(r: *const xcb_get_motion_events_reply_t) -> xcb_timecoord_iterator_t>,
    xcb_get_motion_events_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_motion_events_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_get_motion_events_reply_t,
    >,
    xcb_translate_coordinates: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src_window: xcb_window_t,
            dst_window: xcb_window_t,
            src_x: i16,
            src_y: i16,
        ) -> xcb_translate_coordinates_cookie_t,
    >,
    xcb_translate_coordinates_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src_window: xcb_window_t,
            dst_window: xcb_window_t,
            src_x: i16,
            src_y: i16,
        ) -> xcb_translate_coordinates_cookie_t,
    >,
    xcb_translate_coordinates_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_translate_coordinates_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_translate_coordinates_reply_t,
    >,
    xcb_warp_pointer_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src_window: xcb_window_t,
            dst_window: xcb_window_t,
            src_x: i16,
            src_y: i16,
            src_width: u16,
            src_height: u16,
            dst_x: i16,
            dst_y: i16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_warp_pointer: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src_window: xcb_window_t,
            dst_window: xcb_window_t,
            src_x: i16,
            src_y: i16,
            src_width: u16,
            src_height: u16,
            dst_x: i16,
            dst_y: i16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_set_input_focus_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            revert_to: u8,
            focus: xcb_window_t,
            time: xcb_timestamp_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_set_input_focus: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            revert_to: u8,
            focus: xcb_window_t,
            time: xcb_timestamp_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_get_input_focus:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_get_input_focus_cookie_t>,
    xcb_get_input_focus_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_get_input_focus_cookie_t>,
    xcb_get_input_focus_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_input_focus_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_get_input_focus_reply_t,
    >,
    xcb_query_keymap: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_query_keymap_cookie_t>,
    xcb_query_keymap_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_query_keymap_cookie_t>,
    xcb_query_keymap_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_query_keymap_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_query_keymap_reply_t,
    >,
    xcb_open_font_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_open_font_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            fid: xcb_font_t,
            name_len: u16,
            name: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_open_font: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            fid: xcb_font_t,
            name_len: u16,
            name: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_open_font_name: LazySymbol<unsafe fn(r: *const xcb_open_font_request_t) -> *mut c_char>,
    xcb_open_font_name_length: LazySymbol<unsafe fn(r: *const xcb_open_font_request_t) -> c_int>,
    xcb_open_font_name_end:
        LazySymbol<unsafe fn(r: *const xcb_open_font_request_t) -> xcb_generic_iterator_t>,
    xcb_close_font_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, font: xcb_font_t) -> xcb_void_cookie_t>,
    xcb_close_font:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, font: xcb_font_t) -> xcb_void_cookie_t>,
    xcb_fontprop_next: LazySymbol<unsafe fn(i: *mut xcb_fontprop_iterator_t)>,
    xcb_fontprop_end: LazySymbol<unsafe fn(i: xcb_fontprop_iterator_t) -> xcb_generic_iterator_t>,
    xcb_charinfo_next: LazySymbol<unsafe fn(i: *mut xcb_charinfo_iterator_t)>,
    xcb_charinfo_end: LazySymbol<unsafe fn(i: xcb_charinfo_iterator_t) -> xcb_generic_iterator_t>,
    xcb_query_font_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_query_font: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, font: xcb_fontable_t) -> xcb_query_font_cookie_t,
    >,
    xcb_query_font_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, font: xcb_fontable_t) -> xcb_query_font_cookie_t,
    >,
    xcb_query_font_properties:
        LazySymbol<unsafe fn(r: *const xcb_query_font_reply_t) -> *mut xcb_fontprop_t>,
    xcb_query_font_properties_length:
        LazySymbol<unsafe fn(r: *const xcb_query_font_reply_t) -> c_int>,
    xcb_query_font_properties_iterator:
        LazySymbol<unsafe fn(r: *const xcb_query_font_reply_t) -> xcb_fontprop_iterator_t>,
    xcb_query_font_char_infos:
        LazySymbol<unsafe fn(r: *const xcb_query_font_reply_t) -> *mut xcb_charinfo_t>,
    xcb_query_font_char_infos_length:
        LazySymbol<unsafe fn(r: *const xcb_query_font_reply_t) -> c_int>,
    xcb_query_font_char_infos_iterator:
        LazySymbol<unsafe fn(r: *const xcb_query_font_reply_t) -> xcb_charinfo_iterator_t>,
    xcb_query_font_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_query_font_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_query_font_reply_t,
    >,
    xcb_query_text_extents_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, string_len: u32) -> c_int>,
    xcb_query_text_extents: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            font: xcb_fontable_t,
            string_len: u32,
            string: *const xcb_char2b_t,
        ) -> xcb_query_text_extents_cookie_t,
    >,
    xcb_query_text_extents_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            font: xcb_fontable_t,
            string_len: u32,
            string: *const xcb_char2b_t,
        ) -> xcb_query_text_extents_cookie_t,
    >,
    xcb_query_text_extents_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_query_text_extents_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_query_text_extents_reply_t,
    >,
    xcb_str_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_str_name: LazySymbol<unsafe fn(r: *const xcb_str_t) -> *mut c_char>,
    xcb_str_name_length: LazySymbol<unsafe fn(r: *const xcb_str_t) -> c_int>,
    xcb_str_name_end: LazySymbol<unsafe fn(r: *const xcb_str_t) -> xcb_generic_iterator_t>,
    xcb_str_next: LazySymbol<unsafe fn(i: *mut xcb_str_iterator_t)>,
    xcb_str_end: LazySymbol<unsafe fn(i: xcb_str_iterator_t) -> xcb_generic_iterator_t>,
    xcb_list_fonts_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_list_fonts: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            max_names: u16,
            pattern_len: u16,
            pattern: *const c_char,
        ) -> xcb_list_fonts_cookie_t,
    >,
    xcb_list_fonts_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            max_names: u16,
            pattern_len: u16,
            pattern: *const c_char,
        ) -> xcb_list_fonts_cookie_t,
    >,
    xcb_list_fonts_names_length: LazySymbol<unsafe fn(r: *const xcb_list_fonts_reply_t) -> c_int>,
    xcb_list_fonts_names_iterator:
        LazySymbol<unsafe fn(r: *const xcb_list_fonts_reply_t) -> xcb_str_iterator_t>,
    xcb_list_fonts_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_list_fonts_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_list_fonts_reply_t,
    >,
    xcb_list_fonts_with_info_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_list_fonts_with_info: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            max_names: u16,
            pattern_len: u16,
            pattern: *const c_char,
        ) -> xcb_list_fonts_with_info_cookie_t,
    >,
    xcb_list_fonts_with_info_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            max_names: u16,
            pattern_len: u16,
            pattern: *const c_char,
        ) -> xcb_list_fonts_with_info_cookie_t,
    >,
    xcb_list_fonts_with_info_properties:
        LazySymbol<unsafe fn(r: *const xcb_list_fonts_with_info_reply_t) -> *mut xcb_fontprop_t>,
    xcb_list_fonts_with_info_properties_length:
        LazySymbol<unsafe fn(r: *const xcb_list_fonts_with_info_reply_t) -> c_int>,
    xcb_list_fonts_with_info_properties_iterator: LazySymbol<
        unsafe fn(r: *const xcb_list_fonts_with_info_reply_t) -> xcb_fontprop_iterator_t,
    >,
    xcb_list_fonts_with_info_name:
        LazySymbol<unsafe fn(r: *const xcb_list_fonts_with_info_reply_t) -> *mut c_char>,
    xcb_list_fonts_with_info_name_length:
        LazySymbol<unsafe fn(r: *const xcb_list_fonts_with_info_reply_t) -> c_int>,
    xcb_list_fonts_with_info_name_end:
        LazySymbol<unsafe fn(r: *const xcb_list_fonts_with_info_reply_t) -> xcb_generic_iterator_t>,
    xcb_list_fonts_with_info_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_list_fonts_with_info_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_list_fonts_with_info_reply_t,
    >,
    xcb_set_font_path_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_set_font_path_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            font_qty: u16,
            font: *const xcb_str_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_set_font_path: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            font_qty: u16,
            font: *const xcb_str_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_set_font_path_font_length:
        LazySymbol<unsafe fn(r: *const xcb_set_font_path_request_t) -> c_int>,
    xcb_set_font_path_font_iterator:
        LazySymbol<unsafe fn(r: *const xcb_set_font_path_request_t) -> xcb_str_iterator_t>,
    xcb_get_font_path_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_get_font_path:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_get_font_path_cookie_t>,
    xcb_get_font_path_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_get_font_path_cookie_t>,
    xcb_get_font_path_path_length:
        LazySymbol<unsafe fn(r: *const xcb_get_font_path_reply_t) -> c_int>,
    xcb_get_font_path_path_iterator:
        LazySymbol<unsafe fn(r: *const xcb_get_font_path_reply_t) -> xcb_str_iterator_t>,
    xcb_get_font_path_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_font_path_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_get_font_path_reply_t,
    >,
    xcb_create_pixmap_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            depth: u8,
            pid: xcb_pixmap_t,
            drawable: xcb_drawable_t,
            width: u16,
            height: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_create_pixmap: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            depth: u8,
            pid: xcb_pixmap_t,
            drawable: xcb_drawable_t,
            width: u16,
            height: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_free_pixmap_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, pixmap: xcb_pixmap_t) -> xcb_void_cookie_t>,
    xcb_free_pixmap:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, pixmap: xcb_pixmap_t) -> xcb_void_cookie_t>,
    xcb_create_gc_value_list_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            value_mask: u32,
            _aux: *const xcb_create_gc_value_list_t,
        ) -> c_int,
    >,
    xcb_create_gc_value_list_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            value_mask: u32,
            _aux: *mut xcb_create_gc_value_list_t,
        ) -> c_int,
    >,
    xcb_create_gc_value_list_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, value_mask: u32) -> c_int>,
    xcb_create_gc_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_create_gc_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cid: xcb_gcontext_t,
            drawable: xcb_drawable_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_create_gc: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cid: xcb_gcontext_t,
            drawable: xcb_drawable_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_create_gc_aux_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cid: xcb_gcontext_t,
            drawable: xcb_drawable_t,
            value_mask: u32,
            value_list: *const xcb_create_gc_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_create_gc_aux: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cid: xcb_gcontext_t,
            drawable: xcb_drawable_t,
            value_mask: u32,
            value_list: *const xcb_create_gc_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_create_gc_value_list:
        LazySymbol<unsafe fn(r: *const xcb_create_gc_request_t) -> *mut c_void>,
    xcb_change_gc_value_list_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            value_mask: u32,
            _aux: *const xcb_change_gc_value_list_t,
        ) -> c_int,
    >,
    xcb_change_gc_value_list_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            value_mask: u32,
            _aux: *mut xcb_change_gc_value_list_t,
        ) -> c_int,
    >,
    xcb_change_gc_value_list_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, value_mask: u32) -> c_int>,
    xcb_change_gc_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_change_gc_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            gc: xcb_gcontext_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_gc: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            gc: xcb_gcontext_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_gc_aux_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            gc: xcb_gcontext_t,
            value_mask: u32,
            value_list: *const xcb_change_gc_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_gc_aux: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            gc: xcb_gcontext_t,
            value_mask: u32,
            value_list: *const xcb_change_gc_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_gc_value_list:
        LazySymbol<unsafe fn(r: *const xcb_change_gc_request_t) -> *mut c_void>,
    xcb_copy_gc_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src_gc: xcb_gcontext_t,
            dst_gc: xcb_gcontext_t,
            value_mask: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_copy_gc: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src_gc: xcb_gcontext_t,
            dst_gc: xcb_gcontext_t,
            value_mask: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_set_dashes_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_set_dashes_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            gc: xcb_gcontext_t,
            dash_offset: u16,
            dashes_len: u16,
            dashes: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_set_dashes: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            gc: xcb_gcontext_t,
            dash_offset: u16,
            dashes_len: u16,
            dashes: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_set_dashes_dashes: LazySymbol<unsafe fn(r: *const xcb_set_dashes_request_t) -> *mut u8>,
    xcb_set_dashes_dashes_length:
        LazySymbol<unsafe fn(r: *const xcb_set_dashes_request_t) -> c_int>,
    xcb_set_dashes_dashes_end:
        LazySymbol<unsafe fn(r: *const xcb_set_dashes_request_t) -> xcb_generic_iterator_t>,
    xcb_set_clip_rectangles_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, rectangles_len: u32) -> c_int>,
    xcb_set_clip_rectangles_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            ordering: u8,
            gc: xcb_gcontext_t,
            clip_x_origin: i16,
            clip_y_origin: i16,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_set_clip_rectangles: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            ordering: u8,
            gc: xcb_gcontext_t,
            clip_x_origin: i16,
            clip_y_origin: i16,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_set_clip_rectangles_rectangles:
        LazySymbol<unsafe fn(r: *const xcb_set_clip_rectangles_request_t) -> *mut xcb_rectangle_t>,
    xcb_set_clip_rectangles_rectangles_length:
        LazySymbol<unsafe fn(r: *const xcb_set_clip_rectangles_request_t) -> c_int>,
    xcb_set_clip_rectangles_rectangles_iterator: LazySymbol<
        unsafe fn(r: *const xcb_set_clip_rectangles_request_t) -> xcb_rectangle_iterator_t,
    >,
    xcb_free_gc_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, gc: xcb_gcontext_t) -> xcb_void_cookie_t>,
    xcb_free_gc:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, gc: xcb_gcontext_t) -> xcb_void_cookie_t>,
    xcb_clear_area_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            exposures: u8,
            window: xcb_window_t,
            x: i16,
            y: i16,
            width: u16,
            height: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_clear_area: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            exposures: u8,
            window: xcb_window_t,
            x: i16,
            y: i16,
            width: u16,
            height: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_copy_area_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src_drawable: xcb_drawable_t,
            dst_drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            src_x: i16,
            src_y: i16,
            dst_x: i16,
            dst_y: i16,
            width: u16,
            height: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_copy_area: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src_drawable: xcb_drawable_t,
            dst_drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            src_x: i16,
            src_y: i16,
            dst_x: i16,
            dst_y: i16,
            width: u16,
            height: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_copy_plane_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src_drawable: xcb_drawable_t,
            dst_drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            src_x: i16,
            src_y: i16,
            dst_x: i16,
            dst_y: i16,
            width: u16,
            height: u16,
            bit_plane: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_copy_plane: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src_drawable: xcb_drawable_t,
            dst_drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            src_x: i16,
            src_y: i16,
            dst_x: i16,
            dst_y: i16,
            width: u16,
            height: u16,
            bit_plane: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_point_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void, points_len: u32) -> c_int>,
    xcb_poly_point_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            coordinate_mode: u8,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            points_len: u32,
            points: *const xcb_point_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_point: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            coordinate_mode: u8,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            points_len: u32,
            points: *const xcb_point_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_point_points:
        LazySymbol<unsafe fn(r: *const xcb_poly_point_request_t) -> *mut xcb_point_t>,
    xcb_poly_point_points_length:
        LazySymbol<unsafe fn(r: *const xcb_poly_point_request_t) -> c_int>,
    xcb_poly_point_points_iterator:
        LazySymbol<unsafe fn(r: *const xcb_poly_point_request_t) -> xcb_point_iterator_t>,
    xcb_poly_line_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void, points_len: u32) -> c_int>,
    xcb_poly_line_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            coordinate_mode: u8,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            points_len: u32,
            points: *const xcb_point_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_line: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            coordinate_mode: u8,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            points_len: u32,
            points: *const xcb_point_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_line_points:
        LazySymbol<unsafe fn(r: *const xcb_poly_line_request_t) -> *mut xcb_point_t>,
    xcb_poly_line_points_length: LazySymbol<unsafe fn(r: *const xcb_poly_line_request_t) -> c_int>,
    xcb_poly_line_points_iterator:
        LazySymbol<unsafe fn(r: *const xcb_poly_line_request_t) -> xcb_point_iterator_t>,
    xcb_segment_next: LazySymbol<unsafe fn(i: *mut xcb_segment_iterator_t)>,
    xcb_segment_end: LazySymbol<unsafe fn(i: xcb_segment_iterator_t) -> xcb_generic_iterator_t>,
    xcb_poly_segment_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, segments_len: u32) -> c_int>,
    xcb_poly_segment_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            segments_len: u32,
            segments: *const xcb_segment_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_segment: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            segments_len: u32,
            segments: *const xcb_segment_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_segment_segments:
        LazySymbol<unsafe fn(r: *const xcb_poly_segment_request_t) -> *mut xcb_segment_t>,
    xcb_poly_segment_segments_length:
        LazySymbol<unsafe fn(r: *const xcb_poly_segment_request_t) -> c_int>,
    xcb_poly_segment_segments_iterator:
        LazySymbol<unsafe fn(r: *const xcb_poly_segment_request_t) -> xcb_segment_iterator_t>,
    xcb_poly_rectangle_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, rectangles_len: u32) -> c_int>,
    xcb_poly_rectangle_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_rectangle: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_rectangle_rectangles:
        LazySymbol<unsafe fn(r: *const xcb_poly_rectangle_request_t) -> *mut xcb_rectangle_t>,
    xcb_poly_rectangle_rectangles_length:
        LazySymbol<unsafe fn(r: *const xcb_poly_rectangle_request_t) -> c_int>,
    xcb_poly_rectangle_rectangles_iterator:
        LazySymbol<unsafe fn(r: *const xcb_poly_rectangle_request_t) -> xcb_rectangle_iterator_t>,
    xcb_poly_arc_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void, arcs_len: u32) -> c_int>,
    xcb_poly_arc_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            arcs_len: u32,
            arcs: *const xcb_arc_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_arc: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            arcs_len: u32,
            arcs: *const xcb_arc_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_arc_arcs: LazySymbol<unsafe fn(r: *const xcb_poly_arc_request_t) -> *mut xcb_arc_t>,
    xcb_poly_arc_arcs_length: LazySymbol<unsafe fn(r: *const xcb_poly_arc_request_t) -> c_int>,
    xcb_poly_arc_arcs_iterator:
        LazySymbol<unsafe fn(r: *const xcb_poly_arc_request_t) -> xcb_arc_iterator_t>,
    xcb_fill_poly_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void, points_len: u32) -> c_int>,
    xcb_fill_poly_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            shape: u8,
            coordinate_mode: u8,
            points_len: u32,
            points: *const xcb_point_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_fill_poly: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            shape: u8,
            coordinate_mode: u8,
            points_len: u32,
            points: *const xcb_point_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_fill_poly_points:
        LazySymbol<unsafe fn(r: *const xcb_fill_poly_request_t) -> *mut xcb_point_t>,
    xcb_fill_poly_points_length: LazySymbol<unsafe fn(r: *const xcb_fill_poly_request_t) -> c_int>,
    xcb_fill_poly_points_iterator:
        LazySymbol<unsafe fn(r: *const xcb_fill_poly_request_t) -> xcb_point_iterator_t>,
    xcb_poly_fill_rectangle_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, rectangles_len: u32) -> c_int>,
    xcb_poly_fill_rectangle_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_fill_rectangle: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            rectangles_len: u32,
            rectangles: *const xcb_rectangle_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_fill_rectangle_rectangles:
        LazySymbol<unsafe fn(r: *const xcb_poly_fill_rectangle_request_t) -> *mut xcb_rectangle_t>,
    xcb_poly_fill_rectangle_rectangles_length:
        LazySymbol<unsafe fn(r: *const xcb_poly_fill_rectangle_request_t) -> c_int>,
    xcb_poly_fill_rectangle_rectangles_iterator: LazySymbol<
        unsafe fn(r: *const xcb_poly_fill_rectangle_request_t) -> xcb_rectangle_iterator_t,
    >,
    xcb_poly_fill_arc_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void, arcs_len: u32) -> c_int>,
    xcb_poly_fill_arc_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            arcs_len: u32,
            arcs: *const xcb_arc_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_fill_arc: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            arcs_len: u32,
            arcs: *const xcb_arc_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_fill_arc_arcs:
        LazySymbol<unsafe fn(r: *const xcb_poly_fill_arc_request_t) -> *mut xcb_arc_t>,
    xcb_poly_fill_arc_arcs_length:
        LazySymbol<unsafe fn(r: *const xcb_poly_fill_arc_request_t) -> c_int>,
    xcb_poly_fill_arc_arcs_iterator:
        LazySymbol<unsafe fn(r: *const xcb_poly_fill_arc_request_t) -> xcb_arc_iterator_t>,
    xcb_put_image_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void, data_len: u32) -> c_int>,
    xcb_put_image_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            format: u8,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            width: u16,
            height: u16,
            dst_x: i16,
            dst_y: i16,
            left_pad: u8,
            depth: u8,
            data_len: u32,
            data: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_put_image: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            format: u8,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            width: u16,
            height: u16,
            dst_x: i16,
            dst_y: i16,
            left_pad: u8,
            depth: u8,
            data_len: u32,
            data: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_put_image_data: LazySymbol<unsafe fn(r: *const xcb_put_image_request_t) -> *mut u8>,
    xcb_put_image_data_length: LazySymbol<unsafe fn(r: *const xcb_put_image_request_t) -> c_int>,
    xcb_put_image_data_end:
        LazySymbol<unsafe fn(r: *const xcb_put_image_request_t) -> xcb_generic_iterator_t>,
    xcb_get_image_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_get_image: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            format: u8,
            drawable: xcb_drawable_t,
            x: i16,
            y: i16,
            width: u16,
            height: u16,
            plane_mask: u32,
        ) -> xcb_get_image_cookie_t,
    >,
    xcb_get_image_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            format: u8,
            drawable: xcb_drawable_t,
            x: i16,
            y: i16,
            width: u16,
            height: u16,
            plane_mask: u32,
        ) -> xcb_get_image_cookie_t,
    >,
    xcb_get_image_data: LazySymbol<unsafe fn(r: *const xcb_get_image_reply_t) -> *mut u8>,
    xcb_get_image_data_length: LazySymbol<unsafe fn(r: *const xcb_get_image_reply_t) -> c_int>,
    xcb_get_image_data_end:
        LazySymbol<unsafe fn(r: *const xcb_get_image_reply_t) -> xcb_generic_iterator_t>,
    xcb_get_image_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_image_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_get_image_reply_t,
    >,
    xcb_poly_text_8_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void, items_len: u32) -> c_int>,
    xcb_poly_text_8_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            x: i16,
            y: i16,
            items_len: u32,
            items: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_text_8: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            x: i16,
            y: i16,
            items_len: u32,
            items: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_text_8_items: LazySymbol<unsafe fn(r: *const xcb_poly_text_8_request_t) -> *mut u8>,
    xcb_poly_text_8_items_length:
        LazySymbol<unsafe fn(r: *const xcb_poly_text_8_request_t) -> c_int>,
    xcb_poly_text_8_items_end:
        LazySymbol<unsafe fn(r: *const xcb_poly_text_8_request_t) -> xcb_generic_iterator_t>,
    xcb_poly_text_16_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void, items_len: u32) -> c_int>,
    xcb_poly_text_16_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            x: i16,
            y: i16,
            items_len: u32,
            items: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_text_16: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            x: i16,
            y: i16,
            items_len: u32,
            items: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_poly_text_16_items: LazySymbol<unsafe fn(r: *const xcb_poly_text_16_request_t) -> *mut u8>,
    xcb_poly_text_16_items_length:
        LazySymbol<unsafe fn(r: *const xcb_poly_text_16_request_t) -> c_int>,
    xcb_poly_text_16_items_end:
        LazySymbol<unsafe fn(r: *const xcb_poly_text_16_request_t) -> xcb_generic_iterator_t>,
    xcb_image_text_8_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_image_text_8_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            string_len: u8,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            x: i16,
            y: i16,
            string: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_image_text_8: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            string_len: u8,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            x: i16,
            y: i16,
            string: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_image_text_8_string:
        LazySymbol<unsafe fn(r: *const xcb_image_text_8_request_t) -> *mut c_char>,
    xcb_image_text_8_string_length:
        LazySymbol<unsafe fn(r: *const xcb_image_text_8_request_t) -> c_int>,
    xcb_image_text_8_string_end:
        LazySymbol<unsafe fn(r: *const xcb_image_text_8_request_t) -> xcb_generic_iterator_t>,
    xcb_image_text_16_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_image_text_16_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            string_len: u8,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            x: i16,
            y: i16,
            string: *const xcb_char2b_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_image_text_16: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            string_len: u8,
            drawable: xcb_drawable_t,
            gc: xcb_gcontext_t,
            x: i16,
            y: i16,
            string: *const xcb_char2b_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_image_text_16_string:
        LazySymbol<unsafe fn(r: *const xcb_image_text_16_request_t) -> *mut xcb_char2b_t>,
    xcb_image_text_16_string_length:
        LazySymbol<unsafe fn(r: *const xcb_image_text_16_request_t) -> c_int>,
    xcb_image_text_16_string_iterator:
        LazySymbol<unsafe fn(r: *const xcb_image_text_16_request_t) -> xcb_char2b_iterator_t>,
    xcb_create_colormap_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            alloc: u8,
            mid: xcb_colormap_t,
            window: xcb_window_t,
            visual: xcb_visualid_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_create_colormap: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            alloc: u8,
            mid: xcb_colormap_t,
            window: xcb_window_t,
            visual: xcb_visualid_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_free_colormap_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cmap: xcb_colormap_t) -> xcb_void_cookie_t>,
    xcb_free_colormap:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cmap: xcb_colormap_t) -> xcb_void_cookie_t>,
    xcb_copy_colormap_and_free_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            mid: xcb_colormap_t,
            src_cmap: xcb_colormap_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_copy_colormap_and_free: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            mid: xcb_colormap_t,
            src_cmap: xcb_colormap_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_install_colormap_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cmap: xcb_colormap_t) -> xcb_void_cookie_t>,
    xcb_install_colormap:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cmap: xcb_colormap_t) -> xcb_void_cookie_t>,
    xcb_uninstall_colormap_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cmap: xcb_colormap_t) -> xcb_void_cookie_t>,
    xcb_uninstall_colormap:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cmap: xcb_colormap_t) -> xcb_void_cookie_t>,
    xcb_list_installed_colormaps_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_list_installed_colormaps: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_list_installed_colormaps_cookie_t,
    >,
    xcb_list_installed_colormaps_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_list_installed_colormaps_cookie_t,
    >,
    xcb_list_installed_colormaps_cmaps: LazySymbol<
        unsafe fn(r: *const xcb_list_installed_colormaps_reply_t) -> *mut xcb_colormap_t,
    >,
    xcb_list_installed_colormaps_cmaps_length:
        LazySymbol<unsafe fn(r: *const xcb_list_installed_colormaps_reply_t) -> c_int>,
    xcb_list_installed_colormaps_cmaps_end: LazySymbol<
        unsafe fn(r: *const xcb_list_installed_colormaps_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_list_installed_colormaps_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_list_installed_colormaps_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_list_installed_colormaps_reply_t,
    >,
    xcb_alloc_color: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cmap: xcb_colormap_t,
            red: u16,
            green: u16,
            blue: u16,
        ) -> xcb_alloc_color_cookie_t,
    >,
    xcb_alloc_color_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cmap: xcb_colormap_t,
            red: u16,
            green: u16,
            blue: u16,
        ) -> xcb_alloc_color_cookie_t,
    >,
    xcb_alloc_color_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_alloc_color_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_alloc_color_reply_t,
    >,
    xcb_alloc_named_color_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_alloc_named_color: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cmap: xcb_colormap_t,
            name_len: u16,
            name: *const c_char,
        ) -> xcb_alloc_named_color_cookie_t,
    >,
    xcb_alloc_named_color_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cmap: xcb_colormap_t,
            name_len: u16,
            name: *const c_char,
        ) -> xcb_alloc_named_color_cookie_t,
    >,
    xcb_alloc_named_color_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_alloc_named_color_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_alloc_named_color_reply_t,
    >,
    xcb_alloc_color_cells_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_alloc_color_cells: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            contiguous: u8,
            cmap: xcb_colormap_t,
            colors: u16,
            planes: u16,
        ) -> xcb_alloc_color_cells_cookie_t,
    >,
    xcb_alloc_color_cells_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            contiguous: u8,
            cmap: xcb_colormap_t,
            colors: u16,
            planes: u16,
        ) -> xcb_alloc_color_cells_cookie_t,
    >,
    xcb_alloc_color_cells_pixels:
        LazySymbol<unsafe fn(r: *const xcb_alloc_color_cells_reply_t) -> *mut u32>,
    xcb_alloc_color_cells_pixels_length:
        LazySymbol<unsafe fn(r: *const xcb_alloc_color_cells_reply_t) -> c_int>,
    xcb_alloc_color_cells_pixels_end:
        LazySymbol<unsafe fn(r: *const xcb_alloc_color_cells_reply_t) -> xcb_generic_iterator_t>,
    xcb_alloc_color_cells_masks:
        LazySymbol<unsafe fn(r: *const xcb_alloc_color_cells_reply_t) -> *mut u32>,
    xcb_alloc_color_cells_masks_length:
        LazySymbol<unsafe fn(r: *const xcb_alloc_color_cells_reply_t) -> c_int>,
    xcb_alloc_color_cells_masks_end:
        LazySymbol<unsafe fn(r: *const xcb_alloc_color_cells_reply_t) -> xcb_generic_iterator_t>,
    xcb_alloc_color_cells_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_alloc_color_cells_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_alloc_color_cells_reply_t,
    >,
    xcb_alloc_color_planes_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_alloc_color_planes: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            contiguous: u8,
            cmap: xcb_colormap_t,
            colors: u16,
            reds: u16,
            greens: u16,
            blues: u16,
        ) -> xcb_alloc_color_planes_cookie_t,
    >,
    xcb_alloc_color_planes_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            contiguous: u8,
            cmap: xcb_colormap_t,
            colors: u16,
            reds: u16,
            greens: u16,
            blues: u16,
        ) -> xcb_alloc_color_planes_cookie_t,
    >,
    xcb_alloc_color_planes_pixels:
        LazySymbol<unsafe fn(r: *const xcb_alloc_color_planes_reply_t) -> *mut u32>,
    xcb_alloc_color_planes_pixels_length:
        LazySymbol<unsafe fn(r: *const xcb_alloc_color_planes_reply_t) -> c_int>,
    xcb_alloc_color_planes_pixels_end:
        LazySymbol<unsafe fn(r: *const xcb_alloc_color_planes_reply_t) -> xcb_generic_iterator_t>,
    xcb_alloc_color_planes_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_alloc_color_planes_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_alloc_color_planes_reply_t,
    >,
    xcb_free_colors_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void, pixels_len: u32) -> c_int>,
    xcb_free_colors_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cmap: xcb_colormap_t,
            plane_mask: u32,
            pixels_len: u32,
            pixels: *const u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_free_colors: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cmap: xcb_colormap_t,
            plane_mask: u32,
            pixels_len: u32,
            pixels: *const u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_free_colors_pixels: LazySymbol<unsafe fn(r: *const xcb_free_colors_request_t) -> *mut u32>,
    xcb_free_colors_pixels_length:
        LazySymbol<unsafe fn(r: *const xcb_free_colors_request_t) -> c_int>,
    xcb_free_colors_pixels_end:
        LazySymbol<unsafe fn(r: *const xcb_free_colors_request_t) -> xcb_generic_iterator_t>,
    xcb_coloritem_next: LazySymbol<unsafe fn(i: *mut xcb_coloritem_iterator_t)>,
    xcb_coloritem_end: LazySymbol<unsafe fn(i: xcb_coloritem_iterator_t) -> xcb_generic_iterator_t>,
    xcb_store_colors_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void, items_len: u32) -> c_int>,
    xcb_store_colors_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cmap: xcb_colormap_t,
            items_len: u32,
            items: *const xcb_coloritem_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_store_colors: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cmap: xcb_colormap_t,
            items_len: u32,
            items: *const xcb_coloritem_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_store_colors_items:
        LazySymbol<unsafe fn(r: *const xcb_store_colors_request_t) -> *mut xcb_coloritem_t>,
    xcb_store_colors_items_length:
        LazySymbol<unsafe fn(r: *const xcb_store_colors_request_t) -> c_int>,
    xcb_store_colors_items_iterator:
        LazySymbol<unsafe fn(r: *const xcb_store_colors_request_t) -> xcb_coloritem_iterator_t>,
    xcb_store_named_color_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_store_named_color_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            flags: u8,
            cmap: xcb_colormap_t,
            pixel: u32,
            name_len: u16,
            name: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_store_named_color: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            flags: u8,
            cmap: xcb_colormap_t,
            pixel: u32,
            name_len: u16,
            name: *const c_char,
        ) -> xcb_void_cookie_t,
    >,
    xcb_store_named_color_name:
        LazySymbol<unsafe fn(r: *const xcb_store_named_color_request_t) -> *mut c_char>,
    xcb_store_named_color_name_length:
        LazySymbol<unsafe fn(r: *const xcb_store_named_color_request_t) -> c_int>,
    xcb_store_named_color_name_end:
        LazySymbol<unsafe fn(r: *const xcb_store_named_color_request_t) -> xcb_generic_iterator_t>,
    xcb_rgb_next: LazySymbol<unsafe fn(i: *mut xcb_rgb_iterator_t)>,
    xcb_rgb_end: LazySymbol<unsafe fn(i: xcb_rgb_iterator_t) -> xcb_generic_iterator_t>,
    xcb_query_colors_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, pixels_len: u32) -> c_int>,
    xcb_query_colors: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cmap: xcb_colormap_t,
            pixels_len: u32,
            pixels: *const u32,
        ) -> xcb_query_colors_cookie_t,
    >,
    xcb_query_colors_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cmap: xcb_colormap_t,
            pixels_len: u32,
            pixels: *const u32,
        ) -> xcb_query_colors_cookie_t,
    >,
    xcb_query_colors_colors:
        LazySymbol<unsafe fn(r: *const xcb_query_colors_reply_t) -> *mut xcb_rgb_t>,
    xcb_query_colors_colors_length:
        LazySymbol<unsafe fn(r: *const xcb_query_colors_reply_t) -> c_int>,
    xcb_query_colors_colors_iterator:
        LazySymbol<unsafe fn(r: *const xcb_query_colors_reply_t) -> xcb_rgb_iterator_t>,
    xcb_query_colors_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_query_colors_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_query_colors_reply_t,
    >,
    xcb_lookup_color_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_lookup_color: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cmap: xcb_colormap_t,
            name_len: u16,
            name: *const c_char,
        ) -> xcb_lookup_color_cookie_t,
    >,
    xcb_lookup_color_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cmap: xcb_colormap_t,
            name_len: u16,
            name: *const c_char,
        ) -> xcb_lookup_color_cookie_t,
    >,
    xcb_lookup_color_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_lookup_color_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_lookup_color_reply_t,
    >,
    xcb_create_cursor_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cid: xcb_cursor_t,
            source: xcb_pixmap_t,
            mask: xcb_pixmap_t,
            fore_red: u16,
            fore_green: u16,
            fore_blue: u16,
            back_red: u16,
            back_green: u16,
            back_blue: u16,
            x: u16,
            y: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_create_cursor: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cid: xcb_cursor_t,
            source: xcb_pixmap_t,
            mask: xcb_pixmap_t,
            fore_red: u16,
            fore_green: u16,
            fore_blue: u16,
            back_red: u16,
            back_green: u16,
            back_blue: u16,
            x: u16,
            y: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_create_glyph_cursor_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cid: xcb_cursor_t,
            source_font: xcb_font_t,
            mask_font: xcb_font_t,
            source_char: u16,
            mask_char: u16,
            fore_red: u16,
            fore_green: u16,
            fore_blue: u16,
            back_red: u16,
            back_green: u16,
            back_blue: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_create_glyph_cursor: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cid: xcb_cursor_t,
            source_font: xcb_font_t,
            mask_font: xcb_font_t,
            source_char: u16,
            mask_char: u16,
            fore_red: u16,
            fore_green: u16,
            fore_blue: u16,
            back_red: u16,
            back_green: u16,
            back_blue: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_free_cursor_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cursor: xcb_cursor_t) -> xcb_void_cookie_t>,
    xcb_free_cursor:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, cursor: xcb_cursor_t) -> xcb_void_cookie_t>,
    xcb_recolor_cursor_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cursor: xcb_cursor_t,
            fore_red: u16,
            fore_green: u16,
            fore_blue: u16,
            back_red: u16,
            back_green: u16,
            back_blue: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_recolor_cursor: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cursor: xcb_cursor_t,
            fore_red: u16,
            fore_green: u16,
            fore_blue: u16,
            back_red: u16,
            back_green: u16,
            back_blue: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_query_best_size: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            class: u8,
            drawable: xcb_drawable_t,
            width: u16,
            height: u16,
        ) -> xcb_query_best_size_cookie_t,
    >,
    xcb_query_best_size_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            class: u8,
            drawable: xcb_drawable_t,
            width: u16,
            height: u16,
        ) -> xcb_query_best_size_cookie_t,
    >,
    xcb_query_best_size_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_query_best_size_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_query_best_size_reply_t,
    >,
    xcb_query_extension_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_query_extension: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            name_len: u16,
            name: *const c_char,
        ) -> xcb_query_extension_cookie_t,
    >,
    xcb_query_extension_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            name_len: u16,
            name: *const c_char,
        ) -> xcb_query_extension_cookie_t,
    >,
    xcb_query_extension_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_query_extension_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_query_extension_reply_t,
    >,
    xcb_list_extensions_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_list_extensions:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_list_extensions_cookie_t>,
    xcb_list_extensions_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_list_extensions_cookie_t>,
    xcb_list_extensions_names_length:
        LazySymbol<unsafe fn(r: *const xcb_list_extensions_reply_t) -> c_int>,
    xcb_list_extensions_names_iterator:
        LazySymbol<unsafe fn(r: *const xcb_list_extensions_reply_t) -> xcb_str_iterator_t>,
    xcb_list_extensions_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_list_extensions_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_list_extensions_reply_t,
    >,
    xcb_change_keyboard_mapping_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_change_keyboard_mapping_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            keycode_count: u8,
            first_keycode: xcb_keycode_t,
            keysyms_per_keycode: u8,
            keysyms: *const xcb_keysym_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_keyboard_mapping: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            keycode_count: u8,
            first_keycode: xcb_keycode_t,
            keysyms_per_keycode: u8,
            keysyms: *const xcb_keysym_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_keyboard_mapping_keysyms:
        LazySymbol<unsafe fn(r: *const xcb_change_keyboard_mapping_request_t) -> *mut xcb_keysym_t>,
    xcb_change_keyboard_mapping_keysyms_length:
        LazySymbol<unsafe fn(r: *const xcb_change_keyboard_mapping_request_t) -> c_int>,
    xcb_change_keyboard_mapping_keysyms_end: LazySymbol<
        unsafe fn(r: *const xcb_change_keyboard_mapping_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_get_keyboard_mapping_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_get_keyboard_mapping: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            first_keycode: xcb_keycode_t,
            count: u8,
        ) -> xcb_get_keyboard_mapping_cookie_t,
    >,
    xcb_get_keyboard_mapping_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            first_keycode: xcb_keycode_t,
            count: u8,
        ) -> xcb_get_keyboard_mapping_cookie_t,
    >,
    xcb_get_keyboard_mapping_keysyms:
        LazySymbol<unsafe fn(r: *const xcb_get_keyboard_mapping_reply_t) -> *mut xcb_keysym_t>,
    xcb_get_keyboard_mapping_keysyms_length:
        LazySymbol<unsafe fn(r: *const xcb_get_keyboard_mapping_reply_t) -> c_int>,
    xcb_get_keyboard_mapping_keysyms_end:
        LazySymbol<unsafe fn(r: *const xcb_get_keyboard_mapping_reply_t) -> xcb_generic_iterator_t>,
    xcb_get_keyboard_mapping_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_keyboard_mapping_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_get_keyboard_mapping_reply_t,
    >,
    xcb_change_keyboard_control_value_list_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            value_mask: u32,
            _aux: *const xcb_change_keyboard_control_value_list_t,
        ) -> c_int,
    >,
    xcb_change_keyboard_control_value_list_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            value_mask: u32,
            _aux: *mut xcb_change_keyboard_control_value_list_t,
        ) -> c_int,
    >,
    xcb_change_keyboard_control_value_list_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, value_mask: u32) -> c_int>,
    xcb_change_keyboard_control_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_change_keyboard_control_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_keyboard_control: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            value_mask: u32,
            value_list: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_keyboard_control_aux_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            value_mask: u32,
            value_list: *const xcb_change_keyboard_control_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_keyboard_control_aux: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            value_mask: u32,
            value_list: *const xcb_change_keyboard_control_value_list_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_keyboard_control_value_list:
        LazySymbol<unsafe fn(r: *const xcb_change_keyboard_control_request_t) -> *mut c_void>,
    xcb_get_keyboard_control:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_get_keyboard_control_cookie_t>,
    xcb_get_keyboard_control_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_get_keyboard_control_cookie_t>,
    xcb_get_keyboard_control_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_keyboard_control_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_get_keyboard_control_reply_t,
    >,
    xcb_bell_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, percent: i8) -> xcb_void_cookie_t>,
    xcb_bell: LazySymbol<unsafe fn(c: *mut xcb_connection_t, percent: i8) -> xcb_void_cookie_t>,
    xcb_change_pointer_control_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            acceleration_numerator: i16,
            acceleration_denominator: i16,
            threshold: i16,
            do_acceleration: u8,
            do_threshold: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_pointer_control: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            acceleration_numerator: i16,
            acceleration_denominator: i16,
            threshold: i16,
            do_acceleration: u8,
            do_threshold: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_get_pointer_control:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_get_pointer_control_cookie_t>,
    xcb_get_pointer_control_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_get_pointer_control_cookie_t>,
    xcb_get_pointer_control_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_pointer_control_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_get_pointer_control_reply_t,
    >,
    xcb_set_screen_saver_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            timeout: i16,
            interval: i16,
            prefer_blanking: u8,
            allow_exposures: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_set_screen_saver: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            timeout: i16,
            interval: i16,
            prefer_blanking: u8,
            allow_exposures: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_get_screen_saver:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_get_screen_saver_cookie_t>,
    xcb_get_screen_saver_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_get_screen_saver_cookie_t>,
    xcb_get_screen_saver_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_screen_saver_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_get_screen_saver_reply_t,
    >,
    xcb_change_hosts_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_change_hosts_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            mode: u8,
            family: u8,
            address_len: u16,
            address: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_hosts: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            mode: u8,
            family: u8,
            address_len: u16,
            address: *const u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_change_hosts_address:
        LazySymbol<unsafe fn(r: *const xcb_change_hosts_request_t) -> *mut u8>,
    xcb_change_hosts_address_length:
        LazySymbol<unsafe fn(r: *const xcb_change_hosts_request_t) -> c_int>,
    xcb_change_hosts_address_end:
        LazySymbol<unsafe fn(r: *const xcb_change_hosts_request_t) -> xcb_generic_iterator_t>,
    xcb_host_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_host_address: LazySymbol<unsafe fn(r: *const xcb_host_t) -> *mut u8>,
    xcb_host_address_length: LazySymbol<unsafe fn(r: *const xcb_host_t) -> c_int>,
    xcb_host_address_end: LazySymbol<unsafe fn(r: *const xcb_host_t) -> xcb_generic_iterator_t>,
    xcb_host_next: LazySymbol<unsafe fn(i: *mut xcb_host_iterator_t)>,
    xcb_host_end: LazySymbol<unsafe fn(i: xcb_host_iterator_t) -> xcb_generic_iterator_t>,
    xcb_list_hosts_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_list_hosts: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_list_hosts_cookie_t>,
    xcb_list_hosts_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_list_hosts_cookie_t>,
    xcb_list_hosts_hosts_length: LazySymbol<unsafe fn(r: *const xcb_list_hosts_reply_t) -> c_int>,
    xcb_list_hosts_hosts_iterator:
        LazySymbol<unsafe fn(r: *const xcb_list_hosts_reply_t) -> xcb_host_iterator_t>,
    xcb_list_hosts_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_list_hosts_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_list_hosts_reply_t,
    >,
    xcb_set_access_control_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, mode: u8) -> xcb_void_cookie_t>,
    xcb_set_access_control:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, mode: u8) -> xcb_void_cookie_t>,
    xcb_set_close_down_mode_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, mode: u8) -> xcb_void_cookie_t>,
    xcb_set_close_down_mode:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, mode: u8) -> xcb_void_cookie_t>,
    xcb_kill_client_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, resource: u32) -> xcb_void_cookie_t>,
    xcb_kill_client:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, resource: u32) -> xcb_void_cookie_t>,
    xcb_rotate_properties_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_rotate_properties_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            atoms_len: u16,
            delta: i16,
            atoms: *const xcb_atom_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_rotate_properties: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            atoms_len: u16,
            delta: i16,
            atoms: *const xcb_atom_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_rotate_properties_atoms:
        LazySymbol<unsafe fn(r: *const xcb_rotate_properties_request_t) -> *mut xcb_atom_t>,
    xcb_rotate_properties_atoms_length:
        LazySymbol<unsafe fn(r: *const xcb_rotate_properties_request_t) -> c_int>,
    xcb_rotate_properties_atoms_end:
        LazySymbol<unsafe fn(r: *const xcb_rotate_properties_request_t) -> xcb_generic_iterator_t>,
    xcb_force_screen_saver_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, mode: u8) -> xcb_void_cookie_t>,
    xcb_force_screen_saver:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, mode: u8) -> xcb_void_cookie_t>,
    xcb_set_pointer_mapping_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_set_pointer_mapping: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            map_len: u8,
            map: *const u8,
        ) -> xcb_set_pointer_mapping_cookie_t,
    >,
    xcb_set_pointer_mapping_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            map_len: u8,
            map: *const u8,
        ) -> xcb_set_pointer_mapping_cookie_t,
    >,
    xcb_set_pointer_mapping_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_set_pointer_mapping_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_set_pointer_mapping_reply_t,
    >,
    xcb_get_pointer_mapping_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_get_pointer_mapping:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_get_pointer_mapping_cookie_t>,
    xcb_get_pointer_mapping_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_get_pointer_mapping_cookie_t>,
    xcb_get_pointer_mapping_map:
        LazySymbol<unsafe fn(r: *const xcb_get_pointer_mapping_reply_t) -> *mut u8>,
    xcb_get_pointer_mapping_map_length:
        LazySymbol<unsafe fn(r: *const xcb_get_pointer_mapping_reply_t) -> c_int>,
    xcb_get_pointer_mapping_map_end:
        LazySymbol<unsafe fn(r: *const xcb_get_pointer_mapping_reply_t) -> xcb_generic_iterator_t>,
    xcb_get_pointer_mapping_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_pointer_mapping_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_get_pointer_mapping_reply_t,
    >,
    xcb_set_modifier_mapping_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_set_modifier_mapping: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            keycodes_per_modifier: u8,
            keycodes: *const xcb_keycode_t,
        ) -> xcb_set_modifier_mapping_cookie_t,
    >,
    xcb_set_modifier_mapping_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            keycodes_per_modifier: u8,
            keycodes: *const xcb_keycode_t,
        ) -> xcb_set_modifier_mapping_cookie_t,
    >,
    xcb_set_modifier_mapping_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_set_modifier_mapping_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_set_modifier_mapping_reply_t,
    >,
    xcb_get_modifier_mapping_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_get_modifier_mapping:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_get_modifier_mapping_cookie_t>,
    xcb_get_modifier_mapping_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_get_modifier_mapping_cookie_t>,
    xcb_get_modifier_mapping_keycodes:
        LazySymbol<unsafe fn(r: *const xcb_get_modifier_mapping_reply_t) -> *mut xcb_keycode_t>,
    xcb_get_modifier_mapping_keycodes_length:
        LazySymbol<unsafe fn(r: *const xcb_get_modifier_mapping_reply_t) -> c_int>,
    xcb_get_modifier_mapping_keycodes_end:
        LazySymbol<unsafe fn(r: *const xcb_get_modifier_mapping_reply_t) -> xcb_generic_iterator_t>,
    xcb_get_modifier_mapping_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_get_modifier_mapping_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_get_modifier_mapping_reply_t,
    >,
    xcb_no_operation_checked: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
    xcb_no_operation: LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_void_cookie_t>,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self
            .xproto
            .$f
            .get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .xproto
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

impl Xcb {
    /// Advances a `xcb_char2b_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_char2b_next(&self, i: *mut xcb_char2b_iterator_t) {
        sym!(self, xcb_char2b_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_char2b_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_char2b_next(&self) -> bool {
        has_sym!(self, xcb_char2b_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_char2b_iterator_t`.
    #[inline]
    pub unsafe fn xcb_char2b_end(&self, i: xcb_char2b_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_char2b_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_char2b_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_char2b_end(&self) -> bool {
        has_sym!(self, xcb_char2b_end)
    }

    /// Advances a `xcb_window_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_window_next(&self, i: *mut xcb_window_iterator_t) {
        sym!(self, xcb_window_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_window_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_window_next(&self) -> bool {
        has_sym!(self, xcb_window_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_window_iterator_t`.
    #[inline]
    pub unsafe fn xcb_window_end(&self, i: xcb_window_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_window_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_window_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_window_end(&self) -> bool {
        has_sym!(self, xcb_window_end)
    }

    /// Advances a `xcb_pixmap_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_pixmap_next(&self, i: *mut xcb_pixmap_iterator_t) {
        sym!(self, xcb_pixmap_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_pixmap_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_pixmap_next(&self) -> bool {
        has_sym!(self, xcb_pixmap_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_pixmap_iterator_t`.
    #[inline]
    pub unsafe fn xcb_pixmap_end(&self, i: xcb_pixmap_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_pixmap_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_pixmap_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_pixmap_end(&self) -> bool {
        has_sym!(self, xcb_pixmap_end)
    }

    /// Advances a `xcb_cursor_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_cursor_next(&self, i: *mut xcb_cursor_iterator_t) {
        sym!(self, xcb_cursor_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_cursor_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_cursor_next(&self) -> bool {
        has_sym!(self, xcb_cursor_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_cursor_iterator_t`.
    #[inline]
    pub unsafe fn xcb_cursor_end(&self, i: xcb_cursor_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_cursor_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_cursor_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_cursor_end(&self) -> bool {
        has_sym!(self, xcb_cursor_end)
    }

    /// Advances a `xcb_font_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_font_next(&self, i: *mut xcb_font_iterator_t) {
        sym!(self, xcb_font_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_font_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_font_next(&self) -> bool {
        has_sym!(self, xcb_font_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_font_iterator_t`.
    #[inline]
    pub unsafe fn xcb_font_end(&self, i: xcb_font_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_font_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_font_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_font_end(&self) -> bool {
        has_sym!(self, xcb_font_end)
    }

    /// Advances a `xcb_gcontext_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_gcontext_next(&self, i: *mut xcb_gcontext_iterator_t) {
        sym!(self, xcb_gcontext_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_gcontext_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_gcontext_next(&self) -> bool {
        has_sym!(self, xcb_gcontext_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_gcontext_iterator_t`.
    #[inline]
    pub unsafe fn xcb_gcontext_end(&self, i: xcb_gcontext_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_gcontext_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_gcontext_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_gcontext_end(&self) -> bool {
        has_sym!(self, xcb_gcontext_end)
    }

    /// Advances a `xcb_colormap_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_colormap_next(&self, i: *mut xcb_colormap_iterator_t) {
        sym!(self, xcb_colormap_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_colormap_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_colormap_next(&self) -> bool {
        has_sym!(self, xcb_colormap_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_colormap_iterator_t`.
    #[inline]
    pub unsafe fn xcb_colormap_end(&self, i: xcb_colormap_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_colormap_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_colormap_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_colormap_end(&self) -> bool {
        has_sym!(self, xcb_colormap_end)
    }

    /// Advances a `xcb_atom_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_atom_next(&self, i: *mut xcb_atom_iterator_t) {
        sym!(self, xcb_atom_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_atom_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_atom_next(&self) -> bool {
        has_sym!(self, xcb_atom_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_atom_iterator_t`.
    #[inline]
    pub unsafe fn xcb_atom_end(&self, i: xcb_atom_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_atom_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_atom_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_atom_end(&self) -> bool {
        has_sym!(self, xcb_atom_end)
    }

    /// Advances a `xcb_drawable_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_drawable_next(&self, i: *mut xcb_drawable_iterator_t) {
        sym!(self, xcb_drawable_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_drawable_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_drawable_next(&self) -> bool {
        has_sym!(self, xcb_drawable_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_drawable_iterator_t`.
    #[inline]
    pub unsafe fn xcb_drawable_end(&self, i: xcb_drawable_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_drawable_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_drawable_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_drawable_end(&self) -> bool {
        has_sym!(self, xcb_drawable_end)
    }

    /// Advances a `xcb_fontable_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_fontable_next(&self, i: *mut xcb_fontable_iterator_t) {
        sym!(self, xcb_fontable_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_fontable_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_fontable_next(&self) -> bool {
        has_sym!(self, xcb_fontable_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_fontable_iterator_t`.
    #[inline]
    pub unsafe fn xcb_fontable_end(&self, i: xcb_fontable_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_fontable_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_fontable_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_fontable_end(&self) -> bool {
        has_sym!(self, xcb_fontable_end)
    }

    /// Advances a `xcb_bool32_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_bool32_next(&self, i: *mut xcb_bool32_iterator_t) {
        sym!(self, xcb_bool32_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_bool32_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_bool32_next(&self) -> bool {
        has_sym!(self, xcb_bool32_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_bool32_iterator_t`.
    #[inline]
    pub unsafe fn xcb_bool32_end(&self, i: xcb_bool32_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_bool32_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_bool32_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_bool32_end(&self) -> bool {
        has_sym!(self, xcb_bool32_end)
    }

    /// Advances a `xcb_visualid_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_visualid_next(&self, i: *mut xcb_visualid_iterator_t) {
        sym!(self, xcb_visualid_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_visualid_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_visualid_next(&self) -> bool {
        has_sym!(self, xcb_visualid_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_visualid_iterator_t`.
    #[inline]
    pub unsafe fn xcb_visualid_end(&self, i: xcb_visualid_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_visualid_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_visualid_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_visualid_end(&self) -> bool {
        has_sym!(self, xcb_visualid_end)
    }

    /// Advances a `xcb_timestamp_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_timestamp_next(&self, i: *mut xcb_timestamp_iterator_t) {
        sym!(self, xcb_timestamp_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_timestamp_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_timestamp_next(&self) -> bool {
        has_sym!(self, xcb_timestamp_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_timestamp_iterator_t`.
    #[inline]
    pub unsafe fn xcb_timestamp_end(&self, i: xcb_timestamp_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_timestamp_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_timestamp_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_timestamp_end(&self) -> bool {
        has_sym!(self, xcb_timestamp_end)
    }

    /// Advances a `xcb_keysym_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_keysym_next(&self, i: *mut xcb_keysym_iterator_t) {
        sym!(self, xcb_keysym_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_keysym_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_keysym_next(&self) -> bool {
        has_sym!(self, xcb_keysym_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_keysym_iterator_t`.
    #[inline]
    pub unsafe fn xcb_keysym_end(&self, i: xcb_keysym_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_keysym_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_keysym_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_keysym_end(&self) -> bool {
        has_sym!(self, xcb_keysym_end)
    }

    /// Advances a `xcb_keycode_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_keycode_next(&self, i: *mut xcb_keycode_iterator_t) {
        sym!(self, xcb_keycode_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_keycode_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_keycode_next(&self) -> bool {
        has_sym!(self, xcb_keycode_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_keycode_iterator_t`.
    #[inline]
    pub unsafe fn xcb_keycode_end(&self, i: xcb_keycode_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_keycode_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_keycode_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_keycode_end(&self) -> bool {
        has_sym!(self, xcb_keycode_end)
    }

    /// Advances a `xcb_keycode32_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_keycode32_next(&self, i: *mut xcb_keycode32_iterator_t) {
        sym!(self, xcb_keycode32_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_keycode32_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_keycode32_next(&self) -> bool {
        has_sym!(self, xcb_keycode32_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_keycode32_iterator_t`.
    #[inline]
    pub unsafe fn xcb_keycode32_end(&self, i: xcb_keycode32_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_keycode32_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_keycode32_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_keycode32_end(&self) -> bool {
        has_sym!(self, xcb_keycode32_end)
    }

    /// Advances a `xcb_button_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_button_next(&self, i: *mut xcb_button_iterator_t) {
        sym!(self, xcb_button_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_button_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_button_next(&self) -> bool {
        has_sym!(self, xcb_button_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_button_iterator_t`.
    #[inline]
    pub unsafe fn xcb_button_end(&self, i: xcb_button_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_button_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_button_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_button_end(&self) -> bool {
        has_sym!(self, xcb_button_end)
    }

    /// Advances a `xcb_point_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_point_next(&self, i: *mut xcb_point_iterator_t) {
        sym!(self, xcb_point_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_point_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_point_next(&self) -> bool {
        has_sym!(self, xcb_point_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_point_iterator_t`.
    #[inline]
    pub unsafe fn xcb_point_end(&self, i: xcb_point_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_point_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_point_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_point_end(&self) -> bool {
        has_sym!(self, xcb_point_end)
    }

    /// Advances a `xcb_rectangle_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_rectangle_next(&self, i: *mut xcb_rectangle_iterator_t) {
        sym!(self, xcb_rectangle_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_rectangle_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_rectangle_next(&self) -> bool {
        has_sym!(self, xcb_rectangle_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_rectangle_iterator_t`.
    #[inline]
    pub unsafe fn xcb_rectangle_end(&self, i: xcb_rectangle_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_rectangle_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_rectangle_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_rectangle_end(&self) -> bool {
        has_sym!(self, xcb_rectangle_end)
    }

    /// Advances a `xcb_arc_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_arc_next(&self, i: *mut xcb_arc_iterator_t) {
        sym!(self, xcb_arc_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_arc_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_arc_next(&self) -> bool {
        has_sym!(self, xcb_arc_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_arc_iterator_t`.
    #[inline]
    pub unsafe fn xcb_arc_end(&self, i: xcb_arc_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_arc_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_arc_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_arc_end(&self) -> bool {
        has_sym!(self, xcb_arc_end)
    }

    /// Advances a `xcb_format_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_format_next(&self, i: *mut xcb_format_iterator_t) {
        sym!(self, xcb_format_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_format_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_format_next(&self) -> bool {
        has_sym!(self, xcb_format_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_format_iterator_t`.
    #[inline]
    pub unsafe fn xcb_format_end(&self, i: xcb_format_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_format_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_format_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_format_end(&self) -> bool {
        has_sym!(self, xcb_format_end)
    }

    /// Advances a `xcb_visualtype_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_visualtype_next(&self, i: *mut xcb_visualtype_iterator_t) {
        sym!(self, xcb_visualtype_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_visualtype_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_visualtype_next(&self) -> bool {
        has_sym!(self, xcb_visualtype_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_visualtype_iterator_t`.
    #[inline]
    pub unsafe fn xcb_visualtype_end(
        &self,
        i: xcb_visualtype_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_visualtype_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_visualtype_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_visualtype_end(&self) -> bool {
        has_sym!(self, xcb_visualtype_end)
    }

    /// Computes the size of a `xcb_depth_t` object.
    #[inline]
    pub unsafe fn xcb_depth_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_depth_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_depth_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_depth_sizeof(&self) -> bool {
        has_sym!(self, xcb_depth_sizeof)
    }

    /// Returns a pointer to the `visuals` field of a `xcb_depth_t` struct.
    #[inline]
    pub unsafe fn xcb_depth_visuals(&self, r: *const xcb_depth_t) -> *mut xcb_visualtype_t {
        sym!(self, xcb_depth_visuals)(r)
    }

    /// Returns `true` iff the symbol `xcb_depth_visuals` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_depth_visuals(&self) -> bool {
        has_sym!(self, xcb_depth_visuals)
    }

    /// Returns the number of elements of the `visuals` field of a `xcb_depth_t` struct.
    #[inline]
    pub unsafe fn xcb_depth_visuals_length(&self, r: *const xcb_depth_t) -> c_int {
        sym!(self, xcb_depth_visuals_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_depth_visuals_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_depth_visuals_length(&self) -> bool {
        has_sym!(self, xcb_depth_visuals_length)
    }

    /// Returns an iterator over the elements of the
    /// `visuals` field of a `xcb_depth_t` struct.
    #[inline]
    pub unsafe fn xcb_depth_visuals_iterator(
        &self,
        r: *const xcb_depth_t,
    ) -> xcb_visualtype_iterator_t {
        sym!(self, xcb_depth_visuals_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_depth_visuals_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_depth_visuals_iterator(&self) -> bool {
        has_sym!(self, xcb_depth_visuals_iterator)
    }

    /// Advances a `xcb_depth_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_depth_next(&self, i: *mut xcb_depth_iterator_t) {
        sym!(self, xcb_depth_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_depth_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_depth_next(&self) -> bool {
        has_sym!(self, xcb_depth_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_depth_iterator_t`.
    #[inline]
    pub unsafe fn xcb_depth_end(&self, i: xcb_depth_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_depth_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_depth_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_depth_end(&self) -> bool {
        has_sym!(self, xcb_depth_end)
    }

    /// Computes the size of a `xcb_screen_t` object.
    #[inline]
    pub unsafe fn xcb_screen_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_screen_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_screen_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screen_sizeof(&self) -> bool {
        has_sym!(self, xcb_screen_sizeof)
    }

    /// Returns the number of elements of the `allowed_depths` field of a `xcb_screen_t` struct.
    #[inline]
    pub unsafe fn xcb_screen_allowed_depths_length(&self, r: *const xcb_screen_t) -> c_int {
        sym!(self, xcb_screen_allowed_depths_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_screen_allowed_depths_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screen_allowed_depths_length(&self) -> bool {
        has_sym!(self, xcb_screen_allowed_depths_length)
    }

    /// Returns an iterator over the elements of the
    /// `allowed_depths` field of a `xcb_screen_t` struct.
    #[inline]
    pub unsafe fn xcb_screen_allowed_depths_iterator(
        &self,
        r: *const xcb_screen_t,
    ) -> xcb_depth_iterator_t {
        sym!(self, xcb_screen_allowed_depths_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_screen_allowed_depths_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screen_allowed_depths_iterator(&self) -> bool {
        has_sym!(self, xcb_screen_allowed_depths_iterator)
    }

    /// Advances a `xcb_screen_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_screen_next(&self, i: *mut xcb_screen_iterator_t) {
        sym!(self, xcb_screen_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_screen_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screen_next(&self) -> bool {
        has_sym!(self, xcb_screen_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_screen_iterator_t`.
    #[inline]
    pub unsafe fn xcb_screen_end(&self, i: xcb_screen_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_screen_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_screen_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_screen_end(&self) -> bool {
        has_sym!(self, xcb_screen_end)
    }

    /// Computes the size of a `xcb_setup_request_t` object.
    #[inline]
    pub unsafe fn xcb_setup_request_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_setup_request_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_setup_request_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_request_sizeof(&self) -> bool {
        has_sym!(self, xcb_setup_request_sizeof)
    }

    /// Returns a pointer to the `authorization_protocol_name` field of a `xcb_setup_request_t` struct.
    #[inline]
    pub unsafe fn xcb_setup_request_authorization_protocol_name(
        &self,
        r: *const xcb_setup_request_t,
    ) -> *mut c_char {
        sym!(self, xcb_setup_request_authorization_protocol_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_request_authorization_protocol_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_request_authorization_protocol_name(&self) -> bool {
        has_sym!(self, xcb_setup_request_authorization_protocol_name)
    }

    /// Returns the number of elements of the `authorization_protocol_name` field of a `xcb_setup_request_t` struct.
    #[inline]
    pub unsafe fn xcb_setup_request_authorization_protocol_name_length(
        &self,
        r: *const xcb_setup_request_t,
    ) -> c_int {
        sym!(self, xcb_setup_request_authorization_protocol_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_request_authorization_protocol_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_request_authorization_protocol_name_length(&self) -> bool {
        has_sym!(self, xcb_setup_request_authorization_protocol_name_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `authorization_protocol_name` field of a `xcb_setup_request_t` struct.
    #[inline]
    pub unsafe fn xcb_setup_request_authorization_protocol_name_end(
        &self,
        r: *const xcb_setup_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_setup_request_authorization_protocol_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_request_authorization_protocol_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_request_authorization_protocol_name_end(&self) -> bool {
        has_sym!(self, xcb_setup_request_authorization_protocol_name_end)
    }

    /// Returns a pointer to the `authorization_protocol_data` field of a `xcb_setup_request_t` struct.
    #[inline]
    pub unsafe fn xcb_setup_request_authorization_protocol_data(
        &self,
        r: *const xcb_setup_request_t,
    ) -> *mut c_char {
        sym!(self, xcb_setup_request_authorization_protocol_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_request_authorization_protocol_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_request_authorization_protocol_data(&self) -> bool {
        has_sym!(self, xcb_setup_request_authorization_protocol_data)
    }

    /// Returns the number of elements of the `authorization_protocol_data` field of a `xcb_setup_request_t` struct.
    #[inline]
    pub unsafe fn xcb_setup_request_authorization_protocol_data_length(
        &self,
        r: *const xcb_setup_request_t,
    ) -> c_int {
        sym!(self, xcb_setup_request_authorization_protocol_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_request_authorization_protocol_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_request_authorization_protocol_data_length(&self) -> bool {
        has_sym!(self, xcb_setup_request_authorization_protocol_data_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `authorization_protocol_data` field of a `xcb_setup_request_t` struct.
    #[inline]
    pub unsafe fn xcb_setup_request_authorization_protocol_data_end(
        &self,
        r: *const xcb_setup_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_setup_request_authorization_protocol_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_request_authorization_protocol_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_request_authorization_protocol_data_end(&self) -> bool {
        has_sym!(self, xcb_setup_request_authorization_protocol_data_end)
    }

    /// Advances a `xcb_setup_request_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_setup_request_next(&self, i: *mut xcb_setup_request_iterator_t) {
        sym!(self, xcb_setup_request_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_setup_request_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_request_next(&self) -> bool {
        has_sym!(self, xcb_setup_request_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_setup_request_iterator_t`.
    #[inline]
    pub unsafe fn xcb_setup_request_end(
        &self,
        i: xcb_setup_request_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_setup_request_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_setup_request_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_request_end(&self) -> bool {
        has_sym!(self, xcb_setup_request_end)
    }

    /// Computes the size of a `xcb_setup_failed_t` object.
    #[inline]
    pub unsafe fn xcb_setup_failed_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_setup_failed_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_setup_failed_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_failed_sizeof(&self) -> bool {
        has_sym!(self, xcb_setup_failed_sizeof)
    }

    /// Returns a pointer to the `reason` field of a `xcb_setup_failed_t` struct.
    #[inline]
    pub unsafe fn xcb_setup_failed_reason(&self, r: *const xcb_setup_failed_t) -> *mut c_char {
        sym!(self, xcb_setup_failed_reason)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_failed_reason` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_failed_reason(&self) -> bool {
        has_sym!(self, xcb_setup_failed_reason)
    }

    /// Returns the number of elements of the `reason` field of a `xcb_setup_failed_t` struct.
    #[inline]
    pub unsafe fn xcb_setup_failed_reason_length(&self, r: *const xcb_setup_failed_t) -> c_int {
        sym!(self, xcb_setup_failed_reason_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_failed_reason_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_failed_reason_length(&self) -> bool {
        has_sym!(self, xcb_setup_failed_reason_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `reason` field of a `xcb_setup_failed_t` struct.
    #[inline]
    pub unsafe fn xcb_setup_failed_reason_end(
        &self,
        r: *const xcb_setup_failed_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_setup_failed_reason_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_failed_reason_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_failed_reason_end(&self) -> bool {
        has_sym!(self, xcb_setup_failed_reason_end)
    }

    /// Advances a `xcb_setup_failed_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_setup_failed_next(&self, i: *mut xcb_setup_failed_iterator_t) {
        sym!(self, xcb_setup_failed_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_setup_failed_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_failed_next(&self) -> bool {
        has_sym!(self, xcb_setup_failed_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_setup_failed_iterator_t`.
    #[inline]
    pub unsafe fn xcb_setup_failed_end(
        &self,
        i: xcb_setup_failed_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_setup_failed_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_setup_failed_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_failed_end(&self) -> bool {
        has_sym!(self, xcb_setup_failed_end)
    }

    /// Computes the size of a `xcb_setup_authenticate_t` object.
    #[inline]
    pub unsafe fn xcb_setup_authenticate_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_setup_authenticate_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_setup_authenticate_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_authenticate_sizeof(&self) -> bool {
        has_sym!(self, xcb_setup_authenticate_sizeof)
    }

    /// Returns a pointer to the `reason` field of a `xcb_setup_authenticate_t` struct.
    #[inline]
    pub unsafe fn xcb_setup_authenticate_reason(
        &self,
        r: *const xcb_setup_authenticate_t,
    ) -> *mut c_char {
        sym!(self, xcb_setup_authenticate_reason)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_authenticate_reason` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_authenticate_reason(&self) -> bool {
        has_sym!(self, xcb_setup_authenticate_reason)
    }

    /// Returns the number of elements of the `reason` field of a `xcb_setup_authenticate_t` struct.
    #[inline]
    pub unsafe fn xcb_setup_authenticate_reason_length(
        &self,
        r: *const xcb_setup_authenticate_t,
    ) -> c_int {
        sym!(self, xcb_setup_authenticate_reason_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_authenticate_reason_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_authenticate_reason_length(&self) -> bool {
        has_sym!(self, xcb_setup_authenticate_reason_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `reason` field of a `xcb_setup_authenticate_t` struct.
    #[inline]
    pub unsafe fn xcb_setup_authenticate_reason_end(
        &self,
        r: *const xcb_setup_authenticate_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_setup_authenticate_reason_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_authenticate_reason_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_authenticate_reason_end(&self) -> bool {
        has_sym!(self, xcb_setup_authenticate_reason_end)
    }

    /// Advances a `xcb_setup_authenticate_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_setup_authenticate_next(&self, i: *mut xcb_setup_authenticate_iterator_t) {
        sym!(self, xcb_setup_authenticate_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_setup_authenticate_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_authenticate_next(&self) -> bool {
        has_sym!(self, xcb_setup_authenticate_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_setup_authenticate_iterator_t`.
    #[inline]
    pub unsafe fn xcb_setup_authenticate_end(
        &self,
        i: xcb_setup_authenticate_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_setup_authenticate_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_setup_authenticate_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_authenticate_end(&self) -> bool {
        has_sym!(self, xcb_setup_authenticate_end)
    }

    /// Computes the size of a `xcb_setup_t` object.
    #[inline]
    pub unsafe fn xcb_setup_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_setup_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_setup_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_sizeof(&self) -> bool {
        has_sym!(self, xcb_setup_sizeof)
    }

    /// Returns a pointer to the `vendor` field of a `xcb_setup_t` struct.
    #[inline]
    pub unsafe fn xcb_setup_vendor(&self, r: *const xcb_setup_t) -> *mut c_char {
        sym!(self, xcb_setup_vendor)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_vendor` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_vendor(&self) -> bool {
        has_sym!(self, xcb_setup_vendor)
    }

    /// Returns the number of elements of the `vendor` field of a `xcb_setup_t` struct.
    #[inline]
    pub unsafe fn xcb_setup_vendor_length(&self, r: *const xcb_setup_t) -> c_int {
        sym!(self, xcb_setup_vendor_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_vendor_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_vendor_length(&self) -> bool {
        has_sym!(self, xcb_setup_vendor_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `vendor` field of a `xcb_setup_t` struct.
    #[inline]
    pub unsafe fn xcb_setup_vendor_end(&self, r: *const xcb_setup_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_setup_vendor_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_vendor_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_vendor_end(&self) -> bool {
        has_sym!(self, xcb_setup_vendor_end)
    }

    /// Returns a pointer to the `pixmap_formats` field of a `xcb_setup_t` struct.
    #[inline]
    pub unsafe fn xcb_setup_pixmap_formats(&self, r: *const xcb_setup_t) -> *mut xcb_format_t {
        sym!(self, xcb_setup_pixmap_formats)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_pixmap_formats` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_pixmap_formats(&self) -> bool {
        has_sym!(self, xcb_setup_pixmap_formats)
    }

    /// Returns the number of elements of the `pixmap_formats` field of a `xcb_setup_t` struct.
    #[inline]
    pub unsafe fn xcb_setup_pixmap_formats_length(&self, r: *const xcb_setup_t) -> c_int {
        sym!(self, xcb_setup_pixmap_formats_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_pixmap_formats_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_pixmap_formats_length(&self) -> bool {
        has_sym!(self, xcb_setup_pixmap_formats_length)
    }

    /// Returns an iterator over the elements of the
    /// `pixmap_formats` field of a `xcb_setup_t` struct.
    #[inline]
    pub unsafe fn xcb_setup_pixmap_formats_iterator(
        &self,
        r: *const xcb_setup_t,
    ) -> xcb_format_iterator_t {
        sym!(self, xcb_setup_pixmap_formats_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_pixmap_formats_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_pixmap_formats_iterator(&self) -> bool {
        has_sym!(self, xcb_setup_pixmap_formats_iterator)
    }

    /// Returns the number of elements of the `roots` field of a `xcb_setup_t` struct.
    #[inline]
    pub unsafe fn xcb_setup_roots_length(&self, r: *const xcb_setup_t) -> c_int {
        sym!(self, xcb_setup_roots_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_roots_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_roots_length(&self) -> bool {
        has_sym!(self, xcb_setup_roots_length)
    }

    /// Returns an iterator over the elements of the
    /// `roots` field of a `xcb_setup_t` struct.
    #[inline]
    pub unsafe fn xcb_setup_roots_iterator(&self, r: *const xcb_setup_t) -> xcb_screen_iterator_t {
        sym!(self, xcb_setup_roots_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_setup_roots_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_roots_iterator(&self) -> bool {
        has_sym!(self, xcb_setup_roots_iterator)
    }

    /// Advances a `xcb_setup_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_setup_next(&self, i: *mut xcb_setup_iterator_t) {
        sym!(self, xcb_setup_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_setup_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_next(&self) -> bool {
        has_sym!(self, xcb_setup_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_setup_iterator_t`.
    #[inline]
    pub unsafe fn xcb_setup_end(&self, i: xcb_setup_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_setup_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_setup_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_setup_end(&self) -> bool {
        has_sym!(self, xcb_setup_end)
    }

    /// Advances a `xcb_client_message_data_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_client_message_data_next(&self, i: *mut xcb_client_message_data_iterator_t) {
        sym!(self, xcb_client_message_data_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_client_message_data_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_client_message_data_next(&self) -> bool {
        has_sym!(self, xcb_client_message_data_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_client_message_data_iterator_t`.
    #[inline]
    pub unsafe fn xcb_client_message_data_end(
        &self,
        i: xcb_client_message_data_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_client_message_data_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_client_message_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_client_message_data_end(&self) -> bool {
        has_sym!(self, xcb_client_message_data_end)
    }

    /// Serializes a `xcb_create_window_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_create_window_value_list_serialize(
        &self,
        _buffer: *mut *mut c_void,
        value_mask: u32,
        _aux: *const xcb_create_window_value_list_t,
    ) -> c_int {
        sym!(self, xcb_create_window_value_list_serialize)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_create_window_value_list_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_window_value_list_serialize(&self) -> bool {
        has_sym!(self, xcb_create_window_value_list_serialize)
    }

    /// Unpacks a `xcb_create_window_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_create_window_value_list_unpack(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
        _aux: *mut xcb_create_window_value_list_t,
    ) -> c_int {
        sym!(self, xcb_create_window_value_list_unpack)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_create_window_value_list_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_window_value_list_unpack(&self) -> bool {
        has_sym!(self, xcb_create_window_value_list_unpack)
    }

    /// Computes the size of a `xcb_create_window_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_create_window_value_list_sizeof(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
    ) -> c_int {
        sym!(self, xcb_create_window_value_list_sizeof)(_buffer, value_mask)
    }

    /// Returns `true` iff the symbol `xcb_create_window_value_list_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_window_value_list_sizeof(&self) -> bool {
        has_sym!(self, xcb_create_window_value_list_sizeof)
    }

    /// Computes the size of a `xcb_create_window_request_t` object.
    #[inline]
    pub unsafe fn xcb_create_window_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_create_window_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_create_window_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_window_sizeof(&self) -> bool {
        has_sym!(self, xcb_create_window_sizeof)
    }

    /// Sends a `CreateWindow` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    ///
    /// There is an auxiliary version of this function: [`xcb_create_window_aux_checked`].
    ///
    /// [`xcb_create_window_aux_checked`]: Self::xcb_create_window_aux_checked
    #[inline]
    pub unsafe fn xcb_create_window_checked(
        &self,
        c: *mut xcb_connection_t,
        depth: u8,
        wid: xcb_window_t,
        parent: xcb_window_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        class: u16,
        visual: xcb_visualid_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_window_checked)(
            c,
            depth,
            wid,
            parent,
            x,
            y,
            width,
            height,
            border_width,
            class,
            visual,
            value_mask,
            value_list,
        )
    }

    /// Returns `true` iff the symbol `xcb_create_window_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_window_checked(&self) -> bool {
        has_sym!(self, xcb_create_window_checked)
    }

    /// Sends a `CreateWindow` request (unchecked).
    ///
    /// There is an auxiliary version of this function: [`xcb_create_window_aux`].
    ///
    /// [`xcb_create_window_aux`]: Self::xcb_create_window_aux
    #[inline]
    pub unsafe fn xcb_create_window(
        &self,
        c: *mut xcb_connection_t,
        depth: u8,
        wid: xcb_window_t,
        parent: xcb_window_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        class: u16,
        visual: xcb_visualid_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_window)(
            c,
            depth,
            wid,
            parent,
            x,
            y,
            width,
            height,
            border_width,
            class,
            visual,
            value_mask,
            value_list,
        )
    }

    /// Returns `true` iff the symbol `xcb_create_window` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_window(&self) -> bool {
        has_sym!(self, xcb_create_window)
    }

    /// Sends a `CreateWindow` request (checked) (aux).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_create_window_aux_checked(
        &self,
        c: *mut xcb_connection_t,
        depth: u8,
        wid: xcb_window_t,
        parent: xcb_window_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        class: u16,
        visual: xcb_visualid_t,
        value_mask: u32,
        value_list: *const xcb_create_window_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_window_aux_checked)(
            c,
            depth,
            wid,
            parent,
            x,
            y,
            width,
            height,
            border_width,
            class,
            visual,
            value_mask,
            value_list,
        )
    }

    /// Returns `true` iff the symbol `xcb_create_window_aux_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_window_aux_checked(&self) -> bool {
        has_sym!(self, xcb_create_window_aux_checked)
    }

    /// Sends a `CreateWindow` request (unchecked) (aux).
    #[inline]
    pub unsafe fn xcb_create_window_aux(
        &self,
        c: *mut xcb_connection_t,
        depth: u8,
        wid: xcb_window_t,
        parent: xcb_window_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        class: u16,
        visual: xcb_visualid_t,
        value_mask: u32,
        value_list: *const xcb_create_window_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_window_aux)(
            c,
            depth,
            wid,
            parent,
            x,
            y,
            width,
            height,
            border_width,
            class,
            visual,
            value_mask,
            value_list,
        )
    }

    /// Returns `true` iff the symbol `xcb_create_window_aux` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_window_aux(&self) -> bool {
        has_sym!(self, xcb_create_window_aux)
    }

    /// Returns a pointer to the `value_list` field of a `xcb_create_window_request_t` struct.
    #[inline]
    pub unsafe fn xcb_create_window_value_list(
        &self,
        r: *const xcb_create_window_request_t,
    ) -> *mut c_void {
        sym!(self, xcb_create_window_value_list)(r)
    }

    /// Returns `true` iff the symbol `xcb_create_window_value_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_window_value_list(&self) -> bool {
        has_sym!(self, xcb_create_window_value_list)
    }

    /// Serializes a `xcb_change_window_attributes_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_change_window_attributes_value_list_serialize(
        &self,
        _buffer: *mut *mut c_void,
        value_mask: u32,
        _aux: *const xcb_change_window_attributes_value_list_t,
    ) -> c_int {
        sym!(self, xcb_change_window_attributes_value_list_serialize)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_change_window_attributes_value_list_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_window_attributes_value_list_serialize(&self) -> bool {
        has_sym!(self, xcb_change_window_attributes_value_list_serialize)
    }

    /// Unpacks a `xcb_change_window_attributes_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_change_window_attributes_value_list_unpack(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
        _aux: *mut xcb_change_window_attributes_value_list_t,
    ) -> c_int {
        sym!(self, xcb_change_window_attributes_value_list_unpack)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_change_window_attributes_value_list_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_window_attributes_value_list_unpack(&self) -> bool {
        has_sym!(self, xcb_change_window_attributes_value_list_unpack)
    }

    /// Computes the size of a `xcb_change_window_attributes_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_change_window_attributes_value_list_sizeof(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
    ) -> c_int {
        sym!(self, xcb_change_window_attributes_value_list_sizeof)(_buffer, value_mask)
    }

    /// Returns `true` iff the symbol `xcb_change_window_attributes_value_list_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_window_attributes_value_list_sizeof(&self) -> bool {
        has_sym!(self, xcb_change_window_attributes_value_list_sizeof)
    }

    /// Computes the size of a `xcb_change_window_attributes_request_t` object.
    #[inline]
    pub unsafe fn xcb_change_window_attributes_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_change_window_attributes_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_change_window_attributes_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_window_attributes_sizeof(&self) -> bool {
        has_sym!(self, xcb_change_window_attributes_sizeof)
    }

    /// Sends a `ChangeWindowAttributes` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    ///
    /// There is an auxiliary version of this function: [`xcb_change_window_attributes_aux_checked`].
    ///
    /// [`xcb_change_window_attributes_aux_checked`]: Self::xcb_change_window_attributes_aux_checked
    #[inline]
    pub unsafe fn xcb_change_window_attributes_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_window_attributes_checked)(c, window, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_change_window_attributes_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_window_attributes_checked(&self) -> bool {
        has_sym!(self, xcb_change_window_attributes_checked)
    }

    /// Sends a `ChangeWindowAttributes` request (unchecked).
    ///
    /// There is an auxiliary version of this function: [`xcb_change_window_attributes_aux`].
    ///
    /// [`xcb_change_window_attributes_aux`]: Self::xcb_change_window_attributes_aux
    #[inline]
    pub unsafe fn xcb_change_window_attributes(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_window_attributes)(c, window, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_change_window_attributes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_window_attributes(&self) -> bool {
        has_sym!(self, xcb_change_window_attributes)
    }

    /// Sends a `ChangeWindowAttributes` request (checked) (aux).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_change_window_attributes_aux_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u32,
        value_list: *const xcb_change_window_attributes_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_window_attributes_aux_checked)(c, window, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_change_window_attributes_aux_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_window_attributes_aux_checked(&self) -> bool {
        has_sym!(self, xcb_change_window_attributes_aux_checked)
    }

    /// Sends a `ChangeWindowAttributes` request (unchecked) (aux).
    #[inline]
    pub unsafe fn xcb_change_window_attributes_aux(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u32,
        value_list: *const xcb_change_window_attributes_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_window_attributes_aux)(c, window, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_change_window_attributes_aux` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_window_attributes_aux(&self) -> bool {
        has_sym!(self, xcb_change_window_attributes_aux)
    }

    /// Returns a pointer to the `value_list` field of a `xcb_change_window_attributes_request_t` struct.
    #[inline]
    pub unsafe fn xcb_change_window_attributes_value_list(
        &self,
        r: *const xcb_change_window_attributes_request_t,
    ) -> *mut c_void {
        sym!(self, xcb_change_window_attributes_value_list)(r)
    }

    /// Returns `true` iff the symbol `xcb_change_window_attributes_value_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_window_attributes_value_list(&self) -> bool {
        has_sym!(self, xcb_change_window_attributes_value_list)
    }

    /// Sends a `GetWindowAttributes` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_window_attributes_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_window_attributes_reply`]: Self::xcb_get_window_attributes_reply
    #[inline]
    pub unsafe fn xcb_get_window_attributes(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_get_window_attributes_cookie_t {
        sym!(self, xcb_get_window_attributes)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_get_window_attributes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_window_attributes(&self) -> bool {
        has_sym!(self, xcb_get_window_attributes)
    }

    /// Sends a `GetWindowAttributes` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_window_attributes_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_window_attributes_reply`]: Self::xcb_get_window_attributes_reply
    #[inline]
    pub unsafe fn xcb_get_window_attributes_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_get_window_attributes_cookie_t {
        sym!(self, xcb_get_window_attributes_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_get_window_attributes_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_window_attributes_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_window_attributes_unchecked)
    }

    /// Waits for the reply to a `GetWindowAttributes` request.
    #[inline]
    pub unsafe fn xcb_get_window_attributes_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_window_attributes_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_window_attributes_reply_t {
        sym!(self, xcb_get_window_attributes_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_window_attributes_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_window_attributes_reply(&self) -> bool {
        has_sym!(self, xcb_get_window_attributes_reply)
    }

    /// Sends a `DestroyWindow` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_destroy_window_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_destroy_window_checked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_destroy_window_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_destroy_window_checked(&self) -> bool {
        has_sym!(self, xcb_destroy_window_checked)
    }

    /// Sends a `DestroyWindow` request (unchecked).
    #[inline]
    pub unsafe fn xcb_destroy_window(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_destroy_window)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_destroy_window` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_destroy_window(&self) -> bool {
        has_sym!(self, xcb_destroy_window)
    }

    /// Sends a `DestroySubwindows` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_destroy_subwindows_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_destroy_subwindows_checked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_destroy_subwindows_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_destroy_subwindows_checked(&self) -> bool {
        has_sym!(self, xcb_destroy_subwindows_checked)
    }

    /// Sends a `DestroySubwindows` request (unchecked).
    #[inline]
    pub unsafe fn xcb_destroy_subwindows(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_destroy_subwindows)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_destroy_subwindows` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_destroy_subwindows(&self) -> bool {
        has_sym!(self, xcb_destroy_subwindows)
    }

    /// Sends a `ChangeSaveSet` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_change_save_set_checked(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_save_set_checked)(c, mode, window)
    }

    /// Returns `true` iff the symbol `xcb_change_save_set_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_save_set_checked(&self) -> bool {
        has_sym!(self, xcb_change_save_set_checked)
    }

    /// Sends a `ChangeSaveSet` request (unchecked).
    #[inline]
    pub unsafe fn xcb_change_save_set(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_save_set)(c, mode, window)
    }

    /// Returns `true` iff the symbol `xcb_change_save_set` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_save_set(&self) -> bool {
        has_sym!(self, xcb_change_save_set)
    }

    /// Sends a `ReparentWindow` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_reparent_window_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        parent: xcb_window_t,
        x: i16,
        y: i16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_reparent_window_checked)(c, window, parent, x, y)
    }

    /// Returns `true` iff the symbol `xcb_reparent_window_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_reparent_window_checked(&self) -> bool {
        has_sym!(self, xcb_reparent_window_checked)
    }

    /// Sends a `ReparentWindow` request (unchecked).
    #[inline]
    pub unsafe fn xcb_reparent_window(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        parent: xcb_window_t,
        x: i16,
        y: i16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_reparent_window)(c, window, parent, x, y)
    }

    /// Returns `true` iff the symbol `xcb_reparent_window` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_reparent_window(&self) -> bool {
        has_sym!(self, xcb_reparent_window)
    }

    /// Sends a `MapWindow` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_map_window_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_map_window_checked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_map_window_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_map_window_checked(&self) -> bool {
        has_sym!(self, xcb_map_window_checked)
    }

    /// Sends a `MapWindow` request (unchecked).
    #[inline]
    pub unsafe fn xcb_map_window(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_map_window)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_map_window` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_map_window(&self) -> bool {
        has_sym!(self, xcb_map_window)
    }

    /// Sends a `MapSubwindows` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_map_subwindows_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_map_subwindows_checked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_map_subwindows_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_map_subwindows_checked(&self) -> bool {
        has_sym!(self, xcb_map_subwindows_checked)
    }

    /// Sends a `MapSubwindows` request (unchecked).
    #[inline]
    pub unsafe fn xcb_map_subwindows(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_map_subwindows)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_map_subwindows` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_map_subwindows(&self) -> bool {
        has_sym!(self, xcb_map_subwindows)
    }

    /// Sends a `UnmapWindow` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_unmap_window_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_unmap_window_checked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_unmap_window_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_unmap_window_checked(&self) -> bool {
        has_sym!(self, xcb_unmap_window_checked)
    }

    /// Sends a `UnmapWindow` request (unchecked).
    #[inline]
    pub unsafe fn xcb_unmap_window(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_unmap_window)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_unmap_window` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_unmap_window(&self) -> bool {
        has_sym!(self, xcb_unmap_window)
    }

    /// Sends a `UnmapSubwindows` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_unmap_subwindows_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_unmap_subwindows_checked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_unmap_subwindows_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_unmap_subwindows_checked(&self) -> bool {
        has_sym!(self, xcb_unmap_subwindows_checked)
    }

    /// Sends a `UnmapSubwindows` request (unchecked).
    #[inline]
    pub unsafe fn xcb_unmap_subwindows(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_unmap_subwindows)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_unmap_subwindows` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_unmap_subwindows(&self) -> bool {
        has_sym!(self, xcb_unmap_subwindows)
    }

    /// Serializes a `xcb_configure_window_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_configure_window_value_list_serialize(
        &self,
        _buffer: *mut *mut c_void,
        value_mask: u16,
        _aux: *const xcb_configure_window_value_list_t,
    ) -> c_int {
        sym!(self, xcb_configure_window_value_list_serialize)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_configure_window_value_list_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_configure_window_value_list_serialize(&self) -> bool {
        has_sym!(self, xcb_configure_window_value_list_serialize)
    }

    /// Unpacks a `xcb_configure_window_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_configure_window_value_list_unpack(
        &self,
        _buffer: *const c_void,
        value_mask: u16,
        _aux: *mut xcb_configure_window_value_list_t,
    ) -> c_int {
        sym!(self, xcb_configure_window_value_list_unpack)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_configure_window_value_list_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_configure_window_value_list_unpack(&self) -> bool {
        has_sym!(self, xcb_configure_window_value_list_unpack)
    }

    /// Computes the size of a `xcb_configure_window_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_configure_window_value_list_sizeof(
        &self,
        _buffer: *const c_void,
        value_mask: u16,
    ) -> c_int {
        sym!(self, xcb_configure_window_value_list_sizeof)(_buffer, value_mask)
    }

    /// Returns `true` iff the symbol `xcb_configure_window_value_list_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_configure_window_value_list_sizeof(&self) -> bool {
        has_sym!(self, xcb_configure_window_value_list_sizeof)
    }

    /// Computes the size of a `xcb_configure_window_request_t` object.
    #[inline]
    pub unsafe fn xcb_configure_window_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_configure_window_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_configure_window_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_configure_window_sizeof(&self) -> bool {
        has_sym!(self, xcb_configure_window_sizeof)
    }

    /// Sends a `ConfigureWindow` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    ///
    /// There is an auxiliary version of this function: [`xcb_configure_window_aux_checked`].
    ///
    /// [`xcb_configure_window_aux_checked`]: Self::xcb_configure_window_aux_checked
    #[inline]
    pub unsafe fn xcb_configure_window_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u16,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_configure_window_checked)(c, window, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_configure_window_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_configure_window_checked(&self) -> bool {
        has_sym!(self, xcb_configure_window_checked)
    }

    /// Sends a `ConfigureWindow` request (unchecked).
    ///
    /// There is an auxiliary version of this function: [`xcb_configure_window_aux`].
    ///
    /// [`xcb_configure_window_aux`]: Self::xcb_configure_window_aux
    #[inline]
    pub unsafe fn xcb_configure_window(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u16,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_configure_window)(c, window, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_configure_window` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_configure_window(&self) -> bool {
        has_sym!(self, xcb_configure_window)
    }

    /// Sends a `ConfigureWindow` request (checked) (aux).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_configure_window_aux_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u16,
        value_list: *const xcb_configure_window_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_configure_window_aux_checked)(c, window, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_configure_window_aux_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_configure_window_aux_checked(&self) -> bool {
        has_sym!(self, xcb_configure_window_aux_checked)
    }

    /// Sends a `ConfigureWindow` request (unchecked) (aux).
    #[inline]
    pub unsafe fn xcb_configure_window_aux(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u16,
        value_list: *const xcb_configure_window_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_configure_window_aux)(c, window, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_configure_window_aux` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_configure_window_aux(&self) -> bool {
        has_sym!(self, xcb_configure_window_aux)
    }

    /// Returns a pointer to the `value_list` field of a `xcb_configure_window_request_t` struct.
    #[inline]
    pub unsafe fn xcb_configure_window_value_list(
        &self,
        r: *const xcb_configure_window_request_t,
    ) -> *mut c_void {
        sym!(self, xcb_configure_window_value_list)(r)
    }

    /// Returns `true` iff the symbol `xcb_configure_window_value_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_configure_window_value_list(&self) -> bool {
        has_sym!(self, xcb_configure_window_value_list)
    }

    /// Sends a `CirculateWindow` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_circulate_window_checked(
        &self,
        c: *mut xcb_connection_t,
        direction: u8,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_circulate_window_checked)(c, direction, window)
    }

    /// Returns `true` iff the symbol `xcb_circulate_window_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_circulate_window_checked(&self) -> bool {
        has_sym!(self, xcb_circulate_window_checked)
    }

    /// Sends a `CirculateWindow` request (unchecked).
    #[inline]
    pub unsafe fn xcb_circulate_window(
        &self,
        c: *mut xcb_connection_t,
        direction: u8,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_circulate_window)(c, direction, window)
    }

    /// Returns `true` iff the symbol `xcb_circulate_window` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_circulate_window(&self) -> bool {
        has_sym!(self, xcb_circulate_window)
    }

    /// Sends a `GetGeometry` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_geometry_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_geometry_reply`]: Self::xcb_get_geometry_reply
    #[inline]
    pub unsafe fn xcb_get_geometry(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_get_geometry_cookie_t {
        sym!(self, xcb_get_geometry)(c, drawable)
    }

    /// Returns `true` iff the symbol `xcb_get_geometry` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_geometry(&self) -> bool {
        has_sym!(self, xcb_get_geometry)
    }

    /// Sends a `GetGeometry` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_geometry_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_geometry_reply`]: Self::xcb_get_geometry_reply
    #[inline]
    pub unsafe fn xcb_get_geometry_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_get_geometry_cookie_t {
        sym!(self, xcb_get_geometry_unchecked)(c, drawable)
    }

    /// Returns `true` iff the symbol `xcb_get_geometry_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_geometry_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_geometry_unchecked)
    }

    /// Waits for the reply to a `GetGeometry` request.
    #[inline]
    pub unsafe fn xcb_get_geometry_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_geometry_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_geometry_reply_t {
        sym!(self, xcb_get_geometry_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_geometry_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_geometry_reply(&self) -> bool {
        has_sym!(self, xcb_get_geometry_reply)
    }

    /// Computes the size of a `xcb_query_tree_reply_t` object.
    #[inline]
    pub unsafe fn xcb_query_tree_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_query_tree_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_query_tree_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_tree_sizeof(&self) -> bool {
        has_sym!(self, xcb_query_tree_sizeof)
    }

    /// Sends a `QueryTree` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_query_tree_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_query_tree_reply`]: Self::xcb_query_tree_reply
    #[inline]
    pub unsafe fn xcb_query_tree(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_query_tree_cookie_t {
        sym!(self, xcb_query_tree)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_query_tree` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_tree(&self) -> bool {
        has_sym!(self, xcb_query_tree)
    }

    /// Sends a `QueryTree` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_query_tree_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_query_tree_reply`]: Self::xcb_query_tree_reply
    #[inline]
    pub unsafe fn xcb_query_tree_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_query_tree_cookie_t {
        sym!(self, xcb_query_tree_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_query_tree_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_tree_unchecked(&self) -> bool {
        has_sym!(self, xcb_query_tree_unchecked)
    }

    /// Returns a pointer to the `children` field of a `xcb_query_tree_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_query_tree_children(
        &self,
        r: *const xcb_query_tree_reply_t,
    ) -> *mut xcb_window_t {
        sym!(self, xcb_query_tree_children)(r)
    }

    /// Returns `true` iff the symbol `xcb_query_tree_children` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_tree_children(&self) -> bool {
        has_sym!(self, xcb_query_tree_children)
    }

    /// Returns the number of elements of the `children` field of a `xcb_query_tree_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_query_tree_children_length(&self, r: *const xcb_query_tree_reply_t) -> c_int {
        sym!(self, xcb_query_tree_children_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_query_tree_children_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_tree_children_length(&self) -> bool {
        has_sym!(self, xcb_query_tree_children_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `children` field of a `xcb_query_tree_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_query_tree_children_end(
        &self,
        r: *const xcb_query_tree_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_query_tree_children_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_query_tree_children_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_tree_children_end(&self) -> bool {
        has_sym!(self, xcb_query_tree_children_end)
    }

    /// Waits for the reply to a `QueryTree` request.
    #[inline]
    pub unsafe fn xcb_query_tree_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_query_tree_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_tree_reply_t {
        sym!(self, xcb_query_tree_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_query_tree_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_tree_reply(&self) -> bool {
        has_sym!(self, xcb_query_tree_reply)
    }

    /// Computes the size of a `xcb_intern_atom_request_t` object.
    #[inline]
    pub unsafe fn xcb_intern_atom_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_intern_atom_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_intern_atom_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_intern_atom_sizeof(&self) -> bool {
        has_sym!(self, xcb_intern_atom_sizeof)
    }

    /// Sends a `InternAtom` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_intern_atom_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_intern_atom_reply`]: Self::xcb_intern_atom_reply
    #[inline]
    pub unsafe fn xcb_intern_atom(
        &self,
        c: *mut xcb_connection_t,
        only_if_exists: u8,
        name_len: u16,
        name: *const c_char,
    ) -> xcb_intern_atom_cookie_t {
        sym!(self, xcb_intern_atom)(c, only_if_exists, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_intern_atom` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_intern_atom(&self) -> bool {
        has_sym!(self, xcb_intern_atom)
    }

    /// Sends a `InternAtom` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_intern_atom_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_intern_atom_reply`]: Self::xcb_intern_atom_reply
    #[inline]
    pub unsafe fn xcb_intern_atom_unchecked(
        &self,
        c: *mut xcb_connection_t,
        only_if_exists: u8,
        name_len: u16,
        name: *const c_char,
    ) -> xcb_intern_atom_cookie_t {
        sym!(self, xcb_intern_atom_unchecked)(c, only_if_exists, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_intern_atom_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_intern_atom_unchecked(&self) -> bool {
        has_sym!(self, xcb_intern_atom_unchecked)
    }

    /// Waits for the reply to a `InternAtom` request.
    #[inline]
    pub unsafe fn xcb_intern_atom_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_intern_atom_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_intern_atom_reply_t {
        sym!(self, xcb_intern_atom_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_intern_atom_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_intern_atom_reply(&self) -> bool {
        has_sym!(self, xcb_intern_atom_reply)
    }

    /// Computes the size of a `xcb_get_atom_name_reply_t` object.
    #[inline]
    pub unsafe fn xcb_get_atom_name_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_get_atom_name_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_get_atom_name_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_atom_name_sizeof(&self) -> bool {
        has_sym!(self, xcb_get_atom_name_sizeof)
    }

    /// Sends a `GetAtomName` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_atom_name_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_atom_name_reply`]: Self::xcb_get_atom_name_reply
    #[inline]
    pub unsafe fn xcb_get_atom_name(
        &self,
        c: *mut xcb_connection_t,
        atom: xcb_atom_t,
    ) -> xcb_get_atom_name_cookie_t {
        sym!(self, xcb_get_atom_name)(c, atom)
    }

    /// Returns `true` iff the symbol `xcb_get_atom_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_atom_name(&self) -> bool {
        has_sym!(self, xcb_get_atom_name)
    }

    /// Sends a `GetAtomName` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_atom_name_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_atom_name_reply`]: Self::xcb_get_atom_name_reply
    #[inline]
    pub unsafe fn xcb_get_atom_name_unchecked(
        &self,
        c: *mut xcb_connection_t,
        atom: xcb_atom_t,
    ) -> xcb_get_atom_name_cookie_t {
        sym!(self, xcb_get_atom_name_unchecked)(c, atom)
    }

    /// Returns `true` iff the symbol `xcb_get_atom_name_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_atom_name_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_atom_name_unchecked)
    }

    /// Returns a pointer to the `name` field of a `xcb_get_atom_name_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_get_atom_name_name(
        &self,
        r: *const xcb_get_atom_name_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_get_atom_name_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_atom_name_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_atom_name_name(&self) -> bool {
        has_sym!(self, xcb_get_atom_name_name)
    }

    /// Returns the number of elements of the `name` field of a `xcb_get_atom_name_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_get_atom_name_name_length(
        &self,
        r: *const xcb_get_atom_name_reply_t,
    ) -> c_int {
        sym!(self, xcb_get_atom_name_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_atom_name_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_atom_name_name_length(&self) -> bool {
        has_sym!(self, xcb_get_atom_name_name_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `name` field of a `xcb_get_atom_name_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_get_atom_name_name_end(
        &self,
        r: *const xcb_get_atom_name_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_get_atom_name_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_atom_name_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_atom_name_name_end(&self) -> bool {
        has_sym!(self, xcb_get_atom_name_name_end)
    }

    /// Waits for the reply to a `GetAtomName` request.
    #[inline]
    pub unsafe fn xcb_get_atom_name_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_atom_name_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_atom_name_reply_t {
        sym!(self, xcb_get_atom_name_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_atom_name_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_atom_name_reply(&self) -> bool {
        has_sym!(self, xcb_get_atom_name_reply)
    }

    /// Computes the size of a `xcb_change_property_request_t` object.
    #[inline]
    pub unsafe fn xcb_change_property_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_change_property_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_change_property_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_property_sizeof(&self) -> bool {
        has_sym!(self, xcb_change_property_sizeof)
    }

    /// Sends a `ChangeProperty` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_change_property_checked(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
        window: xcb_window_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        format: u8,
        data_len: u32,
        data: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_property_checked)(
            c, mode, window, property, type_, format, data_len, data,
        )
    }

    /// Returns `true` iff the symbol `xcb_change_property_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_property_checked(&self) -> bool {
        has_sym!(self, xcb_change_property_checked)
    }

    /// Sends a `ChangeProperty` request (unchecked).
    #[inline]
    pub unsafe fn xcb_change_property(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
        window: xcb_window_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        format: u8,
        data_len: u32,
        data: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_property)(c, mode, window, property, type_, format, data_len, data)
    }

    /// Returns `true` iff the symbol `xcb_change_property` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_property(&self) -> bool {
        has_sym!(self, xcb_change_property)
    }

    /// Returns a pointer to the `data` field of a `xcb_change_property_request_t` struct.
    #[inline]
    pub unsafe fn xcb_change_property_data(
        &self,
        r: *const xcb_change_property_request_t,
    ) -> *mut c_void {
        sym!(self, xcb_change_property_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_change_property_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_property_data(&self) -> bool {
        has_sym!(self, xcb_change_property_data)
    }

    /// Returns the number of elements of the `data` field of a `xcb_change_property_request_t` struct.
    #[inline]
    pub unsafe fn xcb_change_property_data_length(
        &self,
        r: *const xcb_change_property_request_t,
    ) -> c_int {
        sym!(self, xcb_change_property_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_change_property_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_property_data_length(&self) -> bool {
        has_sym!(self, xcb_change_property_data_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `data` field of a `xcb_change_property_request_t` struct.
    #[inline]
    pub unsafe fn xcb_change_property_data_end(
        &self,
        r: *const xcb_change_property_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_change_property_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_change_property_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_property_data_end(&self) -> bool {
        has_sym!(self, xcb_change_property_data_end)
    }

    /// Sends a `DeleteProperty` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_delete_property_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        property: xcb_atom_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_delete_property_checked)(c, window, property)
    }

    /// Returns `true` iff the symbol `xcb_delete_property_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_delete_property_checked(&self) -> bool {
        has_sym!(self, xcb_delete_property_checked)
    }

    /// Sends a `DeleteProperty` request (unchecked).
    #[inline]
    pub unsafe fn xcb_delete_property(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        property: xcb_atom_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_delete_property)(c, window, property)
    }

    /// Returns `true` iff the symbol `xcb_delete_property` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_delete_property(&self) -> bool {
        has_sym!(self, xcb_delete_property)
    }

    /// Computes the size of a `xcb_get_property_reply_t` object.
    #[inline]
    pub unsafe fn xcb_get_property_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_get_property_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_get_property_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_property_sizeof(&self) -> bool {
        has_sym!(self, xcb_get_property_sizeof)
    }

    /// Sends a `GetProperty` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_property_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_property_reply`]: Self::xcb_get_property_reply
    #[inline]
    pub unsafe fn xcb_get_property(
        &self,
        c: *mut xcb_connection_t,
        delete: u8,
        window: xcb_window_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        long_offset: u32,
        long_length: u32,
    ) -> xcb_get_property_cookie_t {
        sym!(self, xcb_get_property)(c, delete, window, property, type_, long_offset, long_length)
    }

    /// Returns `true` iff the symbol `xcb_get_property` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_property(&self) -> bool {
        has_sym!(self, xcb_get_property)
    }

    /// Sends a `GetProperty` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_property_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_property_reply`]: Self::xcb_get_property_reply
    #[inline]
    pub unsafe fn xcb_get_property_unchecked(
        &self,
        c: *mut xcb_connection_t,
        delete: u8,
        window: xcb_window_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        long_offset: u32,
        long_length: u32,
    ) -> xcb_get_property_cookie_t {
        sym!(self, xcb_get_property_unchecked)(
            c,
            delete,
            window,
            property,
            type_,
            long_offset,
            long_length,
        )
    }

    /// Returns `true` iff the symbol `xcb_get_property_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_property_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_property_unchecked)
    }

    /// Returns a pointer to the `value` field of a `xcb_get_property_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_get_property_value(&self, r: *const xcb_get_property_reply_t) -> *mut c_void {
        sym!(self, xcb_get_property_value)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_property_value` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_property_value(&self) -> bool {
        has_sym!(self, xcb_get_property_value)
    }

    /// Returns the number of elements of the `value` field of a `xcb_get_property_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_get_property_value_length(
        &self,
        r: *const xcb_get_property_reply_t,
    ) -> c_int {
        sym!(self, xcb_get_property_value_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_property_value_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_property_value_length(&self) -> bool {
        has_sym!(self, xcb_get_property_value_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `value` field of a `xcb_get_property_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_get_property_value_end(
        &self,
        r: *const xcb_get_property_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_get_property_value_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_property_value_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_property_value_end(&self) -> bool {
        has_sym!(self, xcb_get_property_value_end)
    }

    /// Waits for the reply to a `GetProperty` request.
    #[inline]
    pub unsafe fn xcb_get_property_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_property_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_property_reply_t {
        sym!(self, xcb_get_property_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_property_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_property_reply(&self) -> bool {
        has_sym!(self, xcb_get_property_reply)
    }

    /// Computes the size of a `xcb_list_properties_reply_t` object.
    #[inline]
    pub unsafe fn xcb_list_properties_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_list_properties_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_list_properties_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_properties_sizeof(&self) -> bool {
        has_sym!(self, xcb_list_properties_sizeof)
    }

    /// Sends a `ListProperties` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_list_properties_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_list_properties_reply`]: Self::xcb_list_properties_reply
    #[inline]
    pub unsafe fn xcb_list_properties(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_list_properties_cookie_t {
        sym!(self, xcb_list_properties)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_list_properties` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_properties(&self) -> bool {
        has_sym!(self, xcb_list_properties)
    }

    /// Sends a `ListProperties` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_list_properties_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_list_properties_reply`]: Self::xcb_list_properties_reply
    #[inline]
    pub unsafe fn xcb_list_properties_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_list_properties_cookie_t {
        sym!(self, xcb_list_properties_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_list_properties_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_properties_unchecked(&self) -> bool {
        has_sym!(self, xcb_list_properties_unchecked)
    }

    /// Returns a pointer to the `atoms` field of a `xcb_list_properties_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_list_properties_atoms(
        &self,
        r: *const xcb_list_properties_reply_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_list_properties_atoms)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_properties_atoms` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_properties_atoms(&self) -> bool {
        has_sym!(self, xcb_list_properties_atoms)
    }

    /// Returns the number of elements of the `atoms` field of a `xcb_list_properties_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_list_properties_atoms_length(
        &self,
        r: *const xcb_list_properties_reply_t,
    ) -> c_int {
        sym!(self, xcb_list_properties_atoms_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_properties_atoms_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_properties_atoms_length(&self) -> bool {
        has_sym!(self, xcb_list_properties_atoms_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `atoms` field of a `xcb_list_properties_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_list_properties_atoms_end(
        &self,
        r: *const xcb_list_properties_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_list_properties_atoms_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_properties_atoms_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_properties_atoms_end(&self) -> bool {
        has_sym!(self, xcb_list_properties_atoms_end)
    }

    /// Waits for the reply to a `ListProperties` request.
    #[inline]
    pub unsafe fn xcb_list_properties_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_list_properties_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_list_properties_reply_t {
        sym!(self, xcb_list_properties_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_list_properties_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_properties_reply(&self) -> bool {
        has_sym!(self, xcb_list_properties_reply)
    }

    /// Sends a `SetSelectionOwner` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_set_selection_owner_checked(
        &self,
        c: *mut xcb_connection_t,
        owner: xcb_window_t,
        selection: xcb_atom_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_selection_owner_checked)(c, owner, selection, time)
    }

    /// Returns `true` iff the symbol `xcb_set_selection_owner_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_selection_owner_checked(&self) -> bool {
        has_sym!(self, xcb_set_selection_owner_checked)
    }

    /// Sends a `SetSelectionOwner` request (unchecked).
    #[inline]
    pub unsafe fn xcb_set_selection_owner(
        &self,
        c: *mut xcb_connection_t,
        owner: xcb_window_t,
        selection: xcb_atom_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_selection_owner)(c, owner, selection, time)
    }

    /// Returns `true` iff the symbol `xcb_set_selection_owner` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_selection_owner(&self) -> bool {
        has_sym!(self, xcb_set_selection_owner)
    }

    /// Sends a `GetSelectionOwner` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_selection_owner_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_selection_owner_reply`]: Self::xcb_get_selection_owner_reply
    #[inline]
    pub unsafe fn xcb_get_selection_owner(
        &self,
        c: *mut xcb_connection_t,
        selection: xcb_atom_t,
    ) -> xcb_get_selection_owner_cookie_t {
        sym!(self, xcb_get_selection_owner)(c, selection)
    }

    /// Returns `true` iff the symbol `xcb_get_selection_owner` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_selection_owner(&self) -> bool {
        has_sym!(self, xcb_get_selection_owner)
    }

    /// Sends a `GetSelectionOwner` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_selection_owner_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_selection_owner_reply`]: Self::xcb_get_selection_owner_reply
    #[inline]
    pub unsafe fn xcb_get_selection_owner_unchecked(
        &self,
        c: *mut xcb_connection_t,
        selection: xcb_atom_t,
    ) -> xcb_get_selection_owner_cookie_t {
        sym!(self, xcb_get_selection_owner_unchecked)(c, selection)
    }

    /// Returns `true` iff the symbol `xcb_get_selection_owner_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_selection_owner_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_selection_owner_unchecked)
    }

    /// Waits for the reply to a `GetSelectionOwner` request.
    #[inline]
    pub unsafe fn xcb_get_selection_owner_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_selection_owner_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_selection_owner_reply_t {
        sym!(self, xcb_get_selection_owner_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_selection_owner_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_selection_owner_reply(&self) -> bool {
        has_sym!(self, xcb_get_selection_owner_reply)
    }

    /// Sends a `ConvertSelection` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_convert_selection_checked(
        &self,
        c: *mut xcb_connection_t,
        requestor: xcb_window_t,
        selection: xcb_atom_t,
        target: xcb_atom_t,
        property: xcb_atom_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_convert_selection_checked)(c, requestor, selection, target, property, time)
    }

    /// Returns `true` iff the symbol `xcb_convert_selection_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_convert_selection_checked(&self) -> bool {
        has_sym!(self, xcb_convert_selection_checked)
    }

    /// Sends a `ConvertSelection` request (unchecked).
    #[inline]
    pub unsafe fn xcb_convert_selection(
        &self,
        c: *mut xcb_connection_t,
        requestor: xcb_window_t,
        selection: xcb_atom_t,
        target: xcb_atom_t,
        property: xcb_atom_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_convert_selection)(c, requestor, selection, target, property, time)
    }

    /// Returns `true` iff the symbol `xcb_convert_selection` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_convert_selection(&self) -> bool {
        has_sym!(self, xcb_convert_selection)
    }

    /// Sends a `SendEvent` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_send_event_checked(
        &self,
        c: *mut xcb_connection_t,
        propagate: u8,
        destination: xcb_window_t,
        event_mask: u32,
        event: *const [c_char; 32],
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_send_event_checked)(c, propagate, destination, event_mask, event)
    }

    /// Returns `true` iff the symbol `xcb_send_event_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_send_event_checked(&self) -> bool {
        has_sym!(self, xcb_send_event_checked)
    }

    /// Sends a `SendEvent` request (unchecked).
    #[inline]
    pub unsafe fn xcb_send_event(
        &self,
        c: *mut xcb_connection_t,
        propagate: u8,
        destination: xcb_window_t,
        event_mask: u32,
        event: *const [c_char; 32],
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_send_event)(c, propagate, destination, event_mask, event)
    }

    /// Returns `true` iff the symbol `xcb_send_event` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_send_event(&self) -> bool {
        has_sym!(self, xcb_send_event)
    }

    /// Sends a `GrabPointer` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_grab_pointer_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_grab_pointer_reply`]: Self::xcb_grab_pointer_reply
    #[inline]
    pub unsafe fn xcb_grab_pointer(
        &self,
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        event_mask: u16,
        pointer_mode: u8,
        keyboard_mode: u8,
        confine_to: xcb_window_t,
        cursor: xcb_cursor_t,
        time: xcb_timestamp_t,
    ) -> xcb_grab_pointer_cookie_t {
        sym!(self, xcb_grab_pointer)(
            c,
            owner_events,
            grab_window,
            event_mask,
            pointer_mode,
            keyboard_mode,
            confine_to,
            cursor,
            time,
        )
    }

    /// Returns `true` iff the symbol `xcb_grab_pointer` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_grab_pointer(&self) -> bool {
        has_sym!(self, xcb_grab_pointer)
    }

    /// Sends a `GrabPointer` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_grab_pointer_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_grab_pointer_reply`]: Self::xcb_grab_pointer_reply
    #[inline]
    pub unsafe fn xcb_grab_pointer_unchecked(
        &self,
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        event_mask: u16,
        pointer_mode: u8,
        keyboard_mode: u8,
        confine_to: xcb_window_t,
        cursor: xcb_cursor_t,
        time: xcb_timestamp_t,
    ) -> xcb_grab_pointer_cookie_t {
        sym!(self, xcb_grab_pointer_unchecked)(
            c,
            owner_events,
            grab_window,
            event_mask,
            pointer_mode,
            keyboard_mode,
            confine_to,
            cursor,
            time,
        )
    }

    /// Returns `true` iff the symbol `xcb_grab_pointer_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_grab_pointer_unchecked(&self) -> bool {
        has_sym!(self, xcb_grab_pointer_unchecked)
    }

    /// Waits for the reply to a `GrabPointer` request.
    #[inline]
    pub unsafe fn xcb_grab_pointer_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_grab_pointer_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_grab_pointer_reply_t {
        sym!(self, xcb_grab_pointer_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_grab_pointer_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_grab_pointer_reply(&self) -> bool {
        has_sym!(self, xcb_grab_pointer_reply)
    }

    /// Sends a `UngrabPointer` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_ungrab_pointer_checked(
        &self,
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_ungrab_pointer_checked)(c, time)
    }

    /// Returns `true` iff the symbol `xcb_ungrab_pointer_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_ungrab_pointer_checked(&self) -> bool {
        has_sym!(self, xcb_ungrab_pointer_checked)
    }

    /// Sends a `UngrabPointer` request (unchecked).
    #[inline]
    pub unsafe fn xcb_ungrab_pointer(
        &self,
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_ungrab_pointer)(c, time)
    }

    /// Returns `true` iff the symbol `xcb_ungrab_pointer` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_ungrab_pointer(&self) -> bool {
        has_sym!(self, xcb_ungrab_pointer)
    }

    /// Sends a `GrabButton` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_grab_button_checked(
        &self,
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        event_mask: u16,
        pointer_mode: u8,
        keyboard_mode: u8,
        confine_to: xcb_window_t,
        cursor: xcb_cursor_t,
        button: u8,
        modifiers: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_grab_button_checked)(
            c,
            owner_events,
            grab_window,
            event_mask,
            pointer_mode,
            keyboard_mode,
            confine_to,
            cursor,
            button,
            modifiers,
        )
    }

    /// Returns `true` iff the symbol `xcb_grab_button_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_grab_button_checked(&self) -> bool {
        has_sym!(self, xcb_grab_button_checked)
    }

    /// Sends a `GrabButton` request (unchecked).
    #[inline]
    pub unsafe fn xcb_grab_button(
        &self,
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        event_mask: u16,
        pointer_mode: u8,
        keyboard_mode: u8,
        confine_to: xcb_window_t,
        cursor: xcb_cursor_t,
        button: u8,
        modifiers: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_grab_button)(
            c,
            owner_events,
            grab_window,
            event_mask,
            pointer_mode,
            keyboard_mode,
            confine_to,
            cursor,
            button,
            modifiers,
        )
    }

    /// Returns `true` iff the symbol `xcb_grab_button` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_grab_button(&self) -> bool {
        has_sym!(self, xcb_grab_button)
    }

    /// Sends a `UngrabButton` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_ungrab_button_checked(
        &self,
        c: *mut xcb_connection_t,
        button: u8,
        grab_window: xcb_window_t,
        modifiers: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_ungrab_button_checked)(c, button, grab_window, modifiers)
    }

    /// Returns `true` iff the symbol `xcb_ungrab_button_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_ungrab_button_checked(&self) -> bool {
        has_sym!(self, xcb_ungrab_button_checked)
    }

    /// Sends a `UngrabButton` request (unchecked).
    #[inline]
    pub unsafe fn xcb_ungrab_button(
        &self,
        c: *mut xcb_connection_t,
        button: u8,
        grab_window: xcb_window_t,
        modifiers: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_ungrab_button)(c, button, grab_window, modifiers)
    }

    /// Returns `true` iff the symbol `xcb_ungrab_button` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_ungrab_button(&self) -> bool {
        has_sym!(self, xcb_ungrab_button)
    }

    /// Sends a `ChangeActivePointerGrab` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_change_active_pointer_grab_checked(
        &self,
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
        time: xcb_timestamp_t,
        event_mask: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_active_pointer_grab_checked)(c, cursor, time, event_mask)
    }

    /// Returns `true` iff the symbol `xcb_change_active_pointer_grab_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_active_pointer_grab_checked(&self) -> bool {
        has_sym!(self, xcb_change_active_pointer_grab_checked)
    }

    /// Sends a `ChangeActivePointerGrab` request (unchecked).
    #[inline]
    pub unsafe fn xcb_change_active_pointer_grab(
        &self,
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
        time: xcb_timestamp_t,
        event_mask: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_active_pointer_grab)(c, cursor, time, event_mask)
    }

    /// Returns `true` iff the symbol `xcb_change_active_pointer_grab` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_active_pointer_grab(&self) -> bool {
        has_sym!(self, xcb_change_active_pointer_grab)
    }

    /// Sends a `GrabKeyboard` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_grab_keyboard_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_grab_keyboard_reply`]: Self::xcb_grab_keyboard_reply
    #[inline]
    pub unsafe fn xcb_grab_keyboard(
        &self,
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        time: xcb_timestamp_t,
        pointer_mode: u8,
        keyboard_mode: u8,
    ) -> xcb_grab_keyboard_cookie_t {
        sym!(self, xcb_grab_keyboard)(
            c,
            owner_events,
            grab_window,
            time,
            pointer_mode,
            keyboard_mode,
        )
    }

    /// Returns `true` iff the symbol `xcb_grab_keyboard` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_grab_keyboard(&self) -> bool {
        has_sym!(self, xcb_grab_keyboard)
    }

    /// Sends a `GrabKeyboard` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_grab_keyboard_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_grab_keyboard_reply`]: Self::xcb_grab_keyboard_reply
    #[inline]
    pub unsafe fn xcb_grab_keyboard_unchecked(
        &self,
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        time: xcb_timestamp_t,
        pointer_mode: u8,
        keyboard_mode: u8,
    ) -> xcb_grab_keyboard_cookie_t {
        sym!(self, xcb_grab_keyboard_unchecked)(
            c,
            owner_events,
            grab_window,
            time,
            pointer_mode,
            keyboard_mode,
        )
    }

    /// Returns `true` iff the symbol `xcb_grab_keyboard_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_grab_keyboard_unchecked(&self) -> bool {
        has_sym!(self, xcb_grab_keyboard_unchecked)
    }

    /// Waits for the reply to a `GrabKeyboard` request.
    #[inline]
    pub unsafe fn xcb_grab_keyboard_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_grab_keyboard_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_grab_keyboard_reply_t {
        sym!(self, xcb_grab_keyboard_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_grab_keyboard_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_grab_keyboard_reply(&self) -> bool {
        has_sym!(self, xcb_grab_keyboard_reply)
    }

    /// Sends a `UngrabKeyboard` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_ungrab_keyboard_checked(
        &self,
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_ungrab_keyboard_checked)(c, time)
    }

    /// Returns `true` iff the symbol `xcb_ungrab_keyboard_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_ungrab_keyboard_checked(&self) -> bool {
        has_sym!(self, xcb_ungrab_keyboard_checked)
    }

    /// Sends a `UngrabKeyboard` request (unchecked).
    #[inline]
    pub unsafe fn xcb_ungrab_keyboard(
        &self,
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_ungrab_keyboard)(c, time)
    }

    /// Returns `true` iff the symbol `xcb_ungrab_keyboard` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_ungrab_keyboard(&self) -> bool {
        has_sym!(self, xcb_ungrab_keyboard)
    }

    /// Sends a `GrabKey` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_grab_key_checked(
        &self,
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        modifiers: u16,
        key: xcb_keycode_t,
        pointer_mode: u8,
        keyboard_mode: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_grab_key_checked)(
            c,
            owner_events,
            grab_window,
            modifiers,
            key,
            pointer_mode,
            keyboard_mode,
        )
    }

    /// Returns `true` iff the symbol `xcb_grab_key_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_grab_key_checked(&self) -> bool {
        has_sym!(self, xcb_grab_key_checked)
    }

    /// Sends a `GrabKey` request (unchecked).
    #[inline]
    pub unsafe fn xcb_grab_key(
        &self,
        c: *mut xcb_connection_t,
        owner_events: u8,
        grab_window: xcb_window_t,
        modifiers: u16,
        key: xcb_keycode_t,
        pointer_mode: u8,
        keyboard_mode: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_grab_key)(
            c,
            owner_events,
            grab_window,
            modifiers,
            key,
            pointer_mode,
            keyboard_mode,
        )
    }

    /// Returns `true` iff the symbol `xcb_grab_key` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_grab_key(&self) -> bool {
        has_sym!(self, xcb_grab_key)
    }

    /// Sends a `UngrabKey` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_ungrab_key_checked(
        &self,
        c: *mut xcb_connection_t,
        key: xcb_keycode_t,
        grab_window: xcb_window_t,
        modifiers: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_ungrab_key_checked)(c, key, grab_window, modifiers)
    }

    /// Returns `true` iff the symbol `xcb_ungrab_key_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_ungrab_key_checked(&self) -> bool {
        has_sym!(self, xcb_ungrab_key_checked)
    }

    /// Sends a `UngrabKey` request (unchecked).
    #[inline]
    pub unsafe fn xcb_ungrab_key(
        &self,
        c: *mut xcb_connection_t,
        key: xcb_keycode_t,
        grab_window: xcb_window_t,
        modifiers: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_ungrab_key)(c, key, grab_window, modifiers)
    }

    /// Returns `true` iff the symbol `xcb_ungrab_key` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_ungrab_key(&self) -> bool {
        has_sym!(self, xcb_ungrab_key)
    }

    /// Sends a `AllowEvents` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_allow_events_checked(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_allow_events_checked)(c, mode, time)
    }

    /// Returns `true` iff the symbol `xcb_allow_events_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_allow_events_checked(&self) -> bool {
        has_sym!(self, xcb_allow_events_checked)
    }

    /// Sends a `AllowEvents` request (unchecked).
    #[inline]
    pub unsafe fn xcb_allow_events(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_allow_events)(c, mode, time)
    }

    /// Returns `true` iff the symbol `xcb_allow_events` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_allow_events(&self) -> bool {
        has_sym!(self, xcb_allow_events)
    }

    /// Sends a `GrabServer` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_grab_server_checked(&self, c: *mut xcb_connection_t) -> xcb_void_cookie_t {
        sym!(self, xcb_grab_server_checked)(c)
    }

    /// Returns `true` iff the symbol `xcb_grab_server_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_grab_server_checked(&self) -> bool {
        has_sym!(self, xcb_grab_server_checked)
    }

    /// Sends a `GrabServer` request (unchecked).
    #[inline]
    pub unsafe fn xcb_grab_server(&self, c: *mut xcb_connection_t) -> xcb_void_cookie_t {
        sym!(self, xcb_grab_server)(c)
    }

    /// Returns `true` iff the symbol `xcb_grab_server` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_grab_server(&self) -> bool {
        has_sym!(self, xcb_grab_server)
    }

    /// Sends a `UngrabServer` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_ungrab_server_checked(&self, c: *mut xcb_connection_t) -> xcb_void_cookie_t {
        sym!(self, xcb_ungrab_server_checked)(c)
    }

    /// Returns `true` iff the symbol `xcb_ungrab_server_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_ungrab_server_checked(&self) -> bool {
        has_sym!(self, xcb_ungrab_server_checked)
    }

    /// Sends a `UngrabServer` request (unchecked).
    #[inline]
    pub unsafe fn xcb_ungrab_server(&self, c: *mut xcb_connection_t) -> xcb_void_cookie_t {
        sym!(self, xcb_ungrab_server)(c)
    }

    /// Returns `true` iff the symbol `xcb_ungrab_server` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_ungrab_server(&self) -> bool {
        has_sym!(self, xcb_ungrab_server)
    }

    /// Sends a `QueryPointer` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_query_pointer_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_query_pointer_reply`]: Self::xcb_query_pointer_reply
    #[inline]
    pub unsafe fn xcb_query_pointer(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_query_pointer_cookie_t {
        sym!(self, xcb_query_pointer)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_query_pointer` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_pointer(&self) -> bool {
        has_sym!(self, xcb_query_pointer)
    }

    /// Sends a `QueryPointer` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_query_pointer_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_query_pointer_reply`]: Self::xcb_query_pointer_reply
    #[inline]
    pub unsafe fn xcb_query_pointer_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_query_pointer_cookie_t {
        sym!(self, xcb_query_pointer_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_query_pointer_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_pointer_unchecked(&self) -> bool {
        has_sym!(self, xcb_query_pointer_unchecked)
    }

    /// Waits for the reply to a `QueryPointer` request.
    #[inline]
    pub unsafe fn xcb_query_pointer_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_query_pointer_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_pointer_reply_t {
        sym!(self, xcb_query_pointer_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_query_pointer_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_pointer_reply(&self) -> bool {
        has_sym!(self, xcb_query_pointer_reply)
    }

    /// Advances a `xcb_timecoord_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_timecoord_next(&self, i: *mut xcb_timecoord_iterator_t) {
        sym!(self, xcb_timecoord_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_timecoord_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_timecoord_next(&self) -> bool {
        has_sym!(self, xcb_timecoord_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_timecoord_iterator_t`.
    #[inline]
    pub unsafe fn xcb_timecoord_end(&self, i: xcb_timecoord_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_timecoord_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_timecoord_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_timecoord_end(&self) -> bool {
        has_sym!(self, xcb_timecoord_end)
    }

    /// Computes the size of a `xcb_get_motion_events_reply_t` object.
    #[inline]
    pub unsafe fn xcb_get_motion_events_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_get_motion_events_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_get_motion_events_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_motion_events_sizeof(&self) -> bool {
        has_sym!(self, xcb_get_motion_events_sizeof)
    }

    /// Sends a `GetMotionEvents` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_motion_events_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_motion_events_reply`]: Self::xcb_get_motion_events_reply
    #[inline]
    pub unsafe fn xcb_get_motion_events(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        start: xcb_timestamp_t,
        stop: xcb_timestamp_t,
    ) -> xcb_get_motion_events_cookie_t {
        sym!(self, xcb_get_motion_events)(c, window, start, stop)
    }

    /// Returns `true` iff the symbol `xcb_get_motion_events` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_motion_events(&self) -> bool {
        has_sym!(self, xcb_get_motion_events)
    }

    /// Sends a `GetMotionEvents` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_motion_events_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_motion_events_reply`]: Self::xcb_get_motion_events_reply
    #[inline]
    pub unsafe fn xcb_get_motion_events_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        start: xcb_timestamp_t,
        stop: xcb_timestamp_t,
    ) -> xcb_get_motion_events_cookie_t {
        sym!(self, xcb_get_motion_events_unchecked)(c, window, start, stop)
    }

    /// Returns `true` iff the symbol `xcb_get_motion_events_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_motion_events_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_motion_events_unchecked)
    }

    /// Returns a pointer to the `events` field of a `xcb_get_motion_events_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_get_motion_events_events(
        &self,
        r: *const xcb_get_motion_events_reply_t,
    ) -> *mut xcb_timecoord_t {
        sym!(self, xcb_get_motion_events_events)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_motion_events_events` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_motion_events_events(&self) -> bool {
        has_sym!(self, xcb_get_motion_events_events)
    }

    /// Returns the number of elements of the `events` field of a `xcb_get_motion_events_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_get_motion_events_events_length(
        &self,
        r: *const xcb_get_motion_events_reply_t,
    ) -> c_int {
        sym!(self, xcb_get_motion_events_events_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_motion_events_events_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_motion_events_events_length(&self) -> bool {
        has_sym!(self, xcb_get_motion_events_events_length)
    }

    /// Returns an iterator over the elements of the
    /// `events` field of a `xcb_get_motion_events_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_get_motion_events_events_iterator(
        &self,
        r: *const xcb_get_motion_events_reply_t,
    ) -> xcb_timecoord_iterator_t {
        sym!(self, xcb_get_motion_events_events_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_motion_events_events_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_motion_events_events_iterator(&self) -> bool {
        has_sym!(self, xcb_get_motion_events_events_iterator)
    }

    /// Waits for the reply to a `GetMotionEvents` request.
    #[inline]
    pub unsafe fn xcb_get_motion_events_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_motion_events_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_motion_events_reply_t {
        sym!(self, xcb_get_motion_events_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_motion_events_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_motion_events_reply(&self) -> bool {
        has_sym!(self, xcb_get_motion_events_reply)
    }

    /// Sends a `TranslateCoordinates` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_translate_coordinates_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_translate_coordinates_reply`]: Self::xcb_translate_coordinates_reply
    #[inline]
    pub unsafe fn xcb_translate_coordinates(
        &self,
        c: *mut xcb_connection_t,
        src_window: xcb_window_t,
        dst_window: xcb_window_t,
        src_x: i16,
        src_y: i16,
    ) -> xcb_translate_coordinates_cookie_t {
        sym!(self, xcb_translate_coordinates)(c, src_window, dst_window, src_x, src_y)
    }

    /// Returns `true` iff the symbol `xcb_translate_coordinates` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_translate_coordinates(&self) -> bool {
        has_sym!(self, xcb_translate_coordinates)
    }

    /// Sends a `TranslateCoordinates` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_translate_coordinates_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_translate_coordinates_reply`]: Self::xcb_translate_coordinates_reply
    #[inline]
    pub unsafe fn xcb_translate_coordinates_unchecked(
        &self,
        c: *mut xcb_connection_t,
        src_window: xcb_window_t,
        dst_window: xcb_window_t,
        src_x: i16,
        src_y: i16,
    ) -> xcb_translate_coordinates_cookie_t {
        sym!(self, xcb_translate_coordinates_unchecked)(c, src_window, dst_window, src_x, src_y)
    }

    /// Returns `true` iff the symbol `xcb_translate_coordinates_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_translate_coordinates_unchecked(&self) -> bool {
        has_sym!(self, xcb_translate_coordinates_unchecked)
    }

    /// Waits for the reply to a `TranslateCoordinates` request.
    #[inline]
    pub unsafe fn xcb_translate_coordinates_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_translate_coordinates_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_translate_coordinates_reply_t {
        sym!(self, xcb_translate_coordinates_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_translate_coordinates_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_translate_coordinates_reply(&self) -> bool {
        has_sym!(self, xcb_translate_coordinates_reply)
    }

    /// Sends a `WarpPointer` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_warp_pointer_checked(
        &self,
        c: *mut xcb_connection_t,
        src_window: xcb_window_t,
        dst_window: xcb_window_t,
        src_x: i16,
        src_y: i16,
        src_width: u16,
        src_height: u16,
        dst_x: i16,
        dst_y: i16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_warp_pointer_checked)(
            c, src_window, dst_window, src_x, src_y, src_width, src_height, dst_x, dst_y,
        )
    }

    /// Returns `true` iff the symbol `xcb_warp_pointer_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_warp_pointer_checked(&self) -> bool {
        has_sym!(self, xcb_warp_pointer_checked)
    }

    /// Sends a `WarpPointer` request (unchecked).
    #[inline]
    pub unsafe fn xcb_warp_pointer(
        &self,
        c: *mut xcb_connection_t,
        src_window: xcb_window_t,
        dst_window: xcb_window_t,
        src_x: i16,
        src_y: i16,
        src_width: u16,
        src_height: u16,
        dst_x: i16,
        dst_y: i16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_warp_pointer)(
            c, src_window, dst_window, src_x, src_y, src_width, src_height, dst_x, dst_y,
        )
    }

    /// Returns `true` iff the symbol `xcb_warp_pointer` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_warp_pointer(&self) -> bool {
        has_sym!(self, xcb_warp_pointer)
    }

    /// Sends a `SetInputFocus` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_set_input_focus_checked(
        &self,
        c: *mut xcb_connection_t,
        revert_to: u8,
        focus: xcb_window_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_input_focus_checked)(c, revert_to, focus, time)
    }

    /// Returns `true` iff the symbol `xcb_set_input_focus_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_input_focus_checked(&self) -> bool {
        has_sym!(self, xcb_set_input_focus_checked)
    }

    /// Sends a `SetInputFocus` request (unchecked).
    #[inline]
    pub unsafe fn xcb_set_input_focus(
        &self,
        c: *mut xcb_connection_t,
        revert_to: u8,
        focus: xcb_window_t,
        time: xcb_timestamp_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_input_focus)(c, revert_to, focus, time)
    }

    /// Returns `true` iff the symbol `xcb_set_input_focus` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_input_focus(&self) -> bool {
        has_sym!(self, xcb_set_input_focus)
    }

    /// Sends a `GetInputFocus` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_input_focus_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_input_focus_reply`]: Self::xcb_get_input_focus_reply
    #[inline]
    pub unsafe fn xcb_get_input_focus(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_get_input_focus_cookie_t {
        sym!(self, xcb_get_input_focus)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_input_focus` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_input_focus(&self) -> bool {
        has_sym!(self, xcb_get_input_focus)
    }

    /// Sends a `GetInputFocus` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_input_focus_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_input_focus_reply`]: Self::xcb_get_input_focus_reply
    #[inline]
    pub unsafe fn xcb_get_input_focus_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_get_input_focus_cookie_t {
        sym!(self, xcb_get_input_focus_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_input_focus_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_input_focus_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_input_focus_unchecked)
    }

    /// Waits for the reply to a `GetInputFocus` request.
    #[inline]
    pub unsafe fn xcb_get_input_focus_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_input_focus_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_input_focus_reply_t {
        sym!(self, xcb_get_input_focus_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_input_focus_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_input_focus_reply(&self) -> bool {
        has_sym!(self, xcb_get_input_focus_reply)
    }

    /// Sends a `QueryKeymap` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_query_keymap_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_query_keymap_reply`]: Self::xcb_query_keymap_reply
    #[inline]
    pub unsafe fn xcb_query_keymap(&self, c: *mut xcb_connection_t) -> xcb_query_keymap_cookie_t {
        sym!(self, xcb_query_keymap)(c)
    }

    /// Returns `true` iff the symbol `xcb_query_keymap` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_keymap(&self) -> bool {
        has_sym!(self, xcb_query_keymap)
    }

    /// Sends a `QueryKeymap` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_query_keymap_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_query_keymap_reply`]: Self::xcb_query_keymap_reply
    #[inline]
    pub unsafe fn xcb_query_keymap_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_query_keymap_cookie_t {
        sym!(self, xcb_query_keymap_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_query_keymap_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_keymap_unchecked(&self) -> bool {
        has_sym!(self, xcb_query_keymap_unchecked)
    }

    /// Waits for the reply to a `QueryKeymap` request.
    #[inline]
    pub unsafe fn xcb_query_keymap_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_query_keymap_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_keymap_reply_t {
        sym!(self, xcb_query_keymap_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_query_keymap_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_keymap_reply(&self) -> bool {
        has_sym!(self, xcb_query_keymap_reply)
    }

    /// Computes the size of a `xcb_open_font_request_t` object.
    #[inline]
    pub unsafe fn xcb_open_font_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_open_font_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_open_font_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_open_font_sizeof(&self) -> bool {
        has_sym!(self, xcb_open_font_sizeof)
    }

    /// Sends a `OpenFont` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_open_font_checked(
        &self,
        c: *mut xcb_connection_t,
        fid: xcb_font_t,
        name_len: u16,
        name: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_open_font_checked)(c, fid, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_open_font_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_open_font_checked(&self) -> bool {
        has_sym!(self, xcb_open_font_checked)
    }

    /// Sends a `OpenFont` request (unchecked).
    #[inline]
    pub unsafe fn xcb_open_font(
        &self,
        c: *mut xcb_connection_t,
        fid: xcb_font_t,
        name_len: u16,
        name: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_open_font)(c, fid, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_open_font` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_open_font(&self) -> bool {
        has_sym!(self, xcb_open_font)
    }

    /// Returns a pointer to the `name` field of a `xcb_open_font_request_t` struct.
    #[inline]
    pub unsafe fn xcb_open_font_name(&self, r: *const xcb_open_font_request_t) -> *mut c_char {
        sym!(self, xcb_open_font_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_open_font_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_open_font_name(&self) -> bool {
        has_sym!(self, xcb_open_font_name)
    }

    /// Returns the number of elements of the `name` field of a `xcb_open_font_request_t` struct.
    #[inline]
    pub unsafe fn xcb_open_font_name_length(&self, r: *const xcb_open_font_request_t) -> c_int {
        sym!(self, xcb_open_font_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_open_font_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_open_font_name_length(&self) -> bool {
        has_sym!(self, xcb_open_font_name_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `name` field of a `xcb_open_font_request_t` struct.
    #[inline]
    pub unsafe fn xcb_open_font_name_end(
        &self,
        r: *const xcb_open_font_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_open_font_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_open_font_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_open_font_name_end(&self) -> bool {
        has_sym!(self, xcb_open_font_name_end)
    }

    /// Sends a `CloseFont` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_close_font_checked(
        &self,
        c: *mut xcb_connection_t,
        font: xcb_font_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_close_font_checked)(c, font)
    }

    /// Returns `true` iff the symbol `xcb_close_font_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_close_font_checked(&self) -> bool {
        has_sym!(self, xcb_close_font_checked)
    }

    /// Sends a `CloseFont` request (unchecked).
    #[inline]
    pub unsafe fn xcb_close_font(
        &self,
        c: *mut xcb_connection_t,
        font: xcb_font_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_close_font)(c, font)
    }

    /// Returns `true` iff the symbol `xcb_close_font` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_close_font(&self) -> bool {
        has_sym!(self, xcb_close_font)
    }

    /// Advances a `xcb_fontprop_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_fontprop_next(&self, i: *mut xcb_fontprop_iterator_t) {
        sym!(self, xcb_fontprop_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_fontprop_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_fontprop_next(&self) -> bool {
        has_sym!(self, xcb_fontprop_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_fontprop_iterator_t`.
    #[inline]
    pub unsafe fn xcb_fontprop_end(&self, i: xcb_fontprop_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_fontprop_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_fontprop_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_fontprop_end(&self) -> bool {
        has_sym!(self, xcb_fontprop_end)
    }

    /// Advances a `xcb_charinfo_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_charinfo_next(&self, i: *mut xcb_charinfo_iterator_t) {
        sym!(self, xcb_charinfo_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_charinfo_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_charinfo_next(&self) -> bool {
        has_sym!(self, xcb_charinfo_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_charinfo_iterator_t`.
    #[inline]
    pub unsafe fn xcb_charinfo_end(&self, i: xcb_charinfo_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_charinfo_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_charinfo_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_charinfo_end(&self) -> bool {
        has_sym!(self, xcb_charinfo_end)
    }

    /// Computes the size of a `xcb_query_font_reply_t` object.
    #[inline]
    pub unsafe fn xcb_query_font_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_query_font_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_query_font_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_font_sizeof(&self) -> bool {
        has_sym!(self, xcb_query_font_sizeof)
    }

    /// Sends a `QueryFont` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_query_font_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_query_font_reply`]: Self::xcb_query_font_reply
    #[inline]
    pub unsafe fn xcb_query_font(
        &self,
        c: *mut xcb_connection_t,
        font: xcb_fontable_t,
    ) -> xcb_query_font_cookie_t {
        sym!(self, xcb_query_font)(c, font)
    }

    /// Returns `true` iff the symbol `xcb_query_font` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_font(&self) -> bool {
        has_sym!(self, xcb_query_font)
    }

    /// Sends a `QueryFont` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_query_font_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_query_font_reply`]: Self::xcb_query_font_reply
    #[inline]
    pub unsafe fn xcb_query_font_unchecked(
        &self,
        c: *mut xcb_connection_t,
        font: xcb_fontable_t,
    ) -> xcb_query_font_cookie_t {
        sym!(self, xcb_query_font_unchecked)(c, font)
    }

    /// Returns `true` iff the symbol `xcb_query_font_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_font_unchecked(&self) -> bool {
        has_sym!(self, xcb_query_font_unchecked)
    }

    /// Returns a pointer to the `properties` field of a `xcb_query_font_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_query_font_properties(
        &self,
        r: *const xcb_query_font_reply_t,
    ) -> *mut xcb_fontprop_t {
        sym!(self, xcb_query_font_properties)(r)
    }

    /// Returns `true` iff the symbol `xcb_query_font_properties` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_font_properties(&self) -> bool {
        has_sym!(self, xcb_query_font_properties)
    }

    /// Returns the number of elements of the `properties` field of a `xcb_query_font_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_query_font_properties_length(
        &self,
        r: *const xcb_query_font_reply_t,
    ) -> c_int {
        sym!(self, xcb_query_font_properties_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_query_font_properties_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_font_properties_length(&self) -> bool {
        has_sym!(self, xcb_query_font_properties_length)
    }

    /// Returns an iterator over the elements of the
    /// `properties` field of a `xcb_query_font_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_query_font_properties_iterator(
        &self,
        r: *const xcb_query_font_reply_t,
    ) -> xcb_fontprop_iterator_t {
        sym!(self, xcb_query_font_properties_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_query_font_properties_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_font_properties_iterator(&self) -> bool {
        has_sym!(self, xcb_query_font_properties_iterator)
    }

    /// Returns a pointer to the `char_infos` field of a `xcb_query_font_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_query_font_char_infos(
        &self,
        r: *const xcb_query_font_reply_t,
    ) -> *mut xcb_charinfo_t {
        sym!(self, xcb_query_font_char_infos)(r)
    }

    /// Returns `true` iff the symbol `xcb_query_font_char_infos` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_font_char_infos(&self) -> bool {
        has_sym!(self, xcb_query_font_char_infos)
    }

    /// Returns the number of elements of the `char_infos` field of a `xcb_query_font_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_query_font_char_infos_length(
        &self,
        r: *const xcb_query_font_reply_t,
    ) -> c_int {
        sym!(self, xcb_query_font_char_infos_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_query_font_char_infos_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_font_char_infos_length(&self) -> bool {
        has_sym!(self, xcb_query_font_char_infos_length)
    }

    /// Returns an iterator over the elements of the
    /// `char_infos` field of a `xcb_query_font_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_query_font_char_infos_iterator(
        &self,
        r: *const xcb_query_font_reply_t,
    ) -> xcb_charinfo_iterator_t {
        sym!(self, xcb_query_font_char_infos_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_query_font_char_infos_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_font_char_infos_iterator(&self) -> bool {
        has_sym!(self, xcb_query_font_char_infos_iterator)
    }

    /// Waits for the reply to a `QueryFont` request.
    #[inline]
    pub unsafe fn xcb_query_font_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_query_font_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_font_reply_t {
        sym!(self, xcb_query_font_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_query_font_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_font_reply(&self) -> bool {
        has_sym!(self, xcb_query_font_reply)
    }

    /// Computes the size of a `xcb_query_text_extents_request_t` object.
    #[inline]
    pub unsafe fn xcb_query_text_extents_sizeof(
        &self,
        _buffer: *const c_void,
        string_len: u32,
    ) -> c_int {
        sym!(self, xcb_query_text_extents_sizeof)(_buffer, string_len)
    }

    /// Returns `true` iff the symbol `xcb_query_text_extents_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_text_extents_sizeof(&self) -> bool {
        has_sym!(self, xcb_query_text_extents_sizeof)
    }

    /// Sends a `QueryTextExtents` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_query_text_extents_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_query_text_extents_reply`]: Self::xcb_query_text_extents_reply
    #[inline]
    pub unsafe fn xcb_query_text_extents(
        &self,
        c: *mut xcb_connection_t,
        font: xcb_fontable_t,
        string_len: u32,
        string: *const xcb_char2b_t,
    ) -> xcb_query_text_extents_cookie_t {
        sym!(self, xcb_query_text_extents)(c, font, string_len, string)
    }

    /// Returns `true` iff the symbol `xcb_query_text_extents` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_text_extents(&self) -> bool {
        has_sym!(self, xcb_query_text_extents)
    }

    /// Sends a `QueryTextExtents` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_query_text_extents_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_query_text_extents_reply`]: Self::xcb_query_text_extents_reply
    #[inline]
    pub unsafe fn xcb_query_text_extents_unchecked(
        &self,
        c: *mut xcb_connection_t,
        font: xcb_fontable_t,
        string_len: u32,
        string: *const xcb_char2b_t,
    ) -> xcb_query_text_extents_cookie_t {
        sym!(self, xcb_query_text_extents_unchecked)(c, font, string_len, string)
    }

    /// Returns `true` iff the symbol `xcb_query_text_extents_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_text_extents_unchecked(&self) -> bool {
        has_sym!(self, xcb_query_text_extents_unchecked)
    }

    /// Waits for the reply to a `QueryTextExtents` request.
    #[inline]
    pub unsafe fn xcb_query_text_extents_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_query_text_extents_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_text_extents_reply_t {
        sym!(self, xcb_query_text_extents_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_query_text_extents_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_text_extents_reply(&self) -> bool {
        has_sym!(self, xcb_query_text_extents_reply)
    }

    /// Computes the size of a `xcb_str_t` object.
    #[inline]
    pub unsafe fn xcb_str_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_str_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_str_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_str_sizeof(&self) -> bool {
        has_sym!(self, xcb_str_sizeof)
    }

    /// Returns a pointer to the `name` field of a `xcb_str_t` struct.
    #[inline]
    pub unsafe fn xcb_str_name(&self, r: *const xcb_str_t) -> *mut c_char {
        sym!(self, xcb_str_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_str_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_str_name(&self) -> bool {
        has_sym!(self, xcb_str_name)
    }

    /// Returns the number of elements of the `name` field of a `xcb_str_t` struct.
    #[inline]
    pub unsafe fn xcb_str_name_length(&self, r: *const xcb_str_t) -> c_int {
        sym!(self, xcb_str_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_str_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_str_name_length(&self) -> bool {
        has_sym!(self, xcb_str_name_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `name` field of a `xcb_str_t` struct.
    #[inline]
    pub unsafe fn xcb_str_name_end(&self, r: *const xcb_str_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_str_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_str_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_str_name_end(&self) -> bool {
        has_sym!(self, xcb_str_name_end)
    }

    /// Advances a `xcb_str_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_str_next(&self, i: *mut xcb_str_iterator_t) {
        sym!(self, xcb_str_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_str_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_str_next(&self) -> bool {
        has_sym!(self, xcb_str_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_str_iterator_t`.
    #[inline]
    pub unsafe fn xcb_str_end(&self, i: xcb_str_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_str_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_str_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_str_end(&self) -> bool {
        has_sym!(self, xcb_str_end)
    }

    /// Computes the size of a `xcb_list_fonts_request_t` object.
    #[inline]
    pub unsafe fn xcb_list_fonts_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_list_fonts_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_sizeof(&self) -> bool {
        has_sym!(self, xcb_list_fonts_sizeof)
    }

    /// Sends a `ListFonts` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_list_fonts_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_list_fonts_reply`]: Self::xcb_list_fonts_reply
    #[inline]
    pub unsafe fn xcb_list_fonts(
        &self,
        c: *mut xcb_connection_t,
        max_names: u16,
        pattern_len: u16,
        pattern: *const c_char,
    ) -> xcb_list_fonts_cookie_t {
        sym!(self, xcb_list_fonts)(c, max_names, pattern_len, pattern)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts(&self) -> bool {
        has_sym!(self, xcb_list_fonts)
    }

    /// Sends a `ListFonts` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_list_fonts_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_list_fonts_reply`]: Self::xcb_list_fonts_reply
    #[inline]
    pub unsafe fn xcb_list_fonts_unchecked(
        &self,
        c: *mut xcb_connection_t,
        max_names: u16,
        pattern_len: u16,
        pattern: *const c_char,
    ) -> xcb_list_fonts_cookie_t {
        sym!(self, xcb_list_fonts_unchecked)(c, max_names, pattern_len, pattern)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_unchecked(&self) -> bool {
        has_sym!(self, xcb_list_fonts_unchecked)
    }

    /// Returns the number of elements of the `names` field of a `xcb_list_fonts_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_list_fonts_names_length(&self, r: *const xcb_list_fonts_reply_t) -> c_int {
        sym!(self, xcb_list_fonts_names_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_names_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_names_length(&self) -> bool {
        has_sym!(self, xcb_list_fonts_names_length)
    }

    /// Returns an iterator over the elements of the
    /// `names` field of a `xcb_list_fonts_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_list_fonts_names_iterator(
        &self,
        r: *const xcb_list_fonts_reply_t,
    ) -> xcb_str_iterator_t {
        sym!(self, xcb_list_fonts_names_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_names_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_names_iterator(&self) -> bool {
        has_sym!(self, xcb_list_fonts_names_iterator)
    }

    /// Waits for the reply to a `ListFonts` request.
    #[inline]
    pub unsafe fn xcb_list_fonts_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_list_fonts_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_list_fonts_reply_t {
        sym!(self, xcb_list_fonts_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_reply(&self) -> bool {
        has_sym!(self, xcb_list_fonts_reply)
    }

    /// Computes the size of a `xcb_list_fonts_with_info_request_t` object.
    #[inline]
    pub unsafe fn xcb_list_fonts_with_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_list_fonts_with_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_with_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_with_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_list_fonts_with_info_sizeof)
    }

    /// Sends a `ListFontsWithInfo` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_list_fonts_with_info_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_list_fonts_with_info_reply`]: Self::xcb_list_fonts_with_info_reply
    #[inline]
    pub unsafe fn xcb_list_fonts_with_info(
        &self,
        c: *mut xcb_connection_t,
        max_names: u16,
        pattern_len: u16,
        pattern: *const c_char,
    ) -> xcb_list_fonts_with_info_cookie_t {
        sym!(self, xcb_list_fonts_with_info)(c, max_names, pattern_len, pattern)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_with_info` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_with_info(&self) -> bool {
        has_sym!(self, xcb_list_fonts_with_info)
    }

    /// Sends a `ListFontsWithInfo` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_list_fonts_with_info_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_list_fonts_with_info_reply`]: Self::xcb_list_fonts_with_info_reply
    #[inline]
    pub unsafe fn xcb_list_fonts_with_info_unchecked(
        &self,
        c: *mut xcb_connection_t,
        max_names: u16,
        pattern_len: u16,
        pattern: *const c_char,
    ) -> xcb_list_fonts_with_info_cookie_t {
        sym!(self, xcb_list_fonts_with_info_unchecked)(c, max_names, pattern_len, pattern)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_with_info_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_with_info_unchecked(&self) -> bool {
        has_sym!(self, xcb_list_fonts_with_info_unchecked)
    }

    /// Returns a pointer to the `properties` field of a `xcb_list_fonts_with_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_list_fonts_with_info_properties(
        &self,
        r: *const xcb_list_fonts_with_info_reply_t,
    ) -> *mut xcb_fontprop_t {
        sym!(self, xcb_list_fonts_with_info_properties)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_with_info_properties` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_with_info_properties(&self) -> bool {
        has_sym!(self, xcb_list_fonts_with_info_properties)
    }

    /// Returns the number of elements of the `properties` field of a `xcb_list_fonts_with_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_list_fonts_with_info_properties_length(
        &self,
        r: *const xcb_list_fonts_with_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_list_fonts_with_info_properties_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_with_info_properties_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_with_info_properties_length(&self) -> bool {
        has_sym!(self, xcb_list_fonts_with_info_properties_length)
    }

    /// Returns an iterator over the elements of the
    /// `properties` field of a `xcb_list_fonts_with_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_list_fonts_with_info_properties_iterator(
        &self,
        r: *const xcb_list_fonts_with_info_reply_t,
    ) -> xcb_fontprop_iterator_t {
        sym!(self, xcb_list_fonts_with_info_properties_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_with_info_properties_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_with_info_properties_iterator(&self) -> bool {
        has_sym!(self, xcb_list_fonts_with_info_properties_iterator)
    }

    /// Returns a pointer to the `name` field of a `xcb_list_fonts_with_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_list_fonts_with_info_name(
        &self,
        r: *const xcb_list_fonts_with_info_reply_t,
    ) -> *mut c_char {
        sym!(self, xcb_list_fonts_with_info_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_with_info_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_with_info_name(&self) -> bool {
        has_sym!(self, xcb_list_fonts_with_info_name)
    }

    /// Returns the number of elements of the `name` field of a `xcb_list_fonts_with_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_list_fonts_with_info_name_length(
        &self,
        r: *const xcb_list_fonts_with_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_list_fonts_with_info_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_with_info_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_with_info_name_length(&self) -> bool {
        has_sym!(self, xcb_list_fonts_with_info_name_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `name` field of a `xcb_list_fonts_with_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_list_fonts_with_info_name_end(
        &self,
        r: *const xcb_list_fonts_with_info_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_list_fonts_with_info_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_with_info_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_with_info_name_end(&self) -> bool {
        has_sym!(self, xcb_list_fonts_with_info_name_end)
    }

    /// Waits for the reply to a `ListFontsWithInfo` request.
    #[inline]
    pub unsafe fn xcb_list_fonts_with_info_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_list_fonts_with_info_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_list_fonts_with_info_reply_t {
        sym!(self, xcb_list_fonts_with_info_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_list_fonts_with_info_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_fonts_with_info_reply(&self) -> bool {
        has_sym!(self, xcb_list_fonts_with_info_reply)
    }

    /// Computes the size of a `xcb_set_font_path_request_t` object.
    #[inline]
    pub unsafe fn xcb_set_font_path_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_set_font_path_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_set_font_path_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_font_path_sizeof(&self) -> bool {
        has_sym!(self, xcb_set_font_path_sizeof)
    }

    /// Sends a `SetFontPath` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_set_font_path_checked(
        &self,
        c: *mut xcb_connection_t,
        font_qty: u16,
        font: *const xcb_str_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_font_path_checked)(c, font_qty, font)
    }

    /// Returns `true` iff the symbol `xcb_set_font_path_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_font_path_checked(&self) -> bool {
        has_sym!(self, xcb_set_font_path_checked)
    }

    /// Sends a `SetFontPath` request (unchecked).
    #[inline]
    pub unsafe fn xcb_set_font_path(
        &self,
        c: *mut xcb_connection_t,
        font_qty: u16,
        font: *const xcb_str_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_font_path)(c, font_qty, font)
    }

    /// Returns `true` iff the symbol `xcb_set_font_path` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_font_path(&self) -> bool {
        has_sym!(self, xcb_set_font_path)
    }

    /// Returns the number of elements of the `font` field of a `xcb_set_font_path_request_t` struct.
    #[inline]
    pub unsafe fn xcb_set_font_path_font_length(
        &self,
        r: *const xcb_set_font_path_request_t,
    ) -> c_int {
        sym!(self, xcb_set_font_path_font_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_set_font_path_font_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_font_path_font_length(&self) -> bool {
        has_sym!(self, xcb_set_font_path_font_length)
    }

    /// Returns an iterator over the elements of the
    /// `font` field of a `xcb_set_font_path_request_t` struct.
    #[inline]
    pub unsafe fn xcb_set_font_path_font_iterator(
        &self,
        r: *const xcb_set_font_path_request_t,
    ) -> xcb_str_iterator_t {
        sym!(self, xcb_set_font_path_font_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_set_font_path_font_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_font_path_font_iterator(&self) -> bool {
        has_sym!(self, xcb_set_font_path_font_iterator)
    }

    /// Computes the size of a `xcb_get_font_path_reply_t` object.
    #[inline]
    pub unsafe fn xcb_get_font_path_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_get_font_path_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_get_font_path_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_font_path_sizeof(&self) -> bool {
        has_sym!(self, xcb_get_font_path_sizeof)
    }

    /// Sends a `GetFontPath` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_font_path_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_font_path_reply`]: Self::xcb_get_font_path_reply
    #[inline]
    pub unsafe fn xcb_get_font_path(&self, c: *mut xcb_connection_t) -> xcb_get_font_path_cookie_t {
        sym!(self, xcb_get_font_path)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_font_path` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_font_path(&self) -> bool {
        has_sym!(self, xcb_get_font_path)
    }

    /// Sends a `GetFontPath` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_font_path_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_font_path_reply`]: Self::xcb_get_font_path_reply
    #[inline]
    pub unsafe fn xcb_get_font_path_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_get_font_path_cookie_t {
        sym!(self, xcb_get_font_path_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_font_path_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_font_path_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_font_path_unchecked)
    }

    /// Returns the number of elements of the `path` field of a `xcb_get_font_path_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_get_font_path_path_length(
        &self,
        r: *const xcb_get_font_path_reply_t,
    ) -> c_int {
        sym!(self, xcb_get_font_path_path_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_font_path_path_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_font_path_path_length(&self) -> bool {
        has_sym!(self, xcb_get_font_path_path_length)
    }

    /// Returns an iterator over the elements of the
    /// `path` field of a `xcb_get_font_path_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_get_font_path_path_iterator(
        &self,
        r: *const xcb_get_font_path_reply_t,
    ) -> xcb_str_iterator_t {
        sym!(self, xcb_get_font_path_path_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_font_path_path_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_font_path_path_iterator(&self) -> bool {
        has_sym!(self, xcb_get_font_path_path_iterator)
    }

    /// Waits for the reply to a `GetFontPath` request.
    #[inline]
    pub unsafe fn xcb_get_font_path_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_font_path_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_font_path_reply_t {
        sym!(self, xcb_get_font_path_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_font_path_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_font_path_reply(&self) -> bool {
        has_sym!(self, xcb_get_font_path_reply)
    }

    /// Sends a `CreatePixmap` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_create_pixmap_checked(
        &self,
        c: *mut xcb_connection_t,
        depth: u8,
        pid: xcb_pixmap_t,
        drawable: xcb_drawable_t,
        width: u16,
        height: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_pixmap_checked)(c, depth, pid, drawable, width, height)
    }

    /// Returns `true` iff the symbol `xcb_create_pixmap_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_pixmap_checked(&self) -> bool {
        has_sym!(self, xcb_create_pixmap_checked)
    }

    /// Sends a `CreatePixmap` request (unchecked).
    #[inline]
    pub unsafe fn xcb_create_pixmap(
        &self,
        c: *mut xcb_connection_t,
        depth: u8,
        pid: xcb_pixmap_t,
        drawable: xcb_drawable_t,
        width: u16,
        height: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_pixmap)(c, depth, pid, drawable, width, height)
    }

    /// Returns `true` iff the symbol `xcb_create_pixmap` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_pixmap(&self) -> bool {
        has_sym!(self, xcb_create_pixmap)
    }

    /// Sends a `FreePixmap` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_free_pixmap_checked(
        &self,
        c: *mut xcb_connection_t,
        pixmap: xcb_pixmap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_free_pixmap_checked)(c, pixmap)
    }

    /// Returns `true` iff the symbol `xcb_free_pixmap_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_free_pixmap_checked(&self) -> bool {
        has_sym!(self, xcb_free_pixmap_checked)
    }

    /// Sends a `FreePixmap` request (unchecked).
    #[inline]
    pub unsafe fn xcb_free_pixmap(
        &self,
        c: *mut xcb_connection_t,
        pixmap: xcb_pixmap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_free_pixmap)(c, pixmap)
    }

    /// Returns `true` iff the symbol `xcb_free_pixmap` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_free_pixmap(&self) -> bool {
        has_sym!(self, xcb_free_pixmap)
    }

    /// Serializes a `xcb_create_gc_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_create_gc_value_list_serialize(
        &self,
        _buffer: *mut *mut c_void,
        value_mask: u32,
        _aux: *const xcb_create_gc_value_list_t,
    ) -> c_int {
        sym!(self, xcb_create_gc_value_list_serialize)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_create_gc_value_list_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_gc_value_list_serialize(&self) -> bool {
        has_sym!(self, xcb_create_gc_value_list_serialize)
    }

    /// Unpacks a `xcb_create_gc_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_create_gc_value_list_unpack(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
        _aux: *mut xcb_create_gc_value_list_t,
    ) -> c_int {
        sym!(self, xcb_create_gc_value_list_unpack)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_create_gc_value_list_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_gc_value_list_unpack(&self) -> bool {
        has_sym!(self, xcb_create_gc_value_list_unpack)
    }

    /// Computes the size of a `xcb_create_gc_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_create_gc_value_list_sizeof(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
    ) -> c_int {
        sym!(self, xcb_create_gc_value_list_sizeof)(_buffer, value_mask)
    }

    /// Returns `true` iff the symbol `xcb_create_gc_value_list_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_gc_value_list_sizeof(&self) -> bool {
        has_sym!(self, xcb_create_gc_value_list_sizeof)
    }

    /// Computes the size of a `xcb_create_gc_request_t` object.
    #[inline]
    pub unsafe fn xcb_create_gc_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_create_gc_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_create_gc_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_gc_sizeof(&self) -> bool {
        has_sym!(self, xcb_create_gc_sizeof)
    }

    /// Sends a `CreateGC` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    ///
    /// There is an auxiliary version of this function: [`xcb_create_gc_aux_checked`].
    ///
    /// [`xcb_create_gc_aux_checked`]: Self::xcb_create_gc_aux_checked
    #[inline]
    pub unsafe fn xcb_create_gc_checked(
        &self,
        c: *mut xcb_connection_t,
        cid: xcb_gcontext_t,
        drawable: xcb_drawable_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_gc_checked)(c, cid, drawable, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_create_gc_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_gc_checked(&self) -> bool {
        has_sym!(self, xcb_create_gc_checked)
    }

    /// Sends a `CreateGC` request (unchecked).
    ///
    /// There is an auxiliary version of this function: [`xcb_create_gc_aux`].
    ///
    /// [`xcb_create_gc_aux`]: Self::xcb_create_gc_aux
    #[inline]
    pub unsafe fn xcb_create_gc(
        &self,
        c: *mut xcb_connection_t,
        cid: xcb_gcontext_t,
        drawable: xcb_drawable_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_gc)(c, cid, drawable, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_create_gc` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_gc(&self) -> bool {
        has_sym!(self, xcb_create_gc)
    }

    /// Sends a `CreateGC` request (checked) (aux).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_create_gc_aux_checked(
        &self,
        c: *mut xcb_connection_t,
        cid: xcb_gcontext_t,
        drawable: xcb_drawable_t,
        value_mask: u32,
        value_list: *const xcb_create_gc_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_gc_aux_checked)(c, cid, drawable, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_create_gc_aux_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_gc_aux_checked(&self) -> bool {
        has_sym!(self, xcb_create_gc_aux_checked)
    }

    /// Sends a `CreateGC` request (unchecked) (aux).
    #[inline]
    pub unsafe fn xcb_create_gc_aux(
        &self,
        c: *mut xcb_connection_t,
        cid: xcb_gcontext_t,
        drawable: xcb_drawable_t,
        value_mask: u32,
        value_list: *const xcb_create_gc_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_gc_aux)(c, cid, drawable, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_create_gc_aux` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_gc_aux(&self) -> bool {
        has_sym!(self, xcb_create_gc_aux)
    }

    /// Returns a pointer to the `value_list` field of a `xcb_create_gc_request_t` struct.
    #[inline]
    pub unsafe fn xcb_create_gc_value_list(
        &self,
        r: *const xcb_create_gc_request_t,
    ) -> *mut c_void {
        sym!(self, xcb_create_gc_value_list)(r)
    }

    /// Returns `true` iff the symbol `xcb_create_gc_value_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_gc_value_list(&self) -> bool {
        has_sym!(self, xcb_create_gc_value_list)
    }

    /// Serializes a `xcb_change_gc_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_change_gc_value_list_serialize(
        &self,
        _buffer: *mut *mut c_void,
        value_mask: u32,
        _aux: *const xcb_change_gc_value_list_t,
    ) -> c_int {
        sym!(self, xcb_change_gc_value_list_serialize)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_change_gc_value_list_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_gc_value_list_serialize(&self) -> bool {
        has_sym!(self, xcb_change_gc_value_list_serialize)
    }

    /// Unpacks a `xcb_change_gc_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_change_gc_value_list_unpack(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
        _aux: *mut xcb_change_gc_value_list_t,
    ) -> c_int {
        sym!(self, xcb_change_gc_value_list_unpack)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_change_gc_value_list_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_gc_value_list_unpack(&self) -> bool {
        has_sym!(self, xcb_change_gc_value_list_unpack)
    }

    /// Computes the size of a `xcb_change_gc_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_change_gc_value_list_sizeof(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
    ) -> c_int {
        sym!(self, xcb_change_gc_value_list_sizeof)(_buffer, value_mask)
    }

    /// Returns `true` iff the symbol `xcb_change_gc_value_list_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_gc_value_list_sizeof(&self) -> bool {
        has_sym!(self, xcb_change_gc_value_list_sizeof)
    }

    /// Computes the size of a `xcb_change_gc_request_t` object.
    #[inline]
    pub unsafe fn xcb_change_gc_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_change_gc_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_change_gc_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_gc_sizeof(&self) -> bool {
        has_sym!(self, xcb_change_gc_sizeof)
    }

    /// Sends a `ChangeGC` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    ///
    /// There is an auxiliary version of this function: [`xcb_change_gc_aux_checked`].
    ///
    /// [`xcb_change_gc_aux_checked`]: Self::xcb_change_gc_aux_checked
    #[inline]
    pub unsafe fn xcb_change_gc_checked(
        &self,
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_gc_checked)(c, gc, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_change_gc_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_gc_checked(&self) -> bool {
        has_sym!(self, xcb_change_gc_checked)
    }

    /// Sends a `ChangeGC` request (unchecked).
    ///
    /// There is an auxiliary version of this function: [`xcb_change_gc_aux`].
    ///
    /// [`xcb_change_gc_aux`]: Self::xcb_change_gc_aux
    #[inline]
    pub unsafe fn xcb_change_gc(
        &self,
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_gc)(c, gc, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_change_gc` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_gc(&self) -> bool {
        has_sym!(self, xcb_change_gc)
    }

    /// Sends a `ChangeGC` request (checked) (aux).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_change_gc_aux_checked(
        &self,
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        value_mask: u32,
        value_list: *const xcb_change_gc_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_gc_aux_checked)(c, gc, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_change_gc_aux_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_gc_aux_checked(&self) -> bool {
        has_sym!(self, xcb_change_gc_aux_checked)
    }

    /// Sends a `ChangeGC` request (unchecked) (aux).
    #[inline]
    pub unsafe fn xcb_change_gc_aux(
        &self,
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        value_mask: u32,
        value_list: *const xcb_change_gc_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_gc_aux)(c, gc, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_change_gc_aux` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_gc_aux(&self) -> bool {
        has_sym!(self, xcb_change_gc_aux)
    }

    /// Returns a pointer to the `value_list` field of a `xcb_change_gc_request_t` struct.
    #[inline]
    pub unsafe fn xcb_change_gc_value_list(
        &self,
        r: *const xcb_change_gc_request_t,
    ) -> *mut c_void {
        sym!(self, xcb_change_gc_value_list)(r)
    }

    /// Returns `true` iff the symbol `xcb_change_gc_value_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_gc_value_list(&self) -> bool {
        has_sym!(self, xcb_change_gc_value_list)
    }

    /// Sends a `CopyGC` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_copy_gc_checked(
        &self,
        c: *mut xcb_connection_t,
        src_gc: xcb_gcontext_t,
        dst_gc: xcb_gcontext_t,
        value_mask: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_copy_gc_checked)(c, src_gc, dst_gc, value_mask)
    }

    /// Returns `true` iff the symbol `xcb_copy_gc_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_copy_gc_checked(&self) -> bool {
        has_sym!(self, xcb_copy_gc_checked)
    }

    /// Sends a `CopyGC` request (unchecked).
    #[inline]
    pub unsafe fn xcb_copy_gc(
        &self,
        c: *mut xcb_connection_t,
        src_gc: xcb_gcontext_t,
        dst_gc: xcb_gcontext_t,
        value_mask: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_copy_gc)(c, src_gc, dst_gc, value_mask)
    }

    /// Returns `true` iff the symbol `xcb_copy_gc` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_copy_gc(&self) -> bool {
        has_sym!(self, xcb_copy_gc)
    }

    /// Computes the size of a `xcb_set_dashes_request_t` object.
    #[inline]
    pub unsafe fn xcb_set_dashes_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_set_dashes_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_set_dashes_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_dashes_sizeof(&self) -> bool {
        has_sym!(self, xcb_set_dashes_sizeof)
    }

    /// Sends a `SetDashes` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_set_dashes_checked(
        &self,
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        dash_offset: u16,
        dashes_len: u16,
        dashes: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_dashes_checked)(c, gc, dash_offset, dashes_len, dashes)
    }

    /// Returns `true` iff the symbol `xcb_set_dashes_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_dashes_checked(&self) -> bool {
        has_sym!(self, xcb_set_dashes_checked)
    }

    /// Sends a `SetDashes` request (unchecked).
    #[inline]
    pub unsafe fn xcb_set_dashes(
        &self,
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
        dash_offset: u16,
        dashes_len: u16,
        dashes: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_dashes)(c, gc, dash_offset, dashes_len, dashes)
    }

    /// Returns `true` iff the symbol `xcb_set_dashes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_dashes(&self) -> bool {
        has_sym!(self, xcb_set_dashes)
    }

    /// Returns a pointer to the `dashes` field of a `xcb_set_dashes_request_t` struct.
    #[inline]
    pub unsafe fn xcb_set_dashes_dashes(&self, r: *const xcb_set_dashes_request_t) -> *mut u8 {
        sym!(self, xcb_set_dashes_dashes)(r)
    }

    /// Returns `true` iff the symbol `xcb_set_dashes_dashes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_dashes_dashes(&self) -> bool {
        has_sym!(self, xcb_set_dashes_dashes)
    }

    /// Returns the number of elements of the `dashes` field of a `xcb_set_dashes_request_t` struct.
    #[inline]
    pub unsafe fn xcb_set_dashes_dashes_length(&self, r: *const xcb_set_dashes_request_t) -> c_int {
        sym!(self, xcb_set_dashes_dashes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_set_dashes_dashes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_dashes_dashes_length(&self) -> bool {
        has_sym!(self, xcb_set_dashes_dashes_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `dashes` field of a `xcb_set_dashes_request_t` struct.
    #[inline]
    pub unsafe fn xcb_set_dashes_dashes_end(
        &self,
        r: *const xcb_set_dashes_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_set_dashes_dashes_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_set_dashes_dashes_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_dashes_dashes_end(&self) -> bool {
        has_sym!(self, xcb_set_dashes_dashes_end)
    }

    /// Computes the size of a `xcb_set_clip_rectangles_request_t` object.
    #[inline]
    pub unsafe fn xcb_set_clip_rectangles_sizeof(
        &self,
        _buffer: *const c_void,
        rectangles_len: u32,
    ) -> c_int {
        sym!(self, xcb_set_clip_rectangles_sizeof)(_buffer, rectangles_len)
    }

    /// Returns `true` iff the symbol `xcb_set_clip_rectangles_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_clip_rectangles_sizeof(&self) -> bool {
        has_sym!(self, xcb_set_clip_rectangles_sizeof)
    }

    /// Sends a `SetClipRectangles` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_set_clip_rectangles_checked(
        &self,
        c: *mut xcb_connection_t,
        ordering: u8,
        gc: xcb_gcontext_t,
        clip_x_origin: i16,
        clip_y_origin: i16,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_clip_rectangles_checked)(
            c,
            ordering,
            gc,
            clip_x_origin,
            clip_y_origin,
            rectangles_len,
            rectangles,
        )
    }

    /// Returns `true` iff the symbol `xcb_set_clip_rectangles_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_clip_rectangles_checked(&self) -> bool {
        has_sym!(self, xcb_set_clip_rectangles_checked)
    }

    /// Sends a `SetClipRectangles` request (unchecked).
    #[inline]
    pub unsafe fn xcb_set_clip_rectangles(
        &self,
        c: *mut xcb_connection_t,
        ordering: u8,
        gc: xcb_gcontext_t,
        clip_x_origin: i16,
        clip_y_origin: i16,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_clip_rectangles)(
            c,
            ordering,
            gc,
            clip_x_origin,
            clip_y_origin,
            rectangles_len,
            rectangles,
        )
    }

    /// Returns `true` iff the symbol `xcb_set_clip_rectangles` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_clip_rectangles(&self) -> bool {
        has_sym!(self, xcb_set_clip_rectangles)
    }

    /// Returns a pointer to the `rectangles` field of a `xcb_set_clip_rectangles_request_t` struct.
    #[inline]
    pub unsafe fn xcb_set_clip_rectangles_rectangles(
        &self,
        r: *const xcb_set_clip_rectangles_request_t,
    ) -> *mut xcb_rectangle_t {
        sym!(self, xcb_set_clip_rectangles_rectangles)(r)
    }

    /// Returns `true` iff the symbol `xcb_set_clip_rectangles_rectangles` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_clip_rectangles_rectangles(&self) -> bool {
        has_sym!(self, xcb_set_clip_rectangles_rectangles)
    }

    /// Returns the number of elements of the `rectangles` field of a `xcb_set_clip_rectangles_request_t` struct.
    #[inline]
    pub unsafe fn xcb_set_clip_rectangles_rectangles_length(
        &self,
        r: *const xcb_set_clip_rectangles_request_t,
    ) -> c_int {
        sym!(self, xcb_set_clip_rectangles_rectangles_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_set_clip_rectangles_rectangles_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_clip_rectangles_rectangles_length(&self) -> bool {
        has_sym!(self, xcb_set_clip_rectangles_rectangles_length)
    }

    /// Returns an iterator over the elements of the
    /// `rectangles` field of a `xcb_set_clip_rectangles_request_t` struct.
    #[inline]
    pub unsafe fn xcb_set_clip_rectangles_rectangles_iterator(
        &self,
        r: *const xcb_set_clip_rectangles_request_t,
    ) -> xcb_rectangle_iterator_t {
        sym!(self, xcb_set_clip_rectangles_rectangles_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_set_clip_rectangles_rectangles_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_clip_rectangles_rectangles_iterator(&self) -> bool {
        has_sym!(self, xcb_set_clip_rectangles_rectangles_iterator)
    }

    /// Sends a `FreeGC` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_free_gc_checked(
        &self,
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_free_gc_checked)(c, gc)
    }

    /// Returns `true` iff the symbol `xcb_free_gc_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_free_gc_checked(&self) -> bool {
        has_sym!(self, xcb_free_gc_checked)
    }

    /// Sends a `FreeGC` request (unchecked).
    #[inline]
    pub unsafe fn xcb_free_gc(
        &self,
        c: *mut xcb_connection_t,
        gc: xcb_gcontext_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_free_gc)(c, gc)
    }

    /// Returns `true` iff the symbol `xcb_free_gc` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_free_gc(&self) -> bool {
        has_sym!(self, xcb_free_gc)
    }

    /// Sends a `ClearArea` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_clear_area_checked(
        &self,
        c: *mut xcb_connection_t,
        exposures: u8,
        window: xcb_window_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_clear_area_checked)(c, exposures, window, x, y, width, height)
    }

    /// Returns `true` iff the symbol `xcb_clear_area_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_clear_area_checked(&self) -> bool {
        has_sym!(self, xcb_clear_area_checked)
    }

    /// Sends a `ClearArea` request (unchecked).
    #[inline]
    pub unsafe fn xcb_clear_area(
        &self,
        c: *mut xcb_connection_t,
        exposures: u8,
        window: xcb_window_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_clear_area)(c, exposures, window, x, y, width, height)
    }

    /// Returns `true` iff the symbol `xcb_clear_area` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_clear_area(&self) -> bool {
        has_sym!(self, xcb_clear_area)
    }

    /// Sends a `CopyArea` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_copy_area_checked(
        &self,
        c: *mut xcb_connection_t,
        src_drawable: xcb_drawable_t,
        dst_drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        src_x: i16,
        src_y: i16,
        dst_x: i16,
        dst_y: i16,
        width: u16,
        height: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_copy_area_checked)(
            c,
            src_drawable,
            dst_drawable,
            gc,
            src_x,
            src_y,
            dst_x,
            dst_y,
            width,
            height,
        )
    }

    /// Returns `true` iff the symbol `xcb_copy_area_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_copy_area_checked(&self) -> bool {
        has_sym!(self, xcb_copy_area_checked)
    }

    /// Sends a `CopyArea` request (unchecked).
    #[inline]
    pub unsafe fn xcb_copy_area(
        &self,
        c: *mut xcb_connection_t,
        src_drawable: xcb_drawable_t,
        dst_drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        src_x: i16,
        src_y: i16,
        dst_x: i16,
        dst_y: i16,
        width: u16,
        height: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_copy_area)(
            c,
            src_drawable,
            dst_drawable,
            gc,
            src_x,
            src_y,
            dst_x,
            dst_y,
            width,
            height,
        )
    }

    /// Returns `true` iff the symbol `xcb_copy_area` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_copy_area(&self) -> bool {
        has_sym!(self, xcb_copy_area)
    }

    /// Sends a `CopyPlane` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_copy_plane_checked(
        &self,
        c: *mut xcb_connection_t,
        src_drawable: xcb_drawable_t,
        dst_drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        src_x: i16,
        src_y: i16,
        dst_x: i16,
        dst_y: i16,
        width: u16,
        height: u16,
        bit_plane: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_copy_plane_checked)(
            c,
            src_drawable,
            dst_drawable,
            gc,
            src_x,
            src_y,
            dst_x,
            dst_y,
            width,
            height,
            bit_plane,
        )
    }

    /// Returns `true` iff the symbol `xcb_copy_plane_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_copy_plane_checked(&self) -> bool {
        has_sym!(self, xcb_copy_plane_checked)
    }

    /// Sends a `CopyPlane` request (unchecked).
    #[inline]
    pub unsafe fn xcb_copy_plane(
        &self,
        c: *mut xcb_connection_t,
        src_drawable: xcb_drawable_t,
        dst_drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        src_x: i16,
        src_y: i16,
        dst_x: i16,
        dst_y: i16,
        width: u16,
        height: u16,
        bit_plane: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_copy_plane)(
            c,
            src_drawable,
            dst_drawable,
            gc,
            src_x,
            src_y,
            dst_x,
            dst_y,
            width,
            height,
            bit_plane,
        )
    }

    /// Returns `true` iff the symbol `xcb_copy_plane` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_copy_plane(&self) -> bool {
        has_sym!(self, xcb_copy_plane)
    }

    /// Computes the size of a `xcb_poly_point_request_t` object.
    #[inline]
    pub unsafe fn xcb_poly_point_sizeof(&self, _buffer: *const c_void, points_len: u32) -> c_int {
        sym!(self, xcb_poly_point_sizeof)(_buffer, points_len)
    }

    /// Returns `true` iff the symbol `xcb_poly_point_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_point_sizeof(&self) -> bool {
        has_sym!(self, xcb_poly_point_sizeof)
    }

    /// Sends a `PolyPoint` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_poly_point_checked(
        &self,
        c: *mut xcb_connection_t,
        coordinate_mode: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        points_len: u32,
        points: *const xcb_point_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_point_checked)(c, coordinate_mode, drawable, gc, points_len, points)
    }

    /// Returns `true` iff the symbol `xcb_poly_point_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_point_checked(&self) -> bool {
        has_sym!(self, xcb_poly_point_checked)
    }

    /// Sends a `PolyPoint` request (unchecked).
    #[inline]
    pub unsafe fn xcb_poly_point(
        &self,
        c: *mut xcb_connection_t,
        coordinate_mode: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        points_len: u32,
        points: *const xcb_point_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_point)(c, coordinate_mode, drawable, gc, points_len, points)
    }

    /// Returns `true` iff the symbol `xcb_poly_point` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_point(&self) -> bool {
        has_sym!(self, xcb_poly_point)
    }

    /// Returns a pointer to the `points` field of a `xcb_poly_point_request_t` struct.
    #[inline]
    pub unsafe fn xcb_poly_point_points(
        &self,
        r: *const xcb_poly_point_request_t,
    ) -> *mut xcb_point_t {
        sym!(self, xcb_poly_point_points)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_point_points` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_point_points(&self) -> bool {
        has_sym!(self, xcb_poly_point_points)
    }

    /// Returns the number of elements of the `points` field of a `xcb_poly_point_request_t` struct.
    #[inline]
    pub unsafe fn xcb_poly_point_points_length(&self, r: *const xcb_poly_point_request_t) -> c_int {
        sym!(self, xcb_poly_point_points_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_point_points_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_point_points_length(&self) -> bool {
        has_sym!(self, xcb_poly_point_points_length)
    }

    /// Returns an iterator over the elements of the
    /// `points` field of a `xcb_poly_point_request_t` struct.
    #[inline]
    pub unsafe fn xcb_poly_point_points_iterator(
        &self,
        r: *const xcb_poly_point_request_t,
    ) -> xcb_point_iterator_t {
        sym!(self, xcb_poly_point_points_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_point_points_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_point_points_iterator(&self) -> bool {
        has_sym!(self, xcb_poly_point_points_iterator)
    }

    /// Computes the size of a `xcb_poly_line_request_t` object.
    #[inline]
    pub unsafe fn xcb_poly_line_sizeof(&self, _buffer: *const c_void, points_len: u32) -> c_int {
        sym!(self, xcb_poly_line_sizeof)(_buffer, points_len)
    }

    /// Returns `true` iff the symbol `xcb_poly_line_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_line_sizeof(&self) -> bool {
        has_sym!(self, xcb_poly_line_sizeof)
    }

    /// Sends a `PolyLine` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_poly_line_checked(
        &self,
        c: *mut xcb_connection_t,
        coordinate_mode: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        points_len: u32,
        points: *const xcb_point_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_line_checked)(c, coordinate_mode, drawable, gc, points_len, points)
    }

    /// Returns `true` iff the symbol `xcb_poly_line_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_line_checked(&self) -> bool {
        has_sym!(self, xcb_poly_line_checked)
    }

    /// Sends a `PolyLine` request (unchecked).
    #[inline]
    pub unsafe fn xcb_poly_line(
        &self,
        c: *mut xcb_connection_t,
        coordinate_mode: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        points_len: u32,
        points: *const xcb_point_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_line)(c, coordinate_mode, drawable, gc, points_len, points)
    }

    /// Returns `true` iff the symbol `xcb_poly_line` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_line(&self) -> bool {
        has_sym!(self, xcb_poly_line)
    }

    /// Returns a pointer to the `points` field of a `xcb_poly_line_request_t` struct.
    #[inline]
    pub unsafe fn xcb_poly_line_points(
        &self,
        r: *const xcb_poly_line_request_t,
    ) -> *mut xcb_point_t {
        sym!(self, xcb_poly_line_points)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_line_points` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_line_points(&self) -> bool {
        has_sym!(self, xcb_poly_line_points)
    }

    /// Returns the number of elements of the `points` field of a `xcb_poly_line_request_t` struct.
    #[inline]
    pub unsafe fn xcb_poly_line_points_length(&self, r: *const xcb_poly_line_request_t) -> c_int {
        sym!(self, xcb_poly_line_points_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_line_points_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_line_points_length(&self) -> bool {
        has_sym!(self, xcb_poly_line_points_length)
    }

    /// Returns an iterator over the elements of the
    /// `points` field of a `xcb_poly_line_request_t` struct.
    #[inline]
    pub unsafe fn xcb_poly_line_points_iterator(
        &self,
        r: *const xcb_poly_line_request_t,
    ) -> xcb_point_iterator_t {
        sym!(self, xcb_poly_line_points_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_line_points_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_line_points_iterator(&self) -> bool {
        has_sym!(self, xcb_poly_line_points_iterator)
    }

    /// Advances a `xcb_segment_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_segment_next(&self, i: *mut xcb_segment_iterator_t) {
        sym!(self, xcb_segment_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_segment_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_segment_next(&self) -> bool {
        has_sym!(self, xcb_segment_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_segment_iterator_t`.
    #[inline]
    pub unsafe fn xcb_segment_end(&self, i: xcb_segment_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_segment_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_segment_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_segment_end(&self) -> bool {
        has_sym!(self, xcb_segment_end)
    }

    /// Computes the size of a `xcb_poly_segment_request_t` object.
    #[inline]
    pub unsafe fn xcb_poly_segment_sizeof(
        &self,
        _buffer: *const c_void,
        segments_len: u32,
    ) -> c_int {
        sym!(self, xcb_poly_segment_sizeof)(_buffer, segments_len)
    }

    /// Returns `true` iff the symbol `xcb_poly_segment_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_segment_sizeof(&self) -> bool {
        has_sym!(self, xcb_poly_segment_sizeof)
    }

    /// Sends a `PolySegment` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_poly_segment_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        segments_len: u32,
        segments: *const xcb_segment_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_segment_checked)(c, drawable, gc, segments_len, segments)
    }

    /// Returns `true` iff the symbol `xcb_poly_segment_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_segment_checked(&self) -> bool {
        has_sym!(self, xcb_poly_segment_checked)
    }

    /// Sends a `PolySegment` request (unchecked).
    #[inline]
    pub unsafe fn xcb_poly_segment(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        segments_len: u32,
        segments: *const xcb_segment_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_segment)(c, drawable, gc, segments_len, segments)
    }

    /// Returns `true` iff the symbol `xcb_poly_segment` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_segment(&self) -> bool {
        has_sym!(self, xcb_poly_segment)
    }

    /// Returns a pointer to the `segments` field of a `xcb_poly_segment_request_t` struct.
    #[inline]
    pub unsafe fn xcb_poly_segment_segments(
        &self,
        r: *const xcb_poly_segment_request_t,
    ) -> *mut xcb_segment_t {
        sym!(self, xcb_poly_segment_segments)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_segment_segments` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_segment_segments(&self) -> bool {
        has_sym!(self, xcb_poly_segment_segments)
    }

    /// Returns the number of elements of the `segments` field of a `xcb_poly_segment_request_t` struct.
    #[inline]
    pub unsafe fn xcb_poly_segment_segments_length(
        &self,
        r: *const xcb_poly_segment_request_t,
    ) -> c_int {
        sym!(self, xcb_poly_segment_segments_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_segment_segments_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_segment_segments_length(&self) -> bool {
        has_sym!(self, xcb_poly_segment_segments_length)
    }

    /// Returns an iterator over the elements of the
    /// `segments` field of a `xcb_poly_segment_request_t` struct.
    #[inline]
    pub unsafe fn xcb_poly_segment_segments_iterator(
        &self,
        r: *const xcb_poly_segment_request_t,
    ) -> xcb_segment_iterator_t {
        sym!(self, xcb_poly_segment_segments_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_segment_segments_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_segment_segments_iterator(&self) -> bool {
        has_sym!(self, xcb_poly_segment_segments_iterator)
    }

    /// Computes the size of a `xcb_poly_rectangle_request_t` object.
    #[inline]
    pub unsafe fn xcb_poly_rectangle_sizeof(
        &self,
        _buffer: *const c_void,
        rectangles_len: u32,
    ) -> c_int {
        sym!(self, xcb_poly_rectangle_sizeof)(_buffer, rectangles_len)
    }

    /// Returns `true` iff the symbol `xcb_poly_rectangle_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_rectangle_sizeof(&self) -> bool {
        has_sym!(self, xcb_poly_rectangle_sizeof)
    }

    /// Sends a `PolyRectangle` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_poly_rectangle_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_rectangle_checked)(c, drawable, gc, rectangles_len, rectangles)
    }

    /// Returns `true` iff the symbol `xcb_poly_rectangle_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_rectangle_checked(&self) -> bool {
        has_sym!(self, xcb_poly_rectangle_checked)
    }

    /// Sends a `PolyRectangle` request (unchecked).
    #[inline]
    pub unsafe fn xcb_poly_rectangle(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_rectangle)(c, drawable, gc, rectangles_len, rectangles)
    }

    /// Returns `true` iff the symbol `xcb_poly_rectangle` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_rectangle(&self) -> bool {
        has_sym!(self, xcb_poly_rectangle)
    }

    /// Returns a pointer to the `rectangles` field of a `xcb_poly_rectangle_request_t` struct.
    #[inline]
    pub unsafe fn xcb_poly_rectangle_rectangles(
        &self,
        r: *const xcb_poly_rectangle_request_t,
    ) -> *mut xcb_rectangle_t {
        sym!(self, xcb_poly_rectangle_rectangles)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_rectangle_rectangles` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_rectangle_rectangles(&self) -> bool {
        has_sym!(self, xcb_poly_rectangle_rectangles)
    }

    /// Returns the number of elements of the `rectangles` field of a `xcb_poly_rectangle_request_t` struct.
    #[inline]
    pub unsafe fn xcb_poly_rectangle_rectangles_length(
        &self,
        r: *const xcb_poly_rectangle_request_t,
    ) -> c_int {
        sym!(self, xcb_poly_rectangle_rectangles_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_rectangle_rectangles_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_rectangle_rectangles_length(&self) -> bool {
        has_sym!(self, xcb_poly_rectangle_rectangles_length)
    }

    /// Returns an iterator over the elements of the
    /// `rectangles` field of a `xcb_poly_rectangle_request_t` struct.
    #[inline]
    pub unsafe fn xcb_poly_rectangle_rectangles_iterator(
        &self,
        r: *const xcb_poly_rectangle_request_t,
    ) -> xcb_rectangle_iterator_t {
        sym!(self, xcb_poly_rectangle_rectangles_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_rectangle_rectangles_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_rectangle_rectangles_iterator(&self) -> bool {
        has_sym!(self, xcb_poly_rectangle_rectangles_iterator)
    }

    /// Computes the size of a `xcb_poly_arc_request_t` object.
    #[inline]
    pub unsafe fn xcb_poly_arc_sizeof(&self, _buffer: *const c_void, arcs_len: u32) -> c_int {
        sym!(self, xcb_poly_arc_sizeof)(_buffer, arcs_len)
    }

    /// Returns `true` iff the symbol `xcb_poly_arc_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_arc_sizeof(&self) -> bool {
        has_sym!(self, xcb_poly_arc_sizeof)
    }

    /// Sends a `PolyArc` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_poly_arc_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        arcs_len: u32,
        arcs: *const xcb_arc_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_arc_checked)(c, drawable, gc, arcs_len, arcs)
    }

    /// Returns `true` iff the symbol `xcb_poly_arc_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_arc_checked(&self) -> bool {
        has_sym!(self, xcb_poly_arc_checked)
    }

    /// Sends a `PolyArc` request (unchecked).
    #[inline]
    pub unsafe fn xcb_poly_arc(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        arcs_len: u32,
        arcs: *const xcb_arc_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_arc)(c, drawable, gc, arcs_len, arcs)
    }

    /// Returns `true` iff the symbol `xcb_poly_arc` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_arc(&self) -> bool {
        has_sym!(self, xcb_poly_arc)
    }

    /// Returns a pointer to the `arcs` field of a `xcb_poly_arc_request_t` struct.
    #[inline]
    pub unsafe fn xcb_poly_arc_arcs(&self, r: *const xcb_poly_arc_request_t) -> *mut xcb_arc_t {
        sym!(self, xcb_poly_arc_arcs)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_arc_arcs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_arc_arcs(&self) -> bool {
        has_sym!(self, xcb_poly_arc_arcs)
    }

    /// Returns the number of elements of the `arcs` field of a `xcb_poly_arc_request_t` struct.
    #[inline]
    pub unsafe fn xcb_poly_arc_arcs_length(&self, r: *const xcb_poly_arc_request_t) -> c_int {
        sym!(self, xcb_poly_arc_arcs_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_arc_arcs_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_arc_arcs_length(&self) -> bool {
        has_sym!(self, xcb_poly_arc_arcs_length)
    }

    /// Returns an iterator over the elements of the
    /// `arcs` field of a `xcb_poly_arc_request_t` struct.
    #[inline]
    pub unsafe fn xcb_poly_arc_arcs_iterator(
        &self,
        r: *const xcb_poly_arc_request_t,
    ) -> xcb_arc_iterator_t {
        sym!(self, xcb_poly_arc_arcs_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_arc_arcs_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_arc_arcs_iterator(&self) -> bool {
        has_sym!(self, xcb_poly_arc_arcs_iterator)
    }

    /// Computes the size of a `xcb_fill_poly_request_t` object.
    #[inline]
    pub unsafe fn xcb_fill_poly_sizeof(&self, _buffer: *const c_void, points_len: u32) -> c_int {
        sym!(self, xcb_fill_poly_sizeof)(_buffer, points_len)
    }

    /// Returns `true` iff the symbol `xcb_fill_poly_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_fill_poly_sizeof(&self) -> bool {
        has_sym!(self, xcb_fill_poly_sizeof)
    }

    /// Sends a `FillPoly` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_fill_poly_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        shape: u8,
        coordinate_mode: u8,
        points_len: u32,
        points: *const xcb_point_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_fill_poly_checked)(
            c,
            drawable,
            gc,
            shape,
            coordinate_mode,
            points_len,
            points,
        )
    }

    /// Returns `true` iff the symbol `xcb_fill_poly_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_fill_poly_checked(&self) -> bool {
        has_sym!(self, xcb_fill_poly_checked)
    }

    /// Sends a `FillPoly` request (unchecked).
    #[inline]
    pub unsafe fn xcb_fill_poly(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        shape: u8,
        coordinate_mode: u8,
        points_len: u32,
        points: *const xcb_point_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_fill_poly)(c, drawable, gc, shape, coordinate_mode, points_len, points)
    }

    /// Returns `true` iff the symbol `xcb_fill_poly` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_fill_poly(&self) -> bool {
        has_sym!(self, xcb_fill_poly)
    }

    /// Returns a pointer to the `points` field of a `xcb_fill_poly_request_t` struct.
    #[inline]
    pub unsafe fn xcb_fill_poly_points(
        &self,
        r: *const xcb_fill_poly_request_t,
    ) -> *mut xcb_point_t {
        sym!(self, xcb_fill_poly_points)(r)
    }

    /// Returns `true` iff the symbol `xcb_fill_poly_points` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_fill_poly_points(&self) -> bool {
        has_sym!(self, xcb_fill_poly_points)
    }

    /// Returns the number of elements of the `points` field of a `xcb_fill_poly_request_t` struct.
    #[inline]
    pub unsafe fn xcb_fill_poly_points_length(&self, r: *const xcb_fill_poly_request_t) -> c_int {
        sym!(self, xcb_fill_poly_points_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_fill_poly_points_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_fill_poly_points_length(&self) -> bool {
        has_sym!(self, xcb_fill_poly_points_length)
    }

    /// Returns an iterator over the elements of the
    /// `points` field of a `xcb_fill_poly_request_t` struct.
    #[inline]
    pub unsafe fn xcb_fill_poly_points_iterator(
        &self,
        r: *const xcb_fill_poly_request_t,
    ) -> xcb_point_iterator_t {
        sym!(self, xcb_fill_poly_points_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_fill_poly_points_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_fill_poly_points_iterator(&self) -> bool {
        has_sym!(self, xcb_fill_poly_points_iterator)
    }

    /// Computes the size of a `xcb_poly_fill_rectangle_request_t` object.
    #[inline]
    pub unsafe fn xcb_poly_fill_rectangle_sizeof(
        &self,
        _buffer: *const c_void,
        rectangles_len: u32,
    ) -> c_int {
        sym!(self, xcb_poly_fill_rectangle_sizeof)(_buffer, rectangles_len)
    }

    /// Returns `true` iff the symbol `xcb_poly_fill_rectangle_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_fill_rectangle_sizeof(&self) -> bool {
        has_sym!(self, xcb_poly_fill_rectangle_sizeof)
    }

    /// Sends a `PolyFillRectangle` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_poly_fill_rectangle_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_fill_rectangle_checked)(c, drawable, gc, rectangles_len, rectangles)
    }

    /// Returns `true` iff the symbol `xcb_poly_fill_rectangle_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_fill_rectangle_checked(&self) -> bool {
        has_sym!(self, xcb_poly_fill_rectangle_checked)
    }

    /// Sends a `PolyFillRectangle` request (unchecked).
    #[inline]
    pub unsafe fn xcb_poly_fill_rectangle(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_fill_rectangle)(c, drawable, gc, rectangles_len, rectangles)
    }

    /// Returns `true` iff the symbol `xcb_poly_fill_rectangle` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_fill_rectangle(&self) -> bool {
        has_sym!(self, xcb_poly_fill_rectangle)
    }

    /// Returns a pointer to the `rectangles` field of a `xcb_poly_fill_rectangle_request_t` struct.
    #[inline]
    pub unsafe fn xcb_poly_fill_rectangle_rectangles(
        &self,
        r: *const xcb_poly_fill_rectangle_request_t,
    ) -> *mut xcb_rectangle_t {
        sym!(self, xcb_poly_fill_rectangle_rectangles)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_fill_rectangle_rectangles` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_fill_rectangle_rectangles(&self) -> bool {
        has_sym!(self, xcb_poly_fill_rectangle_rectangles)
    }

    /// Returns the number of elements of the `rectangles` field of a `xcb_poly_fill_rectangle_request_t` struct.
    #[inline]
    pub unsafe fn xcb_poly_fill_rectangle_rectangles_length(
        &self,
        r: *const xcb_poly_fill_rectangle_request_t,
    ) -> c_int {
        sym!(self, xcb_poly_fill_rectangle_rectangles_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_fill_rectangle_rectangles_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_fill_rectangle_rectangles_length(&self) -> bool {
        has_sym!(self, xcb_poly_fill_rectangle_rectangles_length)
    }

    /// Returns an iterator over the elements of the
    /// `rectangles` field of a `xcb_poly_fill_rectangle_request_t` struct.
    #[inline]
    pub unsafe fn xcb_poly_fill_rectangle_rectangles_iterator(
        &self,
        r: *const xcb_poly_fill_rectangle_request_t,
    ) -> xcb_rectangle_iterator_t {
        sym!(self, xcb_poly_fill_rectangle_rectangles_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_fill_rectangle_rectangles_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_fill_rectangle_rectangles_iterator(&self) -> bool {
        has_sym!(self, xcb_poly_fill_rectangle_rectangles_iterator)
    }

    /// Computes the size of a `xcb_poly_fill_arc_request_t` object.
    #[inline]
    pub unsafe fn xcb_poly_fill_arc_sizeof(&self, _buffer: *const c_void, arcs_len: u32) -> c_int {
        sym!(self, xcb_poly_fill_arc_sizeof)(_buffer, arcs_len)
    }

    /// Returns `true` iff the symbol `xcb_poly_fill_arc_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_fill_arc_sizeof(&self) -> bool {
        has_sym!(self, xcb_poly_fill_arc_sizeof)
    }

    /// Sends a `PolyFillArc` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_poly_fill_arc_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        arcs_len: u32,
        arcs: *const xcb_arc_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_fill_arc_checked)(c, drawable, gc, arcs_len, arcs)
    }

    /// Returns `true` iff the symbol `xcb_poly_fill_arc_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_fill_arc_checked(&self) -> bool {
        has_sym!(self, xcb_poly_fill_arc_checked)
    }

    /// Sends a `PolyFillArc` request (unchecked).
    #[inline]
    pub unsafe fn xcb_poly_fill_arc(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        arcs_len: u32,
        arcs: *const xcb_arc_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_fill_arc)(c, drawable, gc, arcs_len, arcs)
    }

    /// Returns `true` iff the symbol `xcb_poly_fill_arc` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_fill_arc(&self) -> bool {
        has_sym!(self, xcb_poly_fill_arc)
    }

    /// Returns a pointer to the `arcs` field of a `xcb_poly_fill_arc_request_t` struct.
    #[inline]
    pub unsafe fn xcb_poly_fill_arc_arcs(
        &self,
        r: *const xcb_poly_fill_arc_request_t,
    ) -> *mut xcb_arc_t {
        sym!(self, xcb_poly_fill_arc_arcs)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_fill_arc_arcs` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_fill_arc_arcs(&self) -> bool {
        has_sym!(self, xcb_poly_fill_arc_arcs)
    }

    /// Returns the number of elements of the `arcs` field of a `xcb_poly_fill_arc_request_t` struct.
    #[inline]
    pub unsafe fn xcb_poly_fill_arc_arcs_length(
        &self,
        r: *const xcb_poly_fill_arc_request_t,
    ) -> c_int {
        sym!(self, xcb_poly_fill_arc_arcs_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_fill_arc_arcs_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_fill_arc_arcs_length(&self) -> bool {
        has_sym!(self, xcb_poly_fill_arc_arcs_length)
    }

    /// Returns an iterator over the elements of the
    /// `arcs` field of a `xcb_poly_fill_arc_request_t` struct.
    #[inline]
    pub unsafe fn xcb_poly_fill_arc_arcs_iterator(
        &self,
        r: *const xcb_poly_fill_arc_request_t,
    ) -> xcb_arc_iterator_t {
        sym!(self, xcb_poly_fill_arc_arcs_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_fill_arc_arcs_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_fill_arc_arcs_iterator(&self) -> bool {
        has_sym!(self, xcb_poly_fill_arc_arcs_iterator)
    }

    /// Computes the size of a `xcb_put_image_request_t` object.
    #[inline]
    pub unsafe fn xcb_put_image_sizeof(&self, _buffer: *const c_void, data_len: u32) -> c_int {
        sym!(self, xcb_put_image_sizeof)(_buffer, data_len)
    }

    /// Returns `true` iff the symbol `xcb_put_image_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_put_image_sizeof(&self) -> bool {
        has_sym!(self, xcb_put_image_sizeof)
    }

    /// Sends a `PutImage` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_put_image_checked(
        &self,
        c: *mut xcb_connection_t,
        format: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        width: u16,
        height: u16,
        dst_x: i16,
        dst_y: i16,
        left_pad: u8,
        depth: u8,
        data_len: u32,
        data: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_put_image_checked)(
            c, format, drawable, gc, width, height, dst_x, dst_y, left_pad, depth, data_len, data,
        )
    }

    /// Returns `true` iff the symbol `xcb_put_image_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_put_image_checked(&self) -> bool {
        has_sym!(self, xcb_put_image_checked)
    }

    /// Sends a `PutImage` request (unchecked).
    #[inline]
    pub unsafe fn xcb_put_image(
        &self,
        c: *mut xcb_connection_t,
        format: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        width: u16,
        height: u16,
        dst_x: i16,
        dst_y: i16,
        left_pad: u8,
        depth: u8,
        data_len: u32,
        data: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_put_image)(
            c, format, drawable, gc, width, height, dst_x, dst_y, left_pad, depth, data_len, data,
        )
    }

    /// Returns `true` iff the symbol `xcb_put_image` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_put_image(&self) -> bool {
        has_sym!(self, xcb_put_image)
    }

    /// Returns a pointer to the `data` field of a `xcb_put_image_request_t` struct.
    #[inline]
    pub unsafe fn xcb_put_image_data(&self, r: *const xcb_put_image_request_t) -> *mut u8 {
        sym!(self, xcb_put_image_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_put_image_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_put_image_data(&self) -> bool {
        has_sym!(self, xcb_put_image_data)
    }

    /// Returns the number of elements of the `data` field of a `xcb_put_image_request_t` struct.
    #[inline]
    pub unsafe fn xcb_put_image_data_length(&self, r: *const xcb_put_image_request_t) -> c_int {
        sym!(self, xcb_put_image_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_put_image_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_put_image_data_length(&self) -> bool {
        has_sym!(self, xcb_put_image_data_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `data` field of a `xcb_put_image_request_t` struct.
    #[inline]
    pub unsafe fn xcb_put_image_data_end(
        &self,
        r: *const xcb_put_image_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_put_image_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_put_image_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_put_image_data_end(&self) -> bool {
        has_sym!(self, xcb_put_image_data_end)
    }

    /// Computes the size of a `xcb_get_image_reply_t` object.
    #[inline]
    pub unsafe fn xcb_get_image_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_get_image_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_get_image_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_image_sizeof(&self) -> bool {
        has_sym!(self, xcb_get_image_sizeof)
    }

    /// Sends a `GetImage` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_image_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_image_reply`]: Self::xcb_get_image_reply
    #[inline]
    pub unsafe fn xcb_get_image(
        &self,
        c: *mut xcb_connection_t,
        format: u8,
        drawable: xcb_drawable_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        plane_mask: u32,
    ) -> xcb_get_image_cookie_t {
        sym!(self, xcb_get_image)(c, format, drawable, x, y, width, height, plane_mask)
    }

    /// Returns `true` iff the symbol `xcb_get_image` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_image(&self) -> bool {
        has_sym!(self, xcb_get_image)
    }

    /// Sends a `GetImage` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_image_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_image_reply`]: Self::xcb_get_image_reply
    #[inline]
    pub unsafe fn xcb_get_image_unchecked(
        &self,
        c: *mut xcb_connection_t,
        format: u8,
        drawable: xcb_drawable_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        plane_mask: u32,
    ) -> xcb_get_image_cookie_t {
        sym!(self, xcb_get_image_unchecked)(c, format, drawable, x, y, width, height, plane_mask)
    }

    /// Returns `true` iff the symbol `xcb_get_image_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_image_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_image_unchecked)
    }

    /// Returns a pointer to the `data` field of a `xcb_get_image_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_get_image_data(&self, r: *const xcb_get_image_reply_t) -> *mut u8 {
        sym!(self, xcb_get_image_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_image_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_image_data(&self) -> bool {
        has_sym!(self, xcb_get_image_data)
    }

    /// Returns the number of elements of the `data` field of a `xcb_get_image_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_get_image_data_length(&self, r: *const xcb_get_image_reply_t) -> c_int {
        sym!(self, xcb_get_image_data_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_image_data_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_image_data_length(&self) -> bool {
        has_sym!(self, xcb_get_image_data_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `data` field of a `xcb_get_image_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_get_image_data_end(
        &self,
        r: *const xcb_get_image_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_get_image_data_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_image_data_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_image_data_end(&self) -> bool {
        has_sym!(self, xcb_get_image_data_end)
    }

    /// Waits for the reply to a `GetImage` request.
    #[inline]
    pub unsafe fn xcb_get_image_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_image_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_image_reply_t {
        sym!(self, xcb_get_image_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_image_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_image_reply(&self) -> bool {
        has_sym!(self, xcb_get_image_reply)
    }

    /// Computes the size of a `xcb_poly_text_8_request_t` object.
    #[inline]
    pub unsafe fn xcb_poly_text_8_sizeof(&self, _buffer: *const c_void, items_len: u32) -> c_int {
        sym!(self, xcb_poly_text_8_sizeof)(_buffer, items_len)
    }

    /// Returns `true` iff the symbol `xcb_poly_text_8_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_text_8_sizeof(&self) -> bool {
        has_sym!(self, xcb_poly_text_8_sizeof)
    }

    /// Sends a `PolyText8` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_poly_text_8_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: i16,
        y: i16,
        items_len: u32,
        items: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_text_8_checked)(c, drawable, gc, x, y, items_len, items)
    }

    /// Returns `true` iff the symbol `xcb_poly_text_8_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_text_8_checked(&self) -> bool {
        has_sym!(self, xcb_poly_text_8_checked)
    }

    /// Sends a `PolyText8` request (unchecked).
    #[inline]
    pub unsafe fn xcb_poly_text_8(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: i16,
        y: i16,
        items_len: u32,
        items: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_text_8)(c, drawable, gc, x, y, items_len, items)
    }

    /// Returns `true` iff the symbol `xcb_poly_text_8` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_text_8(&self) -> bool {
        has_sym!(self, xcb_poly_text_8)
    }

    /// Returns a pointer to the `items` field of a `xcb_poly_text_8_request_t` struct.
    #[inline]
    pub unsafe fn xcb_poly_text_8_items(&self, r: *const xcb_poly_text_8_request_t) -> *mut u8 {
        sym!(self, xcb_poly_text_8_items)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_text_8_items` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_text_8_items(&self) -> bool {
        has_sym!(self, xcb_poly_text_8_items)
    }

    /// Returns the number of elements of the `items` field of a `xcb_poly_text_8_request_t` struct.
    #[inline]
    pub unsafe fn xcb_poly_text_8_items_length(
        &self,
        r: *const xcb_poly_text_8_request_t,
    ) -> c_int {
        sym!(self, xcb_poly_text_8_items_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_text_8_items_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_text_8_items_length(&self) -> bool {
        has_sym!(self, xcb_poly_text_8_items_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `items` field of a `xcb_poly_text_8_request_t` struct.
    #[inline]
    pub unsafe fn xcb_poly_text_8_items_end(
        &self,
        r: *const xcb_poly_text_8_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_poly_text_8_items_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_text_8_items_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_text_8_items_end(&self) -> bool {
        has_sym!(self, xcb_poly_text_8_items_end)
    }

    /// Computes the size of a `xcb_poly_text_16_request_t` object.
    #[inline]
    pub unsafe fn xcb_poly_text_16_sizeof(&self, _buffer: *const c_void, items_len: u32) -> c_int {
        sym!(self, xcb_poly_text_16_sizeof)(_buffer, items_len)
    }

    /// Returns `true` iff the symbol `xcb_poly_text_16_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_text_16_sizeof(&self) -> bool {
        has_sym!(self, xcb_poly_text_16_sizeof)
    }

    /// Sends a `PolyText16` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_poly_text_16_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: i16,
        y: i16,
        items_len: u32,
        items: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_text_16_checked)(c, drawable, gc, x, y, items_len, items)
    }

    /// Returns `true` iff the symbol `xcb_poly_text_16_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_text_16_checked(&self) -> bool {
        has_sym!(self, xcb_poly_text_16_checked)
    }

    /// Sends a `PolyText16` request (unchecked).
    #[inline]
    pub unsafe fn xcb_poly_text_16(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: i16,
        y: i16,
        items_len: u32,
        items: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_poly_text_16)(c, drawable, gc, x, y, items_len, items)
    }

    /// Returns `true` iff the symbol `xcb_poly_text_16` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_text_16(&self) -> bool {
        has_sym!(self, xcb_poly_text_16)
    }

    /// Returns a pointer to the `items` field of a `xcb_poly_text_16_request_t` struct.
    #[inline]
    pub unsafe fn xcb_poly_text_16_items(&self, r: *const xcb_poly_text_16_request_t) -> *mut u8 {
        sym!(self, xcb_poly_text_16_items)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_text_16_items` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_text_16_items(&self) -> bool {
        has_sym!(self, xcb_poly_text_16_items)
    }

    /// Returns the number of elements of the `items` field of a `xcb_poly_text_16_request_t` struct.
    #[inline]
    pub unsafe fn xcb_poly_text_16_items_length(
        &self,
        r: *const xcb_poly_text_16_request_t,
    ) -> c_int {
        sym!(self, xcb_poly_text_16_items_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_text_16_items_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_text_16_items_length(&self) -> bool {
        has_sym!(self, xcb_poly_text_16_items_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `items` field of a `xcb_poly_text_16_request_t` struct.
    #[inline]
    pub unsafe fn xcb_poly_text_16_items_end(
        &self,
        r: *const xcb_poly_text_16_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_poly_text_16_items_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_poly_text_16_items_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_poly_text_16_items_end(&self) -> bool {
        has_sym!(self, xcb_poly_text_16_items_end)
    }

    /// Computes the size of a `xcb_image_text_8_request_t` object.
    #[inline]
    pub unsafe fn xcb_image_text_8_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_image_text_8_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_image_text_8_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_image_text_8_sizeof(&self) -> bool {
        has_sym!(self, xcb_image_text_8_sizeof)
    }

    /// Sends a `ImageText8` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_image_text_8_checked(
        &self,
        c: *mut xcb_connection_t,
        string_len: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: i16,
        y: i16,
        string: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_image_text_8_checked)(c, string_len, drawable, gc, x, y, string)
    }

    /// Returns `true` iff the symbol `xcb_image_text_8_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_image_text_8_checked(&self) -> bool {
        has_sym!(self, xcb_image_text_8_checked)
    }

    /// Sends a `ImageText8` request (unchecked).
    #[inline]
    pub unsafe fn xcb_image_text_8(
        &self,
        c: *mut xcb_connection_t,
        string_len: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: i16,
        y: i16,
        string: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_image_text_8)(c, string_len, drawable, gc, x, y, string)
    }

    /// Returns `true` iff the symbol `xcb_image_text_8` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_image_text_8(&self) -> bool {
        has_sym!(self, xcb_image_text_8)
    }

    /// Returns a pointer to the `string` field of a `xcb_image_text_8_request_t` struct.
    #[inline]
    pub unsafe fn xcb_image_text_8_string(
        &self,
        r: *const xcb_image_text_8_request_t,
    ) -> *mut c_char {
        sym!(self, xcb_image_text_8_string)(r)
    }

    /// Returns `true` iff the symbol `xcb_image_text_8_string` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_image_text_8_string(&self) -> bool {
        has_sym!(self, xcb_image_text_8_string)
    }

    /// Returns the number of elements of the `string` field of a `xcb_image_text_8_request_t` struct.
    #[inline]
    pub unsafe fn xcb_image_text_8_string_length(
        &self,
        r: *const xcb_image_text_8_request_t,
    ) -> c_int {
        sym!(self, xcb_image_text_8_string_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_image_text_8_string_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_image_text_8_string_length(&self) -> bool {
        has_sym!(self, xcb_image_text_8_string_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `string` field of a `xcb_image_text_8_request_t` struct.
    #[inline]
    pub unsafe fn xcb_image_text_8_string_end(
        &self,
        r: *const xcb_image_text_8_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_image_text_8_string_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_image_text_8_string_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_image_text_8_string_end(&self) -> bool {
        has_sym!(self, xcb_image_text_8_string_end)
    }

    /// Computes the size of a `xcb_image_text_16_request_t` object.
    #[inline]
    pub unsafe fn xcb_image_text_16_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_image_text_16_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_image_text_16_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_image_text_16_sizeof(&self) -> bool {
        has_sym!(self, xcb_image_text_16_sizeof)
    }

    /// Sends a `ImageText16` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_image_text_16_checked(
        &self,
        c: *mut xcb_connection_t,
        string_len: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: i16,
        y: i16,
        string: *const xcb_char2b_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_image_text_16_checked)(c, string_len, drawable, gc, x, y, string)
    }

    /// Returns `true` iff the symbol `xcb_image_text_16_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_image_text_16_checked(&self) -> bool {
        has_sym!(self, xcb_image_text_16_checked)
    }

    /// Sends a `ImageText16` request (unchecked).
    #[inline]
    pub unsafe fn xcb_image_text_16(
        &self,
        c: *mut xcb_connection_t,
        string_len: u8,
        drawable: xcb_drawable_t,
        gc: xcb_gcontext_t,
        x: i16,
        y: i16,
        string: *const xcb_char2b_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_image_text_16)(c, string_len, drawable, gc, x, y, string)
    }

    /// Returns `true` iff the symbol `xcb_image_text_16` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_image_text_16(&self) -> bool {
        has_sym!(self, xcb_image_text_16)
    }

    /// Returns a pointer to the `string` field of a `xcb_image_text_16_request_t` struct.
    #[inline]
    pub unsafe fn xcb_image_text_16_string(
        &self,
        r: *const xcb_image_text_16_request_t,
    ) -> *mut xcb_char2b_t {
        sym!(self, xcb_image_text_16_string)(r)
    }

    /// Returns `true` iff the symbol `xcb_image_text_16_string` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_image_text_16_string(&self) -> bool {
        has_sym!(self, xcb_image_text_16_string)
    }

    /// Returns the number of elements of the `string` field of a `xcb_image_text_16_request_t` struct.
    #[inline]
    pub unsafe fn xcb_image_text_16_string_length(
        &self,
        r: *const xcb_image_text_16_request_t,
    ) -> c_int {
        sym!(self, xcb_image_text_16_string_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_image_text_16_string_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_image_text_16_string_length(&self) -> bool {
        has_sym!(self, xcb_image_text_16_string_length)
    }

    /// Returns an iterator over the elements of the
    /// `string` field of a `xcb_image_text_16_request_t` struct.
    #[inline]
    pub unsafe fn xcb_image_text_16_string_iterator(
        &self,
        r: *const xcb_image_text_16_request_t,
    ) -> xcb_char2b_iterator_t {
        sym!(self, xcb_image_text_16_string_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_image_text_16_string_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_image_text_16_string_iterator(&self) -> bool {
        has_sym!(self, xcb_image_text_16_string_iterator)
    }

    /// Sends a `CreateColormap` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_create_colormap_checked(
        &self,
        c: *mut xcb_connection_t,
        alloc: u8,
        mid: xcb_colormap_t,
        window: xcb_window_t,
        visual: xcb_visualid_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_colormap_checked)(c, alloc, mid, window, visual)
    }

    /// Returns `true` iff the symbol `xcb_create_colormap_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_colormap_checked(&self) -> bool {
        has_sym!(self, xcb_create_colormap_checked)
    }

    /// Sends a `CreateColormap` request (unchecked).
    #[inline]
    pub unsafe fn xcb_create_colormap(
        &self,
        c: *mut xcb_connection_t,
        alloc: u8,
        mid: xcb_colormap_t,
        window: xcb_window_t,
        visual: xcb_visualid_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_colormap)(c, alloc, mid, window, visual)
    }

    /// Returns `true` iff the symbol `xcb_create_colormap` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_colormap(&self) -> bool {
        has_sym!(self, xcb_create_colormap)
    }

    /// Sends a `FreeColormap` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_free_colormap_checked(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_free_colormap_checked)(c, cmap)
    }

    /// Returns `true` iff the symbol `xcb_free_colormap_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_free_colormap_checked(&self) -> bool {
        has_sym!(self, xcb_free_colormap_checked)
    }

    /// Sends a `FreeColormap` request (unchecked).
    #[inline]
    pub unsafe fn xcb_free_colormap(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_free_colormap)(c, cmap)
    }

    /// Returns `true` iff the symbol `xcb_free_colormap` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_free_colormap(&self) -> bool {
        has_sym!(self, xcb_free_colormap)
    }

    /// Sends a `CopyColormapAndFree` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_copy_colormap_and_free_checked(
        &self,
        c: *mut xcb_connection_t,
        mid: xcb_colormap_t,
        src_cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_copy_colormap_and_free_checked)(c, mid, src_cmap)
    }

    /// Returns `true` iff the symbol `xcb_copy_colormap_and_free_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_copy_colormap_and_free_checked(&self) -> bool {
        has_sym!(self, xcb_copy_colormap_and_free_checked)
    }

    /// Sends a `CopyColormapAndFree` request (unchecked).
    #[inline]
    pub unsafe fn xcb_copy_colormap_and_free(
        &self,
        c: *mut xcb_connection_t,
        mid: xcb_colormap_t,
        src_cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_copy_colormap_and_free)(c, mid, src_cmap)
    }

    /// Returns `true` iff the symbol `xcb_copy_colormap_and_free` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_copy_colormap_and_free(&self) -> bool {
        has_sym!(self, xcb_copy_colormap_and_free)
    }

    /// Sends a `InstallColormap` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_install_colormap_checked(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_install_colormap_checked)(c, cmap)
    }

    /// Returns `true` iff the symbol `xcb_install_colormap_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_install_colormap_checked(&self) -> bool {
        has_sym!(self, xcb_install_colormap_checked)
    }

    /// Sends a `InstallColormap` request (unchecked).
    #[inline]
    pub unsafe fn xcb_install_colormap(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_install_colormap)(c, cmap)
    }

    /// Returns `true` iff the symbol `xcb_install_colormap` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_install_colormap(&self) -> bool {
        has_sym!(self, xcb_install_colormap)
    }

    /// Sends a `UninstallColormap` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_uninstall_colormap_checked(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_uninstall_colormap_checked)(c, cmap)
    }

    /// Returns `true` iff the symbol `xcb_uninstall_colormap_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_uninstall_colormap_checked(&self) -> bool {
        has_sym!(self, xcb_uninstall_colormap_checked)
    }

    /// Sends a `UninstallColormap` request (unchecked).
    #[inline]
    pub unsafe fn xcb_uninstall_colormap(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_uninstall_colormap)(c, cmap)
    }

    /// Returns `true` iff the symbol `xcb_uninstall_colormap` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_uninstall_colormap(&self) -> bool {
        has_sym!(self, xcb_uninstall_colormap)
    }

    /// Computes the size of a `xcb_list_installed_colormaps_reply_t` object.
    #[inline]
    pub unsafe fn xcb_list_installed_colormaps_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_list_installed_colormaps_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_list_installed_colormaps_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_installed_colormaps_sizeof(&self) -> bool {
        has_sym!(self, xcb_list_installed_colormaps_sizeof)
    }

    /// Sends a `ListInstalledColormaps` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_list_installed_colormaps_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_list_installed_colormaps_reply`]: Self::xcb_list_installed_colormaps_reply
    #[inline]
    pub unsafe fn xcb_list_installed_colormaps(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_list_installed_colormaps_cookie_t {
        sym!(self, xcb_list_installed_colormaps)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_list_installed_colormaps` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_installed_colormaps(&self) -> bool {
        has_sym!(self, xcb_list_installed_colormaps)
    }

    /// Sends a `ListInstalledColormaps` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_list_installed_colormaps_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_list_installed_colormaps_reply`]: Self::xcb_list_installed_colormaps_reply
    #[inline]
    pub unsafe fn xcb_list_installed_colormaps_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_list_installed_colormaps_cookie_t {
        sym!(self, xcb_list_installed_colormaps_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_list_installed_colormaps_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_installed_colormaps_unchecked(&self) -> bool {
        has_sym!(self, xcb_list_installed_colormaps_unchecked)
    }

    /// Returns a pointer to the `cmaps` field of a `xcb_list_installed_colormaps_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_list_installed_colormaps_cmaps(
        &self,
        r: *const xcb_list_installed_colormaps_reply_t,
    ) -> *mut xcb_colormap_t {
        sym!(self, xcb_list_installed_colormaps_cmaps)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_installed_colormaps_cmaps` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_installed_colormaps_cmaps(&self) -> bool {
        has_sym!(self, xcb_list_installed_colormaps_cmaps)
    }

    /// Returns the number of elements of the `cmaps` field of a `xcb_list_installed_colormaps_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_list_installed_colormaps_cmaps_length(
        &self,
        r: *const xcb_list_installed_colormaps_reply_t,
    ) -> c_int {
        sym!(self, xcb_list_installed_colormaps_cmaps_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_installed_colormaps_cmaps_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_installed_colormaps_cmaps_length(&self) -> bool {
        has_sym!(self, xcb_list_installed_colormaps_cmaps_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `cmaps` field of a `xcb_list_installed_colormaps_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_list_installed_colormaps_cmaps_end(
        &self,
        r: *const xcb_list_installed_colormaps_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_list_installed_colormaps_cmaps_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_installed_colormaps_cmaps_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_installed_colormaps_cmaps_end(&self) -> bool {
        has_sym!(self, xcb_list_installed_colormaps_cmaps_end)
    }

    /// Waits for the reply to a `ListInstalledColormaps` request.
    #[inline]
    pub unsafe fn xcb_list_installed_colormaps_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_list_installed_colormaps_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_list_installed_colormaps_reply_t {
        sym!(self, xcb_list_installed_colormaps_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_list_installed_colormaps_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_installed_colormaps_reply(&self) -> bool {
        has_sym!(self, xcb_list_installed_colormaps_reply)
    }

    /// Sends a `AllocColor` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_alloc_color_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_alloc_color_reply`]: Self::xcb_alloc_color_reply
    #[inline]
    pub unsafe fn xcb_alloc_color(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        red: u16,
        green: u16,
        blue: u16,
    ) -> xcb_alloc_color_cookie_t {
        sym!(self, xcb_alloc_color)(c, cmap, red, green, blue)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color(&self) -> bool {
        has_sym!(self, xcb_alloc_color)
    }

    /// Sends a `AllocColor` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_alloc_color_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_alloc_color_reply`]: Self::xcb_alloc_color_reply
    #[inline]
    pub unsafe fn xcb_alloc_color_unchecked(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        red: u16,
        green: u16,
        blue: u16,
    ) -> xcb_alloc_color_cookie_t {
        sym!(self, xcb_alloc_color_unchecked)(c, cmap, red, green, blue)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_unchecked(&self) -> bool {
        has_sym!(self, xcb_alloc_color_unchecked)
    }

    /// Waits for the reply to a `AllocColor` request.
    #[inline]
    pub unsafe fn xcb_alloc_color_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_alloc_color_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_alloc_color_reply_t {
        sym!(self, xcb_alloc_color_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_reply(&self) -> bool {
        has_sym!(self, xcb_alloc_color_reply)
    }

    /// Computes the size of a `xcb_alloc_named_color_request_t` object.
    #[inline]
    pub unsafe fn xcb_alloc_named_color_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_alloc_named_color_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_alloc_named_color_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_named_color_sizeof(&self) -> bool {
        has_sym!(self, xcb_alloc_named_color_sizeof)
    }

    /// Sends a `AllocNamedColor` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_alloc_named_color_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_alloc_named_color_reply`]: Self::xcb_alloc_named_color_reply
    #[inline]
    pub unsafe fn xcb_alloc_named_color(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        name_len: u16,
        name: *const c_char,
    ) -> xcb_alloc_named_color_cookie_t {
        sym!(self, xcb_alloc_named_color)(c, cmap, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_alloc_named_color` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_named_color(&self) -> bool {
        has_sym!(self, xcb_alloc_named_color)
    }

    /// Sends a `AllocNamedColor` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_alloc_named_color_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_alloc_named_color_reply`]: Self::xcb_alloc_named_color_reply
    #[inline]
    pub unsafe fn xcb_alloc_named_color_unchecked(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        name_len: u16,
        name: *const c_char,
    ) -> xcb_alloc_named_color_cookie_t {
        sym!(self, xcb_alloc_named_color_unchecked)(c, cmap, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_alloc_named_color_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_named_color_unchecked(&self) -> bool {
        has_sym!(self, xcb_alloc_named_color_unchecked)
    }

    /// Waits for the reply to a `AllocNamedColor` request.
    #[inline]
    pub unsafe fn xcb_alloc_named_color_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_alloc_named_color_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_alloc_named_color_reply_t {
        sym!(self, xcb_alloc_named_color_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_alloc_named_color_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_named_color_reply(&self) -> bool {
        has_sym!(self, xcb_alloc_named_color_reply)
    }

    /// Computes the size of a `xcb_alloc_color_cells_reply_t` object.
    #[inline]
    pub unsafe fn xcb_alloc_color_cells_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_alloc_color_cells_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_cells_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_cells_sizeof(&self) -> bool {
        has_sym!(self, xcb_alloc_color_cells_sizeof)
    }

    /// Sends a `AllocColorCells` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_alloc_color_cells_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_alloc_color_cells_reply`]: Self::xcb_alloc_color_cells_reply
    #[inline]
    pub unsafe fn xcb_alloc_color_cells(
        &self,
        c: *mut xcb_connection_t,
        contiguous: u8,
        cmap: xcb_colormap_t,
        colors: u16,
        planes: u16,
    ) -> xcb_alloc_color_cells_cookie_t {
        sym!(self, xcb_alloc_color_cells)(c, contiguous, cmap, colors, planes)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_cells` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_cells(&self) -> bool {
        has_sym!(self, xcb_alloc_color_cells)
    }

    /// Sends a `AllocColorCells` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_alloc_color_cells_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_alloc_color_cells_reply`]: Self::xcb_alloc_color_cells_reply
    #[inline]
    pub unsafe fn xcb_alloc_color_cells_unchecked(
        &self,
        c: *mut xcb_connection_t,
        contiguous: u8,
        cmap: xcb_colormap_t,
        colors: u16,
        planes: u16,
    ) -> xcb_alloc_color_cells_cookie_t {
        sym!(self, xcb_alloc_color_cells_unchecked)(c, contiguous, cmap, colors, planes)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_cells_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_cells_unchecked(&self) -> bool {
        has_sym!(self, xcb_alloc_color_cells_unchecked)
    }

    /// Returns a pointer to the `pixels` field of a `xcb_alloc_color_cells_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_alloc_color_cells_pixels(
        &self,
        r: *const xcb_alloc_color_cells_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_alloc_color_cells_pixels)(r)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_cells_pixels` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_cells_pixels(&self) -> bool {
        has_sym!(self, xcb_alloc_color_cells_pixels)
    }

    /// Returns the number of elements of the `pixels` field of a `xcb_alloc_color_cells_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_alloc_color_cells_pixels_length(
        &self,
        r: *const xcb_alloc_color_cells_reply_t,
    ) -> c_int {
        sym!(self, xcb_alloc_color_cells_pixels_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_cells_pixels_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_cells_pixels_length(&self) -> bool {
        has_sym!(self, xcb_alloc_color_cells_pixels_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `pixels` field of a `xcb_alloc_color_cells_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_alloc_color_cells_pixels_end(
        &self,
        r: *const xcb_alloc_color_cells_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_alloc_color_cells_pixels_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_cells_pixels_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_cells_pixels_end(&self) -> bool {
        has_sym!(self, xcb_alloc_color_cells_pixels_end)
    }

    /// Returns a pointer to the `masks` field of a `xcb_alloc_color_cells_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_alloc_color_cells_masks(
        &self,
        r: *const xcb_alloc_color_cells_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_alloc_color_cells_masks)(r)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_cells_masks` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_cells_masks(&self) -> bool {
        has_sym!(self, xcb_alloc_color_cells_masks)
    }

    /// Returns the number of elements of the `masks` field of a `xcb_alloc_color_cells_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_alloc_color_cells_masks_length(
        &self,
        r: *const xcb_alloc_color_cells_reply_t,
    ) -> c_int {
        sym!(self, xcb_alloc_color_cells_masks_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_cells_masks_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_cells_masks_length(&self) -> bool {
        has_sym!(self, xcb_alloc_color_cells_masks_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `masks` field of a `xcb_alloc_color_cells_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_alloc_color_cells_masks_end(
        &self,
        r: *const xcb_alloc_color_cells_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_alloc_color_cells_masks_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_cells_masks_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_cells_masks_end(&self) -> bool {
        has_sym!(self, xcb_alloc_color_cells_masks_end)
    }

    /// Waits for the reply to a `AllocColorCells` request.
    #[inline]
    pub unsafe fn xcb_alloc_color_cells_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_alloc_color_cells_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_alloc_color_cells_reply_t {
        sym!(self, xcb_alloc_color_cells_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_cells_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_cells_reply(&self) -> bool {
        has_sym!(self, xcb_alloc_color_cells_reply)
    }

    /// Computes the size of a `xcb_alloc_color_planes_reply_t` object.
    #[inline]
    pub unsafe fn xcb_alloc_color_planes_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_alloc_color_planes_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_planes_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_planes_sizeof(&self) -> bool {
        has_sym!(self, xcb_alloc_color_planes_sizeof)
    }

    /// Sends a `AllocColorPlanes` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_alloc_color_planes_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_alloc_color_planes_reply`]: Self::xcb_alloc_color_planes_reply
    #[inline]
    pub unsafe fn xcb_alloc_color_planes(
        &self,
        c: *mut xcb_connection_t,
        contiguous: u8,
        cmap: xcb_colormap_t,
        colors: u16,
        reds: u16,
        greens: u16,
        blues: u16,
    ) -> xcb_alloc_color_planes_cookie_t {
        sym!(self, xcb_alloc_color_planes)(c, contiguous, cmap, colors, reds, greens, blues)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_planes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_planes(&self) -> bool {
        has_sym!(self, xcb_alloc_color_planes)
    }

    /// Sends a `AllocColorPlanes` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_alloc_color_planes_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_alloc_color_planes_reply`]: Self::xcb_alloc_color_planes_reply
    #[inline]
    pub unsafe fn xcb_alloc_color_planes_unchecked(
        &self,
        c: *mut xcb_connection_t,
        contiguous: u8,
        cmap: xcb_colormap_t,
        colors: u16,
        reds: u16,
        greens: u16,
        blues: u16,
    ) -> xcb_alloc_color_planes_cookie_t {
        sym!(self, xcb_alloc_color_planes_unchecked)(
            c, contiguous, cmap, colors, reds, greens, blues,
        )
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_planes_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_planes_unchecked(&self) -> bool {
        has_sym!(self, xcb_alloc_color_planes_unchecked)
    }

    /// Returns a pointer to the `pixels` field of a `xcb_alloc_color_planes_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_alloc_color_planes_pixels(
        &self,
        r: *const xcb_alloc_color_planes_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_alloc_color_planes_pixels)(r)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_planes_pixels` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_planes_pixels(&self) -> bool {
        has_sym!(self, xcb_alloc_color_planes_pixels)
    }

    /// Returns the number of elements of the `pixels` field of a `xcb_alloc_color_planes_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_alloc_color_planes_pixels_length(
        &self,
        r: *const xcb_alloc_color_planes_reply_t,
    ) -> c_int {
        sym!(self, xcb_alloc_color_planes_pixels_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_planes_pixels_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_planes_pixels_length(&self) -> bool {
        has_sym!(self, xcb_alloc_color_planes_pixels_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `pixels` field of a `xcb_alloc_color_planes_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_alloc_color_planes_pixels_end(
        &self,
        r: *const xcb_alloc_color_planes_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_alloc_color_planes_pixels_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_planes_pixels_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_planes_pixels_end(&self) -> bool {
        has_sym!(self, xcb_alloc_color_planes_pixels_end)
    }

    /// Waits for the reply to a `AllocColorPlanes` request.
    #[inline]
    pub unsafe fn xcb_alloc_color_planes_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_alloc_color_planes_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_alloc_color_planes_reply_t {
        sym!(self, xcb_alloc_color_planes_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_alloc_color_planes_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_alloc_color_planes_reply(&self) -> bool {
        has_sym!(self, xcb_alloc_color_planes_reply)
    }

    /// Computes the size of a `xcb_free_colors_request_t` object.
    #[inline]
    pub unsafe fn xcb_free_colors_sizeof(&self, _buffer: *const c_void, pixels_len: u32) -> c_int {
        sym!(self, xcb_free_colors_sizeof)(_buffer, pixels_len)
    }

    /// Returns `true` iff the symbol `xcb_free_colors_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_free_colors_sizeof(&self) -> bool {
        has_sym!(self, xcb_free_colors_sizeof)
    }

    /// Sends a `FreeColors` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_free_colors_checked(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        plane_mask: u32,
        pixels_len: u32,
        pixels: *const u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_free_colors_checked)(c, cmap, plane_mask, pixels_len, pixels)
    }

    /// Returns `true` iff the symbol `xcb_free_colors_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_free_colors_checked(&self) -> bool {
        has_sym!(self, xcb_free_colors_checked)
    }

    /// Sends a `FreeColors` request (unchecked).
    #[inline]
    pub unsafe fn xcb_free_colors(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        plane_mask: u32,
        pixels_len: u32,
        pixels: *const u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_free_colors)(c, cmap, plane_mask, pixels_len, pixels)
    }

    /// Returns `true` iff the symbol `xcb_free_colors` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_free_colors(&self) -> bool {
        has_sym!(self, xcb_free_colors)
    }

    /// Returns a pointer to the `pixels` field of a `xcb_free_colors_request_t` struct.
    #[inline]
    pub unsafe fn xcb_free_colors_pixels(&self, r: *const xcb_free_colors_request_t) -> *mut u32 {
        sym!(self, xcb_free_colors_pixels)(r)
    }

    /// Returns `true` iff the symbol `xcb_free_colors_pixels` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_free_colors_pixels(&self) -> bool {
        has_sym!(self, xcb_free_colors_pixels)
    }

    /// Returns the number of elements of the `pixels` field of a `xcb_free_colors_request_t` struct.
    #[inline]
    pub unsafe fn xcb_free_colors_pixels_length(
        &self,
        r: *const xcb_free_colors_request_t,
    ) -> c_int {
        sym!(self, xcb_free_colors_pixels_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_free_colors_pixels_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_free_colors_pixels_length(&self) -> bool {
        has_sym!(self, xcb_free_colors_pixels_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `pixels` field of a `xcb_free_colors_request_t` struct.
    #[inline]
    pub unsafe fn xcb_free_colors_pixels_end(
        &self,
        r: *const xcb_free_colors_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_free_colors_pixels_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_free_colors_pixels_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_free_colors_pixels_end(&self) -> bool {
        has_sym!(self, xcb_free_colors_pixels_end)
    }

    /// Advances a `xcb_coloritem_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_coloritem_next(&self, i: *mut xcb_coloritem_iterator_t) {
        sym!(self, xcb_coloritem_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_coloritem_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_coloritem_next(&self) -> bool {
        has_sym!(self, xcb_coloritem_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_coloritem_iterator_t`.
    #[inline]
    pub unsafe fn xcb_coloritem_end(&self, i: xcb_coloritem_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_coloritem_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_coloritem_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_coloritem_end(&self) -> bool {
        has_sym!(self, xcb_coloritem_end)
    }

    /// Computes the size of a `xcb_store_colors_request_t` object.
    #[inline]
    pub unsafe fn xcb_store_colors_sizeof(&self, _buffer: *const c_void, items_len: u32) -> c_int {
        sym!(self, xcb_store_colors_sizeof)(_buffer, items_len)
    }

    /// Returns `true` iff the symbol `xcb_store_colors_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_store_colors_sizeof(&self) -> bool {
        has_sym!(self, xcb_store_colors_sizeof)
    }

    /// Sends a `StoreColors` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_store_colors_checked(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        items_len: u32,
        items: *const xcb_coloritem_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_store_colors_checked)(c, cmap, items_len, items)
    }

    /// Returns `true` iff the symbol `xcb_store_colors_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_store_colors_checked(&self) -> bool {
        has_sym!(self, xcb_store_colors_checked)
    }

    /// Sends a `StoreColors` request (unchecked).
    #[inline]
    pub unsafe fn xcb_store_colors(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        items_len: u32,
        items: *const xcb_coloritem_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_store_colors)(c, cmap, items_len, items)
    }

    /// Returns `true` iff the symbol `xcb_store_colors` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_store_colors(&self) -> bool {
        has_sym!(self, xcb_store_colors)
    }

    /// Returns a pointer to the `items` field of a `xcb_store_colors_request_t` struct.
    #[inline]
    pub unsafe fn xcb_store_colors_items(
        &self,
        r: *const xcb_store_colors_request_t,
    ) -> *mut xcb_coloritem_t {
        sym!(self, xcb_store_colors_items)(r)
    }

    /// Returns `true` iff the symbol `xcb_store_colors_items` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_store_colors_items(&self) -> bool {
        has_sym!(self, xcb_store_colors_items)
    }

    /// Returns the number of elements of the `items` field of a `xcb_store_colors_request_t` struct.
    #[inline]
    pub unsafe fn xcb_store_colors_items_length(
        &self,
        r: *const xcb_store_colors_request_t,
    ) -> c_int {
        sym!(self, xcb_store_colors_items_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_store_colors_items_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_store_colors_items_length(&self) -> bool {
        has_sym!(self, xcb_store_colors_items_length)
    }

    /// Returns an iterator over the elements of the
    /// `items` field of a `xcb_store_colors_request_t` struct.
    #[inline]
    pub unsafe fn xcb_store_colors_items_iterator(
        &self,
        r: *const xcb_store_colors_request_t,
    ) -> xcb_coloritem_iterator_t {
        sym!(self, xcb_store_colors_items_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_store_colors_items_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_store_colors_items_iterator(&self) -> bool {
        has_sym!(self, xcb_store_colors_items_iterator)
    }

    /// Computes the size of a `xcb_store_named_color_request_t` object.
    #[inline]
    pub unsafe fn xcb_store_named_color_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_store_named_color_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_store_named_color_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_store_named_color_sizeof(&self) -> bool {
        has_sym!(self, xcb_store_named_color_sizeof)
    }

    /// Sends a `StoreNamedColor` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_store_named_color_checked(
        &self,
        c: *mut xcb_connection_t,
        flags: u8,
        cmap: xcb_colormap_t,
        pixel: u32,
        name_len: u16,
        name: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_store_named_color_checked)(c, flags, cmap, pixel, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_store_named_color_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_store_named_color_checked(&self) -> bool {
        has_sym!(self, xcb_store_named_color_checked)
    }

    /// Sends a `StoreNamedColor` request (unchecked).
    #[inline]
    pub unsafe fn xcb_store_named_color(
        &self,
        c: *mut xcb_connection_t,
        flags: u8,
        cmap: xcb_colormap_t,
        pixel: u32,
        name_len: u16,
        name: *const c_char,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_store_named_color)(c, flags, cmap, pixel, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_store_named_color` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_store_named_color(&self) -> bool {
        has_sym!(self, xcb_store_named_color)
    }

    /// Returns a pointer to the `name` field of a `xcb_store_named_color_request_t` struct.
    #[inline]
    pub unsafe fn xcb_store_named_color_name(
        &self,
        r: *const xcb_store_named_color_request_t,
    ) -> *mut c_char {
        sym!(self, xcb_store_named_color_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_store_named_color_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_store_named_color_name(&self) -> bool {
        has_sym!(self, xcb_store_named_color_name)
    }

    /// Returns the number of elements of the `name` field of a `xcb_store_named_color_request_t` struct.
    #[inline]
    pub unsafe fn xcb_store_named_color_name_length(
        &self,
        r: *const xcb_store_named_color_request_t,
    ) -> c_int {
        sym!(self, xcb_store_named_color_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_store_named_color_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_store_named_color_name_length(&self) -> bool {
        has_sym!(self, xcb_store_named_color_name_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `name` field of a `xcb_store_named_color_request_t` struct.
    #[inline]
    pub unsafe fn xcb_store_named_color_name_end(
        &self,
        r: *const xcb_store_named_color_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_store_named_color_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_store_named_color_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_store_named_color_name_end(&self) -> bool {
        has_sym!(self, xcb_store_named_color_name_end)
    }

    /// Advances a `xcb_rgb_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_rgb_next(&self, i: *mut xcb_rgb_iterator_t) {
        sym!(self, xcb_rgb_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_rgb_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_rgb_next(&self) -> bool {
        has_sym!(self, xcb_rgb_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_rgb_iterator_t`.
    #[inline]
    pub unsafe fn xcb_rgb_end(&self, i: xcb_rgb_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_rgb_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_rgb_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_rgb_end(&self) -> bool {
        has_sym!(self, xcb_rgb_end)
    }

    /// Computes the size of a `xcb_query_colors_request_t` object.
    #[inline]
    pub unsafe fn xcb_query_colors_sizeof(&self, _buffer: *const c_void, pixels_len: u32) -> c_int {
        sym!(self, xcb_query_colors_sizeof)(_buffer, pixels_len)
    }

    /// Returns `true` iff the symbol `xcb_query_colors_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_colors_sizeof(&self) -> bool {
        has_sym!(self, xcb_query_colors_sizeof)
    }

    /// Sends a `QueryColors` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_query_colors_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_query_colors_reply`]: Self::xcb_query_colors_reply
    #[inline]
    pub unsafe fn xcb_query_colors(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        pixels_len: u32,
        pixels: *const u32,
    ) -> xcb_query_colors_cookie_t {
        sym!(self, xcb_query_colors)(c, cmap, pixels_len, pixels)
    }

    /// Returns `true` iff the symbol `xcb_query_colors` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_colors(&self) -> bool {
        has_sym!(self, xcb_query_colors)
    }

    /// Sends a `QueryColors` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_query_colors_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_query_colors_reply`]: Self::xcb_query_colors_reply
    #[inline]
    pub unsafe fn xcb_query_colors_unchecked(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        pixels_len: u32,
        pixels: *const u32,
    ) -> xcb_query_colors_cookie_t {
        sym!(self, xcb_query_colors_unchecked)(c, cmap, pixels_len, pixels)
    }

    /// Returns `true` iff the symbol `xcb_query_colors_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_colors_unchecked(&self) -> bool {
        has_sym!(self, xcb_query_colors_unchecked)
    }

    /// Returns a pointer to the `colors` field of a `xcb_query_colors_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_query_colors_colors(
        &self,
        r: *const xcb_query_colors_reply_t,
    ) -> *mut xcb_rgb_t {
        sym!(self, xcb_query_colors_colors)(r)
    }

    /// Returns `true` iff the symbol `xcb_query_colors_colors` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_colors_colors(&self) -> bool {
        has_sym!(self, xcb_query_colors_colors)
    }

    /// Returns the number of elements of the `colors` field of a `xcb_query_colors_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_query_colors_colors_length(
        &self,
        r: *const xcb_query_colors_reply_t,
    ) -> c_int {
        sym!(self, xcb_query_colors_colors_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_query_colors_colors_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_colors_colors_length(&self) -> bool {
        has_sym!(self, xcb_query_colors_colors_length)
    }

    /// Returns an iterator over the elements of the
    /// `colors` field of a `xcb_query_colors_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_query_colors_colors_iterator(
        &self,
        r: *const xcb_query_colors_reply_t,
    ) -> xcb_rgb_iterator_t {
        sym!(self, xcb_query_colors_colors_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_query_colors_colors_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_colors_colors_iterator(&self) -> bool {
        has_sym!(self, xcb_query_colors_colors_iterator)
    }

    /// Waits for the reply to a `QueryColors` request.
    #[inline]
    pub unsafe fn xcb_query_colors_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_query_colors_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_colors_reply_t {
        sym!(self, xcb_query_colors_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_query_colors_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_colors_reply(&self) -> bool {
        has_sym!(self, xcb_query_colors_reply)
    }

    /// Computes the size of a `xcb_lookup_color_request_t` object.
    #[inline]
    pub unsafe fn xcb_lookup_color_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_lookup_color_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_lookup_color_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_lookup_color_sizeof(&self) -> bool {
        has_sym!(self, xcb_lookup_color_sizeof)
    }

    /// Sends a `LookupColor` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_lookup_color_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_lookup_color_reply`]: Self::xcb_lookup_color_reply
    #[inline]
    pub unsafe fn xcb_lookup_color(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        name_len: u16,
        name: *const c_char,
    ) -> xcb_lookup_color_cookie_t {
        sym!(self, xcb_lookup_color)(c, cmap, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_lookup_color` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_lookup_color(&self) -> bool {
        has_sym!(self, xcb_lookup_color)
    }

    /// Sends a `LookupColor` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_lookup_color_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_lookup_color_reply`]: Self::xcb_lookup_color_reply
    #[inline]
    pub unsafe fn xcb_lookup_color_unchecked(
        &self,
        c: *mut xcb_connection_t,
        cmap: xcb_colormap_t,
        name_len: u16,
        name: *const c_char,
    ) -> xcb_lookup_color_cookie_t {
        sym!(self, xcb_lookup_color_unchecked)(c, cmap, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_lookup_color_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_lookup_color_unchecked(&self) -> bool {
        has_sym!(self, xcb_lookup_color_unchecked)
    }

    /// Waits for the reply to a `LookupColor` request.
    #[inline]
    pub unsafe fn xcb_lookup_color_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_lookup_color_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_lookup_color_reply_t {
        sym!(self, xcb_lookup_color_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_lookup_color_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_lookup_color_reply(&self) -> bool {
        has_sym!(self, xcb_lookup_color_reply)
    }

    /// Sends a `CreateCursor` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_create_cursor_checked(
        &self,
        c: *mut xcb_connection_t,
        cid: xcb_cursor_t,
        source: xcb_pixmap_t,
        mask: xcb_pixmap_t,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
        x: u16,
        y: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_cursor_checked)(
            c, cid, source, mask, fore_red, fore_green, fore_blue, back_red, back_green, back_blue,
            x, y,
        )
    }

    /// Returns `true` iff the symbol `xcb_create_cursor_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_cursor_checked(&self) -> bool {
        has_sym!(self, xcb_create_cursor_checked)
    }

    /// Sends a `CreateCursor` request (unchecked).
    #[inline]
    pub unsafe fn xcb_create_cursor(
        &self,
        c: *mut xcb_connection_t,
        cid: xcb_cursor_t,
        source: xcb_pixmap_t,
        mask: xcb_pixmap_t,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
        x: u16,
        y: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_cursor)(
            c, cid, source, mask, fore_red, fore_green, fore_blue, back_red, back_green, back_blue,
            x, y,
        )
    }

    /// Returns `true` iff the symbol `xcb_create_cursor` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_cursor(&self) -> bool {
        has_sym!(self, xcb_create_cursor)
    }

    /// Sends a `CreateGlyphCursor` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_create_glyph_cursor_checked(
        &self,
        c: *mut xcb_connection_t,
        cid: xcb_cursor_t,
        source_font: xcb_font_t,
        mask_font: xcb_font_t,
        source_char: u16,
        mask_char: u16,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_glyph_cursor_checked)(
            c,
            cid,
            source_font,
            mask_font,
            source_char,
            mask_char,
            fore_red,
            fore_green,
            fore_blue,
            back_red,
            back_green,
            back_blue,
        )
    }

    /// Returns `true` iff the symbol `xcb_create_glyph_cursor_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_glyph_cursor_checked(&self) -> bool {
        has_sym!(self, xcb_create_glyph_cursor_checked)
    }

    /// Sends a `CreateGlyphCursor` request (unchecked).
    #[inline]
    pub unsafe fn xcb_create_glyph_cursor(
        &self,
        c: *mut xcb_connection_t,
        cid: xcb_cursor_t,
        source_font: xcb_font_t,
        mask_font: xcb_font_t,
        source_char: u16,
        mask_char: u16,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_create_glyph_cursor)(
            c,
            cid,
            source_font,
            mask_font,
            source_char,
            mask_char,
            fore_red,
            fore_green,
            fore_blue,
            back_red,
            back_green,
            back_blue,
        )
    }

    /// Returns `true` iff the symbol `xcb_create_glyph_cursor` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_create_glyph_cursor(&self) -> bool {
        has_sym!(self, xcb_create_glyph_cursor)
    }

    /// Sends a `FreeCursor` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_free_cursor_checked(
        &self,
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_free_cursor_checked)(c, cursor)
    }

    /// Returns `true` iff the symbol `xcb_free_cursor_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_free_cursor_checked(&self) -> bool {
        has_sym!(self, xcb_free_cursor_checked)
    }

    /// Sends a `FreeCursor` request (unchecked).
    #[inline]
    pub unsafe fn xcb_free_cursor(
        &self,
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_free_cursor)(c, cursor)
    }

    /// Returns `true` iff the symbol `xcb_free_cursor` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_free_cursor(&self) -> bool {
        has_sym!(self, xcb_free_cursor)
    }

    /// Sends a `RecolorCursor` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_recolor_cursor_checked(
        &self,
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_recolor_cursor_checked)(
            c, cursor, fore_red, fore_green, fore_blue, back_red, back_green, back_blue,
        )
    }

    /// Returns `true` iff the symbol `xcb_recolor_cursor_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_recolor_cursor_checked(&self) -> bool {
        has_sym!(self, xcb_recolor_cursor_checked)
    }

    /// Sends a `RecolorCursor` request (unchecked).
    #[inline]
    pub unsafe fn xcb_recolor_cursor(
        &self,
        c: *mut xcb_connection_t,
        cursor: xcb_cursor_t,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_recolor_cursor)(
            c, cursor, fore_red, fore_green, fore_blue, back_red, back_green, back_blue,
        )
    }

    /// Returns `true` iff the symbol `xcb_recolor_cursor` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_recolor_cursor(&self) -> bool {
        has_sym!(self, xcb_recolor_cursor)
    }

    /// Sends a `QueryBestSize` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_query_best_size_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_query_best_size_reply`]: Self::xcb_query_best_size_reply
    #[inline]
    pub unsafe fn xcb_query_best_size(
        &self,
        c: *mut xcb_connection_t,
        class: u8,
        drawable: xcb_drawable_t,
        width: u16,
        height: u16,
    ) -> xcb_query_best_size_cookie_t {
        sym!(self, xcb_query_best_size)(c, class, drawable, width, height)
    }

    /// Returns `true` iff the symbol `xcb_query_best_size` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_best_size(&self) -> bool {
        has_sym!(self, xcb_query_best_size)
    }

    /// Sends a `QueryBestSize` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_query_best_size_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_query_best_size_reply`]: Self::xcb_query_best_size_reply
    #[inline]
    pub unsafe fn xcb_query_best_size_unchecked(
        &self,
        c: *mut xcb_connection_t,
        class: u8,
        drawable: xcb_drawable_t,
        width: u16,
        height: u16,
    ) -> xcb_query_best_size_cookie_t {
        sym!(self, xcb_query_best_size_unchecked)(c, class, drawable, width, height)
    }

    /// Returns `true` iff the symbol `xcb_query_best_size_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_best_size_unchecked(&self) -> bool {
        has_sym!(self, xcb_query_best_size_unchecked)
    }

    /// Waits for the reply to a `QueryBestSize` request.
    #[inline]
    pub unsafe fn xcb_query_best_size_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_query_best_size_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_best_size_reply_t {
        sym!(self, xcb_query_best_size_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_query_best_size_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_best_size_reply(&self) -> bool {
        has_sym!(self, xcb_query_best_size_reply)
    }

    /// Computes the size of a `xcb_query_extension_request_t` object.
    #[inline]
    pub unsafe fn xcb_query_extension_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_query_extension_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_query_extension_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_extension_sizeof(&self) -> bool {
        has_sym!(self, xcb_query_extension_sizeof)
    }

    /// Sends a `QueryExtension` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_query_extension_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_query_extension_reply`]: Self::xcb_query_extension_reply
    #[inline]
    pub unsafe fn xcb_query_extension(
        &self,
        c: *mut xcb_connection_t,
        name_len: u16,
        name: *const c_char,
    ) -> xcb_query_extension_cookie_t {
        sym!(self, xcb_query_extension)(c, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_query_extension` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_extension(&self) -> bool {
        has_sym!(self, xcb_query_extension)
    }

    /// Sends a `QueryExtension` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_query_extension_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_query_extension_reply`]: Self::xcb_query_extension_reply
    #[inline]
    pub unsafe fn xcb_query_extension_unchecked(
        &self,
        c: *mut xcb_connection_t,
        name_len: u16,
        name: *const c_char,
    ) -> xcb_query_extension_cookie_t {
        sym!(self, xcb_query_extension_unchecked)(c, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_query_extension_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_extension_unchecked(&self) -> bool {
        has_sym!(self, xcb_query_extension_unchecked)
    }

    /// Waits for the reply to a `QueryExtension` request.
    #[inline]
    pub unsafe fn xcb_query_extension_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_query_extension_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_query_extension_reply_t {
        sym!(self, xcb_query_extension_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_query_extension_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_query_extension_reply(&self) -> bool {
        has_sym!(self, xcb_query_extension_reply)
    }

    /// Computes the size of a `xcb_list_extensions_reply_t` object.
    #[inline]
    pub unsafe fn xcb_list_extensions_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_list_extensions_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_list_extensions_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_extensions_sizeof(&self) -> bool {
        has_sym!(self, xcb_list_extensions_sizeof)
    }

    /// Sends a `ListExtensions` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_list_extensions_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_list_extensions_reply`]: Self::xcb_list_extensions_reply
    #[inline]
    pub unsafe fn xcb_list_extensions(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_list_extensions_cookie_t {
        sym!(self, xcb_list_extensions)(c)
    }

    /// Returns `true` iff the symbol `xcb_list_extensions` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_extensions(&self) -> bool {
        has_sym!(self, xcb_list_extensions)
    }

    /// Sends a `ListExtensions` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_list_extensions_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_list_extensions_reply`]: Self::xcb_list_extensions_reply
    #[inline]
    pub unsafe fn xcb_list_extensions_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_list_extensions_cookie_t {
        sym!(self, xcb_list_extensions_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_list_extensions_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_extensions_unchecked(&self) -> bool {
        has_sym!(self, xcb_list_extensions_unchecked)
    }

    /// Returns the number of elements of the `names` field of a `xcb_list_extensions_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_list_extensions_names_length(
        &self,
        r: *const xcb_list_extensions_reply_t,
    ) -> c_int {
        sym!(self, xcb_list_extensions_names_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_extensions_names_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_extensions_names_length(&self) -> bool {
        has_sym!(self, xcb_list_extensions_names_length)
    }

    /// Returns an iterator over the elements of the
    /// `names` field of a `xcb_list_extensions_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_list_extensions_names_iterator(
        &self,
        r: *const xcb_list_extensions_reply_t,
    ) -> xcb_str_iterator_t {
        sym!(self, xcb_list_extensions_names_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_extensions_names_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_extensions_names_iterator(&self) -> bool {
        has_sym!(self, xcb_list_extensions_names_iterator)
    }

    /// Waits for the reply to a `ListExtensions` request.
    #[inline]
    pub unsafe fn xcb_list_extensions_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_list_extensions_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_list_extensions_reply_t {
        sym!(self, xcb_list_extensions_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_list_extensions_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_extensions_reply(&self) -> bool {
        has_sym!(self, xcb_list_extensions_reply)
    }

    /// Computes the size of a `xcb_change_keyboard_mapping_request_t` object.
    #[inline]
    pub unsafe fn xcb_change_keyboard_mapping_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_change_keyboard_mapping_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_mapping_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_mapping_sizeof(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_mapping_sizeof)
    }

    /// Sends a `ChangeKeyboardMapping` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_change_keyboard_mapping_checked(
        &self,
        c: *mut xcb_connection_t,
        keycode_count: u8,
        first_keycode: xcb_keycode_t,
        keysyms_per_keycode: u8,
        keysyms: *const xcb_keysym_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_keyboard_mapping_checked)(
            c,
            keycode_count,
            first_keycode,
            keysyms_per_keycode,
            keysyms,
        )
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_mapping_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_mapping_checked(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_mapping_checked)
    }

    /// Sends a `ChangeKeyboardMapping` request (unchecked).
    #[inline]
    pub unsafe fn xcb_change_keyboard_mapping(
        &self,
        c: *mut xcb_connection_t,
        keycode_count: u8,
        first_keycode: xcb_keycode_t,
        keysyms_per_keycode: u8,
        keysyms: *const xcb_keysym_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_keyboard_mapping)(
            c,
            keycode_count,
            first_keycode,
            keysyms_per_keycode,
            keysyms,
        )
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_mapping` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_mapping(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_mapping)
    }

    /// Returns a pointer to the `keysyms` field of a `xcb_change_keyboard_mapping_request_t` struct.
    #[inline]
    pub unsafe fn xcb_change_keyboard_mapping_keysyms(
        &self,
        r: *const xcb_change_keyboard_mapping_request_t,
    ) -> *mut xcb_keysym_t {
        sym!(self, xcb_change_keyboard_mapping_keysyms)(r)
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_mapping_keysyms` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_mapping_keysyms(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_mapping_keysyms)
    }

    /// Returns the number of elements of the `keysyms` field of a `xcb_change_keyboard_mapping_request_t` struct.
    #[inline]
    pub unsafe fn xcb_change_keyboard_mapping_keysyms_length(
        &self,
        r: *const xcb_change_keyboard_mapping_request_t,
    ) -> c_int {
        sym!(self, xcb_change_keyboard_mapping_keysyms_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_mapping_keysyms_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_mapping_keysyms_length(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_mapping_keysyms_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `keysyms` field of a `xcb_change_keyboard_mapping_request_t` struct.
    #[inline]
    pub unsafe fn xcb_change_keyboard_mapping_keysyms_end(
        &self,
        r: *const xcb_change_keyboard_mapping_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_change_keyboard_mapping_keysyms_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_mapping_keysyms_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_mapping_keysyms_end(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_mapping_keysyms_end)
    }

    /// Computes the size of a `xcb_get_keyboard_mapping_reply_t` object.
    #[inline]
    pub unsafe fn xcb_get_keyboard_mapping_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_get_keyboard_mapping_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_get_keyboard_mapping_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_keyboard_mapping_sizeof(&self) -> bool {
        has_sym!(self, xcb_get_keyboard_mapping_sizeof)
    }

    /// Sends a `GetKeyboardMapping` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_keyboard_mapping_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_keyboard_mapping_reply`]: Self::xcb_get_keyboard_mapping_reply
    #[inline]
    pub unsafe fn xcb_get_keyboard_mapping(
        &self,
        c: *mut xcb_connection_t,
        first_keycode: xcb_keycode_t,
        count: u8,
    ) -> xcb_get_keyboard_mapping_cookie_t {
        sym!(self, xcb_get_keyboard_mapping)(c, first_keycode, count)
    }

    /// Returns `true` iff the symbol `xcb_get_keyboard_mapping` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_keyboard_mapping(&self) -> bool {
        has_sym!(self, xcb_get_keyboard_mapping)
    }

    /// Sends a `GetKeyboardMapping` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_keyboard_mapping_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_keyboard_mapping_reply`]: Self::xcb_get_keyboard_mapping_reply
    #[inline]
    pub unsafe fn xcb_get_keyboard_mapping_unchecked(
        &self,
        c: *mut xcb_connection_t,
        first_keycode: xcb_keycode_t,
        count: u8,
    ) -> xcb_get_keyboard_mapping_cookie_t {
        sym!(self, xcb_get_keyboard_mapping_unchecked)(c, first_keycode, count)
    }

    /// Returns `true` iff the symbol `xcb_get_keyboard_mapping_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_keyboard_mapping_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_keyboard_mapping_unchecked)
    }

    /// Returns a pointer to the `keysyms` field of a `xcb_get_keyboard_mapping_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_get_keyboard_mapping_keysyms(
        &self,
        r: *const xcb_get_keyboard_mapping_reply_t,
    ) -> *mut xcb_keysym_t {
        sym!(self, xcb_get_keyboard_mapping_keysyms)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_keyboard_mapping_keysyms` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_keyboard_mapping_keysyms(&self) -> bool {
        has_sym!(self, xcb_get_keyboard_mapping_keysyms)
    }

    /// Returns the number of elements of the `keysyms` field of a `xcb_get_keyboard_mapping_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_get_keyboard_mapping_keysyms_length(
        &self,
        r: *const xcb_get_keyboard_mapping_reply_t,
    ) -> c_int {
        sym!(self, xcb_get_keyboard_mapping_keysyms_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_keyboard_mapping_keysyms_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_keyboard_mapping_keysyms_length(&self) -> bool {
        has_sym!(self, xcb_get_keyboard_mapping_keysyms_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `keysyms` field of a `xcb_get_keyboard_mapping_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_get_keyboard_mapping_keysyms_end(
        &self,
        r: *const xcb_get_keyboard_mapping_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_get_keyboard_mapping_keysyms_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_keyboard_mapping_keysyms_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_keyboard_mapping_keysyms_end(&self) -> bool {
        has_sym!(self, xcb_get_keyboard_mapping_keysyms_end)
    }

    /// Waits for the reply to a `GetKeyboardMapping` request.
    #[inline]
    pub unsafe fn xcb_get_keyboard_mapping_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_keyboard_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_keyboard_mapping_reply_t {
        sym!(self, xcb_get_keyboard_mapping_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_keyboard_mapping_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_keyboard_mapping_reply(&self) -> bool {
        has_sym!(self, xcb_get_keyboard_mapping_reply)
    }

    /// Serializes a `xcb_change_keyboard_control_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_change_keyboard_control_value_list_serialize(
        &self,
        _buffer: *mut *mut c_void,
        value_mask: u32,
        _aux: *const xcb_change_keyboard_control_value_list_t,
    ) -> c_int {
        sym!(self, xcb_change_keyboard_control_value_list_serialize)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_control_value_list_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_control_value_list_serialize(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_control_value_list_serialize)
    }

    /// Unpacks a `xcb_change_keyboard_control_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_change_keyboard_control_value_list_unpack(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
        _aux: *mut xcb_change_keyboard_control_value_list_t,
    ) -> c_int {
        sym!(self, xcb_change_keyboard_control_value_list_unpack)(_buffer, value_mask, _aux)
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_control_value_list_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_control_value_list_unpack(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_control_value_list_unpack)
    }

    /// Computes the size of a `xcb_change_keyboard_control_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_change_keyboard_control_value_list_sizeof(
        &self,
        _buffer: *const c_void,
        value_mask: u32,
    ) -> c_int {
        sym!(self, xcb_change_keyboard_control_value_list_sizeof)(_buffer, value_mask)
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_control_value_list_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_control_value_list_sizeof(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_control_value_list_sizeof)
    }

    /// Computes the size of a `xcb_change_keyboard_control_request_t` object.
    #[inline]
    pub unsafe fn xcb_change_keyboard_control_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_change_keyboard_control_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_control_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_control_sizeof(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_control_sizeof)
    }

    /// Sends a `ChangeKeyboardControl` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    ///
    /// There is an auxiliary version of this function: [`xcb_change_keyboard_control_aux_checked`].
    ///
    /// [`xcb_change_keyboard_control_aux_checked`]: Self::xcb_change_keyboard_control_aux_checked
    #[inline]
    pub unsafe fn xcb_change_keyboard_control_checked(
        &self,
        c: *mut xcb_connection_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_keyboard_control_checked)(c, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_control_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_control_checked(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_control_checked)
    }

    /// Sends a `ChangeKeyboardControl` request (unchecked).
    ///
    /// There is an auxiliary version of this function: [`xcb_change_keyboard_control_aux`].
    ///
    /// [`xcb_change_keyboard_control_aux`]: Self::xcb_change_keyboard_control_aux
    #[inline]
    pub unsafe fn xcb_change_keyboard_control(
        &self,
        c: *mut xcb_connection_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_keyboard_control)(c, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_control` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_control(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_control)
    }

    /// Sends a `ChangeKeyboardControl` request (checked) (aux).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_change_keyboard_control_aux_checked(
        &self,
        c: *mut xcb_connection_t,
        value_mask: u32,
        value_list: *const xcb_change_keyboard_control_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_keyboard_control_aux_checked)(c, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_control_aux_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_control_aux_checked(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_control_aux_checked)
    }

    /// Sends a `ChangeKeyboardControl` request (unchecked) (aux).
    #[inline]
    pub unsafe fn xcb_change_keyboard_control_aux(
        &self,
        c: *mut xcb_connection_t,
        value_mask: u32,
        value_list: *const xcb_change_keyboard_control_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_keyboard_control_aux)(c, value_mask, value_list)
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_control_aux` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_control_aux(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_control_aux)
    }

    /// Returns a pointer to the `value_list` field of a `xcb_change_keyboard_control_request_t` struct.
    #[inline]
    pub unsafe fn xcb_change_keyboard_control_value_list(
        &self,
        r: *const xcb_change_keyboard_control_request_t,
    ) -> *mut c_void {
        sym!(self, xcb_change_keyboard_control_value_list)(r)
    }

    /// Returns `true` iff the symbol `xcb_change_keyboard_control_value_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_keyboard_control_value_list(&self) -> bool {
        has_sym!(self, xcb_change_keyboard_control_value_list)
    }

    /// Sends a `GetKeyboardControl` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_keyboard_control_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_keyboard_control_reply`]: Self::xcb_get_keyboard_control_reply
    #[inline]
    pub unsafe fn xcb_get_keyboard_control(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_get_keyboard_control_cookie_t {
        sym!(self, xcb_get_keyboard_control)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_keyboard_control` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_keyboard_control(&self) -> bool {
        has_sym!(self, xcb_get_keyboard_control)
    }

    /// Sends a `GetKeyboardControl` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_keyboard_control_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_keyboard_control_reply`]: Self::xcb_get_keyboard_control_reply
    #[inline]
    pub unsafe fn xcb_get_keyboard_control_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_get_keyboard_control_cookie_t {
        sym!(self, xcb_get_keyboard_control_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_keyboard_control_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_keyboard_control_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_keyboard_control_unchecked)
    }

    /// Waits for the reply to a `GetKeyboardControl` request.
    #[inline]
    pub unsafe fn xcb_get_keyboard_control_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_keyboard_control_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_keyboard_control_reply_t {
        sym!(self, xcb_get_keyboard_control_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_keyboard_control_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_keyboard_control_reply(&self) -> bool {
        has_sym!(self, xcb_get_keyboard_control_reply)
    }

    /// Sends a `Bell` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_bell_checked(
        &self,
        c: *mut xcb_connection_t,
        percent: i8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_bell_checked)(c, percent)
    }

    /// Returns `true` iff the symbol `xcb_bell_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_bell_checked(&self) -> bool {
        has_sym!(self, xcb_bell_checked)
    }

    /// Sends a `Bell` request (unchecked).
    #[inline]
    pub unsafe fn xcb_bell(&self, c: *mut xcb_connection_t, percent: i8) -> xcb_void_cookie_t {
        sym!(self, xcb_bell)(c, percent)
    }

    /// Returns `true` iff the symbol `xcb_bell` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_bell(&self) -> bool {
        has_sym!(self, xcb_bell)
    }

    /// Sends a `ChangePointerControl` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_change_pointer_control_checked(
        &self,
        c: *mut xcb_connection_t,
        acceleration_numerator: i16,
        acceleration_denominator: i16,
        threshold: i16,
        do_acceleration: u8,
        do_threshold: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_pointer_control_checked)(
            c,
            acceleration_numerator,
            acceleration_denominator,
            threshold,
            do_acceleration,
            do_threshold,
        )
    }

    /// Returns `true` iff the symbol `xcb_change_pointer_control_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_pointer_control_checked(&self) -> bool {
        has_sym!(self, xcb_change_pointer_control_checked)
    }

    /// Sends a `ChangePointerControl` request (unchecked).
    #[inline]
    pub unsafe fn xcb_change_pointer_control(
        &self,
        c: *mut xcb_connection_t,
        acceleration_numerator: i16,
        acceleration_denominator: i16,
        threshold: i16,
        do_acceleration: u8,
        do_threshold: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_pointer_control)(
            c,
            acceleration_numerator,
            acceleration_denominator,
            threshold,
            do_acceleration,
            do_threshold,
        )
    }

    /// Returns `true` iff the symbol `xcb_change_pointer_control` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_pointer_control(&self) -> bool {
        has_sym!(self, xcb_change_pointer_control)
    }

    /// Sends a `GetPointerControl` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_pointer_control_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_pointer_control_reply`]: Self::xcb_get_pointer_control_reply
    #[inline]
    pub unsafe fn xcb_get_pointer_control(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_get_pointer_control_cookie_t {
        sym!(self, xcb_get_pointer_control)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_pointer_control` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_pointer_control(&self) -> bool {
        has_sym!(self, xcb_get_pointer_control)
    }

    /// Sends a `GetPointerControl` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_pointer_control_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_pointer_control_reply`]: Self::xcb_get_pointer_control_reply
    #[inline]
    pub unsafe fn xcb_get_pointer_control_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_get_pointer_control_cookie_t {
        sym!(self, xcb_get_pointer_control_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_pointer_control_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_pointer_control_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_pointer_control_unchecked)
    }

    /// Waits for the reply to a `GetPointerControl` request.
    #[inline]
    pub unsafe fn xcb_get_pointer_control_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_pointer_control_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_pointer_control_reply_t {
        sym!(self, xcb_get_pointer_control_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_pointer_control_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_pointer_control_reply(&self) -> bool {
        has_sym!(self, xcb_get_pointer_control_reply)
    }

    /// Sends a `SetScreenSaver` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_set_screen_saver_checked(
        &self,
        c: *mut xcb_connection_t,
        timeout: i16,
        interval: i16,
        prefer_blanking: u8,
        allow_exposures: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_screen_saver_checked)(
            c,
            timeout,
            interval,
            prefer_blanking,
            allow_exposures,
        )
    }

    /// Returns `true` iff the symbol `xcb_set_screen_saver_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_screen_saver_checked(&self) -> bool {
        has_sym!(self, xcb_set_screen_saver_checked)
    }

    /// Sends a `SetScreenSaver` request (unchecked).
    #[inline]
    pub unsafe fn xcb_set_screen_saver(
        &self,
        c: *mut xcb_connection_t,
        timeout: i16,
        interval: i16,
        prefer_blanking: u8,
        allow_exposures: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_screen_saver)(c, timeout, interval, prefer_blanking, allow_exposures)
    }

    /// Returns `true` iff the symbol `xcb_set_screen_saver` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_screen_saver(&self) -> bool {
        has_sym!(self, xcb_set_screen_saver)
    }

    /// Sends a `GetScreenSaver` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_screen_saver_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_screen_saver_reply`]: Self::xcb_get_screen_saver_reply
    #[inline]
    pub unsafe fn xcb_get_screen_saver(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_get_screen_saver_cookie_t {
        sym!(self, xcb_get_screen_saver)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_screen_saver` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_screen_saver(&self) -> bool {
        has_sym!(self, xcb_get_screen_saver)
    }

    /// Sends a `GetScreenSaver` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_screen_saver_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_screen_saver_reply`]: Self::xcb_get_screen_saver_reply
    #[inline]
    pub unsafe fn xcb_get_screen_saver_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_get_screen_saver_cookie_t {
        sym!(self, xcb_get_screen_saver_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_screen_saver_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_screen_saver_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_screen_saver_unchecked)
    }

    /// Waits for the reply to a `GetScreenSaver` request.
    #[inline]
    pub unsafe fn xcb_get_screen_saver_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_screen_saver_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_screen_saver_reply_t {
        sym!(self, xcb_get_screen_saver_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_screen_saver_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_screen_saver_reply(&self) -> bool {
        has_sym!(self, xcb_get_screen_saver_reply)
    }

    /// Computes the size of a `xcb_change_hosts_request_t` object.
    #[inline]
    pub unsafe fn xcb_change_hosts_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_change_hosts_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_change_hosts_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_hosts_sizeof(&self) -> bool {
        has_sym!(self, xcb_change_hosts_sizeof)
    }

    /// Sends a `ChangeHosts` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_change_hosts_checked(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
        family: u8,
        address_len: u16,
        address: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_hosts_checked)(c, mode, family, address_len, address)
    }

    /// Returns `true` iff the symbol `xcb_change_hosts_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_hosts_checked(&self) -> bool {
        has_sym!(self, xcb_change_hosts_checked)
    }

    /// Sends a `ChangeHosts` request (unchecked).
    #[inline]
    pub unsafe fn xcb_change_hosts(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
        family: u8,
        address_len: u16,
        address: *const u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_change_hosts)(c, mode, family, address_len, address)
    }

    /// Returns `true` iff the symbol `xcb_change_hosts` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_hosts(&self) -> bool {
        has_sym!(self, xcb_change_hosts)
    }

    /// Returns a pointer to the `address` field of a `xcb_change_hosts_request_t` struct.
    #[inline]
    pub unsafe fn xcb_change_hosts_address(&self, r: *const xcb_change_hosts_request_t) -> *mut u8 {
        sym!(self, xcb_change_hosts_address)(r)
    }

    /// Returns `true` iff the symbol `xcb_change_hosts_address` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_hosts_address(&self) -> bool {
        has_sym!(self, xcb_change_hosts_address)
    }

    /// Returns the number of elements of the `address` field of a `xcb_change_hosts_request_t` struct.
    #[inline]
    pub unsafe fn xcb_change_hosts_address_length(
        &self,
        r: *const xcb_change_hosts_request_t,
    ) -> c_int {
        sym!(self, xcb_change_hosts_address_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_change_hosts_address_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_hosts_address_length(&self) -> bool {
        has_sym!(self, xcb_change_hosts_address_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `address` field of a `xcb_change_hosts_request_t` struct.
    #[inline]
    pub unsafe fn xcb_change_hosts_address_end(
        &self,
        r: *const xcb_change_hosts_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_change_hosts_address_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_change_hosts_address_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_change_hosts_address_end(&self) -> bool {
        has_sym!(self, xcb_change_hosts_address_end)
    }

    /// Computes the size of a `xcb_host_t` object.
    #[inline]
    pub unsafe fn xcb_host_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_host_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_host_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_host_sizeof(&self) -> bool {
        has_sym!(self, xcb_host_sizeof)
    }

    /// Returns a pointer to the `address` field of a `xcb_host_t` struct.
    #[inline]
    pub unsafe fn xcb_host_address(&self, r: *const xcb_host_t) -> *mut u8 {
        sym!(self, xcb_host_address)(r)
    }

    /// Returns `true` iff the symbol `xcb_host_address` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_host_address(&self) -> bool {
        has_sym!(self, xcb_host_address)
    }

    /// Returns the number of elements of the `address` field of a `xcb_host_t` struct.
    #[inline]
    pub unsafe fn xcb_host_address_length(&self, r: *const xcb_host_t) -> c_int {
        sym!(self, xcb_host_address_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_host_address_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_host_address_length(&self) -> bool {
        has_sym!(self, xcb_host_address_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `address` field of a `xcb_host_t` struct.
    #[inline]
    pub unsafe fn xcb_host_address_end(&self, r: *const xcb_host_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_host_address_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_host_address_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_host_address_end(&self) -> bool {
        has_sym!(self, xcb_host_address_end)
    }

    /// Advances a `xcb_host_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_host_next(&self, i: *mut xcb_host_iterator_t) {
        sym!(self, xcb_host_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_host_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_host_next(&self) -> bool {
        has_sym!(self, xcb_host_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_host_iterator_t`.
    #[inline]
    pub unsafe fn xcb_host_end(&self, i: xcb_host_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_host_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_host_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_host_end(&self) -> bool {
        has_sym!(self, xcb_host_end)
    }

    /// Computes the size of a `xcb_list_hosts_reply_t` object.
    #[inline]
    pub unsafe fn xcb_list_hosts_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_list_hosts_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_list_hosts_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_hosts_sizeof(&self) -> bool {
        has_sym!(self, xcb_list_hosts_sizeof)
    }

    /// Sends a `ListHosts` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_list_hosts_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_list_hosts_reply`]: Self::xcb_list_hosts_reply
    #[inline]
    pub unsafe fn xcb_list_hosts(&self, c: *mut xcb_connection_t) -> xcb_list_hosts_cookie_t {
        sym!(self, xcb_list_hosts)(c)
    }

    /// Returns `true` iff the symbol `xcb_list_hosts` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_hosts(&self) -> bool {
        has_sym!(self, xcb_list_hosts)
    }

    /// Sends a `ListHosts` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_list_hosts_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_list_hosts_reply`]: Self::xcb_list_hosts_reply
    #[inline]
    pub unsafe fn xcb_list_hosts_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_list_hosts_cookie_t {
        sym!(self, xcb_list_hosts_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_list_hosts_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_hosts_unchecked(&self) -> bool {
        has_sym!(self, xcb_list_hosts_unchecked)
    }

    /// Returns the number of elements of the `hosts` field of a `xcb_list_hosts_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_list_hosts_hosts_length(&self, r: *const xcb_list_hosts_reply_t) -> c_int {
        sym!(self, xcb_list_hosts_hosts_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_hosts_hosts_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_hosts_hosts_length(&self) -> bool {
        has_sym!(self, xcb_list_hosts_hosts_length)
    }

    /// Returns an iterator over the elements of the
    /// `hosts` field of a `xcb_list_hosts_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_list_hosts_hosts_iterator(
        &self,
        r: *const xcb_list_hosts_reply_t,
    ) -> xcb_host_iterator_t {
        sym!(self, xcb_list_hosts_hosts_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_list_hosts_hosts_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_hosts_hosts_iterator(&self) -> bool {
        has_sym!(self, xcb_list_hosts_hosts_iterator)
    }

    /// Waits for the reply to a `ListHosts` request.
    #[inline]
    pub unsafe fn xcb_list_hosts_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_list_hosts_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_list_hosts_reply_t {
        sym!(self, xcb_list_hosts_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_list_hosts_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_list_hosts_reply(&self) -> bool {
        has_sym!(self, xcb_list_hosts_reply)
    }

    /// Sends a `SetAccessControl` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_set_access_control_checked(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_access_control_checked)(c, mode)
    }

    /// Returns `true` iff the symbol `xcb_set_access_control_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_access_control_checked(&self) -> bool {
        has_sym!(self, xcb_set_access_control_checked)
    }

    /// Sends a `SetAccessControl` request (unchecked).
    #[inline]
    pub unsafe fn xcb_set_access_control(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_access_control)(c, mode)
    }

    /// Returns `true` iff the symbol `xcb_set_access_control` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_access_control(&self) -> bool {
        has_sym!(self, xcb_set_access_control)
    }

    /// Sends a `SetCloseDownMode` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_set_close_down_mode_checked(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_close_down_mode_checked)(c, mode)
    }

    /// Returns `true` iff the symbol `xcb_set_close_down_mode_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_close_down_mode_checked(&self) -> bool {
        has_sym!(self, xcb_set_close_down_mode_checked)
    }

    /// Sends a `SetCloseDownMode` request (unchecked).
    #[inline]
    pub unsafe fn xcb_set_close_down_mode(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_set_close_down_mode)(c, mode)
    }

    /// Returns `true` iff the symbol `xcb_set_close_down_mode` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_close_down_mode(&self) -> bool {
        has_sym!(self, xcb_set_close_down_mode)
    }

    /// Sends a `KillClient` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_kill_client_checked(
        &self,
        c: *mut xcb_connection_t,
        resource: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_kill_client_checked)(c, resource)
    }

    /// Returns `true` iff the symbol `xcb_kill_client_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_kill_client_checked(&self) -> bool {
        has_sym!(self, xcb_kill_client_checked)
    }

    /// Sends a `KillClient` request (unchecked).
    #[inline]
    pub unsafe fn xcb_kill_client(
        &self,
        c: *mut xcb_connection_t,
        resource: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_kill_client)(c, resource)
    }

    /// Returns `true` iff the symbol `xcb_kill_client` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_kill_client(&self) -> bool {
        has_sym!(self, xcb_kill_client)
    }

    /// Computes the size of a `xcb_rotate_properties_request_t` object.
    #[inline]
    pub unsafe fn xcb_rotate_properties_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_rotate_properties_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_rotate_properties_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_rotate_properties_sizeof(&self) -> bool {
        has_sym!(self, xcb_rotate_properties_sizeof)
    }

    /// Sends a `RotateProperties` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_rotate_properties_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        atoms_len: u16,
        delta: i16,
        atoms: *const xcb_atom_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_rotate_properties_checked)(c, window, atoms_len, delta, atoms)
    }

    /// Returns `true` iff the symbol `xcb_rotate_properties_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_rotate_properties_checked(&self) -> bool {
        has_sym!(self, xcb_rotate_properties_checked)
    }

    /// Sends a `RotateProperties` request (unchecked).
    #[inline]
    pub unsafe fn xcb_rotate_properties(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        atoms_len: u16,
        delta: i16,
        atoms: *const xcb_atom_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_rotate_properties)(c, window, atoms_len, delta, atoms)
    }

    /// Returns `true` iff the symbol `xcb_rotate_properties` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_rotate_properties(&self) -> bool {
        has_sym!(self, xcb_rotate_properties)
    }

    /// Returns a pointer to the `atoms` field of a `xcb_rotate_properties_request_t` struct.
    #[inline]
    pub unsafe fn xcb_rotate_properties_atoms(
        &self,
        r: *const xcb_rotate_properties_request_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_rotate_properties_atoms)(r)
    }

    /// Returns `true` iff the symbol `xcb_rotate_properties_atoms` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_rotate_properties_atoms(&self) -> bool {
        has_sym!(self, xcb_rotate_properties_atoms)
    }

    /// Returns the number of elements of the `atoms` field of a `xcb_rotate_properties_request_t` struct.
    #[inline]
    pub unsafe fn xcb_rotate_properties_atoms_length(
        &self,
        r: *const xcb_rotate_properties_request_t,
    ) -> c_int {
        sym!(self, xcb_rotate_properties_atoms_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_rotate_properties_atoms_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_rotate_properties_atoms_length(&self) -> bool {
        has_sym!(self, xcb_rotate_properties_atoms_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `atoms` field of a `xcb_rotate_properties_request_t` struct.
    #[inline]
    pub unsafe fn xcb_rotate_properties_atoms_end(
        &self,
        r: *const xcb_rotate_properties_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_rotate_properties_atoms_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_rotate_properties_atoms_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_rotate_properties_atoms_end(&self) -> bool {
        has_sym!(self, xcb_rotate_properties_atoms_end)
    }

    /// Sends a `ForceScreenSaver` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_force_screen_saver_checked(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_force_screen_saver_checked)(c, mode)
    }

    /// Returns `true` iff the symbol `xcb_force_screen_saver_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_force_screen_saver_checked(&self) -> bool {
        has_sym!(self, xcb_force_screen_saver_checked)
    }

    /// Sends a `ForceScreenSaver` request (unchecked).
    #[inline]
    pub unsafe fn xcb_force_screen_saver(
        &self,
        c: *mut xcb_connection_t,
        mode: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_force_screen_saver)(c, mode)
    }

    /// Returns `true` iff the symbol `xcb_force_screen_saver` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_force_screen_saver(&self) -> bool {
        has_sym!(self, xcb_force_screen_saver)
    }

    /// Computes the size of a `xcb_set_pointer_mapping_request_t` object.
    #[inline]
    pub unsafe fn xcb_set_pointer_mapping_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_set_pointer_mapping_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_set_pointer_mapping_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_pointer_mapping_sizeof(&self) -> bool {
        has_sym!(self, xcb_set_pointer_mapping_sizeof)
    }

    /// Sends a `SetPointerMapping` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_set_pointer_mapping_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_set_pointer_mapping_reply`]: Self::xcb_set_pointer_mapping_reply
    #[inline]
    pub unsafe fn xcb_set_pointer_mapping(
        &self,
        c: *mut xcb_connection_t,
        map_len: u8,
        map: *const u8,
    ) -> xcb_set_pointer_mapping_cookie_t {
        sym!(self, xcb_set_pointer_mapping)(c, map_len, map)
    }

    /// Returns `true` iff the symbol `xcb_set_pointer_mapping` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_pointer_mapping(&self) -> bool {
        has_sym!(self, xcb_set_pointer_mapping)
    }

    /// Sends a `SetPointerMapping` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_set_pointer_mapping_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_set_pointer_mapping_reply`]: Self::xcb_set_pointer_mapping_reply
    #[inline]
    pub unsafe fn xcb_set_pointer_mapping_unchecked(
        &self,
        c: *mut xcb_connection_t,
        map_len: u8,
        map: *const u8,
    ) -> xcb_set_pointer_mapping_cookie_t {
        sym!(self, xcb_set_pointer_mapping_unchecked)(c, map_len, map)
    }

    /// Returns `true` iff the symbol `xcb_set_pointer_mapping_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_pointer_mapping_unchecked(&self) -> bool {
        has_sym!(self, xcb_set_pointer_mapping_unchecked)
    }

    /// Waits for the reply to a `SetPointerMapping` request.
    #[inline]
    pub unsafe fn xcb_set_pointer_mapping_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_set_pointer_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_set_pointer_mapping_reply_t {
        sym!(self, xcb_set_pointer_mapping_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_set_pointer_mapping_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_pointer_mapping_reply(&self) -> bool {
        has_sym!(self, xcb_set_pointer_mapping_reply)
    }

    /// Computes the size of a `xcb_get_pointer_mapping_reply_t` object.
    #[inline]
    pub unsafe fn xcb_get_pointer_mapping_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_get_pointer_mapping_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_get_pointer_mapping_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_pointer_mapping_sizeof(&self) -> bool {
        has_sym!(self, xcb_get_pointer_mapping_sizeof)
    }

    /// Sends a `GetPointerMapping` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_pointer_mapping_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_pointer_mapping_reply`]: Self::xcb_get_pointer_mapping_reply
    #[inline]
    pub unsafe fn xcb_get_pointer_mapping(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_get_pointer_mapping_cookie_t {
        sym!(self, xcb_get_pointer_mapping)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_pointer_mapping` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_pointer_mapping(&self) -> bool {
        has_sym!(self, xcb_get_pointer_mapping)
    }

    /// Sends a `GetPointerMapping` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_pointer_mapping_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_pointer_mapping_reply`]: Self::xcb_get_pointer_mapping_reply
    #[inline]
    pub unsafe fn xcb_get_pointer_mapping_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_get_pointer_mapping_cookie_t {
        sym!(self, xcb_get_pointer_mapping_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_pointer_mapping_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_pointer_mapping_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_pointer_mapping_unchecked)
    }

    /// Returns a pointer to the `map` field of a `xcb_get_pointer_mapping_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_get_pointer_mapping_map(
        &self,
        r: *const xcb_get_pointer_mapping_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_get_pointer_mapping_map)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_pointer_mapping_map` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_pointer_mapping_map(&self) -> bool {
        has_sym!(self, xcb_get_pointer_mapping_map)
    }

    /// Returns the number of elements of the `map` field of a `xcb_get_pointer_mapping_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_get_pointer_mapping_map_length(
        &self,
        r: *const xcb_get_pointer_mapping_reply_t,
    ) -> c_int {
        sym!(self, xcb_get_pointer_mapping_map_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_pointer_mapping_map_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_pointer_mapping_map_length(&self) -> bool {
        has_sym!(self, xcb_get_pointer_mapping_map_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `map` field of a `xcb_get_pointer_mapping_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_get_pointer_mapping_map_end(
        &self,
        r: *const xcb_get_pointer_mapping_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_get_pointer_mapping_map_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_pointer_mapping_map_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_pointer_mapping_map_end(&self) -> bool {
        has_sym!(self, xcb_get_pointer_mapping_map_end)
    }

    /// Waits for the reply to a `GetPointerMapping` request.
    #[inline]
    pub unsafe fn xcb_get_pointer_mapping_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_pointer_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_pointer_mapping_reply_t {
        sym!(self, xcb_get_pointer_mapping_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_pointer_mapping_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_pointer_mapping_reply(&self) -> bool {
        has_sym!(self, xcb_get_pointer_mapping_reply)
    }

    /// Computes the size of a `xcb_set_modifier_mapping_request_t` object.
    #[inline]
    pub unsafe fn xcb_set_modifier_mapping_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_set_modifier_mapping_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_set_modifier_mapping_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_modifier_mapping_sizeof(&self) -> bool {
        has_sym!(self, xcb_set_modifier_mapping_sizeof)
    }

    /// Sends a `SetModifierMapping` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_set_modifier_mapping_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_set_modifier_mapping_reply`]: Self::xcb_set_modifier_mapping_reply
    #[inline]
    pub unsafe fn xcb_set_modifier_mapping(
        &self,
        c: *mut xcb_connection_t,
        keycodes_per_modifier: u8,
        keycodes: *const xcb_keycode_t,
    ) -> xcb_set_modifier_mapping_cookie_t {
        sym!(self, xcb_set_modifier_mapping)(c, keycodes_per_modifier, keycodes)
    }

    /// Returns `true` iff the symbol `xcb_set_modifier_mapping` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_modifier_mapping(&self) -> bool {
        has_sym!(self, xcb_set_modifier_mapping)
    }

    /// Sends a `SetModifierMapping` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_set_modifier_mapping_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_set_modifier_mapping_reply`]: Self::xcb_set_modifier_mapping_reply
    #[inline]
    pub unsafe fn xcb_set_modifier_mapping_unchecked(
        &self,
        c: *mut xcb_connection_t,
        keycodes_per_modifier: u8,
        keycodes: *const xcb_keycode_t,
    ) -> xcb_set_modifier_mapping_cookie_t {
        sym!(self, xcb_set_modifier_mapping_unchecked)(c, keycodes_per_modifier, keycodes)
    }

    /// Returns `true` iff the symbol `xcb_set_modifier_mapping_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_modifier_mapping_unchecked(&self) -> bool {
        has_sym!(self, xcb_set_modifier_mapping_unchecked)
    }

    /// Waits for the reply to a `SetModifierMapping` request.
    #[inline]
    pub unsafe fn xcb_set_modifier_mapping_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_set_modifier_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_set_modifier_mapping_reply_t {
        sym!(self, xcb_set_modifier_mapping_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_set_modifier_mapping_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_set_modifier_mapping_reply(&self) -> bool {
        has_sym!(self, xcb_set_modifier_mapping_reply)
    }

    /// Computes the size of a `xcb_get_modifier_mapping_reply_t` object.
    #[inline]
    pub unsafe fn xcb_get_modifier_mapping_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_get_modifier_mapping_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_get_modifier_mapping_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_modifier_mapping_sizeof(&self) -> bool {
        has_sym!(self, xcb_get_modifier_mapping_sizeof)
    }

    /// Sends a `GetModifierMapping` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_modifier_mapping_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_modifier_mapping_reply`]: Self::xcb_get_modifier_mapping_reply
    #[inline]
    pub unsafe fn xcb_get_modifier_mapping(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_get_modifier_mapping_cookie_t {
        sym!(self, xcb_get_modifier_mapping)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_modifier_mapping` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_modifier_mapping(&self) -> bool {
        has_sym!(self, xcb_get_modifier_mapping)
    }

    /// Sends a `GetModifierMapping` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_get_modifier_mapping_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_get_modifier_mapping_reply`]: Self::xcb_get_modifier_mapping_reply
    #[inline]
    pub unsafe fn xcb_get_modifier_mapping_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_get_modifier_mapping_cookie_t {
        sym!(self, xcb_get_modifier_mapping_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_get_modifier_mapping_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_modifier_mapping_unchecked(&self) -> bool {
        has_sym!(self, xcb_get_modifier_mapping_unchecked)
    }

    /// Returns a pointer to the `keycodes` field of a `xcb_get_modifier_mapping_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_get_modifier_mapping_keycodes(
        &self,
        r: *const xcb_get_modifier_mapping_reply_t,
    ) -> *mut xcb_keycode_t {
        sym!(self, xcb_get_modifier_mapping_keycodes)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_modifier_mapping_keycodes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_modifier_mapping_keycodes(&self) -> bool {
        has_sym!(self, xcb_get_modifier_mapping_keycodes)
    }

    /// Returns the number of elements of the `keycodes` field of a `xcb_get_modifier_mapping_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_get_modifier_mapping_keycodes_length(
        &self,
        r: *const xcb_get_modifier_mapping_reply_t,
    ) -> c_int {
        sym!(self, xcb_get_modifier_mapping_keycodes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_modifier_mapping_keycodes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_modifier_mapping_keycodes_length(&self) -> bool {
        has_sym!(self, xcb_get_modifier_mapping_keycodes_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `keycodes` field of a `xcb_get_modifier_mapping_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_get_modifier_mapping_keycodes_end(
        &self,
        r: *const xcb_get_modifier_mapping_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_get_modifier_mapping_keycodes_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_get_modifier_mapping_keycodes_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_modifier_mapping_keycodes_end(&self) -> bool {
        has_sym!(self, xcb_get_modifier_mapping_keycodes_end)
    }

    /// Waits for the reply to a `GetModifierMapping` request.
    #[inline]
    pub unsafe fn xcb_get_modifier_mapping_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_get_modifier_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_get_modifier_mapping_reply_t {
        sym!(self, xcb_get_modifier_mapping_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_get_modifier_mapping_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_get_modifier_mapping_reply(&self) -> bool {
        has_sym!(self, xcb_get_modifier_mapping_reply)
    }

    /// Sends a `NoOperation` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_no_operation_checked(&self, c: *mut xcb_connection_t) -> xcb_void_cookie_t {
        sym!(self, xcb_no_operation_checked)(c)
    }

    /// Returns `true` iff the symbol `xcb_no_operation_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_no_operation_checked(&self) -> bool {
        has_sym!(self, xcb_no_operation_checked)
    }

    /// Sends a `NoOperation` request (unchecked).
    #[inline]
    pub unsafe fn xcb_no_operation(&self, c: *mut xcb_connection_t) -> xcb_void_cookie_t {
        sym!(self, xcb_no_operation)(c)
    }

    /// Returns `true` iff the symbol `xcb_no_operation` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_no_operation(&self) -> bool {
        has_sym!(self, xcb_no_operation)
    }
}

#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::Xcb::load().unwrap() };
        assert!(lib.has_xcb_char2b_next());
        assert!(lib.has_xcb_char2b_end());
        assert!(lib.has_xcb_window_next());
        assert!(lib.has_xcb_window_end());
        assert!(lib.has_xcb_pixmap_next());
        assert!(lib.has_xcb_pixmap_end());
        assert!(lib.has_xcb_cursor_next());
        assert!(lib.has_xcb_cursor_end());
        assert!(lib.has_xcb_font_next());
        assert!(lib.has_xcb_font_end());
        assert!(lib.has_xcb_gcontext_next());
        assert!(lib.has_xcb_gcontext_end());
        assert!(lib.has_xcb_colormap_next());
        assert!(lib.has_xcb_colormap_end());
        assert!(lib.has_xcb_atom_next());
        assert!(lib.has_xcb_atom_end());
        assert!(lib.has_xcb_drawable_next());
        assert!(lib.has_xcb_drawable_end());
        assert!(lib.has_xcb_fontable_next());
        assert!(lib.has_xcb_fontable_end());
        assert!(lib.has_xcb_bool32_next());
        assert!(lib.has_xcb_bool32_end());
        assert!(lib.has_xcb_visualid_next());
        assert!(lib.has_xcb_visualid_end());
        assert!(lib.has_xcb_timestamp_next());
        assert!(lib.has_xcb_timestamp_end());
        assert!(lib.has_xcb_keysym_next());
        assert!(lib.has_xcb_keysym_end());
        assert!(lib.has_xcb_keycode_next());
        assert!(lib.has_xcb_keycode_end());
        assert!(lib.has_xcb_keycode32_next());
        assert!(lib.has_xcb_keycode32_end());
        assert!(lib.has_xcb_button_next());
        assert!(lib.has_xcb_button_end());
        assert!(lib.has_xcb_point_next());
        assert!(lib.has_xcb_point_end());
        assert!(lib.has_xcb_rectangle_next());
        assert!(lib.has_xcb_rectangle_end());
        assert!(lib.has_xcb_arc_next());
        assert!(lib.has_xcb_arc_end());
        assert!(lib.has_xcb_format_next());
        assert!(lib.has_xcb_format_end());
        assert!(lib.has_xcb_visualtype_next());
        assert!(lib.has_xcb_visualtype_end());
        assert!(lib.has_xcb_depth_sizeof());
        assert!(lib.has_xcb_depth_visuals());
        assert!(lib.has_xcb_depth_visuals_length());
        assert!(lib.has_xcb_depth_visuals_iterator());
        assert!(lib.has_xcb_depth_next());
        assert!(lib.has_xcb_depth_end());
        assert!(lib.has_xcb_screen_sizeof());
        assert!(lib.has_xcb_screen_allowed_depths_length());
        assert!(lib.has_xcb_screen_allowed_depths_iterator());
        assert!(lib.has_xcb_screen_next());
        assert!(lib.has_xcb_screen_end());
        assert!(lib.has_xcb_setup_request_sizeof());
        assert!(lib.has_xcb_setup_request_authorization_protocol_name());
        assert!(lib.has_xcb_setup_request_authorization_protocol_name_length());
        assert!(lib.has_xcb_setup_request_authorization_protocol_name_end());
        assert!(lib.has_xcb_setup_request_authorization_protocol_data());
        assert!(lib.has_xcb_setup_request_authorization_protocol_data_length());
        assert!(lib.has_xcb_setup_request_authorization_protocol_data_end());
        assert!(lib.has_xcb_setup_request_next());
        assert!(lib.has_xcb_setup_request_end());
        assert!(lib.has_xcb_setup_failed_sizeof());
        assert!(lib.has_xcb_setup_failed_reason());
        assert!(lib.has_xcb_setup_failed_reason_length());
        assert!(lib.has_xcb_setup_failed_reason_end());
        assert!(lib.has_xcb_setup_failed_next());
        assert!(lib.has_xcb_setup_failed_end());
        assert!(lib.has_xcb_setup_authenticate_sizeof());
        assert!(lib.has_xcb_setup_authenticate_reason());
        assert!(lib.has_xcb_setup_authenticate_reason_length());
        assert!(lib.has_xcb_setup_authenticate_reason_end());
        assert!(lib.has_xcb_setup_authenticate_next());
        assert!(lib.has_xcb_setup_authenticate_end());
        assert!(lib.has_xcb_setup_sizeof());
        assert!(lib.has_xcb_setup_vendor());
        assert!(lib.has_xcb_setup_vendor_length());
        assert!(lib.has_xcb_setup_vendor_end());
        assert!(lib.has_xcb_setup_pixmap_formats());
        assert!(lib.has_xcb_setup_pixmap_formats_length());
        assert!(lib.has_xcb_setup_pixmap_formats_iterator());
        assert!(lib.has_xcb_setup_roots_length());
        assert!(lib.has_xcb_setup_roots_iterator());
        assert!(lib.has_xcb_setup_next());
        assert!(lib.has_xcb_setup_end());
        assert!(lib.has_xcb_client_message_data_next());
        assert!(lib.has_xcb_client_message_data_end());
        assert!(lib.has_xcb_create_window_value_list_serialize());
        assert!(lib.has_xcb_create_window_value_list_unpack());
        assert!(lib.has_xcb_create_window_value_list_sizeof());
        assert!(lib.has_xcb_create_window_sizeof());
        assert!(lib.has_xcb_create_window_checked());
        assert!(lib.has_xcb_create_window());
        assert!(lib.has_xcb_create_window_aux_checked());
        assert!(lib.has_xcb_create_window_aux());
        assert!(lib.has_xcb_create_window_value_list());
        assert!(lib.has_xcb_change_window_attributes_value_list_serialize());
        assert!(lib.has_xcb_change_window_attributes_value_list_unpack());
        assert!(lib.has_xcb_change_window_attributes_value_list_sizeof());
        assert!(lib.has_xcb_change_window_attributes_sizeof());
        assert!(lib.has_xcb_change_window_attributes_checked());
        assert!(lib.has_xcb_change_window_attributes());
        assert!(lib.has_xcb_change_window_attributes_aux_checked());
        assert!(lib.has_xcb_change_window_attributes_aux());
        assert!(lib.has_xcb_change_window_attributes_value_list());
        assert!(lib.has_xcb_get_window_attributes());
        assert!(lib.has_xcb_get_window_attributes_unchecked());
        assert!(lib.has_xcb_get_window_attributes_reply());
        assert!(lib.has_xcb_destroy_window_checked());
        assert!(lib.has_xcb_destroy_window());
        assert!(lib.has_xcb_destroy_subwindows_checked());
        assert!(lib.has_xcb_destroy_subwindows());
        assert!(lib.has_xcb_change_save_set_checked());
        assert!(lib.has_xcb_change_save_set());
        assert!(lib.has_xcb_reparent_window_checked());
        assert!(lib.has_xcb_reparent_window());
        assert!(lib.has_xcb_map_window_checked());
        assert!(lib.has_xcb_map_window());
        assert!(lib.has_xcb_map_subwindows_checked());
        assert!(lib.has_xcb_map_subwindows());
        assert!(lib.has_xcb_unmap_window_checked());
        assert!(lib.has_xcb_unmap_window());
        assert!(lib.has_xcb_unmap_subwindows_checked());
        assert!(lib.has_xcb_unmap_subwindows());
        assert!(lib.has_xcb_configure_window_value_list_serialize());
        assert!(lib.has_xcb_configure_window_value_list_unpack());
        assert!(lib.has_xcb_configure_window_value_list_sizeof());
        assert!(lib.has_xcb_configure_window_sizeof());
        assert!(lib.has_xcb_configure_window_checked());
        assert!(lib.has_xcb_configure_window());
        assert!(lib.has_xcb_configure_window_aux_checked());
        assert!(lib.has_xcb_configure_window_aux());
        assert!(lib.has_xcb_configure_window_value_list());
        assert!(lib.has_xcb_circulate_window_checked());
        assert!(lib.has_xcb_circulate_window());
        assert!(lib.has_xcb_get_geometry());
        assert!(lib.has_xcb_get_geometry_unchecked());
        assert!(lib.has_xcb_get_geometry_reply());
        assert!(lib.has_xcb_query_tree_sizeof());
        assert!(lib.has_xcb_query_tree());
        assert!(lib.has_xcb_query_tree_unchecked());
        assert!(lib.has_xcb_query_tree_children());
        assert!(lib.has_xcb_query_tree_children_length());
        assert!(lib.has_xcb_query_tree_children_end());
        assert!(lib.has_xcb_query_tree_reply());
        assert!(lib.has_xcb_intern_atom_sizeof());
        assert!(lib.has_xcb_intern_atom());
        assert!(lib.has_xcb_intern_atom_unchecked());
        assert!(lib.has_xcb_intern_atom_reply());
        assert!(lib.has_xcb_get_atom_name_sizeof());
        assert!(lib.has_xcb_get_atom_name());
        assert!(lib.has_xcb_get_atom_name_unchecked());
        assert!(lib.has_xcb_get_atom_name_name());
        assert!(lib.has_xcb_get_atom_name_name_length());
        assert!(lib.has_xcb_get_atom_name_name_end());
        assert!(lib.has_xcb_get_atom_name_reply());
        assert!(lib.has_xcb_change_property_sizeof());
        assert!(lib.has_xcb_change_property_checked());
        assert!(lib.has_xcb_change_property());
        assert!(lib.has_xcb_change_property_data());
        assert!(lib.has_xcb_change_property_data_length());
        assert!(lib.has_xcb_change_property_data_end());
        assert!(lib.has_xcb_delete_property_checked());
        assert!(lib.has_xcb_delete_property());
        assert!(lib.has_xcb_get_property_sizeof());
        assert!(lib.has_xcb_get_property());
        assert!(lib.has_xcb_get_property_unchecked());
        assert!(lib.has_xcb_get_property_value());
        assert!(lib.has_xcb_get_property_value_length());
        assert!(lib.has_xcb_get_property_value_end());
        assert!(lib.has_xcb_get_property_reply());
        assert!(lib.has_xcb_list_properties_sizeof());
        assert!(lib.has_xcb_list_properties());
        assert!(lib.has_xcb_list_properties_unchecked());
        assert!(lib.has_xcb_list_properties_atoms());
        assert!(lib.has_xcb_list_properties_atoms_length());
        assert!(lib.has_xcb_list_properties_atoms_end());
        assert!(lib.has_xcb_list_properties_reply());
        assert!(lib.has_xcb_set_selection_owner_checked());
        assert!(lib.has_xcb_set_selection_owner());
        assert!(lib.has_xcb_get_selection_owner());
        assert!(lib.has_xcb_get_selection_owner_unchecked());
        assert!(lib.has_xcb_get_selection_owner_reply());
        assert!(lib.has_xcb_convert_selection_checked());
        assert!(lib.has_xcb_convert_selection());
        assert!(lib.has_xcb_send_event_checked());
        assert!(lib.has_xcb_send_event());
        assert!(lib.has_xcb_grab_pointer());
        assert!(lib.has_xcb_grab_pointer_unchecked());
        assert!(lib.has_xcb_grab_pointer_reply());
        assert!(lib.has_xcb_ungrab_pointer_checked());
        assert!(lib.has_xcb_ungrab_pointer());
        assert!(lib.has_xcb_grab_button_checked());
        assert!(lib.has_xcb_grab_button());
        assert!(lib.has_xcb_ungrab_button_checked());
        assert!(lib.has_xcb_ungrab_button());
        assert!(lib.has_xcb_change_active_pointer_grab_checked());
        assert!(lib.has_xcb_change_active_pointer_grab());
        assert!(lib.has_xcb_grab_keyboard());
        assert!(lib.has_xcb_grab_keyboard_unchecked());
        assert!(lib.has_xcb_grab_keyboard_reply());
        assert!(lib.has_xcb_ungrab_keyboard_checked());
        assert!(lib.has_xcb_ungrab_keyboard());
        assert!(lib.has_xcb_grab_key_checked());
        assert!(lib.has_xcb_grab_key());
        assert!(lib.has_xcb_ungrab_key_checked());
        assert!(lib.has_xcb_ungrab_key());
        assert!(lib.has_xcb_allow_events_checked());
        assert!(lib.has_xcb_allow_events());
        assert!(lib.has_xcb_grab_server_checked());
        assert!(lib.has_xcb_grab_server());
        assert!(lib.has_xcb_ungrab_server_checked());
        assert!(lib.has_xcb_ungrab_server());
        assert!(lib.has_xcb_query_pointer());
        assert!(lib.has_xcb_query_pointer_unchecked());
        assert!(lib.has_xcb_query_pointer_reply());
        assert!(lib.has_xcb_timecoord_next());
        assert!(lib.has_xcb_timecoord_end());
        assert!(lib.has_xcb_get_motion_events_sizeof());
        assert!(lib.has_xcb_get_motion_events());
        assert!(lib.has_xcb_get_motion_events_unchecked());
        assert!(lib.has_xcb_get_motion_events_events());
        assert!(lib.has_xcb_get_motion_events_events_length());
        assert!(lib.has_xcb_get_motion_events_events_iterator());
        assert!(lib.has_xcb_get_motion_events_reply());
        assert!(lib.has_xcb_translate_coordinates());
        assert!(lib.has_xcb_translate_coordinates_unchecked());
        assert!(lib.has_xcb_translate_coordinates_reply());
        assert!(lib.has_xcb_warp_pointer_checked());
        assert!(lib.has_xcb_warp_pointer());
        assert!(lib.has_xcb_set_input_focus_checked());
        assert!(lib.has_xcb_set_input_focus());
        assert!(lib.has_xcb_get_input_focus());
        assert!(lib.has_xcb_get_input_focus_unchecked());
        assert!(lib.has_xcb_get_input_focus_reply());
        assert!(lib.has_xcb_query_keymap());
        assert!(lib.has_xcb_query_keymap_unchecked());
        assert!(lib.has_xcb_query_keymap_reply());
        assert!(lib.has_xcb_open_font_sizeof());
        assert!(lib.has_xcb_open_font_checked());
        assert!(lib.has_xcb_open_font());
        assert!(lib.has_xcb_open_font_name());
        assert!(lib.has_xcb_open_font_name_length());
        assert!(lib.has_xcb_open_font_name_end());
        assert!(lib.has_xcb_close_font_checked());
        assert!(lib.has_xcb_close_font());
        assert!(lib.has_xcb_fontprop_next());
        assert!(lib.has_xcb_fontprop_end());
        assert!(lib.has_xcb_charinfo_next());
        assert!(lib.has_xcb_charinfo_end());
        assert!(lib.has_xcb_query_font_sizeof());
        assert!(lib.has_xcb_query_font());
        assert!(lib.has_xcb_query_font_unchecked());
        assert!(lib.has_xcb_query_font_properties());
        assert!(lib.has_xcb_query_font_properties_length());
        assert!(lib.has_xcb_query_font_properties_iterator());
        assert!(lib.has_xcb_query_font_char_infos());
        assert!(lib.has_xcb_query_font_char_infos_length());
        assert!(lib.has_xcb_query_font_char_infos_iterator());
        assert!(lib.has_xcb_query_font_reply());
        assert!(lib.has_xcb_query_text_extents_sizeof());
        assert!(lib.has_xcb_query_text_extents());
        assert!(lib.has_xcb_query_text_extents_unchecked());
        assert!(lib.has_xcb_query_text_extents_reply());
        assert!(lib.has_xcb_str_sizeof());
        assert!(lib.has_xcb_str_name());
        assert!(lib.has_xcb_str_name_length());
        assert!(lib.has_xcb_str_name_end());
        assert!(lib.has_xcb_str_next());
        assert!(lib.has_xcb_str_end());
        assert!(lib.has_xcb_list_fonts_sizeof());
        assert!(lib.has_xcb_list_fonts());
        assert!(lib.has_xcb_list_fonts_unchecked());
        assert!(lib.has_xcb_list_fonts_names_length());
        assert!(lib.has_xcb_list_fonts_names_iterator());
        assert!(lib.has_xcb_list_fonts_reply());
        assert!(lib.has_xcb_list_fonts_with_info_sizeof());
        assert!(lib.has_xcb_list_fonts_with_info());
        assert!(lib.has_xcb_list_fonts_with_info_unchecked());
        assert!(lib.has_xcb_list_fonts_with_info_properties());
        assert!(lib.has_xcb_list_fonts_with_info_properties_length());
        assert!(lib.has_xcb_list_fonts_with_info_properties_iterator());
        assert!(lib.has_xcb_list_fonts_with_info_name());
        assert!(lib.has_xcb_list_fonts_with_info_name_length());
        assert!(lib.has_xcb_list_fonts_with_info_name_end());
        assert!(lib.has_xcb_list_fonts_with_info_reply());
        assert!(lib.has_xcb_set_font_path_sizeof());
        assert!(lib.has_xcb_set_font_path_checked());
        assert!(lib.has_xcb_set_font_path());
        assert!(lib.has_xcb_set_font_path_font_length());
        assert!(lib.has_xcb_set_font_path_font_iterator());
        assert!(lib.has_xcb_get_font_path_sizeof());
        assert!(lib.has_xcb_get_font_path());
        assert!(lib.has_xcb_get_font_path_unchecked());
        assert!(lib.has_xcb_get_font_path_path_length());
        assert!(lib.has_xcb_get_font_path_path_iterator());
        assert!(lib.has_xcb_get_font_path_reply());
        assert!(lib.has_xcb_create_pixmap_checked());
        assert!(lib.has_xcb_create_pixmap());
        assert!(lib.has_xcb_free_pixmap_checked());
        assert!(lib.has_xcb_free_pixmap());
        assert!(lib.has_xcb_create_gc_value_list_serialize());
        assert!(lib.has_xcb_create_gc_value_list_unpack());
        assert!(lib.has_xcb_create_gc_value_list_sizeof());
        assert!(lib.has_xcb_create_gc_sizeof());
        assert!(lib.has_xcb_create_gc_checked());
        assert!(lib.has_xcb_create_gc());
        assert!(lib.has_xcb_create_gc_aux_checked());
        assert!(lib.has_xcb_create_gc_aux());
        assert!(lib.has_xcb_create_gc_value_list());
        assert!(lib.has_xcb_change_gc_value_list_serialize());
        assert!(lib.has_xcb_change_gc_value_list_unpack());
        assert!(lib.has_xcb_change_gc_value_list_sizeof());
        assert!(lib.has_xcb_change_gc_sizeof());
        assert!(lib.has_xcb_change_gc_checked());
        assert!(lib.has_xcb_change_gc());
        assert!(lib.has_xcb_change_gc_aux_checked());
        assert!(lib.has_xcb_change_gc_aux());
        assert!(lib.has_xcb_change_gc_value_list());
        assert!(lib.has_xcb_copy_gc_checked());
        assert!(lib.has_xcb_copy_gc());
        assert!(lib.has_xcb_set_dashes_sizeof());
        assert!(lib.has_xcb_set_dashes_checked());
        assert!(lib.has_xcb_set_dashes());
        assert!(lib.has_xcb_set_dashes_dashes());
        assert!(lib.has_xcb_set_dashes_dashes_length());
        assert!(lib.has_xcb_set_dashes_dashes_end());
        assert!(lib.has_xcb_set_clip_rectangles_sizeof());
        assert!(lib.has_xcb_set_clip_rectangles_checked());
        assert!(lib.has_xcb_set_clip_rectangles());
        assert!(lib.has_xcb_set_clip_rectangles_rectangles());
        assert!(lib.has_xcb_set_clip_rectangles_rectangles_length());
        assert!(lib.has_xcb_set_clip_rectangles_rectangles_iterator());
        assert!(lib.has_xcb_free_gc_checked());
        assert!(lib.has_xcb_free_gc());
        assert!(lib.has_xcb_clear_area_checked());
        assert!(lib.has_xcb_clear_area());
        assert!(lib.has_xcb_copy_area_checked());
        assert!(lib.has_xcb_copy_area());
        assert!(lib.has_xcb_copy_plane_checked());
        assert!(lib.has_xcb_copy_plane());
        assert!(lib.has_xcb_poly_point_sizeof());
        assert!(lib.has_xcb_poly_point_checked());
        assert!(lib.has_xcb_poly_point());
        assert!(lib.has_xcb_poly_point_points());
        assert!(lib.has_xcb_poly_point_points_length());
        assert!(lib.has_xcb_poly_point_points_iterator());
        assert!(lib.has_xcb_poly_line_sizeof());
        assert!(lib.has_xcb_poly_line_checked());
        assert!(lib.has_xcb_poly_line());
        assert!(lib.has_xcb_poly_line_points());
        assert!(lib.has_xcb_poly_line_points_length());
        assert!(lib.has_xcb_poly_line_points_iterator());
        assert!(lib.has_xcb_segment_next());
        assert!(lib.has_xcb_segment_end());
        assert!(lib.has_xcb_poly_segment_sizeof());
        assert!(lib.has_xcb_poly_segment_checked());
        assert!(lib.has_xcb_poly_segment());
        assert!(lib.has_xcb_poly_segment_segments());
        assert!(lib.has_xcb_poly_segment_segments_length());
        assert!(lib.has_xcb_poly_segment_segments_iterator());
        assert!(lib.has_xcb_poly_rectangle_sizeof());
        assert!(lib.has_xcb_poly_rectangle_checked());
        assert!(lib.has_xcb_poly_rectangle());
        assert!(lib.has_xcb_poly_rectangle_rectangles());
        assert!(lib.has_xcb_poly_rectangle_rectangles_length());
        assert!(lib.has_xcb_poly_rectangle_rectangles_iterator());
        assert!(lib.has_xcb_poly_arc_sizeof());
        assert!(lib.has_xcb_poly_arc_checked());
        assert!(lib.has_xcb_poly_arc());
        assert!(lib.has_xcb_poly_arc_arcs());
        assert!(lib.has_xcb_poly_arc_arcs_length());
        assert!(lib.has_xcb_poly_arc_arcs_iterator());
        assert!(lib.has_xcb_fill_poly_sizeof());
        assert!(lib.has_xcb_fill_poly_checked());
        assert!(lib.has_xcb_fill_poly());
        assert!(lib.has_xcb_fill_poly_points());
        assert!(lib.has_xcb_fill_poly_points_length());
        assert!(lib.has_xcb_fill_poly_points_iterator());
        assert!(lib.has_xcb_poly_fill_rectangle_sizeof());
        assert!(lib.has_xcb_poly_fill_rectangle_checked());
        assert!(lib.has_xcb_poly_fill_rectangle());
        assert!(lib.has_xcb_poly_fill_rectangle_rectangles());
        assert!(lib.has_xcb_poly_fill_rectangle_rectangles_length());
        assert!(lib.has_xcb_poly_fill_rectangle_rectangles_iterator());
        assert!(lib.has_xcb_poly_fill_arc_sizeof());
        assert!(lib.has_xcb_poly_fill_arc_checked());
        assert!(lib.has_xcb_poly_fill_arc());
        assert!(lib.has_xcb_poly_fill_arc_arcs());
        assert!(lib.has_xcb_poly_fill_arc_arcs_length());
        assert!(lib.has_xcb_poly_fill_arc_arcs_iterator());
        assert!(lib.has_xcb_put_image_sizeof());
        assert!(lib.has_xcb_put_image_checked());
        assert!(lib.has_xcb_put_image());
        assert!(lib.has_xcb_put_image_data());
        assert!(lib.has_xcb_put_image_data_length());
        assert!(lib.has_xcb_put_image_data_end());
        assert!(lib.has_xcb_get_image_sizeof());
        assert!(lib.has_xcb_get_image());
        assert!(lib.has_xcb_get_image_unchecked());
        assert!(lib.has_xcb_get_image_data());
        assert!(lib.has_xcb_get_image_data_length());
        assert!(lib.has_xcb_get_image_data_end());
        assert!(lib.has_xcb_get_image_reply());
        assert!(lib.has_xcb_poly_text_8_sizeof());
        assert!(lib.has_xcb_poly_text_8_checked());
        assert!(lib.has_xcb_poly_text_8());
        assert!(lib.has_xcb_poly_text_8_items());
        assert!(lib.has_xcb_poly_text_8_items_length());
        assert!(lib.has_xcb_poly_text_8_items_end());
        assert!(lib.has_xcb_poly_text_16_sizeof());
        assert!(lib.has_xcb_poly_text_16_checked());
        assert!(lib.has_xcb_poly_text_16());
        assert!(lib.has_xcb_poly_text_16_items());
        assert!(lib.has_xcb_poly_text_16_items_length());
        assert!(lib.has_xcb_poly_text_16_items_end());
        assert!(lib.has_xcb_image_text_8_sizeof());
        assert!(lib.has_xcb_image_text_8_checked());
        assert!(lib.has_xcb_image_text_8());
        assert!(lib.has_xcb_image_text_8_string());
        assert!(lib.has_xcb_image_text_8_string_length());
        assert!(lib.has_xcb_image_text_8_string_end());
        assert!(lib.has_xcb_image_text_16_sizeof());
        assert!(lib.has_xcb_image_text_16_checked());
        assert!(lib.has_xcb_image_text_16());
        assert!(lib.has_xcb_image_text_16_string());
        assert!(lib.has_xcb_image_text_16_string_length());
        assert!(lib.has_xcb_image_text_16_string_iterator());
        assert!(lib.has_xcb_create_colormap_checked());
        assert!(lib.has_xcb_create_colormap());
        assert!(lib.has_xcb_free_colormap_checked());
        assert!(lib.has_xcb_free_colormap());
        assert!(lib.has_xcb_copy_colormap_and_free_checked());
        assert!(lib.has_xcb_copy_colormap_and_free());
        assert!(lib.has_xcb_install_colormap_checked());
        assert!(lib.has_xcb_install_colormap());
        assert!(lib.has_xcb_uninstall_colormap_checked());
        assert!(lib.has_xcb_uninstall_colormap());
        assert!(lib.has_xcb_list_installed_colormaps_sizeof());
        assert!(lib.has_xcb_list_installed_colormaps());
        assert!(lib.has_xcb_list_installed_colormaps_unchecked());
        assert!(lib.has_xcb_list_installed_colormaps_cmaps());
        assert!(lib.has_xcb_list_installed_colormaps_cmaps_length());
        assert!(lib.has_xcb_list_installed_colormaps_cmaps_end());
        assert!(lib.has_xcb_list_installed_colormaps_reply());
        assert!(lib.has_xcb_alloc_color());
        assert!(lib.has_xcb_alloc_color_unchecked());
        assert!(lib.has_xcb_alloc_color_reply());
        assert!(lib.has_xcb_alloc_named_color_sizeof());
        assert!(lib.has_xcb_alloc_named_color());
        assert!(lib.has_xcb_alloc_named_color_unchecked());
        assert!(lib.has_xcb_alloc_named_color_reply());
        assert!(lib.has_xcb_alloc_color_cells_sizeof());
        assert!(lib.has_xcb_alloc_color_cells());
        assert!(lib.has_xcb_alloc_color_cells_unchecked());
        assert!(lib.has_xcb_alloc_color_cells_pixels());
        assert!(lib.has_xcb_alloc_color_cells_pixels_length());
        assert!(lib.has_xcb_alloc_color_cells_pixels_end());
        assert!(lib.has_xcb_alloc_color_cells_masks());
        assert!(lib.has_xcb_alloc_color_cells_masks_length());
        assert!(lib.has_xcb_alloc_color_cells_masks_end());
        assert!(lib.has_xcb_alloc_color_cells_reply());
        assert!(lib.has_xcb_alloc_color_planes_sizeof());
        assert!(lib.has_xcb_alloc_color_planes());
        assert!(lib.has_xcb_alloc_color_planes_unchecked());
        assert!(lib.has_xcb_alloc_color_planes_pixels());
        assert!(lib.has_xcb_alloc_color_planes_pixels_length());
        assert!(lib.has_xcb_alloc_color_planes_pixels_end());
        assert!(lib.has_xcb_alloc_color_planes_reply());
        assert!(lib.has_xcb_free_colors_sizeof());
        assert!(lib.has_xcb_free_colors_checked());
        assert!(lib.has_xcb_free_colors());
        assert!(lib.has_xcb_free_colors_pixels());
        assert!(lib.has_xcb_free_colors_pixels_length());
        assert!(lib.has_xcb_free_colors_pixels_end());
        assert!(lib.has_xcb_coloritem_next());
        assert!(lib.has_xcb_coloritem_end());
        assert!(lib.has_xcb_store_colors_sizeof());
        assert!(lib.has_xcb_store_colors_checked());
        assert!(lib.has_xcb_store_colors());
        assert!(lib.has_xcb_store_colors_items());
        assert!(lib.has_xcb_store_colors_items_length());
        assert!(lib.has_xcb_store_colors_items_iterator());
        assert!(lib.has_xcb_store_named_color_sizeof());
        assert!(lib.has_xcb_store_named_color_checked());
        assert!(lib.has_xcb_store_named_color());
        assert!(lib.has_xcb_store_named_color_name());
        assert!(lib.has_xcb_store_named_color_name_length());
        assert!(lib.has_xcb_store_named_color_name_end());
        assert!(lib.has_xcb_rgb_next());
        assert!(lib.has_xcb_rgb_end());
        assert!(lib.has_xcb_query_colors_sizeof());
        assert!(lib.has_xcb_query_colors());
        assert!(lib.has_xcb_query_colors_unchecked());
        assert!(lib.has_xcb_query_colors_colors());
        assert!(lib.has_xcb_query_colors_colors_length());
        assert!(lib.has_xcb_query_colors_colors_iterator());
        assert!(lib.has_xcb_query_colors_reply());
        assert!(lib.has_xcb_lookup_color_sizeof());
        assert!(lib.has_xcb_lookup_color());
        assert!(lib.has_xcb_lookup_color_unchecked());
        assert!(lib.has_xcb_lookup_color_reply());
        assert!(lib.has_xcb_create_cursor_checked());
        assert!(lib.has_xcb_create_cursor());
        assert!(lib.has_xcb_create_glyph_cursor_checked());
        assert!(lib.has_xcb_create_glyph_cursor());
        assert!(lib.has_xcb_free_cursor_checked());
        assert!(lib.has_xcb_free_cursor());
        assert!(lib.has_xcb_recolor_cursor_checked());
        assert!(lib.has_xcb_recolor_cursor());
        assert!(lib.has_xcb_query_best_size());
        assert!(lib.has_xcb_query_best_size_unchecked());
        assert!(lib.has_xcb_query_best_size_reply());
        assert!(lib.has_xcb_query_extension_sizeof());
        assert!(lib.has_xcb_query_extension());
        assert!(lib.has_xcb_query_extension_unchecked());
        assert!(lib.has_xcb_query_extension_reply());
        assert!(lib.has_xcb_list_extensions_sizeof());
        assert!(lib.has_xcb_list_extensions());
        assert!(lib.has_xcb_list_extensions_unchecked());
        assert!(lib.has_xcb_list_extensions_names_length());
        assert!(lib.has_xcb_list_extensions_names_iterator());
        assert!(lib.has_xcb_list_extensions_reply());
        assert!(lib.has_xcb_change_keyboard_mapping_sizeof());
        assert!(lib.has_xcb_change_keyboard_mapping_checked());
        assert!(lib.has_xcb_change_keyboard_mapping());
        assert!(lib.has_xcb_change_keyboard_mapping_keysyms());
        assert!(lib.has_xcb_change_keyboard_mapping_keysyms_length());
        assert!(lib.has_xcb_change_keyboard_mapping_keysyms_end());
        assert!(lib.has_xcb_get_keyboard_mapping_sizeof());
        assert!(lib.has_xcb_get_keyboard_mapping());
        assert!(lib.has_xcb_get_keyboard_mapping_unchecked());
        assert!(lib.has_xcb_get_keyboard_mapping_keysyms());
        assert!(lib.has_xcb_get_keyboard_mapping_keysyms_length());
        assert!(lib.has_xcb_get_keyboard_mapping_keysyms_end());
        assert!(lib.has_xcb_get_keyboard_mapping_reply());
        assert!(lib.has_xcb_change_keyboard_control_value_list_serialize());
        assert!(lib.has_xcb_change_keyboard_control_value_list_unpack());
        assert!(lib.has_xcb_change_keyboard_control_value_list_sizeof());
        assert!(lib.has_xcb_change_keyboard_control_sizeof());
        assert!(lib.has_xcb_change_keyboard_control_checked());
        assert!(lib.has_xcb_change_keyboard_control());
        assert!(lib.has_xcb_change_keyboard_control_aux_checked());
        assert!(lib.has_xcb_change_keyboard_control_aux());
        assert!(lib.has_xcb_change_keyboard_control_value_list());
        assert!(lib.has_xcb_get_keyboard_control());
        assert!(lib.has_xcb_get_keyboard_control_unchecked());
        assert!(lib.has_xcb_get_keyboard_control_reply());
        assert!(lib.has_xcb_bell_checked());
        assert!(lib.has_xcb_bell());
        assert!(lib.has_xcb_change_pointer_control_checked());
        assert!(lib.has_xcb_change_pointer_control());
        assert!(lib.has_xcb_get_pointer_control());
        assert!(lib.has_xcb_get_pointer_control_unchecked());
        assert!(lib.has_xcb_get_pointer_control_reply());
        assert!(lib.has_xcb_set_screen_saver_checked());
        assert!(lib.has_xcb_set_screen_saver());
        assert!(lib.has_xcb_get_screen_saver());
        assert!(lib.has_xcb_get_screen_saver_unchecked());
        assert!(lib.has_xcb_get_screen_saver_reply());
        assert!(lib.has_xcb_change_hosts_sizeof());
        assert!(lib.has_xcb_change_hosts_checked());
        assert!(lib.has_xcb_change_hosts());
        assert!(lib.has_xcb_change_hosts_address());
        assert!(lib.has_xcb_change_hosts_address_length());
        assert!(lib.has_xcb_change_hosts_address_end());
        assert!(lib.has_xcb_host_sizeof());
        assert!(lib.has_xcb_host_address());
        assert!(lib.has_xcb_host_address_length());
        assert!(lib.has_xcb_host_address_end());
        assert!(lib.has_xcb_host_next());
        assert!(lib.has_xcb_host_end());
        assert!(lib.has_xcb_list_hosts_sizeof());
        assert!(lib.has_xcb_list_hosts());
        assert!(lib.has_xcb_list_hosts_unchecked());
        assert!(lib.has_xcb_list_hosts_hosts_length());
        assert!(lib.has_xcb_list_hosts_hosts_iterator());
        assert!(lib.has_xcb_list_hosts_reply());
        assert!(lib.has_xcb_set_access_control_checked());
        assert!(lib.has_xcb_set_access_control());
        assert!(lib.has_xcb_set_close_down_mode_checked());
        assert!(lib.has_xcb_set_close_down_mode());
        assert!(lib.has_xcb_kill_client_checked());
        assert!(lib.has_xcb_kill_client());
        assert!(lib.has_xcb_rotate_properties_sizeof());
        assert!(lib.has_xcb_rotate_properties_checked());
        assert!(lib.has_xcb_rotate_properties());
        assert!(lib.has_xcb_rotate_properties_atoms());
        assert!(lib.has_xcb_rotate_properties_atoms_length());
        assert!(lib.has_xcb_rotate_properties_atoms_end());
        assert!(lib.has_xcb_force_screen_saver_checked());
        assert!(lib.has_xcb_force_screen_saver());
        assert!(lib.has_xcb_set_pointer_mapping_sizeof());
        assert!(lib.has_xcb_set_pointer_mapping());
        assert!(lib.has_xcb_set_pointer_mapping_unchecked());
        assert!(lib.has_xcb_set_pointer_mapping_reply());
        assert!(lib.has_xcb_get_pointer_mapping_sizeof());
        assert!(lib.has_xcb_get_pointer_mapping());
        assert!(lib.has_xcb_get_pointer_mapping_unchecked());
        assert!(lib.has_xcb_get_pointer_mapping_map());
        assert!(lib.has_xcb_get_pointer_mapping_map_length());
        assert!(lib.has_xcb_get_pointer_mapping_map_end());
        assert!(lib.has_xcb_get_pointer_mapping_reply());
        assert!(lib.has_xcb_set_modifier_mapping_sizeof());
        assert!(lib.has_xcb_set_modifier_mapping());
        assert!(lib.has_xcb_set_modifier_mapping_unchecked());
        assert!(lib.has_xcb_set_modifier_mapping_reply());
        assert!(lib.has_xcb_get_modifier_mapping_sizeof());
        assert!(lib.has_xcb_get_modifier_mapping());
        assert!(lib.has_xcb_get_modifier_mapping_unchecked());
        assert!(lib.has_xcb_get_modifier_mapping_keycodes());
        assert!(lib.has_xcb_get_modifier_mapping_keycodes_length());
        assert!(lib.has_xcb_get_modifier_mapping_keycodes_end());
        assert!(lib.has_xcb_get_modifier_mapping_reply());
        assert!(lib.has_xcb_no_operation_checked());
        assert!(lib.has_xcb_no_operation());
    }
}
