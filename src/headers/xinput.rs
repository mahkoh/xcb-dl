// This file was generated using generate.py. Do not edit.
#![allow(unused_macros)]

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

/// The `Input::EventClass` type.
pub type xcb_input_event_class_t = u32;

/// An iterator over `Input::EventClass` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_event_class_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_event_class_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_event_class_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::KeyCode` type.
pub type xcb_input_key_code_t = u8;

/// An iterator over `Input::KeyCode` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_key_code_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_key_code_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_key_code_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::DeviceId` type.
pub type xcb_input_device_id_t = u16;

/// An iterator over `Input::DeviceId` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_id_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_device_id_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_device_id_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::FP1616` type.
pub type xcb_input_fp1616_t = i32;

/// An iterator over `Input::FP1616` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_fp1616_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_fp1616_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_fp1616_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::FP3232` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_fp3232_t {
    pub integral: i32,
    pub frac: u32,
}

impl Default for xcb_input_fp3232_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::FP3232` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_fp3232_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_fp3232_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_fp3232_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::GetExtensionVersion` request.
///
/// Pass this cookie to [`xcb_input_get_extension_version_reply`] to retrieve the reply.
///
/// [`xcb_input_get_extension_version_reply`]: XcbXinput::xcb_input_get_extension_version_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_extension_version_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_get_extension_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::GetExtensionVersion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_get_extension_version_request_t`].
pub const XCB_INPUT_GET_EXTENSION_VERSION: u8 = 1i32 as u8;

/// The `Input::GetExtensionVersion` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `name`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_extension_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub name_len: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_get_extension_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::GetExtensionVersion` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_extension_version_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub server_major: u16,
    pub server_minor: u16,
    pub present: u8,
    pub pad0: [u8; 19],
}

impl Default for xcb_input_get_extension_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::DeviceUse` enum.
///
/// This enum has the following variants:
///
/// - [`Input::DeviceUse::IsXPointer`](XCB_INPUT_DEVICE_USE_IS_X_POINTER)
/// - [`Input::DeviceUse::IsXKeyboard`](XCB_INPUT_DEVICE_USE_IS_X_KEYBOARD)
/// - [`Input::DeviceUse::IsXExtensionDevice`](XCB_INPUT_DEVICE_USE_IS_X_EXTENSION_DEVICE)
/// - [`Input::DeviceUse::IsXExtensionKeyboard`](XCB_INPUT_DEVICE_USE_IS_X_EXTENSION_KEYBOARD)
/// - [`Input::DeviceUse::IsXExtensionPointer`](XCB_INPUT_DEVICE_USE_IS_X_EXTENSION_POINTER)
pub type xcb_input_device_use_t = u32;
/// The `Input::DeviceUse::IsXPointer` enum variant.
///
/// This is a variant of [`xcb_input_device_use_t`].
pub const XCB_INPUT_DEVICE_USE_IS_X_POINTER: xcb_input_device_use_t = 0;
/// The `Input::DeviceUse::IsXKeyboard` enum variant.
///
/// This is a variant of [`xcb_input_device_use_t`].
pub const XCB_INPUT_DEVICE_USE_IS_X_KEYBOARD: xcb_input_device_use_t = 1;
/// The `Input::DeviceUse::IsXExtensionDevice` enum variant.
///
/// This is a variant of [`xcb_input_device_use_t`].
pub const XCB_INPUT_DEVICE_USE_IS_X_EXTENSION_DEVICE: xcb_input_device_use_t = 2;
/// The `Input::DeviceUse::IsXExtensionKeyboard` enum variant.
///
/// This is a variant of [`xcb_input_device_use_t`].
pub const XCB_INPUT_DEVICE_USE_IS_X_EXTENSION_KEYBOARD: xcb_input_device_use_t = 3;
/// The `Input::DeviceUse::IsXExtensionPointer` enum variant.
///
/// This is a variant of [`xcb_input_device_use_t`].
pub const XCB_INPUT_DEVICE_USE_IS_X_EXTENSION_POINTER: xcb_input_device_use_t = 4;

/// The `Input::InputClass` enum.
///
/// This enum has the following variants:
///
/// - [`Input::InputClass::Key`](XCB_INPUT_INPUT_CLASS_KEY)
/// - [`Input::InputClass::Button`](XCB_INPUT_INPUT_CLASS_BUTTON)
/// - [`Input::InputClass::Valuator`](XCB_INPUT_INPUT_CLASS_VALUATOR)
/// - [`Input::InputClass::Feedback`](XCB_INPUT_INPUT_CLASS_FEEDBACK)
/// - [`Input::InputClass::Proximity`](XCB_INPUT_INPUT_CLASS_PROXIMITY)
/// - [`Input::InputClass::Focus`](XCB_INPUT_INPUT_CLASS_FOCUS)
/// - [`Input::InputClass::Other`](XCB_INPUT_INPUT_CLASS_OTHER)
pub type xcb_input_input_class_t = u32;
/// The `Input::InputClass::Key` enum variant.
///
/// This is a variant of [`xcb_input_input_class_t`].
pub const XCB_INPUT_INPUT_CLASS_KEY: xcb_input_input_class_t = 0;
/// The `Input::InputClass::Button` enum variant.
///
/// This is a variant of [`xcb_input_input_class_t`].
pub const XCB_INPUT_INPUT_CLASS_BUTTON: xcb_input_input_class_t = 1;
/// The `Input::InputClass::Valuator` enum variant.
///
/// This is a variant of [`xcb_input_input_class_t`].
pub const XCB_INPUT_INPUT_CLASS_VALUATOR: xcb_input_input_class_t = 2;
/// The `Input::InputClass::Feedback` enum variant.
///
/// This is a variant of [`xcb_input_input_class_t`].
pub const XCB_INPUT_INPUT_CLASS_FEEDBACK: xcb_input_input_class_t = 3;
/// The `Input::InputClass::Proximity` enum variant.
///
/// This is a variant of [`xcb_input_input_class_t`].
pub const XCB_INPUT_INPUT_CLASS_PROXIMITY: xcb_input_input_class_t = 4;
/// The `Input::InputClass::Focus` enum variant.
///
/// This is a variant of [`xcb_input_input_class_t`].
pub const XCB_INPUT_INPUT_CLASS_FOCUS: xcb_input_input_class_t = 5;
/// The `Input::InputClass::Other` enum variant.
///
/// This is a variant of [`xcb_input_input_class_t`].
pub const XCB_INPUT_INPUT_CLASS_OTHER: xcb_input_input_class_t = 6;

/// The `Input::ValuatorMode` enum.
///
/// This enum has the following variants:
///
/// - [`Input::ValuatorMode::Relative`](XCB_INPUT_VALUATOR_MODE_RELATIVE)
/// - [`Input::ValuatorMode::Absolute`](XCB_INPUT_VALUATOR_MODE_ABSOLUTE)
pub type xcb_input_valuator_mode_t = u32;
/// The `Input::ValuatorMode::Relative` enum variant.
///
/// This is a variant of [`xcb_input_valuator_mode_t`].
pub const XCB_INPUT_VALUATOR_MODE_RELATIVE: xcb_input_valuator_mode_t = 0;
/// The `Input::ValuatorMode::Absolute` enum variant.
///
/// This is a variant of [`xcb_input_valuator_mode_t`].
pub const XCB_INPUT_VALUATOR_MODE_ABSOLUTE: xcb_input_valuator_mode_t = 1;

/// The `Input::DeviceInfo` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_info_t {
    pub device_type: xcb_atom_t,
    pub device_id: u8,
    pub num_class_info: u8,
    pub device_use: u8,
    pub pad0: u8,
}

impl Default for xcb_input_device_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::DeviceInfo` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_info_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_device_info_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_device_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::KeyInfo` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_key_info_t {
    pub class_id: u8,
    pub len: u8,
    pub min_keycode: xcb_input_key_code_t,
    pub max_keycode: xcb_input_key_code_t,
    pub num_keys: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_key_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::KeyInfo` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_key_info_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_key_info_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_key_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::ButtonInfo` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_button_info_t {
    pub class_id: u8,
    pub len: u8,
    pub num_buttons: u16,
}

impl Default for xcb_input_button_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::ButtonInfo` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_button_info_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_button_info_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_button_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::AxisInfo` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_axis_info_t {
    pub resolution: u32,
    pub minimum: i32,
    pub maximum: i32,
}

impl Default for xcb_input_axis_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::AxisInfo` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_axis_info_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_axis_info_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_axis_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::ValuatorInfo` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `axes`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_valuator_info_t {
    pub class_id: u8,
    pub len: u8,
    pub axes_len: u8,
    pub mode: u8,
    pub motion_size: u32,
}

impl Default for xcb_input_valuator_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::ValuatorInfo` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_valuator_info_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_valuator_info_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_valuator_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_input_info_info_t::key`].
///
/// In libxcb, this type is an anonymous struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_input_info_info_t__key {
    pub min_keycode: xcb_input_key_code_t,
    pub max_keycode: xcb_input_key_code_t,
    pub num_keys: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_input_info_info_t__key {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_input_info_info_t::button`].
///
/// In libxcb, this type is an anonymous struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_input_info_info_t__button {
    pub num_buttons: u16,
}

impl Default for xcb_input_input_info_info_t__button {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_input_info_info_t::valuator`].
///
/// In libxcb, this type is an anonymous struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `axes`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_input_info_info_t__valuator {
    pub axes_len: u8,
    pub mode: u8,
    pub motion_size: u32,
    pub axes: *mut xcb_input_axis_info_t,
}

impl Default for xcb_input_input_info_info_t__valuator {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::info` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_input_info_info_t {
    pub key: xcb_input_input_info_info_t__key,
    pub button: xcb_input_input_info_info_t__button,
    pub valuator: xcb_input_input_info_info_t__valuator,
}

impl Default for xcb_input_input_info_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::InputInfo` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `info`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_input_info_t {
    pub class_id: u8,
    pub len: u8,
}

impl Default for xcb_input_input_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::InputInfo` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_input_info_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_input_info_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_input_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::DeviceName` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `string`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_name_t {
    pub len: u8,
}

impl Default for xcb_input_device_name_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::DeviceName` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_name_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_device_name_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_device_name_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::ListInputDevices` request.
///
/// Pass this cookie to [`xcb_input_list_input_devices_reply`] to retrieve the reply.
///
/// [`xcb_input_list_input_devices_reply`]: XcbXinput::xcb_input_list_input_devices_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_list_input_devices_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_list_input_devices_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::ListInputDevices` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_list_input_devices_request_t`].
pub const XCB_INPUT_LIST_INPUT_DEVICES: u8 = 2i32 as u8;

/// The `Input::ListInputDevices` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_list_input_devices_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
}

impl Default for xcb_input_list_input_devices_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::ListInputDevices` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `devices`
/// - `infos`
/// - `names`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_list_input_devices_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub devices_len: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_list_input_devices_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::EventTypeBase` type.
pub type xcb_input_event_type_base_t = u8;

/// An iterator over `Input::EventTypeBase` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_event_type_base_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_event_type_base_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_event_type_base_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::InputClassInfo` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_input_class_info_t {
    pub class_id: u8,
    pub event_type_base: xcb_input_event_type_base_t,
}

impl Default for xcb_input_input_class_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::InputClassInfo` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_input_class_info_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_input_class_info_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_input_class_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::OpenDevice` request.
///
/// Pass this cookie to [`xcb_input_open_device_reply`] to retrieve the reply.
///
/// [`xcb_input_open_device_reply`]: XcbXinput::xcb_input_open_device_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_open_device_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_open_device_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::OpenDevice` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_open_device_request_t`].
pub const XCB_INPUT_OPEN_DEVICE: u8 = 3i32 as u8;

/// The `Input::OpenDevice` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_open_device_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_open_device_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::OpenDevice` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `class_info`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_open_device_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_classes: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_open_device_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::CloseDevice` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_close_device_request_t`].
pub const XCB_INPUT_CLOSE_DEVICE: u8 = 4i32 as u8;

/// The `Input::CloseDevice` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_close_device_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_close_device_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::SetDeviceMode` request.
///
/// Pass this cookie to [`xcb_input_set_device_mode_reply`] to retrieve the reply.
///
/// [`xcb_input_set_device_mode_reply`]: XcbXinput::xcb_input_set_device_mode_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_set_device_mode_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_set_device_mode_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::SetDeviceMode` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_set_device_mode_request_t`].
pub const XCB_INPUT_SET_DEVICE_MODE: u8 = 5i32 as u8;

/// The `Input::SetDeviceMode` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_set_device_mode_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub mode: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_set_device_mode_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::SetDeviceMode` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_set_device_mode_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_set_device_mode_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::SelectExtensionEvent` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_select_extension_event_request_t`].
pub const XCB_INPUT_SELECT_EXTENSION_EVENT: u8 = 6i32 as u8;

/// The `Input::SelectExtensionEvent` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `classes`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_select_extension_event_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub num_classes: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_select_extension_event_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::GetSelectedExtensionEvents` request.
///
/// Pass this cookie to [`xcb_input_get_selected_extension_events_reply`] to retrieve the reply.
///
/// [`xcb_input_get_selected_extension_events_reply`]: XcbXinput::xcb_input_get_selected_extension_events_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_selected_extension_events_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_get_selected_extension_events_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::GetSelectedExtensionEvents` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_get_selected_extension_events_request_t`].
pub const XCB_INPUT_GET_SELECTED_EXTENSION_EVENTS: u8 = 7i32 as u8;

/// The `Input::GetSelectedExtensionEvents` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_selected_extension_events_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_input_get_selected_extension_events_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::GetSelectedExtensionEvents` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `this_classes`
/// - `all_classes`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_selected_extension_events_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_this_classes: u16,
    pub num_all_classes: u16,
    pub pad0: [u8; 20],
}

impl Default for xcb_input_get_selected_extension_events_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::PropagateMode` enum.
///
/// This enum has the following variants:
///
/// - [`Input::PropagateMode::AddToList`](XCB_INPUT_PROPAGATE_MODE_ADD_TO_LIST)
/// - [`Input::PropagateMode::DeleteFromList`](XCB_INPUT_PROPAGATE_MODE_DELETE_FROM_LIST)
pub type xcb_input_propagate_mode_t = u32;
/// The `Input::PropagateMode::AddToList` enum variant.
///
/// This is a variant of [`xcb_input_propagate_mode_t`].
pub const XCB_INPUT_PROPAGATE_MODE_ADD_TO_LIST: xcb_input_propagate_mode_t = 0;
/// The `Input::PropagateMode::DeleteFromList` enum variant.
///
/// This is a variant of [`xcb_input_propagate_mode_t`].
pub const XCB_INPUT_PROPAGATE_MODE_DELETE_FROM_LIST: xcb_input_propagate_mode_t = 1;

/// The opcode for `Input::ChangeDeviceDontPropagateList` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_change_device_dont_propagate_list_request_t`].
pub const XCB_INPUT_CHANGE_DEVICE_DONT_PROPAGATE_LIST: u8 = 8i32 as u8;

/// The `Input::ChangeDeviceDontPropagateList` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `classes`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_device_dont_propagate_list_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub num_classes: u16,
    pub mode: u8,
    pub pad0: u8,
}

impl Default for xcb_input_change_device_dont_propagate_list_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::GetDeviceDontPropagateList` request.
///
/// Pass this cookie to [`xcb_input_get_device_dont_propagate_list_reply`] to retrieve the reply.
///
/// [`xcb_input_get_device_dont_propagate_list_reply`]: XcbXinput::xcb_input_get_device_dont_propagate_list_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_dont_propagate_list_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_get_device_dont_propagate_list_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::GetDeviceDontPropagateList` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_get_device_dont_propagate_list_request_t`].
pub const XCB_INPUT_GET_DEVICE_DONT_PROPAGATE_LIST: u8 = 9i32 as u8;

/// The `Input::GetDeviceDontPropagateList` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_dont_propagate_list_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_input_get_device_dont_propagate_list_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::GetDeviceDontPropagateList` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `classes`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_dont_propagate_list_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_classes: u16,
    pub pad0: [u8; 22],
}

impl Default for xcb_input_get_device_dont_propagate_list_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::DeviceTimeCoord` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `axisvalues`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_time_coord_t {
    pub time: xcb_timestamp_t,
}

impl Default for xcb_input_device_time_coord_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::DeviceTimeCoord` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_time_coord_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_device_time_coord_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
    pub num_axes: u8,
}

impl Default for xcb_input_device_time_coord_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::GetDeviceMotionEvents` request.
///
/// Pass this cookie to [`xcb_input_get_device_motion_events_reply`] to retrieve the reply.
///
/// [`xcb_input_get_device_motion_events_reply`]: XcbXinput::xcb_input_get_device_motion_events_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_motion_events_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_get_device_motion_events_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::GetDeviceMotionEvents` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_get_device_motion_events_request_t`].
pub const XCB_INPUT_GET_DEVICE_MOTION_EVENTS: u8 = 10i32 as u8;

/// The `Input::GetDeviceMotionEvents` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_motion_events_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub start: xcb_timestamp_t,
    pub stop: xcb_timestamp_t,
    pub device_id: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_get_device_motion_events_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::GetDeviceMotionEvents` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `events`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_motion_events_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_events: u32,
    pub num_axes: u8,
    pub device_mode: u8,
    pub pad0: [u8; 18],
}

impl Default for xcb_input_get_device_motion_events_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::ChangeKeyboardDevice` request.
///
/// Pass this cookie to [`xcb_input_change_keyboard_device_reply`] to retrieve the reply.
///
/// [`xcb_input_change_keyboard_device_reply`]: XcbXinput::xcb_input_change_keyboard_device_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_keyboard_device_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_change_keyboard_device_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::ChangeKeyboardDevice` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_change_keyboard_device_request_t`].
pub const XCB_INPUT_CHANGE_KEYBOARD_DEVICE: u8 = 11i32 as u8;

/// The `Input::ChangeKeyboardDevice` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_keyboard_device_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_change_keyboard_device_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::ChangeKeyboardDevice` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_keyboard_device_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_change_keyboard_device_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::ChangePointerDevice` request.
///
/// Pass this cookie to [`xcb_input_change_pointer_device_reply`] to retrieve the reply.
///
/// [`xcb_input_change_pointer_device_reply`]: XcbXinput::xcb_input_change_pointer_device_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_pointer_device_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_change_pointer_device_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::ChangePointerDevice` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_change_pointer_device_request_t`].
pub const XCB_INPUT_CHANGE_POINTER_DEVICE: u8 = 12i32 as u8;

/// The `Input::ChangePointerDevice` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_pointer_device_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub x_axis: u8,
    pub y_axis: u8,
    pub device_id: u8,
    pub pad0: u8,
}

impl Default for xcb_input_change_pointer_device_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::ChangePointerDevice` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_pointer_device_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_change_pointer_device_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::GrabDevice` request.
///
/// Pass this cookie to [`xcb_input_grab_device_reply`] to retrieve the reply.
///
/// [`xcb_input_grab_device_reply`]: XcbXinput::xcb_input_grab_device_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_grab_device_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_grab_device_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::GrabDevice` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_grab_device_request_t`].
pub const XCB_INPUT_GRAB_DEVICE: u8 = 13i32 as u8;

/// The `Input::GrabDevice` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `classes`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_grab_device_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub time: xcb_timestamp_t,
    pub num_classes: u16,
    pub this_device_mode: u8,
    pub other_device_mode: u8,
    pub owner_events: u8,
    pub device_id: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_grab_device_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::GrabDevice` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_grab_device_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_grab_device_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::UngrabDevice` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_ungrab_device_request_t`].
pub const XCB_INPUT_UNGRAB_DEVICE: u8 = 14i32 as u8;

/// The `Input::UngrabDevice` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_ungrab_device_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub time: xcb_timestamp_t,
    pub device_id: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_ungrab_device_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::ModifierDevice` enum.
///
/// This enum has the following variants:
///
/// - [`Input::ModifierDevice::UseXKeyboard`](XCB_INPUT_MODIFIER_DEVICE_USE_X_KEYBOARD)
pub type xcb_input_modifier_device_t = u32;
/// The `Input::ModifierDevice::UseXKeyboard` enum variant.
///
/// This is a variant of [`xcb_input_modifier_device_t`].
pub const XCB_INPUT_MODIFIER_DEVICE_USE_X_KEYBOARD: xcb_input_modifier_device_t = 255;

/// The opcode for `Input::GrabDeviceKey` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_grab_device_key_request_t`].
pub const XCB_INPUT_GRAB_DEVICE_KEY: u8 = 15i32 as u8;

/// The `Input::GrabDeviceKey` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `classes`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_grab_device_key_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub num_classes: u16,
    pub modifiers: u16,
    pub modifier_device: u8,
    pub grabbed_device: u8,
    pub key: u8,
    pub this_device_mode: u8,
    pub other_device_mode: u8,
    pub owner_events: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_grab_device_key_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::UngrabDeviceKey` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_ungrab_device_key_request_t`].
pub const XCB_INPUT_UNGRAB_DEVICE_KEY: u8 = 16i32 as u8;

/// The `Input::UngrabDeviceKey` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_ungrab_device_key_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub modifiers: u16,
    pub modifier_device: u8,
    pub key: u8,
    pub grabbed_device: u8,
}

impl Default for xcb_input_ungrab_device_key_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::GrabDeviceButton` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_grab_device_button_request_t`].
pub const XCB_INPUT_GRAB_DEVICE_BUTTON: u8 = 17i32 as u8;

/// The `Input::GrabDeviceButton` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `classes`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_grab_device_button_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub grabbed_device: u8,
    pub modifier_device: u8,
    pub num_classes: u16,
    pub modifiers: u16,
    pub this_device_mode: u8,
    pub other_device_mode: u8,
    pub button: u8,
    pub owner_events: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_grab_device_button_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::UngrabDeviceButton` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_ungrab_device_button_request_t`].
pub const XCB_INPUT_UNGRAB_DEVICE_BUTTON: u8 = 18i32 as u8;

/// The `Input::UngrabDeviceButton` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_ungrab_device_button_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub modifiers: u16,
    pub modifier_device: u8,
    pub button: u8,
    pub grabbed_device: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_ungrab_device_button_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::DeviceInputMode` enum.
///
/// This enum has the following variants:
///
/// - [`Input::DeviceInputMode::AsyncThisDevice`](XCB_INPUT_DEVICE_INPUT_MODE_ASYNC_THIS_DEVICE)
/// - [`Input::DeviceInputMode::SyncThisDevice`](XCB_INPUT_DEVICE_INPUT_MODE_SYNC_THIS_DEVICE)
/// - [`Input::DeviceInputMode::ReplayThisDevice`](XCB_INPUT_DEVICE_INPUT_MODE_REPLAY_THIS_DEVICE)
/// - [`Input::DeviceInputMode::AsyncOtherDevices`](XCB_INPUT_DEVICE_INPUT_MODE_ASYNC_OTHER_DEVICES)
/// - [`Input::DeviceInputMode::AsyncAll`](XCB_INPUT_DEVICE_INPUT_MODE_ASYNC_ALL)
/// - [`Input::DeviceInputMode::SyncAll`](XCB_INPUT_DEVICE_INPUT_MODE_SYNC_ALL)
pub type xcb_input_device_input_mode_t = u32;
/// The `Input::DeviceInputMode::AsyncThisDevice` enum variant.
///
/// This is a variant of [`xcb_input_device_input_mode_t`].
pub const XCB_INPUT_DEVICE_INPUT_MODE_ASYNC_THIS_DEVICE: xcb_input_device_input_mode_t = 0;
/// The `Input::DeviceInputMode::SyncThisDevice` enum variant.
///
/// This is a variant of [`xcb_input_device_input_mode_t`].
pub const XCB_INPUT_DEVICE_INPUT_MODE_SYNC_THIS_DEVICE: xcb_input_device_input_mode_t = 1;
/// The `Input::DeviceInputMode::ReplayThisDevice` enum variant.
///
/// This is a variant of [`xcb_input_device_input_mode_t`].
pub const XCB_INPUT_DEVICE_INPUT_MODE_REPLAY_THIS_DEVICE: xcb_input_device_input_mode_t = 2;
/// The `Input::DeviceInputMode::AsyncOtherDevices` enum variant.
///
/// This is a variant of [`xcb_input_device_input_mode_t`].
pub const XCB_INPUT_DEVICE_INPUT_MODE_ASYNC_OTHER_DEVICES: xcb_input_device_input_mode_t = 3;
/// The `Input::DeviceInputMode::AsyncAll` enum variant.
///
/// This is a variant of [`xcb_input_device_input_mode_t`].
pub const XCB_INPUT_DEVICE_INPUT_MODE_ASYNC_ALL: xcb_input_device_input_mode_t = 4;
/// The `Input::DeviceInputMode::SyncAll` enum variant.
///
/// This is a variant of [`xcb_input_device_input_mode_t`].
pub const XCB_INPUT_DEVICE_INPUT_MODE_SYNC_ALL: xcb_input_device_input_mode_t = 5;

/// The opcode for `Input::AllowDeviceEvents` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_allow_device_events_request_t`].
pub const XCB_INPUT_ALLOW_DEVICE_EVENTS: u8 = 19i32 as u8;

/// The `Input::AllowDeviceEvents` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_allow_device_events_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub time: xcb_timestamp_t,
    pub mode: u8,
    pub device_id: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_allow_device_events_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::GetDeviceFocus` request.
///
/// Pass this cookie to [`xcb_input_get_device_focus_reply`] to retrieve the reply.
///
/// [`xcb_input_get_device_focus_reply`]: XcbXinput::xcb_input_get_device_focus_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_focus_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_get_device_focus_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::GetDeviceFocus` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_get_device_focus_request_t`].
pub const XCB_INPUT_GET_DEVICE_FOCUS: u8 = 20i32 as u8;

/// The `Input::GetDeviceFocus` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_focus_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_get_device_focus_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::GetDeviceFocus` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_focus_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub focus: xcb_window_t,
    pub time: xcb_timestamp_t,
    pub revert_to: u8,
    pub pad0: [u8; 15],
}

impl Default for xcb_input_get_device_focus_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::SetDeviceFocus` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_set_device_focus_request_t`].
pub const XCB_INPUT_SET_DEVICE_FOCUS: u8 = 21i32 as u8;

/// The `Input::SetDeviceFocus` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_set_device_focus_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub focus: xcb_window_t,
    pub time: xcb_timestamp_t,
    pub revert_to: u8,
    pub device_id: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_set_device_focus_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::FeedbackClass` enum.
///
/// This enum has the following variants:
///
/// - [`Input::FeedbackClass::Keyboard`](XCB_INPUT_FEEDBACK_CLASS_KEYBOARD)
/// - [`Input::FeedbackClass::Pointer`](XCB_INPUT_FEEDBACK_CLASS_POINTER)
/// - [`Input::FeedbackClass::String`](XCB_INPUT_FEEDBACK_CLASS_STRING)
/// - [`Input::FeedbackClass::Integer`](XCB_INPUT_FEEDBACK_CLASS_INTEGER)
/// - [`Input::FeedbackClass::Led`](XCB_INPUT_FEEDBACK_CLASS_LED)
/// - [`Input::FeedbackClass::Bell`](XCB_INPUT_FEEDBACK_CLASS_BELL)
pub type xcb_input_feedback_class_t = u32;
/// The `Input::FeedbackClass::Keyboard` enum variant.
///
/// This is a variant of [`xcb_input_feedback_class_t`].
pub const XCB_INPUT_FEEDBACK_CLASS_KEYBOARD: xcb_input_feedback_class_t = 0;
/// The `Input::FeedbackClass::Pointer` enum variant.
///
/// This is a variant of [`xcb_input_feedback_class_t`].
pub const XCB_INPUT_FEEDBACK_CLASS_POINTER: xcb_input_feedback_class_t = 1;
/// The `Input::FeedbackClass::String` enum variant.
///
/// This is a variant of [`xcb_input_feedback_class_t`].
pub const XCB_INPUT_FEEDBACK_CLASS_STRING: xcb_input_feedback_class_t = 2;
/// The `Input::FeedbackClass::Integer` enum variant.
///
/// This is a variant of [`xcb_input_feedback_class_t`].
pub const XCB_INPUT_FEEDBACK_CLASS_INTEGER: xcb_input_feedback_class_t = 3;
/// The `Input::FeedbackClass::Led` enum variant.
///
/// This is a variant of [`xcb_input_feedback_class_t`].
pub const XCB_INPUT_FEEDBACK_CLASS_LED: xcb_input_feedback_class_t = 4;
/// The `Input::FeedbackClass::Bell` enum variant.
///
/// This is a variant of [`xcb_input_feedback_class_t`].
pub const XCB_INPUT_FEEDBACK_CLASS_BELL: xcb_input_feedback_class_t = 5;

/// The `Input::KbdFeedbackState` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_kbd_feedback_state_t {
    pub class_id: u8,
    pub feedback_id: u8,
    pub len: u16,
    pub pitch: u16,
    pub duration: u16,
    pub led_mask: u32,
    pub led_values: u32,
    pub global_auto_repeat: u8,
    pub click: u8,
    pub percent: u8,
    pub pad0: u8,
    pub auto_repeats: [u8; 32],
}

impl Default for xcb_input_kbd_feedback_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::KbdFeedbackState` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_kbd_feedback_state_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_kbd_feedback_state_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_kbd_feedback_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::PtrFeedbackState` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_ptr_feedback_state_t {
    pub class_id: u8,
    pub feedback_id: u8,
    pub len: u16,
    pub pad0: [u8; 2],
    pub accel_num: u16,
    pub accel_denom: u16,
    pub threshold: u16,
}

impl Default for xcb_input_ptr_feedback_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::PtrFeedbackState` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_ptr_feedback_state_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_ptr_feedback_state_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_ptr_feedback_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::IntegerFeedbackState` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_integer_feedback_state_t {
    pub class_id: u8,
    pub feedback_id: u8,
    pub len: u16,
    pub resolution: u32,
    pub min_value: i32,
    pub max_value: i32,
}

impl Default for xcb_input_integer_feedback_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::IntegerFeedbackState` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_integer_feedback_state_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_integer_feedback_state_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_integer_feedback_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::StringFeedbackState` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `keysyms`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_string_feedback_state_t {
    pub class_id: u8,
    pub feedback_id: u8,
    pub len: u16,
    pub max_symbols: u16,
    pub num_keysyms: u16,
}

impl Default for xcb_input_string_feedback_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::StringFeedbackState` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_string_feedback_state_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_string_feedback_state_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_string_feedback_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::BellFeedbackState` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_bell_feedback_state_t {
    pub class_id: u8,
    pub feedback_id: u8,
    pub len: u16,
    pub percent: u8,
    pub pad0: [u8; 3],
    pub pitch: u16,
    pub duration: u16,
}

impl Default for xcb_input_bell_feedback_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::BellFeedbackState` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_bell_feedback_state_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_bell_feedback_state_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_bell_feedback_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::LedFeedbackState` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_led_feedback_state_t {
    pub class_id: u8,
    pub feedback_id: u8,
    pub len: u16,
    pub led_mask: u32,
    pub led_values: u32,
}

impl Default for xcb_input_led_feedback_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::LedFeedbackState` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_led_feedback_state_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_led_feedback_state_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_led_feedback_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_feedback_state_data_t::keyboard`].
///
/// In libxcb, this type is an anonymous struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_state_data_t__keyboard {
    pub pitch: u16,
    pub duration: u16,
    pub led_mask: u32,
    pub led_values: u32,
    pub global_auto_repeat: u8,
    pub click: u8,
    pub percent: u8,
    pub pad0: u8,
    pub auto_repeats: [u8; 32],
}

impl Default for xcb_input_feedback_state_data_t__keyboard {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_feedback_state_data_t::pointer`].
///
/// In libxcb, this type is an anonymous struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_state_data_t__pointer {
    pub pad1: [u8; 2],
    pub accel_num: u16,
    pub accel_denom: u16,
    pub threshold: u16,
}

impl Default for xcb_input_feedback_state_data_t__pointer {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_feedback_state_data_t::string`].
///
/// In libxcb, this type is an anonymous struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `keysyms`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_state_data_t__string {
    pub max_symbols: u16,
    pub num_keysyms: u16,
    pub keysyms: *mut xcb_keysym_t,
}

impl Default for xcb_input_feedback_state_data_t__string {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_feedback_state_data_t::integer`].
///
/// In libxcb, this type is an anonymous struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_state_data_t__integer {
    pub resolution: u32,
    pub min_value: i32,
    pub max_value: i32,
}

impl Default for xcb_input_feedback_state_data_t__integer {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_feedback_state_data_t::led`].
///
/// In libxcb, this type is an anonymous struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_state_data_t__led {
    pub led_mask: u32,
    pub led_values: u32,
}

impl Default for xcb_input_feedback_state_data_t__led {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_feedback_state_data_t::bell`].
///
/// In libxcb, this type is an anonymous struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_state_data_t__bell {
    pub percent: u8,
    pub pad2: [u8; 3],
    pub pitch: u16,
    pub duration: u16,
}

impl Default for xcb_input_feedback_state_data_t__bell {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::data` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_state_data_t {
    pub keyboard: xcb_input_feedback_state_data_t__keyboard,
    pub pointer: xcb_input_feedback_state_data_t__pointer,
    pub string: xcb_input_feedback_state_data_t__string,
    pub integer: xcb_input_feedback_state_data_t__integer,
    pub led: xcb_input_feedback_state_data_t__led,
    pub bell: xcb_input_feedback_state_data_t__bell,
}

impl Default for xcb_input_feedback_state_data_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::FeedbackState` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `data`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_state_t {
    pub class_id: u8,
    pub feedback_id: u8,
    pub len: u16,
}

impl Default for xcb_input_feedback_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::FeedbackState` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_state_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_feedback_state_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_feedback_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::GetFeedbackControl` request.
///
/// Pass this cookie to [`xcb_input_get_feedback_control_reply`] to retrieve the reply.
///
/// [`xcb_input_get_feedback_control_reply`]: XcbXinput::xcb_input_get_feedback_control_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_feedback_control_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_get_feedback_control_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::GetFeedbackControl` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_get_feedback_control_request_t`].
pub const XCB_INPUT_GET_FEEDBACK_CONTROL: u8 = 22i32 as u8;

/// The `Input::GetFeedbackControl` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_feedback_control_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_get_feedback_control_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::GetFeedbackControl` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `feedbacks`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_feedback_control_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_feedbacks: u16,
    pub pad0: [u8; 22],
}

impl Default for xcb_input_get_feedback_control_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::KbdFeedbackCtl` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_kbd_feedback_ctl_t {
    pub class_id: u8,
    pub feedback_id: u8,
    pub len: u16,
    pub key: xcb_input_key_code_t,
    pub auto_repeat_mode: u8,
    pub key_click_percent: i8,
    pub bell_percent: i8,
    pub bell_pitch: i16,
    pub bell_duration: i16,
    pub led_mask: u32,
    pub led_values: u32,
}

impl Default for xcb_input_kbd_feedback_ctl_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::KbdFeedbackCtl` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_kbd_feedback_ctl_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_kbd_feedback_ctl_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_kbd_feedback_ctl_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::PtrFeedbackCtl` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_ptr_feedback_ctl_t {
    pub class_id: u8,
    pub feedback_id: u8,
    pub len: u16,
    pub pad0: [u8; 2],
    pub num: i16,
    pub denom: i16,
    pub threshold: i16,
}

impl Default for xcb_input_ptr_feedback_ctl_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::PtrFeedbackCtl` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_ptr_feedback_ctl_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_ptr_feedback_ctl_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_ptr_feedback_ctl_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::IntegerFeedbackCtl` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_integer_feedback_ctl_t {
    pub class_id: u8,
    pub feedback_id: u8,
    pub len: u16,
    pub int_to_display: i32,
}

impl Default for xcb_input_integer_feedback_ctl_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::IntegerFeedbackCtl` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_integer_feedback_ctl_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_integer_feedback_ctl_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_integer_feedback_ctl_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::StringFeedbackCtl` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `keysyms`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_string_feedback_ctl_t {
    pub class_id: u8,
    pub feedback_id: u8,
    pub len: u16,
    pub pad0: [u8; 2],
    pub num_keysyms: u16,
}

impl Default for xcb_input_string_feedback_ctl_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::StringFeedbackCtl` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_string_feedback_ctl_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_string_feedback_ctl_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_string_feedback_ctl_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::BellFeedbackCtl` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_bell_feedback_ctl_t {
    pub class_id: u8,
    pub feedback_id: u8,
    pub len: u16,
    pub percent: i8,
    pub pad0: [u8; 3],
    pub pitch: i16,
    pub duration: i16,
}

impl Default for xcb_input_bell_feedback_ctl_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::BellFeedbackCtl` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_bell_feedback_ctl_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_bell_feedback_ctl_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_bell_feedback_ctl_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::LedFeedbackCtl` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_led_feedback_ctl_t {
    pub class_id: u8,
    pub feedback_id: u8,
    pub len: u16,
    pub led_mask: u32,
    pub led_values: u32,
}

impl Default for xcb_input_led_feedback_ctl_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::LedFeedbackCtl` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_led_feedback_ctl_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_led_feedback_ctl_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_led_feedback_ctl_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_feedback_ctl_data_t::keyboard`].
///
/// In libxcb, this type is an anonymous struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_ctl_data_t__keyboard {
    pub key: xcb_input_key_code_t,
    pub auto_repeat_mode: u8,
    pub key_click_percent: i8,
    pub bell_percent: i8,
    pub bell_pitch: i16,
    pub bell_duration: i16,
    pub led_mask: u32,
    pub led_values: u32,
}

impl Default for xcb_input_feedback_ctl_data_t__keyboard {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_feedback_ctl_data_t::pointer`].
///
/// In libxcb, this type is an anonymous struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_ctl_data_t__pointer {
    pub pad0: [u8; 2],
    pub num: i16,
    pub denom: i16,
    pub threshold: i16,
}

impl Default for xcb_input_feedback_ctl_data_t__pointer {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_feedback_ctl_data_t::string`].
///
/// In libxcb, this type is an anonymous struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `keysyms`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_ctl_data_t__string {
    pub pad1: [u8; 2],
    pub num_keysyms: u16,
    pub keysyms: *mut xcb_keysym_t,
}

impl Default for xcb_input_feedback_ctl_data_t__string {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_feedback_ctl_data_t::integer`].
///
/// In libxcb, this type is an anonymous struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_ctl_data_t__integer {
    pub int_to_display: i32,
}

impl Default for xcb_input_feedback_ctl_data_t__integer {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_feedback_ctl_data_t::led`].
///
/// In libxcb, this type is an anonymous struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_ctl_data_t__led {
    pub led_mask: u32,
    pub led_values: u32,
}

impl Default for xcb_input_feedback_ctl_data_t__led {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_feedback_ctl_data_t::bell`].
///
/// In libxcb, this type is an anonymous struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_ctl_data_t__bell {
    pub percent: i8,
    pub pad2: [u8; 3],
    pub pitch: i16,
    pub duration: i16,
}

impl Default for xcb_input_feedback_ctl_data_t__bell {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::data` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_ctl_data_t {
    pub keyboard: xcb_input_feedback_ctl_data_t__keyboard,
    pub pointer: xcb_input_feedback_ctl_data_t__pointer,
    pub string: xcb_input_feedback_ctl_data_t__string,
    pub integer: xcb_input_feedback_ctl_data_t__integer,
    pub led: xcb_input_feedback_ctl_data_t__led,
    pub bell: xcb_input_feedback_ctl_data_t__bell,
}

impl Default for xcb_input_feedback_ctl_data_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::FeedbackCtl` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `data`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_ctl_t {
    pub class_id: u8,
    pub feedback_id: u8,
    pub len: u16,
}

impl Default for xcb_input_feedback_ctl_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::FeedbackCtl` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_feedback_ctl_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_feedback_ctl_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_feedback_ctl_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::ChangeFeedbackControlMask` enum.
///
/// This enum has the following variants:
///
/// - [`Input::ChangeFeedbackControlMask::KeyClickPercent`](XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_KEY_CLICK_PERCENT)
/// - [`Input::ChangeFeedbackControlMask::Percent`](XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_PERCENT)
/// - [`Input::ChangeFeedbackControlMask::Pitch`](XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_PITCH)
/// - [`Input::ChangeFeedbackControlMask::Duration`](XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_DURATION)
/// - [`Input::ChangeFeedbackControlMask::Led`](XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_LED)
/// - [`Input::ChangeFeedbackControlMask::LedMode`](XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_LED_MODE)
/// - [`Input::ChangeFeedbackControlMask::Key`](XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_KEY)
/// - [`Input::ChangeFeedbackControlMask::AutoRepeatMode`](XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_AUTO_REPEAT_MODE)
/// - [`Input::ChangeFeedbackControlMask::String`](XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_STRING)
/// - [`Input::ChangeFeedbackControlMask::Integer`](XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_INTEGER)
/// - [`Input::ChangeFeedbackControlMask::AccelNum`](XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_ACCEL_NUM)
/// - [`Input::ChangeFeedbackControlMask::AccelDenom`](XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_ACCEL_DENOM)
/// - [`Input::ChangeFeedbackControlMask::Threshold`](XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_THRESHOLD)
pub type xcb_input_change_feedback_control_mask_t = u32;
/// The `Input::ChangeFeedbackControlMask::KeyClickPercent` enum variant.
///
/// This is a variant of [`xcb_input_change_feedback_control_mask_t`].
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_KEY_CLICK_PERCENT:
    xcb_input_change_feedback_control_mask_t = 1;
/// The `Input::ChangeFeedbackControlMask::Percent` enum variant.
///
/// This is a variant of [`xcb_input_change_feedback_control_mask_t`].
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_PERCENT: xcb_input_change_feedback_control_mask_t =
    2;
/// The `Input::ChangeFeedbackControlMask::Pitch` enum variant.
///
/// This is a variant of [`xcb_input_change_feedback_control_mask_t`].
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_PITCH: xcb_input_change_feedback_control_mask_t =
    4;
/// The `Input::ChangeFeedbackControlMask::Duration` enum variant.
///
/// This is a variant of [`xcb_input_change_feedback_control_mask_t`].
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_DURATION:
    xcb_input_change_feedback_control_mask_t = 8;
/// The `Input::ChangeFeedbackControlMask::Led` enum variant.
///
/// This is a variant of [`xcb_input_change_feedback_control_mask_t`].
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_LED: xcb_input_change_feedback_control_mask_t = 16;
/// The `Input::ChangeFeedbackControlMask::LedMode` enum variant.
///
/// This is a variant of [`xcb_input_change_feedback_control_mask_t`].
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_LED_MODE:
    xcb_input_change_feedback_control_mask_t = 32;
/// The `Input::ChangeFeedbackControlMask::Key` enum variant.
///
/// This is a variant of [`xcb_input_change_feedback_control_mask_t`].
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_KEY: xcb_input_change_feedback_control_mask_t = 64;
/// The `Input::ChangeFeedbackControlMask::AutoRepeatMode` enum variant.
///
/// This is a variant of [`xcb_input_change_feedback_control_mask_t`].
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_AUTO_REPEAT_MODE:
    xcb_input_change_feedback_control_mask_t = 128;
/// The `Input::ChangeFeedbackControlMask::String` enum variant.
///
/// This is a variant of [`xcb_input_change_feedback_control_mask_t`].
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_STRING: xcb_input_change_feedback_control_mask_t =
    1;
/// The `Input::ChangeFeedbackControlMask::Integer` enum variant.
///
/// This is a variant of [`xcb_input_change_feedback_control_mask_t`].
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_INTEGER: xcb_input_change_feedback_control_mask_t =
    1;
/// The `Input::ChangeFeedbackControlMask::AccelNum` enum variant.
///
/// This is a variant of [`xcb_input_change_feedback_control_mask_t`].
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_ACCEL_NUM:
    xcb_input_change_feedback_control_mask_t = 1;
/// The `Input::ChangeFeedbackControlMask::AccelDenom` enum variant.
///
/// This is a variant of [`xcb_input_change_feedback_control_mask_t`].
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_ACCEL_DENOM:
    xcb_input_change_feedback_control_mask_t = 2;
/// The `Input::ChangeFeedbackControlMask::Threshold` enum variant.
///
/// This is a variant of [`xcb_input_change_feedback_control_mask_t`].
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL_MASK_THRESHOLD:
    xcb_input_change_feedback_control_mask_t = 4;

/// The opcode for `Input::ChangeFeedbackControl` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_change_feedback_control_request_t`].
pub const XCB_INPUT_CHANGE_FEEDBACK_CONTROL: u8 = 23i32 as u8;

/// The `Input::ChangeFeedbackControl` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `feedback`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_feedback_control_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub mask: u32,
    pub device_id: u8,
    pub feedback_id: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_change_feedback_control_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::GetDeviceKeyMapping` request.
///
/// Pass this cookie to [`xcb_input_get_device_key_mapping_reply`] to retrieve the reply.
///
/// [`xcb_input_get_device_key_mapping_reply`]: XcbXinput::xcb_input_get_device_key_mapping_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_key_mapping_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_get_device_key_mapping_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::GetDeviceKeyMapping` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_get_device_key_mapping_request_t`].
pub const XCB_INPUT_GET_DEVICE_KEY_MAPPING: u8 = 24i32 as u8;

/// The `Input::GetDeviceKeyMapping` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_key_mapping_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub first_keycode: xcb_input_key_code_t,
    pub count: u8,
    pub pad0: u8,
}

impl Default for xcb_input_get_device_key_mapping_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::GetDeviceKeyMapping` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `keysyms`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_key_mapping_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub keysyms_per_keycode: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_get_device_key_mapping_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::ChangeDeviceKeyMapping` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_change_device_key_mapping_request_t`].
pub const XCB_INPUT_CHANGE_DEVICE_KEY_MAPPING: u8 = 25i32 as u8;

/// The `Input::ChangeDeviceKeyMapping` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `keysyms`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_device_key_mapping_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub first_keycode: xcb_input_key_code_t,
    pub keysyms_per_keycode: u8,
    pub keycode_count: u8,
}

impl Default for xcb_input_change_device_key_mapping_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::GetDeviceModifierMapping` request.
///
/// Pass this cookie to [`xcb_input_get_device_modifier_mapping_reply`] to retrieve the reply.
///
/// [`xcb_input_get_device_modifier_mapping_reply`]: XcbXinput::xcb_input_get_device_modifier_mapping_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_modifier_mapping_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_get_device_modifier_mapping_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::GetDeviceModifierMapping` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_get_device_modifier_mapping_request_t`].
pub const XCB_INPUT_GET_DEVICE_MODIFIER_MAPPING: u8 = 26i32 as u8;

/// The `Input::GetDeviceModifierMapping` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_modifier_mapping_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_get_device_modifier_mapping_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::GetDeviceModifierMapping` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `keymaps`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_modifier_mapping_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub keycodes_per_modifier: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_get_device_modifier_mapping_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::SetDeviceModifierMapping` request.
///
/// Pass this cookie to [`xcb_input_set_device_modifier_mapping_reply`] to retrieve the reply.
///
/// [`xcb_input_set_device_modifier_mapping_reply`]: XcbXinput::xcb_input_set_device_modifier_mapping_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_set_device_modifier_mapping_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_set_device_modifier_mapping_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::SetDeviceModifierMapping` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_set_device_modifier_mapping_request_t`].
pub const XCB_INPUT_SET_DEVICE_MODIFIER_MAPPING: u8 = 27i32 as u8;

/// The `Input::SetDeviceModifierMapping` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `keymaps`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_set_device_modifier_mapping_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub keycodes_per_modifier: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_set_device_modifier_mapping_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::SetDeviceModifierMapping` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_set_device_modifier_mapping_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_set_device_modifier_mapping_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::GetDeviceButtonMapping` request.
///
/// Pass this cookie to [`xcb_input_get_device_button_mapping_reply`] to retrieve the reply.
///
/// [`xcb_input_get_device_button_mapping_reply`]: XcbXinput::xcb_input_get_device_button_mapping_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_button_mapping_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_get_device_button_mapping_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::GetDeviceButtonMapping` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_get_device_button_mapping_request_t`].
pub const XCB_INPUT_GET_DEVICE_BUTTON_MAPPING: u8 = 28i32 as u8;

/// The `Input::GetDeviceButtonMapping` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_button_mapping_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_get_device_button_mapping_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::GetDeviceButtonMapping` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `map`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_button_mapping_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub map_size: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_get_device_button_mapping_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::SetDeviceButtonMapping` request.
///
/// Pass this cookie to [`xcb_input_set_device_button_mapping_reply`] to retrieve the reply.
///
/// [`xcb_input_set_device_button_mapping_reply`]: XcbXinput::xcb_input_set_device_button_mapping_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_set_device_button_mapping_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_set_device_button_mapping_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::SetDeviceButtonMapping` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_set_device_button_mapping_request_t`].
pub const XCB_INPUT_SET_DEVICE_BUTTON_MAPPING: u8 = 29i32 as u8;

/// The `Input::SetDeviceButtonMapping` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `map`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_set_device_button_mapping_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub map_size: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_set_device_button_mapping_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::SetDeviceButtonMapping` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_set_device_button_mapping_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_set_device_button_mapping_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::KeyState` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_key_state_t {
    pub class_id: u8,
    pub len: u8,
    pub num_keys: u8,
    pub pad0: u8,
    pub keys: [u8; 32],
}

impl Default for xcb_input_key_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::KeyState` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_key_state_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_key_state_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_key_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::ButtonState` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_button_state_t {
    pub class_id: u8,
    pub len: u8,
    pub num_buttons: u8,
    pub pad0: u8,
    pub buttons: [u8; 32],
}

impl Default for xcb_input_button_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::ButtonState` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_button_state_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_button_state_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_button_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::ValuatorStateModeMask` enum.
///
/// This enum has the following variants:
///
/// - [`Input::ValuatorStateModeMask::DeviceModeAbsolute`](XCB_INPUT_VALUATOR_STATE_MODE_MASK_DEVICE_MODE_ABSOLUTE)
/// - [`Input::ValuatorStateModeMask::OutOfProximity`](XCB_INPUT_VALUATOR_STATE_MODE_MASK_OUT_OF_PROXIMITY)
pub type xcb_input_valuator_state_mode_mask_t = u32;
/// The `Input::ValuatorStateModeMask::DeviceModeAbsolute` enum variant.
///
/// This is a variant of [`xcb_input_valuator_state_mode_mask_t`].
pub const XCB_INPUT_VALUATOR_STATE_MODE_MASK_DEVICE_MODE_ABSOLUTE:
    xcb_input_valuator_state_mode_mask_t = 1;
/// The `Input::ValuatorStateModeMask::OutOfProximity` enum variant.
///
/// This is a variant of [`xcb_input_valuator_state_mode_mask_t`].
pub const XCB_INPUT_VALUATOR_STATE_MODE_MASK_OUT_OF_PROXIMITY:
    xcb_input_valuator_state_mode_mask_t = 2;

/// The `Input::ValuatorState` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `valuators`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_valuator_state_t {
    pub class_id: u8,
    pub len: u8,
    pub num_valuators: u8,
    pub mode: u8,
}

impl Default for xcb_input_valuator_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::ValuatorState` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_valuator_state_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_valuator_state_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_valuator_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_input_state_data_t::key`].
///
/// In libxcb, this type is an anonymous struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_input_state_data_t__key {
    pub num_keys: u8,
    pub pad0: u8,
    pub keys: [u8; 32],
}

impl Default for xcb_input_input_state_data_t__key {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_input_state_data_t::button`].
///
/// In libxcb, this type is an anonymous struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_input_state_data_t__button {
    pub num_buttons: u8,
    pub pad1: u8,
    pub buttons: [u8; 32],
}

impl Default for xcb_input_input_state_data_t__button {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_input_state_data_t::valuator`].
///
/// In libxcb, this type is an anonymous struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `valuators`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_input_state_data_t__valuator {
    pub num_valuators: u8,
    pub mode: u8,
    pub valuators: *mut i32,
}

impl Default for xcb_input_input_state_data_t__valuator {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::data` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_input_state_data_t {
    pub key: xcb_input_input_state_data_t__key,
    pub button: xcb_input_input_state_data_t__button,
    pub valuator: xcb_input_input_state_data_t__valuator,
}

impl Default for xcb_input_input_state_data_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::InputState` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `data`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_input_state_t {
    pub class_id: u8,
    pub len: u8,
}

impl Default for xcb_input_input_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::InputState` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_input_state_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_input_state_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_input_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::QueryDeviceState` request.
///
/// Pass this cookie to [`xcb_input_query_device_state_reply`] to retrieve the reply.
///
/// [`xcb_input_query_device_state_reply`]: XcbXinput::xcb_input_query_device_state_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_query_device_state_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_query_device_state_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::QueryDeviceState` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_query_device_state_request_t`].
pub const XCB_INPUT_QUERY_DEVICE_STATE: u8 = 30i32 as u8;

/// The `Input::QueryDeviceState` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_query_device_state_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_query_device_state_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::QueryDeviceState` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `classes`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_query_device_state_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_classes: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_query_device_state_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::DeviceBell` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_device_bell_request_t`].
pub const XCB_INPUT_DEVICE_BELL: u8 = 32i32 as u8;

/// The `Input::DeviceBell` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_bell_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub feedback_id: u8,
    pub feedback_class: u8,
    pub percent: i8,
}

impl Default for xcb_input_device_bell_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::SetDeviceValuators` request.
///
/// Pass this cookie to [`xcb_input_set_device_valuators_reply`] to retrieve the reply.
///
/// [`xcb_input_set_device_valuators_reply`]: XcbXinput::xcb_input_set_device_valuators_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_set_device_valuators_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_set_device_valuators_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::SetDeviceValuators` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_set_device_valuators_request_t`].
pub const XCB_INPUT_SET_DEVICE_VALUATORS: u8 = 33i32 as u8;

/// The `Input::SetDeviceValuators` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `valuators`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_set_device_valuators_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub first_valuator: u8,
    pub num_valuators: u8,
    pub pad0: u8,
}

impl Default for xcb_input_set_device_valuators_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::SetDeviceValuators` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_set_device_valuators_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_set_device_valuators_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::DeviceControl` enum.
///
/// This enum has the following variants:
///
/// - [`Input::DeviceControl::resolution`](XCB_INPUT_DEVICE_CONTROL_RESOLUTION)
/// - [`Input::DeviceControl::abs_calib`](XCB_INPUT_DEVICE_CONTROL_ABS_CALIB)
/// - [`Input::DeviceControl::core`](XCB_INPUT_DEVICE_CONTROL_CORE)
/// - [`Input::DeviceControl::enable`](XCB_INPUT_DEVICE_CONTROL_ENABLE)
/// - [`Input::DeviceControl::abs_area`](XCB_INPUT_DEVICE_CONTROL_ABS_AREA)
pub type xcb_input_device_control_t = u32;
/// The `Input::DeviceControl::resolution` enum variant.
///
/// This is a variant of [`xcb_input_device_control_t`].
pub const XCB_INPUT_DEVICE_CONTROL_RESOLUTION: xcb_input_device_control_t = 1;
/// The `Input::DeviceControl::abs_calib` enum variant.
///
/// This is a variant of [`xcb_input_device_control_t`].
pub const XCB_INPUT_DEVICE_CONTROL_ABS_CALIB: xcb_input_device_control_t = 2;
/// The `Input::DeviceControl::core` enum variant.
///
/// This is a variant of [`xcb_input_device_control_t`].
pub const XCB_INPUT_DEVICE_CONTROL_CORE: xcb_input_device_control_t = 3;
/// The `Input::DeviceControl::enable` enum variant.
///
/// This is a variant of [`xcb_input_device_control_t`].
pub const XCB_INPUT_DEVICE_CONTROL_ENABLE: xcb_input_device_control_t = 4;
/// The `Input::DeviceControl::abs_area` enum variant.
///
/// This is a variant of [`xcb_input_device_control_t`].
pub const XCB_INPUT_DEVICE_CONTROL_ABS_AREA: xcb_input_device_control_t = 5;

/// The `Input::DeviceResolutionState` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `resolution_values`
/// - `resolution_min`
/// - `resolution_max`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_resolution_state_t {
    pub control_id: u16,
    pub len: u16,
    pub num_valuators: u32,
}

impl Default for xcb_input_device_resolution_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::DeviceResolutionState` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_resolution_state_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_device_resolution_state_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_device_resolution_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::DeviceAbsCalibState` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_abs_calib_state_t {
    pub control_id: u16,
    pub len: u16,
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub flip_x: u32,
    pub flip_y: u32,
    pub rotation: u32,
    pub button_threshold: u32,
}

impl Default for xcb_input_device_abs_calib_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::DeviceAbsCalibState` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_abs_calib_state_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_device_abs_calib_state_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_device_abs_calib_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::DeviceAbsAreaState` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_abs_area_state_t {
    pub control_id: u16,
    pub len: u16,
    pub offset_x: u32,
    pub offset_y: u32,
    pub width: u32,
    pub height: u32,
    pub screen: u32,
    pub following: u32,
}

impl Default for xcb_input_device_abs_area_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::DeviceAbsAreaState` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_abs_area_state_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_device_abs_area_state_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_device_abs_area_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::DeviceCoreState` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_core_state_t {
    pub control_id: u16,
    pub len: u16,
    pub status: u8,
    pub iscore: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_device_core_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::DeviceCoreState` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_core_state_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_device_core_state_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_device_core_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::DeviceEnableState` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_enable_state_t {
    pub control_id: u16,
    pub len: u16,
    pub enable: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_device_enable_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::DeviceEnableState` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_enable_state_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_device_enable_state_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_device_enable_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_device_state_data_t::resolution`].
///
/// In libxcb, this type is an anonymous struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `resolution_values`
/// - `resolution_min`
/// - `resolution_max`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_state_data_t__resolution {
    pub num_valuators: u32,
    pub resolution_values: *mut u32,
    pub resolution_min: *mut u32,
    pub resolution_max: *mut u32,
}

impl Default for xcb_input_device_state_data_t__resolution {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_device_state_data_t::abs_calib`].
///
/// In libxcb, this type is an anonymous struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_state_data_t__abs_calib {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub flip_x: u32,
    pub flip_y: u32,
    pub rotation: u32,
    pub button_threshold: u32,
}

impl Default for xcb_input_device_state_data_t__abs_calib {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_device_state_data_t::core`].
///
/// In libxcb, this type is an anonymous struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_state_data_t__core {
    pub status: u8,
    pub iscore: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_device_state_data_t__core {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_device_state_data_t::enable`].
///
/// In libxcb, this type is an anonymous struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_state_data_t__enable {
    pub enable: u8,
    pub pad1: [u8; 3],
}

impl Default for xcb_input_device_state_data_t__enable {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_device_state_data_t::abs_area`].
///
/// In libxcb, this type is an anonymous struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_state_data_t__abs_area {
    pub offset_x: u32,
    pub offset_y: u32,
    pub width: u32,
    pub height: u32,
    pub screen: u32,
    pub following: u32,
}

impl Default for xcb_input_device_state_data_t__abs_area {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::data` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_state_data_t {
    pub resolution: xcb_input_device_state_data_t__resolution,
    pub abs_calib: xcb_input_device_state_data_t__abs_calib,
    pub core: xcb_input_device_state_data_t__core,
    pub enable: xcb_input_device_state_data_t__enable,
    pub abs_area: xcb_input_device_state_data_t__abs_area,
}

impl Default for xcb_input_device_state_data_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::DeviceState` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `data`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_state_t {
    pub control_id: u16,
    pub len: u16,
}

impl Default for xcb_input_device_state_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::DeviceState` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_state_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_device_state_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_device_state_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::GetDeviceControl` request.
///
/// Pass this cookie to [`xcb_input_get_device_control_reply`] to retrieve the reply.
///
/// [`xcb_input_get_device_control_reply`]: XcbXinput::xcb_input_get_device_control_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_control_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_get_device_control_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::GetDeviceControl` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_get_device_control_request_t`].
pub const XCB_INPUT_GET_DEVICE_CONTROL: u8 = 34i32 as u8;

/// The `Input::GetDeviceControl` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_control_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub control_id: u16,
    pub device_id: u8,
    pub pad0: u8,
}

impl Default for xcb_input_get_device_control_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::GetDeviceControl` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `control`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_control_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_get_device_control_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::DeviceResolutionCtl` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `resolution_values`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_resolution_ctl_t {
    pub control_id: u16,
    pub len: u16,
    pub first_valuator: u8,
    pub num_valuators: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_device_resolution_ctl_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::DeviceResolutionCtl` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_resolution_ctl_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_device_resolution_ctl_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_device_resolution_ctl_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::DeviceAbsCalibCtl` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_abs_calib_ctl_t {
    pub control_id: u16,
    pub len: u16,
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub flip_x: u32,
    pub flip_y: u32,
    pub rotation: u32,
    pub button_threshold: u32,
}

impl Default for xcb_input_device_abs_calib_ctl_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::DeviceAbsCalibCtl` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_abs_calib_ctl_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_device_abs_calib_ctl_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_device_abs_calib_ctl_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::DeviceAbsAreaCtrl` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_abs_area_ctrl_t {
    pub control_id: u16,
    pub len: u16,
    pub offset_x: u32,
    pub offset_y: u32,
    pub width: i32,
    pub height: i32,
    pub screen: i32,
    pub following: u32,
}

impl Default for xcb_input_device_abs_area_ctrl_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::DeviceAbsAreaCtrl` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_abs_area_ctrl_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_device_abs_area_ctrl_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_device_abs_area_ctrl_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::DeviceCoreCtrl` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_core_ctrl_t {
    pub control_id: u16,
    pub len: u16,
    pub status: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_device_core_ctrl_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::DeviceCoreCtrl` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_core_ctrl_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_device_core_ctrl_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_device_core_ctrl_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::DeviceEnableCtrl` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_enable_ctrl_t {
    pub control_id: u16,
    pub len: u16,
    pub enable: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_device_enable_ctrl_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::DeviceEnableCtrl` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_enable_ctrl_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_device_enable_ctrl_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_device_enable_ctrl_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_device_ctl_data_t::resolution`].
///
/// In libxcb, this type is an anonymous struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `resolution_values`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_ctl_data_t__resolution {
    pub first_valuator: u8,
    pub num_valuators: u8,
    pub pad0: [u8; 2],
    pub resolution_values: *mut u32,
}

impl Default for xcb_input_device_ctl_data_t__resolution {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_device_ctl_data_t::abs_calib`].
///
/// In libxcb, this type is an anonymous struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_ctl_data_t__abs_calib {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub flip_x: u32,
    pub flip_y: u32,
    pub rotation: u32,
    pub button_threshold: u32,
}

impl Default for xcb_input_device_ctl_data_t__abs_calib {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_device_ctl_data_t::core`].
///
/// In libxcb, this type is an anonymous struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_ctl_data_t__core {
    pub status: u8,
    pub pad1: [u8; 3],
}

impl Default for xcb_input_device_ctl_data_t__core {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_device_ctl_data_t::enable`].
///
/// In libxcb, this type is an anonymous struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_ctl_data_t__enable {
    pub enable: u8,
    pub pad2: [u8; 3],
}

impl Default for xcb_input_device_ctl_data_t__enable {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_device_ctl_data_t::abs_area`].
///
/// In libxcb, this type is an anonymous struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_ctl_data_t__abs_area {
    pub offset_x: u32,
    pub offset_y: u32,
    pub width: i32,
    pub height: i32,
    pub screen: i32,
    pub following: u32,
}

impl Default for xcb_input_device_ctl_data_t__abs_area {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::data` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_ctl_data_t {
    pub resolution: xcb_input_device_ctl_data_t__resolution,
    pub abs_calib: xcb_input_device_ctl_data_t__abs_calib,
    pub core: xcb_input_device_ctl_data_t__core,
    pub enable: xcb_input_device_ctl_data_t__enable,
    pub abs_area: xcb_input_device_ctl_data_t__abs_area,
}

impl Default for xcb_input_device_ctl_data_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::DeviceCtl` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `data`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_ctl_t {
    pub control_id: u16,
    pub len: u16,
}

impl Default for xcb_input_device_ctl_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::DeviceCtl` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_ctl_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_device_ctl_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_device_ctl_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::ChangeDeviceControl` request.
///
/// Pass this cookie to [`xcb_input_change_device_control_reply`] to retrieve the reply.
///
/// [`xcb_input_change_device_control_reply`]: XcbXinput::xcb_input_change_device_control_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_device_control_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_change_device_control_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::ChangeDeviceControl` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_change_device_control_request_t`].
pub const XCB_INPUT_CHANGE_DEVICE_CONTROL: u8 = 35i32 as u8;

/// The `Input::ChangeDeviceControl` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `control`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_device_control_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub control_id: u16,
    pub device_id: u8,
    pub pad0: u8,
}

impl Default for xcb_input_change_device_control_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::ChangeDeviceControl` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_device_control_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_change_device_control_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::ListDeviceProperties` request.
///
/// Pass this cookie to [`xcb_input_list_device_properties_reply`] to retrieve the reply.
///
/// [`xcb_input_list_device_properties_reply`]: XcbXinput::xcb_input_list_device_properties_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_list_device_properties_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_list_device_properties_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::ListDeviceProperties` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_list_device_properties_request_t`].
pub const XCB_INPUT_LIST_DEVICE_PROPERTIES: u8 = 36i32 as u8;

/// The `Input::ListDeviceProperties` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_list_device_properties_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_id: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_list_device_properties_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::ListDeviceProperties` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `atoms`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_list_device_properties_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_atoms: u16,
    pub pad0: [u8; 22],
}

impl Default for xcb_input_list_device_properties_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::PropertyFormat` enum.
///
/// This enum has the following variants:
///
/// - [`Input::PropertyFormat::8Bits`](XCB_INPUT_PROPERTY_FORMAT_8_BITS)
/// - [`Input::PropertyFormat::16Bits`](XCB_INPUT_PROPERTY_FORMAT_16_BITS)
/// - [`Input::PropertyFormat::32Bits`](XCB_INPUT_PROPERTY_FORMAT_32_BITS)
pub type xcb_input_property_format_t = u32;
/// The `Input::PropertyFormat::8Bits` enum variant.
///
/// This is a variant of [`xcb_input_property_format_t`].
pub const XCB_INPUT_PROPERTY_FORMAT_8_BITS: xcb_input_property_format_t = 8;
/// The `Input::PropertyFormat::16Bits` enum variant.
///
/// This is a variant of [`xcb_input_property_format_t`].
pub const XCB_INPUT_PROPERTY_FORMAT_16_BITS: xcb_input_property_format_t = 16;
/// The `Input::PropertyFormat::32Bits` enum variant.
///
/// This is a variant of [`xcb_input_property_format_t`].
pub const XCB_INPUT_PROPERTY_FORMAT_32_BITS: xcb_input_property_format_t = 32;

/// The `Input::items` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_device_property_items_t {
    pub data8: *mut u8,
    pub data16: *mut u16,
    pub data32: *mut u32,
}

impl Default for xcb_input_change_device_property_items_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::ChangeDeviceProperty` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_change_device_property_request_t`].
pub const XCB_INPUT_CHANGE_DEVICE_PROPERTY: u8 = 37i32 as u8;

/// The `Input::ChangeDeviceProperty` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `items`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_device_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub property: xcb_atom_t,
    pub type_: xcb_atom_t,
    pub device_id: u8,
    pub format: u8,
    pub mode: u8,
    pub pad0: u8,
    pub num_items: u32,
}

impl Default for xcb_input_change_device_property_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::DeleteDeviceProperty` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_delete_device_property_request_t`].
pub const XCB_INPUT_DELETE_DEVICE_PROPERTY: u8 = 38i32 as u8;

/// The `Input::DeleteDeviceProperty` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_delete_device_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub property: xcb_atom_t,
    pub device_id: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_delete_device_property_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::GetDeviceProperty` request.
///
/// Pass this cookie to [`xcb_input_get_device_property_reply`] to retrieve the reply.
///
/// [`xcb_input_get_device_property_reply`]: XcbXinput::xcb_input_get_device_property_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_property_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_get_device_property_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::GetDeviceProperty` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_get_device_property_request_t`].
pub const XCB_INPUT_GET_DEVICE_PROPERTY: u8 = 39i32 as u8;

/// The `Input::GetDeviceProperty` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub property: xcb_atom_t,
    pub type_: xcb_atom_t,
    pub offset: u32,
    pub len: u32,
    pub device_id: u8,
    pub delete: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_get_device_property_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::items` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_property_items_t {
    pub data8: *mut u8,
    pub data16: *mut u16,
    pub data32: *mut u32,
}

impl Default for xcb_input_get_device_property_items_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::GetDeviceProperty` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `items`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_get_device_property_reply_t {
    pub response_type: u8,
    pub xi_reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub type_: xcb_atom_t,
    pub bytes_after: u32,
    pub num_items: u32,
    pub format: u8,
    pub device_id: u8,
    pub pad0: [u8; 10],
}

impl Default for xcb_input_get_device_property_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::Device` enum.
///
/// This enum has the following variants:
///
/// - [`Input::Device::All`](XCB_INPUT_DEVICE_ALL)
/// - [`Input::Device::AllMaster`](XCB_INPUT_DEVICE_ALL_MASTER)
pub type xcb_input_device_t = u32;
/// The `Input::Device::All` enum variant.
///
/// This is a variant of [`xcb_input_device_t`].
pub const XCB_INPUT_DEVICE_ALL: xcb_input_device_t = 0;
/// The `Input::Device::AllMaster` enum variant.
///
/// This is a variant of [`xcb_input_device_t`].
pub const XCB_INPUT_DEVICE_ALL_MASTER: xcb_input_device_t = 1;

/// The `Input::GroupInfo` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_group_info_t {
    pub base: u8,
    pub latched: u8,
    pub locked: u8,
    pub effective: u8,
}

impl Default for xcb_input_group_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::GroupInfo` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_group_info_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_group_info_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_group_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::ModifierInfo` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_modifier_info_t {
    pub base: u32,
    pub latched: u32,
    pub locked: u32,
    pub effective: u32,
}

impl Default for xcb_input_modifier_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::ModifierInfo` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_modifier_info_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_modifier_info_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_modifier_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::XIQueryPointer` request.
///
/// Pass this cookie to [`xcb_input_xi_query_pointer_reply`] to retrieve the reply.
///
/// [`xcb_input_xi_query_pointer_reply`]: XcbXinput::xcb_input_xi_query_pointer_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_query_pointer_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_xi_query_pointer_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::XIQueryPointer` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_xi_query_pointer_request_t`].
pub const XCB_INPUT_XI_QUERY_POINTER: u8 = 40i32 as u8;

/// The `Input::XIQueryPointer` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_query_pointer_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub deviceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_xi_query_pointer_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::XIQueryPointer` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `buttons`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_query_pointer_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: xcb_input_fp1616_t,
    pub root_y: xcb_input_fp1616_t,
    pub win_x: xcb_input_fp1616_t,
    pub win_y: xcb_input_fp1616_t,
    pub same_screen: u8,
    pub pad1: u8,
    pub buttons_len: u16,
    pub mods: xcb_input_modifier_info_t,
    pub group: xcb_input_group_info_t,
}

impl Default for xcb_input_xi_query_pointer_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::XIWarpPointer` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_xi_warp_pointer_request_t`].
pub const XCB_INPUT_XI_WARP_POINTER: u8 = 41i32 as u8;

/// The `Input::XIWarpPointer` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_warp_pointer_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub src_win: xcb_window_t,
    pub dst_win: xcb_window_t,
    pub src_x: xcb_input_fp1616_t,
    pub src_y: xcb_input_fp1616_t,
    pub src_width: u16,
    pub src_height: u16,
    pub dst_x: xcb_input_fp1616_t,
    pub dst_y: xcb_input_fp1616_t,
    pub deviceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_xi_warp_pointer_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::XIChangeCursor` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_xi_change_cursor_request_t`].
pub const XCB_INPUT_XI_CHANGE_CURSOR: u8 = 42i32 as u8;

/// The `Input::XIChangeCursor` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_change_cursor_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub cursor: xcb_cursor_t,
    pub deviceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_xi_change_cursor_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::HierarchyChangeType` enum.
///
/// This enum has the following variants:
///
/// - [`Input::HierarchyChangeType::AddMaster`](XCB_INPUT_HIERARCHY_CHANGE_TYPE_ADD_MASTER)
/// - [`Input::HierarchyChangeType::RemoveMaster`](XCB_INPUT_HIERARCHY_CHANGE_TYPE_REMOVE_MASTER)
/// - [`Input::HierarchyChangeType::AttachSlave`](XCB_INPUT_HIERARCHY_CHANGE_TYPE_ATTACH_SLAVE)
/// - [`Input::HierarchyChangeType::DetachSlave`](XCB_INPUT_HIERARCHY_CHANGE_TYPE_DETACH_SLAVE)
pub type xcb_input_hierarchy_change_type_t = u32;
/// The `Input::HierarchyChangeType::AddMaster` enum variant.
///
/// This is a variant of [`xcb_input_hierarchy_change_type_t`].
pub const XCB_INPUT_HIERARCHY_CHANGE_TYPE_ADD_MASTER: xcb_input_hierarchy_change_type_t = 1;
/// The `Input::HierarchyChangeType::RemoveMaster` enum variant.
///
/// This is a variant of [`xcb_input_hierarchy_change_type_t`].
pub const XCB_INPUT_HIERARCHY_CHANGE_TYPE_REMOVE_MASTER: xcb_input_hierarchy_change_type_t = 2;
/// The `Input::HierarchyChangeType::AttachSlave` enum variant.
///
/// This is a variant of [`xcb_input_hierarchy_change_type_t`].
pub const XCB_INPUT_HIERARCHY_CHANGE_TYPE_ATTACH_SLAVE: xcb_input_hierarchy_change_type_t = 3;
/// The `Input::HierarchyChangeType::DetachSlave` enum variant.
///
/// This is a variant of [`xcb_input_hierarchy_change_type_t`].
pub const XCB_INPUT_HIERARCHY_CHANGE_TYPE_DETACH_SLAVE: xcb_input_hierarchy_change_type_t = 4;

/// The `Input::ChangeMode` enum.
///
/// This enum has the following variants:
///
/// - [`Input::ChangeMode::Attach`](XCB_INPUT_CHANGE_MODE_ATTACH)
/// - [`Input::ChangeMode::Float`](XCB_INPUT_CHANGE_MODE_FLOAT)
pub type xcb_input_change_mode_t = u32;
/// The `Input::ChangeMode::Attach` enum variant.
///
/// This is a variant of [`xcb_input_change_mode_t`].
pub const XCB_INPUT_CHANGE_MODE_ATTACH: xcb_input_change_mode_t = 1;
/// The `Input::ChangeMode::Float` enum variant.
///
/// This is a variant of [`xcb_input_change_mode_t`].
pub const XCB_INPUT_CHANGE_MODE_FLOAT: xcb_input_change_mode_t = 2;

/// The `Input::AddMaster` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `name`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_add_master_t {
    pub type_: u16,
    pub len: u16,
    pub name_len: u16,
    pub send_core: u8,
    pub enable: u8,
}

impl Default for xcb_input_add_master_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::AddMaster` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_add_master_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_add_master_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_add_master_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::RemoveMaster` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_remove_master_t {
    pub type_: u16,
    pub len: u16,
    pub deviceid: xcb_input_device_id_t,
    pub return_mode: u8,
    pub pad0: u8,
    pub return_pointer: xcb_input_device_id_t,
    pub return_keyboard: xcb_input_device_id_t,
}

impl Default for xcb_input_remove_master_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::RemoveMaster` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_remove_master_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_remove_master_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_remove_master_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::AttachSlave` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_attach_slave_t {
    pub type_: u16,
    pub len: u16,
    pub deviceid: xcb_input_device_id_t,
    pub master: xcb_input_device_id_t,
}

impl Default for xcb_input_attach_slave_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::AttachSlave` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_attach_slave_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_attach_slave_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_attach_slave_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::DetachSlave` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_detach_slave_t {
    pub type_: u16,
    pub len: u16,
    pub deviceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_detach_slave_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::DetachSlave` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_detach_slave_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_detach_slave_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_detach_slave_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_hierarchy_change_data_t::add_master`].
///
/// In libxcb, this type is an anonymous struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `name`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_hierarchy_change_data_t__add_master {
    pub name_len: u16,
    pub send_core: u8,
    pub enable: u8,
    pub name: *mut c_char,
}

impl Default for xcb_input_hierarchy_change_data_t__add_master {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_hierarchy_change_data_t::remove_master`].
///
/// In libxcb, this type is an anonymous struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_hierarchy_change_data_t__remove_master {
    pub deviceid: xcb_input_device_id_t,
    pub return_mode: u8,
    pub pad1: u8,
    pub return_pointer: xcb_input_device_id_t,
    pub return_keyboard: xcb_input_device_id_t,
}

impl Default for xcb_input_hierarchy_change_data_t__remove_master {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_hierarchy_change_data_t::attach_slave`].
///
/// In libxcb, this type is an anonymous struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_hierarchy_change_data_t__attach_slave {
    pub deviceid: xcb_input_device_id_t,
    pub master: xcb_input_device_id_t,
}

impl Default for xcb_input_hierarchy_change_data_t__attach_slave {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_hierarchy_change_data_t::detach_slave`].
///
/// In libxcb, this type is an anonymous struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_hierarchy_change_data_t__detach_slave {
    pub deviceid: xcb_input_device_id_t,
    pub pad2: [u8; 2],
}

impl Default for xcb_input_hierarchy_change_data_t__detach_slave {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::data` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_hierarchy_change_data_t {
    pub add_master: xcb_input_hierarchy_change_data_t__add_master,
    pub remove_master: xcb_input_hierarchy_change_data_t__remove_master,
    pub attach_slave: xcb_input_hierarchy_change_data_t__attach_slave,
    pub detach_slave: xcb_input_hierarchy_change_data_t__detach_slave,
}

impl Default for xcb_input_hierarchy_change_data_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::HierarchyChange` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `data`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_hierarchy_change_t {
    pub type_: u16,
    pub len: u16,
}

impl Default for xcb_input_hierarchy_change_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::HierarchyChange` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_hierarchy_change_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_hierarchy_change_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_hierarchy_change_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::XIChangeHierarchy` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_xi_change_hierarchy_request_t`].
pub const XCB_INPUT_XI_CHANGE_HIERARCHY: u8 = 43i32 as u8;

/// The `Input::XIChangeHierarchy` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `changes`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_change_hierarchy_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub num_changes: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_xi_change_hierarchy_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::XISetClientPointer` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_xi_set_client_pointer_request_t`].
pub const XCB_INPUT_XI_SET_CLIENT_POINTER: u8 = 44i32 as u8;

/// The `Input::XISetClientPointer` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_set_client_pointer_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub deviceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_xi_set_client_pointer_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::XIGetClientPointer` request.
///
/// Pass this cookie to [`xcb_input_xi_get_client_pointer_reply`] to retrieve the reply.
///
/// [`xcb_input_xi_get_client_pointer_reply`]: XcbXinput::xcb_input_xi_get_client_pointer_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_get_client_pointer_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_xi_get_client_pointer_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::XIGetClientPointer` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_xi_get_client_pointer_request_t`].
pub const XCB_INPUT_XI_GET_CLIENT_POINTER: u8 = 45i32 as u8;

/// The `Input::XIGetClientPointer` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_get_client_pointer_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_input_xi_get_client_pointer_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::XIGetClientPointer` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_get_client_pointer_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub set: u8,
    pub pad1: u8,
    pub deviceid: xcb_input_device_id_t,
    pub pad2: [u8; 20],
}

impl Default for xcb_input_xi_get_client_pointer_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::XIEventMask` enum.
///
/// This enum has the following variants:
///
/// - [`Input::XIEventMask::DeviceChanged`](XCB_INPUT_XI_EVENT_MASK_DEVICE_CHANGED)
/// - [`Input::XIEventMask::KeyPress`](XCB_INPUT_XI_EVENT_MASK_KEY_PRESS)
/// - [`Input::XIEventMask::KeyRelease`](XCB_INPUT_XI_EVENT_MASK_KEY_RELEASE)
/// - [`Input::XIEventMask::ButtonPress`](XCB_INPUT_XI_EVENT_MASK_BUTTON_PRESS)
/// - [`Input::XIEventMask::ButtonRelease`](XCB_INPUT_XI_EVENT_MASK_BUTTON_RELEASE)
/// - [`Input::XIEventMask::Motion`](XCB_INPUT_XI_EVENT_MASK_MOTION)
/// - [`Input::XIEventMask::Enter`](XCB_INPUT_XI_EVENT_MASK_ENTER)
/// - [`Input::XIEventMask::Leave`](XCB_INPUT_XI_EVENT_MASK_LEAVE)
/// - [`Input::XIEventMask::FocusIn`](XCB_INPUT_XI_EVENT_MASK_FOCUS_IN)
/// - [`Input::XIEventMask::FocusOut`](XCB_INPUT_XI_EVENT_MASK_FOCUS_OUT)
/// - [`Input::XIEventMask::Hierarchy`](XCB_INPUT_XI_EVENT_MASK_HIERARCHY)
/// - [`Input::XIEventMask::Property`](XCB_INPUT_XI_EVENT_MASK_PROPERTY)
/// - [`Input::XIEventMask::RawKeyPress`](XCB_INPUT_XI_EVENT_MASK_RAW_KEY_PRESS)
/// - [`Input::XIEventMask::RawKeyRelease`](XCB_INPUT_XI_EVENT_MASK_RAW_KEY_RELEASE)
/// - [`Input::XIEventMask::RawButtonPress`](XCB_INPUT_XI_EVENT_MASK_RAW_BUTTON_PRESS)
/// - [`Input::XIEventMask::RawButtonRelease`](XCB_INPUT_XI_EVENT_MASK_RAW_BUTTON_RELEASE)
/// - [`Input::XIEventMask::RawMotion`](XCB_INPUT_XI_EVENT_MASK_RAW_MOTION)
/// - [`Input::XIEventMask::TouchBegin`](XCB_INPUT_XI_EVENT_MASK_TOUCH_BEGIN)
/// - [`Input::XIEventMask::TouchUpdate`](XCB_INPUT_XI_EVENT_MASK_TOUCH_UPDATE)
/// - [`Input::XIEventMask::TouchEnd`](XCB_INPUT_XI_EVENT_MASK_TOUCH_END)
/// - [`Input::XIEventMask::TouchOwnership`](XCB_INPUT_XI_EVENT_MASK_TOUCH_OWNERSHIP)
/// - [`Input::XIEventMask::RawTouchBegin`](XCB_INPUT_XI_EVENT_MASK_RAW_TOUCH_BEGIN)
/// - [`Input::XIEventMask::RawTouchUpdate`](XCB_INPUT_XI_EVENT_MASK_RAW_TOUCH_UPDATE)
/// - [`Input::XIEventMask::RawTouchEnd`](XCB_INPUT_XI_EVENT_MASK_RAW_TOUCH_END)
/// - [`Input::XIEventMask::BarrierHit`](XCB_INPUT_XI_EVENT_MASK_BARRIER_HIT)
/// - [`Input::XIEventMask::BarrierLeave`](XCB_INPUT_XI_EVENT_MASK_BARRIER_LEAVE)
pub type xcb_input_xi_event_mask_t = u32;
/// The `Input::XIEventMask::DeviceChanged` enum variant.
///
/// This is a variant of [`xcb_input_xi_event_mask_t`].
pub const XCB_INPUT_XI_EVENT_MASK_DEVICE_CHANGED: xcb_input_xi_event_mask_t = 2;
/// The `Input::XIEventMask::KeyPress` enum variant.
///
/// This is a variant of [`xcb_input_xi_event_mask_t`].
pub const XCB_INPUT_XI_EVENT_MASK_KEY_PRESS: xcb_input_xi_event_mask_t = 4;
/// The `Input::XIEventMask::KeyRelease` enum variant.
///
/// This is a variant of [`xcb_input_xi_event_mask_t`].
pub const XCB_INPUT_XI_EVENT_MASK_KEY_RELEASE: xcb_input_xi_event_mask_t = 8;
/// The `Input::XIEventMask::ButtonPress` enum variant.
///
/// This is a variant of [`xcb_input_xi_event_mask_t`].
pub const XCB_INPUT_XI_EVENT_MASK_BUTTON_PRESS: xcb_input_xi_event_mask_t = 16;
/// The `Input::XIEventMask::ButtonRelease` enum variant.
///
/// This is a variant of [`xcb_input_xi_event_mask_t`].
pub const XCB_INPUT_XI_EVENT_MASK_BUTTON_RELEASE: xcb_input_xi_event_mask_t = 32;
/// The `Input::XIEventMask::Motion` enum variant.
///
/// This is a variant of [`xcb_input_xi_event_mask_t`].
pub const XCB_INPUT_XI_EVENT_MASK_MOTION: xcb_input_xi_event_mask_t = 64;
/// The `Input::XIEventMask::Enter` enum variant.
///
/// This is a variant of [`xcb_input_xi_event_mask_t`].
pub const XCB_INPUT_XI_EVENT_MASK_ENTER: xcb_input_xi_event_mask_t = 128;
/// The `Input::XIEventMask::Leave` enum variant.
///
/// This is a variant of [`xcb_input_xi_event_mask_t`].
pub const XCB_INPUT_XI_EVENT_MASK_LEAVE: xcb_input_xi_event_mask_t = 256;
/// The `Input::XIEventMask::FocusIn` enum variant.
///
/// This is a variant of [`xcb_input_xi_event_mask_t`].
pub const XCB_INPUT_XI_EVENT_MASK_FOCUS_IN: xcb_input_xi_event_mask_t = 512;
/// The `Input::XIEventMask::FocusOut` enum variant.
///
/// This is a variant of [`xcb_input_xi_event_mask_t`].
pub const XCB_INPUT_XI_EVENT_MASK_FOCUS_OUT: xcb_input_xi_event_mask_t = 1024;
/// The `Input::XIEventMask::Hierarchy` enum variant.
///
/// This is a variant of [`xcb_input_xi_event_mask_t`].
pub const XCB_INPUT_XI_EVENT_MASK_HIERARCHY: xcb_input_xi_event_mask_t = 2048;
/// The `Input::XIEventMask::Property` enum variant.
///
/// This is a variant of [`xcb_input_xi_event_mask_t`].
pub const XCB_INPUT_XI_EVENT_MASK_PROPERTY: xcb_input_xi_event_mask_t = 4096;
/// The `Input::XIEventMask::RawKeyPress` enum variant.
///
/// This is a variant of [`xcb_input_xi_event_mask_t`].
pub const XCB_INPUT_XI_EVENT_MASK_RAW_KEY_PRESS: xcb_input_xi_event_mask_t = 8192;
/// The `Input::XIEventMask::RawKeyRelease` enum variant.
///
/// This is a variant of [`xcb_input_xi_event_mask_t`].
pub const XCB_INPUT_XI_EVENT_MASK_RAW_KEY_RELEASE: xcb_input_xi_event_mask_t = 16384;
/// The `Input::XIEventMask::RawButtonPress` enum variant.
///
/// This is a variant of [`xcb_input_xi_event_mask_t`].
pub const XCB_INPUT_XI_EVENT_MASK_RAW_BUTTON_PRESS: xcb_input_xi_event_mask_t = 32768;
/// The `Input::XIEventMask::RawButtonRelease` enum variant.
///
/// This is a variant of [`xcb_input_xi_event_mask_t`].
pub const XCB_INPUT_XI_EVENT_MASK_RAW_BUTTON_RELEASE: xcb_input_xi_event_mask_t = 65536;
/// The `Input::XIEventMask::RawMotion` enum variant.
///
/// This is a variant of [`xcb_input_xi_event_mask_t`].
pub const XCB_INPUT_XI_EVENT_MASK_RAW_MOTION: xcb_input_xi_event_mask_t = 131072;
/// The `Input::XIEventMask::TouchBegin` enum variant.
///
/// This is a variant of [`xcb_input_xi_event_mask_t`].
pub const XCB_INPUT_XI_EVENT_MASK_TOUCH_BEGIN: xcb_input_xi_event_mask_t = 262144;
/// The `Input::XIEventMask::TouchUpdate` enum variant.
///
/// This is a variant of [`xcb_input_xi_event_mask_t`].
pub const XCB_INPUT_XI_EVENT_MASK_TOUCH_UPDATE: xcb_input_xi_event_mask_t = 524288;
/// The `Input::XIEventMask::TouchEnd` enum variant.
///
/// This is a variant of [`xcb_input_xi_event_mask_t`].
pub const XCB_INPUT_XI_EVENT_MASK_TOUCH_END: xcb_input_xi_event_mask_t = 1048576;
/// The `Input::XIEventMask::TouchOwnership` enum variant.
///
/// This is a variant of [`xcb_input_xi_event_mask_t`].
pub const XCB_INPUT_XI_EVENT_MASK_TOUCH_OWNERSHIP: xcb_input_xi_event_mask_t = 2097152;
/// The `Input::XIEventMask::RawTouchBegin` enum variant.
///
/// This is a variant of [`xcb_input_xi_event_mask_t`].
pub const XCB_INPUT_XI_EVENT_MASK_RAW_TOUCH_BEGIN: xcb_input_xi_event_mask_t = 4194304;
/// The `Input::XIEventMask::RawTouchUpdate` enum variant.
///
/// This is a variant of [`xcb_input_xi_event_mask_t`].
pub const XCB_INPUT_XI_EVENT_MASK_RAW_TOUCH_UPDATE: xcb_input_xi_event_mask_t = 8388608;
/// The `Input::XIEventMask::RawTouchEnd` enum variant.
///
/// This is a variant of [`xcb_input_xi_event_mask_t`].
pub const XCB_INPUT_XI_EVENT_MASK_RAW_TOUCH_END: xcb_input_xi_event_mask_t = 16777216;
/// The `Input::XIEventMask::BarrierHit` enum variant.
///
/// This is a variant of [`xcb_input_xi_event_mask_t`].
pub const XCB_INPUT_XI_EVENT_MASK_BARRIER_HIT: xcb_input_xi_event_mask_t = 33554432;
/// The `Input::XIEventMask::BarrierLeave` enum variant.
///
/// This is a variant of [`xcb_input_xi_event_mask_t`].
pub const XCB_INPUT_XI_EVENT_MASK_BARRIER_LEAVE: xcb_input_xi_event_mask_t = 67108864;

/// The `Input::EventMask` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `mask`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_event_mask_t {
    pub deviceid: xcb_input_device_id_t,
    pub mask_len: u16,
}

impl Default for xcb_input_event_mask_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::EventMask` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_event_mask_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_event_mask_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_event_mask_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::XISelectEvents` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_xi_select_events_request_t`].
pub const XCB_INPUT_XI_SELECT_EVENTS: u8 = 46i32 as u8;

/// The `Input::XISelectEvents` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `masks`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_select_events_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub num_mask: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_xi_select_events_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::XIQueryVersion` request.
///
/// Pass this cookie to [`xcb_input_xi_query_version_reply`] to retrieve the reply.
///
/// [`xcb_input_xi_query_version_reply`]: XcbXinput::xcb_input_xi_query_version_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_query_version_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_xi_query_version_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::XIQueryVersion` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_xi_query_version_request_t`].
pub const XCB_INPUT_XI_QUERY_VERSION: u8 = 47i32 as u8;

/// The `Input::XIQueryVersion` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub major_version: u16,
    pub minor_version: u16,
}

impl Default for xcb_input_xi_query_version_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::XIQueryVersion` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub pad1: [u8; 20],
}

impl Default for xcb_input_xi_query_version_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::DeviceClassType` enum.
///
/// This enum has the following variants:
///
/// - [`Input::DeviceClassType::Key`](XCB_INPUT_DEVICE_CLASS_TYPE_KEY)
/// - [`Input::DeviceClassType::Button`](XCB_INPUT_DEVICE_CLASS_TYPE_BUTTON)
/// - [`Input::DeviceClassType::Valuator`](XCB_INPUT_DEVICE_CLASS_TYPE_VALUATOR)
/// - [`Input::DeviceClassType::Scroll`](XCB_INPUT_DEVICE_CLASS_TYPE_SCROLL)
/// - [`Input::DeviceClassType::Touch`](XCB_INPUT_DEVICE_CLASS_TYPE_TOUCH)
pub type xcb_input_device_class_type_t = u32;
/// The `Input::DeviceClassType::Key` enum variant.
///
/// This is a variant of [`xcb_input_device_class_type_t`].
pub const XCB_INPUT_DEVICE_CLASS_TYPE_KEY: xcb_input_device_class_type_t = 0;
/// The `Input::DeviceClassType::Button` enum variant.
///
/// This is a variant of [`xcb_input_device_class_type_t`].
pub const XCB_INPUT_DEVICE_CLASS_TYPE_BUTTON: xcb_input_device_class_type_t = 1;
/// The `Input::DeviceClassType::Valuator` enum variant.
///
/// This is a variant of [`xcb_input_device_class_type_t`].
pub const XCB_INPUT_DEVICE_CLASS_TYPE_VALUATOR: xcb_input_device_class_type_t = 2;
/// The `Input::DeviceClassType::Scroll` enum variant.
///
/// This is a variant of [`xcb_input_device_class_type_t`].
pub const XCB_INPUT_DEVICE_CLASS_TYPE_SCROLL: xcb_input_device_class_type_t = 3;
/// The `Input::DeviceClassType::Touch` enum variant.
///
/// This is a variant of [`xcb_input_device_class_type_t`].
pub const XCB_INPUT_DEVICE_CLASS_TYPE_TOUCH: xcb_input_device_class_type_t = 8;

/// The `Input::DeviceType` enum.
///
/// This enum has the following variants:
///
/// - [`Input::DeviceType::MasterPointer`](XCB_INPUT_DEVICE_TYPE_MASTER_POINTER)
/// - [`Input::DeviceType::MasterKeyboard`](XCB_INPUT_DEVICE_TYPE_MASTER_KEYBOARD)
/// - [`Input::DeviceType::SlavePointer`](XCB_INPUT_DEVICE_TYPE_SLAVE_POINTER)
/// - [`Input::DeviceType::SlaveKeyboard`](XCB_INPUT_DEVICE_TYPE_SLAVE_KEYBOARD)
/// - [`Input::DeviceType::FloatingSlave`](XCB_INPUT_DEVICE_TYPE_FLOATING_SLAVE)
pub type xcb_input_device_type_t = u32;
/// The `Input::DeviceType::MasterPointer` enum variant.
///
/// This is a variant of [`xcb_input_device_type_t`].
pub const XCB_INPUT_DEVICE_TYPE_MASTER_POINTER: xcb_input_device_type_t = 1;
/// The `Input::DeviceType::MasterKeyboard` enum variant.
///
/// This is a variant of [`xcb_input_device_type_t`].
pub const XCB_INPUT_DEVICE_TYPE_MASTER_KEYBOARD: xcb_input_device_type_t = 2;
/// The `Input::DeviceType::SlavePointer` enum variant.
///
/// This is a variant of [`xcb_input_device_type_t`].
pub const XCB_INPUT_DEVICE_TYPE_SLAVE_POINTER: xcb_input_device_type_t = 3;
/// The `Input::DeviceType::SlaveKeyboard` enum variant.
///
/// This is a variant of [`xcb_input_device_type_t`].
pub const XCB_INPUT_DEVICE_TYPE_SLAVE_KEYBOARD: xcb_input_device_type_t = 4;
/// The `Input::DeviceType::FloatingSlave` enum variant.
///
/// This is a variant of [`xcb_input_device_type_t`].
pub const XCB_INPUT_DEVICE_TYPE_FLOATING_SLAVE: xcb_input_device_type_t = 5;

/// The `Input::ScrollFlags` enum.
///
/// This enum has the following variants:
///
/// - [`Input::ScrollFlags::NoEmulation`](XCB_INPUT_SCROLL_FLAGS_NO_EMULATION)
/// - [`Input::ScrollFlags::Preferred`](XCB_INPUT_SCROLL_FLAGS_PREFERRED)
pub type xcb_input_scroll_flags_t = u32;
/// The `Input::ScrollFlags::NoEmulation` enum variant.
///
/// This is a variant of [`xcb_input_scroll_flags_t`].
pub const XCB_INPUT_SCROLL_FLAGS_NO_EMULATION: xcb_input_scroll_flags_t = 1;
/// The `Input::ScrollFlags::Preferred` enum variant.
///
/// This is a variant of [`xcb_input_scroll_flags_t`].
pub const XCB_INPUT_SCROLL_FLAGS_PREFERRED: xcb_input_scroll_flags_t = 2;

/// The `Input::ScrollType` enum.
///
/// This enum has the following variants:
///
/// - [`Input::ScrollType::Vertical`](XCB_INPUT_SCROLL_TYPE_VERTICAL)
/// - [`Input::ScrollType::Horizontal`](XCB_INPUT_SCROLL_TYPE_HORIZONTAL)
pub type xcb_input_scroll_type_t = u32;
/// The `Input::ScrollType::Vertical` enum variant.
///
/// This is a variant of [`xcb_input_scroll_type_t`].
pub const XCB_INPUT_SCROLL_TYPE_VERTICAL: xcb_input_scroll_type_t = 1;
/// The `Input::ScrollType::Horizontal` enum variant.
///
/// This is a variant of [`xcb_input_scroll_type_t`].
pub const XCB_INPUT_SCROLL_TYPE_HORIZONTAL: xcb_input_scroll_type_t = 2;

/// The `Input::TouchMode` enum.
///
/// This enum has the following variants:
///
/// - [`Input::TouchMode::Direct`](XCB_INPUT_TOUCH_MODE_DIRECT)
/// - [`Input::TouchMode::Dependent`](XCB_INPUT_TOUCH_MODE_DEPENDENT)
pub type xcb_input_touch_mode_t = u32;
/// The `Input::TouchMode::Direct` enum variant.
///
/// This is a variant of [`xcb_input_touch_mode_t`].
pub const XCB_INPUT_TOUCH_MODE_DIRECT: xcb_input_touch_mode_t = 1;
/// The `Input::TouchMode::Dependent` enum variant.
///
/// This is a variant of [`xcb_input_touch_mode_t`].
pub const XCB_INPUT_TOUCH_MODE_DEPENDENT: xcb_input_touch_mode_t = 2;

/// The `Input::ButtonClass` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `state`
/// - `labels`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_button_class_t {
    pub type_: u16,
    pub len: u16,
    pub sourceid: xcb_input_device_id_t,
    pub num_buttons: u16,
}

impl Default for xcb_input_button_class_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::ButtonClass` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_button_class_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_button_class_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_button_class_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::KeyClass` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `keys`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_key_class_t {
    pub type_: u16,
    pub len: u16,
    pub sourceid: xcb_input_device_id_t,
    pub num_keys: u16,
}

impl Default for xcb_input_key_class_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::KeyClass` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_key_class_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_key_class_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_key_class_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::ScrollClass` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_scroll_class_t {
    pub type_: u16,
    pub len: u16,
    pub sourceid: xcb_input_device_id_t,
    pub number: u16,
    pub scroll_type: u16,
    pub pad0: [u8; 2],
    pub flags: u32,
    pub increment: xcb_input_fp3232_t,
}

impl Default for xcb_input_scroll_class_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::ScrollClass` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_scroll_class_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_scroll_class_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_scroll_class_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::TouchClass` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_touch_class_t {
    pub type_: u16,
    pub len: u16,
    pub sourceid: xcb_input_device_id_t,
    pub mode: u8,
    pub num_touches: u8,
}

impl Default for xcb_input_touch_class_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::TouchClass` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_touch_class_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_touch_class_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_touch_class_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::ValuatorClass` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_valuator_class_t {
    pub type_: u16,
    pub len: u16,
    pub sourceid: xcb_input_device_id_t,
    pub number: u16,
    pub label: xcb_atom_t,
    pub min: xcb_input_fp3232_t,
    pub max: xcb_input_fp3232_t,
    pub value: xcb_input_fp3232_t,
    pub resolution: u32,
    pub mode: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_valuator_class_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::ValuatorClass` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_valuator_class_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_valuator_class_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_valuator_class_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_device_class_data_t::key`].
///
/// In libxcb, this type is an anonymous struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `keys`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_class_data_t__key {
    pub num_keys: u16,
    pub keys: *mut u32,
}

impl Default for xcb_input_device_class_data_t__key {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_device_class_data_t::button`].
///
/// In libxcb, this type is an anonymous struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `state`
/// - `labels`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_class_data_t__button {
    pub num_buttons: u16,
    pub state: *mut u32,
    pub labels: *mut xcb_atom_t,
}

impl Default for xcb_input_device_class_data_t__button {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_device_class_data_t::valuator`].
///
/// In libxcb, this type is an anonymous struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_class_data_t__valuator {
    pub number: u16,
    pub label: xcb_atom_t,
    pub min: xcb_input_fp3232_t,
    pub max: xcb_input_fp3232_t,
    pub value: xcb_input_fp3232_t,
    pub resolution: u32,
    pub mode: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_device_class_data_t__valuator {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_device_class_data_t::scroll`].
///
/// In libxcb, this type is an anonymous struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_class_data_t__scroll {
    pub number: u16,
    pub scroll_type: u16,
    pub pad1: [u8; 2],
    pub flags: u32,
    pub increment: xcb_input_fp3232_t,
}

impl Default for xcb_input_device_class_data_t__scroll {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_input_device_class_data_t::touch`].
///
/// In libxcb, this type is an anonymous struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_class_data_t__touch {
    pub mode: u8,
    pub num_touches: u8,
}

impl Default for xcb_input_device_class_data_t__touch {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::data` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_class_data_t {
    pub key: xcb_input_device_class_data_t__key,
    pub button: xcb_input_device_class_data_t__button,
    pub valuator: xcb_input_device_class_data_t__valuator,
    pub scroll: xcb_input_device_class_data_t__scroll,
    pub touch: xcb_input_device_class_data_t__touch,
}

impl Default for xcb_input_device_class_data_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::DeviceClass` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `data`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_class_t {
    pub type_: u16,
    pub len: u16,
    pub sourceid: xcb_input_device_id_t,
}

impl Default for xcb_input_device_class_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::DeviceClass` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_class_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_device_class_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_device_class_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::XIDeviceInfo` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `name`
/// - `classes`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_device_info_t {
    pub deviceid: xcb_input_device_id_t,
    pub type_: u16,
    pub attachment: xcb_input_device_id_t,
    pub num_classes: u16,
    pub name_len: u16,
    pub enabled: u8,
    pub pad0: u8,
}

impl Default for xcb_input_xi_device_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::XIDeviceInfo` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_device_info_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_xi_device_info_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_xi_device_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::XIQueryDevice` request.
///
/// Pass this cookie to [`xcb_input_xi_query_device_reply`] to retrieve the reply.
///
/// [`xcb_input_xi_query_device_reply`]: XcbXinput::xcb_input_xi_query_device_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_query_device_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_xi_query_device_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::XIQueryDevice` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_xi_query_device_request_t`].
pub const XCB_INPUT_XI_QUERY_DEVICE: u8 = 48i32 as u8;

/// The `Input::XIQueryDevice` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_query_device_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub deviceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_xi_query_device_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::XIQueryDevice` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `infos`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_query_device_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_infos: u16,
    pub pad1: [u8; 22],
}

impl Default for xcb_input_xi_query_device_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::XISetFocus` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_xi_set_focus_request_t`].
pub const XCB_INPUT_XI_SET_FOCUS: u8 = 49i32 as u8;

/// The `Input::XISetFocus` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_set_focus_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub time: xcb_timestamp_t,
    pub deviceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_xi_set_focus_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::XIGetFocus` request.
///
/// Pass this cookie to [`xcb_input_xi_get_focus_reply`] to retrieve the reply.
///
/// [`xcb_input_xi_get_focus_reply`]: XcbXinput::xcb_input_xi_get_focus_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_get_focus_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_xi_get_focus_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::XIGetFocus` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_xi_get_focus_request_t`].
pub const XCB_INPUT_XI_GET_FOCUS: u8 = 50i32 as u8;

/// The `Input::XIGetFocus` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_get_focus_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub deviceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_xi_get_focus_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::XIGetFocus` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_get_focus_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub focus: xcb_window_t,
    pub pad1: [u8; 20],
}

impl Default for xcb_input_xi_get_focus_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::GrabOwner` enum.
///
/// This enum has the following variants:
///
/// - [`Input::GrabOwner::NoOwner`](XCB_INPUT_GRAB_OWNER_NO_OWNER)
/// - [`Input::GrabOwner::Owner`](XCB_INPUT_GRAB_OWNER_OWNER)
pub type xcb_input_grab_owner_t = u32;
/// The `Input::GrabOwner::NoOwner` enum variant.
///
/// This is a variant of [`xcb_input_grab_owner_t`].
pub const XCB_INPUT_GRAB_OWNER_NO_OWNER: xcb_input_grab_owner_t = 0;
/// The `Input::GrabOwner::Owner` enum variant.
///
/// This is a variant of [`xcb_input_grab_owner_t`].
pub const XCB_INPUT_GRAB_OWNER_OWNER: xcb_input_grab_owner_t = 1;

/// The cookie for the reply to a `Input::XIGrabDevice` request.
///
/// Pass this cookie to [`xcb_input_xi_grab_device_reply`] to retrieve the reply.
///
/// [`xcb_input_xi_grab_device_reply`]: XcbXinput::xcb_input_xi_grab_device_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_grab_device_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_xi_grab_device_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::XIGrabDevice` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_xi_grab_device_request_t`].
pub const XCB_INPUT_XI_GRAB_DEVICE: u8 = 51i32 as u8;

/// The `Input::XIGrabDevice` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `mask`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_grab_device_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub time: xcb_timestamp_t,
    pub cursor: xcb_cursor_t,
    pub deviceid: xcb_input_device_id_t,
    pub mode: u8,
    pub paired_device_mode: u8,
    pub owner_events: u8,
    pub pad0: u8,
    pub mask_len: u16,
}

impl Default for xcb_input_xi_grab_device_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::XIGrabDevice` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_grab_device_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: u8,
    pub pad1: [u8; 23],
}

impl Default for xcb_input_xi_grab_device_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::XIUngrabDevice` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_xi_ungrab_device_request_t`].
pub const XCB_INPUT_XI_UNGRAB_DEVICE: u8 = 52i32 as u8;

/// The `Input::XIUngrabDevice` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_ungrab_device_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub time: xcb_timestamp_t,
    pub deviceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_xi_ungrab_device_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::EventMode` enum.
///
/// This enum has the following variants:
///
/// - [`Input::EventMode::AsyncDevice`](XCB_INPUT_EVENT_MODE_ASYNC_DEVICE)
/// - [`Input::EventMode::SyncDevice`](XCB_INPUT_EVENT_MODE_SYNC_DEVICE)
/// - [`Input::EventMode::ReplayDevice`](XCB_INPUT_EVENT_MODE_REPLAY_DEVICE)
/// - [`Input::EventMode::AsyncPairedDevice`](XCB_INPUT_EVENT_MODE_ASYNC_PAIRED_DEVICE)
/// - [`Input::EventMode::AsyncPair`](XCB_INPUT_EVENT_MODE_ASYNC_PAIR)
/// - [`Input::EventMode::SyncPair`](XCB_INPUT_EVENT_MODE_SYNC_PAIR)
/// - [`Input::EventMode::AcceptTouch`](XCB_INPUT_EVENT_MODE_ACCEPT_TOUCH)
/// - [`Input::EventMode::RejectTouch`](XCB_INPUT_EVENT_MODE_REJECT_TOUCH)
pub type xcb_input_event_mode_t = u32;
/// The `Input::EventMode::AsyncDevice` enum variant.
///
/// This is a variant of [`xcb_input_event_mode_t`].
pub const XCB_INPUT_EVENT_MODE_ASYNC_DEVICE: xcb_input_event_mode_t = 0;
/// The `Input::EventMode::SyncDevice` enum variant.
///
/// This is a variant of [`xcb_input_event_mode_t`].
pub const XCB_INPUT_EVENT_MODE_SYNC_DEVICE: xcb_input_event_mode_t = 1;
/// The `Input::EventMode::ReplayDevice` enum variant.
///
/// This is a variant of [`xcb_input_event_mode_t`].
pub const XCB_INPUT_EVENT_MODE_REPLAY_DEVICE: xcb_input_event_mode_t = 2;
/// The `Input::EventMode::AsyncPairedDevice` enum variant.
///
/// This is a variant of [`xcb_input_event_mode_t`].
pub const XCB_INPUT_EVENT_MODE_ASYNC_PAIRED_DEVICE: xcb_input_event_mode_t = 3;
/// The `Input::EventMode::AsyncPair` enum variant.
///
/// This is a variant of [`xcb_input_event_mode_t`].
pub const XCB_INPUT_EVENT_MODE_ASYNC_PAIR: xcb_input_event_mode_t = 4;
/// The `Input::EventMode::SyncPair` enum variant.
///
/// This is a variant of [`xcb_input_event_mode_t`].
pub const XCB_INPUT_EVENT_MODE_SYNC_PAIR: xcb_input_event_mode_t = 5;
/// The `Input::EventMode::AcceptTouch` enum variant.
///
/// This is a variant of [`xcb_input_event_mode_t`].
pub const XCB_INPUT_EVENT_MODE_ACCEPT_TOUCH: xcb_input_event_mode_t = 6;
/// The `Input::EventMode::RejectTouch` enum variant.
///
/// This is a variant of [`xcb_input_event_mode_t`].
pub const XCB_INPUT_EVENT_MODE_REJECT_TOUCH: xcb_input_event_mode_t = 7;

/// The opcode for `Input::XIAllowEvents` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_xi_allow_events_request_t`].
pub const XCB_INPUT_XI_ALLOW_EVENTS: u8 = 53i32 as u8;

/// The `Input::XIAllowEvents` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_allow_events_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub time: xcb_timestamp_t,
    pub deviceid: xcb_input_device_id_t,
    pub event_mode: u8,
    pub pad0: u8,
    pub touchid: u32,
    pub grab_window: xcb_window_t,
}

impl Default for xcb_input_xi_allow_events_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::GrabMode22` enum.
///
/// This enum has the following variants:
///
/// - [`Input::GrabMode22::Sync`](XCB_INPUT_GRAB_MODE_22_SYNC)
/// - [`Input::GrabMode22::Async`](XCB_INPUT_GRAB_MODE_22_ASYNC)
/// - [`Input::GrabMode22::Touch`](XCB_INPUT_GRAB_MODE_22_TOUCH)
pub type xcb_input_grab_mode_22_t = u32;
/// The `Input::GrabMode22::Sync` enum variant.
///
/// This is a variant of [`xcb_input_grab_mode_22_t`].
pub const XCB_INPUT_GRAB_MODE_22_SYNC: xcb_input_grab_mode_22_t = 0;
/// The `Input::GrabMode22::Async` enum variant.
///
/// This is a variant of [`xcb_input_grab_mode_22_t`].
pub const XCB_INPUT_GRAB_MODE_22_ASYNC: xcb_input_grab_mode_22_t = 1;
/// The `Input::GrabMode22::Touch` enum variant.
///
/// This is a variant of [`xcb_input_grab_mode_22_t`].
pub const XCB_INPUT_GRAB_MODE_22_TOUCH: xcb_input_grab_mode_22_t = 2;

/// The `Input::GrabType` enum.
///
/// This enum has the following variants:
///
/// - [`Input::GrabType::Button`](XCB_INPUT_GRAB_TYPE_BUTTON)
/// - [`Input::GrabType::Keycode`](XCB_INPUT_GRAB_TYPE_KEYCODE)
/// - [`Input::GrabType::Enter`](XCB_INPUT_GRAB_TYPE_ENTER)
/// - [`Input::GrabType::FocusIn`](XCB_INPUT_GRAB_TYPE_FOCUS_IN)
/// - [`Input::GrabType::TouchBegin`](XCB_INPUT_GRAB_TYPE_TOUCH_BEGIN)
pub type xcb_input_grab_type_t = u32;
/// The `Input::GrabType::Button` enum variant.
///
/// This is a variant of [`xcb_input_grab_type_t`].
pub const XCB_INPUT_GRAB_TYPE_BUTTON: xcb_input_grab_type_t = 0;
/// The `Input::GrabType::Keycode` enum variant.
///
/// This is a variant of [`xcb_input_grab_type_t`].
pub const XCB_INPUT_GRAB_TYPE_KEYCODE: xcb_input_grab_type_t = 1;
/// The `Input::GrabType::Enter` enum variant.
///
/// This is a variant of [`xcb_input_grab_type_t`].
pub const XCB_INPUT_GRAB_TYPE_ENTER: xcb_input_grab_type_t = 2;
/// The `Input::GrabType::FocusIn` enum variant.
///
/// This is a variant of [`xcb_input_grab_type_t`].
pub const XCB_INPUT_GRAB_TYPE_FOCUS_IN: xcb_input_grab_type_t = 3;
/// The `Input::GrabType::TouchBegin` enum variant.
///
/// This is a variant of [`xcb_input_grab_type_t`].
pub const XCB_INPUT_GRAB_TYPE_TOUCH_BEGIN: xcb_input_grab_type_t = 4;

/// The `Input::ModifierMask` enum.
///
/// This enum has the following variants:
///
/// - [`Input::ModifierMask::Any`](XCB_INPUT_MODIFIER_MASK_ANY)
pub type xcb_input_modifier_mask_t = u32;
/// The `Input::ModifierMask::Any` enum variant.
///
/// This is a variant of [`xcb_input_modifier_mask_t`].
pub const XCB_INPUT_MODIFIER_MASK_ANY: xcb_input_modifier_mask_t = 2147483648;

/// The `Input::GrabModifierInfo` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_grab_modifier_info_t {
    pub modifiers: u32,
    pub status: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_grab_modifier_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::GrabModifierInfo` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_grab_modifier_info_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_grab_modifier_info_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_grab_modifier_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::XIPassiveGrabDevice` request.
///
/// Pass this cookie to [`xcb_input_xi_passive_grab_device_reply`] to retrieve the reply.
///
/// [`xcb_input_xi_passive_grab_device_reply`]: XcbXinput::xcb_input_xi_passive_grab_device_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_passive_grab_device_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_xi_passive_grab_device_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::XIPassiveGrabDevice` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_xi_passive_grab_device_request_t`].
pub const XCB_INPUT_XI_PASSIVE_GRAB_DEVICE: u8 = 54i32 as u8;

/// The `Input::XIPassiveGrabDevice` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `mask`
/// - `modifiers`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_passive_grab_device_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub time: xcb_timestamp_t,
    pub grab_window: xcb_window_t,
    pub cursor: xcb_cursor_t,
    pub detail: u32,
    pub deviceid: xcb_input_device_id_t,
    pub num_modifiers: u16,
    pub mask_len: u16,
    pub grab_type: u8,
    pub grab_mode: u8,
    pub paired_device_mode: u8,
    pub owner_events: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_xi_passive_grab_device_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::XIPassiveGrabDevice` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `modifiers`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_passive_grab_device_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_modifiers: u16,
    pub pad1: [u8; 22],
}

impl Default for xcb_input_xi_passive_grab_device_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::XIPassiveUngrabDevice` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_xi_passive_ungrab_device_request_t`].
pub const XCB_INPUT_XI_PASSIVE_UNGRAB_DEVICE: u8 = 55i32 as u8;

/// The `Input::XIPassiveUngrabDevice` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `modifiers`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_passive_ungrab_device_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub grab_window: xcb_window_t,
    pub detail: u32,
    pub deviceid: xcb_input_device_id_t,
    pub num_modifiers: u16,
    pub grab_type: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_xi_passive_ungrab_device_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::XIListProperties` request.
///
/// Pass this cookie to [`xcb_input_xi_list_properties_reply`] to retrieve the reply.
///
/// [`xcb_input_xi_list_properties_reply`]: XcbXinput::xcb_input_xi_list_properties_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_list_properties_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_xi_list_properties_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::XIListProperties` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_xi_list_properties_request_t`].
pub const XCB_INPUT_XI_LIST_PROPERTIES: u8 = 56i32 as u8;

/// The `Input::XIListProperties` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_list_properties_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub deviceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
}

impl Default for xcb_input_xi_list_properties_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::XIListProperties` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `properties`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_list_properties_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_properties: u16,
    pub pad1: [u8; 22],
}

impl Default for xcb_input_xi_list_properties_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::items` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_change_property_items_t {
    pub data8: *mut u8,
    pub data16: *mut u16,
    pub data32: *mut u32,
}

impl Default for xcb_input_xi_change_property_items_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::XIChangeProperty` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_xi_change_property_request_t`].
pub const XCB_INPUT_XI_CHANGE_PROPERTY: u8 = 57i32 as u8;

/// The `Input::XIChangeProperty` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `items`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_change_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub deviceid: xcb_input_device_id_t,
    pub mode: u8,
    pub format: u8,
    pub property: xcb_atom_t,
    pub type_: xcb_atom_t,
    pub num_items: u32,
}

impl Default for xcb_input_xi_change_property_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::XIDeleteProperty` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_xi_delete_property_request_t`].
pub const XCB_INPUT_XI_DELETE_PROPERTY: u8 = 58i32 as u8;

/// The `Input::XIDeleteProperty` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_delete_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub deviceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
    pub property: xcb_atom_t,
}

impl Default for xcb_input_xi_delete_property_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::XIGetProperty` request.
///
/// Pass this cookie to [`xcb_input_xi_get_property_reply`] to retrieve the reply.
///
/// [`xcb_input_xi_get_property_reply`]: XcbXinput::xcb_input_xi_get_property_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_get_property_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_xi_get_property_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::XIGetProperty` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_xi_get_property_request_t`].
pub const XCB_INPUT_XI_GET_PROPERTY: u8 = 59i32 as u8;

/// The `Input::XIGetProperty` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_get_property_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub deviceid: xcb_input_device_id_t,
    pub delete: u8,
    pub pad0: u8,
    pub property: xcb_atom_t,
    pub type_: xcb_atom_t,
    pub offset: u32,
    pub len: u32,
}

impl Default for xcb_input_xi_get_property_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::items` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_get_property_items_t {
    pub data8: *mut u8,
    pub data16: *mut u16,
    pub data32: *mut u32,
}

impl Default for xcb_input_xi_get_property_items_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::XIGetProperty` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `items`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_get_property_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub type_: xcb_atom_t,
    pub bytes_after: u32,
    pub num_items: u32,
    pub format: u8,
    pub pad1: [u8; 11],
}

impl Default for xcb_input_xi_get_property_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `Input::XIGetSelectedEvents` request.
///
/// Pass this cookie to [`xcb_input_xi_get_selected_events_reply`] to retrieve the reply.
///
/// [`xcb_input_xi_get_selected_events_reply`]: XcbXinput::xcb_input_xi_get_selected_events_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_get_selected_events_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_input_xi_get_selected_events_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::XIGetSelectedEvents` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_xi_get_selected_events_request_t`].
pub const XCB_INPUT_XI_GET_SELECTED_EVENTS: u8 = 60i32 as u8;

/// The `Input::XIGetSelectedEvents` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_get_selected_events_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
}

impl Default for xcb_input_xi_get_selected_events_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::XIGetSelectedEvents` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `masks`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_get_selected_events_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub num_masks: u16,
    pub pad1: [u8; 22],
}

impl Default for xcb_input_xi_get_selected_events_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::BarrierReleasePointerInfo` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_barrier_release_pointer_info_t {
    pub deviceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
    pub barrier: xcb_xfixes_barrier_t,
    pub eventid: u32,
}

impl Default for xcb_input_barrier_release_pointer_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::BarrierReleasePointerInfo` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_barrier_release_pointer_info_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_barrier_release_pointer_info_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_barrier_release_pointer_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::XIBarrierReleasePointer` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_xi_barrier_release_pointer_request_t`].
pub const XCB_INPUT_XI_BARRIER_RELEASE_POINTER: u8 = 61i32 as u8;

/// The `Input::XIBarrierReleasePointer` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `barriers`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_xi_barrier_release_pointer_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub num_barriers: u32,
}

impl Default for xcb_input_xi_barrier_release_pointer_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::DeviceValuator` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_input_device_valuator_event_t`].
pub const XCB_INPUT_DEVICE_VALUATOR: u8 = 0i32 as u8;

/// The `Input::DeviceValuator` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_valuator_event_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub device_state: u16,
    pub num_valuators: u8,
    pub first_valuator: u8,
    pub valuators: [i32; 6],
}

impl Default for xcb_input_device_valuator_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::MoreEventsMask` enum.
///
/// This enum has the following variants:
///
/// - [`Input::MoreEventsMask::MoreEvents`](XCB_INPUT_MORE_EVENTS_MASK_MORE_EVENTS)
pub type xcb_input_more_events_mask_t = u32;
/// The `Input::MoreEventsMask::MoreEvents` enum variant.
///
/// This is a variant of [`xcb_input_more_events_mask_t`].
pub const XCB_INPUT_MORE_EVENTS_MASK_MORE_EVENTS: xcb_input_more_events_mask_t = 128;

/// The opcode for `Input::DeviceKeyPress` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_input_device_key_press_event_t`].
pub const XCB_INPUT_DEVICE_KEY_PRESS: u8 = 1i32 as u8;

/// The `Input::DeviceKeyPress` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_key_press_event_t {
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
    pub device_id: u8,
}

impl Default for xcb_input_device_key_press_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::DeviceKeyRelease` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_input_device_key_release_event_t`].
pub const XCB_INPUT_DEVICE_KEY_RELEASE: u8 = 2i32 as u8;

/// The `Input::DeviceKeyRelease` event.
pub type xcb_input_device_key_release_event_t = xcb_input_device_key_press_event_t;

/// The opcode for `Input::DeviceButtonPress` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_input_device_button_press_event_t`].
pub const XCB_INPUT_DEVICE_BUTTON_PRESS: u8 = 3i32 as u8;

/// The `Input::DeviceButtonPress` event.
pub type xcb_input_device_button_press_event_t = xcb_input_device_key_press_event_t;

/// The opcode for `Input::DeviceButtonRelease` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_input_device_button_release_event_t`].
pub const XCB_INPUT_DEVICE_BUTTON_RELEASE: u8 = 4i32 as u8;

/// The `Input::DeviceButtonRelease` event.
pub type xcb_input_device_button_release_event_t = xcb_input_device_key_press_event_t;

/// The opcode for `Input::DeviceMotionNotify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_input_device_motion_notify_event_t`].
pub const XCB_INPUT_DEVICE_MOTION_NOTIFY: u8 = 5i32 as u8;

/// The `Input::DeviceMotionNotify` event.
pub type xcb_input_device_motion_notify_event_t = xcb_input_device_key_press_event_t;

/// The opcode for `Input::DeviceFocusIn` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_input_device_focus_in_event_t`].
pub const XCB_INPUT_DEVICE_FOCUS_IN: u8 = 6i32 as u8;

/// The `Input::DeviceFocusIn` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_focus_in_event_t {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub window: xcb_window_t,
    pub mode: u8,
    pub device_id: u8,
    pub pad0: [u8; 18],
}

impl Default for xcb_input_device_focus_in_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::DeviceFocusOut` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_input_device_focus_out_event_t`].
pub const XCB_INPUT_DEVICE_FOCUS_OUT: u8 = 7i32 as u8;

/// The `Input::DeviceFocusOut` event.
pub type xcb_input_device_focus_out_event_t = xcb_input_device_focus_in_event_t;

/// The opcode for `Input::ProximityIn` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_input_proximity_in_event_t`].
pub const XCB_INPUT_PROXIMITY_IN: u8 = 8i32 as u8;

/// The `Input::ProximityIn` event.
pub type xcb_input_proximity_in_event_t = xcb_input_device_key_press_event_t;

/// The opcode for `Input::ProximityOut` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_input_proximity_out_event_t`].
pub const XCB_INPUT_PROXIMITY_OUT: u8 = 9i32 as u8;

/// The `Input::ProximityOut` event.
pub type xcb_input_proximity_out_event_t = xcb_input_device_key_press_event_t;

/// The `Input::ClassesReportedMask` enum.
///
/// This enum has the following variants:
///
/// - [`Input::ClassesReportedMask::OutOfProximity`](XCB_INPUT_CLASSES_REPORTED_MASK_OUT_OF_PROXIMITY)
/// - [`Input::ClassesReportedMask::DeviceModeAbsolute`](XCB_INPUT_CLASSES_REPORTED_MASK_DEVICE_MODE_ABSOLUTE)
/// - [`Input::ClassesReportedMask::ReportingValuators`](XCB_INPUT_CLASSES_REPORTED_MASK_REPORTING_VALUATORS)
/// - [`Input::ClassesReportedMask::ReportingButtons`](XCB_INPUT_CLASSES_REPORTED_MASK_REPORTING_BUTTONS)
/// - [`Input::ClassesReportedMask::ReportingKeys`](XCB_INPUT_CLASSES_REPORTED_MASK_REPORTING_KEYS)
pub type xcb_input_classes_reported_mask_t = u32;
/// The `Input::ClassesReportedMask::OutOfProximity` enum variant.
///
/// This is a variant of [`xcb_input_classes_reported_mask_t`].
pub const XCB_INPUT_CLASSES_REPORTED_MASK_OUT_OF_PROXIMITY: xcb_input_classes_reported_mask_t = 128;
/// The `Input::ClassesReportedMask::DeviceModeAbsolute` enum variant.
///
/// This is a variant of [`xcb_input_classes_reported_mask_t`].
pub const XCB_INPUT_CLASSES_REPORTED_MASK_DEVICE_MODE_ABSOLUTE: xcb_input_classes_reported_mask_t =
    64;
/// The `Input::ClassesReportedMask::ReportingValuators` enum variant.
///
/// This is a variant of [`xcb_input_classes_reported_mask_t`].
pub const XCB_INPUT_CLASSES_REPORTED_MASK_REPORTING_VALUATORS: xcb_input_classes_reported_mask_t =
    4;
/// The `Input::ClassesReportedMask::ReportingButtons` enum variant.
///
/// This is a variant of [`xcb_input_classes_reported_mask_t`].
pub const XCB_INPUT_CLASSES_REPORTED_MASK_REPORTING_BUTTONS: xcb_input_classes_reported_mask_t = 2;
/// The `Input::ClassesReportedMask::ReportingKeys` enum variant.
///
/// This is a variant of [`xcb_input_classes_reported_mask_t`].
pub const XCB_INPUT_CLASSES_REPORTED_MASK_REPORTING_KEYS: xcb_input_classes_reported_mask_t = 1;

/// The opcode for `Input::DeviceStateNotify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_input_device_state_notify_event_t`].
pub const XCB_INPUT_DEVICE_STATE_NOTIFY: u8 = 10i32 as u8;

/// The `Input::DeviceStateNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_state_notify_event_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub num_keys: u8,
    pub num_buttons: u8,
    pub num_valuators: u8,
    pub classes_reported: u8,
    pub buttons: [u8; 4],
    pub keys: [u8; 4],
    pub valuators: [u32; 3],
}

impl Default for xcb_input_device_state_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::DeviceMappingNotify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_input_device_mapping_notify_event_t`].
pub const XCB_INPUT_DEVICE_MAPPING_NOTIFY: u8 = 11i32 as u8;

/// The `Input::DeviceMappingNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_mapping_notify_event_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub request: u8,
    pub first_keycode: xcb_input_key_code_t,
    pub count: u8,
    pub pad0: u8,
    pub time: xcb_timestamp_t,
    pub pad1: [u8; 20],
}

impl Default for xcb_input_device_mapping_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::ChangeDevice` enum.
///
/// This enum has the following variants:
///
/// - [`Input::ChangeDevice::NewPointer`](XCB_INPUT_CHANGE_DEVICE_NEW_POINTER)
/// - [`Input::ChangeDevice::NewKeyboard`](XCB_INPUT_CHANGE_DEVICE_NEW_KEYBOARD)
pub type xcb_input_change_device_t = u32;
/// The `Input::ChangeDevice::NewPointer` enum variant.
///
/// This is a variant of [`xcb_input_change_device_t`].
pub const XCB_INPUT_CHANGE_DEVICE_NEW_POINTER: xcb_input_change_device_t = 0;
/// The `Input::ChangeDevice::NewKeyboard` enum variant.
///
/// This is a variant of [`xcb_input_change_device_t`].
pub const XCB_INPUT_CHANGE_DEVICE_NEW_KEYBOARD: xcb_input_change_device_t = 1;

/// The opcode for `Input::ChangeDeviceNotify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_input_change_device_notify_event_t`].
pub const XCB_INPUT_CHANGE_DEVICE_NOTIFY: u8 = 12i32 as u8;

/// The `Input::ChangeDeviceNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_change_device_notify_event_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub request: u8,
    pub pad0: [u8; 23],
}

impl Default for xcb_input_change_device_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::DeviceKeyStateNotify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_input_device_key_state_notify_event_t`].
pub const XCB_INPUT_DEVICE_KEY_STATE_NOTIFY: u8 = 13i32 as u8;

/// The `Input::DeviceKeyStateNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_key_state_notify_event_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub keys: [u8; 28],
}

impl Default for xcb_input_device_key_state_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::DeviceButtonStateNotify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_input_device_button_state_notify_event_t`].
pub const XCB_INPUT_DEVICE_BUTTON_STATE_NOTIFY: u8 = 14i32 as u8;

/// The `Input::DeviceButtonStateNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_button_state_notify_event_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub buttons: [u8; 28],
}

impl Default for xcb_input_device_button_state_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::DeviceChange` enum.
///
/// This enum has the following variants:
///
/// - [`Input::DeviceChange::Added`](XCB_INPUT_DEVICE_CHANGE_ADDED)
/// - [`Input::DeviceChange::Removed`](XCB_INPUT_DEVICE_CHANGE_REMOVED)
/// - [`Input::DeviceChange::Enabled`](XCB_INPUT_DEVICE_CHANGE_ENABLED)
/// - [`Input::DeviceChange::Disabled`](XCB_INPUT_DEVICE_CHANGE_DISABLED)
/// - [`Input::DeviceChange::Unrecoverable`](XCB_INPUT_DEVICE_CHANGE_UNRECOVERABLE)
/// - [`Input::DeviceChange::ControlChanged`](XCB_INPUT_DEVICE_CHANGE_CONTROL_CHANGED)
pub type xcb_input_device_change_t = u32;
/// The `Input::DeviceChange::Added` enum variant.
///
/// This is a variant of [`xcb_input_device_change_t`].
pub const XCB_INPUT_DEVICE_CHANGE_ADDED: xcb_input_device_change_t = 0;
/// The `Input::DeviceChange::Removed` enum variant.
///
/// This is a variant of [`xcb_input_device_change_t`].
pub const XCB_INPUT_DEVICE_CHANGE_REMOVED: xcb_input_device_change_t = 1;
/// The `Input::DeviceChange::Enabled` enum variant.
///
/// This is a variant of [`xcb_input_device_change_t`].
pub const XCB_INPUT_DEVICE_CHANGE_ENABLED: xcb_input_device_change_t = 2;
/// The `Input::DeviceChange::Disabled` enum variant.
///
/// This is a variant of [`xcb_input_device_change_t`].
pub const XCB_INPUT_DEVICE_CHANGE_DISABLED: xcb_input_device_change_t = 3;
/// The `Input::DeviceChange::Unrecoverable` enum variant.
///
/// This is a variant of [`xcb_input_device_change_t`].
pub const XCB_INPUT_DEVICE_CHANGE_UNRECOVERABLE: xcb_input_device_change_t = 4;
/// The `Input::DeviceChange::ControlChanged` enum variant.
///
/// This is a variant of [`xcb_input_device_change_t`].
pub const XCB_INPUT_DEVICE_CHANGE_CONTROL_CHANGED: xcb_input_device_change_t = 5;

/// The opcode for `Input::DevicePresenceNotify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_input_device_presence_notify_event_t`].
pub const XCB_INPUT_DEVICE_PRESENCE_NOTIFY: u8 = 15i32 as u8;

/// The `Input::DevicePresenceNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_presence_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub devchange: u8,
    pub device_id: u8,
    pub control: u16,
    pub pad1: [u8; 20],
}

impl Default for xcb_input_device_presence_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::DevicePropertyNotify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_input_device_property_notify_event_t`].
pub const XCB_INPUT_DEVICE_PROPERTY_NOTIFY: u8 = 16i32 as u8;

/// The `Input::DevicePropertyNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_property_notify_event_t {
    pub response_type: u8,
    pub state: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub property: xcb_atom_t,
    pub pad0: [u8; 19],
    pub device_id: u8,
}

impl Default for xcb_input_device_property_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::ChangeReason` enum.
///
/// This enum has the following variants:
///
/// - [`Input::ChangeReason::SlaveSwitch`](XCB_INPUT_CHANGE_REASON_SLAVE_SWITCH)
/// - [`Input::ChangeReason::DeviceChange`](XCB_INPUT_CHANGE_REASON_DEVICE_CHANGE)
pub type xcb_input_change_reason_t = u32;
/// The `Input::ChangeReason::SlaveSwitch` enum variant.
///
/// This is a variant of [`xcb_input_change_reason_t`].
pub const XCB_INPUT_CHANGE_REASON_SLAVE_SWITCH: xcb_input_change_reason_t = 1;
/// The `Input::ChangeReason::DeviceChange` enum variant.
///
/// This is a variant of [`xcb_input_change_reason_t`].
pub const XCB_INPUT_CHANGE_REASON_DEVICE_CHANGE: xcb_input_change_reason_t = 2;

/// The opcode for `Input::DeviceChanged` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Input` extension,
/// then the type of the event is [`xcb_input_device_changed_event_t`].
pub const XCB_INPUT_DEVICE_CHANGED: u16 = 1i32 as u16;

/// The `Input::DeviceChanged` event.
///
/// The following fields can be accessed via accessor functions:
///
/// - `classes`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_changed_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: xcb_input_device_id_t,
    pub time: xcb_timestamp_t,
    pub num_classes: u16,
    pub sourceid: xcb_input_device_id_t,
    pub reason: u8,
    pub pad0: [u8; 11],
    pub full_sequence: u32,
}

impl Default for xcb_input_device_changed_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::KeyEventFlags` enum.
///
/// This enum has the following variants:
///
/// - [`Input::KeyEventFlags::KeyRepeat`](XCB_INPUT_KEY_EVENT_FLAGS_KEY_REPEAT)
pub type xcb_input_key_event_flags_t = u32;
/// The `Input::KeyEventFlags::KeyRepeat` enum variant.
///
/// This is a variant of [`xcb_input_key_event_flags_t`].
pub const XCB_INPUT_KEY_EVENT_FLAGS_KEY_REPEAT: xcb_input_key_event_flags_t = 65536;

/// The opcode for `Input::KeyPress` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Input` extension,
/// then the type of the event is [`xcb_input_key_press_event_t`].
pub const XCB_INPUT_KEY_PRESS: u16 = 2i32 as u16;

/// The `Input::KeyPress` event.
///
/// The following fields can be accessed via accessor functions:
///
/// - `button_mask`
/// - `valuator_mask`
/// - `axisvalues`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_key_press_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: xcb_input_device_id_t,
    pub time: xcb_timestamp_t,
    pub detail: u32,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub full_sequence: u32,
    pub root_x: xcb_input_fp1616_t,
    pub root_y: xcb_input_fp1616_t,
    pub event_x: xcb_input_fp1616_t,
    pub event_y: xcb_input_fp1616_t,
    pub buttons_len: u16,
    pub valuators_len: u16,
    pub sourceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
    pub flags: u32,
    pub mods: xcb_input_modifier_info_t,
    pub group: xcb_input_group_info_t,
}

impl Default for xcb_input_key_press_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::KeyRelease` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Input` extension,
/// then the type of the event is [`xcb_input_key_release_event_t`].
pub const XCB_INPUT_KEY_RELEASE: u16 = 3i32 as u16;

/// The `Input::KeyRelease` event.
pub type xcb_input_key_release_event_t = xcb_input_key_press_event_t;

/// The `Input::PointerEventFlags` enum.
///
/// This enum has the following variants:
///
/// - [`Input::PointerEventFlags::PointerEmulated`](XCB_INPUT_POINTER_EVENT_FLAGS_POINTER_EMULATED)
pub type xcb_input_pointer_event_flags_t = u32;
/// The `Input::PointerEventFlags::PointerEmulated` enum variant.
///
/// This is a variant of [`xcb_input_pointer_event_flags_t`].
pub const XCB_INPUT_POINTER_EVENT_FLAGS_POINTER_EMULATED: xcb_input_pointer_event_flags_t = 65536;

/// The opcode for `Input::ButtonPress` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Input` extension,
/// then the type of the event is [`xcb_input_button_press_event_t`].
pub const XCB_INPUT_BUTTON_PRESS: u16 = 4i32 as u16;

/// The `Input::ButtonPress` event.
///
/// The following fields can be accessed via accessor functions:
///
/// - `button_mask`
/// - `valuator_mask`
/// - `axisvalues`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_button_press_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: xcb_input_device_id_t,
    pub time: xcb_timestamp_t,
    pub detail: u32,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub full_sequence: u32,
    pub root_x: xcb_input_fp1616_t,
    pub root_y: xcb_input_fp1616_t,
    pub event_x: xcb_input_fp1616_t,
    pub event_y: xcb_input_fp1616_t,
    pub buttons_len: u16,
    pub valuators_len: u16,
    pub sourceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
    pub flags: u32,
    pub mods: xcb_input_modifier_info_t,
    pub group: xcb_input_group_info_t,
}

impl Default for xcb_input_button_press_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::ButtonRelease` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Input` extension,
/// then the type of the event is [`xcb_input_button_release_event_t`].
pub const XCB_INPUT_BUTTON_RELEASE: u16 = 5i32 as u16;

/// The `Input::ButtonRelease` event.
pub type xcb_input_button_release_event_t = xcb_input_button_press_event_t;

/// The opcode for `Input::Motion` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Input` extension,
/// then the type of the event is [`xcb_input_motion_event_t`].
pub const XCB_INPUT_MOTION: u16 = 6i32 as u16;

/// The `Input::Motion` event.
pub type xcb_input_motion_event_t = xcb_input_button_press_event_t;

/// The `Input::NotifyMode` enum.
///
/// This enum has the following variants:
///
/// - [`Input::NotifyMode::Normal`](XCB_INPUT_NOTIFY_MODE_NORMAL)
/// - [`Input::NotifyMode::Grab`](XCB_INPUT_NOTIFY_MODE_GRAB)
/// - [`Input::NotifyMode::Ungrab`](XCB_INPUT_NOTIFY_MODE_UNGRAB)
/// - [`Input::NotifyMode::WhileGrabbed`](XCB_INPUT_NOTIFY_MODE_WHILE_GRABBED)
/// - [`Input::NotifyMode::PassiveGrab`](XCB_INPUT_NOTIFY_MODE_PASSIVE_GRAB)
/// - [`Input::NotifyMode::PassiveUngrab`](XCB_INPUT_NOTIFY_MODE_PASSIVE_UNGRAB)
pub type xcb_input_notify_mode_t = u32;
/// The `Input::NotifyMode::Normal` enum variant.
///
/// This is a variant of [`xcb_input_notify_mode_t`].
pub const XCB_INPUT_NOTIFY_MODE_NORMAL: xcb_input_notify_mode_t = 0;
/// The `Input::NotifyMode::Grab` enum variant.
///
/// This is a variant of [`xcb_input_notify_mode_t`].
pub const XCB_INPUT_NOTIFY_MODE_GRAB: xcb_input_notify_mode_t = 1;
/// The `Input::NotifyMode::Ungrab` enum variant.
///
/// This is a variant of [`xcb_input_notify_mode_t`].
pub const XCB_INPUT_NOTIFY_MODE_UNGRAB: xcb_input_notify_mode_t = 2;
/// The `Input::NotifyMode::WhileGrabbed` enum variant.
///
/// This is a variant of [`xcb_input_notify_mode_t`].
pub const XCB_INPUT_NOTIFY_MODE_WHILE_GRABBED: xcb_input_notify_mode_t = 3;
/// The `Input::NotifyMode::PassiveGrab` enum variant.
///
/// This is a variant of [`xcb_input_notify_mode_t`].
pub const XCB_INPUT_NOTIFY_MODE_PASSIVE_GRAB: xcb_input_notify_mode_t = 4;
/// The `Input::NotifyMode::PassiveUngrab` enum variant.
///
/// This is a variant of [`xcb_input_notify_mode_t`].
pub const XCB_INPUT_NOTIFY_MODE_PASSIVE_UNGRAB: xcb_input_notify_mode_t = 5;

/// The `Input::NotifyDetail` enum.
///
/// This enum has the following variants:
///
/// - [`Input::NotifyDetail::Ancestor`](XCB_INPUT_NOTIFY_DETAIL_ANCESTOR)
/// - [`Input::NotifyDetail::Virtual`](XCB_INPUT_NOTIFY_DETAIL_VIRTUAL)
/// - [`Input::NotifyDetail::Inferior`](XCB_INPUT_NOTIFY_DETAIL_INFERIOR)
/// - [`Input::NotifyDetail::Nonlinear`](XCB_INPUT_NOTIFY_DETAIL_NONLINEAR)
/// - [`Input::NotifyDetail::NonlinearVirtual`](XCB_INPUT_NOTIFY_DETAIL_NONLINEAR_VIRTUAL)
/// - [`Input::NotifyDetail::Pointer`](XCB_INPUT_NOTIFY_DETAIL_POINTER)
/// - [`Input::NotifyDetail::PointerRoot`](XCB_INPUT_NOTIFY_DETAIL_POINTER_ROOT)
/// - [`Input::NotifyDetail::None`](XCB_INPUT_NOTIFY_DETAIL_NONE)
pub type xcb_input_notify_detail_t = u32;
/// The `Input::NotifyDetail::Ancestor` enum variant.
///
/// This is a variant of [`xcb_input_notify_detail_t`].
pub const XCB_INPUT_NOTIFY_DETAIL_ANCESTOR: xcb_input_notify_detail_t = 0;
/// The `Input::NotifyDetail::Virtual` enum variant.
///
/// This is a variant of [`xcb_input_notify_detail_t`].
pub const XCB_INPUT_NOTIFY_DETAIL_VIRTUAL: xcb_input_notify_detail_t = 1;
/// The `Input::NotifyDetail::Inferior` enum variant.
///
/// This is a variant of [`xcb_input_notify_detail_t`].
pub const XCB_INPUT_NOTIFY_DETAIL_INFERIOR: xcb_input_notify_detail_t = 2;
/// The `Input::NotifyDetail::Nonlinear` enum variant.
///
/// This is a variant of [`xcb_input_notify_detail_t`].
pub const XCB_INPUT_NOTIFY_DETAIL_NONLINEAR: xcb_input_notify_detail_t = 3;
/// The `Input::NotifyDetail::NonlinearVirtual` enum variant.
///
/// This is a variant of [`xcb_input_notify_detail_t`].
pub const XCB_INPUT_NOTIFY_DETAIL_NONLINEAR_VIRTUAL: xcb_input_notify_detail_t = 4;
/// The `Input::NotifyDetail::Pointer` enum variant.
///
/// This is a variant of [`xcb_input_notify_detail_t`].
pub const XCB_INPUT_NOTIFY_DETAIL_POINTER: xcb_input_notify_detail_t = 5;
/// The `Input::NotifyDetail::PointerRoot` enum variant.
///
/// This is a variant of [`xcb_input_notify_detail_t`].
pub const XCB_INPUT_NOTIFY_DETAIL_POINTER_ROOT: xcb_input_notify_detail_t = 6;
/// The `Input::NotifyDetail::None` enum variant.
///
/// This is a variant of [`xcb_input_notify_detail_t`].
pub const XCB_INPUT_NOTIFY_DETAIL_NONE: xcb_input_notify_detail_t = 7;

/// The opcode for `Input::Enter` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Input` extension,
/// then the type of the event is [`xcb_input_enter_event_t`].
pub const XCB_INPUT_ENTER: u16 = 7i32 as u16;

/// The `Input::Enter` event.
///
/// The following fields can be accessed via accessor functions:
///
/// - `buttons`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_enter_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: xcb_input_device_id_t,
    pub time: xcb_timestamp_t,
    pub sourceid: xcb_input_device_id_t,
    pub mode: u8,
    pub detail: u8,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub full_sequence: u32,
    pub root_x: xcb_input_fp1616_t,
    pub root_y: xcb_input_fp1616_t,
    pub event_x: xcb_input_fp1616_t,
    pub event_y: xcb_input_fp1616_t,
    pub same_screen: u8,
    pub focus: u8,
    pub buttons_len: u16,
    pub mods: xcb_input_modifier_info_t,
    pub group: xcb_input_group_info_t,
}

impl Default for xcb_input_enter_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::Leave` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Input` extension,
/// then the type of the event is [`xcb_input_leave_event_t`].
pub const XCB_INPUT_LEAVE: u16 = 8i32 as u16;

/// The `Input::Leave` event.
pub type xcb_input_leave_event_t = xcb_input_enter_event_t;

/// The opcode for `Input::FocusIn` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Input` extension,
/// then the type of the event is [`xcb_input_focus_in_event_t`].
pub const XCB_INPUT_FOCUS_IN: u16 = 9i32 as u16;

/// The `Input::FocusIn` event.
pub type xcb_input_focus_in_event_t = xcb_input_enter_event_t;

/// The opcode for `Input::FocusOut` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Input` extension,
/// then the type of the event is [`xcb_input_focus_out_event_t`].
pub const XCB_INPUT_FOCUS_OUT: u16 = 10i32 as u16;

/// The `Input::FocusOut` event.
pub type xcb_input_focus_out_event_t = xcb_input_enter_event_t;

/// The `Input::HierarchyMask` enum.
///
/// This enum has the following variants:
///
/// - [`Input::HierarchyMask::MasterAdded`](XCB_INPUT_HIERARCHY_MASK_MASTER_ADDED)
/// - [`Input::HierarchyMask::MasterRemoved`](XCB_INPUT_HIERARCHY_MASK_MASTER_REMOVED)
/// - [`Input::HierarchyMask::SlaveAdded`](XCB_INPUT_HIERARCHY_MASK_SLAVE_ADDED)
/// - [`Input::HierarchyMask::SlaveRemoved`](XCB_INPUT_HIERARCHY_MASK_SLAVE_REMOVED)
/// - [`Input::HierarchyMask::SlaveAttached`](XCB_INPUT_HIERARCHY_MASK_SLAVE_ATTACHED)
/// - [`Input::HierarchyMask::SlaveDetached`](XCB_INPUT_HIERARCHY_MASK_SLAVE_DETACHED)
/// - [`Input::HierarchyMask::DeviceEnabled`](XCB_INPUT_HIERARCHY_MASK_DEVICE_ENABLED)
/// - [`Input::HierarchyMask::DeviceDisabled`](XCB_INPUT_HIERARCHY_MASK_DEVICE_DISABLED)
pub type xcb_input_hierarchy_mask_t = u32;
/// The `Input::HierarchyMask::MasterAdded` enum variant.
///
/// This is a variant of [`xcb_input_hierarchy_mask_t`].
pub const XCB_INPUT_HIERARCHY_MASK_MASTER_ADDED: xcb_input_hierarchy_mask_t = 1;
/// The `Input::HierarchyMask::MasterRemoved` enum variant.
///
/// This is a variant of [`xcb_input_hierarchy_mask_t`].
pub const XCB_INPUT_HIERARCHY_MASK_MASTER_REMOVED: xcb_input_hierarchy_mask_t = 2;
/// The `Input::HierarchyMask::SlaveAdded` enum variant.
///
/// This is a variant of [`xcb_input_hierarchy_mask_t`].
pub const XCB_INPUT_HIERARCHY_MASK_SLAVE_ADDED: xcb_input_hierarchy_mask_t = 4;
/// The `Input::HierarchyMask::SlaveRemoved` enum variant.
///
/// This is a variant of [`xcb_input_hierarchy_mask_t`].
pub const XCB_INPUT_HIERARCHY_MASK_SLAVE_REMOVED: xcb_input_hierarchy_mask_t = 8;
/// The `Input::HierarchyMask::SlaveAttached` enum variant.
///
/// This is a variant of [`xcb_input_hierarchy_mask_t`].
pub const XCB_INPUT_HIERARCHY_MASK_SLAVE_ATTACHED: xcb_input_hierarchy_mask_t = 16;
/// The `Input::HierarchyMask::SlaveDetached` enum variant.
///
/// This is a variant of [`xcb_input_hierarchy_mask_t`].
pub const XCB_INPUT_HIERARCHY_MASK_SLAVE_DETACHED: xcb_input_hierarchy_mask_t = 32;
/// The `Input::HierarchyMask::DeviceEnabled` enum variant.
///
/// This is a variant of [`xcb_input_hierarchy_mask_t`].
pub const XCB_INPUT_HIERARCHY_MASK_DEVICE_ENABLED: xcb_input_hierarchy_mask_t = 64;
/// The `Input::HierarchyMask::DeviceDisabled` enum variant.
///
/// This is a variant of [`xcb_input_hierarchy_mask_t`].
pub const XCB_INPUT_HIERARCHY_MASK_DEVICE_DISABLED: xcb_input_hierarchy_mask_t = 128;

/// The `Input::HierarchyInfo` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_hierarchy_info_t {
    pub deviceid: xcb_input_device_id_t,
    pub attachment: xcb_input_device_id_t,
    pub type_: u8,
    pub enabled: u8,
    pub pad0: [u8; 2],
    pub flags: u32,
}

impl Default for xcb_input_hierarchy_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::HierarchyInfo` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_hierarchy_info_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_hierarchy_info_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_hierarchy_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::Hierarchy` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Input` extension,
/// then the type of the event is [`xcb_input_hierarchy_event_t`].
pub const XCB_INPUT_HIERARCHY: u16 = 11i32 as u16;

/// The `Input::Hierarchy` event.
///
/// The following fields can be accessed via accessor functions:
///
/// - `infos`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_hierarchy_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: xcb_input_device_id_t,
    pub time: xcb_timestamp_t,
    pub flags: u32,
    pub num_infos: u16,
    pub pad0: [u8; 10],
    pub full_sequence: u32,
}

impl Default for xcb_input_hierarchy_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `Input::PropertyFlag` enum.
///
/// This enum has the following variants:
///
/// - [`Input::PropertyFlag::Deleted`](XCB_INPUT_PROPERTY_FLAG_DELETED)
/// - [`Input::PropertyFlag::Created`](XCB_INPUT_PROPERTY_FLAG_CREATED)
/// - [`Input::PropertyFlag::Modified`](XCB_INPUT_PROPERTY_FLAG_MODIFIED)
pub type xcb_input_property_flag_t = u32;
/// The `Input::PropertyFlag::Deleted` enum variant.
///
/// This is a variant of [`xcb_input_property_flag_t`].
pub const XCB_INPUT_PROPERTY_FLAG_DELETED: xcb_input_property_flag_t = 0;
/// The `Input::PropertyFlag::Created` enum variant.
///
/// This is a variant of [`xcb_input_property_flag_t`].
pub const XCB_INPUT_PROPERTY_FLAG_CREATED: xcb_input_property_flag_t = 1;
/// The `Input::PropertyFlag::Modified` enum variant.
///
/// This is a variant of [`xcb_input_property_flag_t`].
pub const XCB_INPUT_PROPERTY_FLAG_MODIFIED: xcb_input_property_flag_t = 2;

/// The opcode for `Input::Property` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Input` extension,
/// then the type of the event is [`xcb_input_property_event_t`].
pub const XCB_INPUT_PROPERTY: u16 = 12i32 as u16;

/// The `Input::Property` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_property_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: xcb_input_device_id_t,
    pub time: xcb_timestamp_t,
    pub property: xcb_atom_t,
    pub what: u8,
    pub pad0: [u8; 11],
    pub full_sequence: u32,
}

impl Default for xcb_input_property_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::RawKeyPress` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Input` extension,
/// then the type of the event is [`xcb_input_raw_key_press_event_t`].
pub const XCB_INPUT_RAW_KEY_PRESS: u16 = 13i32 as u16;

/// The `Input::RawKeyPress` event.
///
/// The following fields can be accessed via accessor functions:
///
/// - `valuator_mask`
/// - `axisvalues`
/// - `axisvalues_raw`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_raw_key_press_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: xcb_input_device_id_t,
    pub time: xcb_timestamp_t,
    pub detail: u32,
    pub sourceid: xcb_input_device_id_t,
    pub valuators_len: u16,
    pub flags: u32,
    pub pad0: [u8; 4],
    pub full_sequence: u32,
}

impl Default for xcb_input_raw_key_press_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::RawKeyRelease` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Input` extension,
/// then the type of the event is [`xcb_input_raw_key_release_event_t`].
pub const XCB_INPUT_RAW_KEY_RELEASE: u16 = 14i32 as u16;

/// The `Input::RawKeyRelease` event.
pub type xcb_input_raw_key_release_event_t = xcb_input_raw_key_press_event_t;

/// The opcode for `Input::RawButtonPress` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Input` extension,
/// then the type of the event is [`xcb_input_raw_button_press_event_t`].
pub const XCB_INPUT_RAW_BUTTON_PRESS: u16 = 15i32 as u16;

/// The `Input::RawButtonPress` event.
///
/// The following fields can be accessed via accessor functions:
///
/// - `valuator_mask`
/// - `axisvalues`
/// - `axisvalues_raw`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_raw_button_press_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: xcb_input_device_id_t,
    pub time: xcb_timestamp_t,
    pub detail: u32,
    pub sourceid: xcb_input_device_id_t,
    pub valuators_len: u16,
    pub flags: u32,
    pub pad0: [u8; 4],
    pub full_sequence: u32,
}

impl Default for xcb_input_raw_button_press_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::RawButtonRelease` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Input` extension,
/// then the type of the event is [`xcb_input_raw_button_release_event_t`].
pub const XCB_INPUT_RAW_BUTTON_RELEASE: u16 = 16i32 as u16;

/// The `Input::RawButtonRelease` event.
pub type xcb_input_raw_button_release_event_t = xcb_input_raw_button_press_event_t;

/// The opcode for `Input::RawMotion` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Input` extension,
/// then the type of the event is [`xcb_input_raw_motion_event_t`].
pub const XCB_INPUT_RAW_MOTION: u16 = 17i32 as u16;

/// The `Input::RawMotion` event.
pub type xcb_input_raw_motion_event_t = xcb_input_raw_button_press_event_t;

/// The `Input::TouchEventFlags` enum.
///
/// This enum has the following variants:
///
/// - [`Input::TouchEventFlags::TouchPendingEnd`](XCB_INPUT_TOUCH_EVENT_FLAGS_TOUCH_PENDING_END)
/// - [`Input::TouchEventFlags::TouchEmulatingPointer`](XCB_INPUT_TOUCH_EVENT_FLAGS_TOUCH_EMULATING_POINTER)
pub type xcb_input_touch_event_flags_t = u32;
/// The `Input::TouchEventFlags::TouchPendingEnd` enum variant.
///
/// This is a variant of [`xcb_input_touch_event_flags_t`].
pub const XCB_INPUT_TOUCH_EVENT_FLAGS_TOUCH_PENDING_END: xcb_input_touch_event_flags_t = 65536;
/// The `Input::TouchEventFlags::TouchEmulatingPointer` enum variant.
///
/// This is a variant of [`xcb_input_touch_event_flags_t`].
pub const XCB_INPUT_TOUCH_EVENT_FLAGS_TOUCH_EMULATING_POINTER: xcb_input_touch_event_flags_t =
    131072;

/// The opcode for `Input::TouchBegin` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Input` extension,
/// then the type of the event is [`xcb_input_touch_begin_event_t`].
pub const XCB_INPUT_TOUCH_BEGIN: u16 = 18i32 as u16;

/// The `Input::TouchBegin` event.
///
/// The following fields can be accessed via accessor functions:
///
/// - `button_mask`
/// - `valuator_mask`
/// - `axisvalues`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_touch_begin_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: xcb_input_device_id_t,
    pub time: xcb_timestamp_t,
    pub detail: u32,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub full_sequence: u32,
    pub root_x: xcb_input_fp1616_t,
    pub root_y: xcb_input_fp1616_t,
    pub event_x: xcb_input_fp1616_t,
    pub event_y: xcb_input_fp1616_t,
    pub buttons_len: u16,
    pub valuators_len: u16,
    pub sourceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
    pub flags: u32,
    pub mods: xcb_input_modifier_info_t,
    pub group: xcb_input_group_info_t,
}

impl Default for xcb_input_touch_begin_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::TouchUpdate` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Input` extension,
/// then the type of the event is [`xcb_input_touch_update_event_t`].
pub const XCB_INPUT_TOUCH_UPDATE: u16 = 19i32 as u16;

/// The `Input::TouchUpdate` event.
pub type xcb_input_touch_update_event_t = xcb_input_touch_begin_event_t;

/// The opcode for `Input::TouchEnd` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Input` extension,
/// then the type of the event is [`xcb_input_touch_end_event_t`].
pub const XCB_INPUT_TOUCH_END: u16 = 20i32 as u16;

/// The `Input::TouchEnd` event.
pub type xcb_input_touch_end_event_t = xcb_input_touch_begin_event_t;

/// The `Input::TouchOwnershipFlags` enum.
///
/// This enum has the following variants:
///
/// - [`Input::TouchOwnershipFlags::None`](XCB_INPUT_TOUCH_OWNERSHIP_FLAGS_NONE)
pub type xcb_input_touch_ownership_flags_t = u32;
/// The `Input::TouchOwnershipFlags::None` enum variant.
///
/// This is a variant of [`xcb_input_touch_ownership_flags_t`].
pub const XCB_INPUT_TOUCH_OWNERSHIP_FLAGS_NONE: xcb_input_touch_ownership_flags_t = 0;

/// The opcode for `Input::TouchOwnership` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Input` extension,
/// then the type of the event is [`xcb_input_touch_ownership_event_t`].
pub const XCB_INPUT_TOUCH_OWNERSHIP: u16 = 21i32 as u16;

/// The `Input::TouchOwnership` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_touch_ownership_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: xcb_input_device_id_t,
    pub time: xcb_timestamp_t,
    pub touchid: u32,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub full_sequence: u32,
    pub sourceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
    pub flags: u32,
    pub pad1: [u8; 8],
}

impl Default for xcb_input_touch_ownership_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::RawTouchBegin` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Input` extension,
/// then the type of the event is [`xcb_input_raw_touch_begin_event_t`].
pub const XCB_INPUT_RAW_TOUCH_BEGIN: u16 = 22i32 as u16;

/// The `Input::RawTouchBegin` event.
///
/// The following fields can be accessed via accessor functions:
///
/// - `valuator_mask`
/// - `axisvalues`
/// - `axisvalues_raw`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_raw_touch_begin_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: xcb_input_device_id_t,
    pub time: xcb_timestamp_t,
    pub detail: u32,
    pub sourceid: xcb_input_device_id_t,
    pub valuators_len: u16,
    pub flags: u32,
    pub pad0: [u8; 4],
    pub full_sequence: u32,
}

impl Default for xcb_input_raw_touch_begin_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::RawTouchUpdate` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Input` extension,
/// then the type of the event is [`xcb_input_raw_touch_update_event_t`].
pub const XCB_INPUT_RAW_TOUCH_UPDATE: u16 = 23i32 as u16;

/// The `Input::RawTouchUpdate` event.
pub type xcb_input_raw_touch_update_event_t = xcb_input_raw_touch_begin_event_t;

/// The opcode for `Input::RawTouchEnd` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Input` extension,
/// then the type of the event is [`xcb_input_raw_touch_end_event_t`].
pub const XCB_INPUT_RAW_TOUCH_END: u16 = 24i32 as u16;

/// The `Input::RawTouchEnd` event.
pub type xcb_input_raw_touch_end_event_t = xcb_input_raw_touch_begin_event_t;

/// The `Input::BarrierFlags` enum.
///
/// This enum has the following variants:
///
/// - [`Input::BarrierFlags::PointerReleased`](XCB_INPUT_BARRIER_FLAGS_POINTER_RELEASED)
/// - [`Input::BarrierFlags::DeviceIsGrabbed`](XCB_INPUT_BARRIER_FLAGS_DEVICE_IS_GRABBED)
pub type xcb_input_barrier_flags_t = u32;
/// The `Input::BarrierFlags::PointerReleased` enum variant.
///
/// This is a variant of [`xcb_input_barrier_flags_t`].
pub const XCB_INPUT_BARRIER_FLAGS_POINTER_RELEASED: xcb_input_barrier_flags_t = 1;
/// The `Input::BarrierFlags::DeviceIsGrabbed` enum variant.
///
/// This is a variant of [`xcb_input_barrier_flags_t`].
pub const XCB_INPUT_BARRIER_FLAGS_DEVICE_IS_GRABBED: xcb_input_barrier_flags_t = 2;

/// The opcode for `Input::BarrierHit` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Input` extension,
/// then the type of the event is [`xcb_input_barrier_hit_event_t`].
pub const XCB_INPUT_BARRIER_HIT: u16 = 25i32 as u16;

/// The `Input::BarrierHit` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_barrier_hit_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub deviceid: xcb_input_device_id_t,
    pub time: xcb_timestamp_t,
    pub eventid: u32,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub barrier: xcb_xfixes_barrier_t,
    pub full_sequence: u32,
    pub dtime: u32,
    pub flags: u32,
    pub sourceid: xcb_input_device_id_t,
    pub pad0: [u8; 2],
    pub root_x: xcb_input_fp1616_t,
    pub root_y: xcb_input_fp1616_t,
    pub dx: xcb_input_fp3232_t,
    pub dy: xcb_input_fp3232_t,
}

impl Default for xcb_input_barrier_hit_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::BarrierLeave` events.
///
/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and
/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `Input` extension,
/// then the type of the event is [`xcb_input_barrier_leave_event_t`].
pub const XCB_INPUT_BARRIER_LEAVE: u16 = 26i32 as u16;

/// The `Input::BarrierLeave` event.
pub type xcb_input_barrier_leave_event_t = xcb_input_barrier_hit_event_t;

/// The `Input::EventForSend` eventstruct.
#[derive(Copy, Clone)]
#[repr(C)]
pub union xcb_input_event_for_send_t {
    pub device_valuator: xcb_input_device_valuator_event_t,
    pub device_key_press: xcb_input_device_key_press_event_t,
    pub device_key_release: xcb_input_device_key_release_event_t,
    pub device_button_press: xcb_input_device_button_press_event_t,
    pub device_button_release: xcb_input_device_button_release_event_t,
    pub device_motion_notify: xcb_input_device_motion_notify_event_t,
    pub device_focus_in: xcb_input_device_focus_in_event_t,
    pub device_focus_out: xcb_input_device_focus_out_event_t,
    pub proximity_in: xcb_input_proximity_in_event_t,
    pub proximity_out: xcb_input_proximity_out_event_t,
    pub device_state_notify: xcb_input_device_state_notify_event_t,
    pub device_mapping_notify: xcb_input_device_mapping_notify_event_t,
    pub change_device_notify: xcb_input_change_device_notify_event_t,
    pub device_key_state_notify: xcb_input_device_key_state_notify_event_t,
    pub device_button_state_notify: xcb_input_device_button_state_notify_event_t,
    pub device_presence_notify: xcb_input_device_presence_notify_event_t,
    pub event_header: xcb_raw_generic_event_t,
}

impl Default for xcb_input_event_for_send_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `Input::EventForSend` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_event_for_send_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_input_event_for_send_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_input_event_for_send_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::SendExtensionEvent` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXinput::xcb_input_id()`], then the type of the request is
/// [`xcb_input_send_extension_event_request_t`].
pub const XCB_INPUT_SEND_EXTENSION_EVENT: u8 = 31i32 as u8;

/// The `Input::SendExtensionEvent` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `events`
/// - `classes`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_send_extension_event_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub destination: xcb_window_t,
    pub device_id: u8,
    pub propagate: u8,
    pub num_classes: u16,
    pub num_events: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_input_send_extension_event_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::Device` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_input_device_error_t`].
pub const XCB_INPUT_DEVICE: u8 = 0i32 as u8;

/// The `Input::Device` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

impl Default for xcb_input_device_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::Event` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_input_event_error_t`].
pub const XCB_INPUT_EVENT: u8 = 1i32 as u8;

/// The `Input::Event` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_event_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

impl Default for xcb_input_event_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::Mode` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_input_mode_error_t`].
pub const XCB_INPUT_MODE: u8 = 2i32 as u8;

/// The `Input::Mode` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_mode_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

impl Default for xcb_input_mode_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::DeviceBusy` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_input_device_busy_error_t`].
pub const XCB_INPUT_DEVICE_BUSY: u8 = 3i32 as u8;

/// The `Input::DeviceBusy` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_device_busy_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

impl Default for xcb_input_device_busy_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `Input::Class` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_input_class_error_t`].
pub const XCB_INPUT_CLASS: u8 = 4i32 as u8;

/// The `Input::Class` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_input_class_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

impl Default for xcb_input_class_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[cfg(feature = "xcb_xinput")]
pub(crate) struct XcbXinputXinput {
    xcb_input_id: LazySymbol<*mut xcb_extension_t>,
    xcb_input_event_class_next: LazySymbol<unsafe fn(i: *mut xcb_input_event_class_iterator_t)>,
    xcb_input_event_class_end:
        LazySymbol<unsafe fn(i: xcb_input_event_class_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_key_code_next: LazySymbol<unsafe fn(i: *mut xcb_input_key_code_iterator_t)>,
    xcb_input_key_code_end:
        LazySymbol<unsafe fn(i: xcb_input_key_code_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_device_id_next: LazySymbol<unsafe fn(i: *mut xcb_input_device_id_iterator_t)>,
    xcb_input_device_id_end:
        LazySymbol<unsafe fn(i: xcb_input_device_id_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_fp1616_next: LazySymbol<unsafe fn(i: *mut xcb_input_fp1616_iterator_t)>,
    xcb_input_fp1616_end:
        LazySymbol<unsafe fn(i: xcb_input_fp1616_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_fp3232_next: LazySymbol<unsafe fn(i: *mut xcb_input_fp3232_iterator_t)>,
    xcb_input_fp3232_end:
        LazySymbol<unsafe fn(i: xcb_input_fp3232_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_get_extension_version_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_get_extension_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            name_len: u16,
            name: *const c_char,
        ) -> xcb_input_get_extension_version_cookie_t,
    >,
    xcb_input_get_extension_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            name_len: u16,
            name: *const c_char,
        ) -> xcb_input_get_extension_version_cookie_t,
    >,
    xcb_input_get_extension_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_get_extension_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_get_extension_version_reply_t,
    >,
    xcb_input_device_info_next: LazySymbol<unsafe fn(i: *mut xcb_input_device_info_iterator_t)>,
    xcb_input_device_info_end:
        LazySymbol<unsafe fn(i: xcb_input_device_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_key_info_next: LazySymbol<unsafe fn(i: *mut xcb_input_key_info_iterator_t)>,
    xcb_input_key_info_end:
        LazySymbol<unsafe fn(i: xcb_input_key_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_button_info_next: LazySymbol<unsafe fn(i: *mut xcb_input_button_info_iterator_t)>,
    xcb_input_button_info_end:
        LazySymbol<unsafe fn(i: xcb_input_button_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_axis_info_next: LazySymbol<unsafe fn(i: *mut xcb_input_axis_info_iterator_t)>,
    xcb_input_axis_info_end:
        LazySymbol<unsafe fn(i: xcb_input_axis_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_valuator_info_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_valuator_info_axes:
        LazySymbol<unsafe fn(r: *const xcb_input_valuator_info_t) -> *mut xcb_input_axis_info_t>,
    xcb_input_valuator_info_axes_length:
        LazySymbol<unsafe fn(r: *const xcb_input_valuator_info_t) -> c_int>,
    xcb_input_valuator_info_axes_iterator: LazySymbol<
        unsafe fn(r: *const xcb_input_valuator_info_t) -> xcb_input_axis_info_iterator_t,
    >,
    xcb_input_valuator_info_next: LazySymbol<unsafe fn(i: *mut xcb_input_valuator_info_iterator_t)>,
    xcb_input_valuator_info_end:
        LazySymbol<unsafe fn(i: xcb_input_valuator_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_input_info_info_valuator_axes:
        LazySymbol<unsafe fn(s: *const xcb_input_input_info_info_t) -> *mut xcb_input_axis_info_t>,
    xcb_input_input_info_info_valuator_axes_length: LazySymbol<
        unsafe fn(r: *const xcb_input_input_info_t, s: *const xcb_input_input_info_info_t) -> c_int,
    >,
    xcb_input_input_info_info_valuator_axes_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_input_input_info_t,
            s: *const xcb_input_input_info_info_t,
        ) -> xcb_input_axis_info_iterator_t,
    >,
    xcb_input_input_info_info_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            class_id: u8,
            _aux: *const xcb_input_input_info_info_t,
        ) -> c_int,
    >,
    xcb_input_input_info_info_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            class_id: u8,
            _aux: *mut xcb_input_input_info_info_t,
        ) -> c_int,
    >,
    xcb_input_input_info_info_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, class_id: u8) -> c_int>,
    xcb_input_input_info_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_input_info_info:
        LazySymbol<unsafe fn(r: *const xcb_input_input_info_t) -> *mut c_void>,
    xcb_input_input_info_next: LazySymbol<unsafe fn(i: *mut xcb_input_input_info_iterator_t)>,
    xcb_input_input_info_end:
        LazySymbol<unsafe fn(i: xcb_input_input_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_device_name_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_device_name_string:
        LazySymbol<unsafe fn(r: *const xcb_input_device_name_t) -> *mut c_char>,
    xcb_input_device_name_string_length:
        LazySymbol<unsafe fn(r: *const xcb_input_device_name_t) -> c_int>,
    xcb_input_device_name_string_end:
        LazySymbol<unsafe fn(r: *const xcb_input_device_name_t) -> xcb_generic_iterator_t>,
    xcb_input_device_name_next: LazySymbol<unsafe fn(i: *mut xcb_input_device_name_iterator_t)>,
    xcb_input_device_name_end:
        LazySymbol<unsafe fn(i: xcb_input_device_name_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_list_input_devices_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_list_input_devices:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_input_list_input_devices_cookie_t>,
    xcb_input_list_input_devices_unchecked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t) -> xcb_input_list_input_devices_cookie_t>,
    xcb_input_list_input_devices_devices: LazySymbol<
        unsafe fn(r: *const xcb_input_list_input_devices_reply_t) -> *mut xcb_input_device_info_t,
    >,
    xcb_input_list_input_devices_devices_length:
        LazySymbol<unsafe fn(r: *const xcb_input_list_input_devices_reply_t) -> c_int>,
    xcb_input_list_input_devices_devices_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_input_list_input_devices_reply_t,
        ) -> xcb_input_device_info_iterator_t,
    >,
    xcb_input_list_input_devices_infos_length:
        LazySymbol<unsafe fn(r: *const xcb_input_list_input_devices_reply_t) -> c_int>,
    xcb_input_list_input_devices_infos_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_input_list_input_devices_reply_t,
        ) -> xcb_input_input_info_iterator_t,
    >,
    xcb_input_list_input_devices_names_length:
        LazySymbol<unsafe fn(r: *const xcb_input_list_input_devices_reply_t) -> c_int>,
    xcb_input_list_input_devices_names_iterator:
        LazySymbol<unsafe fn(r: *const xcb_input_list_input_devices_reply_t) -> xcb_str_iterator_t>,
    xcb_input_list_input_devices_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_list_input_devices_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_list_input_devices_reply_t,
    >,
    xcb_input_event_type_base_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_event_type_base_iterator_t)>,
    xcb_input_event_type_base_end:
        LazySymbol<unsafe fn(i: xcb_input_event_type_base_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_input_class_info_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_input_class_info_iterator_t)>,
    xcb_input_input_class_info_end:
        LazySymbol<unsafe fn(i: xcb_input_input_class_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_open_device_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_open_device: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, device_id: u8) -> xcb_input_open_device_cookie_t,
    >,
    xcb_input_open_device_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, device_id: u8) -> xcb_input_open_device_cookie_t,
    >,
    xcb_input_open_device_class_info: LazySymbol<
        unsafe fn(r: *const xcb_input_open_device_reply_t) -> *mut xcb_input_input_class_info_t,
    >,
    xcb_input_open_device_class_info_length:
        LazySymbol<unsafe fn(r: *const xcb_input_open_device_reply_t) -> c_int>,
    xcb_input_open_device_class_info_iterator: LazySymbol<
        unsafe fn(r: *const xcb_input_open_device_reply_t) -> xcb_input_input_class_info_iterator_t,
    >,
    xcb_input_open_device_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_open_device_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_open_device_reply_t,
    >,
    xcb_input_close_device_checked:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, device_id: u8) -> xcb_void_cookie_t>,
    xcb_input_close_device:
        LazySymbol<unsafe fn(c: *mut xcb_connection_t, device_id: u8) -> xcb_void_cookie_t>,
    xcb_input_set_device_mode: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
            mode: u8,
        ) -> xcb_input_set_device_mode_cookie_t,
    >,
    xcb_input_set_device_mode_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
            mode: u8,
        ) -> xcb_input_set_device_mode_cookie_t,
    >,
    xcb_input_set_device_mode_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_set_device_mode_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_set_device_mode_reply_t,
    >,
    xcb_input_select_extension_event_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_select_extension_event_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            num_classes: u16,
            classes: *const xcb_input_event_class_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_select_extension_event: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            num_classes: u16,
            classes: *const xcb_input_event_class_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_select_extension_event_classes: LazySymbol<
        unsafe fn(
            r: *const xcb_input_select_extension_event_request_t,
        ) -> *mut xcb_input_event_class_t,
    >,
    xcb_input_select_extension_event_classes_length:
        LazySymbol<unsafe fn(r: *const xcb_input_select_extension_event_request_t) -> c_int>,
    xcb_input_select_extension_event_classes_end: LazySymbol<
        unsafe fn(r: *const xcb_input_select_extension_event_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_get_selected_extension_events_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_get_selected_extension_events: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_input_get_selected_extension_events_cookie_t,
    >,
    xcb_input_get_selected_extension_events_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_input_get_selected_extension_events_cookie_t,
    >,
    xcb_input_get_selected_extension_events_this_classes: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_selected_extension_events_reply_t,
        ) -> *mut xcb_input_event_class_t,
    >,
    xcb_input_get_selected_extension_events_this_classes_length:
        LazySymbol<unsafe fn(r: *const xcb_input_get_selected_extension_events_reply_t) -> c_int>,
    xcb_input_get_selected_extension_events_this_classes_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_selected_extension_events_reply_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_get_selected_extension_events_all_classes: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_selected_extension_events_reply_t,
        ) -> *mut xcb_input_event_class_t,
    >,
    xcb_input_get_selected_extension_events_all_classes_length:
        LazySymbol<unsafe fn(r: *const xcb_input_get_selected_extension_events_reply_t) -> c_int>,
    xcb_input_get_selected_extension_events_all_classes_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_selected_extension_events_reply_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_get_selected_extension_events_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_get_selected_extension_events_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_get_selected_extension_events_reply_t,
    >,
    xcb_input_change_device_dont_propagate_list_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_change_device_dont_propagate_list_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            num_classes: u16,
            mode: u8,
            classes: *const xcb_input_event_class_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_change_device_dont_propagate_list: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            num_classes: u16,
            mode: u8,
            classes: *const xcb_input_event_class_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_change_device_dont_propagate_list_classes: LazySymbol<
        unsafe fn(
            r: *const xcb_input_change_device_dont_propagate_list_request_t,
        ) -> *mut xcb_input_event_class_t,
    >,
    xcb_input_change_device_dont_propagate_list_classes_length: LazySymbol<
        unsafe fn(r: *const xcb_input_change_device_dont_propagate_list_request_t) -> c_int,
    >,
    xcb_input_change_device_dont_propagate_list_classes_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_change_device_dont_propagate_list_request_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_get_device_dont_propagate_list_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_get_device_dont_propagate_list: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_input_get_device_dont_propagate_list_cookie_t,
    >,
    xcb_input_get_device_dont_propagate_list_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_input_get_device_dont_propagate_list_cookie_t,
    >,
    xcb_input_get_device_dont_propagate_list_classes: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_device_dont_propagate_list_reply_t,
        ) -> *mut xcb_input_event_class_t,
    >,
    xcb_input_get_device_dont_propagate_list_classes_length:
        LazySymbol<unsafe fn(r: *const xcb_input_get_device_dont_propagate_list_reply_t) -> c_int>,
    xcb_input_get_device_dont_propagate_list_classes_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_device_dont_propagate_list_reply_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_get_device_dont_propagate_list_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_get_device_dont_propagate_list_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_get_device_dont_propagate_list_reply_t,
    >,
    xcb_input_device_time_coord_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, num_axes: u8) -> c_int>,
    xcb_input_device_time_coord_axisvalues:
        LazySymbol<unsafe fn(r: *const xcb_input_device_time_coord_t) -> *mut i32>,
    xcb_input_device_time_coord_axisvalues_length:
        LazySymbol<unsafe fn(r: *const xcb_input_device_time_coord_t, num_axes: u8) -> c_int>,
    xcb_input_device_time_coord_axisvalues_end: LazySymbol<
        unsafe fn(r: *const xcb_input_device_time_coord_t, num_axes: u8) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_time_coord_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_device_time_coord_iterator_t)>,
    xcb_input_device_time_coord_end:
        LazySymbol<unsafe fn(i: xcb_input_device_time_coord_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_get_device_motion_events_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_get_device_motion_events: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            start: xcb_timestamp_t,
            stop: xcb_timestamp_t,
            device_id: u8,
        ) -> xcb_input_get_device_motion_events_cookie_t,
    >,
    xcb_input_get_device_motion_events_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            start: xcb_timestamp_t,
            stop: xcb_timestamp_t,
            device_id: u8,
        ) -> xcb_input_get_device_motion_events_cookie_t,
    >,
    xcb_input_get_device_motion_events_events_length:
        LazySymbol<unsafe fn(r: *const xcb_input_get_device_motion_events_reply_t) -> c_int>,
    xcb_input_get_device_motion_events_events_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_device_motion_events_reply_t,
        ) -> xcb_input_device_time_coord_iterator_t,
    >,
    xcb_input_get_device_motion_events_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_get_device_motion_events_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_get_device_motion_events_reply_t,
    >,
    xcb_input_change_keyboard_device: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
        ) -> xcb_input_change_keyboard_device_cookie_t,
    >,
    xcb_input_change_keyboard_device_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
        ) -> xcb_input_change_keyboard_device_cookie_t,
    >,
    xcb_input_change_keyboard_device_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_change_keyboard_device_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_change_keyboard_device_reply_t,
    >,
    xcb_input_change_pointer_device: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            x_axis: u8,
            y_axis: u8,
            device_id: u8,
        ) -> xcb_input_change_pointer_device_cookie_t,
    >,
    xcb_input_change_pointer_device_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            x_axis: u8,
            y_axis: u8,
            device_id: u8,
        ) -> xcb_input_change_pointer_device_cookie_t,
    >,
    xcb_input_change_pointer_device_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_change_pointer_device_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_change_pointer_device_reply_t,
    >,
    xcb_input_grab_device_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_grab_device: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            grab_window: xcb_window_t,
            time: xcb_timestamp_t,
            num_classes: u16,
            this_device_mode: u8,
            other_device_mode: u8,
            owner_events: u8,
            device_id: u8,
            classes: *const xcb_input_event_class_t,
        ) -> xcb_input_grab_device_cookie_t,
    >,
    xcb_input_grab_device_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            grab_window: xcb_window_t,
            time: xcb_timestamp_t,
            num_classes: u16,
            this_device_mode: u8,
            other_device_mode: u8,
            owner_events: u8,
            device_id: u8,
            classes: *const xcb_input_event_class_t,
        ) -> xcb_input_grab_device_cookie_t,
    >,
    xcb_input_grab_device_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_grab_device_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_grab_device_reply_t,
    >,
    xcb_input_ungrab_device_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            time: xcb_timestamp_t,
            device_id: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_ungrab_device: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            time: xcb_timestamp_t,
            device_id: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_grab_device_key_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_grab_device_key_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            grab_window: xcb_window_t,
            num_classes: u16,
            modifiers: u16,
            modifier_device: u8,
            grabbed_device: u8,
            key: u8,
            this_device_mode: u8,
            other_device_mode: u8,
            owner_events: u8,
            classes: *const xcb_input_event_class_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_grab_device_key: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            grab_window: xcb_window_t,
            num_classes: u16,
            modifiers: u16,
            modifier_device: u8,
            grabbed_device: u8,
            key: u8,
            this_device_mode: u8,
            other_device_mode: u8,
            owner_events: u8,
            classes: *const xcb_input_event_class_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_grab_device_key_classes: LazySymbol<
        unsafe fn(r: *const xcb_input_grab_device_key_request_t) -> *mut xcb_input_event_class_t,
    >,
    xcb_input_grab_device_key_classes_length:
        LazySymbol<unsafe fn(r: *const xcb_input_grab_device_key_request_t) -> c_int>,
    xcb_input_grab_device_key_classes_end: LazySymbol<
        unsafe fn(r: *const xcb_input_grab_device_key_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_ungrab_device_key_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            grab_window: xcb_window_t,
            modifiers: u16,
            modifier_device: u8,
            key: u8,
            grabbed_device: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_ungrab_device_key: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            grab_window: xcb_window_t,
            modifiers: u16,
            modifier_device: u8,
            key: u8,
            grabbed_device: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_grab_device_button_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_grab_device_button_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            grab_window: xcb_window_t,
            grabbed_device: u8,
            modifier_device: u8,
            num_classes: u16,
            modifiers: u16,
            this_device_mode: u8,
            other_device_mode: u8,
            button: u8,
            owner_events: u8,
            classes: *const xcb_input_event_class_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_grab_device_button: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            grab_window: xcb_window_t,
            grabbed_device: u8,
            modifier_device: u8,
            num_classes: u16,
            modifiers: u16,
            this_device_mode: u8,
            other_device_mode: u8,
            button: u8,
            owner_events: u8,
            classes: *const xcb_input_event_class_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_grab_device_button_classes: LazySymbol<
        unsafe fn(r: *const xcb_input_grab_device_button_request_t) -> *mut xcb_input_event_class_t,
    >,
    xcb_input_grab_device_button_classes_length:
        LazySymbol<unsafe fn(r: *const xcb_input_grab_device_button_request_t) -> c_int>,
    xcb_input_grab_device_button_classes_end: LazySymbol<
        unsafe fn(r: *const xcb_input_grab_device_button_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_ungrab_device_button_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            grab_window: xcb_window_t,
            modifiers: u16,
            modifier_device: u8,
            button: u8,
            grabbed_device: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_ungrab_device_button: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            grab_window: xcb_window_t,
            modifiers: u16,
            modifier_device: u8,
            button: u8,
            grabbed_device: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_allow_device_events_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            time: xcb_timestamp_t,
            mode: u8,
            device_id: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_allow_device_events: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            time: xcb_timestamp_t,
            mode: u8,
            device_id: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_get_device_focus: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, device_id: u8) -> xcb_input_get_device_focus_cookie_t,
    >,
    xcb_input_get_device_focus_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, device_id: u8) -> xcb_input_get_device_focus_cookie_t,
    >,
    xcb_input_get_device_focus_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_get_device_focus_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_get_device_focus_reply_t,
    >,
    xcb_input_set_device_focus_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            focus: xcb_window_t,
            time: xcb_timestamp_t,
            revert_to: u8,
            device_id: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_set_device_focus: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            focus: xcb_window_t,
            time: xcb_timestamp_t,
            revert_to: u8,
            device_id: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_kbd_feedback_state_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_kbd_feedback_state_iterator_t)>,
    xcb_input_kbd_feedback_state_end:
        LazySymbol<unsafe fn(i: xcb_input_kbd_feedback_state_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_ptr_feedback_state_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_ptr_feedback_state_iterator_t)>,
    xcb_input_ptr_feedback_state_end:
        LazySymbol<unsafe fn(i: xcb_input_ptr_feedback_state_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_integer_feedback_state_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_integer_feedback_state_iterator_t)>,
    xcb_input_integer_feedback_state_end: LazySymbol<
        unsafe fn(i: xcb_input_integer_feedback_state_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_string_feedback_state_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_string_feedback_state_keysyms:
        LazySymbol<unsafe fn(r: *const xcb_input_string_feedback_state_t) -> *mut xcb_keysym_t>,
    xcb_input_string_feedback_state_keysyms_length:
        LazySymbol<unsafe fn(r: *const xcb_input_string_feedback_state_t) -> c_int>,
    xcb_input_string_feedback_state_keysyms_end: LazySymbol<
        unsafe fn(r: *const xcb_input_string_feedback_state_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_string_feedback_state_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_string_feedback_state_iterator_t)>,
    xcb_input_string_feedback_state_end: LazySymbol<
        unsafe fn(i: xcb_input_string_feedback_state_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_bell_feedback_state_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_bell_feedback_state_iterator_t)>,
    xcb_input_bell_feedback_state_end: LazySymbol<
        unsafe fn(i: xcb_input_bell_feedback_state_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_led_feedback_state_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_led_feedback_state_iterator_t)>,
    xcb_input_led_feedback_state_end:
        LazySymbol<unsafe fn(i: xcb_input_led_feedback_state_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_feedback_state_data_string_keysyms:
        LazySymbol<unsafe fn(s: *const xcb_input_feedback_state_data_t) -> *mut xcb_keysym_t>,
    xcb_input_feedback_state_data_string_keysyms_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_feedback_state_t,
            s: *const xcb_input_feedback_state_data_t,
        ) -> c_int,
    >,
    xcb_input_feedback_state_data_string_keysyms_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_feedback_state_t,
            s: *const xcb_input_feedback_state_data_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_feedback_state_data_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            class_id: u8,
            _aux: *const xcb_input_feedback_state_data_t,
        ) -> c_int,
    >,
    xcb_input_feedback_state_data_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            class_id: u8,
            _aux: *mut xcb_input_feedback_state_data_t,
        ) -> c_int,
    >,
    xcb_input_feedback_state_data_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, class_id: u8) -> c_int>,
    xcb_input_feedback_state_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_feedback_state_data:
        LazySymbol<unsafe fn(r: *const xcb_input_feedback_state_t) -> *mut c_void>,
    xcb_input_feedback_state_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_feedback_state_iterator_t)>,
    xcb_input_feedback_state_end:
        LazySymbol<unsafe fn(i: xcb_input_feedback_state_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_get_feedback_control_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_get_feedback_control: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
        ) -> xcb_input_get_feedback_control_cookie_t,
    >,
    xcb_input_get_feedback_control_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
        ) -> xcb_input_get_feedback_control_cookie_t,
    >,
    xcb_input_get_feedback_control_feedbacks_length:
        LazySymbol<unsafe fn(r: *const xcb_input_get_feedback_control_reply_t) -> c_int>,
    xcb_input_get_feedback_control_feedbacks_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_feedback_control_reply_t,
        ) -> xcb_input_feedback_state_iterator_t,
    >,
    xcb_input_get_feedback_control_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_get_feedback_control_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_get_feedback_control_reply_t,
    >,
    xcb_input_kbd_feedback_ctl_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_kbd_feedback_ctl_iterator_t)>,
    xcb_input_kbd_feedback_ctl_end:
        LazySymbol<unsafe fn(i: xcb_input_kbd_feedback_ctl_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_ptr_feedback_ctl_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_ptr_feedback_ctl_iterator_t)>,
    xcb_input_ptr_feedback_ctl_end:
        LazySymbol<unsafe fn(i: xcb_input_ptr_feedback_ctl_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_integer_feedback_ctl_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_integer_feedback_ctl_iterator_t)>,
    xcb_input_integer_feedback_ctl_end: LazySymbol<
        unsafe fn(i: xcb_input_integer_feedback_ctl_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_string_feedback_ctl_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_string_feedback_ctl_keysyms:
        LazySymbol<unsafe fn(r: *const xcb_input_string_feedback_ctl_t) -> *mut xcb_keysym_t>,
    xcb_input_string_feedback_ctl_keysyms_length:
        LazySymbol<unsafe fn(r: *const xcb_input_string_feedback_ctl_t) -> c_int>,
    xcb_input_string_feedback_ctl_keysyms_end:
        LazySymbol<unsafe fn(r: *const xcb_input_string_feedback_ctl_t) -> xcb_generic_iterator_t>,
    xcb_input_string_feedback_ctl_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_string_feedback_ctl_iterator_t)>,
    xcb_input_string_feedback_ctl_end: LazySymbol<
        unsafe fn(i: xcb_input_string_feedback_ctl_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_bell_feedback_ctl_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_bell_feedback_ctl_iterator_t)>,
    xcb_input_bell_feedback_ctl_end:
        LazySymbol<unsafe fn(i: xcb_input_bell_feedback_ctl_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_led_feedback_ctl_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_led_feedback_ctl_iterator_t)>,
    xcb_input_led_feedback_ctl_end:
        LazySymbol<unsafe fn(i: xcb_input_led_feedback_ctl_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_feedback_ctl_data_string_keysyms:
        LazySymbol<unsafe fn(s: *const xcb_input_feedback_ctl_data_t) -> *mut xcb_keysym_t>,
    xcb_input_feedback_ctl_data_string_keysyms_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_feedback_ctl_t,
            s: *const xcb_input_feedback_ctl_data_t,
        ) -> c_int,
    >,
    xcb_input_feedback_ctl_data_string_keysyms_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_feedback_ctl_t,
            s: *const xcb_input_feedback_ctl_data_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_feedback_ctl_data_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            class_id: u8,
            _aux: *const xcb_input_feedback_ctl_data_t,
        ) -> c_int,
    >,
    xcb_input_feedback_ctl_data_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            class_id: u8,
            _aux: *mut xcb_input_feedback_ctl_data_t,
        ) -> c_int,
    >,
    xcb_input_feedback_ctl_data_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, class_id: u8) -> c_int>,
    xcb_input_feedback_ctl_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_feedback_ctl_data:
        LazySymbol<unsafe fn(r: *const xcb_input_feedback_ctl_t) -> *mut c_void>,
    xcb_input_feedback_ctl_next: LazySymbol<unsafe fn(i: *mut xcb_input_feedback_ctl_iterator_t)>,
    xcb_input_feedback_ctl_end:
        LazySymbol<unsafe fn(i: xcb_input_feedback_ctl_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_change_feedback_control_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_change_feedback_control_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            mask: u32,
            device_id: u8,
            feedback_id: u8,
            feedback: *mut xcb_input_feedback_ctl_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_change_feedback_control: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            mask: u32,
            device_id: u8,
            feedback_id: u8,
            feedback: *mut xcb_input_feedback_ctl_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_change_feedback_control_feedback: LazySymbol<
        unsafe fn(
            r: *const xcb_input_change_feedback_control_request_t,
        ) -> *mut xcb_input_feedback_ctl_t,
    >,
    xcb_input_get_device_key_mapping_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_get_device_key_mapping: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
            first_keycode: xcb_input_key_code_t,
            count: u8,
        ) -> xcb_input_get_device_key_mapping_cookie_t,
    >,
    xcb_input_get_device_key_mapping_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
            first_keycode: xcb_input_key_code_t,
            count: u8,
        ) -> xcb_input_get_device_key_mapping_cookie_t,
    >,
    xcb_input_get_device_key_mapping_keysyms: LazySymbol<
        unsafe fn(r: *const xcb_input_get_device_key_mapping_reply_t) -> *mut xcb_keysym_t,
    >,
    xcb_input_get_device_key_mapping_keysyms_length:
        LazySymbol<unsafe fn(r: *const xcb_input_get_device_key_mapping_reply_t) -> c_int>,
    xcb_input_get_device_key_mapping_keysyms_end: LazySymbol<
        unsafe fn(r: *const xcb_input_get_device_key_mapping_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_get_device_key_mapping_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_get_device_key_mapping_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_get_device_key_mapping_reply_t,
    >,
    xcb_input_change_device_key_mapping_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_change_device_key_mapping_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
            first_keycode: xcb_input_key_code_t,
            keysyms_per_keycode: u8,
            keycode_count: u8,
            keysyms: *const xcb_keysym_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_change_device_key_mapping: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
            first_keycode: xcb_input_key_code_t,
            keysyms_per_keycode: u8,
            keycode_count: u8,
            keysyms: *const xcb_keysym_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_change_device_key_mapping_keysyms: LazySymbol<
        unsafe fn(r: *const xcb_input_change_device_key_mapping_request_t) -> *mut xcb_keysym_t,
    >,
    xcb_input_change_device_key_mapping_keysyms_length:
        LazySymbol<unsafe fn(r: *const xcb_input_change_device_key_mapping_request_t) -> c_int>,
    xcb_input_change_device_key_mapping_keysyms_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_change_device_key_mapping_request_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_get_device_modifier_mapping_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_get_device_modifier_mapping: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
        ) -> xcb_input_get_device_modifier_mapping_cookie_t,
    >,
    xcb_input_get_device_modifier_mapping_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
        ) -> xcb_input_get_device_modifier_mapping_cookie_t,
    >,
    xcb_input_get_device_modifier_mapping_keymaps:
        LazySymbol<unsafe fn(r: *const xcb_input_get_device_modifier_mapping_reply_t) -> *mut u8>,
    xcb_input_get_device_modifier_mapping_keymaps_length:
        LazySymbol<unsafe fn(r: *const xcb_input_get_device_modifier_mapping_reply_t) -> c_int>,
    xcb_input_get_device_modifier_mapping_keymaps_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_device_modifier_mapping_reply_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_get_device_modifier_mapping_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_get_device_modifier_mapping_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_get_device_modifier_mapping_reply_t,
    >,
    xcb_input_set_device_modifier_mapping_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_set_device_modifier_mapping: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
            keycodes_per_modifier: u8,
            keymaps: *const u8,
        ) -> xcb_input_set_device_modifier_mapping_cookie_t,
    >,
    xcb_input_set_device_modifier_mapping_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
            keycodes_per_modifier: u8,
            keymaps: *const u8,
        ) -> xcb_input_set_device_modifier_mapping_cookie_t,
    >,
    xcb_input_set_device_modifier_mapping_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_set_device_modifier_mapping_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_set_device_modifier_mapping_reply_t,
    >,
    xcb_input_get_device_button_mapping_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_get_device_button_mapping: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
        ) -> xcb_input_get_device_button_mapping_cookie_t,
    >,
    xcb_input_get_device_button_mapping_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
        ) -> xcb_input_get_device_button_mapping_cookie_t,
    >,
    xcb_input_get_device_button_mapping_map:
        LazySymbol<unsafe fn(r: *const xcb_input_get_device_button_mapping_reply_t) -> *mut u8>,
    xcb_input_get_device_button_mapping_map_length:
        LazySymbol<unsafe fn(r: *const xcb_input_get_device_button_mapping_reply_t) -> c_int>,
    xcb_input_get_device_button_mapping_map_end: LazySymbol<
        unsafe fn(r: *const xcb_input_get_device_button_mapping_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_get_device_button_mapping_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_get_device_button_mapping_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_get_device_button_mapping_reply_t,
    >,
    xcb_input_set_device_button_mapping_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_set_device_button_mapping: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
            map_size: u8,
            map: *const u8,
        ) -> xcb_input_set_device_button_mapping_cookie_t,
    >,
    xcb_input_set_device_button_mapping_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
            map_size: u8,
            map: *const u8,
        ) -> xcb_input_set_device_button_mapping_cookie_t,
    >,
    xcb_input_set_device_button_mapping_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_set_device_button_mapping_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_set_device_button_mapping_reply_t,
    >,
    xcb_input_key_state_next: LazySymbol<unsafe fn(i: *mut xcb_input_key_state_iterator_t)>,
    xcb_input_key_state_end:
        LazySymbol<unsafe fn(i: xcb_input_key_state_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_button_state_next: LazySymbol<unsafe fn(i: *mut xcb_input_button_state_iterator_t)>,
    xcb_input_button_state_end:
        LazySymbol<unsafe fn(i: xcb_input_button_state_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_valuator_state_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_valuator_state_valuators:
        LazySymbol<unsafe fn(r: *const xcb_input_valuator_state_t) -> *mut i32>,
    xcb_input_valuator_state_valuators_length:
        LazySymbol<unsafe fn(r: *const xcb_input_valuator_state_t) -> c_int>,
    xcb_input_valuator_state_valuators_end:
        LazySymbol<unsafe fn(r: *const xcb_input_valuator_state_t) -> xcb_generic_iterator_t>,
    xcb_input_valuator_state_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_valuator_state_iterator_t)>,
    xcb_input_valuator_state_end:
        LazySymbol<unsafe fn(i: xcb_input_valuator_state_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_input_state_data_valuator_valuators:
        LazySymbol<unsafe fn(s: *const xcb_input_input_state_data_t) -> *mut i32>,
    xcb_input_input_state_data_valuator_valuators_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_input_state_t,
            s: *const xcb_input_input_state_data_t,
        ) -> c_int,
    >,
    xcb_input_input_state_data_valuator_valuators_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_input_state_t,
            s: *const xcb_input_input_state_data_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_input_state_data_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            class_id: u8,
            _aux: *const xcb_input_input_state_data_t,
        ) -> c_int,
    >,
    xcb_input_input_state_data_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            class_id: u8,
            _aux: *mut xcb_input_input_state_data_t,
        ) -> c_int,
    >,
    xcb_input_input_state_data_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, class_id: u8) -> c_int>,
    xcb_input_input_state_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_input_state_data:
        LazySymbol<unsafe fn(r: *const xcb_input_input_state_t) -> *mut c_void>,
    xcb_input_input_state_next: LazySymbol<unsafe fn(i: *mut xcb_input_input_state_iterator_t)>,
    xcb_input_input_state_end:
        LazySymbol<unsafe fn(i: xcb_input_input_state_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_query_device_state_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_query_device_state: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, device_id: u8) -> xcb_input_query_device_state_cookie_t,
    >,
    xcb_input_query_device_state_unchecked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, device_id: u8) -> xcb_input_query_device_state_cookie_t,
    >,
    xcb_input_query_device_state_classes_length:
        LazySymbol<unsafe fn(r: *const xcb_input_query_device_state_reply_t) -> c_int>,
    xcb_input_query_device_state_classes_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_input_query_device_state_reply_t,
        ) -> xcb_input_input_state_iterator_t,
    >,
    xcb_input_query_device_state_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_query_device_state_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_query_device_state_reply_t,
    >,
    xcb_input_device_bell_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
            feedback_id: u8,
            feedback_class: u8,
            percent: i8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_device_bell: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
            feedback_id: u8,
            feedback_class: u8,
            percent: i8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_set_device_valuators_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_set_device_valuators: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
            first_valuator: u8,
            num_valuators: u8,
            valuators: *const i32,
        ) -> xcb_input_set_device_valuators_cookie_t,
    >,
    xcb_input_set_device_valuators_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
            first_valuator: u8,
            num_valuators: u8,
            valuators: *const i32,
        ) -> xcb_input_set_device_valuators_cookie_t,
    >,
    xcb_input_set_device_valuators_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_set_device_valuators_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_set_device_valuators_reply_t,
    >,
    xcb_input_device_resolution_state_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_device_resolution_state_resolution_values:
        LazySymbol<unsafe fn(r: *const xcb_input_device_resolution_state_t) -> *mut u32>,
    xcb_input_device_resolution_state_resolution_values_length:
        LazySymbol<unsafe fn(r: *const xcb_input_device_resolution_state_t) -> c_int>,
    xcb_input_device_resolution_state_resolution_values_end: LazySymbol<
        unsafe fn(r: *const xcb_input_device_resolution_state_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_resolution_state_resolution_min:
        LazySymbol<unsafe fn(r: *const xcb_input_device_resolution_state_t) -> *mut u32>,
    xcb_input_device_resolution_state_resolution_min_length:
        LazySymbol<unsafe fn(r: *const xcb_input_device_resolution_state_t) -> c_int>,
    xcb_input_device_resolution_state_resolution_min_end: LazySymbol<
        unsafe fn(r: *const xcb_input_device_resolution_state_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_resolution_state_resolution_max:
        LazySymbol<unsafe fn(r: *const xcb_input_device_resolution_state_t) -> *mut u32>,
    xcb_input_device_resolution_state_resolution_max_length:
        LazySymbol<unsafe fn(r: *const xcb_input_device_resolution_state_t) -> c_int>,
    xcb_input_device_resolution_state_resolution_max_end: LazySymbol<
        unsafe fn(r: *const xcb_input_device_resolution_state_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_resolution_state_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_device_resolution_state_iterator_t)>,
    xcb_input_device_resolution_state_end: LazySymbol<
        unsafe fn(i: xcb_input_device_resolution_state_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_abs_calib_state_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_device_abs_calib_state_iterator_t)>,
    xcb_input_device_abs_calib_state_end: LazySymbol<
        unsafe fn(i: xcb_input_device_abs_calib_state_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_abs_area_state_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_device_abs_area_state_iterator_t)>,
    xcb_input_device_abs_area_state_end: LazySymbol<
        unsafe fn(i: xcb_input_device_abs_area_state_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_core_state_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_device_core_state_iterator_t)>,
    xcb_input_device_core_state_end:
        LazySymbol<unsafe fn(i: xcb_input_device_core_state_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_device_enable_state_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_device_enable_state_iterator_t)>,
    xcb_input_device_enable_state_end: LazySymbol<
        unsafe fn(i: xcb_input_device_enable_state_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_state_data_resolution_resolution_values:
        LazySymbol<unsafe fn(s: *const xcb_input_device_state_data_t) -> *mut u32>,
    xcb_input_device_state_data_resolution_resolution_values_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_device_state_t,
            s: *const xcb_input_device_state_data_t,
        ) -> c_int,
    >,
    xcb_input_device_state_data_resolution_resolution_values_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_device_state_t,
            s: *const xcb_input_device_state_data_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_state_data_resolution_resolution_min:
        LazySymbol<unsafe fn(s: *const xcb_input_device_state_data_t) -> *mut u32>,
    xcb_input_device_state_data_resolution_resolution_min_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_device_state_t,
            s: *const xcb_input_device_state_data_t,
        ) -> c_int,
    >,
    xcb_input_device_state_data_resolution_resolution_min_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_device_state_t,
            s: *const xcb_input_device_state_data_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_state_data_resolution_resolution_max:
        LazySymbol<unsafe fn(s: *const xcb_input_device_state_data_t) -> *mut u32>,
    xcb_input_device_state_data_resolution_resolution_max_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_device_state_t,
            s: *const xcb_input_device_state_data_t,
        ) -> c_int,
    >,
    xcb_input_device_state_data_resolution_resolution_max_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_device_state_t,
            s: *const xcb_input_device_state_data_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_state_data_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            control_id: u16,
            _aux: *const xcb_input_device_state_data_t,
        ) -> c_int,
    >,
    xcb_input_device_state_data_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            control_id: u16,
            _aux: *mut xcb_input_device_state_data_t,
        ) -> c_int,
    >,
    xcb_input_device_state_data_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, control_id: u16) -> c_int>,
    xcb_input_device_state_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_device_state_data:
        LazySymbol<unsafe fn(r: *const xcb_input_device_state_t) -> *mut c_void>,
    xcb_input_device_state_next: LazySymbol<unsafe fn(i: *mut xcb_input_device_state_iterator_t)>,
    xcb_input_device_state_end:
        LazySymbol<unsafe fn(i: xcb_input_device_state_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_get_device_control_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_get_device_control: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            control_id: u16,
            device_id: u8,
        ) -> xcb_input_get_device_control_cookie_t,
    >,
    xcb_input_get_device_control_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            control_id: u16,
            device_id: u8,
        ) -> xcb_input_get_device_control_cookie_t,
    >,
    xcb_input_get_device_control_control: LazySymbol<
        unsafe fn(r: *const xcb_input_get_device_control_reply_t) -> *mut xcb_input_device_state_t,
    >,
    xcb_input_get_device_control_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_get_device_control_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_get_device_control_reply_t,
    >,
    xcb_input_device_resolution_ctl_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_device_resolution_ctl_resolution_values:
        LazySymbol<unsafe fn(r: *const xcb_input_device_resolution_ctl_t) -> *mut u32>,
    xcb_input_device_resolution_ctl_resolution_values_length:
        LazySymbol<unsafe fn(r: *const xcb_input_device_resolution_ctl_t) -> c_int>,
    xcb_input_device_resolution_ctl_resolution_values_end: LazySymbol<
        unsafe fn(r: *const xcb_input_device_resolution_ctl_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_resolution_ctl_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_device_resolution_ctl_iterator_t)>,
    xcb_input_device_resolution_ctl_end: LazySymbol<
        unsafe fn(i: xcb_input_device_resolution_ctl_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_abs_calib_ctl_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_device_abs_calib_ctl_iterator_t)>,
    xcb_input_device_abs_calib_ctl_end: LazySymbol<
        unsafe fn(i: xcb_input_device_abs_calib_ctl_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_abs_area_ctrl_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_device_abs_area_ctrl_iterator_t)>,
    xcb_input_device_abs_area_ctrl_end: LazySymbol<
        unsafe fn(i: xcb_input_device_abs_area_ctrl_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_core_ctrl_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_device_core_ctrl_iterator_t)>,
    xcb_input_device_core_ctrl_end:
        LazySymbol<unsafe fn(i: xcb_input_device_core_ctrl_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_device_enable_ctrl_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_device_enable_ctrl_iterator_t)>,
    xcb_input_device_enable_ctrl_end:
        LazySymbol<unsafe fn(i: xcb_input_device_enable_ctrl_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_device_ctl_data_resolution_resolution_values:
        LazySymbol<unsafe fn(s: *const xcb_input_device_ctl_data_t) -> *mut u32>,
    xcb_input_device_ctl_data_resolution_resolution_values_length: LazySymbol<
        unsafe fn(r: *const xcb_input_device_ctl_t, s: *const xcb_input_device_ctl_data_t) -> c_int,
    >,
    xcb_input_device_ctl_data_resolution_resolution_values_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_device_ctl_t,
            s: *const xcb_input_device_ctl_data_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_ctl_data_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            control_id: u16,
            _aux: *const xcb_input_device_ctl_data_t,
        ) -> c_int,
    >,
    xcb_input_device_ctl_data_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            control_id: u16,
            _aux: *mut xcb_input_device_ctl_data_t,
        ) -> c_int,
    >,
    xcb_input_device_ctl_data_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, control_id: u16) -> c_int>,
    xcb_input_device_ctl_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_device_ctl_data:
        LazySymbol<unsafe fn(r: *const xcb_input_device_ctl_t) -> *mut c_void>,
    xcb_input_device_ctl_next: LazySymbol<unsafe fn(i: *mut xcb_input_device_ctl_iterator_t)>,
    xcb_input_device_ctl_end:
        LazySymbol<unsafe fn(i: xcb_input_device_ctl_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_change_device_control_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_change_device_control: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            control_id: u16,
            device_id: u8,
            control: *mut xcb_input_device_ctl_t,
        ) -> xcb_input_change_device_control_cookie_t,
    >,
    xcb_input_change_device_control_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            control_id: u16,
            device_id: u8,
            control: *mut xcb_input_device_ctl_t,
        ) -> xcb_input_change_device_control_cookie_t,
    >,
    xcb_input_change_device_control_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_change_device_control_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_change_device_control_reply_t,
    >,
    xcb_input_list_device_properties_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_list_device_properties: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
        ) -> xcb_input_list_device_properties_cookie_t,
    >,
    xcb_input_list_device_properties_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_id: u8,
        ) -> xcb_input_list_device_properties_cookie_t,
    >,
    xcb_input_list_device_properties_atoms: LazySymbol<
        unsafe fn(r: *const xcb_input_list_device_properties_reply_t) -> *mut xcb_atom_t,
    >,
    xcb_input_list_device_properties_atoms_length:
        LazySymbol<unsafe fn(r: *const xcb_input_list_device_properties_reply_t) -> c_int>,
    xcb_input_list_device_properties_atoms_end: LazySymbol<
        unsafe fn(r: *const xcb_input_list_device_properties_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_list_device_properties_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_list_device_properties_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_list_device_properties_reply_t,
    >,
    xcb_input_change_device_property_items_data_8:
        LazySymbol<unsafe fn(s: *const xcb_input_change_device_property_items_t) -> *mut u8>,
    xcb_input_change_device_property_items_data_8_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_change_device_property_request_t,
            s: *const xcb_input_change_device_property_items_t,
        ) -> c_int,
    >,
    xcb_input_change_device_property_items_data_8_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_change_device_property_request_t,
            s: *const xcb_input_change_device_property_items_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_change_device_property_items_data_16:
        LazySymbol<unsafe fn(s: *const xcb_input_change_device_property_items_t) -> *mut u16>,
    xcb_input_change_device_property_items_data_16_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_change_device_property_request_t,
            s: *const xcb_input_change_device_property_items_t,
        ) -> c_int,
    >,
    xcb_input_change_device_property_items_data_16_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_change_device_property_request_t,
            s: *const xcb_input_change_device_property_items_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_change_device_property_items_data_32:
        LazySymbol<unsafe fn(s: *const xcb_input_change_device_property_items_t) -> *mut u32>,
    xcb_input_change_device_property_items_data_32_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_change_device_property_request_t,
            s: *const xcb_input_change_device_property_items_t,
        ) -> c_int,
    >,
    xcb_input_change_device_property_items_data_32_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_change_device_property_request_t,
            s: *const xcb_input_change_device_property_items_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_change_device_property_items_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            num_items: u32,
            format: u8,
            _aux: *const xcb_input_change_device_property_items_t,
        ) -> c_int,
    >,
    xcb_input_change_device_property_items_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            num_items: u32,
            format: u8,
            _aux: *mut xcb_input_change_device_property_items_t,
        ) -> c_int,
    >,
    xcb_input_change_device_property_items_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, num_items: u32, format: u8) -> c_int>,
    xcb_input_change_device_property_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_change_device_property_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            device_id: u8,
            format: u8,
            mode: u8,
            num_items: u32,
            items: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_change_device_property: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            device_id: u8,
            format: u8,
            mode: u8,
            num_items: u32,
            items: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_change_device_property_aux_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            device_id: u8,
            format: u8,
            mode: u8,
            num_items: u32,
            items: *const xcb_input_change_device_property_items_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_change_device_property_aux: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            device_id: u8,
            format: u8,
            mode: u8,
            num_items: u32,
            items: *const xcb_input_change_device_property_items_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_change_device_property_items:
        LazySymbol<unsafe fn(r: *const xcb_input_change_device_property_request_t) -> *mut c_void>,
    xcb_input_delete_device_property_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            property: xcb_atom_t,
            device_id: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_delete_device_property: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            property: xcb_atom_t,
            device_id: u8,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_get_device_property_items_data_8:
        LazySymbol<unsafe fn(s: *const xcb_input_get_device_property_items_t) -> *mut u8>,
    xcb_input_get_device_property_items_data_8_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_device_property_reply_t,
            s: *const xcb_input_get_device_property_items_t,
        ) -> c_int,
    >,
    xcb_input_get_device_property_items_data_8_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_device_property_reply_t,
            s: *const xcb_input_get_device_property_items_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_get_device_property_items_data_16:
        LazySymbol<unsafe fn(s: *const xcb_input_get_device_property_items_t) -> *mut u16>,
    xcb_input_get_device_property_items_data_16_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_device_property_reply_t,
            s: *const xcb_input_get_device_property_items_t,
        ) -> c_int,
    >,
    xcb_input_get_device_property_items_data_16_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_device_property_reply_t,
            s: *const xcb_input_get_device_property_items_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_get_device_property_items_data_32:
        LazySymbol<unsafe fn(s: *const xcb_input_get_device_property_items_t) -> *mut u32>,
    xcb_input_get_device_property_items_data_32_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_device_property_reply_t,
            s: *const xcb_input_get_device_property_items_t,
        ) -> c_int,
    >,
    xcb_input_get_device_property_items_data_32_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_get_device_property_reply_t,
            s: *const xcb_input_get_device_property_items_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_get_device_property_items_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            num_items: u32,
            format: u8,
            _aux: *const xcb_input_get_device_property_items_t,
        ) -> c_int,
    >,
    xcb_input_get_device_property_items_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            num_items: u32,
            format: u8,
            _aux: *mut xcb_input_get_device_property_items_t,
        ) -> c_int,
    >,
    xcb_input_get_device_property_items_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, num_items: u32, format: u8) -> c_int>,
    xcb_input_get_device_property_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_get_device_property: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            offset: u32,
            len: u32,
            device_id: u8,
            delete: u8,
        ) -> xcb_input_get_device_property_cookie_t,
    >,
    xcb_input_get_device_property_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            offset: u32,
            len: u32,
            device_id: u8,
            delete: u8,
        ) -> xcb_input_get_device_property_cookie_t,
    >,
    xcb_input_get_device_property_items:
        LazySymbol<unsafe fn(r: *const xcb_input_get_device_property_reply_t) -> *mut c_void>,
    xcb_input_get_device_property_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_get_device_property_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_get_device_property_reply_t,
    >,
    xcb_input_group_info_next: LazySymbol<unsafe fn(i: *mut xcb_input_group_info_iterator_t)>,
    xcb_input_group_info_end:
        LazySymbol<unsafe fn(i: xcb_input_group_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_modifier_info_next: LazySymbol<unsafe fn(i: *mut xcb_input_modifier_info_iterator_t)>,
    xcb_input_modifier_info_end:
        LazySymbol<unsafe fn(i: xcb_input_modifier_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_xi_query_pointer_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_xi_query_pointer: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_input_xi_query_pointer_cookie_t,
    >,
    xcb_input_xi_query_pointer_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_input_xi_query_pointer_cookie_t,
    >,
    xcb_input_xi_query_pointer_buttons:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_query_pointer_reply_t) -> *mut u32>,
    xcb_input_xi_query_pointer_buttons_length:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_query_pointer_reply_t) -> c_int>,
    xcb_input_xi_query_pointer_buttons_end: LazySymbol<
        unsafe fn(r: *const xcb_input_xi_query_pointer_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_xi_query_pointer_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_xi_query_pointer_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_xi_query_pointer_reply_t,
    >,
    xcb_input_xi_warp_pointer_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src_win: xcb_window_t,
            dst_win: xcb_window_t,
            src_x: xcb_input_fp1616_t,
            src_y: xcb_input_fp1616_t,
            src_width: u16,
            src_height: u16,
            dst_x: xcb_input_fp1616_t,
            dst_y: xcb_input_fp1616_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_warp_pointer: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            src_win: xcb_window_t,
            dst_win: xcb_window_t,
            src_x: xcb_input_fp1616_t,
            src_y: xcb_input_fp1616_t,
            src_width: u16,
            src_height: u16,
            dst_x: xcb_input_fp1616_t,
            dst_y: xcb_input_fp1616_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_change_cursor_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            cursor: xcb_cursor_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_change_cursor: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            cursor: xcb_cursor_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_add_master_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_add_master_name:
        LazySymbol<unsafe fn(r: *const xcb_input_add_master_t) -> *mut c_char>,
    xcb_input_add_master_name_length:
        LazySymbol<unsafe fn(r: *const xcb_input_add_master_t) -> c_int>,
    xcb_input_add_master_name_end:
        LazySymbol<unsafe fn(r: *const xcb_input_add_master_t) -> xcb_generic_iterator_t>,
    xcb_input_add_master_next: LazySymbol<unsafe fn(i: *mut xcb_input_add_master_iterator_t)>,
    xcb_input_add_master_end:
        LazySymbol<unsafe fn(i: xcb_input_add_master_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_remove_master_next: LazySymbol<unsafe fn(i: *mut xcb_input_remove_master_iterator_t)>,
    xcb_input_remove_master_end:
        LazySymbol<unsafe fn(i: xcb_input_remove_master_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_attach_slave_next: LazySymbol<unsafe fn(i: *mut xcb_input_attach_slave_iterator_t)>,
    xcb_input_attach_slave_end:
        LazySymbol<unsafe fn(i: xcb_input_attach_slave_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_detach_slave_next: LazySymbol<unsafe fn(i: *mut xcb_input_detach_slave_iterator_t)>,
    xcb_input_detach_slave_end:
        LazySymbol<unsafe fn(i: xcb_input_detach_slave_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_hierarchy_change_data_add_master_name:
        LazySymbol<unsafe fn(s: *const xcb_input_hierarchy_change_data_t) -> *mut c_char>,
    xcb_input_hierarchy_change_data_add_master_name_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_hierarchy_change_t,
            s: *const xcb_input_hierarchy_change_data_t,
        ) -> c_int,
    >,
    xcb_input_hierarchy_change_data_add_master_name_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_hierarchy_change_t,
            s: *const xcb_input_hierarchy_change_data_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_hierarchy_change_data_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            type_: u16,
            _aux: *const xcb_input_hierarchy_change_data_t,
        ) -> c_int,
    >,
    xcb_input_hierarchy_change_data_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            type_: u16,
            _aux: *mut xcb_input_hierarchy_change_data_t,
        ) -> c_int,
    >,
    xcb_input_hierarchy_change_data_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, type_: u16) -> c_int>,
    xcb_input_hierarchy_change_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_hierarchy_change_data:
        LazySymbol<unsafe fn(r: *const xcb_input_hierarchy_change_t) -> *mut c_void>,
    xcb_input_hierarchy_change_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_hierarchy_change_iterator_t)>,
    xcb_input_hierarchy_change_end:
        LazySymbol<unsafe fn(i: xcb_input_hierarchy_change_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_xi_change_hierarchy_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_xi_change_hierarchy_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            num_changes: u8,
            changes: *const xcb_input_hierarchy_change_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_change_hierarchy: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            num_changes: u8,
            changes: *const xcb_input_hierarchy_change_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_change_hierarchy_changes_length:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_change_hierarchy_request_t) -> c_int>,
    xcb_input_xi_change_hierarchy_changes_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_change_hierarchy_request_t,
        ) -> xcb_input_hierarchy_change_iterator_t,
    >,
    xcb_input_xi_set_client_pointer_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_set_client_pointer: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_get_client_pointer: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_input_xi_get_client_pointer_cookie_t,
    >,
    xcb_input_xi_get_client_pointer_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_input_xi_get_client_pointer_cookie_t,
    >,
    xcb_input_xi_get_client_pointer_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_xi_get_client_pointer_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_xi_get_client_pointer_reply_t,
    >,
    xcb_input_event_mask_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_event_mask_mask: LazySymbol<unsafe fn(r: *const xcb_input_event_mask_t) -> *mut u32>,
    xcb_input_event_mask_mask_length:
        LazySymbol<unsafe fn(r: *const xcb_input_event_mask_t) -> c_int>,
    xcb_input_event_mask_mask_end:
        LazySymbol<unsafe fn(r: *const xcb_input_event_mask_t) -> xcb_generic_iterator_t>,
    xcb_input_event_mask_next: LazySymbol<unsafe fn(i: *mut xcb_input_event_mask_iterator_t)>,
    xcb_input_event_mask_end:
        LazySymbol<unsafe fn(i: xcb_input_event_mask_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_xi_select_events_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_xi_select_events_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            num_mask: u16,
            masks: *const xcb_input_event_mask_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_select_events: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            num_mask: u16,
            masks: *const xcb_input_event_mask_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_select_events_masks_length:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_select_events_request_t) -> c_int>,
    xcb_input_xi_select_events_masks_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_select_events_request_t,
        ) -> xcb_input_event_mask_iterator_t,
    >,
    xcb_input_xi_query_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major_version: u16,
            minor_version: u16,
        ) -> xcb_input_xi_query_version_cookie_t,
    >,
    xcb_input_xi_query_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            major_version: u16,
            minor_version: u16,
        ) -> xcb_input_xi_query_version_cookie_t,
    >,
    xcb_input_xi_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_xi_query_version_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_xi_query_version_reply_t,
    >,
    xcb_input_button_class_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_button_class_state:
        LazySymbol<unsafe fn(r: *const xcb_input_button_class_t) -> *mut u32>,
    xcb_input_button_class_state_length:
        LazySymbol<unsafe fn(r: *const xcb_input_button_class_t) -> c_int>,
    xcb_input_button_class_state_end:
        LazySymbol<unsafe fn(r: *const xcb_input_button_class_t) -> xcb_generic_iterator_t>,
    xcb_input_button_class_labels:
        LazySymbol<unsafe fn(r: *const xcb_input_button_class_t) -> *mut xcb_atom_t>,
    xcb_input_button_class_labels_length:
        LazySymbol<unsafe fn(r: *const xcb_input_button_class_t) -> c_int>,
    xcb_input_button_class_labels_end:
        LazySymbol<unsafe fn(r: *const xcb_input_button_class_t) -> xcb_generic_iterator_t>,
    xcb_input_button_class_next: LazySymbol<unsafe fn(i: *mut xcb_input_button_class_iterator_t)>,
    xcb_input_button_class_end:
        LazySymbol<unsafe fn(i: xcb_input_button_class_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_key_class_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_key_class_keys: LazySymbol<unsafe fn(r: *const xcb_input_key_class_t) -> *mut u32>,
    xcb_input_key_class_keys_length:
        LazySymbol<unsafe fn(r: *const xcb_input_key_class_t) -> c_int>,
    xcb_input_key_class_keys_end:
        LazySymbol<unsafe fn(r: *const xcb_input_key_class_t) -> xcb_generic_iterator_t>,
    xcb_input_key_class_next: LazySymbol<unsafe fn(i: *mut xcb_input_key_class_iterator_t)>,
    xcb_input_key_class_end:
        LazySymbol<unsafe fn(i: xcb_input_key_class_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_scroll_class_next: LazySymbol<unsafe fn(i: *mut xcb_input_scroll_class_iterator_t)>,
    xcb_input_scroll_class_end:
        LazySymbol<unsafe fn(i: xcb_input_scroll_class_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_touch_class_next: LazySymbol<unsafe fn(i: *mut xcb_input_touch_class_iterator_t)>,
    xcb_input_touch_class_end:
        LazySymbol<unsafe fn(i: xcb_input_touch_class_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_valuator_class_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_valuator_class_iterator_t)>,
    xcb_input_valuator_class_end:
        LazySymbol<unsafe fn(i: xcb_input_valuator_class_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_device_class_data_key_keys:
        LazySymbol<unsafe fn(s: *const xcb_input_device_class_data_t) -> *mut u32>,
    xcb_input_device_class_data_key_keys_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_device_class_t,
            s: *const xcb_input_device_class_data_t,
        ) -> c_int,
    >,
    xcb_input_device_class_data_key_keys_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_device_class_t,
            s: *const xcb_input_device_class_data_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_class_data_button_state:
        LazySymbol<unsafe fn(s: *const xcb_input_device_class_data_t) -> *mut u32>,
    xcb_input_device_class_data_button_state_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_device_class_t,
            s: *const xcb_input_device_class_data_t,
        ) -> c_int,
    >,
    xcb_input_device_class_data_button_state_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_device_class_t,
            s: *const xcb_input_device_class_data_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_class_data_button_labels:
        LazySymbol<unsafe fn(s: *const xcb_input_device_class_data_t) -> *mut xcb_atom_t>,
    xcb_input_device_class_data_button_labels_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_device_class_t,
            s: *const xcb_input_device_class_data_t,
        ) -> c_int,
    >,
    xcb_input_device_class_data_button_labels_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_device_class_t,
            s: *const xcb_input_device_class_data_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_device_class_data_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            type_: u16,
            _aux: *const xcb_input_device_class_data_t,
        ) -> c_int,
    >,
    xcb_input_device_class_data_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            type_: u16,
            _aux: *mut xcb_input_device_class_data_t,
        ) -> c_int,
    >,
    xcb_input_device_class_data_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, type_: u16) -> c_int>,
    xcb_input_device_class_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_device_class_data:
        LazySymbol<unsafe fn(r: *const xcb_input_device_class_t) -> *mut c_void>,
    xcb_input_device_class_next: LazySymbol<unsafe fn(i: *mut xcb_input_device_class_iterator_t)>,
    xcb_input_device_class_end:
        LazySymbol<unsafe fn(i: xcb_input_device_class_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_xi_device_info_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_xi_device_info_name:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_device_info_t) -> *mut c_char>,
    xcb_input_xi_device_info_name_length:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_device_info_t) -> c_int>,
    xcb_input_xi_device_info_name_end:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_device_info_t) -> xcb_generic_iterator_t>,
    xcb_input_xi_device_info_classes_length:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_device_info_t) -> c_int>,
    xcb_input_xi_device_info_classes_iterator: LazySymbol<
        unsafe fn(r: *const xcb_input_xi_device_info_t) -> xcb_input_device_class_iterator_t,
    >,
    xcb_input_xi_device_info_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_xi_device_info_iterator_t)>,
    xcb_input_xi_device_info_end:
        LazySymbol<unsafe fn(i: xcb_input_xi_device_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_xi_query_device_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_xi_query_device: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_input_xi_query_device_cookie_t,
    >,
    xcb_input_xi_query_device_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_input_xi_query_device_cookie_t,
    >,
    xcb_input_xi_query_device_infos_length:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_query_device_reply_t) -> c_int>,
    xcb_input_xi_query_device_infos_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_query_device_reply_t,
        ) -> xcb_input_xi_device_info_iterator_t,
    >,
    xcb_input_xi_query_device_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_xi_query_device_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_xi_query_device_reply_t,
    >,
    xcb_input_xi_set_focus_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            time: xcb_timestamp_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_set_focus: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            time: xcb_timestamp_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_get_focus: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_input_xi_get_focus_cookie_t,
    >,
    xcb_input_xi_get_focus_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_input_xi_get_focus_cookie_t,
    >,
    xcb_input_xi_get_focus_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_xi_get_focus_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_xi_get_focus_reply_t,
    >,
    xcb_input_xi_grab_device_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_xi_grab_device: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            time: xcb_timestamp_t,
            cursor: xcb_cursor_t,
            deviceid: xcb_input_device_id_t,
            mode: u8,
            paired_device_mode: u8,
            owner_events: u8,
            mask_len: u16,
            mask: *const u32,
        ) -> xcb_input_xi_grab_device_cookie_t,
    >,
    xcb_input_xi_grab_device_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
            time: xcb_timestamp_t,
            cursor: xcb_cursor_t,
            deviceid: xcb_input_device_id_t,
            mode: u8,
            paired_device_mode: u8,
            owner_events: u8,
            mask_len: u16,
            mask: *const u32,
        ) -> xcb_input_xi_grab_device_cookie_t,
    >,
    xcb_input_xi_grab_device_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_xi_grab_device_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_xi_grab_device_reply_t,
    >,
    xcb_input_xi_ungrab_device_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            time: xcb_timestamp_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_ungrab_device: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            time: xcb_timestamp_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_allow_events_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            time: xcb_timestamp_t,
            deviceid: xcb_input_device_id_t,
            event_mode: u8,
            touchid: u32,
            grab_window: xcb_window_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_allow_events: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            time: xcb_timestamp_t,
            deviceid: xcb_input_device_id_t,
            event_mode: u8,
            touchid: u32,
            grab_window: xcb_window_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_grab_modifier_info_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_grab_modifier_info_iterator_t)>,
    xcb_input_grab_modifier_info_end:
        LazySymbol<unsafe fn(i: xcb_input_grab_modifier_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_xi_passive_grab_device_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_xi_passive_grab_device: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            time: xcb_timestamp_t,
            grab_window: xcb_window_t,
            cursor: xcb_cursor_t,
            detail: u32,
            deviceid: xcb_input_device_id_t,
            num_modifiers: u16,
            mask_len: u16,
            grab_type: u8,
            grab_mode: u8,
            paired_device_mode: u8,
            owner_events: u8,
            mask: *const u32,
            modifiers: *const u32,
        ) -> xcb_input_xi_passive_grab_device_cookie_t,
    >,
    xcb_input_xi_passive_grab_device_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            time: xcb_timestamp_t,
            grab_window: xcb_window_t,
            cursor: xcb_cursor_t,
            detail: u32,
            deviceid: xcb_input_device_id_t,
            num_modifiers: u16,
            mask_len: u16,
            grab_type: u8,
            grab_mode: u8,
            paired_device_mode: u8,
            owner_events: u8,
            mask: *const u32,
            modifiers: *const u32,
        ) -> xcb_input_xi_passive_grab_device_cookie_t,
    >,
    xcb_input_xi_passive_grab_device_modifiers: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_passive_grab_device_reply_t,
        ) -> *mut xcb_input_grab_modifier_info_t,
    >,
    xcb_input_xi_passive_grab_device_modifiers_length:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_passive_grab_device_reply_t) -> c_int>,
    xcb_input_xi_passive_grab_device_modifiers_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_passive_grab_device_reply_t,
        ) -> xcb_input_grab_modifier_info_iterator_t,
    >,
    xcb_input_xi_passive_grab_device_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_xi_passive_grab_device_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_xi_passive_grab_device_reply_t,
    >,
    xcb_input_xi_passive_ungrab_device_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_xi_passive_ungrab_device_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            grab_window: xcb_window_t,
            detail: u32,
            deviceid: xcb_input_device_id_t,
            num_modifiers: u16,
            grab_type: u8,
            modifiers: *const u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_passive_ungrab_device: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            grab_window: xcb_window_t,
            detail: u32,
            deviceid: xcb_input_device_id_t,
            num_modifiers: u16,
            grab_type: u8,
            modifiers: *const u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_passive_ungrab_device_modifiers:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_passive_ungrab_device_request_t) -> *mut u32>,
    xcb_input_xi_passive_ungrab_device_modifiers_length:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_passive_ungrab_device_request_t) -> c_int>,
    xcb_input_xi_passive_ungrab_device_modifiers_end: LazySymbol<
        unsafe fn(r: *const xcb_input_xi_passive_ungrab_device_request_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_xi_list_properties_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_xi_list_properties: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_input_xi_list_properties_cookie_t,
    >,
    xcb_input_xi_list_properties_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            deviceid: xcb_input_device_id_t,
        ) -> xcb_input_xi_list_properties_cookie_t,
    >,
    xcb_input_xi_list_properties_properties:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_list_properties_reply_t) -> *mut xcb_atom_t>,
    xcb_input_xi_list_properties_properties_length:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_list_properties_reply_t) -> c_int>,
    xcb_input_xi_list_properties_properties_end: LazySymbol<
        unsafe fn(r: *const xcb_input_xi_list_properties_reply_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_xi_list_properties_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_xi_list_properties_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_xi_list_properties_reply_t,
    >,
    xcb_input_xi_change_property_items_data_8:
        LazySymbol<unsafe fn(s: *const xcb_input_xi_change_property_items_t) -> *mut u8>,
    xcb_input_xi_change_property_items_data_8_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_change_property_request_t,
            s: *const xcb_input_xi_change_property_items_t,
        ) -> c_int,
    >,
    xcb_input_xi_change_property_items_data_8_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_change_property_request_t,
            s: *const xcb_input_xi_change_property_items_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_xi_change_property_items_data_16:
        LazySymbol<unsafe fn(s: *const xcb_input_xi_change_property_items_t) -> *mut u16>,
    xcb_input_xi_change_property_items_data_16_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_change_property_request_t,
            s: *const xcb_input_xi_change_property_items_t,
        ) -> c_int,
    >,
    xcb_input_xi_change_property_items_data_16_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_change_property_request_t,
            s: *const xcb_input_xi_change_property_items_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_xi_change_property_items_data_32:
        LazySymbol<unsafe fn(s: *const xcb_input_xi_change_property_items_t) -> *mut u32>,
    xcb_input_xi_change_property_items_data_32_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_change_property_request_t,
            s: *const xcb_input_xi_change_property_items_t,
        ) -> c_int,
    >,
    xcb_input_xi_change_property_items_data_32_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_change_property_request_t,
            s: *const xcb_input_xi_change_property_items_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_xi_change_property_items_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            num_items: u32,
            format: u8,
            _aux: *const xcb_input_xi_change_property_items_t,
        ) -> c_int,
    >,
    xcb_input_xi_change_property_items_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            num_items: u32,
            format: u8,
            _aux: *mut xcb_input_xi_change_property_items_t,
        ) -> c_int,
    >,
    xcb_input_xi_change_property_items_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, num_items: u32, format: u8) -> c_int>,
    xcb_input_xi_change_property_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_xi_change_property_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            deviceid: xcb_input_device_id_t,
            mode: u8,
            format: u8,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            num_items: u32,
            items: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_change_property: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            deviceid: xcb_input_device_id_t,
            mode: u8,
            format: u8,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            num_items: u32,
            items: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_change_property_aux_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            deviceid: xcb_input_device_id_t,
            mode: u8,
            format: u8,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            num_items: u32,
            items: *const xcb_input_xi_change_property_items_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_change_property_aux: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            deviceid: xcb_input_device_id_t,
            mode: u8,
            format: u8,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            num_items: u32,
            items: *const xcb_input_xi_change_property_items_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_change_property_items:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_change_property_request_t) -> *mut c_void>,
    xcb_input_xi_delete_property_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            deviceid: xcb_input_device_id_t,
            property: xcb_atom_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_delete_property: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            deviceid: xcb_input_device_id_t,
            property: xcb_atom_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_get_property_items_data_8:
        LazySymbol<unsafe fn(s: *const xcb_input_xi_get_property_items_t) -> *mut u8>,
    xcb_input_xi_get_property_items_data_8_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_get_property_reply_t,
            s: *const xcb_input_xi_get_property_items_t,
        ) -> c_int,
    >,
    xcb_input_xi_get_property_items_data_8_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_get_property_reply_t,
            s: *const xcb_input_xi_get_property_items_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_xi_get_property_items_data_16:
        LazySymbol<unsafe fn(s: *const xcb_input_xi_get_property_items_t) -> *mut u16>,
    xcb_input_xi_get_property_items_data_16_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_get_property_reply_t,
            s: *const xcb_input_xi_get_property_items_t,
        ) -> c_int,
    >,
    xcb_input_xi_get_property_items_data_16_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_get_property_reply_t,
            s: *const xcb_input_xi_get_property_items_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_xi_get_property_items_data_32:
        LazySymbol<unsafe fn(s: *const xcb_input_xi_get_property_items_t) -> *mut u32>,
    xcb_input_xi_get_property_items_data_32_length: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_get_property_reply_t,
            s: *const xcb_input_xi_get_property_items_t,
        ) -> c_int,
    >,
    xcb_input_xi_get_property_items_data_32_end: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_get_property_reply_t,
            s: *const xcb_input_xi_get_property_items_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_input_xi_get_property_items_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            num_items: u32,
            format: u8,
            _aux: *const xcb_input_xi_get_property_items_t,
        ) -> c_int,
    >,
    xcb_input_xi_get_property_items_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            num_items: u32,
            format: u8,
            _aux: *mut xcb_input_xi_get_property_items_t,
        ) -> c_int,
    >,
    xcb_input_xi_get_property_items_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, num_items: u32, format: u8) -> c_int>,
    xcb_input_xi_get_property_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_xi_get_property: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            deviceid: xcb_input_device_id_t,
            delete: u8,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            offset: u32,
            len: u32,
        ) -> xcb_input_xi_get_property_cookie_t,
    >,
    xcb_input_xi_get_property_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            deviceid: xcb_input_device_id_t,
            delete: u8,
            property: xcb_atom_t,
            type_: xcb_atom_t,
            offset: u32,
            len: u32,
        ) -> xcb_input_xi_get_property_cookie_t,
    >,
    xcb_input_xi_get_property_items:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_get_property_reply_t) -> *mut c_void>,
    xcb_input_xi_get_property_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_xi_get_property_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_xi_get_property_reply_t,
    >,
    xcb_input_xi_get_selected_events_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_xi_get_selected_events: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_input_xi_get_selected_events_cookie_t,
    >,
    xcb_input_xi_get_selected_events_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            window: xcb_window_t,
        ) -> xcb_input_xi_get_selected_events_cookie_t,
    >,
    xcb_input_xi_get_selected_events_masks_length:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_get_selected_events_reply_t) -> c_int>,
    xcb_input_xi_get_selected_events_masks_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_get_selected_events_reply_t,
        ) -> xcb_input_event_mask_iterator_t,
    >,
    xcb_input_xi_get_selected_events_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_input_xi_get_selected_events_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_input_xi_get_selected_events_reply_t,
    >,
    xcb_input_barrier_release_pointer_info_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_barrier_release_pointer_info_iterator_t)>,
    xcb_input_barrier_release_pointer_info_end: LazySymbol<
        unsafe fn(i: xcb_input_barrier_release_pointer_info_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_xi_barrier_release_pointer_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_xi_barrier_release_pointer_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            num_barriers: u32,
            barriers: *const xcb_input_barrier_release_pointer_info_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_barrier_release_pointer: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            num_barriers: u32,
            barriers: *const xcb_input_barrier_release_pointer_info_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_xi_barrier_release_pointer_barriers: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_barrier_release_pointer_request_t,
        ) -> *mut xcb_input_barrier_release_pointer_info_t,
    >,
    xcb_input_xi_barrier_release_pointer_barriers_length:
        LazySymbol<unsafe fn(r: *const xcb_input_xi_barrier_release_pointer_request_t) -> c_int>,
    xcb_input_xi_barrier_release_pointer_barriers_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_input_xi_barrier_release_pointer_request_t,
        ) -> xcb_input_barrier_release_pointer_info_iterator_t,
    >,
    xcb_input_device_changed_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_device_changed_classes_length:
        LazySymbol<unsafe fn(r: *const xcb_input_device_changed_event_t) -> c_int>,
    xcb_input_device_changed_classes_iterator: LazySymbol<
        unsafe fn(r: *const xcb_input_device_changed_event_t) -> xcb_input_device_class_iterator_t,
    >,
    xcb_input_key_press_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_key_press_button_mask:
        LazySymbol<unsafe fn(r: *const xcb_input_key_press_event_t) -> *mut u32>,
    xcb_input_key_press_button_mask_length:
        LazySymbol<unsafe fn(r: *const xcb_input_key_press_event_t) -> c_int>,
    xcb_input_key_press_button_mask_end:
        LazySymbol<unsafe fn(r: *const xcb_input_key_press_event_t) -> xcb_generic_iterator_t>,
    xcb_input_key_press_valuator_mask:
        LazySymbol<unsafe fn(r: *const xcb_input_key_press_event_t) -> *mut u32>,
    xcb_input_key_press_valuator_mask_length:
        LazySymbol<unsafe fn(r: *const xcb_input_key_press_event_t) -> c_int>,
    xcb_input_key_press_valuator_mask_end:
        LazySymbol<unsafe fn(r: *const xcb_input_key_press_event_t) -> xcb_generic_iterator_t>,
    xcb_input_key_press_axisvalues:
        LazySymbol<unsafe fn(r: *const xcb_input_key_press_event_t) -> *mut xcb_input_fp3232_t>,
    xcb_input_key_press_axisvalues_length:
        LazySymbol<unsafe fn(r: *const xcb_input_key_press_event_t) -> c_int>,
    xcb_input_key_press_axisvalues_iterator:
        LazySymbol<unsafe fn(r: *const xcb_input_key_press_event_t) -> xcb_input_fp3232_iterator_t>,
    xcb_input_key_release_sizeof:
        LazySymbol<unsafe fn(buffer: *const xcb_input_key_release_event_t) -> c_int>,
    xcb_input_button_press_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_button_press_button_mask:
        LazySymbol<unsafe fn(r: *const xcb_input_button_press_event_t) -> *mut u32>,
    xcb_input_button_press_button_mask_length:
        LazySymbol<unsafe fn(r: *const xcb_input_button_press_event_t) -> c_int>,
    xcb_input_button_press_button_mask_end:
        LazySymbol<unsafe fn(r: *const xcb_input_button_press_event_t) -> xcb_generic_iterator_t>,
    xcb_input_button_press_valuator_mask:
        LazySymbol<unsafe fn(r: *const xcb_input_button_press_event_t) -> *mut u32>,
    xcb_input_button_press_valuator_mask_length:
        LazySymbol<unsafe fn(r: *const xcb_input_button_press_event_t) -> c_int>,
    xcb_input_button_press_valuator_mask_end:
        LazySymbol<unsafe fn(r: *const xcb_input_button_press_event_t) -> xcb_generic_iterator_t>,
    xcb_input_button_press_axisvalues:
        LazySymbol<unsafe fn(r: *const xcb_input_button_press_event_t) -> *mut xcb_input_fp3232_t>,
    xcb_input_button_press_axisvalues_length:
        LazySymbol<unsafe fn(r: *const xcb_input_button_press_event_t) -> c_int>,
    xcb_input_button_press_axisvalues_iterator: LazySymbol<
        unsafe fn(r: *const xcb_input_button_press_event_t) -> xcb_input_fp3232_iterator_t,
    >,
    xcb_input_button_release_sizeof:
        LazySymbol<unsafe fn(buffer: *const xcb_input_button_release_event_t) -> c_int>,
    xcb_input_motion_sizeof:
        LazySymbol<unsafe fn(buffer: *const xcb_input_motion_event_t) -> c_int>,
    xcb_input_enter_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_enter_buttons: LazySymbol<unsafe fn(r: *const xcb_input_enter_event_t) -> *mut u32>,
    xcb_input_enter_buttons_length:
        LazySymbol<unsafe fn(r: *const xcb_input_enter_event_t) -> c_int>,
    xcb_input_enter_buttons_end:
        LazySymbol<unsafe fn(r: *const xcb_input_enter_event_t) -> xcb_generic_iterator_t>,
    xcb_input_leave_sizeof: LazySymbol<unsafe fn(buffer: *const xcb_input_leave_event_t) -> c_int>,
    xcb_input_focus_in_sizeof:
        LazySymbol<unsafe fn(buffer: *const xcb_input_focus_in_event_t) -> c_int>,
    xcb_input_focus_out_sizeof:
        LazySymbol<unsafe fn(buffer: *const xcb_input_focus_out_event_t) -> c_int>,
    xcb_input_hierarchy_info_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_hierarchy_info_iterator_t)>,
    xcb_input_hierarchy_info_end:
        LazySymbol<unsafe fn(i: xcb_input_hierarchy_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_hierarchy_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_hierarchy_infos: LazySymbol<
        unsafe fn(r: *const xcb_input_hierarchy_event_t) -> *mut xcb_input_hierarchy_info_t,
    >,
    xcb_input_hierarchy_infos_length:
        LazySymbol<unsafe fn(r: *const xcb_input_hierarchy_event_t) -> c_int>,
    xcb_input_hierarchy_infos_iterator: LazySymbol<
        unsafe fn(r: *const xcb_input_hierarchy_event_t) -> xcb_input_hierarchy_info_iterator_t,
    >,
    xcb_input_raw_key_press_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_raw_key_press_valuator_mask:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_key_press_event_t) -> *mut u32>,
    xcb_input_raw_key_press_valuator_mask_length:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_key_press_event_t) -> c_int>,
    xcb_input_raw_key_press_valuator_mask_end:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_key_press_event_t) -> xcb_generic_iterator_t>,
    xcb_input_raw_key_press_axisvalues:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_key_press_event_t) -> *mut xcb_input_fp3232_t>,
    xcb_input_raw_key_press_axisvalues_length:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_key_press_event_t) -> c_int>,
    xcb_input_raw_key_press_axisvalues_iterator: LazySymbol<
        unsafe fn(r: *const xcb_input_raw_key_press_event_t) -> xcb_input_fp3232_iterator_t,
    >,
    xcb_input_raw_key_press_axisvalues_raw:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_key_press_event_t) -> *mut xcb_input_fp3232_t>,
    xcb_input_raw_key_press_axisvalues_raw_length:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_key_press_event_t) -> c_int>,
    xcb_input_raw_key_press_axisvalues_raw_iterator: LazySymbol<
        unsafe fn(r: *const xcb_input_raw_key_press_event_t) -> xcb_input_fp3232_iterator_t,
    >,
    xcb_input_raw_key_release_sizeof:
        LazySymbol<unsafe fn(buffer: *const xcb_input_raw_key_release_event_t) -> c_int>,
    xcb_input_raw_button_press_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_raw_button_press_valuator_mask:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_button_press_event_t) -> *mut u32>,
    xcb_input_raw_button_press_valuator_mask_length:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_button_press_event_t) -> c_int>,
    xcb_input_raw_button_press_valuator_mask_end: LazySymbol<
        unsafe fn(r: *const xcb_input_raw_button_press_event_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_raw_button_press_axisvalues: LazySymbol<
        unsafe fn(r: *const xcb_input_raw_button_press_event_t) -> *mut xcb_input_fp3232_t,
    >,
    xcb_input_raw_button_press_axisvalues_length:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_button_press_event_t) -> c_int>,
    xcb_input_raw_button_press_axisvalues_iterator: LazySymbol<
        unsafe fn(r: *const xcb_input_raw_button_press_event_t) -> xcb_input_fp3232_iterator_t,
    >,
    xcb_input_raw_button_press_axisvalues_raw: LazySymbol<
        unsafe fn(r: *const xcb_input_raw_button_press_event_t) -> *mut xcb_input_fp3232_t,
    >,
    xcb_input_raw_button_press_axisvalues_raw_length:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_button_press_event_t) -> c_int>,
    xcb_input_raw_button_press_axisvalues_raw_iterator: LazySymbol<
        unsafe fn(r: *const xcb_input_raw_button_press_event_t) -> xcb_input_fp3232_iterator_t,
    >,
    xcb_input_raw_button_release_sizeof:
        LazySymbol<unsafe fn(buffer: *const xcb_input_raw_button_release_event_t) -> c_int>,
    xcb_input_raw_motion_sizeof:
        LazySymbol<unsafe fn(buffer: *const xcb_input_raw_motion_event_t) -> c_int>,
    xcb_input_touch_begin_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_touch_begin_button_mask:
        LazySymbol<unsafe fn(r: *const xcb_input_touch_begin_event_t) -> *mut u32>,
    xcb_input_touch_begin_button_mask_length:
        LazySymbol<unsafe fn(r: *const xcb_input_touch_begin_event_t) -> c_int>,
    xcb_input_touch_begin_button_mask_end:
        LazySymbol<unsafe fn(r: *const xcb_input_touch_begin_event_t) -> xcb_generic_iterator_t>,
    xcb_input_touch_begin_valuator_mask:
        LazySymbol<unsafe fn(r: *const xcb_input_touch_begin_event_t) -> *mut u32>,
    xcb_input_touch_begin_valuator_mask_length:
        LazySymbol<unsafe fn(r: *const xcb_input_touch_begin_event_t) -> c_int>,
    xcb_input_touch_begin_valuator_mask_end:
        LazySymbol<unsafe fn(r: *const xcb_input_touch_begin_event_t) -> xcb_generic_iterator_t>,
    xcb_input_touch_begin_axisvalues:
        LazySymbol<unsafe fn(r: *const xcb_input_touch_begin_event_t) -> *mut xcb_input_fp3232_t>,
    xcb_input_touch_begin_axisvalues_length:
        LazySymbol<unsafe fn(r: *const xcb_input_touch_begin_event_t) -> c_int>,
    xcb_input_touch_begin_axisvalues_iterator: LazySymbol<
        unsafe fn(r: *const xcb_input_touch_begin_event_t) -> xcb_input_fp3232_iterator_t,
    >,
    xcb_input_touch_update_sizeof:
        LazySymbol<unsafe fn(buffer: *const xcb_input_touch_update_event_t) -> c_int>,
    xcb_input_touch_end_sizeof:
        LazySymbol<unsafe fn(buffer: *const xcb_input_touch_end_event_t) -> c_int>,
    xcb_input_raw_touch_begin_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_raw_touch_begin_valuator_mask:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_touch_begin_event_t) -> *mut u32>,
    xcb_input_raw_touch_begin_valuator_mask_length:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_touch_begin_event_t) -> c_int>,
    xcb_input_raw_touch_begin_valuator_mask_end: LazySymbol<
        unsafe fn(r: *const xcb_input_raw_touch_begin_event_t) -> xcb_generic_iterator_t,
    >,
    xcb_input_raw_touch_begin_axisvalues: LazySymbol<
        unsafe fn(r: *const xcb_input_raw_touch_begin_event_t) -> *mut xcb_input_fp3232_t,
    >,
    xcb_input_raw_touch_begin_axisvalues_length:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_touch_begin_event_t) -> c_int>,
    xcb_input_raw_touch_begin_axisvalues_iterator: LazySymbol<
        unsafe fn(r: *const xcb_input_raw_touch_begin_event_t) -> xcb_input_fp3232_iterator_t,
    >,
    xcb_input_raw_touch_begin_axisvalues_raw: LazySymbol<
        unsafe fn(r: *const xcb_input_raw_touch_begin_event_t) -> *mut xcb_input_fp3232_t,
    >,
    xcb_input_raw_touch_begin_axisvalues_raw_length:
        LazySymbol<unsafe fn(r: *const xcb_input_raw_touch_begin_event_t) -> c_int>,
    xcb_input_raw_touch_begin_axisvalues_raw_iterator: LazySymbol<
        unsafe fn(r: *const xcb_input_raw_touch_begin_event_t) -> xcb_input_fp3232_iterator_t,
    >,
    xcb_input_raw_touch_update_sizeof:
        LazySymbol<unsafe fn(buffer: *const xcb_input_raw_touch_update_event_t) -> c_int>,
    xcb_input_raw_touch_end_sizeof:
        LazySymbol<unsafe fn(buffer: *const xcb_input_raw_touch_end_event_t) -> c_int>,
    xcb_input_event_for_send_next:
        LazySymbol<unsafe fn(i: *mut xcb_input_event_for_send_iterator_t)>,
    xcb_input_event_for_send_end:
        LazySymbol<unsafe fn(i: xcb_input_event_for_send_iterator_t) -> xcb_generic_iterator_t>,
    xcb_input_send_extension_event_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_input_send_extension_event_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            destination: xcb_window_t,
            device_id: u8,
            propagate: u8,
            num_classes: u16,
            num_events: u8,
            events: *const xcb_input_event_for_send_t,
            classes: *const xcb_input_event_class_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_send_extension_event: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            destination: xcb_window_t,
            device_id: u8,
            propagate: u8,
            num_classes: u16,
            num_events: u8,
            events: *const xcb_input_event_for_send_t,
            classes: *const xcb_input_event_class_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_input_send_extension_event_events: LazySymbol<
        unsafe fn(
            r: *const xcb_input_send_extension_event_request_t,
        ) -> *mut xcb_input_event_for_send_t,
    >,
    xcb_input_send_extension_event_events_length:
        LazySymbol<unsafe fn(r: *const xcb_input_send_extension_event_request_t) -> c_int>,
    xcb_input_send_extension_event_events_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_input_send_extension_event_request_t,
        ) -> xcb_input_event_for_send_iterator_t,
    >,
    xcb_input_send_extension_event_classes: LazySymbol<
        unsafe fn(
            r: *const xcb_input_send_extension_event_request_t,
        ) -> *mut xcb_input_event_class_t,
    >,
    xcb_input_send_extension_event_classes_length:
        LazySymbol<unsafe fn(r: *const xcb_input_send_extension_event_request_t) -> c_int>,
    xcb_input_send_extension_event_classes_end: LazySymbol<
        unsafe fn(r: *const xcb_input_send_extension_event_request_t) -> xcb_generic_iterator_t,
    >,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self
            .xinput
            .$f
            .get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .xinput
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

#[cfg(feature = "xcb_xinput")]
impl XcbXinput {
    /// The libxcb identifier of the `Input` extension.
    #[inline]
    pub fn xcb_input_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_input_id) }
    }

    /// Returns `true` iff the symbol `xcb_input_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_id(&self) -> bool {
        has_sym!(self, xcb_input_id)
    }

    /// Advances a `xcb_input_event_class_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_event_class_next(&self, i: *mut xcb_input_event_class_iterator_t) {
        sym!(self, xcb_input_event_class_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_event_class_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_event_class_next(&self) -> bool {
        has_sym!(self, xcb_input_event_class_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_event_class_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_event_class_end(
        &self,
        i: xcb_input_event_class_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_event_class_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_event_class_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_event_class_end(&self) -> bool {
        has_sym!(self, xcb_input_event_class_end)
    }

    /// Advances a `xcb_input_key_code_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_key_code_next(&self, i: *mut xcb_input_key_code_iterator_t) {
        sym!(self, xcb_input_key_code_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_key_code_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_code_next(&self) -> bool {
        has_sym!(self, xcb_input_key_code_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_key_code_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_key_code_end(
        &self,
        i: xcb_input_key_code_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_key_code_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_key_code_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_code_end(&self) -> bool {
        has_sym!(self, xcb_input_key_code_end)
    }

    /// Advances a `xcb_input_device_id_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_device_id_next(&self, i: *mut xcb_input_device_id_iterator_t) {
        sym!(self, xcb_input_device_id_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_id_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_id_next(&self) -> bool {
        has_sym!(self, xcb_input_device_id_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_device_id_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_device_id_end(
        &self,
        i: xcb_input_device_id_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_id_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_id_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_id_end(&self) -> bool {
        has_sym!(self, xcb_input_device_id_end)
    }

    /// Advances a `xcb_input_fp1616_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_fp1616_next(&self, i: *mut xcb_input_fp1616_iterator_t) {
        sym!(self, xcb_input_fp1616_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_fp1616_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_fp1616_next(&self) -> bool {
        has_sym!(self, xcb_input_fp1616_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_fp1616_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_fp1616_end(
        &self,
        i: xcb_input_fp1616_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_fp1616_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_fp1616_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_fp1616_end(&self) -> bool {
        has_sym!(self, xcb_input_fp1616_end)
    }

    /// Advances a `xcb_input_fp3232_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_fp3232_next(&self, i: *mut xcb_input_fp3232_iterator_t) {
        sym!(self, xcb_input_fp3232_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_fp3232_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_fp3232_next(&self) -> bool {
        has_sym!(self, xcb_input_fp3232_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_fp3232_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_fp3232_end(
        &self,
        i: xcb_input_fp3232_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_fp3232_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_fp3232_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_fp3232_end(&self) -> bool {
        has_sym!(self, xcb_input_fp3232_end)
    }

    /// Computes the size of a `xcb_input_get_extension_version_request_t` object.
    #[inline]
    pub unsafe fn xcb_input_get_extension_version_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_get_extension_version_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_get_extension_version_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_extension_version_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_get_extension_version_sizeof)
    }

    /// Sends a `Input::GetExtensionVersion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_get_extension_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_get_extension_version_reply`]: Self::xcb_input_get_extension_version_reply
    #[inline]
    pub unsafe fn xcb_input_get_extension_version(
        &self,
        c: *mut xcb_connection_t,
        name_len: u16,
        name: *const c_char,
    ) -> xcb_input_get_extension_version_cookie_t {
        sym!(self, xcb_input_get_extension_version)(c, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_input_get_extension_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_extension_version(&self) -> bool {
        has_sym!(self, xcb_input_get_extension_version)
    }

    /// Sends a `Input::GetExtensionVersion` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_get_extension_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_get_extension_version_reply`]: Self::xcb_input_get_extension_version_reply
    #[inline]
    pub unsafe fn xcb_input_get_extension_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        name_len: u16,
        name: *const c_char,
    ) -> xcb_input_get_extension_version_cookie_t {
        sym!(self, xcb_input_get_extension_version_unchecked)(c, name_len, name)
    }

    /// Returns `true` iff the symbol `xcb_input_get_extension_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_extension_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_get_extension_version_unchecked)
    }

    /// Waits for the reply to a `Input::GetExtensionVersion` request.
    #[inline]
    pub unsafe fn xcb_input_get_extension_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_get_extension_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_get_extension_version_reply_t {
        sym!(self, xcb_input_get_extension_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_get_extension_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_extension_version_reply(&self) -> bool {
        has_sym!(self, xcb_input_get_extension_version_reply)
    }

    /// Advances a `xcb_input_device_info_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_device_info_next(&self, i: *mut xcb_input_device_info_iterator_t) {
        sym!(self, xcb_input_device_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_info_next(&self) -> bool {
        has_sym!(self, xcb_input_device_info_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_device_info_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_device_info_end(
        &self,
        i: xcb_input_device_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_info_end(&self) -> bool {
        has_sym!(self, xcb_input_device_info_end)
    }

    /// Advances a `xcb_input_key_info_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_key_info_next(&self, i: *mut xcb_input_key_info_iterator_t) {
        sym!(self, xcb_input_key_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_key_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_info_next(&self) -> bool {
        has_sym!(self, xcb_input_key_info_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_key_info_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_key_info_end(
        &self,
        i: xcb_input_key_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_key_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_key_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_info_end(&self) -> bool {
        has_sym!(self, xcb_input_key_info_end)
    }

    /// Advances a `xcb_input_button_info_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_button_info_next(&self, i: *mut xcb_input_button_info_iterator_t) {
        sym!(self, xcb_input_button_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_button_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_info_next(&self) -> bool {
        has_sym!(self, xcb_input_button_info_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_button_info_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_button_info_end(
        &self,
        i: xcb_input_button_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_button_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_button_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_info_end(&self) -> bool {
        has_sym!(self, xcb_input_button_info_end)
    }

    /// Advances a `xcb_input_axis_info_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_axis_info_next(&self, i: *mut xcb_input_axis_info_iterator_t) {
        sym!(self, xcb_input_axis_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_axis_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_axis_info_next(&self) -> bool {
        has_sym!(self, xcb_input_axis_info_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_axis_info_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_axis_info_end(
        &self,
        i: xcb_input_axis_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_axis_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_axis_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_axis_info_end(&self) -> bool {
        has_sym!(self, xcb_input_axis_info_end)
    }

    /// Computes the size of a `xcb_input_valuator_info_t` object.
    #[inline]
    pub unsafe fn xcb_input_valuator_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_valuator_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_valuator_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_valuator_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_valuator_info_sizeof)
    }

    /// Returns a pointer to the `axes` field of a `xcb_input_valuator_info_t` struct.
    #[inline]
    pub unsafe fn xcb_input_valuator_info_axes(
        &self,
        r: *const xcb_input_valuator_info_t,
    ) -> *mut xcb_input_axis_info_t {
        sym!(self, xcb_input_valuator_info_axes)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_valuator_info_axes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_valuator_info_axes(&self) -> bool {
        has_sym!(self, xcb_input_valuator_info_axes)
    }

    /// Returns the number of elements of the `axes` field of a `xcb_input_valuator_info_t` struct.
    #[inline]
    pub unsafe fn xcb_input_valuator_info_axes_length(
        &self,
        r: *const xcb_input_valuator_info_t,
    ) -> c_int {
        sym!(self, xcb_input_valuator_info_axes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_valuator_info_axes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_valuator_info_axes_length(&self) -> bool {
        has_sym!(self, xcb_input_valuator_info_axes_length)
    }

    /// Returns an iterator over the elements of the
    /// `axes` field of a `xcb_input_valuator_info_t` struct.
    #[inline]
    pub unsafe fn xcb_input_valuator_info_axes_iterator(
        &self,
        r: *const xcb_input_valuator_info_t,
    ) -> xcb_input_axis_info_iterator_t {
        sym!(self, xcb_input_valuator_info_axes_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_valuator_info_axes_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_valuator_info_axes_iterator(&self) -> bool {
        has_sym!(self, xcb_input_valuator_info_axes_iterator)
    }

    /// Advances a `xcb_input_valuator_info_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_valuator_info_next(&self, i: *mut xcb_input_valuator_info_iterator_t) {
        sym!(self, xcb_input_valuator_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_valuator_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_valuator_info_next(&self) -> bool {
        has_sym!(self, xcb_input_valuator_info_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_valuator_info_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_valuator_info_end(
        &self,
        i: xcb_input_valuator_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_valuator_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_valuator_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_valuator_info_end(&self) -> bool {
        has_sym!(self, xcb_input_valuator_info_end)
    }

    /// Returns a pointer to the `axes` field of a `xcb_input_input_info_info_valuator_t` struct.
    #[inline]
    pub unsafe fn xcb_input_input_info_info_valuator_axes(
        &self,
        s: *const xcb_input_input_info_info_t,
    ) -> *mut xcb_input_axis_info_t {
        sym!(self, xcb_input_input_info_info_valuator_axes)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_input_info_info_valuator_axes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_info_info_valuator_axes(&self) -> bool {
        has_sym!(self, xcb_input_input_info_info_valuator_axes)
    }

    /// Returns the number of elements of the `axes` field of a `xcb_input_input_info_info_valuator_t` struct.
    #[inline]
    pub unsafe fn xcb_input_input_info_info_valuator_axes_length(
        &self,
        r: *const xcb_input_input_info_t,
        s: *const xcb_input_input_info_info_t,
    ) -> c_int {
        sym!(self, xcb_input_input_info_info_valuator_axes_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_input_info_info_valuator_axes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_info_info_valuator_axes_length(&self) -> bool {
        has_sym!(self, xcb_input_input_info_info_valuator_axes_length)
    }

    /// Returns an iterator over the elements of the
    /// `axes` field of a `xcb_input_input_info_info_valuator_t` struct.
    #[inline]
    pub unsafe fn xcb_input_input_info_info_valuator_axes_iterator(
        &self,
        r: *const xcb_input_input_info_t,
        s: *const xcb_input_input_info_info_t,
    ) -> xcb_input_axis_info_iterator_t {
        sym!(self, xcb_input_input_info_info_valuator_axes_iterator)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_input_info_info_valuator_axes_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_info_info_valuator_axes_iterator(&self) -> bool {
        has_sym!(self, xcb_input_input_info_info_valuator_axes_iterator)
    }

    /// Serializes a `xcb_input_input_info_info_t` object.
    #[inline]
    pub unsafe fn xcb_input_input_info_info_serialize(
        &self,
        _buffer: *mut *mut c_void,
        class_id: u8,
        _aux: *const xcb_input_input_info_info_t,
    ) -> c_int {
        sym!(self, xcb_input_input_info_info_serialize)(_buffer, class_id, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_input_info_info_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_info_info_serialize(&self) -> bool {
        has_sym!(self, xcb_input_input_info_info_serialize)
    }

    /// Unpacks a `xcb_input_input_info_info_t` object.
    #[inline]
    pub unsafe fn xcb_input_input_info_info_unpack(
        &self,
        _buffer: *const c_void,
        class_id: u8,
        _aux: *mut xcb_input_input_info_info_t,
    ) -> c_int {
        sym!(self, xcb_input_input_info_info_unpack)(_buffer, class_id, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_input_info_info_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_info_info_unpack(&self) -> bool {
        has_sym!(self, xcb_input_input_info_info_unpack)
    }

    /// Computes the size of a `xcb_input_input_info_info_t` object.
    #[inline]
    pub unsafe fn xcb_input_input_info_info_sizeof(
        &self,
        _buffer: *const c_void,
        class_id: u8,
    ) -> c_int {
        sym!(self, xcb_input_input_info_info_sizeof)(_buffer, class_id)
    }

    /// Returns `true` iff the symbol `xcb_input_input_info_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_info_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_input_info_info_sizeof)
    }

    /// Computes the size of a `xcb_input_input_info_t` object.
    #[inline]
    pub unsafe fn xcb_input_input_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_input_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_input_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_input_info_sizeof)
    }

    /// Returns a pointer to the `info` field of a `xcb_input_input_info_t` struct.
    #[inline]
    pub unsafe fn xcb_input_input_info_info(
        &self,
        r: *const xcb_input_input_info_t,
    ) -> *mut c_void {
        sym!(self, xcb_input_input_info_info)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_input_info_info` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_info_info(&self) -> bool {
        has_sym!(self, xcb_input_input_info_info)
    }

    /// Advances a `xcb_input_input_info_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_input_info_next(&self, i: *mut xcb_input_input_info_iterator_t) {
        sym!(self, xcb_input_input_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_input_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_info_next(&self) -> bool {
        has_sym!(self, xcb_input_input_info_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_input_info_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_input_info_end(
        &self,
        i: xcb_input_input_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_input_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_input_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_info_end(&self) -> bool {
        has_sym!(self, xcb_input_input_info_end)
    }

    /// Computes the size of a `xcb_input_device_name_t` object.
    #[inline]
    pub unsafe fn xcb_input_device_name_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_device_name_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_device_name_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_name_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_device_name_sizeof)
    }

    /// Returns a pointer to the `string` field of a `xcb_input_device_name_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_name_string(
        &self,
        r: *const xcb_input_device_name_t,
    ) -> *mut c_char {
        sym!(self, xcb_input_device_name_string)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_name_string` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_name_string(&self) -> bool {
        has_sym!(self, xcb_input_device_name_string)
    }

    /// Returns the number of elements of the `string` field of a `xcb_input_device_name_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_name_string_length(
        &self,
        r: *const xcb_input_device_name_t,
    ) -> c_int {
        sym!(self, xcb_input_device_name_string_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_name_string_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_name_string_length(&self) -> bool {
        has_sym!(self, xcb_input_device_name_string_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `string` field of a `xcb_input_device_name_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_name_string_end(
        &self,
        r: *const xcb_input_device_name_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_name_string_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_name_string_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_name_string_end(&self) -> bool {
        has_sym!(self, xcb_input_device_name_string_end)
    }

    /// Advances a `xcb_input_device_name_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_device_name_next(&self, i: *mut xcb_input_device_name_iterator_t) {
        sym!(self, xcb_input_device_name_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_name_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_name_next(&self) -> bool {
        has_sym!(self, xcb_input_device_name_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_device_name_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_device_name_end(
        &self,
        i: xcb_input_device_name_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_name_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_name_end(&self) -> bool {
        has_sym!(self, xcb_input_device_name_end)
    }

    /// Computes the size of a `xcb_input_list_input_devices_reply_t` object.
    #[inline]
    pub unsafe fn xcb_input_list_input_devices_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_list_input_devices_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_list_input_devices_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_input_devices_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_list_input_devices_sizeof)
    }

    /// Sends a `Input::ListInputDevices` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_list_input_devices_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_list_input_devices_reply`]: Self::xcb_input_list_input_devices_reply
    #[inline]
    pub unsafe fn xcb_input_list_input_devices(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_input_list_input_devices_cookie_t {
        sym!(self, xcb_input_list_input_devices)(c)
    }

    /// Returns `true` iff the symbol `xcb_input_list_input_devices` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_input_devices(&self) -> bool {
        has_sym!(self, xcb_input_list_input_devices)
    }

    /// Sends a `Input::ListInputDevices` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_list_input_devices_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_list_input_devices_reply`]: Self::xcb_input_list_input_devices_reply
    #[inline]
    pub unsafe fn xcb_input_list_input_devices_unchecked(
        &self,
        c: *mut xcb_connection_t,
    ) -> xcb_input_list_input_devices_cookie_t {
        sym!(self, xcb_input_list_input_devices_unchecked)(c)
    }

    /// Returns `true` iff the symbol `xcb_input_list_input_devices_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_input_devices_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_list_input_devices_unchecked)
    }

    /// Returns a pointer to the `devices` field of a `xcb_input_list_input_devices_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_list_input_devices_devices(
        &self,
        r: *const xcb_input_list_input_devices_reply_t,
    ) -> *mut xcb_input_device_info_t {
        sym!(self, xcb_input_list_input_devices_devices)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_list_input_devices_devices` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_input_devices_devices(&self) -> bool {
        has_sym!(self, xcb_input_list_input_devices_devices)
    }

    /// Returns the number of elements of the `devices` field of a `xcb_input_list_input_devices_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_list_input_devices_devices_length(
        &self,
        r: *const xcb_input_list_input_devices_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_list_input_devices_devices_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_list_input_devices_devices_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_input_devices_devices_length(&self) -> bool {
        has_sym!(self, xcb_input_list_input_devices_devices_length)
    }

    /// Returns an iterator over the elements of the
    /// `devices` field of a `xcb_input_list_input_devices_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_list_input_devices_devices_iterator(
        &self,
        r: *const xcb_input_list_input_devices_reply_t,
    ) -> xcb_input_device_info_iterator_t {
        sym!(self, xcb_input_list_input_devices_devices_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_list_input_devices_devices_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_input_devices_devices_iterator(&self) -> bool {
        has_sym!(self, xcb_input_list_input_devices_devices_iterator)
    }

    /// Returns the number of elements of the `infos` field of a `xcb_input_list_input_devices_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_list_input_devices_infos_length(
        &self,
        r: *const xcb_input_list_input_devices_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_list_input_devices_infos_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_list_input_devices_infos_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_input_devices_infos_length(&self) -> bool {
        has_sym!(self, xcb_input_list_input_devices_infos_length)
    }

    /// Returns an iterator over the elements of the
    /// `infos` field of a `xcb_input_list_input_devices_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_list_input_devices_infos_iterator(
        &self,
        r: *const xcb_input_list_input_devices_reply_t,
    ) -> xcb_input_input_info_iterator_t {
        sym!(self, xcb_input_list_input_devices_infos_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_list_input_devices_infos_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_input_devices_infos_iterator(&self) -> bool {
        has_sym!(self, xcb_input_list_input_devices_infos_iterator)
    }

    /// Returns the number of elements of the `names` field of a `xcb_input_list_input_devices_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_list_input_devices_names_length(
        &self,
        r: *const xcb_input_list_input_devices_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_list_input_devices_names_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_list_input_devices_names_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_input_devices_names_length(&self) -> bool {
        has_sym!(self, xcb_input_list_input_devices_names_length)
    }

    /// Returns an iterator over the elements of the
    /// `names` field of a `xcb_input_list_input_devices_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_list_input_devices_names_iterator(
        &self,
        r: *const xcb_input_list_input_devices_reply_t,
    ) -> xcb_str_iterator_t {
        sym!(self, xcb_input_list_input_devices_names_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_list_input_devices_names_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_input_devices_names_iterator(&self) -> bool {
        has_sym!(self, xcb_input_list_input_devices_names_iterator)
    }

    /// Waits for the reply to a `Input::ListInputDevices` request.
    #[inline]
    pub unsafe fn xcb_input_list_input_devices_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_list_input_devices_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_list_input_devices_reply_t {
        sym!(self, xcb_input_list_input_devices_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_list_input_devices_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_input_devices_reply(&self) -> bool {
        has_sym!(self, xcb_input_list_input_devices_reply)
    }

    /// Advances a `xcb_input_event_type_base_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_event_type_base_next(
        &self,
        i: *mut xcb_input_event_type_base_iterator_t,
    ) {
        sym!(self, xcb_input_event_type_base_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_event_type_base_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_event_type_base_next(&self) -> bool {
        has_sym!(self, xcb_input_event_type_base_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_event_type_base_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_event_type_base_end(
        &self,
        i: xcb_input_event_type_base_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_event_type_base_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_event_type_base_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_event_type_base_end(&self) -> bool {
        has_sym!(self, xcb_input_event_type_base_end)
    }

    /// Advances a `xcb_input_input_class_info_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_input_class_info_next(
        &self,
        i: *mut xcb_input_input_class_info_iterator_t,
    ) {
        sym!(self, xcb_input_input_class_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_input_class_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_class_info_next(&self) -> bool {
        has_sym!(self, xcb_input_input_class_info_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_input_class_info_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_input_class_info_end(
        &self,
        i: xcb_input_input_class_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_input_class_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_input_class_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_class_info_end(&self) -> bool {
        has_sym!(self, xcb_input_input_class_info_end)
    }

    /// Computes the size of a `xcb_input_open_device_reply_t` object.
    #[inline]
    pub unsafe fn xcb_input_open_device_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_open_device_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_open_device_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_open_device_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_open_device_sizeof)
    }

    /// Sends a `Input::OpenDevice` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_open_device_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_open_device_reply`]: Self::xcb_input_open_device_reply
    #[inline]
    pub unsafe fn xcb_input_open_device(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_open_device_cookie_t {
        sym!(self, xcb_input_open_device)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_open_device` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_open_device(&self) -> bool {
        has_sym!(self, xcb_input_open_device)
    }

    /// Sends a `Input::OpenDevice` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_open_device_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_open_device_reply`]: Self::xcb_input_open_device_reply
    #[inline]
    pub unsafe fn xcb_input_open_device_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_open_device_cookie_t {
        sym!(self, xcb_input_open_device_unchecked)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_open_device_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_open_device_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_open_device_unchecked)
    }

    /// Returns a pointer to the `class_info` field of a `xcb_input_open_device_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_open_device_class_info(
        &self,
        r: *const xcb_input_open_device_reply_t,
    ) -> *mut xcb_input_input_class_info_t {
        sym!(self, xcb_input_open_device_class_info)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_open_device_class_info` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_open_device_class_info(&self) -> bool {
        has_sym!(self, xcb_input_open_device_class_info)
    }

    /// Returns the number of elements of the `class_info` field of a `xcb_input_open_device_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_open_device_class_info_length(
        &self,
        r: *const xcb_input_open_device_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_open_device_class_info_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_open_device_class_info_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_open_device_class_info_length(&self) -> bool {
        has_sym!(self, xcb_input_open_device_class_info_length)
    }

    /// Returns an iterator over the elements of the
    /// `class_info` field of a `xcb_input_open_device_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_open_device_class_info_iterator(
        &self,
        r: *const xcb_input_open_device_reply_t,
    ) -> xcb_input_input_class_info_iterator_t {
        sym!(self, xcb_input_open_device_class_info_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_open_device_class_info_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_open_device_class_info_iterator(&self) -> bool {
        has_sym!(self, xcb_input_open_device_class_info_iterator)
    }

    /// Waits for the reply to a `Input::OpenDevice` request.
    #[inline]
    pub unsafe fn xcb_input_open_device_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_open_device_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_open_device_reply_t {
        sym!(self, xcb_input_open_device_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_open_device_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_open_device_reply(&self) -> bool {
        has_sym!(self, xcb_input_open_device_reply)
    }

    /// Sends a `Input::CloseDevice` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_input_close_device_checked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_close_device_checked)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_close_device_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_close_device_checked(&self) -> bool {
        has_sym!(self, xcb_input_close_device_checked)
    }

    /// Sends a `Input::CloseDevice` request (unchecked).
    #[inline]
    pub unsafe fn xcb_input_close_device(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_close_device)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_close_device` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_close_device(&self) -> bool {
        has_sym!(self, xcb_input_close_device)
    }

    /// Sends a `Input::SetDeviceMode` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_set_device_mode_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_set_device_mode_reply`]: Self::xcb_input_set_device_mode_reply
    #[inline]
    pub unsafe fn xcb_input_set_device_mode(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
        mode: u8,
    ) -> xcb_input_set_device_mode_cookie_t {
        sym!(self, xcb_input_set_device_mode)(c, device_id, mode)
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_mode` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_mode(&self) -> bool {
        has_sym!(self, xcb_input_set_device_mode)
    }

    /// Sends a `Input::SetDeviceMode` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_set_device_mode_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_set_device_mode_reply`]: Self::xcb_input_set_device_mode_reply
    #[inline]
    pub unsafe fn xcb_input_set_device_mode_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
        mode: u8,
    ) -> xcb_input_set_device_mode_cookie_t {
        sym!(self, xcb_input_set_device_mode_unchecked)(c, device_id, mode)
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_mode_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_mode_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_set_device_mode_unchecked)
    }

    /// Waits for the reply to a `Input::SetDeviceMode` request.
    #[inline]
    pub unsafe fn xcb_input_set_device_mode_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_set_device_mode_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_set_device_mode_reply_t {
        sym!(self, xcb_input_set_device_mode_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_mode_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_mode_reply(&self) -> bool {
        has_sym!(self, xcb_input_set_device_mode_reply)
    }

    /// Computes the size of a `xcb_input_select_extension_event_request_t` object.
    #[inline]
    pub unsafe fn xcb_input_select_extension_event_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_select_extension_event_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_select_extension_event_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_select_extension_event_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_select_extension_event_sizeof)
    }

    /// Sends a `Input::SelectExtensionEvent` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_input_select_extension_event_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        num_classes: u16,
        classes: *const xcb_input_event_class_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_select_extension_event_checked)(c, window, num_classes, classes)
    }

    /// Returns `true` iff the symbol `xcb_input_select_extension_event_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_select_extension_event_checked(&self) -> bool {
        has_sym!(self, xcb_input_select_extension_event_checked)
    }

    /// Sends a `Input::SelectExtensionEvent` request (unchecked).
    #[inline]
    pub unsafe fn xcb_input_select_extension_event(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        num_classes: u16,
        classes: *const xcb_input_event_class_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_select_extension_event)(c, window, num_classes, classes)
    }

    /// Returns `true` iff the symbol `xcb_input_select_extension_event` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_select_extension_event(&self) -> bool {
        has_sym!(self, xcb_input_select_extension_event)
    }

    /// Returns a pointer to the `classes` field of a `xcb_input_select_extension_event_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_select_extension_event_classes(
        &self,
        r: *const xcb_input_select_extension_event_request_t,
    ) -> *mut xcb_input_event_class_t {
        sym!(self, xcb_input_select_extension_event_classes)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_select_extension_event_classes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_select_extension_event_classes(&self) -> bool {
        has_sym!(self, xcb_input_select_extension_event_classes)
    }

    /// Returns the number of elements of the `classes` field of a `xcb_input_select_extension_event_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_select_extension_event_classes_length(
        &self,
        r: *const xcb_input_select_extension_event_request_t,
    ) -> c_int {
        sym!(self, xcb_input_select_extension_event_classes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_select_extension_event_classes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_select_extension_event_classes_length(&self) -> bool {
        has_sym!(self, xcb_input_select_extension_event_classes_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `classes` field of a `xcb_input_select_extension_event_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_select_extension_event_classes_end(
        &self,
        r: *const xcb_input_select_extension_event_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_select_extension_event_classes_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_select_extension_event_classes_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_select_extension_event_classes_end(&self) -> bool {
        has_sym!(self, xcb_input_select_extension_event_classes_end)
    }

    /// Computes the size of a `xcb_input_get_selected_extension_events_reply_t` object.
    #[inline]
    pub unsafe fn xcb_input_get_selected_extension_events_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_input_get_selected_extension_events_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_get_selected_extension_events_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_selected_extension_events_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_get_selected_extension_events_sizeof)
    }

    /// Sends a `Input::GetSelectedExtensionEvents` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_get_selected_extension_events_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_get_selected_extension_events_reply`]: Self::xcb_input_get_selected_extension_events_reply
    #[inline]
    pub unsafe fn xcb_input_get_selected_extension_events(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_input_get_selected_extension_events_cookie_t {
        sym!(self, xcb_input_get_selected_extension_events)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_input_get_selected_extension_events` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_selected_extension_events(&self) -> bool {
        has_sym!(self, xcb_input_get_selected_extension_events)
    }

    /// Sends a `Input::GetSelectedExtensionEvents` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_get_selected_extension_events_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_get_selected_extension_events_reply`]: Self::xcb_input_get_selected_extension_events_reply
    #[inline]
    pub unsafe fn xcb_input_get_selected_extension_events_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_input_get_selected_extension_events_cookie_t {
        sym!(self, xcb_input_get_selected_extension_events_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_input_get_selected_extension_events_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_selected_extension_events_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_get_selected_extension_events_unchecked)
    }

    /// Returns a pointer to the `this_classes` field of a `xcb_input_get_selected_extension_events_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_selected_extension_events_this_classes(
        &self,
        r: *const xcb_input_get_selected_extension_events_reply_t,
    ) -> *mut xcb_input_event_class_t {
        sym!(self, xcb_input_get_selected_extension_events_this_classes)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_selected_extension_events_this_classes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_selected_extension_events_this_classes(&self) -> bool {
        has_sym!(self, xcb_input_get_selected_extension_events_this_classes)
    }

    /// Returns the number of elements of the `this_classes` field of a `xcb_input_get_selected_extension_events_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_selected_extension_events_this_classes_length(
        &self,
        r: *const xcb_input_get_selected_extension_events_reply_t,
    ) -> c_int {
        sym!(
            self,
            xcb_input_get_selected_extension_events_this_classes_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_selected_extension_events_this_classes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_selected_extension_events_this_classes_length(&self) -> bool {
        has_sym!(
            self,
            xcb_input_get_selected_extension_events_this_classes_length
        )
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `this_classes` field of a `xcb_input_get_selected_extension_events_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_selected_extension_events_this_classes_end(
        &self,
        r: *const xcb_input_get_selected_extension_events_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(
            self,
            xcb_input_get_selected_extension_events_this_classes_end
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_selected_extension_events_this_classes_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_selected_extension_events_this_classes_end(&self) -> bool {
        has_sym!(
            self,
            xcb_input_get_selected_extension_events_this_classes_end
        )
    }

    /// Returns a pointer to the `all_classes` field of a `xcb_input_get_selected_extension_events_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_selected_extension_events_all_classes(
        &self,
        r: *const xcb_input_get_selected_extension_events_reply_t,
    ) -> *mut xcb_input_event_class_t {
        sym!(self, xcb_input_get_selected_extension_events_all_classes)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_selected_extension_events_all_classes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_selected_extension_events_all_classes(&self) -> bool {
        has_sym!(self, xcb_input_get_selected_extension_events_all_classes)
    }

    /// Returns the number of elements of the `all_classes` field of a `xcb_input_get_selected_extension_events_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_selected_extension_events_all_classes_length(
        &self,
        r: *const xcb_input_get_selected_extension_events_reply_t,
    ) -> c_int {
        sym!(
            self,
            xcb_input_get_selected_extension_events_all_classes_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_selected_extension_events_all_classes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_selected_extension_events_all_classes_length(&self) -> bool {
        has_sym!(
            self,
            xcb_input_get_selected_extension_events_all_classes_length
        )
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `all_classes` field of a `xcb_input_get_selected_extension_events_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_selected_extension_events_all_classes_end(
        &self,
        r: *const xcb_input_get_selected_extension_events_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(
            self,
            xcb_input_get_selected_extension_events_all_classes_end
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_selected_extension_events_all_classes_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_selected_extension_events_all_classes_end(&self) -> bool {
        has_sym!(
            self,
            xcb_input_get_selected_extension_events_all_classes_end
        )
    }

    /// Waits for the reply to a `Input::GetSelectedExtensionEvents` request.
    #[inline]
    pub unsafe fn xcb_input_get_selected_extension_events_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_get_selected_extension_events_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_get_selected_extension_events_reply_t {
        sym!(self, xcb_input_get_selected_extension_events_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_get_selected_extension_events_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_selected_extension_events_reply(&self) -> bool {
        has_sym!(self, xcb_input_get_selected_extension_events_reply)
    }

    /// Computes the size of a `xcb_input_change_device_dont_propagate_list_request_t` object.
    #[inline]
    pub unsafe fn xcb_input_change_device_dont_propagate_list_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_input_change_device_dont_propagate_list_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_dont_propagate_list_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_dont_propagate_list_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_change_device_dont_propagate_list_sizeof)
    }

    /// Sends a `Input::ChangeDeviceDontPropagateList` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_input_change_device_dont_propagate_list_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        num_classes: u16,
        mode: u8,
        classes: *const xcb_input_event_class_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_change_device_dont_propagate_list_checked)(
            c,
            window,
            num_classes,
            mode,
            classes,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_dont_propagate_list_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_dont_propagate_list_checked(&self) -> bool {
        has_sym!(self, xcb_input_change_device_dont_propagate_list_checked)
    }

    /// Sends a `Input::ChangeDeviceDontPropagateList` request (unchecked).
    #[inline]
    pub unsafe fn xcb_input_change_device_dont_propagate_list(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        num_classes: u16,
        mode: u8,
        classes: *const xcb_input_event_class_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_change_device_dont_propagate_list)(
            c,
            window,
            num_classes,
            mode,
            classes,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_dont_propagate_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_dont_propagate_list(&self) -> bool {
        has_sym!(self, xcb_input_change_device_dont_propagate_list)
    }

    /// Returns a pointer to the `classes` field of a `xcb_input_change_device_dont_propagate_list_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_change_device_dont_propagate_list_classes(
        &self,
        r: *const xcb_input_change_device_dont_propagate_list_request_t,
    ) -> *mut xcb_input_event_class_t {
        sym!(self, xcb_input_change_device_dont_propagate_list_classes)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_dont_propagate_list_classes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_dont_propagate_list_classes(&self) -> bool {
        has_sym!(self, xcb_input_change_device_dont_propagate_list_classes)
    }

    /// Returns the number of elements of the `classes` field of a `xcb_input_change_device_dont_propagate_list_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_change_device_dont_propagate_list_classes_length(
        &self,
        r: *const xcb_input_change_device_dont_propagate_list_request_t,
    ) -> c_int {
        sym!(
            self,
            xcb_input_change_device_dont_propagate_list_classes_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_dont_propagate_list_classes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_dont_propagate_list_classes_length(&self) -> bool {
        has_sym!(
            self,
            xcb_input_change_device_dont_propagate_list_classes_length
        )
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `classes` field of a `xcb_input_change_device_dont_propagate_list_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_change_device_dont_propagate_list_classes_end(
        &self,
        r: *const xcb_input_change_device_dont_propagate_list_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(
            self,
            xcb_input_change_device_dont_propagate_list_classes_end
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_dont_propagate_list_classes_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_dont_propagate_list_classes_end(&self) -> bool {
        has_sym!(
            self,
            xcb_input_change_device_dont_propagate_list_classes_end
        )
    }

    /// Computes the size of a `xcb_input_get_device_dont_propagate_list_reply_t` object.
    #[inline]
    pub unsafe fn xcb_input_get_device_dont_propagate_list_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_input_get_device_dont_propagate_list_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_dont_propagate_list_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_dont_propagate_list_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_get_device_dont_propagate_list_sizeof)
    }

    /// Sends a `Input::GetDeviceDontPropagateList` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_get_device_dont_propagate_list_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_get_device_dont_propagate_list_reply`]: Self::xcb_input_get_device_dont_propagate_list_reply
    #[inline]
    pub unsafe fn xcb_input_get_device_dont_propagate_list(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_input_get_device_dont_propagate_list_cookie_t {
        sym!(self, xcb_input_get_device_dont_propagate_list)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_dont_propagate_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_dont_propagate_list(&self) -> bool {
        has_sym!(self, xcb_input_get_device_dont_propagate_list)
    }

    /// Sends a `Input::GetDeviceDontPropagateList` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_get_device_dont_propagate_list_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_get_device_dont_propagate_list_reply`]: Self::xcb_input_get_device_dont_propagate_list_reply
    #[inline]
    pub unsafe fn xcb_input_get_device_dont_propagate_list_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_input_get_device_dont_propagate_list_cookie_t {
        sym!(self, xcb_input_get_device_dont_propagate_list_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_dont_propagate_list_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_dont_propagate_list_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_get_device_dont_propagate_list_unchecked)
    }

    /// Returns a pointer to the `classes` field of a `xcb_input_get_device_dont_propagate_list_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_device_dont_propagate_list_classes(
        &self,
        r: *const xcb_input_get_device_dont_propagate_list_reply_t,
    ) -> *mut xcb_input_event_class_t {
        sym!(self, xcb_input_get_device_dont_propagate_list_classes)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_dont_propagate_list_classes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_dont_propagate_list_classes(&self) -> bool {
        has_sym!(self, xcb_input_get_device_dont_propagate_list_classes)
    }

    /// Returns the number of elements of the `classes` field of a `xcb_input_get_device_dont_propagate_list_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_device_dont_propagate_list_classes_length(
        &self,
        r: *const xcb_input_get_device_dont_propagate_list_reply_t,
    ) -> c_int {
        sym!(
            self,
            xcb_input_get_device_dont_propagate_list_classes_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_dont_propagate_list_classes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_dont_propagate_list_classes_length(&self) -> bool {
        has_sym!(
            self,
            xcb_input_get_device_dont_propagate_list_classes_length
        )
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `classes` field of a `xcb_input_get_device_dont_propagate_list_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_device_dont_propagate_list_classes_end(
        &self,
        r: *const xcb_input_get_device_dont_propagate_list_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_get_device_dont_propagate_list_classes_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_dont_propagate_list_classes_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_dont_propagate_list_classes_end(&self) -> bool {
        has_sym!(self, xcb_input_get_device_dont_propagate_list_classes_end)
    }

    /// Waits for the reply to a `Input::GetDeviceDontPropagateList` request.
    #[inline]
    pub unsafe fn xcb_input_get_device_dont_propagate_list_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_get_device_dont_propagate_list_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_get_device_dont_propagate_list_reply_t {
        sym!(self, xcb_input_get_device_dont_propagate_list_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_dont_propagate_list_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_dont_propagate_list_reply(&self) -> bool {
        has_sym!(self, xcb_input_get_device_dont_propagate_list_reply)
    }

    /// Computes the size of a `xcb_input_device_time_coord_t` object.
    #[inline]
    pub unsafe fn xcb_input_device_time_coord_sizeof(
        &self,
        _buffer: *const c_void,
        num_axes: u8,
    ) -> c_int {
        sym!(self, xcb_input_device_time_coord_sizeof)(_buffer, num_axes)
    }

    /// Returns `true` iff the symbol `xcb_input_device_time_coord_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_time_coord_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_device_time_coord_sizeof)
    }

    /// Returns a pointer to the `axisvalues` field of a `xcb_input_device_time_coord_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_time_coord_axisvalues(
        &self,
        r: *const xcb_input_device_time_coord_t,
    ) -> *mut i32 {
        sym!(self, xcb_input_device_time_coord_axisvalues)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_time_coord_axisvalues` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_time_coord_axisvalues(&self) -> bool {
        has_sym!(self, xcb_input_device_time_coord_axisvalues)
    }

    /// Returns the number of elements of the `axisvalues` field of a `xcb_input_device_time_coord_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_time_coord_axisvalues_length(
        &self,
        r: *const xcb_input_device_time_coord_t,
        num_axes: u8,
    ) -> c_int {
        sym!(self, xcb_input_device_time_coord_axisvalues_length)(r, num_axes)
    }

    /// Returns `true` iff the symbol `xcb_input_device_time_coord_axisvalues_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_time_coord_axisvalues_length(&self) -> bool {
        has_sym!(self, xcb_input_device_time_coord_axisvalues_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `axisvalues` field of a `xcb_input_device_time_coord_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_time_coord_axisvalues_end(
        &self,
        r: *const xcb_input_device_time_coord_t,
        num_axes: u8,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_time_coord_axisvalues_end)(r, num_axes)
    }

    /// Returns `true` iff the symbol `xcb_input_device_time_coord_axisvalues_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_time_coord_axisvalues_end(&self) -> bool {
        has_sym!(self, xcb_input_device_time_coord_axisvalues_end)
    }

    /// Advances a `xcb_input_device_time_coord_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_device_time_coord_next(
        &self,
        i: *mut xcb_input_device_time_coord_iterator_t,
    ) {
        sym!(self, xcb_input_device_time_coord_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_time_coord_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_time_coord_next(&self) -> bool {
        has_sym!(self, xcb_input_device_time_coord_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_device_time_coord_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_device_time_coord_end(
        &self,
        i: xcb_input_device_time_coord_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_time_coord_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_time_coord_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_time_coord_end(&self) -> bool {
        has_sym!(self, xcb_input_device_time_coord_end)
    }

    /// Computes the size of a `xcb_input_get_device_motion_events_reply_t` object.
    #[inline]
    pub unsafe fn xcb_input_get_device_motion_events_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_input_get_device_motion_events_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_motion_events_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_motion_events_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_get_device_motion_events_sizeof)
    }

    /// Sends a `Input::GetDeviceMotionEvents` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_get_device_motion_events_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_get_device_motion_events_reply`]: Self::xcb_input_get_device_motion_events_reply
    #[inline]
    pub unsafe fn xcb_input_get_device_motion_events(
        &self,
        c: *mut xcb_connection_t,
        start: xcb_timestamp_t,
        stop: xcb_timestamp_t,
        device_id: u8,
    ) -> xcb_input_get_device_motion_events_cookie_t {
        sym!(self, xcb_input_get_device_motion_events)(c, start, stop, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_motion_events` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_motion_events(&self) -> bool {
        has_sym!(self, xcb_input_get_device_motion_events)
    }

    /// Sends a `Input::GetDeviceMotionEvents` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_get_device_motion_events_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_get_device_motion_events_reply`]: Self::xcb_input_get_device_motion_events_reply
    #[inline]
    pub unsafe fn xcb_input_get_device_motion_events_unchecked(
        &self,
        c: *mut xcb_connection_t,
        start: xcb_timestamp_t,
        stop: xcb_timestamp_t,
        device_id: u8,
    ) -> xcb_input_get_device_motion_events_cookie_t {
        sym!(self, xcb_input_get_device_motion_events_unchecked)(c, start, stop, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_motion_events_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_motion_events_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_get_device_motion_events_unchecked)
    }

    /// Returns the number of elements of the `events` field of a `xcb_input_get_device_motion_events_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_device_motion_events_events_length(
        &self,
        r: *const xcb_input_get_device_motion_events_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_get_device_motion_events_events_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_motion_events_events_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_motion_events_events_length(&self) -> bool {
        has_sym!(self, xcb_input_get_device_motion_events_events_length)
    }

    /// Returns an iterator over the elements of the
    /// `events` field of a `xcb_input_get_device_motion_events_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_device_motion_events_events_iterator(
        &self,
        r: *const xcb_input_get_device_motion_events_reply_t,
    ) -> xcb_input_device_time_coord_iterator_t {
        sym!(self, xcb_input_get_device_motion_events_events_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_motion_events_events_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_motion_events_events_iterator(&self) -> bool {
        has_sym!(self, xcb_input_get_device_motion_events_events_iterator)
    }

    /// Waits for the reply to a `Input::GetDeviceMotionEvents` request.
    #[inline]
    pub unsafe fn xcb_input_get_device_motion_events_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_get_device_motion_events_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_get_device_motion_events_reply_t {
        sym!(self, xcb_input_get_device_motion_events_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_motion_events_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_motion_events_reply(&self) -> bool {
        has_sym!(self, xcb_input_get_device_motion_events_reply)
    }

    /// Sends a `Input::ChangeKeyboardDevice` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_change_keyboard_device_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_change_keyboard_device_reply`]: Self::xcb_input_change_keyboard_device_reply
    #[inline]
    pub unsafe fn xcb_input_change_keyboard_device(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_change_keyboard_device_cookie_t {
        sym!(self, xcb_input_change_keyboard_device)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_change_keyboard_device` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_keyboard_device(&self) -> bool {
        has_sym!(self, xcb_input_change_keyboard_device)
    }

    /// Sends a `Input::ChangeKeyboardDevice` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_change_keyboard_device_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_change_keyboard_device_reply`]: Self::xcb_input_change_keyboard_device_reply
    #[inline]
    pub unsafe fn xcb_input_change_keyboard_device_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_change_keyboard_device_cookie_t {
        sym!(self, xcb_input_change_keyboard_device_unchecked)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_change_keyboard_device_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_keyboard_device_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_change_keyboard_device_unchecked)
    }

    /// Waits for the reply to a `Input::ChangeKeyboardDevice` request.
    #[inline]
    pub unsafe fn xcb_input_change_keyboard_device_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_change_keyboard_device_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_change_keyboard_device_reply_t {
        sym!(self, xcb_input_change_keyboard_device_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_change_keyboard_device_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_keyboard_device_reply(&self) -> bool {
        has_sym!(self, xcb_input_change_keyboard_device_reply)
    }

    /// Sends a `Input::ChangePointerDevice` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_change_pointer_device_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_change_pointer_device_reply`]: Self::xcb_input_change_pointer_device_reply
    #[inline]
    pub unsafe fn xcb_input_change_pointer_device(
        &self,
        c: *mut xcb_connection_t,
        x_axis: u8,
        y_axis: u8,
        device_id: u8,
    ) -> xcb_input_change_pointer_device_cookie_t {
        sym!(self, xcb_input_change_pointer_device)(c, x_axis, y_axis, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_change_pointer_device` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_pointer_device(&self) -> bool {
        has_sym!(self, xcb_input_change_pointer_device)
    }

    /// Sends a `Input::ChangePointerDevice` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_change_pointer_device_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_change_pointer_device_reply`]: Self::xcb_input_change_pointer_device_reply
    #[inline]
    pub unsafe fn xcb_input_change_pointer_device_unchecked(
        &self,
        c: *mut xcb_connection_t,
        x_axis: u8,
        y_axis: u8,
        device_id: u8,
    ) -> xcb_input_change_pointer_device_cookie_t {
        sym!(self, xcb_input_change_pointer_device_unchecked)(c, x_axis, y_axis, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_change_pointer_device_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_pointer_device_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_change_pointer_device_unchecked)
    }

    /// Waits for the reply to a `Input::ChangePointerDevice` request.
    #[inline]
    pub unsafe fn xcb_input_change_pointer_device_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_change_pointer_device_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_change_pointer_device_reply_t {
        sym!(self, xcb_input_change_pointer_device_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_change_pointer_device_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_pointer_device_reply(&self) -> bool {
        has_sym!(self, xcb_input_change_pointer_device_reply)
    }

    /// Computes the size of a `xcb_input_grab_device_request_t` object.
    #[inline]
    pub unsafe fn xcb_input_grab_device_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_grab_device_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_sizeof)
    }

    /// Sends a `Input::GrabDevice` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_grab_device_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_grab_device_reply`]: Self::xcb_input_grab_device_reply
    #[inline]
    pub unsafe fn xcb_input_grab_device(
        &self,
        c: *mut xcb_connection_t,
        grab_window: xcb_window_t,
        time: xcb_timestamp_t,
        num_classes: u16,
        this_device_mode: u8,
        other_device_mode: u8,
        owner_events: u8,
        device_id: u8,
        classes: *const xcb_input_event_class_t,
    ) -> xcb_input_grab_device_cookie_t {
        sym!(self, xcb_input_grab_device)(
            c,
            grab_window,
            time,
            num_classes,
            this_device_mode,
            other_device_mode,
            owner_events,
            device_id,
            classes,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device(&self) -> bool {
        has_sym!(self, xcb_input_grab_device)
    }

    /// Sends a `Input::GrabDevice` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_grab_device_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_grab_device_reply`]: Self::xcb_input_grab_device_reply
    #[inline]
    pub unsafe fn xcb_input_grab_device_unchecked(
        &self,
        c: *mut xcb_connection_t,
        grab_window: xcb_window_t,
        time: xcb_timestamp_t,
        num_classes: u16,
        this_device_mode: u8,
        other_device_mode: u8,
        owner_events: u8,
        device_id: u8,
        classes: *const xcb_input_event_class_t,
    ) -> xcb_input_grab_device_cookie_t {
        sym!(self, xcb_input_grab_device_unchecked)(
            c,
            grab_window,
            time,
            num_classes,
            this_device_mode,
            other_device_mode,
            owner_events,
            device_id,
            classes,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_unchecked)
    }

    /// Waits for the reply to a `Input::GrabDevice` request.
    #[inline]
    pub unsafe fn xcb_input_grab_device_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_grab_device_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_grab_device_reply_t {
        sym!(self, xcb_input_grab_device_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_reply(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_reply)
    }

    /// Sends a `Input::UngrabDevice` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_input_ungrab_device_checked(
        &self,
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
        device_id: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_ungrab_device_checked)(c, time, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_ungrab_device_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_ungrab_device_checked(&self) -> bool {
        has_sym!(self, xcb_input_ungrab_device_checked)
    }

    /// Sends a `Input::UngrabDevice` request (unchecked).
    #[inline]
    pub unsafe fn xcb_input_ungrab_device(
        &self,
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
        device_id: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_ungrab_device)(c, time, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_ungrab_device` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_ungrab_device(&self) -> bool {
        has_sym!(self, xcb_input_ungrab_device)
    }

    /// Computes the size of a `xcb_input_grab_device_key_request_t` object.
    #[inline]
    pub unsafe fn xcb_input_grab_device_key_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_grab_device_key_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_key_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_key_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_key_sizeof)
    }

    /// Sends a `Input::GrabDeviceKey` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_input_grab_device_key_checked(
        &self,
        c: *mut xcb_connection_t,
        grab_window: xcb_window_t,
        num_classes: u16,
        modifiers: u16,
        modifier_device: u8,
        grabbed_device: u8,
        key: u8,
        this_device_mode: u8,
        other_device_mode: u8,
        owner_events: u8,
        classes: *const xcb_input_event_class_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_grab_device_key_checked)(
            c,
            grab_window,
            num_classes,
            modifiers,
            modifier_device,
            grabbed_device,
            key,
            this_device_mode,
            other_device_mode,
            owner_events,
            classes,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_key_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_key_checked(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_key_checked)
    }

    /// Sends a `Input::GrabDeviceKey` request (unchecked).
    #[inline]
    pub unsafe fn xcb_input_grab_device_key(
        &self,
        c: *mut xcb_connection_t,
        grab_window: xcb_window_t,
        num_classes: u16,
        modifiers: u16,
        modifier_device: u8,
        grabbed_device: u8,
        key: u8,
        this_device_mode: u8,
        other_device_mode: u8,
        owner_events: u8,
        classes: *const xcb_input_event_class_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_grab_device_key)(
            c,
            grab_window,
            num_classes,
            modifiers,
            modifier_device,
            grabbed_device,
            key,
            this_device_mode,
            other_device_mode,
            owner_events,
            classes,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_key` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_key(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_key)
    }

    /// Returns a pointer to the `classes` field of a `xcb_input_grab_device_key_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_grab_device_key_classes(
        &self,
        r: *const xcb_input_grab_device_key_request_t,
    ) -> *mut xcb_input_event_class_t {
        sym!(self, xcb_input_grab_device_key_classes)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_key_classes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_key_classes(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_key_classes)
    }

    /// Returns the number of elements of the `classes` field of a `xcb_input_grab_device_key_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_grab_device_key_classes_length(
        &self,
        r: *const xcb_input_grab_device_key_request_t,
    ) -> c_int {
        sym!(self, xcb_input_grab_device_key_classes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_key_classes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_key_classes_length(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_key_classes_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `classes` field of a `xcb_input_grab_device_key_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_grab_device_key_classes_end(
        &self,
        r: *const xcb_input_grab_device_key_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_grab_device_key_classes_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_key_classes_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_key_classes_end(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_key_classes_end)
    }

    /// Sends a `Input::UngrabDeviceKey` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_input_ungrab_device_key_checked(
        &self,
        c: *mut xcb_connection_t,
        grab_window: xcb_window_t,
        modifiers: u16,
        modifier_device: u8,
        key: u8,
        grabbed_device: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_ungrab_device_key_checked)(
            c,
            grab_window,
            modifiers,
            modifier_device,
            key,
            grabbed_device,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_ungrab_device_key_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_ungrab_device_key_checked(&self) -> bool {
        has_sym!(self, xcb_input_ungrab_device_key_checked)
    }

    /// Sends a `Input::UngrabDeviceKey` request (unchecked).
    #[inline]
    pub unsafe fn xcb_input_ungrab_device_key(
        &self,
        c: *mut xcb_connection_t,
        grab_window: xcb_window_t,
        modifiers: u16,
        modifier_device: u8,
        key: u8,
        grabbed_device: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_ungrab_device_key)(
            c,
            grab_window,
            modifiers,
            modifier_device,
            key,
            grabbed_device,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_ungrab_device_key` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_ungrab_device_key(&self) -> bool {
        has_sym!(self, xcb_input_ungrab_device_key)
    }

    /// Computes the size of a `xcb_input_grab_device_button_request_t` object.
    #[inline]
    pub unsafe fn xcb_input_grab_device_button_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_grab_device_button_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_button_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_button_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_button_sizeof)
    }

    /// Sends a `Input::GrabDeviceButton` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_input_grab_device_button_checked(
        &self,
        c: *mut xcb_connection_t,
        grab_window: xcb_window_t,
        grabbed_device: u8,
        modifier_device: u8,
        num_classes: u16,
        modifiers: u16,
        this_device_mode: u8,
        other_device_mode: u8,
        button: u8,
        owner_events: u8,
        classes: *const xcb_input_event_class_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_grab_device_button_checked)(
            c,
            grab_window,
            grabbed_device,
            modifier_device,
            num_classes,
            modifiers,
            this_device_mode,
            other_device_mode,
            button,
            owner_events,
            classes,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_button_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_button_checked(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_button_checked)
    }

    /// Sends a `Input::GrabDeviceButton` request (unchecked).
    #[inline]
    pub unsafe fn xcb_input_grab_device_button(
        &self,
        c: *mut xcb_connection_t,
        grab_window: xcb_window_t,
        grabbed_device: u8,
        modifier_device: u8,
        num_classes: u16,
        modifiers: u16,
        this_device_mode: u8,
        other_device_mode: u8,
        button: u8,
        owner_events: u8,
        classes: *const xcb_input_event_class_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_grab_device_button)(
            c,
            grab_window,
            grabbed_device,
            modifier_device,
            num_classes,
            modifiers,
            this_device_mode,
            other_device_mode,
            button,
            owner_events,
            classes,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_button` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_button(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_button)
    }

    /// Returns a pointer to the `classes` field of a `xcb_input_grab_device_button_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_grab_device_button_classes(
        &self,
        r: *const xcb_input_grab_device_button_request_t,
    ) -> *mut xcb_input_event_class_t {
        sym!(self, xcb_input_grab_device_button_classes)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_button_classes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_button_classes(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_button_classes)
    }

    /// Returns the number of elements of the `classes` field of a `xcb_input_grab_device_button_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_grab_device_button_classes_length(
        &self,
        r: *const xcb_input_grab_device_button_request_t,
    ) -> c_int {
        sym!(self, xcb_input_grab_device_button_classes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_button_classes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_button_classes_length(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_button_classes_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `classes` field of a `xcb_input_grab_device_button_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_grab_device_button_classes_end(
        &self,
        r: *const xcb_input_grab_device_button_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_grab_device_button_classes_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_grab_device_button_classes_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_device_button_classes_end(&self) -> bool {
        has_sym!(self, xcb_input_grab_device_button_classes_end)
    }

    /// Sends a `Input::UngrabDeviceButton` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_input_ungrab_device_button_checked(
        &self,
        c: *mut xcb_connection_t,
        grab_window: xcb_window_t,
        modifiers: u16,
        modifier_device: u8,
        button: u8,
        grabbed_device: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_ungrab_device_button_checked)(
            c,
            grab_window,
            modifiers,
            modifier_device,
            button,
            grabbed_device,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_ungrab_device_button_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_ungrab_device_button_checked(&self) -> bool {
        has_sym!(self, xcb_input_ungrab_device_button_checked)
    }

    /// Sends a `Input::UngrabDeviceButton` request (unchecked).
    #[inline]
    pub unsafe fn xcb_input_ungrab_device_button(
        &self,
        c: *mut xcb_connection_t,
        grab_window: xcb_window_t,
        modifiers: u16,
        modifier_device: u8,
        button: u8,
        grabbed_device: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_ungrab_device_button)(
            c,
            grab_window,
            modifiers,
            modifier_device,
            button,
            grabbed_device,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_ungrab_device_button` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_ungrab_device_button(&self) -> bool {
        has_sym!(self, xcb_input_ungrab_device_button)
    }

    /// Sends a `Input::AllowDeviceEvents` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_input_allow_device_events_checked(
        &self,
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
        mode: u8,
        device_id: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_allow_device_events_checked)(c, time, mode, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_allow_device_events_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_allow_device_events_checked(&self) -> bool {
        has_sym!(self, xcb_input_allow_device_events_checked)
    }

    /// Sends a `Input::AllowDeviceEvents` request (unchecked).
    #[inline]
    pub unsafe fn xcb_input_allow_device_events(
        &self,
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
        mode: u8,
        device_id: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_allow_device_events)(c, time, mode, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_allow_device_events` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_allow_device_events(&self) -> bool {
        has_sym!(self, xcb_input_allow_device_events)
    }

    /// Sends a `Input::GetDeviceFocus` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_get_device_focus_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_get_device_focus_reply`]: Self::xcb_input_get_device_focus_reply
    #[inline]
    pub unsafe fn xcb_input_get_device_focus(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_get_device_focus_cookie_t {
        sym!(self, xcb_input_get_device_focus)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_focus` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_focus(&self) -> bool {
        has_sym!(self, xcb_input_get_device_focus)
    }

    /// Sends a `Input::GetDeviceFocus` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_get_device_focus_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_get_device_focus_reply`]: Self::xcb_input_get_device_focus_reply
    #[inline]
    pub unsafe fn xcb_input_get_device_focus_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_get_device_focus_cookie_t {
        sym!(self, xcb_input_get_device_focus_unchecked)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_focus_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_focus_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_get_device_focus_unchecked)
    }

    /// Waits for the reply to a `Input::GetDeviceFocus` request.
    #[inline]
    pub unsafe fn xcb_input_get_device_focus_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_get_device_focus_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_get_device_focus_reply_t {
        sym!(self, xcb_input_get_device_focus_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_focus_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_focus_reply(&self) -> bool {
        has_sym!(self, xcb_input_get_device_focus_reply)
    }

    /// Sends a `Input::SetDeviceFocus` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_input_set_device_focus_checked(
        &self,
        c: *mut xcb_connection_t,
        focus: xcb_window_t,
        time: xcb_timestamp_t,
        revert_to: u8,
        device_id: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_set_device_focus_checked)(c, focus, time, revert_to, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_focus_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_focus_checked(&self) -> bool {
        has_sym!(self, xcb_input_set_device_focus_checked)
    }

    /// Sends a `Input::SetDeviceFocus` request (unchecked).
    #[inline]
    pub unsafe fn xcb_input_set_device_focus(
        &self,
        c: *mut xcb_connection_t,
        focus: xcb_window_t,
        time: xcb_timestamp_t,
        revert_to: u8,
        device_id: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_set_device_focus)(c, focus, time, revert_to, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_focus` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_focus(&self) -> bool {
        has_sym!(self, xcb_input_set_device_focus)
    }

    /// Advances a `xcb_input_kbd_feedback_state_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_kbd_feedback_state_next(
        &self,
        i: *mut xcb_input_kbd_feedback_state_iterator_t,
    ) {
        sym!(self, xcb_input_kbd_feedback_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_kbd_feedback_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_kbd_feedback_state_next(&self) -> bool {
        has_sym!(self, xcb_input_kbd_feedback_state_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_kbd_feedback_state_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_kbd_feedback_state_end(
        &self,
        i: xcb_input_kbd_feedback_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_kbd_feedback_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_kbd_feedback_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_kbd_feedback_state_end(&self) -> bool {
        has_sym!(self, xcb_input_kbd_feedback_state_end)
    }

    /// Advances a `xcb_input_ptr_feedback_state_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_ptr_feedback_state_next(
        &self,
        i: *mut xcb_input_ptr_feedback_state_iterator_t,
    ) {
        sym!(self, xcb_input_ptr_feedback_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_ptr_feedback_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_ptr_feedback_state_next(&self) -> bool {
        has_sym!(self, xcb_input_ptr_feedback_state_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_ptr_feedback_state_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_ptr_feedback_state_end(
        &self,
        i: xcb_input_ptr_feedback_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_ptr_feedback_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_ptr_feedback_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_ptr_feedback_state_end(&self) -> bool {
        has_sym!(self, xcb_input_ptr_feedback_state_end)
    }

    /// Advances a `xcb_input_integer_feedback_state_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_integer_feedback_state_next(
        &self,
        i: *mut xcb_input_integer_feedback_state_iterator_t,
    ) {
        sym!(self, xcb_input_integer_feedback_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_integer_feedback_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_integer_feedback_state_next(&self) -> bool {
        has_sym!(self, xcb_input_integer_feedback_state_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_integer_feedback_state_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_integer_feedback_state_end(
        &self,
        i: xcb_input_integer_feedback_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_integer_feedback_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_integer_feedback_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_integer_feedback_state_end(&self) -> bool {
        has_sym!(self, xcb_input_integer_feedback_state_end)
    }

    /// Computes the size of a `xcb_input_string_feedback_state_t` object.
    #[inline]
    pub unsafe fn xcb_input_string_feedback_state_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_string_feedback_state_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_string_feedback_state_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_string_feedback_state_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_string_feedback_state_sizeof)
    }

    /// Returns a pointer to the `keysyms` field of a `xcb_input_string_feedback_state_t` struct.
    #[inline]
    pub unsafe fn xcb_input_string_feedback_state_keysyms(
        &self,
        r: *const xcb_input_string_feedback_state_t,
    ) -> *mut xcb_keysym_t {
        sym!(self, xcb_input_string_feedback_state_keysyms)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_string_feedback_state_keysyms` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_string_feedback_state_keysyms(&self) -> bool {
        has_sym!(self, xcb_input_string_feedback_state_keysyms)
    }

    /// Returns the number of elements of the `keysyms` field of a `xcb_input_string_feedback_state_t` struct.
    #[inline]
    pub unsafe fn xcb_input_string_feedback_state_keysyms_length(
        &self,
        r: *const xcb_input_string_feedback_state_t,
    ) -> c_int {
        sym!(self, xcb_input_string_feedback_state_keysyms_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_string_feedback_state_keysyms_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_string_feedback_state_keysyms_length(&self) -> bool {
        has_sym!(self, xcb_input_string_feedback_state_keysyms_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `keysyms` field of a `xcb_input_string_feedback_state_t` struct.
    #[inline]
    pub unsafe fn xcb_input_string_feedback_state_keysyms_end(
        &self,
        r: *const xcb_input_string_feedback_state_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_string_feedback_state_keysyms_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_string_feedback_state_keysyms_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_string_feedback_state_keysyms_end(&self) -> bool {
        has_sym!(self, xcb_input_string_feedback_state_keysyms_end)
    }

    /// Advances a `xcb_input_string_feedback_state_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_string_feedback_state_next(
        &self,
        i: *mut xcb_input_string_feedback_state_iterator_t,
    ) {
        sym!(self, xcb_input_string_feedback_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_string_feedback_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_string_feedback_state_next(&self) -> bool {
        has_sym!(self, xcb_input_string_feedback_state_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_string_feedback_state_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_string_feedback_state_end(
        &self,
        i: xcb_input_string_feedback_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_string_feedback_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_string_feedback_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_string_feedback_state_end(&self) -> bool {
        has_sym!(self, xcb_input_string_feedback_state_end)
    }

    /// Advances a `xcb_input_bell_feedback_state_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_bell_feedback_state_next(
        &self,
        i: *mut xcb_input_bell_feedback_state_iterator_t,
    ) {
        sym!(self, xcb_input_bell_feedback_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_bell_feedback_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_bell_feedback_state_next(&self) -> bool {
        has_sym!(self, xcb_input_bell_feedback_state_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_bell_feedback_state_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_bell_feedback_state_end(
        &self,
        i: xcb_input_bell_feedback_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_bell_feedback_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_bell_feedback_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_bell_feedback_state_end(&self) -> bool {
        has_sym!(self, xcb_input_bell_feedback_state_end)
    }

    /// Advances a `xcb_input_led_feedback_state_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_led_feedback_state_next(
        &self,
        i: *mut xcb_input_led_feedback_state_iterator_t,
    ) {
        sym!(self, xcb_input_led_feedback_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_led_feedback_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_led_feedback_state_next(&self) -> bool {
        has_sym!(self, xcb_input_led_feedback_state_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_led_feedback_state_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_led_feedback_state_end(
        &self,
        i: xcb_input_led_feedback_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_led_feedback_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_led_feedback_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_led_feedback_state_end(&self) -> bool {
        has_sym!(self, xcb_input_led_feedback_state_end)
    }

    /// Returns a pointer to the `keysyms` field of a `xcb_input_feedback_state_data_string_t` struct.
    #[inline]
    pub unsafe fn xcb_input_feedback_state_data_string_keysyms(
        &self,
        s: *const xcb_input_feedback_state_data_t,
    ) -> *mut xcb_keysym_t {
        sym!(self, xcb_input_feedback_state_data_string_keysyms)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_state_data_string_keysyms` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_state_data_string_keysyms(&self) -> bool {
        has_sym!(self, xcb_input_feedback_state_data_string_keysyms)
    }

    /// Returns the number of elements of the `keysyms` field of a `xcb_input_feedback_state_data_string_t` struct.
    #[inline]
    pub unsafe fn xcb_input_feedback_state_data_string_keysyms_length(
        &self,
        r: *const xcb_input_feedback_state_t,
        s: *const xcb_input_feedback_state_data_t,
    ) -> c_int {
        sym!(self, xcb_input_feedback_state_data_string_keysyms_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_state_data_string_keysyms_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_state_data_string_keysyms_length(&self) -> bool {
        has_sym!(self, xcb_input_feedback_state_data_string_keysyms_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `keysyms` field of a `xcb_input_feedback_state_data_string_t` struct.
    #[inline]
    pub unsafe fn xcb_input_feedback_state_data_string_keysyms_end(
        &self,
        r: *const xcb_input_feedback_state_t,
        s: *const xcb_input_feedback_state_data_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_feedback_state_data_string_keysyms_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_state_data_string_keysyms_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_state_data_string_keysyms_end(&self) -> bool {
        has_sym!(self, xcb_input_feedback_state_data_string_keysyms_end)
    }

    /// Serializes a `xcb_input_feedback_state_data_t` object.
    #[inline]
    pub unsafe fn xcb_input_feedback_state_data_serialize(
        &self,
        _buffer: *mut *mut c_void,
        class_id: u8,
        _aux: *const xcb_input_feedback_state_data_t,
    ) -> c_int {
        sym!(self, xcb_input_feedback_state_data_serialize)(_buffer, class_id, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_state_data_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_state_data_serialize(&self) -> bool {
        has_sym!(self, xcb_input_feedback_state_data_serialize)
    }

    /// Unpacks a `xcb_input_feedback_state_data_t` object.
    #[inline]
    pub unsafe fn xcb_input_feedback_state_data_unpack(
        &self,
        _buffer: *const c_void,
        class_id: u8,
        _aux: *mut xcb_input_feedback_state_data_t,
    ) -> c_int {
        sym!(self, xcb_input_feedback_state_data_unpack)(_buffer, class_id, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_state_data_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_state_data_unpack(&self) -> bool {
        has_sym!(self, xcb_input_feedback_state_data_unpack)
    }

    /// Computes the size of a `xcb_input_feedback_state_data_t` object.
    #[inline]
    pub unsafe fn xcb_input_feedback_state_data_sizeof(
        &self,
        _buffer: *const c_void,
        class_id: u8,
    ) -> c_int {
        sym!(self, xcb_input_feedback_state_data_sizeof)(_buffer, class_id)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_state_data_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_state_data_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_feedback_state_data_sizeof)
    }

    /// Computes the size of a `xcb_input_feedback_state_t` object.
    #[inline]
    pub unsafe fn xcb_input_feedback_state_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_feedback_state_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_state_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_state_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_feedback_state_sizeof)
    }

    /// Returns a pointer to the `data` field of a `xcb_input_feedback_state_t` struct.
    #[inline]
    pub unsafe fn xcb_input_feedback_state_data(
        &self,
        r: *const xcb_input_feedback_state_t,
    ) -> *mut c_void {
        sym!(self, xcb_input_feedback_state_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_state_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_state_data(&self) -> bool {
        has_sym!(self, xcb_input_feedback_state_data)
    }

    /// Advances a `xcb_input_feedback_state_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_feedback_state_next(
        &self,
        i: *mut xcb_input_feedback_state_iterator_t,
    ) {
        sym!(self, xcb_input_feedback_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_state_next(&self) -> bool {
        has_sym!(self, xcb_input_feedback_state_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_feedback_state_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_feedback_state_end(
        &self,
        i: xcb_input_feedback_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_feedback_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_state_end(&self) -> bool {
        has_sym!(self, xcb_input_feedback_state_end)
    }

    /// Computes the size of a `xcb_input_get_feedback_control_reply_t` object.
    #[inline]
    pub unsafe fn xcb_input_get_feedback_control_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_get_feedback_control_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_get_feedback_control_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_feedback_control_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_get_feedback_control_sizeof)
    }

    /// Sends a `Input::GetFeedbackControl` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_get_feedback_control_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_get_feedback_control_reply`]: Self::xcb_input_get_feedback_control_reply
    #[inline]
    pub unsafe fn xcb_input_get_feedback_control(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_get_feedback_control_cookie_t {
        sym!(self, xcb_input_get_feedback_control)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_get_feedback_control` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_feedback_control(&self) -> bool {
        has_sym!(self, xcb_input_get_feedback_control)
    }

    /// Sends a `Input::GetFeedbackControl` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_get_feedback_control_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_get_feedback_control_reply`]: Self::xcb_input_get_feedback_control_reply
    #[inline]
    pub unsafe fn xcb_input_get_feedback_control_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_get_feedback_control_cookie_t {
        sym!(self, xcb_input_get_feedback_control_unchecked)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_get_feedback_control_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_feedback_control_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_get_feedback_control_unchecked)
    }

    /// Returns the number of elements of the `feedbacks` field of a `xcb_input_get_feedback_control_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_feedback_control_feedbacks_length(
        &self,
        r: *const xcb_input_get_feedback_control_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_get_feedback_control_feedbacks_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_feedback_control_feedbacks_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_feedback_control_feedbacks_length(&self) -> bool {
        has_sym!(self, xcb_input_get_feedback_control_feedbacks_length)
    }

    /// Returns an iterator over the elements of the
    /// `feedbacks` field of a `xcb_input_get_feedback_control_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_feedback_control_feedbacks_iterator(
        &self,
        r: *const xcb_input_get_feedback_control_reply_t,
    ) -> xcb_input_feedback_state_iterator_t {
        sym!(self, xcb_input_get_feedback_control_feedbacks_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_feedback_control_feedbacks_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_feedback_control_feedbacks_iterator(&self) -> bool {
        has_sym!(self, xcb_input_get_feedback_control_feedbacks_iterator)
    }

    /// Waits for the reply to a `Input::GetFeedbackControl` request.
    #[inline]
    pub unsafe fn xcb_input_get_feedback_control_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_get_feedback_control_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_get_feedback_control_reply_t {
        sym!(self, xcb_input_get_feedback_control_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_get_feedback_control_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_feedback_control_reply(&self) -> bool {
        has_sym!(self, xcb_input_get_feedback_control_reply)
    }

    /// Advances a `xcb_input_kbd_feedback_ctl_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_kbd_feedback_ctl_next(
        &self,
        i: *mut xcb_input_kbd_feedback_ctl_iterator_t,
    ) {
        sym!(self, xcb_input_kbd_feedback_ctl_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_kbd_feedback_ctl_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_kbd_feedback_ctl_next(&self) -> bool {
        has_sym!(self, xcb_input_kbd_feedback_ctl_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_kbd_feedback_ctl_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_kbd_feedback_ctl_end(
        &self,
        i: xcb_input_kbd_feedback_ctl_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_kbd_feedback_ctl_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_kbd_feedback_ctl_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_kbd_feedback_ctl_end(&self) -> bool {
        has_sym!(self, xcb_input_kbd_feedback_ctl_end)
    }

    /// Advances a `xcb_input_ptr_feedback_ctl_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_ptr_feedback_ctl_next(
        &self,
        i: *mut xcb_input_ptr_feedback_ctl_iterator_t,
    ) {
        sym!(self, xcb_input_ptr_feedback_ctl_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_ptr_feedback_ctl_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_ptr_feedback_ctl_next(&self) -> bool {
        has_sym!(self, xcb_input_ptr_feedback_ctl_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_ptr_feedback_ctl_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_ptr_feedback_ctl_end(
        &self,
        i: xcb_input_ptr_feedback_ctl_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_ptr_feedback_ctl_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_ptr_feedback_ctl_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_ptr_feedback_ctl_end(&self) -> bool {
        has_sym!(self, xcb_input_ptr_feedback_ctl_end)
    }

    /// Advances a `xcb_input_integer_feedback_ctl_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_integer_feedback_ctl_next(
        &self,
        i: *mut xcb_input_integer_feedback_ctl_iterator_t,
    ) {
        sym!(self, xcb_input_integer_feedback_ctl_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_integer_feedback_ctl_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_integer_feedback_ctl_next(&self) -> bool {
        has_sym!(self, xcb_input_integer_feedback_ctl_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_integer_feedback_ctl_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_integer_feedback_ctl_end(
        &self,
        i: xcb_input_integer_feedback_ctl_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_integer_feedback_ctl_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_integer_feedback_ctl_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_integer_feedback_ctl_end(&self) -> bool {
        has_sym!(self, xcb_input_integer_feedback_ctl_end)
    }

    /// Computes the size of a `xcb_input_string_feedback_ctl_t` object.
    #[inline]
    pub unsafe fn xcb_input_string_feedback_ctl_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_string_feedback_ctl_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_string_feedback_ctl_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_string_feedback_ctl_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_string_feedback_ctl_sizeof)
    }

    /// Returns a pointer to the `keysyms` field of a `xcb_input_string_feedback_ctl_t` struct.
    #[inline]
    pub unsafe fn xcb_input_string_feedback_ctl_keysyms(
        &self,
        r: *const xcb_input_string_feedback_ctl_t,
    ) -> *mut xcb_keysym_t {
        sym!(self, xcb_input_string_feedback_ctl_keysyms)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_string_feedback_ctl_keysyms` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_string_feedback_ctl_keysyms(&self) -> bool {
        has_sym!(self, xcb_input_string_feedback_ctl_keysyms)
    }

    /// Returns the number of elements of the `keysyms` field of a `xcb_input_string_feedback_ctl_t` struct.
    #[inline]
    pub unsafe fn xcb_input_string_feedback_ctl_keysyms_length(
        &self,
        r: *const xcb_input_string_feedback_ctl_t,
    ) -> c_int {
        sym!(self, xcb_input_string_feedback_ctl_keysyms_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_string_feedback_ctl_keysyms_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_string_feedback_ctl_keysyms_length(&self) -> bool {
        has_sym!(self, xcb_input_string_feedback_ctl_keysyms_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `keysyms` field of a `xcb_input_string_feedback_ctl_t` struct.
    #[inline]
    pub unsafe fn xcb_input_string_feedback_ctl_keysyms_end(
        &self,
        r: *const xcb_input_string_feedback_ctl_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_string_feedback_ctl_keysyms_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_string_feedback_ctl_keysyms_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_string_feedback_ctl_keysyms_end(&self) -> bool {
        has_sym!(self, xcb_input_string_feedback_ctl_keysyms_end)
    }

    /// Advances a `xcb_input_string_feedback_ctl_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_string_feedback_ctl_next(
        &self,
        i: *mut xcb_input_string_feedback_ctl_iterator_t,
    ) {
        sym!(self, xcb_input_string_feedback_ctl_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_string_feedback_ctl_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_string_feedback_ctl_next(&self) -> bool {
        has_sym!(self, xcb_input_string_feedback_ctl_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_string_feedback_ctl_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_string_feedback_ctl_end(
        &self,
        i: xcb_input_string_feedback_ctl_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_string_feedback_ctl_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_string_feedback_ctl_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_string_feedback_ctl_end(&self) -> bool {
        has_sym!(self, xcb_input_string_feedback_ctl_end)
    }

    /// Advances a `xcb_input_bell_feedback_ctl_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_bell_feedback_ctl_next(
        &self,
        i: *mut xcb_input_bell_feedback_ctl_iterator_t,
    ) {
        sym!(self, xcb_input_bell_feedback_ctl_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_bell_feedback_ctl_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_bell_feedback_ctl_next(&self) -> bool {
        has_sym!(self, xcb_input_bell_feedback_ctl_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_bell_feedback_ctl_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_bell_feedback_ctl_end(
        &self,
        i: xcb_input_bell_feedback_ctl_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_bell_feedback_ctl_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_bell_feedback_ctl_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_bell_feedback_ctl_end(&self) -> bool {
        has_sym!(self, xcb_input_bell_feedback_ctl_end)
    }

    /// Advances a `xcb_input_led_feedback_ctl_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_led_feedback_ctl_next(
        &self,
        i: *mut xcb_input_led_feedback_ctl_iterator_t,
    ) {
        sym!(self, xcb_input_led_feedback_ctl_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_led_feedback_ctl_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_led_feedback_ctl_next(&self) -> bool {
        has_sym!(self, xcb_input_led_feedback_ctl_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_led_feedback_ctl_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_led_feedback_ctl_end(
        &self,
        i: xcb_input_led_feedback_ctl_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_led_feedback_ctl_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_led_feedback_ctl_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_led_feedback_ctl_end(&self) -> bool {
        has_sym!(self, xcb_input_led_feedback_ctl_end)
    }

    /// Returns a pointer to the `keysyms` field of a `xcb_input_feedback_ctl_data_string_t` struct.
    #[inline]
    pub unsafe fn xcb_input_feedback_ctl_data_string_keysyms(
        &self,
        s: *const xcb_input_feedback_ctl_data_t,
    ) -> *mut xcb_keysym_t {
        sym!(self, xcb_input_feedback_ctl_data_string_keysyms)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_ctl_data_string_keysyms` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_ctl_data_string_keysyms(&self) -> bool {
        has_sym!(self, xcb_input_feedback_ctl_data_string_keysyms)
    }

    /// Returns the number of elements of the `keysyms` field of a `xcb_input_feedback_ctl_data_string_t` struct.
    #[inline]
    pub unsafe fn xcb_input_feedback_ctl_data_string_keysyms_length(
        &self,
        r: *const xcb_input_feedback_ctl_t,
        s: *const xcb_input_feedback_ctl_data_t,
    ) -> c_int {
        sym!(self, xcb_input_feedback_ctl_data_string_keysyms_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_ctl_data_string_keysyms_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_ctl_data_string_keysyms_length(&self) -> bool {
        has_sym!(self, xcb_input_feedback_ctl_data_string_keysyms_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `keysyms` field of a `xcb_input_feedback_ctl_data_string_t` struct.
    #[inline]
    pub unsafe fn xcb_input_feedback_ctl_data_string_keysyms_end(
        &self,
        r: *const xcb_input_feedback_ctl_t,
        s: *const xcb_input_feedback_ctl_data_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_feedback_ctl_data_string_keysyms_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_ctl_data_string_keysyms_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_ctl_data_string_keysyms_end(&self) -> bool {
        has_sym!(self, xcb_input_feedback_ctl_data_string_keysyms_end)
    }

    /// Serializes a `xcb_input_feedback_ctl_data_t` object.
    #[inline]
    pub unsafe fn xcb_input_feedback_ctl_data_serialize(
        &self,
        _buffer: *mut *mut c_void,
        class_id: u8,
        _aux: *const xcb_input_feedback_ctl_data_t,
    ) -> c_int {
        sym!(self, xcb_input_feedback_ctl_data_serialize)(_buffer, class_id, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_ctl_data_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_ctl_data_serialize(&self) -> bool {
        has_sym!(self, xcb_input_feedback_ctl_data_serialize)
    }

    /// Unpacks a `xcb_input_feedback_ctl_data_t` object.
    #[inline]
    pub unsafe fn xcb_input_feedback_ctl_data_unpack(
        &self,
        _buffer: *const c_void,
        class_id: u8,
        _aux: *mut xcb_input_feedback_ctl_data_t,
    ) -> c_int {
        sym!(self, xcb_input_feedback_ctl_data_unpack)(_buffer, class_id, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_ctl_data_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_ctl_data_unpack(&self) -> bool {
        has_sym!(self, xcb_input_feedback_ctl_data_unpack)
    }

    /// Computes the size of a `xcb_input_feedback_ctl_data_t` object.
    #[inline]
    pub unsafe fn xcb_input_feedback_ctl_data_sizeof(
        &self,
        _buffer: *const c_void,
        class_id: u8,
    ) -> c_int {
        sym!(self, xcb_input_feedback_ctl_data_sizeof)(_buffer, class_id)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_ctl_data_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_ctl_data_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_feedback_ctl_data_sizeof)
    }

    /// Computes the size of a `xcb_input_feedback_ctl_t` object.
    #[inline]
    pub unsafe fn xcb_input_feedback_ctl_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_feedback_ctl_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_ctl_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_ctl_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_feedback_ctl_sizeof)
    }

    /// Returns a pointer to the `data` field of a `xcb_input_feedback_ctl_t` struct.
    #[inline]
    pub unsafe fn xcb_input_feedback_ctl_data(
        &self,
        r: *const xcb_input_feedback_ctl_t,
    ) -> *mut c_void {
        sym!(self, xcb_input_feedback_ctl_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_ctl_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_ctl_data(&self) -> bool {
        has_sym!(self, xcb_input_feedback_ctl_data)
    }

    /// Advances a `xcb_input_feedback_ctl_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_feedback_ctl_next(&self, i: *mut xcb_input_feedback_ctl_iterator_t) {
        sym!(self, xcb_input_feedback_ctl_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_ctl_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_ctl_next(&self) -> bool {
        has_sym!(self, xcb_input_feedback_ctl_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_feedback_ctl_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_feedback_ctl_end(
        &self,
        i: xcb_input_feedback_ctl_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_feedback_ctl_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_feedback_ctl_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_feedback_ctl_end(&self) -> bool {
        has_sym!(self, xcb_input_feedback_ctl_end)
    }

    /// Computes the size of a `xcb_input_change_feedback_control_request_t` object.
    #[inline]
    pub unsafe fn xcb_input_change_feedback_control_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_change_feedback_control_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_change_feedback_control_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_feedback_control_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_change_feedback_control_sizeof)
    }

    /// Sends a `Input::ChangeFeedbackControl` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_input_change_feedback_control_checked(
        &self,
        c: *mut xcb_connection_t,
        mask: u32,
        device_id: u8,
        feedback_id: u8,
        feedback: *mut xcb_input_feedback_ctl_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_change_feedback_control_checked)(
            c,
            mask,
            device_id,
            feedback_id,
            feedback,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_change_feedback_control_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_feedback_control_checked(&self) -> bool {
        has_sym!(self, xcb_input_change_feedback_control_checked)
    }

    /// Sends a `Input::ChangeFeedbackControl` request (unchecked).
    #[inline]
    pub unsafe fn xcb_input_change_feedback_control(
        &self,
        c: *mut xcb_connection_t,
        mask: u32,
        device_id: u8,
        feedback_id: u8,
        feedback: *mut xcb_input_feedback_ctl_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_change_feedback_control)(c, mask, device_id, feedback_id, feedback)
    }

    /// Returns `true` iff the symbol `xcb_input_change_feedback_control` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_feedback_control(&self) -> bool {
        has_sym!(self, xcb_input_change_feedback_control)
    }

    /// Returns a pointer to the `feedback` field of a `xcb_input_change_feedback_control_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_change_feedback_control_feedback(
        &self,
        r: *const xcb_input_change_feedback_control_request_t,
    ) -> *mut xcb_input_feedback_ctl_t {
        sym!(self, xcb_input_change_feedback_control_feedback)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_change_feedback_control_feedback` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_feedback_control_feedback(&self) -> bool {
        has_sym!(self, xcb_input_change_feedback_control_feedback)
    }

    /// Computes the size of a `xcb_input_get_device_key_mapping_reply_t` object.
    #[inline]
    pub unsafe fn xcb_input_get_device_key_mapping_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_get_device_key_mapping_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_key_mapping_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_key_mapping_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_get_device_key_mapping_sizeof)
    }

    /// Sends a `Input::GetDeviceKeyMapping` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_get_device_key_mapping_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_get_device_key_mapping_reply`]: Self::xcb_input_get_device_key_mapping_reply
    #[inline]
    pub unsafe fn xcb_input_get_device_key_mapping(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
        first_keycode: xcb_input_key_code_t,
        count: u8,
    ) -> xcb_input_get_device_key_mapping_cookie_t {
        sym!(self, xcb_input_get_device_key_mapping)(c, device_id, first_keycode, count)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_key_mapping` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_key_mapping(&self) -> bool {
        has_sym!(self, xcb_input_get_device_key_mapping)
    }

    /// Sends a `Input::GetDeviceKeyMapping` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_get_device_key_mapping_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_get_device_key_mapping_reply`]: Self::xcb_input_get_device_key_mapping_reply
    #[inline]
    pub unsafe fn xcb_input_get_device_key_mapping_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
        first_keycode: xcb_input_key_code_t,
        count: u8,
    ) -> xcb_input_get_device_key_mapping_cookie_t {
        sym!(self, xcb_input_get_device_key_mapping_unchecked)(c, device_id, first_keycode, count)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_key_mapping_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_key_mapping_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_get_device_key_mapping_unchecked)
    }

    /// Returns a pointer to the `keysyms` field of a `xcb_input_get_device_key_mapping_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_device_key_mapping_keysyms(
        &self,
        r: *const xcb_input_get_device_key_mapping_reply_t,
    ) -> *mut xcb_keysym_t {
        sym!(self, xcb_input_get_device_key_mapping_keysyms)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_key_mapping_keysyms` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_key_mapping_keysyms(&self) -> bool {
        has_sym!(self, xcb_input_get_device_key_mapping_keysyms)
    }

    /// Returns the number of elements of the `keysyms` field of a `xcb_input_get_device_key_mapping_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_device_key_mapping_keysyms_length(
        &self,
        r: *const xcb_input_get_device_key_mapping_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_get_device_key_mapping_keysyms_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_key_mapping_keysyms_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_key_mapping_keysyms_length(&self) -> bool {
        has_sym!(self, xcb_input_get_device_key_mapping_keysyms_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `keysyms` field of a `xcb_input_get_device_key_mapping_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_device_key_mapping_keysyms_end(
        &self,
        r: *const xcb_input_get_device_key_mapping_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_get_device_key_mapping_keysyms_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_key_mapping_keysyms_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_key_mapping_keysyms_end(&self) -> bool {
        has_sym!(self, xcb_input_get_device_key_mapping_keysyms_end)
    }

    /// Waits for the reply to a `Input::GetDeviceKeyMapping` request.
    #[inline]
    pub unsafe fn xcb_input_get_device_key_mapping_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_get_device_key_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_get_device_key_mapping_reply_t {
        sym!(self, xcb_input_get_device_key_mapping_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_key_mapping_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_key_mapping_reply(&self) -> bool {
        has_sym!(self, xcb_input_get_device_key_mapping_reply)
    }

    /// Computes the size of a `xcb_input_change_device_key_mapping_request_t` object.
    #[inline]
    pub unsafe fn xcb_input_change_device_key_mapping_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_input_change_device_key_mapping_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_key_mapping_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_key_mapping_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_change_device_key_mapping_sizeof)
    }

    /// Sends a `Input::ChangeDeviceKeyMapping` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_input_change_device_key_mapping_checked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
        first_keycode: xcb_input_key_code_t,
        keysyms_per_keycode: u8,
        keycode_count: u8,
        keysyms: *const xcb_keysym_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_change_device_key_mapping_checked)(
            c,
            device_id,
            first_keycode,
            keysyms_per_keycode,
            keycode_count,
            keysyms,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_key_mapping_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_key_mapping_checked(&self) -> bool {
        has_sym!(self, xcb_input_change_device_key_mapping_checked)
    }

    /// Sends a `Input::ChangeDeviceKeyMapping` request (unchecked).
    #[inline]
    pub unsafe fn xcb_input_change_device_key_mapping(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
        first_keycode: xcb_input_key_code_t,
        keysyms_per_keycode: u8,
        keycode_count: u8,
        keysyms: *const xcb_keysym_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_change_device_key_mapping)(
            c,
            device_id,
            first_keycode,
            keysyms_per_keycode,
            keycode_count,
            keysyms,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_key_mapping` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_key_mapping(&self) -> bool {
        has_sym!(self, xcb_input_change_device_key_mapping)
    }

    /// Returns a pointer to the `keysyms` field of a `xcb_input_change_device_key_mapping_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_change_device_key_mapping_keysyms(
        &self,
        r: *const xcb_input_change_device_key_mapping_request_t,
    ) -> *mut xcb_keysym_t {
        sym!(self, xcb_input_change_device_key_mapping_keysyms)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_key_mapping_keysyms` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_key_mapping_keysyms(&self) -> bool {
        has_sym!(self, xcb_input_change_device_key_mapping_keysyms)
    }

    /// Returns the number of elements of the `keysyms` field of a `xcb_input_change_device_key_mapping_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_change_device_key_mapping_keysyms_length(
        &self,
        r: *const xcb_input_change_device_key_mapping_request_t,
    ) -> c_int {
        sym!(self, xcb_input_change_device_key_mapping_keysyms_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_key_mapping_keysyms_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_key_mapping_keysyms_length(&self) -> bool {
        has_sym!(self, xcb_input_change_device_key_mapping_keysyms_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `keysyms` field of a `xcb_input_change_device_key_mapping_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_change_device_key_mapping_keysyms_end(
        &self,
        r: *const xcb_input_change_device_key_mapping_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_change_device_key_mapping_keysyms_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_key_mapping_keysyms_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_key_mapping_keysyms_end(&self) -> bool {
        has_sym!(self, xcb_input_change_device_key_mapping_keysyms_end)
    }

    /// Computes the size of a `xcb_input_get_device_modifier_mapping_reply_t` object.
    #[inline]
    pub unsafe fn xcb_input_get_device_modifier_mapping_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_input_get_device_modifier_mapping_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_modifier_mapping_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_modifier_mapping_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_get_device_modifier_mapping_sizeof)
    }

    /// Sends a `Input::GetDeviceModifierMapping` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_get_device_modifier_mapping_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_get_device_modifier_mapping_reply`]: Self::xcb_input_get_device_modifier_mapping_reply
    #[inline]
    pub unsafe fn xcb_input_get_device_modifier_mapping(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_get_device_modifier_mapping_cookie_t {
        sym!(self, xcb_input_get_device_modifier_mapping)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_modifier_mapping` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_modifier_mapping(&self) -> bool {
        has_sym!(self, xcb_input_get_device_modifier_mapping)
    }

    /// Sends a `Input::GetDeviceModifierMapping` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_get_device_modifier_mapping_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_get_device_modifier_mapping_reply`]: Self::xcb_input_get_device_modifier_mapping_reply
    #[inline]
    pub unsafe fn xcb_input_get_device_modifier_mapping_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_get_device_modifier_mapping_cookie_t {
        sym!(self, xcb_input_get_device_modifier_mapping_unchecked)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_modifier_mapping_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_modifier_mapping_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_get_device_modifier_mapping_unchecked)
    }

    /// Returns a pointer to the `keymaps` field of a `xcb_input_get_device_modifier_mapping_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_device_modifier_mapping_keymaps(
        &self,
        r: *const xcb_input_get_device_modifier_mapping_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_input_get_device_modifier_mapping_keymaps)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_modifier_mapping_keymaps` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_modifier_mapping_keymaps(&self) -> bool {
        has_sym!(self, xcb_input_get_device_modifier_mapping_keymaps)
    }

    /// Returns the number of elements of the `keymaps` field of a `xcb_input_get_device_modifier_mapping_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_device_modifier_mapping_keymaps_length(
        &self,
        r: *const xcb_input_get_device_modifier_mapping_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_get_device_modifier_mapping_keymaps_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_modifier_mapping_keymaps_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_modifier_mapping_keymaps_length(&self) -> bool {
        has_sym!(self, xcb_input_get_device_modifier_mapping_keymaps_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `keymaps` field of a `xcb_input_get_device_modifier_mapping_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_device_modifier_mapping_keymaps_end(
        &self,
        r: *const xcb_input_get_device_modifier_mapping_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_get_device_modifier_mapping_keymaps_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_modifier_mapping_keymaps_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_modifier_mapping_keymaps_end(&self) -> bool {
        has_sym!(self, xcb_input_get_device_modifier_mapping_keymaps_end)
    }

    /// Waits for the reply to a `Input::GetDeviceModifierMapping` request.
    #[inline]
    pub unsafe fn xcb_input_get_device_modifier_mapping_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_get_device_modifier_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_get_device_modifier_mapping_reply_t {
        sym!(self, xcb_input_get_device_modifier_mapping_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_modifier_mapping_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_modifier_mapping_reply(&self) -> bool {
        has_sym!(self, xcb_input_get_device_modifier_mapping_reply)
    }

    /// Computes the size of a `xcb_input_set_device_modifier_mapping_request_t` object.
    #[inline]
    pub unsafe fn xcb_input_set_device_modifier_mapping_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_input_set_device_modifier_mapping_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_modifier_mapping_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_modifier_mapping_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_set_device_modifier_mapping_sizeof)
    }

    /// Sends a `Input::SetDeviceModifierMapping` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_set_device_modifier_mapping_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_set_device_modifier_mapping_reply`]: Self::xcb_input_set_device_modifier_mapping_reply
    #[inline]
    pub unsafe fn xcb_input_set_device_modifier_mapping(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
        keycodes_per_modifier: u8,
        keymaps: *const u8,
    ) -> xcb_input_set_device_modifier_mapping_cookie_t {
        sym!(self, xcb_input_set_device_modifier_mapping)(
            c,
            device_id,
            keycodes_per_modifier,
            keymaps,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_modifier_mapping` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_modifier_mapping(&self) -> bool {
        has_sym!(self, xcb_input_set_device_modifier_mapping)
    }

    /// Sends a `Input::SetDeviceModifierMapping` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_set_device_modifier_mapping_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_set_device_modifier_mapping_reply`]: Self::xcb_input_set_device_modifier_mapping_reply
    #[inline]
    pub unsafe fn xcb_input_set_device_modifier_mapping_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
        keycodes_per_modifier: u8,
        keymaps: *const u8,
    ) -> xcb_input_set_device_modifier_mapping_cookie_t {
        sym!(self, xcb_input_set_device_modifier_mapping_unchecked)(
            c,
            device_id,
            keycodes_per_modifier,
            keymaps,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_modifier_mapping_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_modifier_mapping_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_set_device_modifier_mapping_unchecked)
    }

    /// Waits for the reply to a `Input::SetDeviceModifierMapping` request.
    #[inline]
    pub unsafe fn xcb_input_set_device_modifier_mapping_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_set_device_modifier_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_set_device_modifier_mapping_reply_t {
        sym!(self, xcb_input_set_device_modifier_mapping_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_modifier_mapping_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_modifier_mapping_reply(&self) -> bool {
        has_sym!(self, xcb_input_set_device_modifier_mapping_reply)
    }

    /// Computes the size of a `xcb_input_get_device_button_mapping_reply_t` object.
    #[inline]
    pub unsafe fn xcb_input_get_device_button_mapping_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_input_get_device_button_mapping_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_button_mapping_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_button_mapping_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_get_device_button_mapping_sizeof)
    }

    /// Sends a `Input::GetDeviceButtonMapping` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_get_device_button_mapping_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_get_device_button_mapping_reply`]: Self::xcb_input_get_device_button_mapping_reply
    #[inline]
    pub unsafe fn xcb_input_get_device_button_mapping(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_get_device_button_mapping_cookie_t {
        sym!(self, xcb_input_get_device_button_mapping)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_button_mapping` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_button_mapping(&self) -> bool {
        has_sym!(self, xcb_input_get_device_button_mapping)
    }

    /// Sends a `Input::GetDeviceButtonMapping` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_get_device_button_mapping_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_get_device_button_mapping_reply`]: Self::xcb_input_get_device_button_mapping_reply
    #[inline]
    pub unsafe fn xcb_input_get_device_button_mapping_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_get_device_button_mapping_cookie_t {
        sym!(self, xcb_input_get_device_button_mapping_unchecked)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_button_mapping_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_button_mapping_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_get_device_button_mapping_unchecked)
    }

    /// Returns a pointer to the `map` field of a `xcb_input_get_device_button_mapping_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_device_button_mapping_map(
        &self,
        r: *const xcb_input_get_device_button_mapping_reply_t,
    ) -> *mut u8 {
        sym!(self, xcb_input_get_device_button_mapping_map)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_button_mapping_map` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_button_mapping_map(&self) -> bool {
        has_sym!(self, xcb_input_get_device_button_mapping_map)
    }

    /// Returns the number of elements of the `map` field of a `xcb_input_get_device_button_mapping_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_device_button_mapping_map_length(
        &self,
        r: *const xcb_input_get_device_button_mapping_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_get_device_button_mapping_map_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_button_mapping_map_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_button_mapping_map_length(&self) -> bool {
        has_sym!(self, xcb_input_get_device_button_mapping_map_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `map` field of a `xcb_input_get_device_button_mapping_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_device_button_mapping_map_end(
        &self,
        r: *const xcb_input_get_device_button_mapping_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_get_device_button_mapping_map_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_button_mapping_map_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_button_mapping_map_end(&self) -> bool {
        has_sym!(self, xcb_input_get_device_button_mapping_map_end)
    }

    /// Waits for the reply to a `Input::GetDeviceButtonMapping` request.
    #[inline]
    pub unsafe fn xcb_input_get_device_button_mapping_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_get_device_button_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_get_device_button_mapping_reply_t {
        sym!(self, xcb_input_get_device_button_mapping_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_button_mapping_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_button_mapping_reply(&self) -> bool {
        has_sym!(self, xcb_input_get_device_button_mapping_reply)
    }

    /// Computes the size of a `xcb_input_set_device_button_mapping_request_t` object.
    #[inline]
    pub unsafe fn xcb_input_set_device_button_mapping_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_input_set_device_button_mapping_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_button_mapping_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_button_mapping_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_set_device_button_mapping_sizeof)
    }

    /// Sends a `Input::SetDeviceButtonMapping` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_set_device_button_mapping_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_set_device_button_mapping_reply`]: Self::xcb_input_set_device_button_mapping_reply
    #[inline]
    pub unsafe fn xcb_input_set_device_button_mapping(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
        map_size: u8,
        map: *const u8,
    ) -> xcb_input_set_device_button_mapping_cookie_t {
        sym!(self, xcb_input_set_device_button_mapping)(c, device_id, map_size, map)
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_button_mapping` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_button_mapping(&self) -> bool {
        has_sym!(self, xcb_input_set_device_button_mapping)
    }

    /// Sends a `Input::SetDeviceButtonMapping` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_set_device_button_mapping_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_set_device_button_mapping_reply`]: Self::xcb_input_set_device_button_mapping_reply
    #[inline]
    pub unsafe fn xcb_input_set_device_button_mapping_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
        map_size: u8,
        map: *const u8,
    ) -> xcb_input_set_device_button_mapping_cookie_t {
        sym!(self, xcb_input_set_device_button_mapping_unchecked)(c, device_id, map_size, map)
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_button_mapping_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_button_mapping_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_set_device_button_mapping_unchecked)
    }

    /// Waits for the reply to a `Input::SetDeviceButtonMapping` request.
    #[inline]
    pub unsafe fn xcb_input_set_device_button_mapping_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_set_device_button_mapping_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_set_device_button_mapping_reply_t {
        sym!(self, xcb_input_set_device_button_mapping_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_button_mapping_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_button_mapping_reply(&self) -> bool {
        has_sym!(self, xcb_input_set_device_button_mapping_reply)
    }

    /// Advances a `xcb_input_key_state_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_key_state_next(&self, i: *mut xcb_input_key_state_iterator_t) {
        sym!(self, xcb_input_key_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_key_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_state_next(&self) -> bool {
        has_sym!(self, xcb_input_key_state_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_key_state_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_key_state_end(
        &self,
        i: xcb_input_key_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_key_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_key_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_state_end(&self) -> bool {
        has_sym!(self, xcb_input_key_state_end)
    }

    /// Advances a `xcb_input_button_state_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_button_state_next(&self, i: *mut xcb_input_button_state_iterator_t) {
        sym!(self, xcb_input_button_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_button_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_state_next(&self) -> bool {
        has_sym!(self, xcb_input_button_state_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_button_state_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_button_state_end(
        &self,
        i: xcb_input_button_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_button_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_button_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_state_end(&self) -> bool {
        has_sym!(self, xcb_input_button_state_end)
    }

    /// Computes the size of a `xcb_input_valuator_state_t` object.
    #[inline]
    pub unsafe fn xcb_input_valuator_state_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_valuator_state_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_valuator_state_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_valuator_state_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_valuator_state_sizeof)
    }

    /// Returns a pointer to the `valuators` field of a `xcb_input_valuator_state_t` struct.
    #[inline]
    pub unsafe fn xcb_input_valuator_state_valuators(
        &self,
        r: *const xcb_input_valuator_state_t,
    ) -> *mut i32 {
        sym!(self, xcb_input_valuator_state_valuators)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_valuator_state_valuators` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_valuator_state_valuators(&self) -> bool {
        has_sym!(self, xcb_input_valuator_state_valuators)
    }

    /// Returns the number of elements of the `valuators` field of a `xcb_input_valuator_state_t` struct.
    #[inline]
    pub unsafe fn xcb_input_valuator_state_valuators_length(
        &self,
        r: *const xcb_input_valuator_state_t,
    ) -> c_int {
        sym!(self, xcb_input_valuator_state_valuators_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_valuator_state_valuators_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_valuator_state_valuators_length(&self) -> bool {
        has_sym!(self, xcb_input_valuator_state_valuators_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `valuators` field of a `xcb_input_valuator_state_t` struct.
    #[inline]
    pub unsafe fn xcb_input_valuator_state_valuators_end(
        &self,
        r: *const xcb_input_valuator_state_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_valuator_state_valuators_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_valuator_state_valuators_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_valuator_state_valuators_end(&self) -> bool {
        has_sym!(self, xcb_input_valuator_state_valuators_end)
    }

    /// Advances a `xcb_input_valuator_state_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_valuator_state_next(
        &self,
        i: *mut xcb_input_valuator_state_iterator_t,
    ) {
        sym!(self, xcb_input_valuator_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_valuator_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_valuator_state_next(&self) -> bool {
        has_sym!(self, xcb_input_valuator_state_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_valuator_state_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_valuator_state_end(
        &self,
        i: xcb_input_valuator_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_valuator_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_valuator_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_valuator_state_end(&self) -> bool {
        has_sym!(self, xcb_input_valuator_state_end)
    }

    /// Returns a pointer to the `valuators` field of a `xcb_input_input_state_data_valuator_t` struct.
    #[inline]
    pub unsafe fn xcb_input_input_state_data_valuator_valuators(
        &self,
        s: *const xcb_input_input_state_data_t,
    ) -> *mut i32 {
        sym!(self, xcb_input_input_state_data_valuator_valuators)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_input_state_data_valuator_valuators` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_state_data_valuator_valuators(&self) -> bool {
        has_sym!(self, xcb_input_input_state_data_valuator_valuators)
    }

    /// Returns the number of elements of the `valuators` field of a `xcb_input_input_state_data_valuator_t` struct.
    #[inline]
    pub unsafe fn xcb_input_input_state_data_valuator_valuators_length(
        &self,
        r: *const xcb_input_input_state_t,
        s: *const xcb_input_input_state_data_t,
    ) -> c_int {
        sym!(self, xcb_input_input_state_data_valuator_valuators_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_input_state_data_valuator_valuators_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_state_data_valuator_valuators_length(&self) -> bool {
        has_sym!(self, xcb_input_input_state_data_valuator_valuators_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `valuators` field of a `xcb_input_input_state_data_valuator_t` struct.
    #[inline]
    pub unsafe fn xcb_input_input_state_data_valuator_valuators_end(
        &self,
        r: *const xcb_input_input_state_t,
        s: *const xcb_input_input_state_data_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_input_state_data_valuator_valuators_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_input_state_data_valuator_valuators_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_state_data_valuator_valuators_end(&self) -> bool {
        has_sym!(self, xcb_input_input_state_data_valuator_valuators_end)
    }

    /// Serializes a `xcb_input_input_state_data_t` object.
    #[inline]
    pub unsafe fn xcb_input_input_state_data_serialize(
        &self,
        _buffer: *mut *mut c_void,
        class_id: u8,
        _aux: *const xcb_input_input_state_data_t,
    ) -> c_int {
        sym!(self, xcb_input_input_state_data_serialize)(_buffer, class_id, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_input_state_data_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_state_data_serialize(&self) -> bool {
        has_sym!(self, xcb_input_input_state_data_serialize)
    }

    /// Unpacks a `xcb_input_input_state_data_t` object.
    #[inline]
    pub unsafe fn xcb_input_input_state_data_unpack(
        &self,
        _buffer: *const c_void,
        class_id: u8,
        _aux: *mut xcb_input_input_state_data_t,
    ) -> c_int {
        sym!(self, xcb_input_input_state_data_unpack)(_buffer, class_id, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_input_state_data_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_state_data_unpack(&self) -> bool {
        has_sym!(self, xcb_input_input_state_data_unpack)
    }

    /// Computes the size of a `xcb_input_input_state_data_t` object.
    #[inline]
    pub unsafe fn xcb_input_input_state_data_sizeof(
        &self,
        _buffer: *const c_void,
        class_id: u8,
    ) -> c_int {
        sym!(self, xcb_input_input_state_data_sizeof)(_buffer, class_id)
    }

    /// Returns `true` iff the symbol `xcb_input_input_state_data_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_state_data_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_input_state_data_sizeof)
    }

    /// Computes the size of a `xcb_input_input_state_t` object.
    #[inline]
    pub unsafe fn xcb_input_input_state_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_input_state_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_input_state_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_state_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_input_state_sizeof)
    }

    /// Returns a pointer to the `data` field of a `xcb_input_input_state_t` struct.
    #[inline]
    pub unsafe fn xcb_input_input_state_data(
        &self,
        r: *const xcb_input_input_state_t,
    ) -> *mut c_void {
        sym!(self, xcb_input_input_state_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_input_state_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_state_data(&self) -> bool {
        has_sym!(self, xcb_input_input_state_data)
    }

    /// Advances a `xcb_input_input_state_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_input_state_next(&self, i: *mut xcb_input_input_state_iterator_t) {
        sym!(self, xcb_input_input_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_input_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_state_next(&self) -> bool {
        has_sym!(self, xcb_input_input_state_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_input_state_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_input_state_end(
        &self,
        i: xcb_input_input_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_input_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_input_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_input_state_end(&self) -> bool {
        has_sym!(self, xcb_input_input_state_end)
    }

    /// Computes the size of a `xcb_input_query_device_state_reply_t` object.
    #[inline]
    pub unsafe fn xcb_input_query_device_state_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_query_device_state_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_query_device_state_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_query_device_state_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_query_device_state_sizeof)
    }

    /// Sends a `Input::QueryDeviceState` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_query_device_state_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_query_device_state_reply`]: Self::xcb_input_query_device_state_reply
    #[inline]
    pub unsafe fn xcb_input_query_device_state(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_query_device_state_cookie_t {
        sym!(self, xcb_input_query_device_state)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_query_device_state` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_query_device_state(&self) -> bool {
        has_sym!(self, xcb_input_query_device_state)
    }

    /// Sends a `Input::QueryDeviceState` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_query_device_state_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_query_device_state_reply`]: Self::xcb_input_query_device_state_reply
    #[inline]
    pub unsafe fn xcb_input_query_device_state_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_query_device_state_cookie_t {
        sym!(self, xcb_input_query_device_state_unchecked)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_query_device_state_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_query_device_state_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_query_device_state_unchecked)
    }

    /// Returns the number of elements of the `classes` field of a `xcb_input_query_device_state_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_query_device_state_classes_length(
        &self,
        r: *const xcb_input_query_device_state_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_query_device_state_classes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_query_device_state_classes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_query_device_state_classes_length(&self) -> bool {
        has_sym!(self, xcb_input_query_device_state_classes_length)
    }

    /// Returns an iterator over the elements of the
    /// `classes` field of a `xcb_input_query_device_state_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_query_device_state_classes_iterator(
        &self,
        r: *const xcb_input_query_device_state_reply_t,
    ) -> xcb_input_input_state_iterator_t {
        sym!(self, xcb_input_query_device_state_classes_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_query_device_state_classes_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_query_device_state_classes_iterator(&self) -> bool {
        has_sym!(self, xcb_input_query_device_state_classes_iterator)
    }

    /// Waits for the reply to a `Input::QueryDeviceState` request.
    #[inline]
    pub unsafe fn xcb_input_query_device_state_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_query_device_state_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_query_device_state_reply_t {
        sym!(self, xcb_input_query_device_state_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_query_device_state_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_query_device_state_reply(&self) -> bool {
        has_sym!(self, xcb_input_query_device_state_reply)
    }

    /// Sends a `Input::DeviceBell` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_input_device_bell_checked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
        feedback_id: u8,
        feedback_class: u8,
        percent: i8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_device_bell_checked)(
            c,
            device_id,
            feedback_id,
            feedback_class,
            percent,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_device_bell_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_bell_checked(&self) -> bool {
        has_sym!(self, xcb_input_device_bell_checked)
    }

    /// Sends a `Input::DeviceBell` request (unchecked).
    #[inline]
    pub unsafe fn xcb_input_device_bell(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
        feedback_id: u8,
        feedback_class: u8,
        percent: i8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_device_bell)(c, device_id, feedback_id, feedback_class, percent)
    }

    /// Returns `true` iff the symbol `xcb_input_device_bell` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_bell(&self) -> bool {
        has_sym!(self, xcb_input_device_bell)
    }

    /// Computes the size of a `xcb_input_set_device_valuators_request_t` object.
    #[inline]
    pub unsafe fn xcb_input_set_device_valuators_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_set_device_valuators_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_valuators_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_valuators_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_set_device_valuators_sizeof)
    }

    /// Sends a `Input::SetDeviceValuators` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_set_device_valuators_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_set_device_valuators_reply`]: Self::xcb_input_set_device_valuators_reply
    #[inline]
    pub unsafe fn xcb_input_set_device_valuators(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
        first_valuator: u8,
        num_valuators: u8,
        valuators: *const i32,
    ) -> xcb_input_set_device_valuators_cookie_t {
        sym!(self, xcb_input_set_device_valuators)(
            c,
            device_id,
            first_valuator,
            num_valuators,
            valuators,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_valuators` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_valuators(&self) -> bool {
        has_sym!(self, xcb_input_set_device_valuators)
    }

    /// Sends a `Input::SetDeviceValuators` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_set_device_valuators_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_set_device_valuators_reply`]: Self::xcb_input_set_device_valuators_reply
    #[inline]
    pub unsafe fn xcb_input_set_device_valuators_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
        first_valuator: u8,
        num_valuators: u8,
        valuators: *const i32,
    ) -> xcb_input_set_device_valuators_cookie_t {
        sym!(self, xcb_input_set_device_valuators_unchecked)(
            c,
            device_id,
            first_valuator,
            num_valuators,
            valuators,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_valuators_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_valuators_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_set_device_valuators_unchecked)
    }

    /// Waits for the reply to a `Input::SetDeviceValuators` request.
    #[inline]
    pub unsafe fn xcb_input_set_device_valuators_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_set_device_valuators_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_set_device_valuators_reply_t {
        sym!(self, xcb_input_set_device_valuators_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_set_device_valuators_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_set_device_valuators_reply(&self) -> bool {
        has_sym!(self, xcb_input_set_device_valuators_reply)
    }

    /// Computes the size of a `xcb_input_device_resolution_state_t` object.
    #[inline]
    pub unsafe fn xcb_input_device_resolution_state_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_device_resolution_state_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_state_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_state_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_device_resolution_state_sizeof)
    }

    /// Returns a pointer to the `resolution_values` field of a `xcb_input_device_resolution_state_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_resolution_state_resolution_values(
        &self,
        r: *const xcb_input_device_resolution_state_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_device_resolution_state_resolution_values)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_state_resolution_values` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_state_resolution_values(&self) -> bool {
        has_sym!(self, xcb_input_device_resolution_state_resolution_values)
    }

    /// Returns the number of elements of the `resolution_values` field of a `xcb_input_device_resolution_state_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_resolution_state_resolution_values_length(
        &self,
        r: *const xcb_input_device_resolution_state_t,
    ) -> c_int {
        sym!(
            self,
            xcb_input_device_resolution_state_resolution_values_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_state_resolution_values_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_state_resolution_values_length(&self) -> bool {
        has_sym!(
            self,
            xcb_input_device_resolution_state_resolution_values_length
        )
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `resolution_values` field of a `xcb_input_device_resolution_state_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_resolution_state_resolution_values_end(
        &self,
        r: *const xcb_input_device_resolution_state_t,
    ) -> xcb_generic_iterator_t {
        sym!(
            self,
            xcb_input_device_resolution_state_resolution_values_end
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_state_resolution_values_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_state_resolution_values_end(&self) -> bool {
        has_sym!(
            self,
            xcb_input_device_resolution_state_resolution_values_end
        )
    }

    /// Returns a pointer to the `resolution_min` field of a `xcb_input_device_resolution_state_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_resolution_state_resolution_min(
        &self,
        r: *const xcb_input_device_resolution_state_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_device_resolution_state_resolution_min)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_state_resolution_min` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_state_resolution_min(&self) -> bool {
        has_sym!(self, xcb_input_device_resolution_state_resolution_min)
    }

    /// Returns the number of elements of the `resolution_min` field of a `xcb_input_device_resolution_state_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_resolution_state_resolution_min_length(
        &self,
        r: *const xcb_input_device_resolution_state_t,
    ) -> c_int {
        sym!(
            self,
            xcb_input_device_resolution_state_resolution_min_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_state_resolution_min_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_state_resolution_min_length(&self) -> bool {
        has_sym!(
            self,
            xcb_input_device_resolution_state_resolution_min_length
        )
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `resolution_min` field of a `xcb_input_device_resolution_state_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_resolution_state_resolution_min_end(
        &self,
        r: *const xcb_input_device_resolution_state_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_resolution_state_resolution_min_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_state_resolution_min_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_state_resolution_min_end(&self) -> bool {
        has_sym!(self, xcb_input_device_resolution_state_resolution_min_end)
    }

    /// Returns a pointer to the `resolution_max` field of a `xcb_input_device_resolution_state_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_resolution_state_resolution_max(
        &self,
        r: *const xcb_input_device_resolution_state_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_device_resolution_state_resolution_max)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_state_resolution_max` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_state_resolution_max(&self) -> bool {
        has_sym!(self, xcb_input_device_resolution_state_resolution_max)
    }

    /// Returns the number of elements of the `resolution_max` field of a `xcb_input_device_resolution_state_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_resolution_state_resolution_max_length(
        &self,
        r: *const xcb_input_device_resolution_state_t,
    ) -> c_int {
        sym!(
            self,
            xcb_input_device_resolution_state_resolution_max_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_state_resolution_max_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_state_resolution_max_length(&self) -> bool {
        has_sym!(
            self,
            xcb_input_device_resolution_state_resolution_max_length
        )
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `resolution_max` field of a `xcb_input_device_resolution_state_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_resolution_state_resolution_max_end(
        &self,
        r: *const xcb_input_device_resolution_state_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_resolution_state_resolution_max_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_state_resolution_max_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_state_resolution_max_end(&self) -> bool {
        has_sym!(self, xcb_input_device_resolution_state_resolution_max_end)
    }

    /// Advances a `xcb_input_device_resolution_state_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_device_resolution_state_next(
        &self,
        i: *mut xcb_input_device_resolution_state_iterator_t,
    ) {
        sym!(self, xcb_input_device_resolution_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_state_next(&self) -> bool {
        has_sym!(self, xcb_input_device_resolution_state_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_device_resolution_state_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_device_resolution_state_end(
        &self,
        i: xcb_input_device_resolution_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_resolution_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_state_end(&self) -> bool {
        has_sym!(self, xcb_input_device_resolution_state_end)
    }

    /// Advances a `xcb_input_device_abs_calib_state_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_device_abs_calib_state_next(
        &self,
        i: *mut xcb_input_device_abs_calib_state_iterator_t,
    ) {
        sym!(self, xcb_input_device_abs_calib_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_abs_calib_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_abs_calib_state_next(&self) -> bool {
        has_sym!(self, xcb_input_device_abs_calib_state_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_device_abs_calib_state_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_device_abs_calib_state_end(
        &self,
        i: xcb_input_device_abs_calib_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_abs_calib_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_abs_calib_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_abs_calib_state_end(&self) -> bool {
        has_sym!(self, xcb_input_device_abs_calib_state_end)
    }

    /// Advances a `xcb_input_device_abs_area_state_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_device_abs_area_state_next(
        &self,
        i: *mut xcb_input_device_abs_area_state_iterator_t,
    ) {
        sym!(self, xcb_input_device_abs_area_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_abs_area_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_abs_area_state_next(&self) -> bool {
        has_sym!(self, xcb_input_device_abs_area_state_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_device_abs_area_state_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_device_abs_area_state_end(
        &self,
        i: xcb_input_device_abs_area_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_abs_area_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_abs_area_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_abs_area_state_end(&self) -> bool {
        has_sym!(self, xcb_input_device_abs_area_state_end)
    }

    /// Advances a `xcb_input_device_core_state_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_device_core_state_next(
        &self,
        i: *mut xcb_input_device_core_state_iterator_t,
    ) {
        sym!(self, xcb_input_device_core_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_core_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_core_state_next(&self) -> bool {
        has_sym!(self, xcb_input_device_core_state_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_device_core_state_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_device_core_state_end(
        &self,
        i: xcb_input_device_core_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_core_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_core_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_core_state_end(&self) -> bool {
        has_sym!(self, xcb_input_device_core_state_end)
    }

    /// Advances a `xcb_input_device_enable_state_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_device_enable_state_next(
        &self,
        i: *mut xcb_input_device_enable_state_iterator_t,
    ) {
        sym!(self, xcb_input_device_enable_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_enable_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_enable_state_next(&self) -> bool {
        has_sym!(self, xcb_input_device_enable_state_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_device_enable_state_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_device_enable_state_end(
        &self,
        i: xcb_input_device_enable_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_enable_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_enable_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_enable_state_end(&self) -> bool {
        has_sym!(self, xcb_input_device_enable_state_end)
    }

    /// Returns a pointer to the `resolution_values` field of a `xcb_input_device_state_data_resolution_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_state_data_resolution_resolution_values(
        &self,
        s: *const xcb_input_device_state_data_t,
    ) -> *mut u32 {
        sym!(
            self,
            xcb_input_device_state_data_resolution_resolution_values
        )(s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_data_resolution_resolution_values` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_data_resolution_resolution_values(&self) -> bool {
        has_sym!(
            self,
            xcb_input_device_state_data_resolution_resolution_values
        )
    }

    /// Returns the number of elements of the `resolution_values` field of a `xcb_input_device_state_data_resolution_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_state_data_resolution_resolution_values_length(
        &self,
        r: *const xcb_input_device_state_t,
        s: *const xcb_input_device_state_data_t,
    ) -> c_int {
        sym!(
            self,
            xcb_input_device_state_data_resolution_resolution_values_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_data_resolution_resolution_values_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_data_resolution_resolution_values_length(&self) -> bool {
        has_sym!(
            self,
            xcb_input_device_state_data_resolution_resolution_values_length
        )
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `resolution_values` field of a `xcb_input_device_state_data_resolution_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_state_data_resolution_resolution_values_end(
        &self,
        r: *const xcb_input_device_state_t,
        s: *const xcb_input_device_state_data_t,
    ) -> xcb_generic_iterator_t {
        sym!(
            self,
            xcb_input_device_state_data_resolution_resolution_values_end
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_data_resolution_resolution_values_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_data_resolution_resolution_values_end(&self) -> bool {
        has_sym!(
            self,
            xcb_input_device_state_data_resolution_resolution_values_end
        )
    }

    /// Returns a pointer to the `resolution_min` field of a `xcb_input_device_state_data_resolution_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_state_data_resolution_resolution_min(
        &self,
        s: *const xcb_input_device_state_data_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_device_state_data_resolution_resolution_min)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_data_resolution_resolution_min` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_data_resolution_resolution_min(&self) -> bool {
        has_sym!(self, xcb_input_device_state_data_resolution_resolution_min)
    }

    /// Returns the number of elements of the `resolution_min` field of a `xcb_input_device_state_data_resolution_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_state_data_resolution_resolution_min_length(
        &self,
        r: *const xcb_input_device_state_t,
        s: *const xcb_input_device_state_data_t,
    ) -> c_int {
        sym!(
            self,
            xcb_input_device_state_data_resolution_resolution_min_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_data_resolution_resolution_min_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_data_resolution_resolution_min_length(&self) -> bool {
        has_sym!(
            self,
            xcb_input_device_state_data_resolution_resolution_min_length
        )
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `resolution_min` field of a `xcb_input_device_state_data_resolution_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_state_data_resolution_resolution_min_end(
        &self,
        r: *const xcb_input_device_state_t,
        s: *const xcb_input_device_state_data_t,
    ) -> xcb_generic_iterator_t {
        sym!(
            self,
            xcb_input_device_state_data_resolution_resolution_min_end
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_data_resolution_resolution_min_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_data_resolution_resolution_min_end(&self) -> bool {
        has_sym!(
            self,
            xcb_input_device_state_data_resolution_resolution_min_end
        )
    }

    /// Returns a pointer to the `resolution_max` field of a `xcb_input_device_state_data_resolution_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_state_data_resolution_resolution_max(
        &self,
        s: *const xcb_input_device_state_data_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_device_state_data_resolution_resolution_max)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_data_resolution_resolution_max` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_data_resolution_resolution_max(&self) -> bool {
        has_sym!(self, xcb_input_device_state_data_resolution_resolution_max)
    }

    /// Returns the number of elements of the `resolution_max` field of a `xcb_input_device_state_data_resolution_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_state_data_resolution_resolution_max_length(
        &self,
        r: *const xcb_input_device_state_t,
        s: *const xcb_input_device_state_data_t,
    ) -> c_int {
        sym!(
            self,
            xcb_input_device_state_data_resolution_resolution_max_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_data_resolution_resolution_max_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_data_resolution_resolution_max_length(&self) -> bool {
        has_sym!(
            self,
            xcb_input_device_state_data_resolution_resolution_max_length
        )
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `resolution_max` field of a `xcb_input_device_state_data_resolution_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_state_data_resolution_resolution_max_end(
        &self,
        r: *const xcb_input_device_state_t,
        s: *const xcb_input_device_state_data_t,
    ) -> xcb_generic_iterator_t {
        sym!(
            self,
            xcb_input_device_state_data_resolution_resolution_max_end
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_data_resolution_resolution_max_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_data_resolution_resolution_max_end(&self) -> bool {
        has_sym!(
            self,
            xcb_input_device_state_data_resolution_resolution_max_end
        )
    }

    /// Serializes a `xcb_input_device_state_data_t` object.
    #[inline]
    pub unsafe fn xcb_input_device_state_data_serialize(
        &self,
        _buffer: *mut *mut c_void,
        control_id: u16,
        _aux: *const xcb_input_device_state_data_t,
    ) -> c_int {
        sym!(self, xcb_input_device_state_data_serialize)(_buffer, control_id, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_data_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_data_serialize(&self) -> bool {
        has_sym!(self, xcb_input_device_state_data_serialize)
    }

    /// Unpacks a `xcb_input_device_state_data_t` object.
    #[inline]
    pub unsafe fn xcb_input_device_state_data_unpack(
        &self,
        _buffer: *const c_void,
        control_id: u16,
        _aux: *mut xcb_input_device_state_data_t,
    ) -> c_int {
        sym!(self, xcb_input_device_state_data_unpack)(_buffer, control_id, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_data_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_data_unpack(&self) -> bool {
        has_sym!(self, xcb_input_device_state_data_unpack)
    }

    /// Computes the size of a `xcb_input_device_state_data_t` object.
    #[inline]
    pub unsafe fn xcb_input_device_state_data_sizeof(
        &self,
        _buffer: *const c_void,
        control_id: u16,
    ) -> c_int {
        sym!(self, xcb_input_device_state_data_sizeof)(_buffer, control_id)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_data_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_data_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_device_state_data_sizeof)
    }

    /// Computes the size of a `xcb_input_device_state_t` object.
    #[inline]
    pub unsafe fn xcb_input_device_state_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_device_state_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_device_state_sizeof)
    }

    /// Returns a pointer to the `data` field of a `xcb_input_device_state_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_state_data(
        &self,
        r: *const xcb_input_device_state_t,
    ) -> *mut c_void {
        sym!(self, xcb_input_device_state_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_data(&self) -> bool {
        has_sym!(self, xcb_input_device_state_data)
    }

    /// Advances a `xcb_input_device_state_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_device_state_next(&self, i: *mut xcb_input_device_state_iterator_t) {
        sym!(self, xcb_input_device_state_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_next(&self) -> bool {
        has_sym!(self, xcb_input_device_state_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_device_state_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_device_state_end(
        &self,
        i: xcb_input_device_state_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_state_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_state_end(&self) -> bool {
        has_sym!(self, xcb_input_device_state_end)
    }

    /// Computes the size of a `xcb_input_get_device_control_reply_t` object.
    #[inline]
    pub unsafe fn xcb_input_get_device_control_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_get_device_control_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_control_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_control_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_get_device_control_sizeof)
    }

    /// Sends a `Input::GetDeviceControl` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_get_device_control_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_get_device_control_reply`]: Self::xcb_input_get_device_control_reply
    #[inline]
    pub unsafe fn xcb_input_get_device_control(
        &self,
        c: *mut xcb_connection_t,
        control_id: u16,
        device_id: u8,
    ) -> xcb_input_get_device_control_cookie_t {
        sym!(self, xcb_input_get_device_control)(c, control_id, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_control` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_control(&self) -> bool {
        has_sym!(self, xcb_input_get_device_control)
    }

    /// Sends a `Input::GetDeviceControl` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_get_device_control_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_get_device_control_reply`]: Self::xcb_input_get_device_control_reply
    #[inline]
    pub unsafe fn xcb_input_get_device_control_unchecked(
        &self,
        c: *mut xcb_connection_t,
        control_id: u16,
        device_id: u8,
    ) -> xcb_input_get_device_control_cookie_t {
        sym!(self, xcb_input_get_device_control_unchecked)(c, control_id, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_control_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_control_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_get_device_control_unchecked)
    }

    /// Returns a pointer to the `control` field of a `xcb_input_get_device_control_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_device_control_control(
        &self,
        r: *const xcb_input_get_device_control_reply_t,
    ) -> *mut xcb_input_device_state_t {
        sym!(self, xcb_input_get_device_control_control)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_control_control` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_control_control(&self) -> bool {
        has_sym!(self, xcb_input_get_device_control_control)
    }

    /// Waits for the reply to a `Input::GetDeviceControl` request.
    #[inline]
    pub unsafe fn xcb_input_get_device_control_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_get_device_control_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_get_device_control_reply_t {
        sym!(self, xcb_input_get_device_control_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_control_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_control_reply(&self) -> bool {
        has_sym!(self, xcb_input_get_device_control_reply)
    }

    /// Computes the size of a `xcb_input_device_resolution_ctl_t` object.
    #[inline]
    pub unsafe fn xcb_input_device_resolution_ctl_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_device_resolution_ctl_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_ctl_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_ctl_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_device_resolution_ctl_sizeof)
    }

    /// Returns a pointer to the `resolution_values` field of a `xcb_input_device_resolution_ctl_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_resolution_ctl_resolution_values(
        &self,
        r: *const xcb_input_device_resolution_ctl_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_device_resolution_ctl_resolution_values)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_ctl_resolution_values` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_ctl_resolution_values(&self) -> bool {
        has_sym!(self, xcb_input_device_resolution_ctl_resolution_values)
    }

    /// Returns the number of elements of the `resolution_values` field of a `xcb_input_device_resolution_ctl_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_resolution_ctl_resolution_values_length(
        &self,
        r: *const xcb_input_device_resolution_ctl_t,
    ) -> c_int {
        sym!(
            self,
            xcb_input_device_resolution_ctl_resolution_values_length
        )(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_ctl_resolution_values_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_ctl_resolution_values_length(&self) -> bool {
        has_sym!(
            self,
            xcb_input_device_resolution_ctl_resolution_values_length
        )
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `resolution_values` field of a `xcb_input_device_resolution_ctl_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_resolution_ctl_resolution_values_end(
        &self,
        r: *const xcb_input_device_resolution_ctl_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_resolution_ctl_resolution_values_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_ctl_resolution_values_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_ctl_resolution_values_end(&self) -> bool {
        has_sym!(self, xcb_input_device_resolution_ctl_resolution_values_end)
    }

    /// Advances a `xcb_input_device_resolution_ctl_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_device_resolution_ctl_next(
        &self,
        i: *mut xcb_input_device_resolution_ctl_iterator_t,
    ) {
        sym!(self, xcb_input_device_resolution_ctl_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_ctl_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_ctl_next(&self) -> bool {
        has_sym!(self, xcb_input_device_resolution_ctl_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_device_resolution_ctl_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_device_resolution_ctl_end(
        &self,
        i: xcb_input_device_resolution_ctl_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_resolution_ctl_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_resolution_ctl_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_resolution_ctl_end(&self) -> bool {
        has_sym!(self, xcb_input_device_resolution_ctl_end)
    }

    /// Advances a `xcb_input_device_abs_calib_ctl_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_device_abs_calib_ctl_next(
        &self,
        i: *mut xcb_input_device_abs_calib_ctl_iterator_t,
    ) {
        sym!(self, xcb_input_device_abs_calib_ctl_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_abs_calib_ctl_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_abs_calib_ctl_next(&self) -> bool {
        has_sym!(self, xcb_input_device_abs_calib_ctl_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_device_abs_calib_ctl_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_device_abs_calib_ctl_end(
        &self,
        i: xcb_input_device_abs_calib_ctl_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_abs_calib_ctl_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_abs_calib_ctl_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_abs_calib_ctl_end(&self) -> bool {
        has_sym!(self, xcb_input_device_abs_calib_ctl_end)
    }

    /// Advances a `xcb_input_device_abs_area_ctrl_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_device_abs_area_ctrl_next(
        &self,
        i: *mut xcb_input_device_abs_area_ctrl_iterator_t,
    ) {
        sym!(self, xcb_input_device_abs_area_ctrl_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_abs_area_ctrl_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_abs_area_ctrl_next(&self) -> bool {
        has_sym!(self, xcb_input_device_abs_area_ctrl_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_device_abs_area_ctrl_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_device_abs_area_ctrl_end(
        &self,
        i: xcb_input_device_abs_area_ctrl_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_abs_area_ctrl_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_abs_area_ctrl_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_abs_area_ctrl_end(&self) -> bool {
        has_sym!(self, xcb_input_device_abs_area_ctrl_end)
    }

    /// Advances a `xcb_input_device_core_ctrl_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_device_core_ctrl_next(
        &self,
        i: *mut xcb_input_device_core_ctrl_iterator_t,
    ) {
        sym!(self, xcb_input_device_core_ctrl_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_core_ctrl_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_core_ctrl_next(&self) -> bool {
        has_sym!(self, xcb_input_device_core_ctrl_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_device_core_ctrl_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_device_core_ctrl_end(
        &self,
        i: xcb_input_device_core_ctrl_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_core_ctrl_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_core_ctrl_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_core_ctrl_end(&self) -> bool {
        has_sym!(self, xcb_input_device_core_ctrl_end)
    }

    /// Advances a `xcb_input_device_enable_ctrl_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_device_enable_ctrl_next(
        &self,
        i: *mut xcb_input_device_enable_ctrl_iterator_t,
    ) {
        sym!(self, xcb_input_device_enable_ctrl_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_enable_ctrl_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_enable_ctrl_next(&self) -> bool {
        has_sym!(self, xcb_input_device_enable_ctrl_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_device_enable_ctrl_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_device_enable_ctrl_end(
        &self,
        i: xcb_input_device_enable_ctrl_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_enable_ctrl_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_enable_ctrl_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_enable_ctrl_end(&self) -> bool {
        has_sym!(self, xcb_input_device_enable_ctrl_end)
    }

    /// Returns a pointer to the `resolution_values` field of a `xcb_input_device_ctl_data_resolution_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_ctl_data_resolution_resolution_values(
        &self,
        s: *const xcb_input_device_ctl_data_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_device_ctl_data_resolution_resolution_values)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_ctl_data_resolution_resolution_values` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_ctl_data_resolution_resolution_values(&self) -> bool {
        has_sym!(self, xcb_input_device_ctl_data_resolution_resolution_values)
    }

    /// Returns the number of elements of the `resolution_values` field of a `xcb_input_device_ctl_data_resolution_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_ctl_data_resolution_resolution_values_length(
        &self,
        r: *const xcb_input_device_ctl_t,
        s: *const xcb_input_device_ctl_data_t,
    ) -> c_int {
        sym!(
            self,
            xcb_input_device_ctl_data_resolution_resolution_values_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_ctl_data_resolution_resolution_values_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_ctl_data_resolution_resolution_values_length(&self) -> bool {
        has_sym!(
            self,
            xcb_input_device_ctl_data_resolution_resolution_values_length
        )
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `resolution_values` field of a `xcb_input_device_ctl_data_resolution_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_ctl_data_resolution_resolution_values_end(
        &self,
        r: *const xcb_input_device_ctl_t,
        s: *const xcb_input_device_ctl_data_t,
    ) -> xcb_generic_iterator_t {
        sym!(
            self,
            xcb_input_device_ctl_data_resolution_resolution_values_end
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_ctl_data_resolution_resolution_values_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_ctl_data_resolution_resolution_values_end(&self) -> bool {
        has_sym!(
            self,
            xcb_input_device_ctl_data_resolution_resolution_values_end
        )
    }

    /// Serializes a `xcb_input_device_ctl_data_t` object.
    #[inline]
    pub unsafe fn xcb_input_device_ctl_data_serialize(
        &self,
        _buffer: *mut *mut c_void,
        control_id: u16,
        _aux: *const xcb_input_device_ctl_data_t,
    ) -> c_int {
        sym!(self, xcb_input_device_ctl_data_serialize)(_buffer, control_id, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_device_ctl_data_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_ctl_data_serialize(&self) -> bool {
        has_sym!(self, xcb_input_device_ctl_data_serialize)
    }

    /// Unpacks a `xcb_input_device_ctl_data_t` object.
    #[inline]
    pub unsafe fn xcb_input_device_ctl_data_unpack(
        &self,
        _buffer: *const c_void,
        control_id: u16,
        _aux: *mut xcb_input_device_ctl_data_t,
    ) -> c_int {
        sym!(self, xcb_input_device_ctl_data_unpack)(_buffer, control_id, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_device_ctl_data_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_ctl_data_unpack(&self) -> bool {
        has_sym!(self, xcb_input_device_ctl_data_unpack)
    }

    /// Computes the size of a `xcb_input_device_ctl_data_t` object.
    #[inline]
    pub unsafe fn xcb_input_device_ctl_data_sizeof(
        &self,
        _buffer: *const c_void,
        control_id: u16,
    ) -> c_int {
        sym!(self, xcb_input_device_ctl_data_sizeof)(_buffer, control_id)
    }

    /// Returns `true` iff the symbol `xcb_input_device_ctl_data_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_ctl_data_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_device_ctl_data_sizeof)
    }

    /// Computes the size of a `xcb_input_device_ctl_t` object.
    #[inline]
    pub unsafe fn xcb_input_device_ctl_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_device_ctl_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_device_ctl_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_ctl_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_device_ctl_sizeof)
    }

    /// Returns a pointer to the `data` field of a `xcb_input_device_ctl_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_ctl_data(
        &self,
        r: *const xcb_input_device_ctl_t,
    ) -> *mut c_void {
        sym!(self, xcb_input_device_ctl_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_ctl_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_ctl_data(&self) -> bool {
        has_sym!(self, xcb_input_device_ctl_data)
    }

    /// Advances a `xcb_input_device_ctl_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_device_ctl_next(&self, i: *mut xcb_input_device_ctl_iterator_t) {
        sym!(self, xcb_input_device_ctl_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_ctl_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_ctl_next(&self) -> bool {
        has_sym!(self, xcb_input_device_ctl_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_device_ctl_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_device_ctl_end(
        &self,
        i: xcb_input_device_ctl_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_ctl_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_ctl_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_ctl_end(&self) -> bool {
        has_sym!(self, xcb_input_device_ctl_end)
    }

    /// Computes the size of a `xcb_input_change_device_control_request_t` object.
    #[inline]
    pub unsafe fn xcb_input_change_device_control_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_change_device_control_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_control_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_control_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_change_device_control_sizeof)
    }

    /// Sends a `Input::ChangeDeviceControl` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_change_device_control_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_change_device_control_reply`]: Self::xcb_input_change_device_control_reply
    #[inline]
    pub unsafe fn xcb_input_change_device_control(
        &self,
        c: *mut xcb_connection_t,
        control_id: u16,
        device_id: u8,
        control: *mut xcb_input_device_ctl_t,
    ) -> xcb_input_change_device_control_cookie_t {
        sym!(self, xcb_input_change_device_control)(c, control_id, device_id, control)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_control` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_control(&self) -> bool {
        has_sym!(self, xcb_input_change_device_control)
    }

    /// Sends a `Input::ChangeDeviceControl` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_change_device_control_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_change_device_control_reply`]: Self::xcb_input_change_device_control_reply
    #[inline]
    pub unsafe fn xcb_input_change_device_control_unchecked(
        &self,
        c: *mut xcb_connection_t,
        control_id: u16,
        device_id: u8,
        control: *mut xcb_input_device_ctl_t,
    ) -> xcb_input_change_device_control_cookie_t {
        sym!(self, xcb_input_change_device_control_unchecked)(c, control_id, device_id, control)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_control_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_control_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_change_device_control_unchecked)
    }

    /// Waits for the reply to a `Input::ChangeDeviceControl` request.
    #[inline]
    pub unsafe fn xcb_input_change_device_control_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_change_device_control_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_change_device_control_reply_t {
        sym!(self, xcb_input_change_device_control_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_control_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_control_reply(&self) -> bool {
        has_sym!(self, xcb_input_change_device_control_reply)
    }

    /// Computes the size of a `xcb_input_list_device_properties_reply_t` object.
    #[inline]
    pub unsafe fn xcb_input_list_device_properties_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_list_device_properties_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_list_device_properties_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_device_properties_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_list_device_properties_sizeof)
    }

    /// Sends a `Input::ListDeviceProperties` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_list_device_properties_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_list_device_properties_reply`]: Self::xcb_input_list_device_properties_reply
    #[inline]
    pub unsafe fn xcb_input_list_device_properties(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_list_device_properties_cookie_t {
        sym!(self, xcb_input_list_device_properties)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_list_device_properties` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_device_properties(&self) -> bool {
        has_sym!(self, xcb_input_list_device_properties)
    }

    /// Sends a `Input::ListDeviceProperties` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_list_device_properties_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_list_device_properties_reply`]: Self::xcb_input_list_device_properties_reply
    #[inline]
    pub unsafe fn xcb_input_list_device_properties_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_id: u8,
    ) -> xcb_input_list_device_properties_cookie_t {
        sym!(self, xcb_input_list_device_properties_unchecked)(c, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_list_device_properties_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_device_properties_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_list_device_properties_unchecked)
    }

    /// Returns a pointer to the `atoms` field of a `xcb_input_list_device_properties_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_list_device_properties_atoms(
        &self,
        r: *const xcb_input_list_device_properties_reply_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_input_list_device_properties_atoms)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_list_device_properties_atoms` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_device_properties_atoms(&self) -> bool {
        has_sym!(self, xcb_input_list_device_properties_atoms)
    }

    /// Returns the number of elements of the `atoms` field of a `xcb_input_list_device_properties_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_list_device_properties_atoms_length(
        &self,
        r: *const xcb_input_list_device_properties_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_list_device_properties_atoms_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_list_device_properties_atoms_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_device_properties_atoms_length(&self) -> bool {
        has_sym!(self, xcb_input_list_device_properties_atoms_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `atoms` field of a `xcb_input_list_device_properties_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_list_device_properties_atoms_end(
        &self,
        r: *const xcb_input_list_device_properties_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_list_device_properties_atoms_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_list_device_properties_atoms_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_device_properties_atoms_end(&self) -> bool {
        has_sym!(self, xcb_input_list_device_properties_atoms_end)
    }

    /// Waits for the reply to a `Input::ListDeviceProperties` request.
    #[inline]
    pub unsafe fn xcb_input_list_device_properties_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_list_device_properties_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_list_device_properties_reply_t {
        sym!(self, xcb_input_list_device_properties_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_list_device_properties_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_list_device_properties_reply(&self) -> bool {
        has_sym!(self, xcb_input_list_device_properties_reply)
    }

    /// Returns a pointer to the `data8` field of a `xcb_input_change_device_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_change_device_property_items_data_8(
        &self,
        s: *const xcb_input_change_device_property_items_t,
    ) -> *mut u8 {
        sym!(self, xcb_input_change_device_property_items_data_8)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_items_data_8` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_items_data_8(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_items_data_8)
    }

    /// Returns the number of elements of the `data8` field of a `xcb_input_change_device_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_change_device_property_items_data_8_length(
        &self,
        r: *const xcb_input_change_device_property_request_t,
        s: *const xcb_input_change_device_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_change_device_property_items_data_8_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_items_data_8_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_items_data_8_length(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_items_data_8_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `data8` field of a `xcb_input_change_device_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_change_device_property_items_data_8_end(
        &self,
        r: *const xcb_input_change_device_property_request_t,
        s: *const xcb_input_change_device_property_items_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_change_device_property_items_data_8_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_items_data_8_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_items_data_8_end(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_items_data_8_end)
    }

    /// Returns a pointer to the `data16` field of a `xcb_input_change_device_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_change_device_property_items_data_16(
        &self,
        s: *const xcb_input_change_device_property_items_t,
    ) -> *mut u16 {
        sym!(self, xcb_input_change_device_property_items_data_16)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_items_data_16` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_items_data_16(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_items_data_16)
    }

    /// Returns the number of elements of the `data16` field of a `xcb_input_change_device_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_change_device_property_items_data_16_length(
        &self,
        r: *const xcb_input_change_device_property_request_t,
        s: *const xcb_input_change_device_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_change_device_property_items_data_16_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_items_data_16_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_items_data_16_length(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_items_data_16_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `data16` field of a `xcb_input_change_device_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_change_device_property_items_data_16_end(
        &self,
        r: *const xcb_input_change_device_property_request_t,
        s: *const xcb_input_change_device_property_items_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_change_device_property_items_data_16_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_items_data_16_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_items_data_16_end(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_items_data_16_end)
    }

    /// Returns a pointer to the `data32` field of a `xcb_input_change_device_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_change_device_property_items_data_32(
        &self,
        s: *const xcb_input_change_device_property_items_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_change_device_property_items_data_32)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_items_data_32` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_items_data_32(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_items_data_32)
    }

    /// Returns the number of elements of the `data32` field of a `xcb_input_change_device_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_change_device_property_items_data_32_length(
        &self,
        r: *const xcb_input_change_device_property_request_t,
        s: *const xcb_input_change_device_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_change_device_property_items_data_32_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_items_data_32_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_items_data_32_length(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_items_data_32_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `data32` field of a `xcb_input_change_device_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_change_device_property_items_data_32_end(
        &self,
        r: *const xcb_input_change_device_property_request_t,
        s: *const xcb_input_change_device_property_items_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_change_device_property_items_data_32_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_items_data_32_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_items_data_32_end(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_items_data_32_end)
    }

    /// Serializes a `xcb_input_change_device_property_items_t` object.
    #[inline]
    pub unsafe fn xcb_input_change_device_property_items_serialize(
        &self,
        _buffer: *mut *mut c_void,
        num_items: u32,
        format: u8,
        _aux: *const xcb_input_change_device_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_change_device_property_items_serialize)(
            _buffer, num_items, format, _aux,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_items_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_items_serialize(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_items_serialize)
    }

    /// Unpacks a `xcb_input_change_device_property_items_t` object.
    #[inline]
    pub unsafe fn xcb_input_change_device_property_items_unpack(
        &self,
        _buffer: *const c_void,
        num_items: u32,
        format: u8,
        _aux: *mut xcb_input_change_device_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_change_device_property_items_unpack)(_buffer, num_items, format, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_items_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_items_unpack(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_items_unpack)
    }

    /// Computes the size of a `xcb_input_change_device_property_items_t` object.
    #[inline]
    pub unsafe fn xcb_input_change_device_property_items_sizeof(
        &self,
        _buffer: *const c_void,
        num_items: u32,
        format: u8,
    ) -> c_int {
        sym!(self, xcb_input_change_device_property_items_sizeof)(_buffer, num_items, format)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_items_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_items_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_items_sizeof)
    }

    /// Computes the size of a `xcb_input_change_device_property_request_t` object.
    #[inline]
    pub unsafe fn xcb_input_change_device_property_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_change_device_property_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_sizeof)
    }

    /// Sends a `Input::ChangeDeviceProperty` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    ///
    /// There is an auxiliary version of this function: [`xcb_input_change_device_property_aux_checked`].
    ///
    /// [`xcb_input_change_device_property_aux_checked`]: Self::xcb_input_change_device_property_aux_checked
    #[inline]
    pub unsafe fn xcb_input_change_device_property_checked(
        &self,
        c: *mut xcb_connection_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        device_id: u8,
        format: u8,
        mode: u8,
        num_items: u32,
        items: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_change_device_property_checked)(
            c, property, type_, device_id, format, mode, num_items, items,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_checked(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_checked)
    }

    /// Sends a `Input::ChangeDeviceProperty` request (unchecked).
    ///
    /// There is an auxiliary version of this function: [`xcb_input_change_device_property_aux`].
    ///
    /// [`xcb_input_change_device_property_aux`]: Self::xcb_input_change_device_property_aux
    #[inline]
    pub unsafe fn xcb_input_change_device_property(
        &self,
        c: *mut xcb_connection_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        device_id: u8,
        format: u8,
        mode: u8,
        num_items: u32,
        items: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_change_device_property)(
            c, property, type_, device_id, format, mode, num_items, items,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property)
    }

    /// Sends a `Input::ChangeDeviceProperty` request (checked) (aux).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_input_change_device_property_aux_checked(
        &self,
        c: *mut xcb_connection_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        device_id: u8,
        format: u8,
        mode: u8,
        num_items: u32,
        items: *const xcb_input_change_device_property_items_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_change_device_property_aux_checked)(
            c, property, type_, device_id, format, mode, num_items, items,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_aux_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_aux_checked(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_aux_checked)
    }

    /// Sends a `Input::ChangeDeviceProperty` request (unchecked) (aux).
    #[inline]
    pub unsafe fn xcb_input_change_device_property_aux(
        &self,
        c: *mut xcb_connection_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        device_id: u8,
        format: u8,
        mode: u8,
        num_items: u32,
        items: *const xcb_input_change_device_property_items_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_change_device_property_aux)(
            c, property, type_, device_id, format, mode, num_items, items,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_aux` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_aux(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_aux)
    }

    /// Returns a pointer to the `items` field of a `xcb_input_change_device_property_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_change_device_property_items(
        &self,
        r: *const xcb_input_change_device_property_request_t,
    ) -> *mut c_void {
        sym!(self, xcb_input_change_device_property_items)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_change_device_property_items` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_change_device_property_items(&self) -> bool {
        has_sym!(self, xcb_input_change_device_property_items)
    }

    /// Sends a `Input::DeleteDeviceProperty` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_input_delete_device_property_checked(
        &self,
        c: *mut xcb_connection_t,
        property: xcb_atom_t,
        device_id: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_delete_device_property_checked)(c, property, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_delete_device_property_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_delete_device_property_checked(&self) -> bool {
        has_sym!(self, xcb_input_delete_device_property_checked)
    }

    /// Sends a `Input::DeleteDeviceProperty` request (unchecked).
    #[inline]
    pub unsafe fn xcb_input_delete_device_property(
        &self,
        c: *mut xcb_connection_t,
        property: xcb_atom_t,
        device_id: u8,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_delete_device_property)(c, property, device_id)
    }

    /// Returns `true` iff the symbol `xcb_input_delete_device_property` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_delete_device_property(&self) -> bool {
        has_sym!(self, xcb_input_delete_device_property)
    }

    /// Returns a pointer to the `data8` field of a `xcb_input_get_device_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_device_property_items_data_8(
        &self,
        s: *const xcb_input_get_device_property_items_t,
    ) -> *mut u8 {
        sym!(self, xcb_input_get_device_property_items_data_8)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_items_data_8` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_items_data_8(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_items_data_8)
    }

    /// Returns the number of elements of the `data8` field of a `xcb_input_get_device_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_device_property_items_data_8_length(
        &self,
        r: *const xcb_input_get_device_property_reply_t,
        s: *const xcb_input_get_device_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_get_device_property_items_data_8_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_items_data_8_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_items_data_8_length(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_items_data_8_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `data8` field of a `xcb_input_get_device_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_device_property_items_data_8_end(
        &self,
        r: *const xcb_input_get_device_property_reply_t,
        s: *const xcb_input_get_device_property_items_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_get_device_property_items_data_8_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_items_data_8_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_items_data_8_end(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_items_data_8_end)
    }

    /// Returns a pointer to the `data16` field of a `xcb_input_get_device_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_device_property_items_data_16(
        &self,
        s: *const xcb_input_get_device_property_items_t,
    ) -> *mut u16 {
        sym!(self, xcb_input_get_device_property_items_data_16)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_items_data_16` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_items_data_16(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_items_data_16)
    }

    /// Returns the number of elements of the `data16` field of a `xcb_input_get_device_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_device_property_items_data_16_length(
        &self,
        r: *const xcb_input_get_device_property_reply_t,
        s: *const xcb_input_get_device_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_get_device_property_items_data_16_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_items_data_16_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_items_data_16_length(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_items_data_16_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `data16` field of a `xcb_input_get_device_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_device_property_items_data_16_end(
        &self,
        r: *const xcb_input_get_device_property_reply_t,
        s: *const xcb_input_get_device_property_items_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_get_device_property_items_data_16_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_items_data_16_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_items_data_16_end(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_items_data_16_end)
    }

    /// Returns a pointer to the `data32` field of a `xcb_input_get_device_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_device_property_items_data_32(
        &self,
        s: *const xcb_input_get_device_property_items_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_get_device_property_items_data_32)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_items_data_32` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_items_data_32(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_items_data_32)
    }

    /// Returns the number of elements of the `data32` field of a `xcb_input_get_device_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_device_property_items_data_32_length(
        &self,
        r: *const xcb_input_get_device_property_reply_t,
        s: *const xcb_input_get_device_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_get_device_property_items_data_32_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_items_data_32_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_items_data_32_length(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_items_data_32_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `data32` field of a `xcb_input_get_device_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_device_property_items_data_32_end(
        &self,
        r: *const xcb_input_get_device_property_reply_t,
        s: *const xcb_input_get_device_property_items_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_get_device_property_items_data_32_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_items_data_32_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_items_data_32_end(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_items_data_32_end)
    }

    /// Serializes a `xcb_input_get_device_property_items_t` object.
    #[inline]
    pub unsafe fn xcb_input_get_device_property_items_serialize(
        &self,
        _buffer: *mut *mut c_void,
        num_items: u32,
        format: u8,
        _aux: *const xcb_input_get_device_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_get_device_property_items_serialize)(_buffer, num_items, format, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_items_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_items_serialize(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_items_serialize)
    }

    /// Unpacks a `xcb_input_get_device_property_items_t` object.
    #[inline]
    pub unsafe fn xcb_input_get_device_property_items_unpack(
        &self,
        _buffer: *const c_void,
        num_items: u32,
        format: u8,
        _aux: *mut xcb_input_get_device_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_get_device_property_items_unpack)(_buffer, num_items, format, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_items_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_items_unpack(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_items_unpack)
    }

    /// Computes the size of a `xcb_input_get_device_property_items_t` object.
    #[inline]
    pub unsafe fn xcb_input_get_device_property_items_sizeof(
        &self,
        _buffer: *const c_void,
        num_items: u32,
        format: u8,
    ) -> c_int {
        sym!(self, xcb_input_get_device_property_items_sizeof)(_buffer, num_items, format)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_items_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_items_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_items_sizeof)
    }

    /// Computes the size of a `xcb_input_get_device_property_reply_t` object.
    #[inline]
    pub unsafe fn xcb_input_get_device_property_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_get_device_property_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_sizeof)
    }

    /// Sends a `Input::GetDeviceProperty` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_get_device_property_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_get_device_property_reply`]: Self::xcb_input_get_device_property_reply
    #[inline]
    pub unsafe fn xcb_input_get_device_property(
        &self,
        c: *mut xcb_connection_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        offset: u32,
        len: u32,
        device_id: u8,
        delete: u8,
    ) -> xcb_input_get_device_property_cookie_t {
        sym!(self, xcb_input_get_device_property)(
            c, property, type_, offset, len, device_id, delete,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property)
    }

    /// Sends a `Input::GetDeviceProperty` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_get_device_property_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_get_device_property_reply`]: Self::xcb_input_get_device_property_reply
    #[inline]
    pub unsafe fn xcb_input_get_device_property_unchecked(
        &self,
        c: *mut xcb_connection_t,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        offset: u32,
        len: u32,
        device_id: u8,
        delete: u8,
    ) -> xcb_input_get_device_property_cookie_t {
        sym!(self, xcb_input_get_device_property_unchecked)(
            c, property, type_, offset, len, device_id, delete,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_unchecked)
    }

    /// Returns a pointer to the `items` field of a `xcb_input_get_device_property_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_get_device_property_items(
        &self,
        r: *const xcb_input_get_device_property_reply_t,
    ) -> *mut c_void {
        sym!(self, xcb_input_get_device_property_items)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_items` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_items(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_items)
    }

    /// Waits for the reply to a `Input::GetDeviceProperty` request.
    #[inline]
    pub unsafe fn xcb_input_get_device_property_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_get_device_property_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_get_device_property_reply_t {
        sym!(self, xcb_input_get_device_property_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_get_device_property_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_get_device_property_reply(&self) -> bool {
        has_sym!(self, xcb_input_get_device_property_reply)
    }

    /// Advances a `xcb_input_group_info_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_group_info_next(&self, i: *mut xcb_input_group_info_iterator_t) {
        sym!(self, xcb_input_group_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_group_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_group_info_next(&self) -> bool {
        has_sym!(self, xcb_input_group_info_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_group_info_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_group_info_end(
        &self,
        i: xcb_input_group_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_group_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_group_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_group_info_end(&self) -> bool {
        has_sym!(self, xcb_input_group_info_end)
    }

    /// Advances a `xcb_input_modifier_info_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_modifier_info_next(&self, i: *mut xcb_input_modifier_info_iterator_t) {
        sym!(self, xcb_input_modifier_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_modifier_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_modifier_info_next(&self) -> bool {
        has_sym!(self, xcb_input_modifier_info_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_modifier_info_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_modifier_info_end(
        &self,
        i: xcb_input_modifier_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_modifier_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_modifier_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_modifier_info_end(&self) -> bool {
        has_sym!(self, xcb_input_modifier_info_end)
    }

    /// Computes the size of a `xcb_input_xi_query_pointer_reply_t` object.
    #[inline]
    pub unsafe fn xcb_input_xi_query_pointer_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_xi_query_pointer_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_pointer_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_pointer_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_pointer_sizeof)
    }

    /// Sends a `Input::XIQueryPointer` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_xi_query_pointer_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_xi_query_pointer_reply`]: Self::xcb_input_xi_query_pointer_reply
    #[inline]
    pub unsafe fn xcb_input_xi_query_pointer(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_input_xi_query_pointer_cookie_t {
        sym!(self, xcb_input_xi_query_pointer)(c, window, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_pointer` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_pointer(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_pointer)
    }

    /// Sends a `Input::XIQueryPointer` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_xi_query_pointer_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_xi_query_pointer_reply`]: Self::xcb_input_xi_query_pointer_reply
    #[inline]
    pub unsafe fn xcb_input_xi_query_pointer_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_input_xi_query_pointer_cookie_t {
        sym!(self, xcb_input_xi_query_pointer_unchecked)(c, window, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_pointer_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_pointer_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_pointer_unchecked)
    }

    /// Returns a pointer to the `buttons` field of a `xcb_input_xi_query_pointer_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_query_pointer_buttons(
        &self,
        r: *const xcb_input_xi_query_pointer_reply_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_xi_query_pointer_buttons)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_pointer_buttons` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_pointer_buttons(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_pointer_buttons)
    }

    /// Returns the number of elements of the `buttons` field of a `xcb_input_xi_query_pointer_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_query_pointer_buttons_length(
        &self,
        r: *const xcb_input_xi_query_pointer_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_query_pointer_buttons_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_pointer_buttons_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_pointer_buttons_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_pointer_buttons_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `buttons` field of a `xcb_input_xi_query_pointer_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_query_pointer_buttons_end(
        &self,
        r: *const xcb_input_xi_query_pointer_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_xi_query_pointer_buttons_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_pointer_buttons_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_pointer_buttons_end(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_pointer_buttons_end)
    }

    /// Waits for the reply to a `Input::XIQueryPointer` request.
    #[inline]
    pub unsafe fn xcb_input_xi_query_pointer_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_xi_query_pointer_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_xi_query_pointer_reply_t {
        sym!(self, xcb_input_xi_query_pointer_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_pointer_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_pointer_reply(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_pointer_reply)
    }

    /// Sends a `Input::XIWarpPointer` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_input_xi_warp_pointer_checked(
        &self,
        c: *mut xcb_connection_t,
        src_win: xcb_window_t,
        dst_win: xcb_window_t,
        src_x: xcb_input_fp1616_t,
        src_y: xcb_input_fp1616_t,
        src_width: u16,
        src_height: u16,
        dst_x: xcb_input_fp1616_t,
        dst_y: xcb_input_fp1616_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_warp_pointer_checked)(
            c, src_win, dst_win, src_x, src_y, src_width, src_height, dst_x, dst_y, deviceid,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_xi_warp_pointer_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_warp_pointer_checked(&self) -> bool {
        has_sym!(self, xcb_input_xi_warp_pointer_checked)
    }

    /// Sends a `Input::XIWarpPointer` request (unchecked).
    #[inline]
    pub unsafe fn xcb_input_xi_warp_pointer(
        &self,
        c: *mut xcb_connection_t,
        src_win: xcb_window_t,
        dst_win: xcb_window_t,
        src_x: xcb_input_fp1616_t,
        src_y: xcb_input_fp1616_t,
        src_width: u16,
        src_height: u16,
        dst_x: xcb_input_fp1616_t,
        dst_y: xcb_input_fp1616_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_warp_pointer)(
            c, src_win, dst_win, src_x, src_y, src_width, src_height, dst_x, dst_y, deviceid,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_xi_warp_pointer` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_warp_pointer(&self) -> bool {
        has_sym!(self, xcb_input_xi_warp_pointer)
    }

    /// Sends a `Input::XIChangeCursor` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_input_xi_change_cursor_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        cursor: xcb_cursor_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_change_cursor_checked)(c, window, cursor, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_cursor_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_cursor_checked(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_cursor_checked)
    }

    /// Sends a `Input::XIChangeCursor` request (unchecked).
    #[inline]
    pub unsafe fn xcb_input_xi_change_cursor(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        cursor: xcb_cursor_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_change_cursor)(c, window, cursor, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_cursor` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_cursor(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_cursor)
    }

    /// Computes the size of a `xcb_input_add_master_t` object.
    #[inline]
    pub unsafe fn xcb_input_add_master_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_add_master_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_add_master_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_add_master_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_add_master_sizeof)
    }

    /// Returns a pointer to the `name` field of a `xcb_input_add_master_t` struct.
    #[inline]
    pub unsafe fn xcb_input_add_master_name(
        &self,
        r: *const xcb_input_add_master_t,
    ) -> *mut c_char {
        sym!(self, xcb_input_add_master_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_add_master_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_add_master_name(&self) -> bool {
        has_sym!(self, xcb_input_add_master_name)
    }

    /// Returns the number of elements of the `name` field of a `xcb_input_add_master_t` struct.
    #[inline]
    pub unsafe fn xcb_input_add_master_name_length(
        &self,
        r: *const xcb_input_add_master_t,
    ) -> c_int {
        sym!(self, xcb_input_add_master_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_add_master_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_add_master_name_length(&self) -> bool {
        has_sym!(self, xcb_input_add_master_name_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `name` field of a `xcb_input_add_master_t` struct.
    #[inline]
    pub unsafe fn xcb_input_add_master_name_end(
        &self,
        r: *const xcb_input_add_master_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_add_master_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_add_master_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_add_master_name_end(&self) -> bool {
        has_sym!(self, xcb_input_add_master_name_end)
    }

    /// Advances a `xcb_input_add_master_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_add_master_next(&self, i: *mut xcb_input_add_master_iterator_t) {
        sym!(self, xcb_input_add_master_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_add_master_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_add_master_next(&self) -> bool {
        has_sym!(self, xcb_input_add_master_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_add_master_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_add_master_end(
        &self,
        i: xcb_input_add_master_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_add_master_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_add_master_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_add_master_end(&self) -> bool {
        has_sym!(self, xcb_input_add_master_end)
    }

    /// Advances a `xcb_input_remove_master_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_remove_master_next(&self, i: *mut xcb_input_remove_master_iterator_t) {
        sym!(self, xcb_input_remove_master_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_remove_master_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_remove_master_next(&self) -> bool {
        has_sym!(self, xcb_input_remove_master_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_remove_master_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_remove_master_end(
        &self,
        i: xcb_input_remove_master_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_remove_master_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_remove_master_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_remove_master_end(&self) -> bool {
        has_sym!(self, xcb_input_remove_master_end)
    }

    /// Advances a `xcb_input_attach_slave_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_attach_slave_next(&self, i: *mut xcb_input_attach_slave_iterator_t) {
        sym!(self, xcb_input_attach_slave_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_attach_slave_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_attach_slave_next(&self) -> bool {
        has_sym!(self, xcb_input_attach_slave_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_attach_slave_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_attach_slave_end(
        &self,
        i: xcb_input_attach_slave_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_attach_slave_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_attach_slave_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_attach_slave_end(&self) -> bool {
        has_sym!(self, xcb_input_attach_slave_end)
    }

    /// Advances a `xcb_input_detach_slave_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_detach_slave_next(&self, i: *mut xcb_input_detach_slave_iterator_t) {
        sym!(self, xcb_input_detach_slave_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_detach_slave_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_detach_slave_next(&self) -> bool {
        has_sym!(self, xcb_input_detach_slave_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_detach_slave_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_detach_slave_end(
        &self,
        i: xcb_input_detach_slave_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_detach_slave_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_detach_slave_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_detach_slave_end(&self) -> bool {
        has_sym!(self, xcb_input_detach_slave_end)
    }

    /// Returns a pointer to the `name` field of a `xcb_input_hierarchy_change_data_add_master_t` struct.
    #[inline]
    pub unsafe fn xcb_input_hierarchy_change_data_add_master_name(
        &self,
        s: *const xcb_input_hierarchy_change_data_t,
    ) -> *mut c_char {
        sym!(self, xcb_input_hierarchy_change_data_add_master_name)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_change_data_add_master_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_change_data_add_master_name(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_change_data_add_master_name)
    }

    /// Returns the number of elements of the `name` field of a `xcb_input_hierarchy_change_data_add_master_t` struct.
    #[inline]
    pub unsafe fn xcb_input_hierarchy_change_data_add_master_name_length(
        &self,
        r: *const xcb_input_hierarchy_change_t,
        s: *const xcb_input_hierarchy_change_data_t,
    ) -> c_int {
        sym!(self, xcb_input_hierarchy_change_data_add_master_name_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_change_data_add_master_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_change_data_add_master_name_length(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_change_data_add_master_name_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `name` field of a `xcb_input_hierarchy_change_data_add_master_t` struct.
    #[inline]
    pub unsafe fn xcb_input_hierarchy_change_data_add_master_name_end(
        &self,
        r: *const xcb_input_hierarchy_change_t,
        s: *const xcb_input_hierarchy_change_data_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_hierarchy_change_data_add_master_name_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_change_data_add_master_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_change_data_add_master_name_end(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_change_data_add_master_name_end)
    }

    /// Serializes a `xcb_input_hierarchy_change_data_t` object.
    #[inline]
    pub unsafe fn xcb_input_hierarchy_change_data_serialize(
        &self,
        _buffer: *mut *mut c_void,
        type_: u16,
        _aux: *const xcb_input_hierarchy_change_data_t,
    ) -> c_int {
        sym!(self, xcb_input_hierarchy_change_data_serialize)(_buffer, type_, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_change_data_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_change_data_serialize(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_change_data_serialize)
    }

    /// Unpacks a `xcb_input_hierarchy_change_data_t` object.
    #[inline]
    pub unsafe fn xcb_input_hierarchy_change_data_unpack(
        &self,
        _buffer: *const c_void,
        type_: u16,
        _aux: *mut xcb_input_hierarchy_change_data_t,
    ) -> c_int {
        sym!(self, xcb_input_hierarchy_change_data_unpack)(_buffer, type_, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_change_data_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_change_data_unpack(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_change_data_unpack)
    }

    /// Computes the size of a `xcb_input_hierarchy_change_data_t` object.
    #[inline]
    pub unsafe fn xcb_input_hierarchy_change_data_sizeof(
        &self,
        _buffer: *const c_void,
        type_: u16,
    ) -> c_int {
        sym!(self, xcb_input_hierarchy_change_data_sizeof)(_buffer, type_)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_change_data_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_change_data_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_change_data_sizeof)
    }

    /// Computes the size of a `xcb_input_hierarchy_change_t` object.
    #[inline]
    pub unsafe fn xcb_input_hierarchy_change_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_hierarchy_change_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_change_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_change_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_change_sizeof)
    }

    /// Returns a pointer to the `data` field of a `xcb_input_hierarchy_change_t` struct.
    #[inline]
    pub unsafe fn xcb_input_hierarchy_change_data(
        &self,
        r: *const xcb_input_hierarchy_change_t,
    ) -> *mut c_void {
        sym!(self, xcb_input_hierarchy_change_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_change_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_change_data(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_change_data)
    }

    /// Advances a `xcb_input_hierarchy_change_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_hierarchy_change_next(
        &self,
        i: *mut xcb_input_hierarchy_change_iterator_t,
    ) {
        sym!(self, xcb_input_hierarchy_change_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_change_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_change_next(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_change_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_hierarchy_change_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_hierarchy_change_end(
        &self,
        i: xcb_input_hierarchy_change_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_hierarchy_change_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_change_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_change_end(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_change_end)
    }

    /// Computes the size of a `xcb_input_xi_change_hierarchy_request_t` object.
    #[inline]
    pub unsafe fn xcb_input_xi_change_hierarchy_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_xi_change_hierarchy_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_hierarchy_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_hierarchy_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_hierarchy_sizeof)
    }

    /// Sends a `Input::XIChangeHierarchy` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_input_xi_change_hierarchy_checked(
        &self,
        c: *mut xcb_connection_t,
        num_changes: u8,
        changes: *const xcb_input_hierarchy_change_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_change_hierarchy_checked)(c, num_changes, changes)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_hierarchy_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_hierarchy_checked(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_hierarchy_checked)
    }

    /// Sends a `Input::XIChangeHierarchy` request (unchecked).
    #[inline]
    pub unsafe fn xcb_input_xi_change_hierarchy(
        &self,
        c: *mut xcb_connection_t,
        num_changes: u8,
        changes: *const xcb_input_hierarchy_change_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_change_hierarchy)(c, num_changes, changes)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_hierarchy` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_hierarchy(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_hierarchy)
    }

    /// Returns the number of elements of the `changes` field of a `xcb_input_xi_change_hierarchy_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_change_hierarchy_changes_length(
        &self,
        r: *const xcb_input_xi_change_hierarchy_request_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_change_hierarchy_changes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_hierarchy_changes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_hierarchy_changes_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_hierarchy_changes_length)
    }

    /// Returns an iterator over the elements of the
    /// `changes` field of a `xcb_input_xi_change_hierarchy_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_change_hierarchy_changes_iterator(
        &self,
        r: *const xcb_input_xi_change_hierarchy_request_t,
    ) -> xcb_input_hierarchy_change_iterator_t {
        sym!(self, xcb_input_xi_change_hierarchy_changes_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_hierarchy_changes_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_hierarchy_changes_iterator(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_hierarchy_changes_iterator)
    }

    /// Sends a `Input::XISetClientPointer` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_input_xi_set_client_pointer_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_set_client_pointer_checked)(c, window, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_set_client_pointer_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_set_client_pointer_checked(&self) -> bool {
        has_sym!(self, xcb_input_xi_set_client_pointer_checked)
    }

    /// Sends a `Input::XISetClientPointer` request (unchecked).
    #[inline]
    pub unsafe fn xcb_input_xi_set_client_pointer(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_set_client_pointer)(c, window, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_set_client_pointer` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_set_client_pointer(&self) -> bool {
        has_sym!(self, xcb_input_xi_set_client_pointer)
    }

    /// Sends a `Input::XIGetClientPointer` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_xi_get_client_pointer_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_xi_get_client_pointer_reply`]: Self::xcb_input_xi_get_client_pointer_reply
    #[inline]
    pub unsafe fn xcb_input_xi_get_client_pointer(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_input_xi_get_client_pointer_cookie_t {
        sym!(self, xcb_input_xi_get_client_pointer)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_client_pointer` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_client_pointer(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_client_pointer)
    }

    /// Sends a `Input::XIGetClientPointer` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_xi_get_client_pointer_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_xi_get_client_pointer_reply`]: Self::xcb_input_xi_get_client_pointer_reply
    #[inline]
    pub unsafe fn xcb_input_xi_get_client_pointer_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_input_xi_get_client_pointer_cookie_t {
        sym!(self, xcb_input_xi_get_client_pointer_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_client_pointer_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_client_pointer_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_client_pointer_unchecked)
    }

    /// Waits for the reply to a `Input::XIGetClientPointer` request.
    #[inline]
    pub unsafe fn xcb_input_xi_get_client_pointer_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_xi_get_client_pointer_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_xi_get_client_pointer_reply_t {
        sym!(self, xcb_input_xi_get_client_pointer_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_client_pointer_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_client_pointer_reply(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_client_pointer_reply)
    }

    /// Computes the size of a `xcb_input_event_mask_t` object.
    #[inline]
    pub unsafe fn xcb_input_event_mask_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_event_mask_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_event_mask_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_event_mask_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_event_mask_sizeof)
    }

    /// Returns a pointer to the `mask` field of a `xcb_input_event_mask_t` struct.
    #[inline]
    pub unsafe fn xcb_input_event_mask_mask(&self, r: *const xcb_input_event_mask_t) -> *mut u32 {
        sym!(self, xcb_input_event_mask_mask)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_event_mask_mask` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_event_mask_mask(&self) -> bool {
        has_sym!(self, xcb_input_event_mask_mask)
    }

    /// Returns the number of elements of the `mask` field of a `xcb_input_event_mask_t` struct.
    #[inline]
    pub unsafe fn xcb_input_event_mask_mask_length(
        &self,
        r: *const xcb_input_event_mask_t,
    ) -> c_int {
        sym!(self, xcb_input_event_mask_mask_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_event_mask_mask_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_event_mask_mask_length(&self) -> bool {
        has_sym!(self, xcb_input_event_mask_mask_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `mask` field of a `xcb_input_event_mask_t` struct.
    #[inline]
    pub unsafe fn xcb_input_event_mask_mask_end(
        &self,
        r: *const xcb_input_event_mask_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_event_mask_mask_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_event_mask_mask_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_event_mask_mask_end(&self) -> bool {
        has_sym!(self, xcb_input_event_mask_mask_end)
    }

    /// Advances a `xcb_input_event_mask_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_event_mask_next(&self, i: *mut xcb_input_event_mask_iterator_t) {
        sym!(self, xcb_input_event_mask_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_event_mask_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_event_mask_next(&self) -> bool {
        has_sym!(self, xcb_input_event_mask_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_event_mask_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_event_mask_end(
        &self,
        i: xcb_input_event_mask_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_event_mask_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_event_mask_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_event_mask_end(&self) -> bool {
        has_sym!(self, xcb_input_event_mask_end)
    }

    /// Computes the size of a `xcb_input_xi_select_events_request_t` object.
    #[inline]
    pub unsafe fn xcb_input_xi_select_events_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_xi_select_events_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_select_events_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_select_events_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_select_events_sizeof)
    }

    /// Sends a `Input::XISelectEvents` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_input_xi_select_events_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        num_mask: u16,
        masks: *const xcb_input_event_mask_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_select_events_checked)(c, window, num_mask, masks)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_select_events_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_select_events_checked(&self) -> bool {
        has_sym!(self, xcb_input_xi_select_events_checked)
    }

    /// Sends a `Input::XISelectEvents` request (unchecked).
    #[inline]
    pub unsafe fn xcb_input_xi_select_events(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        num_mask: u16,
        masks: *const xcb_input_event_mask_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_select_events)(c, window, num_mask, masks)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_select_events` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_select_events(&self) -> bool {
        has_sym!(self, xcb_input_xi_select_events)
    }

    /// Returns the number of elements of the `masks` field of a `xcb_input_xi_select_events_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_select_events_masks_length(
        &self,
        r: *const xcb_input_xi_select_events_request_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_select_events_masks_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_select_events_masks_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_select_events_masks_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_select_events_masks_length)
    }

    /// Returns an iterator over the elements of the
    /// `masks` field of a `xcb_input_xi_select_events_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_select_events_masks_iterator(
        &self,
        r: *const xcb_input_xi_select_events_request_t,
    ) -> xcb_input_event_mask_iterator_t {
        sym!(self, xcb_input_xi_select_events_masks_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_select_events_masks_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_select_events_masks_iterator(&self) -> bool {
        has_sym!(self, xcb_input_xi_select_events_masks_iterator)
    }

    /// Sends a `Input::XIQueryVersion` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_xi_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_xi_query_version_reply`]: Self::xcb_input_xi_query_version_reply
    #[inline]
    pub unsafe fn xcb_input_xi_query_version(
        &self,
        c: *mut xcb_connection_t,
        major_version: u16,
        minor_version: u16,
    ) -> xcb_input_xi_query_version_cookie_t {
        sym!(self, xcb_input_xi_query_version)(c, major_version, minor_version)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_version` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_version(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_version)
    }

    /// Sends a `Input::XIQueryVersion` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_xi_query_version_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_xi_query_version_reply`]: Self::xcb_input_xi_query_version_reply
    #[inline]
    pub unsafe fn xcb_input_xi_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        major_version: u16,
        minor_version: u16,
    ) -> xcb_input_xi_query_version_cookie_t {
        sym!(self, xcb_input_xi_query_version_unchecked)(c, major_version, minor_version)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_version_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_version_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_version_unchecked)
    }

    /// Waits for the reply to a `Input::XIQueryVersion` request.
    #[inline]
    pub unsafe fn xcb_input_xi_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_xi_query_version_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_xi_query_version_reply_t {
        sym!(self, xcb_input_xi_query_version_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_version_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_version_reply(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_version_reply)
    }

    /// Computes the size of a `xcb_input_button_class_t` object.
    #[inline]
    pub unsafe fn xcb_input_button_class_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_button_class_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_button_class_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_class_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_button_class_sizeof)
    }

    /// Returns a pointer to the `state` field of a `xcb_input_button_class_t` struct.
    #[inline]
    pub unsafe fn xcb_input_button_class_state(
        &self,
        r: *const xcb_input_button_class_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_button_class_state)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_class_state` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_class_state(&self) -> bool {
        has_sym!(self, xcb_input_button_class_state)
    }

    /// Returns the number of elements of the `state` field of a `xcb_input_button_class_t` struct.
    #[inline]
    pub unsafe fn xcb_input_button_class_state_length(
        &self,
        r: *const xcb_input_button_class_t,
    ) -> c_int {
        sym!(self, xcb_input_button_class_state_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_class_state_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_class_state_length(&self) -> bool {
        has_sym!(self, xcb_input_button_class_state_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `state` field of a `xcb_input_button_class_t` struct.
    #[inline]
    pub unsafe fn xcb_input_button_class_state_end(
        &self,
        r: *const xcb_input_button_class_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_button_class_state_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_class_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_class_state_end(&self) -> bool {
        has_sym!(self, xcb_input_button_class_state_end)
    }

    /// Returns a pointer to the `labels` field of a `xcb_input_button_class_t` struct.
    #[inline]
    pub unsafe fn xcb_input_button_class_labels(
        &self,
        r: *const xcb_input_button_class_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_input_button_class_labels)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_class_labels` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_class_labels(&self) -> bool {
        has_sym!(self, xcb_input_button_class_labels)
    }

    /// Returns the number of elements of the `labels` field of a `xcb_input_button_class_t` struct.
    #[inline]
    pub unsafe fn xcb_input_button_class_labels_length(
        &self,
        r: *const xcb_input_button_class_t,
    ) -> c_int {
        sym!(self, xcb_input_button_class_labels_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_class_labels_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_class_labels_length(&self) -> bool {
        has_sym!(self, xcb_input_button_class_labels_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `labels` field of a `xcb_input_button_class_t` struct.
    #[inline]
    pub unsafe fn xcb_input_button_class_labels_end(
        &self,
        r: *const xcb_input_button_class_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_button_class_labels_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_class_labels_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_class_labels_end(&self) -> bool {
        has_sym!(self, xcb_input_button_class_labels_end)
    }

    /// Advances a `xcb_input_button_class_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_button_class_next(&self, i: *mut xcb_input_button_class_iterator_t) {
        sym!(self, xcb_input_button_class_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_button_class_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_class_next(&self) -> bool {
        has_sym!(self, xcb_input_button_class_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_button_class_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_button_class_end(
        &self,
        i: xcb_input_button_class_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_button_class_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_button_class_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_class_end(&self) -> bool {
        has_sym!(self, xcb_input_button_class_end)
    }

    /// Computes the size of a `xcb_input_key_class_t` object.
    #[inline]
    pub unsafe fn xcb_input_key_class_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_key_class_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_key_class_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_class_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_key_class_sizeof)
    }

    /// Returns a pointer to the `keys` field of a `xcb_input_key_class_t` struct.
    #[inline]
    pub unsafe fn xcb_input_key_class_keys(&self, r: *const xcb_input_key_class_t) -> *mut u32 {
        sym!(self, xcb_input_key_class_keys)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_key_class_keys` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_class_keys(&self) -> bool {
        has_sym!(self, xcb_input_key_class_keys)
    }

    /// Returns the number of elements of the `keys` field of a `xcb_input_key_class_t` struct.
    #[inline]
    pub unsafe fn xcb_input_key_class_keys_length(&self, r: *const xcb_input_key_class_t) -> c_int {
        sym!(self, xcb_input_key_class_keys_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_key_class_keys_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_class_keys_length(&self) -> bool {
        has_sym!(self, xcb_input_key_class_keys_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `keys` field of a `xcb_input_key_class_t` struct.
    #[inline]
    pub unsafe fn xcb_input_key_class_keys_end(
        &self,
        r: *const xcb_input_key_class_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_key_class_keys_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_key_class_keys_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_class_keys_end(&self) -> bool {
        has_sym!(self, xcb_input_key_class_keys_end)
    }

    /// Advances a `xcb_input_key_class_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_key_class_next(&self, i: *mut xcb_input_key_class_iterator_t) {
        sym!(self, xcb_input_key_class_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_key_class_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_class_next(&self) -> bool {
        has_sym!(self, xcb_input_key_class_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_key_class_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_key_class_end(
        &self,
        i: xcb_input_key_class_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_key_class_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_key_class_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_class_end(&self) -> bool {
        has_sym!(self, xcb_input_key_class_end)
    }

    /// Advances a `xcb_input_scroll_class_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_scroll_class_next(&self, i: *mut xcb_input_scroll_class_iterator_t) {
        sym!(self, xcb_input_scroll_class_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_scroll_class_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_scroll_class_next(&self) -> bool {
        has_sym!(self, xcb_input_scroll_class_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_scroll_class_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_scroll_class_end(
        &self,
        i: xcb_input_scroll_class_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_scroll_class_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_scroll_class_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_scroll_class_end(&self) -> bool {
        has_sym!(self, xcb_input_scroll_class_end)
    }

    /// Advances a `xcb_input_touch_class_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_touch_class_next(&self, i: *mut xcb_input_touch_class_iterator_t) {
        sym!(self, xcb_input_touch_class_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_touch_class_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_touch_class_next(&self) -> bool {
        has_sym!(self, xcb_input_touch_class_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_touch_class_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_touch_class_end(
        &self,
        i: xcb_input_touch_class_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_touch_class_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_touch_class_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_touch_class_end(&self) -> bool {
        has_sym!(self, xcb_input_touch_class_end)
    }

    /// Advances a `xcb_input_valuator_class_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_valuator_class_next(
        &self,
        i: *mut xcb_input_valuator_class_iterator_t,
    ) {
        sym!(self, xcb_input_valuator_class_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_valuator_class_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_valuator_class_next(&self) -> bool {
        has_sym!(self, xcb_input_valuator_class_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_valuator_class_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_valuator_class_end(
        &self,
        i: xcb_input_valuator_class_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_valuator_class_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_valuator_class_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_valuator_class_end(&self) -> bool {
        has_sym!(self, xcb_input_valuator_class_end)
    }

    /// Returns a pointer to the `keys` field of a `xcb_input_device_class_data_key_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_class_data_key_keys(
        &self,
        s: *const xcb_input_device_class_data_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_device_class_data_key_keys)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_data_key_keys` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_data_key_keys(&self) -> bool {
        has_sym!(self, xcb_input_device_class_data_key_keys)
    }

    /// Returns the number of elements of the `keys` field of a `xcb_input_device_class_data_key_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_class_data_key_keys_length(
        &self,
        r: *const xcb_input_device_class_t,
        s: *const xcb_input_device_class_data_t,
    ) -> c_int {
        sym!(self, xcb_input_device_class_data_key_keys_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_data_key_keys_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_data_key_keys_length(&self) -> bool {
        has_sym!(self, xcb_input_device_class_data_key_keys_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `keys` field of a `xcb_input_device_class_data_key_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_class_data_key_keys_end(
        &self,
        r: *const xcb_input_device_class_t,
        s: *const xcb_input_device_class_data_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_class_data_key_keys_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_data_key_keys_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_data_key_keys_end(&self) -> bool {
        has_sym!(self, xcb_input_device_class_data_key_keys_end)
    }

    /// Returns a pointer to the `state` field of a `xcb_input_device_class_data_button_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_class_data_button_state(
        &self,
        s: *const xcb_input_device_class_data_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_device_class_data_button_state)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_data_button_state` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_data_button_state(&self) -> bool {
        has_sym!(self, xcb_input_device_class_data_button_state)
    }

    /// Returns the number of elements of the `state` field of a `xcb_input_device_class_data_button_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_class_data_button_state_length(
        &self,
        r: *const xcb_input_device_class_t,
        s: *const xcb_input_device_class_data_t,
    ) -> c_int {
        sym!(self, xcb_input_device_class_data_button_state_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_data_button_state_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_data_button_state_length(&self) -> bool {
        has_sym!(self, xcb_input_device_class_data_button_state_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `state` field of a `xcb_input_device_class_data_button_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_class_data_button_state_end(
        &self,
        r: *const xcb_input_device_class_t,
        s: *const xcb_input_device_class_data_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_class_data_button_state_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_data_button_state_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_data_button_state_end(&self) -> bool {
        has_sym!(self, xcb_input_device_class_data_button_state_end)
    }

    /// Returns a pointer to the `labels` field of a `xcb_input_device_class_data_button_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_class_data_button_labels(
        &self,
        s: *const xcb_input_device_class_data_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_input_device_class_data_button_labels)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_data_button_labels` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_data_button_labels(&self) -> bool {
        has_sym!(self, xcb_input_device_class_data_button_labels)
    }

    /// Returns the number of elements of the `labels` field of a `xcb_input_device_class_data_button_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_class_data_button_labels_length(
        &self,
        r: *const xcb_input_device_class_t,
        s: *const xcb_input_device_class_data_t,
    ) -> c_int {
        sym!(self, xcb_input_device_class_data_button_labels_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_data_button_labels_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_data_button_labels_length(&self) -> bool {
        has_sym!(self, xcb_input_device_class_data_button_labels_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `labels` field of a `xcb_input_device_class_data_button_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_class_data_button_labels_end(
        &self,
        r: *const xcb_input_device_class_t,
        s: *const xcb_input_device_class_data_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_class_data_button_labels_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_data_button_labels_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_data_button_labels_end(&self) -> bool {
        has_sym!(self, xcb_input_device_class_data_button_labels_end)
    }

    /// Serializes a `xcb_input_device_class_data_t` object.
    #[inline]
    pub unsafe fn xcb_input_device_class_data_serialize(
        &self,
        _buffer: *mut *mut c_void,
        type_: u16,
        _aux: *const xcb_input_device_class_data_t,
    ) -> c_int {
        sym!(self, xcb_input_device_class_data_serialize)(_buffer, type_, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_data_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_data_serialize(&self) -> bool {
        has_sym!(self, xcb_input_device_class_data_serialize)
    }

    /// Unpacks a `xcb_input_device_class_data_t` object.
    #[inline]
    pub unsafe fn xcb_input_device_class_data_unpack(
        &self,
        _buffer: *const c_void,
        type_: u16,
        _aux: *mut xcb_input_device_class_data_t,
    ) -> c_int {
        sym!(self, xcb_input_device_class_data_unpack)(_buffer, type_, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_data_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_data_unpack(&self) -> bool {
        has_sym!(self, xcb_input_device_class_data_unpack)
    }

    /// Computes the size of a `xcb_input_device_class_data_t` object.
    #[inline]
    pub unsafe fn xcb_input_device_class_data_sizeof(
        &self,
        _buffer: *const c_void,
        type_: u16,
    ) -> c_int {
        sym!(self, xcb_input_device_class_data_sizeof)(_buffer, type_)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_data_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_data_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_device_class_data_sizeof)
    }

    /// Computes the size of a `xcb_input_device_class_t` object.
    #[inline]
    pub unsafe fn xcb_input_device_class_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_device_class_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_device_class_sizeof)
    }

    /// Returns a pointer to the `data` field of a `xcb_input_device_class_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_class_data(
        &self,
        r: *const xcb_input_device_class_t,
    ) -> *mut c_void {
        sym!(self, xcb_input_device_class_data)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_data` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_data(&self) -> bool {
        has_sym!(self, xcb_input_device_class_data)
    }

    /// Advances a `xcb_input_device_class_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_device_class_next(&self, i: *mut xcb_input_device_class_iterator_t) {
        sym!(self, xcb_input_device_class_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_next(&self) -> bool {
        has_sym!(self, xcb_input_device_class_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_device_class_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_device_class_end(
        &self,
        i: xcb_input_device_class_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_device_class_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_device_class_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_class_end(&self) -> bool {
        has_sym!(self, xcb_input_device_class_end)
    }

    /// Computes the size of a `xcb_input_xi_device_info_t` object.
    #[inline]
    pub unsafe fn xcb_input_xi_device_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_xi_device_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_device_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_device_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_device_info_sizeof)
    }

    /// Returns a pointer to the `name` field of a `xcb_input_xi_device_info_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_device_info_name(
        &self,
        r: *const xcb_input_xi_device_info_t,
    ) -> *mut c_char {
        sym!(self, xcb_input_xi_device_info_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_device_info_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_device_info_name(&self) -> bool {
        has_sym!(self, xcb_input_xi_device_info_name)
    }

    /// Returns the number of elements of the `name` field of a `xcb_input_xi_device_info_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_device_info_name_length(
        &self,
        r: *const xcb_input_xi_device_info_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_device_info_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_device_info_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_device_info_name_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_device_info_name_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `name` field of a `xcb_input_xi_device_info_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_device_info_name_end(
        &self,
        r: *const xcb_input_xi_device_info_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_xi_device_info_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_device_info_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_device_info_name_end(&self) -> bool {
        has_sym!(self, xcb_input_xi_device_info_name_end)
    }

    /// Returns the number of elements of the `classes` field of a `xcb_input_xi_device_info_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_device_info_classes_length(
        &self,
        r: *const xcb_input_xi_device_info_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_device_info_classes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_device_info_classes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_device_info_classes_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_device_info_classes_length)
    }

    /// Returns an iterator over the elements of the
    /// `classes` field of a `xcb_input_xi_device_info_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_device_info_classes_iterator(
        &self,
        r: *const xcb_input_xi_device_info_t,
    ) -> xcb_input_device_class_iterator_t {
        sym!(self, xcb_input_xi_device_info_classes_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_device_info_classes_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_device_info_classes_iterator(&self) -> bool {
        has_sym!(self, xcb_input_xi_device_info_classes_iterator)
    }

    /// Advances a `xcb_input_xi_device_info_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_xi_device_info_next(
        &self,
        i: *mut xcb_input_xi_device_info_iterator_t,
    ) {
        sym!(self, xcb_input_xi_device_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_device_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_device_info_next(&self) -> bool {
        has_sym!(self, xcb_input_xi_device_info_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_xi_device_info_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_xi_device_info_end(
        &self,
        i: xcb_input_xi_device_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_xi_device_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_device_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_device_info_end(&self) -> bool {
        has_sym!(self, xcb_input_xi_device_info_end)
    }

    /// Computes the size of a `xcb_input_xi_query_device_reply_t` object.
    #[inline]
    pub unsafe fn xcb_input_xi_query_device_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_xi_query_device_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_device_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_device_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_device_sizeof)
    }

    /// Sends a `Input::XIQueryDevice` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_xi_query_device_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_xi_query_device_reply`]: Self::xcb_input_xi_query_device_reply
    #[inline]
    pub unsafe fn xcb_input_xi_query_device(
        &self,
        c: *mut xcb_connection_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_input_xi_query_device_cookie_t {
        sym!(self, xcb_input_xi_query_device)(c, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_device` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_device(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_device)
    }

    /// Sends a `Input::XIQueryDevice` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_xi_query_device_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_xi_query_device_reply`]: Self::xcb_input_xi_query_device_reply
    #[inline]
    pub unsafe fn xcb_input_xi_query_device_unchecked(
        &self,
        c: *mut xcb_connection_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_input_xi_query_device_cookie_t {
        sym!(self, xcb_input_xi_query_device_unchecked)(c, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_device_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_device_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_device_unchecked)
    }

    /// Returns the number of elements of the `infos` field of a `xcb_input_xi_query_device_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_query_device_infos_length(
        &self,
        r: *const xcb_input_xi_query_device_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_query_device_infos_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_device_infos_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_device_infos_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_device_infos_length)
    }

    /// Returns an iterator over the elements of the
    /// `infos` field of a `xcb_input_xi_query_device_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_query_device_infos_iterator(
        &self,
        r: *const xcb_input_xi_query_device_reply_t,
    ) -> xcb_input_xi_device_info_iterator_t {
        sym!(self, xcb_input_xi_query_device_infos_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_device_infos_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_device_infos_iterator(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_device_infos_iterator)
    }

    /// Waits for the reply to a `Input::XIQueryDevice` request.
    #[inline]
    pub unsafe fn xcb_input_xi_query_device_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_xi_query_device_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_xi_query_device_reply_t {
        sym!(self, xcb_input_xi_query_device_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_query_device_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_query_device_reply(&self) -> bool {
        has_sym!(self, xcb_input_xi_query_device_reply)
    }

    /// Sends a `Input::XISetFocus` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_input_xi_set_focus_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        time: xcb_timestamp_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_set_focus_checked)(c, window, time, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_set_focus_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_set_focus_checked(&self) -> bool {
        has_sym!(self, xcb_input_xi_set_focus_checked)
    }

    /// Sends a `Input::XISetFocus` request (unchecked).
    #[inline]
    pub unsafe fn xcb_input_xi_set_focus(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        time: xcb_timestamp_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_set_focus)(c, window, time, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_set_focus` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_set_focus(&self) -> bool {
        has_sym!(self, xcb_input_xi_set_focus)
    }

    /// Sends a `Input::XIGetFocus` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_xi_get_focus_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_xi_get_focus_reply`]: Self::xcb_input_xi_get_focus_reply
    #[inline]
    pub unsafe fn xcb_input_xi_get_focus(
        &self,
        c: *mut xcb_connection_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_input_xi_get_focus_cookie_t {
        sym!(self, xcb_input_xi_get_focus)(c, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_focus` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_focus(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_focus)
    }

    /// Sends a `Input::XIGetFocus` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_xi_get_focus_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_xi_get_focus_reply`]: Self::xcb_input_xi_get_focus_reply
    #[inline]
    pub unsafe fn xcb_input_xi_get_focus_unchecked(
        &self,
        c: *mut xcb_connection_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_input_xi_get_focus_cookie_t {
        sym!(self, xcb_input_xi_get_focus_unchecked)(c, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_focus_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_focus_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_focus_unchecked)
    }

    /// Waits for the reply to a `Input::XIGetFocus` request.
    #[inline]
    pub unsafe fn xcb_input_xi_get_focus_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_xi_get_focus_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_xi_get_focus_reply_t {
        sym!(self, xcb_input_xi_get_focus_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_focus_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_focus_reply(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_focus_reply)
    }

    /// Computes the size of a `xcb_input_xi_grab_device_request_t` object.
    #[inline]
    pub unsafe fn xcb_input_xi_grab_device_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_xi_grab_device_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_grab_device_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_grab_device_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_grab_device_sizeof)
    }

    /// Sends a `Input::XIGrabDevice` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_xi_grab_device_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_xi_grab_device_reply`]: Self::xcb_input_xi_grab_device_reply
    #[inline]
    pub unsafe fn xcb_input_xi_grab_device(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        time: xcb_timestamp_t,
        cursor: xcb_cursor_t,
        deviceid: xcb_input_device_id_t,
        mode: u8,
        paired_device_mode: u8,
        owner_events: u8,
        mask_len: u16,
        mask: *const u32,
    ) -> xcb_input_xi_grab_device_cookie_t {
        sym!(self, xcb_input_xi_grab_device)(
            c,
            window,
            time,
            cursor,
            deviceid,
            mode,
            paired_device_mode,
            owner_events,
            mask_len,
            mask,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_xi_grab_device` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_grab_device(&self) -> bool {
        has_sym!(self, xcb_input_xi_grab_device)
    }

    /// Sends a `Input::XIGrabDevice` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_xi_grab_device_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_xi_grab_device_reply`]: Self::xcb_input_xi_grab_device_reply
    #[inline]
    pub unsafe fn xcb_input_xi_grab_device_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        time: xcb_timestamp_t,
        cursor: xcb_cursor_t,
        deviceid: xcb_input_device_id_t,
        mode: u8,
        paired_device_mode: u8,
        owner_events: u8,
        mask_len: u16,
        mask: *const u32,
    ) -> xcb_input_xi_grab_device_cookie_t {
        sym!(self, xcb_input_xi_grab_device_unchecked)(
            c,
            window,
            time,
            cursor,
            deviceid,
            mode,
            paired_device_mode,
            owner_events,
            mask_len,
            mask,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_xi_grab_device_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_grab_device_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_xi_grab_device_unchecked)
    }

    /// Waits for the reply to a `Input::XIGrabDevice` request.
    #[inline]
    pub unsafe fn xcb_input_xi_grab_device_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_xi_grab_device_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_xi_grab_device_reply_t {
        sym!(self, xcb_input_xi_grab_device_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_grab_device_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_grab_device_reply(&self) -> bool {
        has_sym!(self, xcb_input_xi_grab_device_reply)
    }

    /// Sends a `Input::XIUngrabDevice` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_input_xi_ungrab_device_checked(
        &self,
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_ungrab_device_checked)(c, time, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_ungrab_device_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_ungrab_device_checked(&self) -> bool {
        has_sym!(self, xcb_input_xi_ungrab_device_checked)
    }

    /// Sends a `Input::XIUngrabDevice` request (unchecked).
    #[inline]
    pub unsafe fn xcb_input_xi_ungrab_device(
        &self,
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_ungrab_device)(c, time, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_ungrab_device` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_ungrab_device(&self) -> bool {
        has_sym!(self, xcb_input_xi_ungrab_device)
    }

    /// Sends a `Input::XIAllowEvents` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_input_xi_allow_events_checked(
        &self,
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
        deviceid: xcb_input_device_id_t,
        event_mode: u8,
        touchid: u32,
        grab_window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_allow_events_checked)(
            c,
            time,
            deviceid,
            event_mode,
            touchid,
            grab_window,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_xi_allow_events_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_allow_events_checked(&self) -> bool {
        has_sym!(self, xcb_input_xi_allow_events_checked)
    }

    /// Sends a `Input::XIAllowEvents` request (unchecked).
    #[inline]
    pub unsafe fn xcb_input_xi_allow_events(
        &self,
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
        deviceid: xcb_input_device_id_t,
        event_mode: u8,
        touchid: u32,
        grab_window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_allow_events)(c, time, deviceid, event_mode, touchid, grab_window)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_allow_events` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_allow_events(&self) -> bool {
        has_sym!(self, xcb_input_xi_allow_events)
    }

    /// Advances a `xcb_input_grab_modifier_info_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_grab_modifier_info_next(
        &self,
        i: *mut xcb_input_grab_modifier_info_iterator_t,
    ) {
        sym!(self, xcb_input_grab_modifier_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_grab_modifier_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_modifier_info_next(&self) -> bool {
        has_sym!(self, xcb_input_grab_modifier_info_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_grab_modifier_info_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_grab_modifier_info_end(
        &self,
        i: xcb_input_grab_modifier_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_grab_modifier_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_grab_modifier_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_grab_modifier_info_end(&self) -> bool {
        has_sym!(self, xcb_input_grab_modifier_info_end)
    }

    /// Computes the size of a `xcb_input_xi_passive_grab_device_request_t` object.
    #[inline]
    pub unsafe fn xcb_input_xi_passive_grab_device_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_xi_passive_grab_device_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_passive_grab_device_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_passive_grab_device_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_passive_grab_device_sizeof)
    }

    /// Sends a `Input::XIPassiveGrabDevice` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_xi_passive_grab_device_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_xi_passive_grab_device_reply`]: Self::xcb_input_xi_passive_grab_device_reply
    #[inline]
    pub unsafe fn xcb_input_xi_passive_grab_device(
        &self,
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
        grab_window: xcb_window_t,
        cursor: xcb_cursor_t,
        detail: u32,
        deviceid: xcb_input_device_id_t,
        num_modifiers: u16,
        mask_len: u16,
        grab_type: u8,
        grab_mode: u8,
        paired_device_mode: u8,
        owner_events: u8,
        mask: *const u32,
        modifiers: *const u32,
    ) -> xcb_input_xi_passive_grab_device_cookie_t {
        sym!(self, xcb_input_xi_passive_grab_device)(
            c,
            time,
            grab_window,
            cursor,
            detail,
            deviceid,
            num_modifiers,
            mask_len,
            grab_type,
            grab_mode,
            paired_device_mode,
            owner_events,
            mask,
            modifiers,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_xi_passive_grab_device` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_passive_grab_device(&self) -> bool {
        has_sym!(self, xcb_input_xi_passive_grab_device)
    }

    /// Sends a `Input::XIPassiveGrabDevice` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_xi_passive_grab_device_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_xi_passive_grab_device_reply`]: Self::xcb_input_xi_passive_grab_device_reply
    #[inline]
    pub unsafe fn xcb_input_xi_passive_grab_device_unchecked(
        &self,
        c: *mut xcb_connection_t,
        time: xcb_timestamp_t,
        grab_window: xcb_window_t,
        cursor: xcb_cursor_t,
        detail: u32,
        deviceid: xcb_input_device_id_t,
        num_modifiers: u16,
        mask_len: u16,
        grab_type: u8,
        grab_mode: u8,
        paired_device_mode: u8,
        owner_events: u8,
        mask: *const u32,
        modifiers: *const u32,
    ) -> xcb_input_xi_passive_grab_device_cookie_t {
        sym!(self, xcb_input_xi_passive_grab_device_unchecked)(
            c,
            time,
            grab_window,
            cursor,
            detail,
            deviceid,
            num_modifiers,
            mask_len,
            grab_type,
            grab_mode,
            paired_device_mode,
            owner_events,
            mask,
            modifiers,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_xi_passive_grab_device_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_passive_grab_device_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_xi_passive_grab_device_unchecked)
    }

    /// Returns a pointer to the `modifiers` field of a `xcb_input_xi_passive_grab_device_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_passive_grab_device_modifiers(
        &self,
        r: *const xcb_input_xi_passive_grab_device_reply_t,
    ) -> *mut xcb_input_grab_modifier_info_t {
        sym!(self, xcb_input_xi_passive_grab_device_modifiers)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_passive_grab_device_modifiers` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_passive_grab_device_modifiers(&self) -> bool {
        has_sym!(self, xcb_input_xi_passive_grab_device_modifiers)
    }

    /// Returns the number of elements of the `modifiers` field of a `xcb_input_xi_passive_grab_device_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_passive_grab_device_modifiers_length(
        &self,
        r: *const xcb_input_xi_passive_grab_device_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_passive_grab_device_modifiers_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_passive_grab_device_modifiers_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_passive_grab_device_modifiers_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_passive_grab_device_modifiers_length)
    }

    /// Returns an iterator over the elements of the
    /// `modifiers` field of a `xcb_input_xi_passive_grab_device_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_passive_grab_device_modifiers_iterator(
        &self,
        r: *const xcb_input_xi_passive_grab_device_reply_t,
    ) -> xcb_input_grab_modifier_info_iterator_t {
        sym!(self, xcb_input_xi_passive_grab_device_modifiers_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_passive_grab_device_modifiers_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_passive_grab_device_modifiers_iterator(&self) -> bool {
        has_sym!(self, xcb_input_xi_passive_grab_device_modifiers_iterator)
    }

    /// Waits for the reply to a `Input::XIPassiveGrabDevice` request.
    #[inline]
    pub unsafe fn xcb_input_xi_passive_grab_device_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_xi_passive_grab_device_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_xi_passive_grab_device_reply_t {
        sym!(self, xcb_input_xi_passive_grab_device_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_passive_grab_device_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_passive_grab_device_reply(&self) -> bool {
        has_sym!(self, xcb_input_xi_passive_grab_device_reply)
    }

    /// Computes the size of a `xcb_input_xi_passive_ungrab_device_request_t` object.
    #[inline]
    pub unsafe fn xcb_input_xi_passive_ungrab_device_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_input_xi_passive_ungrab_device_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_passive_ungrab_device_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_passive_ungrab_device_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_passive_ungrab_device_sizeof)
    }

    /// Sends a `Input::XIPassiveUngrabDevice` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_input_xi_passive_ungrab_device_checked(
        &self,
        c: *mut xcb_connection_t,
        grab_window: xcb_window_t,
        detail: u32,
        deviceid: xcb_input_device_id_t,
        num_modifiers: u16,
        grab_type: u8,
        modifiers: *const u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_passive_ungrab_device_checked)(
            c,
            grab_window,
            detail,
            deviceid,
            num_modifiers,
            grab_type,
            modifiers,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_xi_passive_ungrab_device_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_passive_ungrab_device_checked(&self) -> bool {
        has_sym!(self, xcb_input_xi_passive_ungrab_device_checked)
    }

    /// Sends a `Input::XIPassiveUngrabDevice` request (unchecked).
    #[inline]
    pub unsafe fn xcb_input_xi_passive_ungrab_device(
        &self,
        c: *mut xcb_connection_t,
        grab_window: xcb_window_t,
        detail: u32,
        deviceid: xcb_input_device_id_t,
        num_modifiers: u16,
        grab_type: u8,
        modifiers: *const u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_passive_ungrab_device)(
            c,
            grab_window,
            detail,
            deviceid,
            num_modifiers,
            grab_type,
            modifiers,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_xi_passive_ungrab_device` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_passive_ungrab_device(&self) -> bool {
        has_sym!(self, xcb_input_xi_passive_ungrab_device)
    }

    /// Returns a pointer to the `modifiers` field of a `xcb_input_xi_passive_ungrab_device_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_passive_ungrab_device_modifiers(
        &self,
        r: *const xcb_input_xi_passive_ungrab_device_request_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_xi_passive_ungrab_device_modifiers)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_passive_ungrab_device_modifiers` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_passive_ungrab_device_modifiers(&self) -> bool {
        has_sym!(self, xcb_input_xi_passive_ungrab_device_modifiers)
    }

    /// Returns the number of elements of the `modifiers` field of a `xcb_input_xi_passive_ungrab_device_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_passive_ungrab_device_modifiers_length(
        &self,
        r: *const xcb_input_xi_passive_ungrab_device_request_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_passive_ungrab_device_modifiers_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_passive_ungrab_device_modifiers_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_passive_ungrab_device_modifiers_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_passive_ungrab_device_modifiers_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `modifiers` field of a `xcb_input_xi_passive_ungrab_device_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_passive_ungrab_device_modifiers_end(
        &self,
        r: *const xcb_input_xi_passive_ungrab_device_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_xi_passive_ungrab_device_modifiers_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_passive_ungrab_device_modifiers_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_passive_ungrab_device_modifiers_end(&self) -> bool {
        has_sym!(self, xcb_input_xi_passive_ungrab_device_modifiers_end)
    }

    /// Computes the size of a `xcb_input_xi_list_properties_reply_t` object.
    #[inline]
    pub unsafe fn xcb_input_xi_list_properties_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_xi_list_properties_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_list_properties_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_list_properties_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_list_properties_sizeof)
    }

    /// Sends a `Input::XIListProperties` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_xi_list_properties_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_xi_list_properties_reply`]: Self::xcb_input_xi_list_properties_reply
    #[inline]
    pub unsafe fn xcb_input_xi_list_properties(
        &self,
        c: *mut xcb_connection_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_input_xi_list_properties_cookie_t {
        sym!(self, xcb_input_xi_list_properties)(c, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_list_properties` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_list_properties(&self) -> bool {
        has_sym!(self, xcb_input_xi_list_properties)
    }

    /// Sends a `Input::XIListProperties` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_xi_list_properties_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_xi_list_properties_reply`]: Self::xcb_input_xi_list_properties_reply
    #[inline]
    pub unsafe fn xcb_input_xi_list_properties_unchecked(
        &self,
        c: *mut xcb_connection_t,
        deviceid: xcb_input_device_id_t,
    ) -> xcb_input_xi_list_properties_cookie_t {
        sym!(self, xcb_input_xi_list_properties_unchecked)(c, deviceid)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_list_properties_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_list_properties_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_xi_list_properties_unchecked)
    }

    /// Returns a pointer to the `properties` field of a `xcb_input_xi_list_properties_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_list_properties_properties(
        &self,
        r: *const xcb_input_xi_list_properties_reply_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_input_xi_list_properties_properties)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_list_properties_properties` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_list_properties_properties(&self) -> bool {
        has_sym!(self, xcb_input_xi_list_properties_properties)
    }

    /// Returns the number of elements of the `properties` field of a `xcb_input_xi_list_properties_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_list_properties_properties_length(
        &self,
        r: *const xcb_input_xi_list_properties_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_list_properties_properties_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_list_properties_properties_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_list_properties_properties_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_list_properties_properties_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `properties` field of a `xcb_input_xi_list_properties_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_list_properties_properties_end(
        &self,
        r: *const xcb_input_xi_list_properties_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_xi_list_properties_properties_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_list_properties_properties_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_list_properties_properties_end(&self) -> bool {
        has_sym!(self, xcb_input_xi_list_properties_properties_end)
    }

    /// Waits for the reply to a `Input::XIListProperties` request.
    #[inline]
    pub unsafe fn xcb_input_xi_list_properties_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_xi_list_properties_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_xi_list_properties_reply_t {
        sym!(self, xcb_input_xi_list_properties_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_list_properties_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_list_properties_reply(&self) -> bool {
        has_sym!(self, xcb_input_xi_list_properties_reply)
    }

    /// Returns a pointer to the `data8` field of a `xcb_input_xi_change_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_change_property_items_data_8(
        &self,
        s: *const xcb_input_xi_change_property_items_t,
    ) -> *mut u8 {
        sym!(self, xcb_input_xi_change_property_items_data_8)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_items_data_8` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_items_data_8(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_items_data_8)
    }

    /// Returns the number of elements of the `data8` field of a `xcb_input_xi_change_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_change_property_items_data_8_length(
        &self,
        r: *const xcb_input_xi_change_property_request_t,
        s: *const xcb_input_xi_change_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_change_property_items_data_8_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_items_data_8_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_items_data_8_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_items_data_8_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `data8` field of a `xcb_input_xi_change_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_change_property_items_data_8_end(
        &self,
        r: *const xcb_input_xi_change_property_request_t,
        s: *const xcb_input_xi_change_property_items_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_xi_change_property_items_data_8_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_items_data_8_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_items_data_8_end(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_items_data_8_end)
    }

    /// Returns a pointer to the `data16` field of a `xcb_input_xi_change_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_change_property_items_data_16(
        &self,
        s: *const xcb_input_xi_change_property_items_t,
    ) -> *mut u16 {
        sym!(self, xcb_input_xi_change_property_items_data_16)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_items_data_16` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_items_data_16(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_items_data_16)
    }

    /// Returns the number of elements of the `data16` field of a `xcb_input_xi_change_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_change_property_items_data_16_length(
        &self,
        r: *const xcb_input_xi_change_property_request_t,
        s: *const xcb_input_xi_change_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_change_property_items_data_16_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_items_data_16_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_items_data_16_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_items_data_16_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `data16` field of a `xcb_input_xi_change_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_change_property_items_data_16_end(
        &self,
        r: *const xcb_input_xi_change_property_request_t,
        s: *const xcb_input_xi_change_property_items_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_xi_change_property_items_data_16_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_items_data_16_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_items_data_16_end(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_items_data_16_end)
    }

    /// Returns a pointer to the `data32` field of a `xcb_input_xi_change_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_change_property_items_data_32(
        &self,
        s: *const xcb_input_xi_change_property_items_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_xi_change_property_items_data_32)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_items_data_32` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_items_data_32(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_items_data_32)
    }

    /// Returns the number of elements of the `data32` field of a `xcb_input_xi_change_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_change_property_items_data_32_length(
        &self,
        r: *const xcb_input_xi_change_property_request_t,
        s: *const xcb_input_xi_change_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_change_property_items_data_32_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_items_data_32_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_items_data_32_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_items_data_32_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `data32` field of a `xcb_input_xi_change_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_change_property_items_data_32_end(
        &self,
        r: *const xcb_input_xi_change_property_request_t,
        s: *const xcb_input_xi_change_property_items_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_xi_change_property_items_data_32_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_items_data_32_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_items_data_32_end(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_items_data_32_end)
    }

    /// Serializes a `xcb_input_xi_change_property_items_t` object.
    #[inline]
    pub unsafe fn xcb_input_xi_change_property_items_serialize(
        &self,
        _buffer: *mut *mut c_void,
        num_items: u32,
        format: u8,
        _aux: *const xcb_input_xi_change_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_change_property_items_serialize)(_buffer, num_items, format, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_items_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_items_serialize(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_items_serialize)
    }

    /// Unpacks a `xcb_input_xi_change_property_items_t` object.
    #[inline]
    pub unsafe fn xcb_input_xi_change_property_items_unpack(
        &self,
        _buffer: *const c_void,
        num_items: u32,
        format: u8,
        _aux: *mut xcb_input_xi_change_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_change_property_items_unpack)(_buffer, num_items, format, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_items_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_items_unpack(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_items_unpack)
    }

    /// Computes the size of a `xcb_input_xi_change_property_items_t` object.
    #[inline]
    pub unsafe fn xcb_input_xi_change_property_items_sizeof(
        &self,
        _buffer: *const c_void,
        num_items: u32,
        format: u8,
    ) -> c_int {
        sym!(self, xcb_input_xi_change_property_items_sizeof)(_buffer, num_items, format)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_items_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_items_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_items_sizeof)
    }

    /// Computes the size of a `xcb_input_xi_change_property_request_t` object.
    #[inline]
    pub unsafe fn xcb_input_xi_change_property_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_xi_change_property_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_sizeof)
    }

    /// Sends a `Input::XIChangeProperty` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    ///
    /// There is an auxiliary version of this function: [`xcb_input_xi_change_property_aux_checked`].
    ///
    /// [`xcb_input_xi_change_property_aux_checked`]: Self::xcb_input_xi_change_property_aux_checked
    #[inline]
    pub unsafe fn xcb_input_xi_change_property_checked(
        &self,
        c: *mut xcb_connection_t,
        deviceid: xcb_input_device_id_t,
        mode: u8,
        format: u8,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        num_items: u32,
        items: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_change_property_checked)(
            c, deviceid, mode, format, property, type_, num_items, items,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_checked(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_checked)
    }

    /// Sends a `Input::XIChangeProperty` request (unchecked).
    ///
    /// There is an auxiliary version of this function: [`xcb_input_xi_change_property_aux`].
    ///
    /// [`xcb_input_xi_change_property_aux`]: Self::xcb_input_xi_change_property_aux
    #[inline]
    pub unsafe fn xcb_input_xi_change_property(
        &self,
        c: *mut xcb_connection_t,
        deviceid: xcb_input_device_id_t,
        mode: u8,
        format: u8,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        num_items: u32,
        items: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_change_property)(
            c, deviceid, mode, format, property, type_, num_items, items,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property)
    }

    /// Sends a `Input::XIChangeProperty` request (checked) (aux).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_input_xi_change_property_aux_checked(
        &self,
        c: *mut xcb_connection_t,
        deviceid: xcb_input_device_id_t,
        mode: u8,
        format: u8,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        num_items: u32,
        items: *const xcb_input_xi_change_property_items_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_change_property_aux_checked)(
            c, deviceid, mode, format, property, type_, num_items, items,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_aux_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_aux_checked(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_aux_checked)
    }

    /// Sends a `Input::XIChangeProperty` request (unchecked) (aux).
    #[inline]
    pub unsafe fn xcb_input_xi_change_property_aux(
        &self,
        c: *mut xcb_connection_t,
        deviceid: xcb_input_device_id_t,
        mode: u8,
        format: u8,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        num_items: u32,
        items: *const xcb_input_xi_change_property_items_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_change_property_aux)(
            c, deviceid, mode, format, property, type_, num_items, items,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_aux` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_aux(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_aux)
    }

    /// Returns a pointer to the `items` field of a `xcb_input_xi_change_property_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_change_property_items(
        &self,
        r: *const xcb_input_xi_change_property_request_t,
    ) -> *mut c_void {
        sym!(self, xcb_input_xi_change_property_items)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_change_property_items` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_change_property_items(&self) -> bool {
        has_sym!(self, xcb_input_xi_change_property_items)
    }

    /// Sends a `Input::XIDeleteProperty` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_input_xi_delete_property_checked(
        &self,
        c: *mut xcb_connection_t,
        deviceid: xcb_input_device_id_t,
        property: xcb_atom_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_delete_property_checked)(c, deviceid, property)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_delete_property_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_delete_property_checked(&self) -> bool {
        has_sym!(self, xcb_input_xi_delete_property_checked)
    }

    /// Sends a `Input::XIDeleteProperty` request (unchecked).
    #[inline]
    pub unsafe fn xcb_input_xi_delete_property(
        &self,
        c: *mut xcb_connection_t,
        deviceid: xcb_input_device_id_t,
        property: xcb_atom_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_delete_property)(c, deviceid, property)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_delete_property` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_delete_property(&self) -> bool {
        has_sym!(self, xcb_input_xi_delete_property)
    }

    /// Returns a pointer to the `data8` field of a `xcb_input_xi_get_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_get_property_items_data_8(
        &self,
        s: *const xcb_input_xi_get_property_items_t,
    ) -> *mut u8 {
        sym!(self, xcb_input_xi_get_property_items_data_8)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_items_data_8` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_items_data_8(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_items_data_8)
    }

    /// Returns the number of elements of the `data8` field of a `xcb_input_xi_get_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_get_property_items_data_8_length(
        &self,
        r: *const xcb_input_xi_get_property_reply_t,
        s: *const xcb_input_xi_get_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_get_property_items_data_8_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_items_data_8_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_items_data_8_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_items_data_8_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `data8` field of a `xcb_input_xi_get_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_get_property_items_data_8_end(
        &self,
        r: *const xcb_input_xi_get_property_reply_t,
        s: *const xcb_input_xi_get_property_items_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_xi_get_property_items_data_8_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_items_data_8_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_items_data_8_end(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_items_data_8_end)
    }

    /// Returns a pointer to the `data16` field of a `xcb_input_xi_get_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_get_property_items_data_16(
        &self,
        s: *const xcb_input_xi_get_property_items_t,
    ) -> *mut u16 {
        sym!(self, xcb_input_xi_get_property_items_data_16)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_items_data_16` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_items_data_16(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_items_data_16)
    }

    /// Returns the number of elements of the `data16` field of a `xcb_input_xi_get_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_get_property_items_data_16_length(
        &self,
        r: *const xcb_input_xi_get_property_reply_t,
        s: *const xcb_input_xi_get_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_get_property_items_data_16_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_items_data_16_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_items_data_16_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_items_data_16_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `data16` field of a `xcb_input_xi_get_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_get_property_items_data_16_end(
        &self,
        r: *const xcb_input_xi_get_property_reply_t,
        s: *const xcb_input_xi_get_property_items_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_xi_get_property_items_data_16_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_items_data_16_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_items_data_16_end(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_items_data_16_end)
    }

    /// Returns a pointer to the `data32` field of a `xcb_input_xi_get_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_get_property_items_data_32(
        &self,
        s: *const xcb_input_xi_get_property_items_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_xi_get_property_items_data_32)(s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_items_data_32` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_items_data_32(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_items_data_32)
    }

    /// Returns the number of elements of the `data32` field of a `xcb_input_xi_get_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_get_property_items_data_32_length(
        &self,
        r: *const xcb_input_xi_get_property_reply_t,
        s: *const xcb_input_xi_get_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_get_property_items_data_32_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_items_data_32_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_items_data_32_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_items_data_32_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `data32` field of a `xcb_input_xi_get_property_items_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_get_property_items_data_32_end(
        &self,
        r: *const xcb_input_xi_get_property_reply_t,
        s: *const xcb_input_xi_get_property_items_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_xi_get_property_items_data_32_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_items_data_32_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_items_data_32_end(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_items_data_32_end)
    }

    /// Serializes a `xcb_input_xi_get_property_items_t` object.
    #[inline]
    pub unsafe fn xcb_input_xi_get_property_items_serialize(
        &self,
        _buffer: *mut *mut c_void,
        num_items: u32,
        format: u8,
        _aux: *const xcb_input_xi_get_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_get_property_items_serialize)(_buffer, num_items, format, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_items_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_items_serialize(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_items_serialize)
    }

    /// Unpacks a `xcb_input_xi_get_property_items_t` object.
    #[inline]
    pub unsafe fn xcb_input_xi_get_property_items_unpack(
        &self,
        _buffer: *const c_void,
        num_items: u32,
        format: u8,
        _aux: *mut xcb_input_xi_get_property_items_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_get_property_items_unpack)(_buffer, num_items, format, _aux)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_items_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_items_unpack(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_items_unpack)
    }

    /// Computes the size of a `xcb_input_xi_get_property_items_t` object.
    #[inline]
    pub unsafe fn xcb_input_xi_get_property_items_sizeof(
        &self,
        _buffer: *const c_void,
        num_items: u32,
        format: u8,
    ) -> c_int {
        sym!(self, xcb_input_xi_get_property_items_sizeof)(_buffer, num_items, format)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_items_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_items_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_items_sizeof)
    }

    /// Computes the size of a `xcb_input_xi_get_property_reply_t` object.
    #[inline]
    pub unsafe fn xcb_input_xi_get_property_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_xi_get_property_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_sizeof)
    }

    /// Sends a `Input::XIGetProperty` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_xi_get_property_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_xi_get_property_reply`]: Self::xcb_input_xi_get_property_reply
    #[inline]
    pub unsafe fn xcb_input_xi_get_property(
        &self,
        c: *mut xcb_connection_t,
        deviceid: xcb_input_device_id_t,
        delete: u8,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        offset: u32,
        len: u32,
    ) -> xcb_input_xi_get_property_cookie_t {
        sym!(self, xcb_input_xi_get_property)(c, deviceid, delete, property, type_, offset, len)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property)
    }

    /// Sends a `Input::XIGetProperty` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_xi_get_property_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_xi_get_property_reply`]: Self::xcb_input_xi_get_property_reply
    #[inline]
    pub unsafe fn xcb_input_xi_get_property_unchecked(
        &self,
        c: *mut xcb_connection_t,
        deviceid: xcb_input_device_id_t,
        delete: u8,
        property: xcb_atom_t,
        type_: xcb_atom_t,
        offset: u32,
        len: u32,
    ) -> xcb_input_xi_get_property_cookie_t {
        sym!(self, xcb_input_xi_get_property_unchecked)(
            c, deviceid, delete, property, type_, offset, len,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_unchecked)
    }

    /// Returns a pointer to the `items` field of a `xcb_input_xi_get_property_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_get_property_items(
        &self,
        r: *const xcb_input_xi_get_property_reply_t,
    ) -> *mut c_void {
        sym!(self, xcb_input_xi_get_property_items)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_items` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_items(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_items)
    }

    /// Waits for the reply to a `Input::XIGetProperty` request.
    #[inline]
    pub unsafe fn xcb_input_xi_get_property_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_xi_get_property_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_xi_get_property_reply_t {
        sym!(self, xcb_input_xi_get_property_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_property_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_property_reply(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_property_reply)
    }

    /// Computes the size of a `xcb_input_xi_get_selected_events_reply_t` object.
    #[inline]
    pub unsafe fn xcb_input_xi_get_selected_events_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_xi_get_selected_events_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_selected_events_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_selected_events_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_selected_events_sizeof)
    }

    /// Sends a `Input::XIGetSelectedEvents` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_xi_get_selected_events_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_xi_get_selected_events_reply`]: Self::xcb_input_xi_get_selected_events_reply
    #[inline]
    pub unsafe fn xcb_input_xi_get_selected_events(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_input_xi_get_selected_events_cookie_t {
        sym!(self, xcb_input_xi_get_selected_events)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_selected_events` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_selected_events(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_selected_events)
    }

    /// Sends a `Input::XIGetSelectedEvents` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_input_xi_get_selected_events_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_input_xi_get_selected_events_reply`]: Self::xcb_input_xi_get_selected_events_reply
    #[inline]
    pub unsafe fn xcb_input_xi_get_selected_events_unchecked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_input_xi_get_selected_events_cookie_t {
        sym!(self, xcb_input_xi_get_selected_events_unchecked)(c, window)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_selected_events_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_selected_events_unchecked(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_selected_events_unchecked)
    }

    /// Returns the number of elements of the `masks` field of a `xcb_input_xi_get_selected_events_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_get_selected_events_masks_length(
        &self,
        r: *const xcb_input_xi_get_selected_events_reply_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_get_selected_events_masks_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_selected_events_masks_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_selected_events_masks_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_selected_events_masks_length)
    }

    /// Returns an iterator over the elements of the
    /// `masks` field of a `xcb_input_xi_get_selected_events_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_get_selected_events_masks_iterator(
        &self,
        r: *const xcb_input_xi_get_selected_events_reply_t,
    ) -> xcb_input_event_mask_iterator_t {
        sym!(self, xcb_input_xi_get_selected_events_masks_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_selected_events_masks_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_selected_events_masks_iterator(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_selected_events_masks_iterator)
    }

    /// Waits for the reply to a `Input::XIGetSelectedEvents` request.
    #[inline]
    pub unsafe fn xcb_input_xi_get_selected_events_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_input_xi_get_selected_events_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_input_xi_get_selected_events_reply_t {
        sym!(self, xcb_input_xi_get_selected_events_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_get_selected_events_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_get_selected_events_reply(&self) -> bool {
        has_sym!(self, xcb_input_xi_get_selected_events_reply)
    }

    /// Advances a `xcb_input_barrier_release_pointer_info_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_barrier_release_pointer_info_next(
        &self,
        i: *mut xcb_input_barrier_release_pointer_info_iterator_t,
    ) {
        sym!(self, xcb_input_barrier_release_pointer_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_barrier_release_pointer_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_barrier_release_pointer_info_next(&self) -> bool {
        has_sym!(self, xcb_input_barrier_release_pointer_info_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_barrier_release_pointer_info_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_barrier_release_pointer_info_end(
        &self,
        i: xcb_input_barrier_release_pointer_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_barrier_release_pointer_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_barrier_release_pointer_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_barrier_release_pointer_info_end(&self) -> bool {
        has_sym!(self, xcb_input_barrier_release_pointer_info_end)
    }

    /// Computes the size of a `xcb_input_xi_barrier_release_pointer_request_t` object.
    #[inline]
    pub unsafe fn xcb_input_xi_barrier_release_pointer_sizeof(
        &self,
        _buffer: *const c_void,
    ) -> c_int {
        sym!(self, xcb_input_xi_barrier_release_pointer_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_barrier_release_pointer_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_barrier_release_pointer_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_xi_barrier_release_pointer_sizeof)
    }

    /// Sends a `Input::XIBarrierReleasePointer` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_input_xi_barrier_release_pointer_checked(
        &self,
        c: *mut xcb_connection_t,
        num_barriers: u32,
        barriers: *const xcb_input_barrier_release_pointer_info_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_barrier_release_pointer_checked)(c, num_barriers, barriers)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_barrier_release_pointer_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_barrier_release_pointer_checked(&self) -> bool {
        has_sym!(self, xcb_input_xi_barrier_release_pointer_checked)
    }

    /// Sends a `Input::XIBarrierReleasePointer` request (unchecked).
    #[inline]
    pub unsafe fn xcb_input_xi_barrier_release_pointer(
        &self,
        c: *mut xcb_connection_t,
        num_barriers: u32,
        barriers: *const xcb_input_barrier_release_pointer_info_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_xi_barrier_release_pointer)(c, num_barriers, barriers)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_barrier_release_pointer` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_barrier_release_pointer(&self) -> bool {
        has_sym!(self, xcb_input_xi_barrier_release_pointer)
    }

    /// Returns a pointer to the `barriers` field of a `xcb_input_xi_barrier_release_pointer_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_barrier_release_pointer_barriers(
        &self,
        r: *const xcb_input_xi_barrier_release_pointer_request_t,
    ) -> *mut xcb_input_barrier_release_pointer_info_t {
        sym!(self, xcb_input_xi_barrier_release_pointer_barriers)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_barrier_release_pointer_barriers` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_barrier_release_pointer_barriers(&self) -> bool {
        has_sym!(self, xcb_input_xi_barrier_release_pointer_barriers)
    }

    /// Returns the number of elements of the `barriers` field of a `xcb_input_xi_barrier_release_pointer_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_barrier_release_pointer_barriers_length(
        &self,
        r: *const xcb_input_xi_barrier_release_pointer_request_t,
    ) -> c_int {
        sym!(self, xcb_input_xi_barrier_release_pointer_barriers_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_barrier_release_pointer_barriers_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_barrier_release_pointer_barriers_length(&self) -> bool {
        has_sym!(self, xcb_input_xi_barrier_release_pointer_barriers_length)
    }

    /// Returns an iterator over the elements of the
    /// `barriers` field of a `xcb_input_xi_barrier_release_pointer_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_xi_barrier_release_pointer_barriers_iterator(
        &self,
        r: *const xcb_input_xi_barrier_release_pointer_request_t,
    ) -> xcb_input_barrier_release_pointer_info_iterator_t {
        sym!(self, xcb_input_xi_barrier_release_pointer_barriers_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_xi_barrier_release_pointer_barriers_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_xi_barrier_release_pointer_barriers_iterator(&self) -> bool {
        has_sym!(self, xcb_input_xi_barrier_release_pointer_barriers_iterator)
    }

    /// Computes the size of a `xcb_input_device_changed_event_t` object.
    #[inline]
    pub unsafe fn xcb_input_device_changed_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_device_changed_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_device_changed_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_changed_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_device_changed_sizeof)
    }

    /// Returns the number of elements of the `classes` field of a `xcb_input_device_changed_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_changed_classes_length(
        &self,
        r: *const xcb_input_device_changed_event_t,
    ) -> c_int {
        sym!(self, xcb_input_device_changed_classes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_changed_classes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_changed_classes_length(&self) -> bool {
        has_sym!(self, xcb_input_device_changed_classes_length)
    }

    /// Returns an iterator over the elements of the
    /// `classes` field of a `xcb_input_device_changed_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_device_changed_classes_iterator(
        &self,
        r: *const xcb_input_device_changed_event_t,
    ) -> xcb_input_device_class_iterator_t {
        sym!(self, xcb_input_device_changed_classes_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_device_changed_classes_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_device_changed_classes_iterator(&self) -> bool {
        has_sym!(self, xcb_input_device_changed_classes_iterator)
    }

    /// Computes the size of a `xcb_input_key_press_event_t` object.
    #[inline]
    pub unsafe fn xcb_input_key_press_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_key_press_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_key_press_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_press_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_key_press_sizeof)
    }

    /// Returns a pointer to the `button_mask` field of a `xcb_input_key_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_key_press_button_mask(
        &self,
        r: *const xcb_input_key_press_event_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_key_press_button_mask)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_key_press_button_mask` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_press_button_mask(&self) -> bool {
        has_sym!(self, xcb_input_key_press_button_mask)
    }

    /// Returns the number of elements of the `button_mask` field of a `xcb_input_key_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_key_press_button_mask_length(
        &self,
        r: *const xcb_input_key_press_event_t,
    ) -> c_int {
        sym!(self, xcb_input_key_press_button_mask_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_key_press_button_mask_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_press_button_mask_length(&self) -> bool {
        has_sym!(self, xcb_input_key_press_button_mask_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `button_mask` field of a `xcb_input_key_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_key_press_button_mask_end(
        &self,
        r: *const xcb_input_key_press_event_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_key_press_button_mask_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_key_press_button_mask_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_press_button_mask_end(&self) -> bool {
        has_sym!(self, xcb_input_key_press_button_mask_end)
    }

    /// Returns a pointer to the `valuator_mask` field of a `xcb_input_key_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_key_press_valuator_mask(
        &self,
        r: *const xcb_input_key_press_event_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_key_press_valuator_mask)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_key_press_valuator_mask` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_press_valuator_mask(&self) -> bool {
        has_sym!(self, xcb_input_key_press_valuator_mask)
    }

    /// Returns the number of elements of the `valuator_mask` field of a `xcb_input_key_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_key_press_valuator_mask_length(
        &self,
        r: *const xcb_input_key_press_event_t,
    ) -> c_int {
        sym!(self, xcb_input_key_press_valuator_mask_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_key_press_valuator_mask_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_press_valuator_mask_length(&self) -> bool {
        has_sym!(self, xcb_input_key_press_valuator_mask_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `valuator_mask` field of a `xcb_input_key_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_key_press_valuator_mask_end(
        &self,
        r: *const xcb_input_key_press_event_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_key_press_valuator_mask_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_key_press_valuator_mask_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_press_valuator_mask_end(&self) -> bool {
        has_sym!(self, xcb_input_key_press_valuator_mask_end)
    }

    /// Returns a pointer to the `axisvalues` field of a `xcb_input_key_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_key_press_axisvalues(
        &self,
        r: *const xcb_input_key_press_event_t,
    ) -> *mut xcb_input_fp3232_t {
        sym!(self, xcb_input_key_press_axisvalues)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_key_press_axisvalues` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_press_axisvalues(&self) -> bool {
        has_sym!(self, xcb_input_key_press_axisvalues)
    }

    /// Returns the number of elements of the `axisvalues` field of a `xcb_input_key_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_key_press_axisvalues_length(
        &self,
        r: *const xcb_input_key_press_event_t,
    ) -> c_int {
        sym!(self, xcb_input_key_press_axisvalues_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_key_press_axisvalues_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_press_axisvalues_length(&self) -> bool {
        has_sym!(self, xcb_input_key_press_axisvalues_length)
    }

    /// Returns an iterator over the elements of the
    /// `axisvalues` field of a `xcb_input_key_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_key_press_axisvalues_iterator(
        &self,
        r: *const xcb_input_key_press_event_t,
    ) -> xcb_input_fp3232_iterator_t {
        sym!(self, xcb_input_key_press_axisvalues_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_key_press_axisvalues_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_press_axisvalues_iterator(&self) -> bool {
        has_sym!(self, xcb_input_key_press_axisvalues_iterator)
    }

    /// Computes the size of a `xcb_input_key_release_event_t` object.
    ///
    /// Note: The libxcb function uses `const void*` as an argument because all pointers
    /// implicitly coerce to `const void*`. This is not the case in Rust so we have to use
    /// the correct pointer type to ensure backwards compatibility.
    #[inline]
    pub unsafe fn xcb_input_key_release_sizeof(
        &self,
        buffer: *const xcb_input_key_release_event_t,
    ) -> c_int {
        sym!(self, xcb_input_key_release_sizeof)(buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_key_release_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_key_release_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_key_release_sizeof)
    }

    /// Computes the size of a `xcb_input_button_press_event_t` object.
    #[inline]
    pub unsafe fn xcb_input_button_press_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_button_press_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_button_press_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_press_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_button_press_sizeof)
    }

    /// Returns a pointer to the `button_mask` field of a `xcb_input_button_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_button_press_button_mask(
        &self,
        r: *const xcb_input_button_press_event_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_button_press_button_mask)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_press_button_mask` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_press_button_mask(&self) -> bool {
        has_sym!(self, xcb_input_button_press_button_mask)
    }

    /// Returns the number of elements of the `button_mask` field of a `xcb_input_button_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_button_press_button_mask_length(
        &self,
        r: *const xcb_input_button_press_event_t,
    ) -> c_int {
        sym!(self, xcb_input_button_press_button_mask_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_press_button_mask_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_press_button_mask_length(&self) -> bool {
        has_sym!(self, xcb_input_button_press_button_mask_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `button_mask` field of a `xcb_input_button_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_button_press_button_mask_end(
        &self,
        r: *const xcb_input_button_press_event_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_button_press_button_mask_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_press_button_mask_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_press_button_mask_end(&self) -> bool {
        has_sym!(self, xcb_input_button_press_button_mask_end)
    }

    /// Returns a pointer to the `valuator_mask` field of a `xcb_input_button_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_button_press_valuator_mask(
        &self,
        r: *const xcb_input_button_press_event_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_button_press_valuator_mask)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_press_valuator_mask` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_press_valuator_mask(&self) -> bool {
        has_sym!(self, xcb_input_button_press_valuator_mask)
    }

    /// Returns the number of elements of the `valuator_mask` field of a `xcb_input_button_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_button_press_valuator_mask_length(
        &self,
        r: *const xcb_input_button_press_event_t,
    ) -> c_int {
        sym!(self, xcb_input_button_press_valuator_mask_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_press_valuator_mask_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_press_valuator_mask_length(&self) -> bool {
        has_sym!(self, xcb_input_button_press_valuator_mask_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `valuator_mask` field of a `xcb_input_button_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_button_press_valuator_mask_end(
        &self,
        r: *const xcb_input_button_press_event_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_button_press_valuator_mask_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_press_valuator_mask_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_press_valuator_mask_end(&self) -> bool {
        has_sym!(self, xcb_input_button_press_valuator_mask_end)
    }

    /// Returns a pointer to the `axisvalues` field of a `xcb_input_button_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_button_press_axisvalues(
        &self,
        r: *const xcb_input_button_press_event_t,
    ) -> *mut xcb_input_fp3232_t {
        sym!(self, xcb_input_button_press_axisvalues)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_press_axisvalues` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_press_axisvalues(&self) -> bool {
        has_sym!(self, xcb_input_button_press_axisvalues)
    }

    /// Returns the number of elements of the `axisvalues` field of a `xcb_input_button_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_button_press_axisvalues_length(
        &self,
        r: *const xcb_input_button_press_event_t,
    ) -> c_int {
        sym!(self, xcb_input_button_press_axisvalues_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_press_axisvalues_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_press_axisvalues_length(&self) -> bool {
        has_sym!(self, xcb_input_button_press_axisvalues_length)
    }

    /// Returns an iterator over the elements of the
    /// `axisvalues` field of a `xcb_input_button_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_button_press_axisvalues_iterator(
        &self,
        r: *const xcb_input_button_press_event_t,
    ) -> xcb_input_fp3232_iterator_t {
        sym!(self, xcb_input_button_press_axisvalues_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_button_press_axisvalues_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_press_axisvalues_iterator(&self) -> bool {
        has_sym!(self, xcb_input_button_press_axisvalues_iterator)
    }

    /// Computes the size of a `xcb_input_button_release_event_t` object.
    ///
    /// Note: The libxcb function uses `const void*` as an argument because all pointers
    /// implicitly coerce to `const void*`. This is not the case in Rust so we have to use
    /// the correct pointer type to ensure backwards compatibility.
    #[inline]
    pub unsafe fn xcb_input_button_release_sizeof(
        &self,
        buffer: *const xcb_input_button_release_event_t,
    ) -> c_int {
        sym!(self, xcb_input_button_release_sizeof)(buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_button_release_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_button_release_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_button_release_sizeof)
    }

    /// Computes the size of a `xcb_input_motion_event_t` object.
    ///
    /// Note: The libxcb function uses `const void*` as an argument because all pointers
    /// implicitly coerce to `const void*`. This is not the case in Rust so we have to use
    /// the correct pointer type to ensure backwards compatibility.
    #[inline]
    pub unsafe fn xcb_input_motion_sizeof(&self, buffer: *const xcb_input_motion_event_t) -> c_int {
        sym!(self, xcb_input_motion_sizeof)(buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_motion_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_motion_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_motion_sizeof)
    }

    /// Computes the size of a `xcb_input_enter_event_t` object.
    #[inline]
    pub unsafe fn xcb_input_enter_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_enter_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_enter_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_enter_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_enter_sizeof)
    }

    /// Returns a pointer to the `buttons` field of a `xcb_input_enter_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_enter_buttons(&self, r: *const xcb_input_enter_event_t) -> *mut u32 {
        sym!(self, xcb_input_enter_buttons)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_enter_buttons` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_enter_buttons(&self) -> bool {
        has_sym!(self, xcb_input_enter_buttons)
    }

    /// Returns the number of elements of the `buttons` field of a `xcb_input_enter_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_enter_buttons_length(
        &self,
        r: *const xcb_input_enter_event_t,
    ) -> c_int {
        sym!(self, xcb_input_enter_buttons_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_enter_buttons_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_enter_buttons_length(&self) -> bool {
        has_sym!(self, xcb_input_enter_buttons_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `buttons` field of a `xcb_input_enter_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_enter_buttons_end(
        &self,
        r: *const xcb_input_enter_event_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_enter_buttons_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_enter_buttons_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_enter_buttons_end(&self) -> bool {
        has_sym!(self, xcb_input_enter_buttons_end)
    }

    /// Computes the size of a `xcb_input_leave_event_t` object.
    ///
    /// Note: The libxcb function uses `const void*` as an argument because all pointers
    /// implicitly coerce to `const void*`. This is not the case in Rust so we have to use
    /// the correct pointer type to ensure backwards compatibility.
    #[inline]
    pub unsafe fn xcb_input_leave_sizeof(&self, buffer: *const xcb_input_leave_event_t) -> c_int {
        sym!(self, xcb_input_leave_sizeof)(buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_leave_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_leave_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_leave_sizeof)
    }

    /// Computes the size of a `xcb_input_focus_in_event_t` object.
    ///
    /// Note: The libxcb function uses `const void*` as an argument because all pointers
    /// implicitly coerce to `const void*`. This is not the case in Rust so we have to use
    /// the correct pointer type to ensure backwards compatibility.
    #[inline]
    pub unsafe fn xcb_input_focus_in_sizeof(
        &self,
        buffer: *const xcb_input_focus_in_event_t,
    ) -> c_int {
        sym!(self, xcb_input_focus_in_sizeof)(buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_focus_in_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_focus_in_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_focus_in_sizeof)
    }

    /// Computes the size of a `xcb_input_focus_out_event_t` object.
    ///
    /// Note: The libxcb function uses `const void*` as an argument because all pointers
    /// implicitly coerce to `const void*`. This is not the case in Rust so we have to use
    /// the correct pointer type to ensure backwards compatibility.
    #[inline]
    pub unsafe fn xcb_input_focus_out_sizeof(
        &self,
        buffer: *const xcb_input_focus_out_event_t,
    ) -> c_int {
        sym!(self, xcb_input_focus_out_sizeof)(buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_focus_out_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_focus_out_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_focus_out_sizeof)
    }

    /// Advances a `xcb_input_hierarchy_info_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_hierarchy_info_next(
        &self,
        i: *mut xcb_input_hierarchy_info_iterator_t,
    ) {
        sym!(self, xcb_input_hierarchy_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_info_next(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_info_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_hierarchy_info_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_hierarchy_info_end(
        &self,
        i: xcb_input_hierarchy_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_hierarchy_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_info_end(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_info_end)
    }

    /// Computes the size of a `xcb_input_hierarchy_event_t` object.
    #[inline]
    pub unsafe fn xcb_input_hierarchy_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_hierarchy_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_sizeof)
    }

    /// Returns a pointer to the `infos` field of a `xcb_input_hierarchy_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_hierarchy_infos(
        &self,
        r: *const xcb_input_hierarchy_event_t,
    ) -> *mut xcb_input_hierarchy_info_t {
        sym!(self, xcb_input_hierarchy_infos)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_infos` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_infos(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_infos)
    }

    /// Returns the number of elements of the `infos` field of a `xcb_input_hierarchy_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_hierarchy_infos_length(
        &self,
        r: *const xcb_input_hierarchy_event_t,
    ) -> c_int {
        sym!(self, xcb_input_hierarchy_infos_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_infos_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_infos_length(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_infos_length)
    }

    /// Returns an iterator over the elements of the
    /// `infos` field of a `xcb_input_hierarchy_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_hierarchy_infos_iterator(
        &self,
        r: *const xcb_input_hierarchy_event_t,
    ) -> xcb_input_hierarchy_info_iterator_t {
        sym!(self, xcb_input_hierarchy_infos_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_hierarchy_infos_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_hierarchy_infos_iterator(&self) -> bool {
        has_sym!(self, xcb_input_hierarchy_infos_iterator)
    }

    /// Computes the size of a `xcb_input_raw_key_press_event_t` object.
    #[inline]
    pub unsafe fn xcb_input_raw_key_press_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_raw_key_press_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_key_press_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_key_press_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_raw_key_press_sizeof)
    }

    /// Returns a pointer to the `valuator_mask` field of a `xcb_input_raw_key_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_raw_key_press_valuator_mask(
        &self,
        r: *const xcb_input_raw_key_press_event_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_raw_key_press_valuator_mask)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_key_press_valuator_mask` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_key_press_valuator_mask(&self) -> bool {
        has_sym!(self, xcb_input_raw_key_press_valuator_mask)
    }

    /// Returns the number of elements of the `valuator_mask` field of a `xcb_input_raw_key_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_raw_key_press_valuator_mask_length(
        &self,
        r: *const xcb_input_raw_key_press_event_t,
    ) -> c_int {
        sym!(self, xcb_input_raw_key_press_valuator_mask_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_key_press_valuator_mask_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_key_press_valuator_mask_length(&self) -> bool {
        has_sym!(self, xcb_input_raw_key_press_valuator_mask_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `valuator_mask` field of a `xcb_input_raw_key_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_raw_key_press_valuator_mask_end(
        &self,
        r: *const xcb_input_raw_key_press_event_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_raw_key_press_valuator_mask_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_key_press_valuator_mask_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_key_press_valuator_mask_end(&self) -> bool {
        has_sym!(self, xcb_input_raw_key_press_valuator_mask_end)
    }

    /// Returns a pointer to the `axisvalues` field of a `xcb_input_raw_key_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_raw_key_press_axisvalues(
        &self,
        r: *const xcb_input_raw_key_press_event_t,
    ) -> *mut xcb_input_fp3232_t {
        sym!(self, xcb_input_raw_key_press_axisvalues)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_key_press_axisvalues` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_key_press_axisvalues(&self) -> bool {
        has_sym!(self, xcb_input_raw_key_press_axisvalues)
    }

    /// Returns the number of elements of the `axisvalues` field of a `xcb_input_raw_key_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_raw_key_press_axisvalues_length(
        &self,
        r: *const xcb_input_raw_key_press_event_t,
    ) -> c_int {
        sym!(self, xcb_input_raw_key_press_axisvalues_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_key_press_axisvalues_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_key_press_axisvalues_length(&self) -> bool {
        has_sym!(self, xcb_input_raw_key_press_axisvalues_length)
    }

    /// Returns an iterator over the elements of the
    /// `axisvalues` field of a `xcb_input_raw_key_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_raw_key_press_axisvalues_iterator(
        &self,
        r: *const xcb_input_raw_key_press_event_t,
    ) -> xcb_input_fp3232_iterator_t {
        sym!(self, xcb_input_raw_key_press_axisvalues_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_key_press_axisvalues_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_key_press_axisvalues_iterator(&self) -> bool {
        has_sym!(self, xcb_input_raw_key_press_axisvalues_iterator)
    }

    /// Returns a pointer to the `axisvalues_raw` field of a `xcb_input_raw_key_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_raw_key_press_axisvalues_raw(
        &self,
        r: *const xcb_input_raw_key_press_event_t,
    ) -> *mut xcb_input_fp3232_t {
        sym!(self, xcb_input_raw_key_press_axisvalues_raw)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_key_press_axisvalues_raw` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_key_press_axisvalues_raw(&self) -> bool {
        has_sym!(self, xcb_input_raw_key_press_axisvalues_raw)
    }

    /// Returns the number of elements of the `axisvalues_raw` field of a `xcb_input_raw_key_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_raw_key_press_axisvalues_raw_length(
        &self,
        r: *const xcb_input_raw_key_press_event_t,
    ) -> c_int {
        sym!(self, xcb_input_raw_key_press_axisvalues_raw_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_key_press_axisvalues_raw_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_key_press_axisvalues_raw_length(&self) -> bool {
        has_sym!(self, xcb_input_raw_key_press_axisvalues_raw_length)
    }

    /// Returns an iterator over the elements of the
    /// `axisvalues_raw` field of a `xcb_input_raw_key_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_raw_key_press_axisvalues_raw_iterator(
        &self,
        r: *const xcb_input_raw_key_press_event_t,
    ) -> xcb_input_fp3232_iterator_t {
        sym!(self, xcb_input_raw_key_press_axisvalues_raw_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_key_press_axisvalues_raw_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_key_press_axisvalues_raw_iterator(&self) -> bool {
        has_sym!(self, xcb_input_raw_key_press_axisvalues_raw_iterator)
    }

    /// Computes the size of a `xcb_input_raw_key_release_event_t` object.
    ///
    /// Note: The libxcb function uses `const void*` as an argument because all pointers
    /// implicitly coerce to `const void*`. This is not the case in Rust so we have to use
    /// the correct pointer type to ensure backwards compatibility.
    #[inline]
    pub unsafe fn xcb_input_raw_key_release_sizeof(
        &self,
        buffer: *const xcb_input_raw_key_release_event_t,
    ) -> c_int {
        sym!(self, xcb_input_raw_key_release_sizeof)(buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_key_release_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_key_release_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_raw_key_release_sizeof)
    }

    /// Computes the size of a `xcb_input_raw_button_press_event_t` object.
    #[inline]
    pub unsafe fn xcb_input_raw_button_press_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_raw_button_press_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_button_press_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_button_press_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_raw_button_press_sizeof)
    }

    /// Returns a pointer to the `valuator_mask` field of a `xcb_input_raw_button_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_raw_button_press_valuator_mask(
        &self,
        r: *const xcb_input_raw_button_press_event_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_raw_button_press_valuator_mask)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_button_press_valuator_mask` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_button_press_valuator_mask(&self) -> bool {
        has_sym!(self, xcb_input_raw_button_press_valuator_mask)
    }

    /// Returns the number of elements of the `valuator_mask` field of a `xcb_input_raw_button_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_raw_button_press_valuator_mask_length(
        &self,
        r: *const xcb_input_raw_button_press_event_t,
    ) -> c_int {
        sym!(self, xcb_input_raw_button_press_valuator_mask_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_button_press_valuator_mask_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_button_press_valuator_mask_length(&self) -> bool {
        has_sym!(self, xcb_input_raw_button_press_valuator_mask_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `valuator_mask` field of a `xcb_input_raw_button_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_raw_button_press_valuator_mask_end(
        &self,
        r: *const xcb_input_raw_button_press_event_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_raw_button_press_valuator_mask_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_button_press_valuator_mask_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_button_press_valuator_mask_end(&self) -> bool {
        has_sym!(self, xcb_input_raw_button_press_valuator_mask_end)
    }

    /// Returns a pointer to the `axisvalues` field of a `xcb_input_raw_button_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_raw_button_press_axisvalues(
        &self,
        r: *const xcb_input_raw_button_press_event_t,
    ) -> *mut xcb_input_fp3232_t {
        sym!(self, xcb_input_raw_button_press_axisvalues)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_button_press_axisvalues` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_button_press_axisvalues(&self) -> bool {
        has_sym!(self, xcb_input_raw_button_press_axisvalues)
    }

    /// Returns the number of elements of the `axisvalues` field of a `xcb_input_raw_button_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_raw_button_press_axisvalues_length(
        &self,
        r: *const xcb_input_raw_button_press_event_t,
    ) -> c_int {
        sym!(self, xcb_input_raw_button_press_axisvalues_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_button_press_axisvalues_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_button_press_axisvalues_length(&self) -> bool {
        has_sym!(self, xcb_input_raw_button_press_axisvalues_length)
    }

    /// Returns an iterator over the elements of the
    /// `axisvalues` field of a `xcb_input_raw_button_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_raw_button_press_axisvalues_iterator(
        &self,
        r: *const xcb_input_raw_button_press_event_t,
    ) -> xcb_input_fp3232_iterator_t {
        sym!(self, xcb_input_raw_button_press_axisvalues_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_button_press_axisvalues_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_button_press_axisvalues_iterator(&self) -> bool {
        has_sym!(self, xcb_input_raw_button_press_axisvalues_iterator)
    }

    /// Returns a pointer to the `axisvalues_raw` field of a `xcb_input_raw_button_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_raw_button_press_axisvalues_raw(
        &self,
        r: *const xcb_input_raw_button_press_event_t,
    ) -> *mut xcb_input_fp3232_t {
        sym!(self, xcb_input_raw_button_press_axisvalues_raw)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_button_press_axisvalues_raw` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_button_press_axisvalues_raw(&self) -> bool {
        has_sym!(self, xcb_input_raw_button_press_axisvalues_raw)
    }

    /// Returns the number of elements of the `axisvalues_raw` field of a `xcb_input_raw_button_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_raw_button_press_axisvalues_raw_length(
        &self,
        r: *const xcb_input_raw_button_press_event_t,
    ) -> c_int {
        sym!(self, xcb_input_raw_button_press_axisvalues_raw_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_button_press_axisvalues_raw_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_button_press_axisvalues_raw_length(&self) -> bool {
        has_sym!(self, xcb_input_raw_button_press_axisvalues_raw_length)
    }

    /// Returns an iterator over the elements of the
    /// `axisvalues_raw` field of a `xcb_input_raw_button_press_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_raw_button_press_axisvalues_raw_iterator(
        &self,
        r: *const xcb_input_raw_button_press_event_t,
    ) -> xcb_input_fp3232_iterator_t {
        sym!(self, xcb_input_raw_button_press_axisvalues_raw_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_button_press_axisvalues_raw_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_button_press_axisvalues_raw_iterator(&self) -> bool {
        has_sym!(self, xcb_input_raw_button_press_axisvalues_raw_iterator)
    }

    /// Computes the size of a `xcb_input_raw_button_release_event_t` object.
    ///
    /// Note: The libxcb function uses `const void*` as an argument because all pointers
    /// implicitly coerce to `const void*`. This is not the case in Rust so we have to use
    /// the correct pointer type to ensure backwards compatibility.
    #[inline]
    pub unsafe fn xcb_input_raw_button_release_sizeof(
        &self,
        buffer: *const xcb_input_raw_button_release_event_t,
    ) -> c_int {
        sym!(self, xcb_input_raw_button_release_sizeof)(buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_button_release_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_button_release_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_raw_button_release_sizeof)
    }

    /// Computes the size of a `xcb_input_raw_motion_event_t` object.
    ///
    /// Note: The libxcb function uses `const void*` as an argument because all pointers
    /// implicitly coerce to `const void*`. This is not the case in Rust so we have to use
    /// the correct pointer type to ensure backwards compatibility.
    #[inline]
    pub unsafe fn xcb_input_raw_motion_sizeof(
        &self,
        buffer: *const xcb_input_raw_motion_event_t,
    ) -> c_int {
        sym!(self, xcb_input_raw_motion_sizeof)(buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_motion_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_motion_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_raw_motion_sizeof)
    }

    /// Computes the size of a `xcb_input_touch_begin_event_t` object.
    #[inline]
    pub unsafe fn xcb_input_touch_begin_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_touch_begin_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_touch_begin_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_touch_begin_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_touch_begin_sizeof)
    }

    /// Returns a pointer to the `button_mask` field of a `xcb_input_touch_begin_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_touch_begin_button_mask(
        &self,
        r: *const xcb_input_touch_begin_event_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_touch_begin_button_mask)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_touch_begin_button_mask` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_touch_begin_button_mask(&self) -> bool {
        has_sym!(self, xcb_input_touch_begin_button_mask)
    }

    /// Returns the number of elements of the `button_mask` field of a `xcb_input_touch_begin_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_touch_begin_button_mask_length(
        &self,
        r: *const xcb_input_touch_begin_event_t,
    ) -> c_int {
        sym!(self, xcb_input_touch_begin_button_mask_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_touch_begin_button_mask_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_touch_begin_button_mask_length(&self) -> bool {
        has_sym!(self, xcb_input_touch_begin_button_mask_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `button_mask` field of a `xcb_input_touch_begin_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_touch_begin_button_mask_end(
        &self,
        r: *const xcb_input_touch_begin_event_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_touch_begin_button_mask_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_touch_begin_button_mask_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_touch_begin_button_mask_end(&self) -> bool {
        has_sym!(self, xcb_input_touch_begin_button_mask_end)
    }

    /// Returns a pointer to the `valuator_mask` field of a `xcb_input_touch_begin_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_touch_begin_valuator_mask(
        &self,
        r: *const xcb_input_touch_begin_event_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_touch_begin_valuator_mask)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_touch_begin_valuator_mask` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_touch_begin_valuator_mask(&self) -> bool {
        has_sym!(self, xcb_input_touch_begin_valuator_mask)
    }

    /// Returns the number of elements of the `valuator_mask` field of a `xcb_input_touch_begin_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_touch_begin_valuator_mask_length(
        &self,
        r: *const xcb_input_touch_begin_event_t,
    ) -> c_int {
        sym!(self, xcb_input_touch_begin_valuator_mask_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_touch_begin_valuator_mask_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_touch_begin_valuator_mask_length(&self) -> bool {
        has_sym!(self, xcb_input_touch_begin_valuator_mask_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `valuator_mask` field of a `xcb_input_touch_begin_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_touch_begin_valuator_mask_end(
        &self,
        r: *const xcb_input_touch_begin_event_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_touch_begin_valuator_mask_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_touch_begin_valuator_mask_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_touch_begin_valuator_mask_end(&self) -> bool {
        has_sym!(self, xcb_input_touch_begin_valuator_mask_end)
    }

    /// Returns a pointer to the `axisvalues` field of a `xcb_input_touch_begin_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_touch_begin_axisvalues(
        &self,
        r: *const xcb_input_touch_begin_event_t,
    ) -> *mut xcb_input_fp3232_t {
        sym!(self, xcb_input_touch_begin_axisvalues)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_touch_begin_axisvalues` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_touch_begin_axisvalues(&self) -> bool {
        has_sym!(self, xcb_input_touch_begin_axisvalues)
    }

    /// Returns the number of elements of the `axisvalues` field of a `xcb_input_touch_begin_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_touch_begin_axisvalues_length(
        &self,
        r: *const xcb_input_touch_begin_event_t,
    ) -> c_int {
        sym!(self, xcb_input_touch_begin_axisvalues_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_touch_begin_axisvalues_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_touch_begin_axisvalues_length(&self) -> bool {
        has_sym!(self, xcb_input_touch_begin_axisvalues_length)
    }

    /// Returns an iterator over the elements of the
    /// `axisvalues` field of a `xcb_input_touch_begin_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_touch_begin_axisvalues_iterator(
        &self,
        r: *const xcb_input_touch_begin_event_t,
    ) -> xcb_input_fp3232_iterator_t {
        sym!(self, xcb_input_touch_begin_axisvalues_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_touch_begin_axisvalues_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_touch_begin_axisvalues_iterator(&self) -> bool {
        has_sym!(self, xcb_input_touch_begin_axisvalues_iterator)
    }

    /// Computes the size of a `xcb_input_touch_update_event_t` object.
    ///
    /// Note: The libxcb function uses `const void*` as an argument because all pointers
    /// implicitly coerce to `const void*`. This is not the case in Rust so we have to use
    /// the correct pointer type to ensure backwards compatibility.
    #[inline]
    pub unsafe fn xcb_input_touch_update_sizeof(
        &self,
        buffer: *const xcb_input_touch_update_event_t,
    ) -> c_int {
        sym!(self, xcb_input_touch_update_sizeof)(buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_touch_update_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_touch_update_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_touch_update_sizeof)
    }

    /// Computes the size of a `xcb_input_touch_end_event_t` object.
    ///
    /// Note: The libxcb function uses `const void*` as an argument because all pointers
    /// implicitly coerce to `const void*`. This is not the case in Rust so we have to use
    /// the correct pointer type to ensure backwards compatibility.
    #[inline]
    pub unsafe fn xcb_input_touch_end_sizeof(
        &self,
        buffer: *const xcb_input_touch_end_event_t,
    ) -> c_int {
        sym!(self, xcb_input_touch_end_sizeof)(buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_touch_end_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_touch_end_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_touch_end_sizeof)
    }

    /// Computes the size of a `xcb_input_raw_touch_begin_event_t` object.
    #[inline]
    pub unsafe fn xcb_input_raw_touch_begin_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_raw_touch_begin_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_touch_begin_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_touch_begin_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_raw_touch_begin_sizeof)
    }

    /// Returns a pointer to the `valuator_mask` field of a `xcb_input_raw_touch_begin_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_raw_touch_begin_valuator_mask(
        &self,
        r: *const xcb_input_raw_touch_begin_event_t,
    ) -> *mut u32 {
        sym!(self, xcb_input_raw_touch_begin_valuator_mask)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_touch_begin_valuator_mask` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_touch_begin_valuator_mask(&self) -> bool {
        has_sym!(self, xcb_input_raw_touch_begin_valuator_mask)
    }

    /// Returns the number of elements of the `valuator_mask` field of a `xcb_input_raw_touch_begin_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_raw_touch_begin_valuator_mask_length(
        &self,
        r: *const xcb_input_raw_touch_begin_event_t,
    ) -> c_int {
        sym!(self, xcb_input_raw_touch_begin_valuator_mask_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_touch_begin_valuator_mask_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_touch_begin_valuator_mask_length(&self) -> bool {
        has_sym!(self, xcb_input_raw_touch_begin_valuator_mask_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `valuator_mask` field of a `xcb_input_raw_touch_begin_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_raw_touch_begin_valuator_mask_end(
        &self,
        r: *const xcb_input_raw_touch_begin_event_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_raw_touch_begin_valuator_mask_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_touch_begin_valuator_mask_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_touch_begin_valuator_mask_end(&self) -> bool {
        has_sym!(self, xcb_input_raw_touch_begin_valuator_mask_end)
    }

    /// Returns a pointer to the `axisvalues` field of a `xcb_input_raw_touch_begin_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_raw_touch_begin_axisvalues(
        &self,
        r: *const xcb_input_raw_touch_begin_event_t,
    ) -> *mut xcb_input_fp3232_t {
        sym!(self, xcb_input_raw_touch_begin_axisvalues)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_touch_begin_axisvalues` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_touch_begin_axisvalues(&self) -> bool {
        has_sym!(self, xcb_input_raw_touch_begin_axisvalues)
    }

    /// Returns the number of elements of the `axisvalues` field of a `xcb_input_raw_touch_begin_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_raw_touch_begin_axisvalues_length(
        &self,
        r: *const xcb_input_raw_touch_begin_event_t,
    ) -> c_int {
        sym!(self, xcb_input_raw_touch_begin_axisvalues_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_touch_begin_axisvalues_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_touch_begin_axisvalues_length(&self) -> bool {
        has_sym!(self, xcb_input_raw_touch_begin_axisvalues_length)
    }

    /// Returns an iterator over the elements of the
    /// `axisvalues` field of a `xcb_input_raw_touch_begin_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_raw_touch_begin_axisvalues_iterator(
        &self,
        r: *const xcb_input_raw_touch_begin_event_t,
    ) -> xcb_input_fp3232_iterator_t {
        sym!(self, xcb_input_raw_touch_begin_axisvalues_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_touch_begin_axisvalues_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_touch_begin_axisvalues_iterator(&self) -> bool {
        has_sym!(self, xcb_input_raw_touch_begin_axisvalues_iterator)
    }

    /// Returns a pointer to the `axisvalues_raw` field of a `xcb_input_raw_touch_begin_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_raw_touch_begin_axisvalues_raw(
        &self,
        r: *const xcb_input_raw_touch_begin_event_t,
    ) -> *mut xcb_input_fp3232_t {
        sym!(self, xcb_input_raw_touch_begin_axisvalues_raw)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_touch_begin_axisvalues_raw` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_touch_begin_axisvalues_raw(&self) -> bool {
        has_sym!(self, xcb_input_raw_touch_begin_axisvalues_raw)
    }

    /// Returns the number of elements of the `axisvalues_raw` field of a `xcb_input_raw_touch_begin_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_raw_touch_begin_axisvalues_raw_length(
        &self,
        r: *const xcb_input_raw_touch_begin_event_t,
    ) -> c_int {
        sym!(self, xcb_input_raw_touch_begin_axisvalues_raw_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_touch_begin_axisvalues_raw_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_touch_begin_axisvalues_raw_length(&self) -> bool {
        has_sym!(self, xcb_input_raw_touch_begin_axisvalues_raw_length)
    }

    /// Returns an iterator over the elements of the
    /// `axisvalues_raw` field of a `xcb_input_raw_touch_begin_event_t` struct.
    #[inline]
    pub unsafe fn xcb_input_raw_touch_begin_axisvalues_raw_iterator(
        &self,
        r: *const xcb_input_raw_touch_begin_event_t,
    ) -> xcb_input_fp3232_iterator_t {
        sym!(self, xcb_input_raw_touch_begin_axisvalues_raw_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_touch_begin_axisvalues_raw_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_touch_begin_axisvalues_raw_iterator(&self) -> bool {
        has_sym!(self, xcb_input_raw_touch_begin_axisvalues_raw_iterator)
    }

    /// Computes the size of a `xcb_input_raw_touch_update_event_t` object.
    ///
    /// Note: The libxcb function uses `const void*` as an argument because all pointers
    /// implicitly coerce to `const void*`. This is not the case in Rust so we have to use
    /// the correct pointer type to ensure backwards compatibility.
    #[inline]
    pub unsafe fn xcb_input_raw_touch_update_sizeof(
        &self,
        buffer: *const xcb_input_raw_touch_update_event_t,
    ) -> c_int {
        sym!(self, xcb_input_raw_touch_update_sizeof)(buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_touch_update_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_touch_update_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_raw_touch_update_sizeof)
    }

    /// Computes the size of a `xcb_input_raw_touch_end_event_t` object.
    ///
    /// Note: The libxcb function uses `const void*` as an argument because all pointers
    /// implicitly coerce to `const void*`. This is not the case in Rust so we have to use
    /// the correct pointer type to ensure backwards compatibility.
    #[inline]
    pub unsafe fn xcb_input_raw_touch_end_sizeof(
        &self,
        buffer: *const xcb_input_raw_touch_end_event_t,
    ) -> c_int {
        sym!(self, xcb_input_raw_touch_end_sizeof)(buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_raw_touch_end_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_raw_touch_end_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_raw_touch_end_sizeof)
    }

    /// Advances a `xcb_input_event_for_send_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_input_event_for_send_next(
        &self,
        i: *mut xcb_input_event_for_send_iterator_t,
    ) {
        sym!(self, xcb_input_event_for_send_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_event_for_send_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_event_for_send_next(&self) -> bool {
        has_sym!(self, xcb_input_event_for_send_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_input_event_for_send_iterator_t`.
    #[inline]
    pub unsafe fn xcb_input_event_for_send_end(
        &self,
        i: xcb_input_event_for_send_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_event_for_send_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_input_event_for_send_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_event_for_send_end(&self) -> bool {
        has_sym!(self, xcb_input_event_for_send_end)
    }

    /// Computes the size of a `xcb_input_send_extension_event_request_t` object.
    #[inline]
    pub unsafe fn xcb_input_send_extension_event_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_input_send_extension_event_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_input_send_extension_event_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_send_extension_event_sizeof(&self) -> bool {
        has_sym!(self, xcb_input_send_extension_event_sizeof)
    }

    /// Sends a `Input::SendExtensionEvent` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_input_send_extension_event_checked(
        &self,
        c: *mut xcb_connection_t,
        destination: xcb_window_t,
        device_id: u8,
        propagate: u8,
        num_classes: u16,
        num_events: u8,
        events: *const xcb_input_event_for_send_t,
        classes: *const xcb_input_event_class_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_send_extension_event_checked)(
            c,
            destination,
            device_id,
            propagate,
            num_classes,
            num_events,
            events,
            classes,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_send_extension_event_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_send_extension_event_checked(&self) -> bool {
        has_sym!(self, xcb_input_send_extension_event_checked)
    }

    /// Sends a `Input::SendExtensionEvent` request (unchecked).
    #[inline]
    pub unsafe fn xcb_input_send_extension_event(
        &self,
        c: *mut xcb_connection_t,
        destination: xcb_window_t,
        device_id: u8,
        propagate: u8,
        num_classes: u16,
        num_events: u8,
        events: *const xcb_input_event_for_send_t,
        classes: *const xcb_input_event_class_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_input_send_extension_event)(
            c,
            destination,
            device_id,
            propagate,
            num_classes,
            num_events,
            events,
            classes,
        )
    }

    /// Returns `true` iff the symbol `xcb_input_send_extension_event` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_send_extension_event(&self) -> bool {
        has_sym!(self, xcb_input_send_extension_event)
    }

    /// Returns a pointer to the `events` field of a `xcb_input_send_extension_event_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_send_extension_event_events(
        &self,
        r: *const xcb_input_send_extension_event_request_t,
    ) -> *mut xcb_input_event_for_send_t {
        sym!(self, xcb_input_send_extension_event_events)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_send_extension_event_events` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_send_extension_event_events(&self) -> bool {
        has_sym!(self, xcb_input_send_extension_event_events)
    }

    /// Returns the number of elements of the `events` field of a `xcb_input_send_extension_event_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_send_extension_event_events_length(
        &self,
        r: *const xcb_input_send_extension_event_request_t,
    ) -> c_int {
        sym!(self, xcb_input_send_extension_event_events_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_send_extension_event_events_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_send_extension_event_events_length(&self) -> bool {
        has_sym!(self, xcb_input_send_extension_event_events_length)
    }

    /// Returns an iterator over the elements of the
    /// `events` field of a `xcb_input_send_extension_event_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_send_extension_event_events_iterator(
        &self,
        r: *const xcb_input_send_extension_event_request_t,
    ) -> xcb_input_event_for_send_iterator_t {
        sym!(self, xcb_input_send_extension_event_events_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_send_extension_event_events_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_send_extension_event_events_iterator(&self) -> bool {
        has_sym!(self, xcb_input_send_extension_event_events_iterator)
    }

    /// Returns a pointer to the `classes` field of a `xcb_input_send_extension_event_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_send_extension_event_classes(
        &self,
        r: *const xcb_input_send_extension_event_request_t,
    ) -> *mut xcb_input_event_class_t {
        sym!(self, xcb_input_send_extension_event_classes)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_send_extension_event_classes` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_send_extension_event_classes(&self) -> bool {
        has_sym!(self, xcb_input_send_extension_event_classes)
    }

    /// Returns the number of elements of the `classes` field of a `xcb_input_send_extension_event_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_send_extension_event_classes_length(
        &self,
        r: *const xcb_input_send_extension_event_request_t,
    ) -> c_int {
        sym!(self, xcb_input_send_extension_event_classes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_send_extension_event_classes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_send_extension_event_classes_length(&self) -> bool {
        has_sym!(self, xcb_input_send_extension_event_classes_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `classes` field of a `xcb_input_send_extension_event_request_t` struct.
    #[inline]
    pub unsafe fn xcb_input_send_extension_event_classes_end(
        &self,
        r: *const xcb_input_send_extension_event_request_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_input_send_extension_event_classes_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_input_send_extension_event_classes_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_input_send_extension_event_classes_end(&self) -> bool {
        has_sym!(self, xcb_input_send_extension_event_classes_end)
    }
}

#[cfg(feature = "xcb_xinput")]
#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbXinput::load().unwrap() };
        assert!(lib.has_xcb_input_id());
        assert!(lib.has_xcb_input_event_class_next());
        assert!(lib.has_xcb_input_event_class_end());
        assert!(lib.has_xcb_input_key_code_next());
        assert!(lib.has_xcb_input_key_code_end());
        assert!(lib.has_xcb_input_device_id_next());
        assert!(lib.has_xcb_input_device_id_end());
        assert!(lib.has_xcb_input_fp1616_next());
        assert!(lib.has_xcb_input_fp1616_end());
        assert!(lib.has_xcb_input_fp3232_next());
        assert!(lib.has_xcb_input_fp3232_end());
        assert!(lib.has_xcb_input_get_extension_version_sizeof());
        assert!(lib.has_xcb_input_get_extension_version());
        assert!(lib.has_xcb_input_get_extension_version_unchecked());
        assert!(lib.has_xcb_input_get_extension_version_reply());
        assert!(lib.has_xcb_input_device_info_next());
        assert!(lib.has_xcb_input_device_info_end());
        assert!(lib.has_xcb_input_key_info_next());
        assert!(lib.has_xcb_input_key_info_end());
        assert!(lib.has_xcb_input_button_info_next());
        assert!(lib.has_xcb_input_button_info_end());
        assert!(lib.has_xcb_input_axis_info_next());
        assert!(lib.has_xcb_input_axis_info_end());
        assert!(lib.has_xcb_input_valuator_info_sizeof());
        assert!(lib.has_xcb_input_valuator_info_axes());
        assert!(lib.has_xcb_input_valuator_info_axes_length());
        assert!(lib.has_xcb_input_valuator_info_axes_iterator());
        assert!(lib.has_xcb_input_valuator_info_next());
        assert!(lib.has_xcb_input_valuator_info_end());
        assert!(lib.has_xcb_input_input_info_info_valuator_axes());
        assert!(lib.has_xcb_input_input_info_info_valuator_axes_length());
        assert!(lib.has_xcb_input_input_info_info_valuator_axes_iterator());
        assert!(lib.has_xcb_input_input_info_info_serialize());
        assert!(lib.has_xcb_input_input_info_info_unpack());
        assert!(lib.has_xcb_input_input_info_info_sizeof());
        assert!(lib.has_xcb_input_input_info_sizeof());
        assert!(lib.has_xcb_input_input_info_info());
        assert!(lib.has_xcb_input_input_info_next());
        assert!(lib.has_xcb_input_input_info_end());
        assert!(lib.has_xcb_input_device_name_sizeof());
        assert!(lib.has_xcb_input_device_name_string());
        assert!(lib.has_xcb_input_device_name_string_length());
        assert!(lib.has_xcb_input_device_name_string_end());
        assert!(lib.has_xcb_input_device_name_next());
        assert!(lib.has_xcb_input_device_name_end());
        assert!(lib.has_xcb_input_list_input_devices_sizeof());
        assert!(lib.has_xcb_input_list_input_devices());
        assert!(lib.has_xcb_input_list_input_devices_unchecked());
        assert!(lib.has_xcb_input_list_input_devices_devices());
        assert!(lib.has_xcb_input_list_input_devices_devices_length());
        assert!(lib.has_xcb_input_list_input_devices_devices_iterator());
        assert!(lib.has_xcb_input_list_input_devices_infos_length());
        assert!(lib.has_xcb_input_list_input_devices_infos_iterator());
        assert!(lib.has_xcb_input_list_input_devices_names_length());
        assert!(lib.has_xcb_input_list_input_devices_names_iterator());
        assert!(lib.has_xcb_input_list_input_devices_reply());
        assert!(lib.has_xcb_input_event_type_base_next());
        assert!(lib.has_xcb_input_event_type_base_end());
        assert!(lib.has_xcb_input_input_class_info_next());
        assert!(lib.has_xcb_input_input_class_info_end());
        assert!(lib.has_xcb_input_open_device_sizeof());
        assert!(lib.has_xcb_input_open_device());
        assert!(lib.has_xcb_input_open_device_unchecked());
        assert!(lib.has_xcb_input_open_device_class_info());
        assert!(lib.has_xcb_input_open_device_class_info_length());
        assert!(lib.has_xcb_input_open_device_class_info_iterator());
        assert!(lib.has_xcb_input_open_device_reply());
        assert!(lib.has_xcb_input_close_device_checked());
        assert!(lib.has_xcb_input_close_device());
        assert!(lib.has_xcb_input_set_device_mode());
        assert!(lib.has_xcb_input_set_device_mode_unchecked());
        assert!(lib.has_xcb_input_set_device_mode_reply());
        assert!(lib.has_xcb_input_select_extension_event_sizeof());
        assert!(lib.has_xcb_input_select_extension_event_checked());
        assert!(lib.has_xcb_input_select_extension_event());
        assert!(lib.has_xcb_input_select_extension_event_classes());
        assert!(lib.has_xcb_input_select_extension_event_classes_length());
        assert!(lib.has_xcb_input_select_extension_event_classes_end());
        assert!(lib.has_xcb_input_get_selected_extension_events_sizeof());
        assert!(lib.has_xcb_input_get_selected_extension_events());
        assert!(lib.has_xcb_input_get_selected_extension_events_unchecked());
        assert!(lib.has_xcb_input_get_selected_extension_events_this_classes());
        assert!(lib.has_xcb_input_get_selected_extension_events_this_classes_length());
        assert!(lib.has_xcb_input_get_selected_extension_events_this_classes_end());
        assert!(lib.has_xcb_input_get_selected_extension_events_all_classes());
        assert!(lib.has_xcb_input_get_selected_extension_events_all_classes_length());
        assert!(lib.has_xcb_input_get_selected_extension_events_all_classes_end());
        assert!(lib.has_xcb_input_get_selected_extension_events_reply());
        assert!(lib.has_xcb_input_change_device_dont_propagate_list_sizeof());
        assert!(lib.has_xcb_input_change_device_dont_propagate_list_checked());
        assert!(lib.has_xcb_input_change_device_dont_propagate_list());
        assert!(lib.has_xcb_input_change_device_dont_propagate_list_classes());
        assert!(lib.has_xcb_input_change_device_dont_propagate_list_classes_length());
        assert!(lib.has_xcb_input_change_device_dont_propagate_list_classes_end());
        assert!(lib.has_xcb_input_get_device_dont_propagate_list_sizeof());
        assert!(lib.has_xcb_input_get_device_dont_propagate_list());
        assert!(lib.has_xcb_input_get_device_dont_propagate_list_unchecked());
        assert!(lib.has_xcb_input_get_device_dont_propagate_list_classes());
        assert!(lib.has_xcb_input_get_device_dont_propagate_list_classes_length());
        assert!(lib.has_xcb_input_get_device_dont_propagate_list_classes_end());
        assert!(lib.has_xcb_input_get_device_dont_propagate_list_reply());
        assert!(lib.has_xcb_input_device_time_coord_sizeof());
        assert!(lib.has_xcb_input_device_time_coord_axisvalues());
        assert!(lib.has_xcb_input_device_time_coord_axisvalues_length());
        assert!(lib.has_xcb_input_device_time_coord_axisvalues_end());
        assert!(lib.has_xcb_input_device_time_coord_next());
        assert!(lib.has_xcb_input_device_time_coord_end());
        assert!(lib.has_xcb_input_get_device_motion_events_sizeof());
        assert!(lib.has_xcb_input_get_device_motion_events());
        assert!(lib.has_xcb_input_get_device_motion_events_unchecked());
        assert!(lib.has_xcb_input_get_device_motion_events_events_length());
        assert!(lib.has_xcb_input_get_device_motion_events_events_iterator());
        assert!(lib.has_xcb_input_get_device_motion_events_reply());
        assert!(lib.has_xcb_input_change_keyboard_device());
        assert!(lib.has_xcb_input_change_keyboard_device_unchecked());
        assert!(lib.has_xcb_input_change_keyboard_device_reply());
        assert!(lib.has_xcb_input_change_pointer_device());
        assert!(lib.has_xcb_input_change_pointer_device_unchecked());
        assert!(lib.has_xcb_input_change_pointer_device_reply());
        assert!(lib.has_xcb_input_grab_device_sizeof());
        assert!(lib.has_xcb_input_grab_device());
        assert!(lib.has_xcb_input_grab_device_unchecked());
        assert!(lib.has_xcb_input_grab_device_reply());
        assert!(lib.has_xcb_input_ungrab_device_checked());
        assert!(lib.has_xcb_input_ungrab_device());
        assert!(lib.has_xcb_input_grab_device_key_sizeof());
        assert!(lib.has_xcb_input_grab_device_key_checked());
        assert!(lib.has_xcb_input_grab_device_key());
        assert!(lib.has_xcb_input_grab_device_key_classes());
        assert!(lib.has_xcb_input_grab_device_key_classes_length());
        assert!(lib.has_xcb_input_grab_device_key_classes_end());
        assert!(lib.has_xcb_input_ungrab_device_key_checked());
        assert!(lib.has_xcb_input_ungrab_device_key());
        assert!(lib.has_xcb_input_grab_device_button_sizeof());
        assert!(lib.has_xcb_input_grab_device_button_checked());
        assert!(lib.has_xcb_input_grab_device_button());
        assert!(lib.has_xcb_input_grab_device_button_classes());
        assert!(lib.has_xcb_input_grab_device_button_classes_length());
        assert!(lib.has_xcb_input_grab_device_button_classes_end());
        assert!(lib.has_xcb_input_ungrab_device_button_checked());
        assert!(lib.has_xcb_input_ungrab_device_button());
        assert!(lib.has_xcb_input_allow_device_events_checked());
        assert!(lib.has_xcb_input_allow_device_events());
        assert!(lib.has_xcb_input_get_device_focus());
        assert!(lib.has_xcb_input_get_device_focus_unchecked());
        assert!(lib.has_xcb_input_get_device_focus_reply());
        assert!(lib.has_xcb_input_set_device_focus_checked());
        assert!(lib.has_xcb_input_set_device_focus());
        assert!(lib.has_xcb_input_kbd_feedback_state_next());
        assert!(lib.has_xcb_input_kbd_feedback_state_end());
        assert!(lib.has_xcb_input_ptr_feedback_state_next());
        assert!(lib.has_xcb_input_ptr_feedback_state_end());
        assert!(lib.has_xcb_input_integer_feedback_state_next());
        assert!(lib.has_xcb_input_integer_feedback_state_end());
        assert!(lib.has_xcb_input_string_feedback_state_sizeof());
        assert!(lib.has_xcb_input_string_feedback_state_keysyms());
        assert!(lib.has_xcb_input_string_feedback_state_keysyms_length());
        assert!(lib.has_xcb_input_string_feedback_state_keysyms_end());
        assert!(lib.has_xcb_input_string_feedback_state_next());
        assert!(lib.has_xcb_input_string_feedback_state_end());
        assert!(lib.has_xcb_input_bell_feedback_state_next());
        assert!(lib.has_xcb_input_bell_feedback_state_end());
        assert!(lib.has_xcb_input_led_feedback_state_next());
        assert!(lib.has_xcb_input_led_feedback_state_end());
        assert!(lib.has_xcb_input_feedback_state_data_string_keysyms());
        assert!(lib.has_xcb_input_feedback_state_data_string_keysyms_length());
        assert!(lib.has_xcb_input_feedback_state_data_string_keysyms_end());
        assert!(lib.has_xcb_input_feedback_state_data_serialize());
        assert!(lib.has_xcb_input_feedback_state_data_unpack());
        assert!(lib.has_xcb_input_feedback_state_data_sizeof());
        assert!(lib.has_xcb_input_feedback_state_sizeof());
        assert!(lib.has_xcb_input_feedback_state_data());
        assert!(lib.has_xcb_input_feedback_state_next());
        assert!(lib.has_xcb_input_feedback_state_end());
        assert!(lib.has_xcb_input_get_feedback_control_sizeof());
        assert!(lib.has_xcb_input_get_feedback_control());
        assert!(lib.has_xcb_input_get_feedback_control_unchecked());
        assert!(lib.has_xcb_input_get_feedback_control_feedbacks_length());
        assert!(lib.has_xcb_input_get_feedback_control_feedbacks_iterator());
        assert!(lib.has_xcb_input_get_feedback_control_reply());
        assert!(lib.has_xcb_input_kbd_feedback_ctl_next());
        assert!(lib.has_xcb_input_kbd_feedback_ctl_end());
        assert!(lib.has_xcb_input_ptr_feedback_ctl_next());
        assert!(lib.has_xcb_input_ptr_feedback_ctl_end());
        assert!(lib.has_xcb_input_integer_feedback_ctl_next());
        assert!(lib.has_xcb_input_integer_feedback_ctl_end());
        assert!(lib.has_xcb_input_string_feedback_ctl_sizeof());
        assert!(lib.has_xcb_input_string_feedback_ctl_keysyms());
        assert!(lib.has_xcb_input_string_feedback_ctl_keysyms_length());
        assert!(lib.has_xcb_input_string_feedback_ctl_keysyms_end());
        assert!(lib.has_xcb_input_string_feedback_ctl_next());
        assert!(lib.has_xcb_input_string_feedback_ctl_end());
        assert!(lib.has_xcb_input_bell_feedback_ctl_next());
        assert!(lib.has_xcb_input_bell_feedback_ctl_end());
        assert!(lib.has_xcb_input_led_feedback_ctl_next());
        assert!(lib.has_xcb_input_led_feedback_ctl_end());
        assert!(lib.has_xcb_input_feedback_ctl_data_string_keysyms());
        assert!(lib.has_xcb_input_feedback_ctl_data_string_keysyms_length());
        assert!(lib.has_xcb_input_feedback_ctl_data_string_keysyms_end());
        assert!(lib.has_xcb_input_feedback_ctl_data_serialize());
        assert!(lib.has_xcb_input_feedback_ctl_data_unpack());
        assert!(lib.has_xcb_input_feedback_ctl_data_sizeof());
        assert!(lib.has_xcb_input_feedback_ctl_sizeof());
        assert!(lib.has_xcb_input_feedback_ctl_data());
        assert!(lib.has_xcb_input_feedback_ctl_next());
        assert!(lib.has_xcb_input_feedback_ctl_end());
        assert!(lib.has_xcb_input_change_feedback_control_sizeof());
        assert!(lib.has_xcb_input_change_feedback_control_checked());
        assert!(lib.has_xcb_input_change_feedback_control());
        assert!(lib.has_xcb_input_change_feedback_control_feedback());
        assert!(lib.has_xcb_input_get_device_key_mapping_sizeof());
        assert!(lib.has_xcb_input_get_device_key_mapping());
        assert!(lib.has_xcb_input_get_device_key_mapping_unchecked());
        assert!(lib.has_xcb_input_get_device_key_mapping_keysyms());
        assert!(lib.has_xcb_input_get_device_key_mapping_keysyms_length());
        assert!(lib.has_xcb_input_get_device_key_mapping_keysyms_end());
        assert!(lib.has_xcb_input_get_device_key_mapping_reply());
        assert!(lib.has_xcb_input_change_device_key_mapping_sizeof());
        assert!(lib.has_xcb_input_change_device_key_mapping_checked());
        assert!(lib.has_xcb_input_change_device_key_mapping());
        assert!(lib.has_xcb_input_change_device_key_mapping_keysyms());
        assert!(lib.has_xcb_input_change_device_key_mapping_keysyms_length());
        assert!(lib.has_xcb_input_change_device_key_mapping_keysyms_end());
        assert!(lib.has_xcb_input_get_device_modifier_mapping_sizeof());
        assert!(lib.has_xcb_input_get_device_modifier_mapping());
        assert!(lib.has_xcb_input_get_device_modifier_mapping_unchecked());
        assert!(lib.has_xcb_input_get_device_modifier_mapping_keymaps());
        assert!(lib.has_xcb_input_get_device_modifier_mapping_keymaps_length());
        assert!(lib.has_xcb_input_get_device_modifier_mapping_keymaps_end());
        assert!(lib.has_xcb_input_get_device_modifier_mapping_reply());
        assert!(lib.has_xcb_input_set_device_modifier_mapping_sizeof());
        assert!(lib.has_xcb_input_set_device_modifier_mapping());
        assert!(lib.has_xcb_input_set_device_modifier_mapping_unchecked());
        assert!(lib.has_xcb_input_set_device_modifier_mapping_reply());
        assert!(lib.has_xcb_input_get_device_button_mapping_sizeof());
        assert!(lib.has_xcb_input_get_device_button_mapping());
        assert!(lib.has_xcb_input_get_device_button_mapping_unchecked());
        assert!(lib.has_xcb_input_get_device_button_mapping_map());
        assert!(lib.has_xcb_input_get_device_button_mapping_map_length());
        assert!(lib.has_xcb_input_get_device_button_mapping_map_end());
        assert!(lib.has_xcb_input_get_device_button_mapping_reply());
        assert!(lib.has_xcb_input_set_device_button_mapping_sizeof());
        assert!(lib.has_xcb_input_set_device_button_mapping());
        assert!(lib.has_xcb_input_set_device_button_mapping_unchecked());
        assert!(lib.has_xcb_input_set_device_button_mapping_reply());
        assert!(lib.has_xcb_input_key_state_next());
        assert!(lib.has_xcb_input_key_state_end());
        assert!(lib.has_xcb_input_button_state_next());
        assert!(lib.has_xcb_input_button_state_end());
        assert!(lib.has_xcb_input_valuator_state_sizeof());
        assert!(lib.has_xcb_input_valuator_state_valuators());
        assert!(lib.has_xcb_input_valuator_state_valuators_length());
        assert!(lib.has_xcb_input_valuator_state_valuators_end());
        assert!(lib.has_xcb_input_valuator_state_next());
        assert!(lib.has_xcb_input_valuator_state_end());
        assert!(lib.has_xcb_input_input_state_data_valuator_valuators());
        assert!(lib.has_xcb_input_input_state_data_valuator_valuators_length());
        assert!(lib.has_xcb_input_input_state_data_valuator_valuators_end());
        assert!(lib.has_xcb_input_input_state_data_serialize());
        assert!(lib.has_xcb_input_input_state_data_unpack());
        assert!(lib.has_xcb_input_input_state_data_sizeof());
        assert!(lib.has_xcb_input_input_state_sizeof());
        assert!(lib.has_xcb_input_input_state_data());
        assert!(lib.has_xcb_input_input_state_next());
        assert!(lib.has_xcb_input_input_state_end());
        assert!(lib.has_xcb_input_query_device_state_sizeof());
        assert!(lib.has_xcb_input_query_device_state());
        assert!(lib.has_xcb_input_query_device_state_unchecked());
        assert!(lib.has_xcb_input_query_device_state_classes_length());
        assert!(lib.has_xcb_input_query_device_state_classes_iterator());
        assert!(lib.has_xcb_input_query_device_state_reply());
        assert!(lib.has_xcb_input_device_bell_checked());
        assert!(lib.has_xcb_input_device_bell());
        assert!(lib.has_xcb_input_set_device_valuators_sizeof());
        assert!(lib.has_xcb_input_set_device_valuators());
        assert!(lib.has_xcb_input_set_device_valuators_unchecked());
        assert!(lib.has_xcb_input_set_device_valuators_reply());
        assert!(lib.has_xcb_input_device_resolution_state_sizeof());
        assert!(lib.has_xcb_input_device_resolution_state_resolution_values());
        assert!(lib.has_xcb_input_device_resolution_state_resolution_values_length());
        assert!(lib.has_xcb_input_device_resolution_state_resolution_values_end());
        assert!(lib.has_xcb_input_device_resolution_state_resolution_min());
        assert!(lib.has_xcb_input_device_resolution_state_resolution_min_length());
        assert!(lib.has_xcb_input_device_resolution_state_resolution_min_end());
        assert!(lib.has_xcb_input_device_resolution_state_resolution_max());
        assert!(lib.has_xcb_input_device_resolution_state_resolution_max_length());
        assert!(lib.has_xcb_input_device_resolution_state_resolution_max_end());
        assert!(lib.has_xcb_input_device_resolution_state_next());
        assert!(lib.has_xcb_input_device_resolution_state_end());
        assert!(lib.has_xcb_input_device_abs_calib_state_next());
        assert!(lib.has_xcb_input_device_abs_calib_state_end());
        assert!(lib.has_xcb_input_device_abs_area_state_next());
        assert!(lib.has_xcb_input_device_abs_area_state_end());
        assert!(lib.has_xcb_input_device_core_state_next());
        assert!(lib.has_xcb_input_device_core_state_end());
        assert!(lib.has_xcb_input_device_enable_state_next());
        assert!(lib.has_xcb_input_device_enable_state_end());
        assert!(lib.has_xcb_input_device_state_data_resolution_resolution_values());
        assert!(lib.has_xcb_input_device_state_data_resolution_resolution_values_length());
        assert!(lib.has_xcb_input_device_state_data_resolution_resolution_values_end());
        assert!(lib.has_xcb_input_device_state_data_resolution_resolution_min());
        assert!(lib.has_xcb_input_device_state_data_resolution_resolution_min_length());
        assert!(lib.has_xcb_input_device_state_data_resolution_resolution_min_end());
        assert!(lib.has_xcb_input_device_state_data_resolution_resolution_max());
        assert!(lib.has_xcb_input_device_state_data_resolution_resolution_max_length());
        assert!(lib.has_xcb_input_device_state_data_resolution_resolution_max_end());
        assert!(lib.has_xcb_input_device_state_data_serialize());
        assert!(lib.has_xcb_input_device_state_data_unpack());
        assert!(lib.has_xcb_input_device_state_data_sizeof());
        assert!(lib.has_xcb_input_device_state_sizeof());
        assert!(lib.has_xcb_input_device_state_data());
        assert!(lib.has_xcb_input_device_state_next());
        assert!(lib.has_xcb_input_device_state_end());
        assert!(lib.has_xcb_input_get_device_control_sizeof());
        assert!(lib.has_xcb_input_get_device_control());
        assert!(lib.has_xcb_input_get_device_control_unchecked());
        assert!(lib.has_xcb_input_get_device_control_control());
        assert!(lib.has_xcb_input_get_device_control_reply());
        assert!(lib.has_xcb_input_device_resolution_ctl_sizeof());
        assert!(lib.has_xcb_input_device_resolution_ctl_resolution_values());
        assert!(lib.has_xcb_input_device_resolution_ctl_resolution_values_length());
        assert!(lib.has_xcb_input_device_resolution_ctl_resolution_values_end());
        assert!(lib.has_xcb_input_device_resolution_ctl_next());
        assert!(lib.has_xcb_input_device_resolution_ctl_end());
        assert!(lib.has_xcb_input_device_abs_calib_ctl_next());
        assert!(lib.has_xcb_input_device_abs_calib_ctl_end());
        assert!(lib.has_xcb_input_device_abs_area_ctrl_next());
        assert!(lib.has_xcb_input_device_abs_area_ctrl_end());
        assert!(lib.has_xcb_input_device_core_ctrl_next());
        assert!(lib.has_xcb_input_device_core_ctrl_end());
        assert!(lib.has_xcb_input_device_enable_ctrl_next());
        assert!(lib.has_xcb_input_device_enable_ctrl_end());
        assert!(lib.has_xcb_input_device_ctl_data_resolution_resolution_values());
        assert!(lib.has_xcb_input_device_ctl_data_resolution_resolution_values_length());
        assert!(lib.has_xcb_input_device_ctl_data_resolution_resolution_values_end());
        assert!(lib.has_xcb_input_device_ctl_data_serialize());
        assert!(lib.has_xcb_input_device_ctl_data_unpack());
        assert!(lib.has_xcb_input_device_ctl_data_sizeof());
        assert!(lib.has_xcb_input_device_ctl_sizeof());
        assert!(lib.has_xcb_input_device_ctl_data());
        assert!(lib.has_xcb_input_device_ctl_next());
        assert!(lib.has_xcb_input_device_ctl_end());
        assert!(lib.has_xcb_input_change_device_control_sizeof());
        assert!(lib.has_xcb_input_change_device_control());
        assert!(lib.has_xcb_input_change_device_control_unchecked());
        assert!(lib.has_xcb_input_change_device_control_reply());
        assert!(lib.has_xcb_input_list_device_properties_sizeof());
        assert!(lib.has_xcb_input_list_device_properties());
        assert!(lib.has_xcb_input_list_device_properties_unchecked());
        assert!(lib.has_xcb_input_list_device_properties_atoms());
        assert!(lib.has_xcb_input_list_device_properties_atoms_length());
        assert!(lib.has_xcb_input_list_device_properties_atoms_end());
        assert!(lib.has_xcb_input_list_device_properties_reply());
        assert!(lib.has_xcb_input_change_device_property_items_data_8());
        assert!(lib.has_xcb_input_change_device_property_items_data_8_length());
        assert!(lib.has_xcb_input_change_device_property_items_data_8_end());
        assert!(lib.has_xcb_input_change_device_property_items_data_16());
        assert!(lib.has_xcb_input_change_device_property_items_data_16_length());
        assert!(lib.has_xcb_input_change_device_property_items_data_16_end());
        assert!(lib.has_xcb_input_change_device_property_items_data_32());
        assert!(lib.has_xcb_input_change_device_property_items_data_32_length());
        assert!(lib.has_xcb_input_change_device_property_items_data_32_end());
        assert!(lib.has_xcb_input_change_device_property_items_serialize());
        assert!(lib.has_xcb_input_change_device_property_items_unpack());
        assert!(lib.has_xcb_input_change_device_property_items_sizeof());
        assert!(lib.has_xcb_input_change_device_property_sizeof());
        assert!(lib.has_xcb_input_change_device_property_checked());
        assert!(lib.has_xcb_input_change_device_property());
        assert!(lib.has_xcb_input_change_device_property_aux_checked());
        assert!(lib.has_xcb_input_change_device_property_aux());
        assert!(lib.has_xcb_input_change_device_property_items());
        assert!(lib.has_xcb_input_delete_device_property_checked());
        assert!(lib.has_xcb_input_delete_device_property());
        assert!(lib.has_xcb_input_get_device_property_items_data_8());
        assert!(lib.has_xcb_input_get_device_property_items_data_8_length());
        assert!(lib.has_xcb_input_get_device_property_items_data_8_end());
        assert!(lib.has_xcb_input_get_device_property_items_data_16());
        assert!(lib.has_xcb_input_get_device_property_items_data_16_length());
        assert!(lib.has_xcb_input_get_device_property_items_data_16_end());
        assert!(lib.has_xcb_input_get_device_property_items_data_32());
        assert!(lib.has_xcb_input_get_device_property_items_data_32_length());
        assert!(lib.has_xcb_input_get_device_property_items_data_32_end());
        assert!(lib.has_xcb_input_get_device_property_items_serialize());
        assert!(lib.has_xcb_input_get_device_property_items_unpack());
        assert!(lib.has_xcb_input_get_device_property_items_sizeof());
        assert!(lib.has_xcb_input_get_device_property_sizeof());
        assert!(lib.has_xcb_input_get_device_property());
        assert!(lib.has_xcb_input_get_device_property_unchecked());
        assert!(lib.has_xcb_input_get_device_property_items());
        assert!(lib.has_xcb_input_get_device_property_reply());
        assert!(lib.has_xcb_input_group_info_next());
        assert!(lib.has_xcb_input_group_info_end());
        assert!(lib.has_xcb_input_modifier_info_next());
        assert!(lib.has_xcb_input_modifier_info_end());
        assert!(lib.has_xcb_input_xi_query_pointer_sizeof());
        assert!(lib.has_xcb_input_xi_query_pointer());
        assert!(lib.has_xcb_input_xi_query_pointer_unchecked());
        assert!(lib.has_xcb_input_xi_query_pointer_buttons());
        assert!(lib.has_xcb_input_xi_query_pointer_buttons_length());
        assert!(lib.has_xcb_input_xi_query_pointer_buttons_end());
        assert!(lib.has_xcb_input_xi_query_pointer_reply());
        assert!(lib.has_xcb_input_xi_warp_pointer_checked());
        assert!(lib.has_xcb_input_xi_warp_pointer());
        assert!(lib.has_xcb_input_xi_change_cursor_checked());
        assert!(lib.has_xcb_input_xi_change_cursor());
        assert!(lib.has_xcb_input_add_master_sizeof());
        assert!(lib.has_xcb_input_add_master_name());
        assert!(lib.has_xcb_input_add_master_name_length());
        assert!(lib.has_xcb_input_add_master_name_end());
        assert!(lib.has_xcb_input_add_master_next());
        assert!(lib.has_xcb_input_add_master_end());
        assert!(lib.has_xcb_input_remove_master_next());
        assert!(lib.has_xcb_input_remove_master_end());
        assert!(lib.has_xcb_input_attach_slave_next());
        assert!(lib.has_xcb_input_attach_slave_end());
        assert!(lib.has_xcb_input_detach_slave_next());
        assert!(lib.has_xcb_input_detach_slave_end());
        assert!(lib.has_xcb_input_hierarchy_change_data_add_master_name());
        assert!(lib.has_xcb_input_hierarchy_change_data_add_master_name_length());
        assert!(lib.has_xcb_input_hierarchy_change_data_add_master_name_end());
        assert!(lib.has_xcb_input_hierarchy_change_data_serialize());
        assert!(lib.has_xcb_input_hierarchy_change_data_unpack());
        assert!(lib.has_xcb_input_hierarchy_change_data_sizeof());
        assert!(lib.has_xcb_input_hierarchy_change_sizeof());
        assert!(lib.has_xcb_input_hierarchy_change_data());
        assert!(lib.has_xcb_input_hierarchy_change_next());
        assert!(lib.has_xcb_input_hierarchy_change_end());
        assert!(lib.has_xcb_input_xi_change_hierarchy_sizeof());
        assert!(lib.has_xcb_input_xi_change_hierarchy_checked());
        assert!(lib.has_xcb_input_xi_change_hierarchy());
        assert!(lib.has_xcb_input_xi_change_hierarchy_changes_length());
        assert!(lib.has_xcb_input_xi_change_hierarchy_changes_iterator());
        assert!(lib.has_xcb_input_xi_set_client_pointer_checked());
        assert!(lib.has_xcb_input_xi_set_client_pointer());
        assert!(lib.has_xcb_input_xi_get_client_pointer());
        assert!(lib.has_xcb_input_xi_get_client_pointer_unchecked());
        assert!(lib.has_xcb_input_xi_get_client_pointer_reply());
        assert!(lib.has_xcb_input_event_mask_sizeof());
        assert!(lib.has_xcb_input_event_mask_mask());
        assert!(lib.has_xcb_input_event_mask_mask_length());
        assert!(lib.has_xcb_input_event_mask_mask_end());
        assert!(lib.has_xcb_input_event_mask_next());
        assert!(lib.has_xcb_input_event_mask_end());
        assert!(lib.has_xcb_input_xi_select_events_sizeof());
        assert!(lib.has_xcb_input_xi_select_events_checked());
        assert!(lib.has_xcb_input_xi_select_events());
        assert!(lib.has_xcb_input_xi_select_events_masks_length());
        assert!(lib.has_xcb_input_xi_select_events_masks_iterator());
        assert!(lib.has_xcb_input_xi_query_version());
        assert!(lib.has_xcb_input_xi_query_version_unchecked());
        assert!(lib.has_xcb_input_xi_query_version_reply());
        assert!(lib.has_xcb_input_button_class_sizeof());
        assert!(lib.has_xcb_input_button_class_state());
        assert!(lib.has_xcb_input_button_class_state_length());
        assert!(lib.has_xcb_input_button_class_state_end());
        assert!(lib.has_xcb_input_button_class_labels());
        assert!(lib.has_xcb_input_button_class_labels_length());
        assert!(lib.has_xcb_input_button_class_labels_end());
        assert!(lib.has_xcb_input_button_class_next());
        assert!(lib.has_xcb_input_button_class_end());
        assert!(lib.has_xcb_input_key_class_sizeof());
        assert!(lib.has_xcb_input_key_class_keys());
        assert!(lib.has_xcb_input_key_class_keys_length());
        assert!(lib.has_xcb_input_key_class_keys_end());
        assert!(lib.has_xcb_input_key_class_next());
        assert!(lib.has_xcb_input_key_class_end());
        assert!(lib.has_xcb_input_scroll_class_next());
        assert!(lib.has_xcb_input_scroll_class_end());
        assert!(lib.has_xcb_input_touch_class_next());
        assert!(lib.has_xcb_input_touch_class_end());
        assert!(lib.has_xcb_input_valuator_class_next());
        assert!(lib.has_xcb_input_valuator_class_end());
        assert!(lib.has_xcb_input_device_class_data_key_keys());
        assert!(lib.has_xcb_input_device_class_data_key_keys_length());
        assert!(lib.has_xcb_input_device_class_data_key_keys_end());
        assert!(lib.has_xcb_input_device_class_data_button_state());
        assert!(lib.has_xcb_input_device_class_data_button_state_length());
        assert!(lib.has_xcb_input_device_class_data_button_state_end());
        assert!(lib.has_xcb_input_device_class_data_button_labels());
        assert!(lib.has_xcb_input_device_class_data_button_labels_length());
        assert!(lib.has_xcb_input_device_class_data_button_labels_end());
        assert!(lib.has_xcb_input_device_class_data_serialize());
        assert!(lib.has_xcb_input_device_class_data_unpack());
        assert!(lib.has_xcb_input_device_class_data_sizeof());
        assert!(lib.has_xcb_input_device_class_sizeof());
        assert!(lib.has_xcb_input_device_class_data());
        assert!(lib.has_xcb_input_device_class_next());
        assert!(lib.has_xcb_input_device_class_end());
        assert!(lib.has_xcb_input_xi_device_info_sizeof());
        assert!(lib.has_xcb_input_xi_device_info_name());
        assert!(lib.has_xcb_input_xi_device_info_name_length());
        assert!(lib.has_xcb_input_xi_device_info_name_end());
        assert!(lib.has_xcb_input_xi_device_info_classes_length());
        assert!(lib.has_xcb_input_xi_device_info_classes_iterator());
        assert!(lib.has_xcb_input_xi_device_info_next());
        assert!(lib.has_xcb_input_xi_device_info_end());
        assert!(lib.has_xcb_input_xi_query_device_sizeof());
        assert!(lib.has_xcb_input_xi_query_device());
        assert!(lib.has_xcb_input_xi_query_device_unchecked());
        assert!(lib.has_xcb_input_xi_query_device_infos_length());
        assert!(lib.has_xcb_input_xi_query_device_infos_iterator());
        assert!(lib.has_xcb_input_xi_query_device_reply());
        assert!(lib.has_xcb_input_xi_set_focus_checked());
        assert!(lib.has_xcb_input_xi_set_focus());
        assert!(lib.has_xcb_input_xi_get_focus());
        assert!(lib.has_xcb_input_xi_get_focus_unchecked());
        assert!(lib.has_xcb_input_xi_get_focus_reply());
        assert!(lib.has_xcb_input_xi_grab_device_sizeof());
        assert!(lib.has_xcb_input_xi_grab_device());
        assert!(lib.has_xcb_input_xi_grab_device_unchecked());
        assert!(lib.has_xcb_input_xi_grab_device_reply());
        assert!(lib.has_xcb_input_xi_ungrab_device_checked());
        assert!(lib.has_xcb_input_xi_ungrab_device());
        assert!(lib.has_xcb_input_xi_allow_events_checked());
        assert!(lib.has_xcb_input_xi_allow_events());
        assert!(lib.has_xcb_input_grab_modifier_info_next());
        assert!(lib.has_xcb_input_grab_modifier_info_end());
        assert!(lib.has_xcb_input_xi_passive_grab_device_sizeof());
        assert!(lib.has_xcb_input_xi_passive_grab_device());
        assert!(lib.has_xcb_input_xi_passive_grab_device_unchecked());
        assert!(lib.has_xcb_input_xi_passive_grab_device_modifiers());
        assert!(lib.has_xcb_input_xi_passive_grab_device_modifiers_length());
        assert!(lib.has_xcb_input_xi_passive_grab_device_modifiers_iterator());
        assert!(lib.has_xcb_input_xi_passive_grab_device_reply());
        assert!(lib.has_xcb_input_xi_passive_ungrab_device_sizeof());
        assert!(lib.has_xcb_input_xi_passive_ungrab_device_checked());
        assert!(lib.has_xcb_input_xi_passive_ungrab_device());
        assert!(lib.has_xcb_input_xi_passive_ungrab_device_modifiers());
        assert!(lib.has_xcb_input_xi_passive_ungrab_device_modifiers_length());
        assert!(lib.has_xcb_input_xi_passive_ungrab_device_modifiers_end());
        assert!(lib.has_xcb_input_xi_list_properties_sizeof());
        assert!(lib.has_xcb_input_xi_list_properties());
        assert!(lib.has_xcb_input_xi_list_properties_unchecked());
        assert!(lib.has_xcb_input_xi_list_properties_properties());
        assert!(lib.has_xcb_input_xi_list_properties_properties_length());
        assert!(lib.has_xcb_input_xi_list_properties_properties_end());
        assert!(lib.has_xcb_input_xi_list_properties_reply());
        assert!(lib.has_xcb_input_xi_change_property_items_data_8());
        assert!(lib.has_xcb_input_xi_change_property_items_data_8_length());
        assert!(lib.has_xcb_input_xi_change_property_items_data_8_end());
        assert!(lib.has_xcb_input_xi_change_property_items_data_16());
        assert!(lib.has_xcb_input_xi_change_property_items_data_16_length());
        assert!(lib.has_xcb_input_xi_change_property_items_data_16_end());
        assert!(lib.has_xcb_input_xi_change_property_items_data_32());
        assert!(lib.has_xcb_input_xi_change_property_items_data_32_length());
        assert!(lib.has_xcb_input_xi_change_property_items_data_32_end());
        assert!(lib.has_xcb_input_xi_change_property_items_serialize());
        assert!(lib.has_xcb_input_xi_change_property_items_unpack());
        assert!(lib.has_xcb_input_xi_change_property_items_sizeof());
        assert!(lib.has_xcb_input_xi_change_property_sizeof());
        assert!(lib.has_xcb_input_xi_change_property_checked());
        assert!(lib.has_xcb_input_xi_change_property());
        assert!(lib.has_xcb_input_xi_change_property_aux_checked());
        assert!(lib.has_xcb_input_xi_change_property_aux());
        assert!(lib.has_xcb_input_xi_change_property_items());
        assert!(lib.has_xcb_input_xi_delete_property_checked());
        assert!(lib.has_xcb_input_xi_delete_property());
        assert!(lib.has_xcb_input_xi_get_property_items_data_8());
        assert!(lib.has_xcb_input_xi_get_property_items_data_8_length());
        assert!(lib.has_xcb_input_xi_get_property_items_data_8_end());
        assert!(lib.has_xcb_input_xi_get_property_items_data_16());
        assert!(lib.has_xcb_input_xi_get_property_items_data_16_length());
        assert!(lib.has_xcb_input_xi_get_property_items_data_16_end());
        assert!(lib.has_xcb_input_xi_get_property_items_data_32());
        assert!(lib.has_xcb_input_xi_get_property_items_data_32_length());
        assert!(lib.has_xcb_input_xi_get_property_items_data_32_end());
        assert!(lib.has_xcb_input_xi_get_property_items_serialize());
        assert!(lib.has_xcb_input_xi_get_property_items_unpack());
        assert!(lib.has_xcb_input_xi_get_property_items_sizeof());
        assert!(lib.has_xcb_input_xi_get_property_sizeof());
        assert!(lib.has_xcb_input_xi_get_property());
        assert!(lib.has_xcb_input_xi_get_property_unchecked());
        assert!(lib.has_xcb_input_xi_get_property_items());
        assert!(lib.has_xcb_input_xi_get_property_reply());
        assert!(lib.has_xcb_input_xi_get_selected_events_sizeof());
        assert!(lib.has_xcb_input_xi_get_selected_events());
        assert!(lib.has_xcb_input_xi_get_selected_events_unchecked());
        assert!(lib.has_xcb_input_xi_get_selected_events_masks_length());
        assert!(lib.has_xcb_input_xi_get_selected_events_masks_iterator());
        assert!(lib.has_xcb_input_xi_get_selected_events_reply());
        assert!(lib.has_xcb_input_barrier_release_pointer_info_next());
        assert!(lib.has_xcb_input_barrier_release_pointer_info_end());
        assert!(lib.has_xcb_input_xi_barrier_release_pointer_sizeof());
        assert!(lib.has_xcb_input_xi_barrier_release_pointer_checked());
        assert!(lib.has_xcb_input_xi_barrier_release_pointer());
        assert!(lib.has_xcb_input_xi_barrier_release_pointer_barriers());
        assert!(lib.has_xcb_input_xi_barrier_release_pointer_barriers_length());
        assert!(lib.has_xcb_input_xi_barrier_release_pointer_barriers_iterator());
        assert!(lib.has_xcb_input_device_changed_sizeof());
        assert!(lib.has_xcb_input_device_changed_classes_length());
        assert!(lib.has_xcb_input_device_changed_classes_iterator());
        assert!(lib.has_xcb_input_key_press_sizeof());
        assert!(lib.has_xcb_input_key_press_button_mask());
        assert!(lib.has_xcb_input_key_press_button_mask_length());
        assert!(lib.has_xcb_input_key_press_button_mask_end());
        assert!(lib.has_xcb_input_key_press_valuator_mask());
        assert!(lib.has_xcb_input_key_press_valuator_mask_length());
        assert!(lib.has_xcb_input_key_press_valuator_mask_end());
        assert!(lib.has_xcb_input_key_press_axisvalues());
        assert!(lib.has_xcb_input_key_press_axisvalues_length());
        assert!(lib.has_xcb_input_key_press_axisvalues_iterator());
        assert!(lib.has_xcb_input_key_release_sizeof());
        assert!(lib.has_xcb_input_button_press_sizeof());
        assert!(lib.has_xcb_input_button_press_button_mask());
        assert!(lib.has_xcb_input_button_press_button_mask_length());
        assert!(lib.has_xcb_input_button_press_button_mask_end());
        assert!(lib.has_xcb_input_button_press_valuator_mask());
        assert!(lib.has_xcb_input_button_press_valuator_mask_length());
        assert!(lib.has_xcb_input_button_press_valuator_mask_end());
        assert!(lib.has_xcb_input_button_press_axisvalues());
        assert!(lib.has_xcb_input_button_press_axisvalues_length());
        assert!(lib.has_xcb_input_button_press_axisvalues_iterator());
        assert!(lib.has_xcb_input_button_release_sizeof());
        assert!(lib.has_xcb_input_motion_sizeof());
        assert!(lib.has_xcb_input_enter_sizeof());
        assert!(lib.has_xcb_input_enter_buttons());
        assert!(lib.has_xcb_input_enter_buttons_length());
        assert!(lib.has_xcb_input_enter_buttons_end());
        assert!(lib.has_xcb_input_leave_sizeof());
        assert!(lib.has_xcb_input_focus_in_sizeof());
        assert!(lib.has_xcb_input_focus_out_sizeof());
        assert!(lib.has_xcb_input_hierarchy_info_next());
        assert!(lib.has_xcb_input_hierarchy_info_end());
        assert!(lib.has_xcb_input_hierarchy_sizeof());
        assert!(lib.has_xcb_input_hierarchy_infos());
        assert!(lib.has_xcb_input_hierarchy_infos_length());
        assert!(lib.has_xcb_input_hierarchy_infos_iterator());
        assert!(lib.has_xcb_input_raw_key_press_sizeof());
        assert!(lib.has_xcb_input_raw_key_press_valuator_mask());
        assert!(lib.has_xcb_input_raw_key_press_valuator_mask_length());
        assert!(lib.has_xcb_input_raw_key_press_valuator_mask_end());
        assert!(lib.has_xcb_input_raw_key_press_axisvalues());
        assert!(lib.has_xcb_input_raw_key_press_axisvalues_length());
        assert!(lib.has_xcb_input_raw_key_press_axisvalues_iterator());
        assert!(lib.has_xcb_input_raw_key_press_axisvalues_raw());
        assert!(lib.has_xcb_input_raw_key_press_axisvalues_raw_length());
        assert!(lib.has_xcb_input_raw_key_press_axisvalues_raw_iterator());
        assert!(lib.has_xcb_input_raw_key_release_sizeof());
        assert!(lib.has_xcb_input_raw_button_press_sizeof());
        assert!(lib.has_xcb_input_raw_button_press_valuator_mask());
        assert!(lib.has_xcb_input_raw_button_press_valuator_mask_length());
        assert!(lib.has_xcb_input_raw_button_press_valuator_mask_end());
        assert!(lib.has_xcb_input_raw_button_press_axisvalues());
        assert!(lib.has_xcb_input_raw_button_press_axisvalues_length());
        assert!(lib.has_xcb_input_raw_button_press_axisvalues_iterator());
        assert!(lib.has_xcb_input_raw_button_press_axisvalues_raw());
        assert!(lib.has_xcb_input_raw_button_press_axisvalues_raw_length());
        assert!(lib.has_xcb_input_raw_button_press_axisvalues_raw_iterator());
        assert!(lib.has_xcb_input_raw_button_release_sizeof());
        assert!(lib.has_xcb_input_raw_motion_sizeof());
        assert!(lib.has_xcb_input_touch_begin_sizeof());
        assert!(lib.has_xcb_input_touch_begin_button_mask());
        assert!(lib.has_xcb_input_touch_begin_button_mask_length());
        assert!(lib.has_xcb_input_touch_begin_button_mask_end());
        assert!(lib.has_xcb_input_touch_begin_valuator_mask());
        assert!(lib.has_xcb_input_touch_begin_valuator_mask_length());
        assert!(lib.has_xcb_input_touch_begin_valuator_mask_end());
        assert!(lib.has_xcb_input_touch_begin_axisvalues());
        assert!(lib.has_xcb_input_touch_begin_axisvalues_length());
        assert!(lib.has_xcb_input_touch_begin_axisvalues_iterator());
        assert!(lib.has_xcb_input_touch_update_sizeof());
        assert!(lib.has_xcb_input_touch_end_sizeof());
        assert!(lib.has_xcb_input_raw_touch_begin_sizeof());
        assert!(lib.has_xcb_input_raw_touch_begin_valuator_mask());
        assert!(lib.has_xcb_input_raw_touch_begin_valuator_mask_length());
        assert!(lib.has_xcb_input_raw_touch_begin_valuator_mask_end());
        assert!(lib.has_xcb_input_raw_touch_begin_axisvalues());
        assert!(lib.has_xcb_input_raw_touch_begin_axisvalues_length());
        assert!(lib.has_xcb_input_raw_touch_begin_axisvalues_iterator());
        assert!(lib.has_xcb_input_raw_touch_begin_axisvalues_raw());
        assert!(lib.has_xcb_input_raw_touch_begin_axisvalues_raw_length());
        assert!(lib.has_xcb_input_raw_touch_begin_axisvalues_raw_iterator());
        assert!(lib.has_xcb_input_raw_touch_update_sizeof());
        assert!(lib.has_xcb_input_raw_touch_end_sizeof());
        assert!(lib.has_xcb_input_event_for_send_next());
        assert!(lib.has_xcb_input_event_for_send_end());
        assert!(lib.has_xcb_input_send_extension_event_sizeof());
        assert!(lib.has_xcb_input_send_extension_event_checked());
        assert!(lib.has_xcb_input_send_extension_event());
        assert!(lib.has_xcb_input_send_extension_event_events());
        assert!(lib.has_xcb_input_send_extension_event_events_length());
        assert!(lib.has_xcb_input_send_extension_event_events_iterator());
        assert!(lib.has_xcb_input_send_extension_event_classes());
        assert!(lib.has_xcb_input_send_extension_event_classes_length());
        assert!(lib.has_xcb_input_send_extension_event_classes_end());
    }
}
