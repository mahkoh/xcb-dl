// This file was generated using generate.py. Do not edit.
#![allow(unused_macros)]

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

/// The name of the `xkb` extension.
pub const XCB_XKB_NAME: &[u8] = b"XKEYBOARD";

/// The name of the `xkb` extension.
pub const XCB_XKB_NAME_STR: &str = "XKEYBOARD";

/// The `xkb::Const` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::Const::MaxLegalKeyCode`](XCB_XKB_CONST_MAX_LEGAL_KEY_CODE)
/// - [`xkb::Const::PerKeyBitArraySize`](XCB_XKB_CONST_PER_KEY_BIT_ARRAY_SIZE)
/// - [`xkb::Const::KeyNameLength`](XCB_XKB_CONST_KEY_NAME_LENGTH)
pub type xcb_xkb_const_t = u32;
/// The `xkb::Const::MaxLegalKeyCode` enum variant.
///
/// This is a variant of [`xcb_xkb_const_t`].
pub const XCB_XKB_CONST_MAX_LEGAL_KEY_CODE: xcb_xkb_const_t = 255;
/// The `xkb::Const::PerKeyBitArraySize` enum variant.
///
/// This is a variant of [`xcb_xkb_const_t`].
pub const XCB_XKB_CONST_PER_KEY_BIT_ARRAY_SIZE: xcb_xkb_const_t = 32;
/// The `xkb::Const::KeyNameLength` enum variant.
///
/// This is a variant of [`xcb_xkb_const_t`].
pub const XCB_XKB_CONST_KEY_NAME_LENGTH: xcb_xkb_const_t = 4;

/// The `xkb::EventType` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::EventType::NewKeyboardNotify`](XCB_XKB_EVENT_TYPE_NEW_KEYBOARD_NOTIFY)
/// - [`xkb::EventType::MapNotify`](XCB_XKB_EVENT_TYPE_MAP_NOTIFY)
/// - [`xkb::EventType::StateNotify`](XCB_XKB_EVENT_TYPE_STATE_NOTIFY)
/// - [`xkb::EventType::ControlsNotify`](XCB_XKB_EVENT_TYPE_CONTROLS_NOTIFY)
/// - [`xkb::EventType::IndicatorStateNotify`](XCB_XKB_EVENT_TYPE_INDICATOR_STATE_NOTIFY)
/// - [`xkb::EventType::IndicatorMapNotify`](XCB_XKB_EVENT_TYPE_INDICATOR_MAP_NOTIFY)
/// - [`xkb::EventType::NamesNotify`](XCB_XKB_EVENT_TYPE_NAMES_NOTIFY)
/// - [`xkb::EventType::CompatMapNotify`](XCB_XKB_EVENT_TYPE_COMPAT_MAP_NOTIFY)
/// - [`xkb::EventType::BellNotify`](XCB_XKB_EVENT_TYPE_BELL_NOTIFY)
/// - [`xkb::EventType::ActionMessage`](XCB_XKB_EVENT_TYPE_ACTION_MESSAGE)
/// - [`xkb::EventType::AccessXNotify`](XCB_XKB_EVENT_TYPE_ACCESS_X_NOTIFY)
/// - [`xkb::EventType::ExtensionDeviceNotify`](XCB_XKB_EVENT_TYPE_EXTENSION_DEVICE_NOTIFY)
pub type xcb_xkb_event_type_t = u32;
/// The `xkb::EventType::NewKeyboardNotify` enum variant.
///
/// This is a variant of [`xcb_xkb_event_type_t`].
pub const XCB_XKB_EVENT_TYPE_NEW_KEYBOARD_NOTIFY: xcb_xkb_event_type_t = 1;
/// The `xkb::EventType::MapNotify` enum variant.
///
/// This is a variant of [`xcb_xkb_event_type_t`].
pub const XCB_XKB_EVENT_TYPE_MAP_NOTIFY: xcb_xkb_event_type_t = 2;
/// The `xkb::EventType::StateNotify` enum variant.
///
/// This is a variant of [`xcb_xkb_event_type_t`].
pub const XCB_XKB_EVENT_TYPE_STATE_NOTIFY: xcb_xkb_event_type_t = 4;
/// The `xkb::EventType::ControlsNotify` enum variant.
///
/// This is a variant of [`xcb_xkb_event_type_t`].
pub const XCB_XKB_EVENT_TYPE_CONTROLS_NOTIFY: xcb_xkb_event_type_t = 8;
/// The `xkb::EventType::IndicatorStateNotify` enum variant.
///
/// This is a variant of [`xcb_xkb_event_type_t`].
pub const XCB_XKB_EVENT_TYPE_INDICATOR_STATE_NOTIFY: xcb_xkb_event_type_t = 16;
/// The `xkb::EventType::IndicatorMapNotify` enum variant.
///
/// This is a variant of [`xcb_xkb_event_type_t`].
pub const XCB_XKB_EVENT_TYPE_INDICATOR_MAP_NOTIFY: xcb_xkb_event_type_t = 32;
/// The `xkb::EventType::NamesNotify` enum variant.
///
/// This is a variant of [`xcb_xkb_event_type_t`].
pub const XCB_XKB_EVENT_TYPE_NAMES_NOTIFY: xcb_xkb_event_type_t = 64;
/// The `xkb::EventType::CompatMapNotify` enum variant.
///
/// This is a variant of [`xcb_xkb_event_type_t`].
pub const XCB_XKB_EVENT_TYPE_COMPAT_MAP_NOTIFY: xcb_xkb_event_type_t = 128;
/// The `xkb::EventType::BellNotify` enum variant.
///
/// This is a variant of [`xcb_xkb_event_type_t`].
pub const XCB_XKB_EVENT_TYPE_BELL_NOTIFY: xcb_xkb_event_type_t = 256;
/// The `xkb::EventType::ActionMessage` enum variant.
///
/// This is a variant of [`xcb_xkb_event_type_t`].
pub const XCB_XKB_EVENT_TYPE_ACTION_MESSAGE: xcb_xkb_event_type_t = 512;
/// The `xkb::EventType::AccessXNotify` enum variant.
///
/// This is a variant of [`xcb_xkb_event_type_t`].
pub const XCB_XKB_EVENT_TYPE_ACCESS_X_NOTIFY: xcb_xkb_event_type_t = 1024;
/// The `xkb::EventType::ExtensionDeviceNotify` enum variant.
///
/// This is a variant of [`xcb_xkb_event_type_t`].
pub const XCB_XKB_EVENT_TYPE_EXTENSION_DEVICE_NOTIFY: xcb_xkb_event_type_t = 2048;

/// The `xkb::NKNDetail` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::NKNDetail::Keycodes`](XCB_XKB_NKN_DETAIL_KEYCODES)
/// - [`xkb::NKNDetail::Geometry`](XCB_XKB_NKN_DETAIL_GEOMETRY)
/// - [`xkb::NKNDetail::DeviceID`](XCB_XKB_NKN_DETAIL_DEVICE_ID)
pub type xcb_xkb_nkn_detail_t = u32;
/// The `xkb::NKNDetail::Keycodes` enum variant.
///
/// This is a variant of [`xcb_xkb_nkn_detail_t`].
pub const XCB_XKB_NKN_DETAIL_KEYCODES: xcb_xkb_nkn_detail_t = 1;
/// The `xkb::NKNDetail::Geometry` enum variant.
///
/// This is a variant of [`xcb_xkb_nkn_detail_t`].
pub const XCB_XKB_NKN_DETAIL_GEOMETRY: xcb_xkb_nkn_detail_t = 2;
/// The `xkb::NKNDetail::DeviceID` enum variant.
///
/// This is a variant of [`xcb_xkb_nkn_detail_t`].
pub const XCB_XKB_NKN_DETAIL_DEVICE_ID: xcb_xkb_nkn_detail_t = 4;

/// The `xkb::AXNDetail` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::AXNDetail::SKPress`](XCB_XKB_AXN_DETAIL_SK_PRESS)
/// - [`xkb::AXNDetail::SKAccept`](XCB_XKB_AXN_DETAIL_SK_ACCEPT)
/// - [`xkb::AXNDetail::SKReject`](XCB_XKB_AXN_DETAIL_SK_REJECT)
/// - [`xkb::AXNDetail::SKRelease`](XCB_XKB_AXN_DETAIL_SK_RELEASE)
/// - [`xkb::AXNDetail::BKAccept`](XCB_XKB_AXN_DETAIL_BK_ACCEPT)
/// - [`xkb::AXNDetail::BKReject`](XCB_XKB_AXN_DETAIL_BK_REJECT)
/// - [`xkb::AXNDetail::AXKWarning`](XCB_XKB_AXN_DETAIL_AXK_WARNING)
pub type xcb_xkb_axn_detail_t = u32;
/// The `xkb::AXNDetail::SKPress` enum variant.
///
/// This is a variant of [`xcb_xkb_axn_detail_t`].
pub const XCB_XKB_AXN_DETAIL_SK_PRESS: xcb_xkb_axn_detail_t = 1;
/// The `xkb::AXNDetail::SKAccept` enum variant.
///
/// This is a variant of [`xcb_xkb_axn_detail_t`].
pub const XCB_XKB_AXN_DETAIL_SK_ACCEPT: xcb_xkb_axn_detail_t = 2;
/// The `xkb::AXNDetail::SKReject` enum variant.
///
/// This is a variant of [`xcb_xkb_axn_detail_t`].
pub const XCB_XKB_AXN_DETAIL_SK_REJECT: xcb_xkb_axn_detail_t = 4;
/// The `xkb::AXNDetail::SKRelease` enum variant.
///
/// This is a variant of [`xcb_xkb_axn_detail_t`].
pub const XCB_XKB_AXN_DETAIL_SK_RELEASE: xcb_xkb_axn_detail_t = 8;
/// The `xkb::AXNDetail::BKAccept` enum variant.
///
/// This is a variant of [`xcb_xkb_axn_detail_t`].
pub const XCB_XKB_AXN_DETAIL_BK_ACCEPT: xcb_xkb_axn_detail_t = 16;
/// The `xkb::AXNDetail::BKReject` enum variant.
///
/// This is a variant of [`xcb_xkb_axn_detail_t`].
pub const XCB_XKB_AXN_DETAIL_BK_REJECT: xcb_xkb_axn_detail_t = 32;
/// The `xkb::AXNDetail::AXKWarning` enum variant.
///
/// This is a variant of [`xcb_xkb_axn_detail_t`].
pub const XCB_XKB_AXN_DETAIL_AXK_WARNING: xcb_xkb_axn_detail_t = 64;

/// The `xkb::MapPart` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::MapPart::KeyTypes`](XCB_XKB_MAP_PART_KEY_TYPES)
/// - [`xkb::MapPart::KeySyms`](XCB_XKB_MAP_PART_KEY_SYMS)
/// - [`xkb::MapPart::ModifierMap`](XCB_XKB_MAP_PART_MODIFIER_MAP)
/// - [`xkb::MapPart::ExplicitComponents`](XCB_XKB_MAP_PART_EXPLICIT_COMPONENTS)
/// - [`xkb::MapPart::KeyActions`](XCB_XKB_MAP_PART_KEY_ACTIONS)
/// - [`xkb::MapPart::KeyBehaviors`](XCB_XKB_MAP_PART_KEY_BEHAVIORS)
/// - [`xkb::MapPart::VirtualMods`](XCB_XKB_MAP_PART_VIRTUAL_MODS)
/// - [`xkb::MapPart::VirtualModMap`](XCB_XKB_MAP_PART_VIRTUAL_MOD_MAP)
pub type xcb_xkb_map_part_t = u32;
/// The `xkb::MapPart::KeyTypes` enum variant.
///
/// This is a variant of [`xcb_xkb_map_part_t`].
pub const XCB_XKB_MAP_PART_KEY_TYPES: xcb_xkb_map_part_t = 1;
/// The `xkb::MapPart::KeySyms` enum variant.
///
/// This is a variant of [`xcb_xkb_map_part_t`].
pub const XCB_XKB_MAP_PART_KEY_SYMS: xcb_xkb_map_part_t = 2;
/// The `xkb::MapPart::ModifierMap` enum variant.
///
/// This is a variant of [`xcb_xkb_map_part_t`].
pub const XCB_XKB_MAP_PART_MODIFIER_MAP: xcb_xkb_map_part_t = 4;
/// The `xkb::MapPart::ExplicitComponents` enum variant.
///
/// This is a variant of [`xcb_xkb_map_part_t`].
pub const XCB_XKB_MAP_PART_EXPLICIT_COMPONENTS: xcb_xkb_map_part_t = 8;
/// The `xkb::MapPart::KeyActions` enum variant.
///
/// This is a variant of [`xcb_xkb_map_part_t`].
pub const XCB_XKB_MAP_PART_KEY_ACTIONS: xcb_xkb_map_part_t = 16;
/// The `xkb::MapPart::KeyBehaviors` enum variant.
///
/// This is a variant of [`xcb_xkb_map_part_t`].
pub const XCB_XKB_MAP_PART_KEY_BEHAVIORS: xcb_xkb_map_part_t = 32;
/// The `xkb::MapPart::VirtualMods` enum variant.
///
/// This is a variant of [`xcb_xkb_map_part_t`].
pub const XCB_XKB_MAP_PART_VIRTUAL_MODS: xcb_xkb_map_part_t = 64;
/// The `xkb::MapPart::VirtualModMap` enum variant.
///
/// This is a variant of [`xcb_xkb_map_part_t`].
pub const XCB_XKB_MAP_PART_VIRTUAL_MOD_MAP: xcb_xkb_map_part_t = 128;

/// The `xkb::SetMapFlags` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::SetMapFlags::ResizeTypes`](XCB_XKB_SET_MAP_FLAGS_RESIZE_TYPES)
/// - [`xkb::SetMapFlags::RecomputeActions`](XCB_XKB_SET_MAP_FLAGS_RECOMPUTE_ACTIONS)
pub type xcb_xkb_set_map_flags_t = u32;
/// The `xkb::SetMapFlags::ResizeTypes` enum variant.
///
/// This is a variant of [`xcb_xkb_set_map_flags_t`].
pub const XCB_XKB_SET_MAP_FLAGS_RESIZE_TYPES: xcb_xkb_set_map_flags_t = 1;
/// The `xkb::SetMapFlags::RecomputeActions` enum variant.
///
/// This is a variant of [`xcb_xkb_set_map_flags_t`].
pub const XCB_XKB_SET_MAP_FLAGS_RECOMPUTE_ACTIONS: xcb_xkb_set_map_flags_t = 2;

/// The `xkb::StatePart` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::StatePart::ModifierState`](XCB_XKB_STATE_PART_MODIFIER_STATE)
/// - [`xkb::StatePart::ModifierBase`](XCB_XKB_STATE_PART_MODIFIER_BASE)
/// - [`xkb::StatePart::ModifierLatch`](XCB_XKB_STATE_PART_MODIFIER_LATCH)
/// - [`xkb::StatePart::ModifierLock`](XCB_XKB_STATE_PART_MODIFIER_LOCK)
/// - [`xkb::StatePart::GroupState`](XCB_XKB_STATE_PART_GROUP_STATE)
/// - [`xkb::StatePart::GroupBase`](XCB_XKB_STATE_PART_GROUP_BASE)
/// - [`xkb::StatePart::GroupLatch`](XCB_XKB_STATE_PART_GROUP_LATCH)
/// - [`xkb::StatePart::GroupLock`](XCB_XKB_STATE_PART_GROUP_LOCK)
/// - [`xkb::StatePart::CompatState`](XCB_XKB_STATE_PART_COMPAT_STATE)
/// - [`xkb::StatePart::GrabMods`](XCB_XKB_STATE_PART_GRAB_MODS)
/// - [`xkb::StatePart::CompatGrabMods`](XCB_XKB_STATE_PART_COMPAT_GRAB_MODS)
/// - [`xkb::StatePart::LookupMods`](XCB_XKB_STATE_PART_LOOKUP_MODS)
/// - [`xkb::StatePart::CompatLookupMods`](XCB_XKB_STATE_PART_COMPAT_LOOKUP_MODS)
/// - [`xkb::StatePart::PointerButtons`](XCB_XKB_STATE_PART_POINTER_BUTTONS)
pub type xcb_xkb_state_part_t = u32;
/// The `xkb::StatePart::ModifierState` enum variant.
///
/// This is a variant of [`xcb_xkb_state_part_t`].
pub const XCB_XKB_STATE_PART_MODIFIER_STATE: xcb_xkb_state_part_t = 1;
/// The `xkb::StatePart::ModifierBase` enum variant.
///
/// This is a variant of [`xcb_xkb_state_part_t`].
pub const XCB_XKB_STATE_PART_MODIFIER_BASE: xcb_xkb_state_part_t = 2;
/// The `xkb::StatePart::ModifierLatch` enum variant.
///
/// This is a variant of [`xcb_xkb_state_part_t`].
pub const XCB_XKB_STATE_PART_MODIFIER_LATCH: xcb_xkb_state_part_t = 4;
/// The `xkb::StatePart::ModifierLock` enum variant.
///
/// This is a variant of [`xcb_xkb_state_part_t`].
pub const XCB_XKB_STATE_PART_MODIFIER_LOCK: xcb_xkb_state_part_t = 8;
/// The `xkb::StatePart::GroupState` enum variant.
///
/// This is a variant of [`xcb_xkb_state_part_t`].
pub const XCB_XKB_STATE_PART_GROUP_STATE: xcb_xkb_state_part_t = 16;
/// The `xkb::StatePart::GroupBase` enum variant.
///
/// This is a variant of [`xcb_xkb_state_part_t`].
pub const XCB_XKB_STATE_PART_GROUP_BASE: xcb_xkb_state_part_t = 32;
/// The `xkb::StatePart::GroupLatch` enum variant.
///
/// This is a variant of [`xcb_xkb_state_part_t`].
pub const XCB_XKB_STATE_PART_GROUP_LATCH: xcb_xkb_state_part_t = 64;
/// The `xkb::StatePart::GroupLock` enum variant.
///
/// This is a variant of [`xcb_xkb_state_part_t`].
pub const XCB_XKB_STATE_PART_GROUP_LOCK: xcb_xkb_state_part_t = 128;
/// The `xkb::StatePart::CompatState` enum variant.
///
/// This is a variant of [`xcb_xkb_state_part_t`].
pub const XCB_XKB_STATE_PART_COMPAT_STATE: xcb_xkb_state_part_t = 256;
/// The `xkb::StatePart::GrabMods` enum variant.
///
/// This is a variant of [`xcb_xkb_state_part_t`].
pub const XCB_XKB_STATE_PART_GRAB_MODS: xcb_xkb_state_part_t = 512;
/// The `xkb::StatePart::CompatGrabMods` enum variant.
///
/// This is a variant of [`xcb_xkb_state_part_t`].
pub const XCB_XKB_STATE_PART_COMPAT_GRAB_MODS: xcb_xkb_state_part_t = 1024;
/// The `xkb::StatePart::LookupMods` enum variant.
///
/// This is a variant of [`xcb_xkb_state_part_t`].
pub const XCB_XKB_STATE_PART_LOOKUP_MODS: xcb_xkb_state_part_t = 2048;
/// The `xkb::StatePart::CompatLookupMods` enum variant.
///
/// This is a variant of [`xcb_xkb_state_part_t`].
pub const XCB_XKB_STATE_PART_COMPAT_LOOKUP_MODS: xcb_xkb_state_part_t = 4096;
/// The `xkb::StatePart::PointerButtons` enum variant.
///
/// This is a variant of [`xcb_xkb_state_part_t`].
pub const XCB_XKB_STATE_PART_POINTER_BUTTONS: xcb_xkb_state_part_t = 8192;

/// The `xkb::BoolCtrl` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::BoolCtrl::RepeatKeys`](XCB_XKB_BOOL_CTRL_REPEAT_KEYS)
/// - [`xkb::BoolCtrl::SlowKeys`](XCB_XKB_BOOL_CTRL_SLOW_KEYS)
/// - [`xkb::BoolCtrl::BounceKeys`](XCB_XKB_BOOL_CTRL_BOUNCE_KEYS)
/// - [`xkb::BoolCtrl::StickyKeys`](XCB_XKB_BOOL_CTRL_STICKY_KEYS)
/// - [`xkb::BoolCtrl::MouseKeys`](XCB_XKB_BOOL_CTRL_MOUSE_KEYS)
/// - [`xkb::BoolCtrl::MouseKeysAccel`](XCB_XKB_BOOL_CTRL_MOUSE_KEYS_ACCEL)
/// - [`xkb::BoolCtrl::AccessXKeys`](XCB_XKB_BOOL_CTRL_ACCESS_X_KEYS)
/// - [`xkb::BoolCtrl::AccessXTimeoutMask`](XCB_XKB_BOOL_CTRL_ACCESS_X_TIMEOUT_MASK)
/// - [`xkb::BoolCtrl::AccessXFeedbackMask`](XCB_XKB_BOOL_CTRL_ACCESS_X_FEEDBACK_MASK)
/// - [`xkb::BoolCtrl::AudibleBellMask`](XCB_XKB_BOOL_CTRL_AUDIBLE_BELL_MASK)
/// - [`xkb::BoolCtrl::Overlay1Mask`](XCB_XKB_BOOL_CTRL_OVERLAY_1_MASK)
/// - [`xkb::BoolCtrl::Overlay2Mask`](XCB_XKB_BOOL_CTRL_OVERLAY_2_MASK)
/// - [`xkb::BoolCtrl::IgnoreGroupLockMask`](XCB_XKB_BOOL_CTRL_IGNORE_GROUP_LOCK_MASK)
pub type xcb_xkb_bool_ctrl_t = u32;
/// The `xkb::BoolCtrl::RepeatKeys` enum variant.
///
/// This is a variant of [`xcb_xkb_bool_ctrl_t`].
pub const XCB_XKB_BOOL_CTRL_REPEAT_KEYS: xcb_xkb_bool_ctrl_t = 1;
/// The `xkb::BoolCtrl::SlowKeys` enum variant.
///
/// This is a variant of [`xcb_xkb_bool_ctrl_t`].
pub const XCB_XKB_BOOL_CTRL_SLOW_KEYS: xcb_xkb_bool_ctrl_t = 2;
/// The `xkb::BoolCtrl::BounceKeys` enum variant.
///
/// This is a variant of [`xcb_xkb_bool_ctrl_t`].
pub const XCB_XKB_BOOL_CTRL_BOUNCE_KEYS: xcb_xkb_bool_ctrl_t = 4;
/// The `xkb::BoolCtrl::StickyKeys` enum variant.
///
/// This is a variant of [`xcb_xkb_bool_ctrl_t`].
pub const XCB_XKB_BOOL_CTRL_STICKY_KEYS: xcb_xkb_bool_ctrl_t = 8;
/// The `xkb::BoolCtrl::MouseKeys` enum variant.
///
/// This is a variant of [`xcb_xkb_bool_ctrl_t`].
pub const XCB_XKB_BOOL_CTRL_MOUSE_KEYS: xcb_xkb_bool_ctrl_t = 16;
/// The `xkb::BoolCtrl::MouseKeysAccel` enum variant.
///
/// This is a variant of [`xcb_xkb_bool_ctrl_t`].
pub const XCB_XKB_BOOL_CTRL_MOUSE_KEYS_ACCEL: xcb_xkb_bool_ctrl_t = 32;
/// The `xkb::BoolCtrl::AccessXKeys` enum variant.
///
/// This is a variant of [`xcb_xkb_bool_ctrl_t`].
pub const XCB_XKB_BOOL_CTRL_ACCESS_X_KEYS: xcb_xkb_bool_ctrl_t = 64;
/// The `xkb::BoolCtrl::AccessXTimeoutMask` enum variant.
///
/// This is a variant of [`xcb_xkb_bool_ctrl_t`].
pub const XCB_XKB_BOOL_CTRL_ACCESS_X_TIMEOUT_MASK: xcb_xkb_bool_ctrl_t = 128;
/// The `xkb::BoolCtrl::AccessXFeedbackMask` enum variant.
///
/// This is a variant of [`xcb_xkb_bool_ctrl_t`].
pub const XCB_XKB_BOOL_CTRL_ACCESS_X_FEEDBACK_MASK: xcb_xkb_bool_ctrl_t = 256;
/// The `xkb::BoolCtrl::AudibleBellMask` enum variant.
///
/// This is a variant of [`xcb_xkb_bool_ctrl_t`].
pub const XCB_XKB_BOOL_CTRL_AUDIBLE_BELL_MASK: xcb_xkb_bool_ctrl_t = 512;
/// The `xkb::BoolCtrl::Overlay1Mask` enum variant.
///
/// This is a variant of [`xcb_xkb_bool_ctrl_t`].
pub const XCB_XKB_BOOL_CTRL_OVERLAY_1_MASK: xcb_xkb_bool_ctrl_t = 1024;
/// The `xkb::BoolCtrl::Overlay2Mask` enum variant.
///
/// This is a variant of [`xcb_xkb_bool_ctrl_t`].
pub const XCB_XKB_BOOL_CTRL_OVERLAY_2_MASK: xcb_xkb_bool_ctrl_t = 2048;
/// The `xkb::BoolCtrl::IgnoreGroupLockMask` enum variant.
///
/// This is a variant of [`xcb_xkb_bool_ctrl_t`].
pub const XCB_XKB_BOOL_CTRL_IGNORE_GROUP_LOCK_MASK: xcb_xkb_bool_ctrl_t = 4096;

/// The `xkb::Control` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::Control::GroupsWrap`](XCB_XKB_CONTROL_GROUPS_WRAP)
/// - [`xkb::Control::InternalMods`](XCB_XKB_CONTROL_INTERNAL_MODS)
/// - [`xkb::Control::IgnoreLockMods`](XCB_XKB_CONTROL_IGNORE_LOCK_MODS)
/// - [`xkb::Control::PerKeyRepeat`](XCB_XKB_CONTROL_PER_KEY_REPEAT)
/// - [`xkb::Control::ControlsEnabled`](XCB_XKB_CONTROL_CONTROLS_ENABLED)
pub type xcb_xkb_control_t = u32;
/// The `xkb::Control::GroupsWrap` enum variant.
///
/// This is a variant of [`xcb_xkb_control_t`].
pub const XCB_XKB_CONTROL_GROUPS_WRAP: xcb_xkb_control_t = 134217728;
/// The `xkb::Control::InternalMods` enum variant.
///
/// This is a variant of [`xcb_xkb_control_t`].
pub const XCB_XKB_CONTROL_INTERNAL_MODS: xcb_xkb_control_t = 268435456;
/// The `xkb::Control::IgnoreLockMods` enum variant.
///
/// This is a variant of [`xcb_xkb_control_t`].
pub const XCB_XKB_CONTROL_IGNORE_LOCK_MODS: xcb_xkb_control_t = 536870912;
/// The `xkb::Control::PerKeyRepeat` enum variant.
///
/// This is a variant of [`xcb_xkb_control_t`].
pub const XCB_XKB_CONTROL_PER_KEY_REPEAT: xcb_xkb_control_t = 1073741824;
/// The `xkb::Control::ControlsEnabled` enum variant.
///
/// This is a variant of [`xcb_xkb_control_t`].
pub const XCB_XKB_CONTROL_CONTROLS_ENABLED: xcb_xkb_control_t = 2147483648;

/// The `xkb::AXOption` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::AXOption::SKPressFB`](XCB_XKB_AX_OPTION_SK_PRESS_FB)
/// - [`xkb::AXOption::SKAcceptFB`](XCB_XKB_AX_OPTION_SK_ACCEPT_FB)
/// - [`xkb::AXOption::FeatureFB`](XCB_XKB_AX_OPTION_FEATURE_FB)
/// - [`xkb::AXOption::SlowWarnFB`](XCB_XKB_AX_OPTION_SLOW_WARN_FB)
/// - [`xkb::AXOption::IndicatorFB`](XCB_XKB_AX_OPTION_INDICATOR_FB)
/// - [`xkb::AXOption::StickyKeysFB`](XCB_XKB_AX_OPTION_STICKY_KEYS_FB)
/// - [`xkb::AXOption::TwoKeys`](XCB_XKB_AX_OPTION_TWO_KEYS)
/// - [`xkb::AXOption::LatchToLock`](XCB_XKB_AX_OPTION_LATCH_TO_LOCK)
/// - [`xkb::AXOption::SKReleaseFB`](XCB_XKB_AX_OPTION_SK_RELEASE_FB)
/// - [`xkb::AXOption::SKRejectFB`](XCB_XKB_AX_OPTION_SK_REJECT_FB)
/// - [`xkb::AXOption::BKRejectFB`](XCB_XKB_AX_OPTION_BK_REJECT_FB)
/// - [`xkb::AXOption::DumbBell`](XCB_XKB_AX_OPTION_DUMB_BELL)
pub type xcb_xkb_ax_option_t = u32;
/// The `xkb::AXOption::SKPressFB` enum variant.
///
/// This is a variant of [`xcb_xkb_ax_option_t`].
pub const XCB_XKB_AX_OPTION_SK_PRESS_FB: xcb_xkb_ax_option_t = 1;
/// The `xkb::AXOption::SKAcceptFB` enum variant.
///
/// This is a variant of [`xcb_xkb_ax_option_t`].
pub const XCB_XKB_AX_OPTION_SK_ACCEPT_FB: xcb_xkb_ax_option_t = 2;
/// The `xkb::AXOption::FeatureFB` enum variant.
///
/// This is a variant of [`xcb_xkb_ax_option_t`].
pub const XCB_XKB_AX_OPTION_FEATURE_FB: xcb_xkb_ax_option_t = 4;
/// The `xkb::AXOption::SlowWarnFB` enum variant.
///
/// This is a variant of [`xcb_xkb_ax_option_t`].
pub const XCB_XKB_AX_OPTION_SLOW_WARN_FB: xcb_xkb_ax_option_t = 8;
/// The `xkb::AXOption::IndicatorFB` enum variant.
///
/// This is a variant of [`xcb_xkb_ax_option_t`].
pub const XCB_XKB_AX_OPTION_INDICATOR_FB: xcb_xkb_ax_option_t = 16;
/// The `xkb::AXOption::StickyKeysFB` enum variant.
///
/// This is a variant of [`xcb_xkb_ax_option_t`].
pub const XCB_XKB_AX_OPTION_STICKY_KEYS_FB: xcb_xkb_ax_option_t = 32;
/// The `xkb::AXOption::TwoKeys` enum variant.
///
/// This is a variant of [`xcb_xkb_ax_option_t`].
pub const XCB_XKB_AX_OPTION_TWO_KEYS: xcb_xkb_ax_option_t = 64;
/// The `xkb::AXOption::LatchToLock` enum variant.
///
/// This is a variant of [`xcb_xkb_ax_option_t`].
pub const XCB_XKB_AX_OPTION_LATCH_TO_LOCK: xcb_xkb_ax_option_t = 128;
/// The `xkb::AXOption::SKReleaseFB` enum variant.
///
/// This is a variant of [`xcb_xkb_ax_option_t`].
pub const XCB_XKB_AX_OPTION_SK_RELEASE_FB: xcb_xkb_ax_option_t = 256;
/// The `xkb::AXOption::SKRejectFB` enum variant.
///
/// This is a variant of [`xcb_xkb_ax_option_t`].
pub const XCB_XKB_AX_OPTION_SK_REJECT_FB: xcb_xkb_ax_option_t = 512;
/// The `xkb::AXOption::BKRejectFB` enum variant.
///
/// This is a variant of [`xcb_xkb_ax_option_t`].
pub const XCB_XKB_AX_OPTION_BK_REJECT_FB: xcb_xkb_ax_option_t = 1024;
/// The `xkb::AXOption::DumbBell` enum variant.
///
/// This is a variant of [`xcb_xkb_ax_option_t`].
pub const XCB_XKB_AX_OPTION_DUMB_BELL: xcb_xkb_ax_option_t = 2048;

/// The `xkb::DeviceSpec` type.
pub type xcb_xkb_device_spec_t = u16;

/// An iterator over `xkb::DeviceSpec` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_device_spec_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_device_spec_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_device_spec_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::LedClassResult` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::LedClassResult::KbdFeedbackClass`](XCB_XKB_LED_CLASS_RESULT_KBD_FEEDBACK_CLASS)
/// - [`xkb::LedClassResult::LedFeedbackClass`](XCB_XKB_LED_CLASS_RESULT_LED_FEEDBACK_CLASS)
pub type xcb_xkb_led_class_result_t = u32;
/// The `xkb::LedClassResult::KbdFeedbackClass` enum variant.
///
/// This is a variant of [`xcb_xkb_led_class_result_t`].
pub const XCB_XKB_LED_CLASS_RESULT_KBD_FEEDBACK_CLASS: xcb_xkb_led_class_result_t = 0;
/// The `xkb::LedClassResult::LedFeedbackClass` enum variant.
///
/// This is a variant of [`xcb_xkb_led_class_result_t`].
pub const XCB_XKB_LED_CLASS_RESULT_LED_FEEDBACK_CLASS: xcb_xkb_led_class_result_t = 4;

/// The `xkb::LedClass` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::LedClass::KbdFeedbackClass`](XCB_XKB_LED_CLASS_KBD_FEEDBACK_CLASS)
/// - [`xkb::LedClass::LedFeedbackClass`](XCB_XKB_LED_CLASS_LED_FEEDBACK_CLASS)
/// - [`xkb::LedClass::DfltXIClass`](XCB_XKB_LED_CLASS_DFLT_XI_CLASS)
/// - [`xkb::LedClass::AllXIClasses`](XCB_XKB_LED_CLASS_ALL_XI_CLASSES)
pub type xcb_xkb_led_class_t = u32;
/// The `xkb::LedClass::KbdFeedbackClass` enum variant.
///
/// This is a variant of [`xcb_xkb_led_class_t`].
pub const XCB_XKB_LED_CLASS_KBD_FEEDBACK_CLASS: xcb_xkb_led_class_t = 0;
/// The `xkb::LedClass::LedFeedbackClass` enum variant.
///
/// This is a variant of [`xcb_xkb_led_class_t`].
pub const XCB_XKB_LED_CLASS_LED_FEEDBACK_CLASS: xcb_xkb_led_class_t = 4;
/// The `xkb::LedClass::DfltXIClass` enum variant.
///
/// This is a variant of [`xcb_xkb_led_class_t`].
pub const XCB_XKB_LED_CLASS_DFLT_XI_CLASS: xcb_xkb_led_class_t = 768;
/// The `xkb::LedClass::AllXIClasses` enum variant.
///
/// This is a variant of [`xcb_xkb_led_class_t`].
pub const XCB_XKB_LED_CLASS_ALL_XI_CLASSES: xcb_xkb_led_class_t = 1280;

/// The `xkb::LedClassSpec` type.
pub type xcb_xkb_led_class_spec_t = u16;

/// An iterator over `xkb::LedClassSpec` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_led_class_spec_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_led_class_spec_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_led_class_spec_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::BellClassResult` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::BellClassResult::KbdFeedbackClass`](XCB_XKB_BELL_CLASS_RESULT_KBD_FEEDBACK_CLASS)
/// - [`xkb::BellClassResult::BellFeedbackClass`](XCB_XKB_BELL_CLASS_RESULT_BELL_FEEDBACK_CLASS)
pub type xcb_xkb_bell_class_result_t = u32;
/// The `xkb::BellClassResult::KbdFeedbackClass` enum variant.
///
/// This is a variant of [`xcb_xkb_bell_class_result_t`].
pub const XCB_XKB_BELL_CLASS_RESULT_KBD_FEEDBACK_CLASS: xcb_xkb_bell_class_result_t = 0;
/// The `xkb::BellClassResult::BellFeedbackClass` enum variant.
///
/// This is a variant of [`xcb_xkb_bell_class_result_t`].
pub const XCB_XKB_BELL_CLASS_RESULT_BELL_FEEDBACK_CLASS: xcb_xkb_bell_class_result_t = 5;

/// The `xkb::BellClass` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::BellClass::KbdFeedbackClass`](XCB_XKB_BELL_CLASS_KBD_FEEDBACK_CLASS)
/// - [`xkb::BellClass::BellFeedbackClass`](XCB_XKB_BELL_CLASS_BELL_FEEDBACK_CLASS)
/// - [`xkb::BellClass::DfltXIClass`](XCB_XKB_BELL_CLASS_DFLT_XI_CLASS)
pub type xcb_xkb_bell_class_t = u32;
/// The `xkb::BellClass::KbdFeedbackClass` enum variant.
///
/// This is a variant of [`xcb_xkb_bell_class_t`].
pub const XCB_XKB_BELL_CLASS_KBD_FEEDBACK_CLASS: xcb_xkb_bell_class_t = 0;
/// The `xkb::BellClass::BellFeedbackClass` enum variant.
///
/// This is a variant of [`xcb_xkb_bell_class_t`].
pub const XCB_XKB_BELL_CLASS_BELL_FEEDBACK_CLASS: xcb_xkb_bell_class_t = 5;
/// The `xkb::BellClass::DfltXIClass` enum variant.
///
/// This is a variant of [`xcb_xkb_bell_class_t`].
pub const XCB_XKB_BELL_CLASS_DFLT_XI_CLASS: xcb_xkb_bell_class_t = 768;

/// The `xkb::BellClassSpec` type.
pub type xcb_xkb_bell_class_spec_t = u16;

/// An iterator over `xkb::BellClassSpec` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_bell_class_spec_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_bell_class_spec_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_bell_class_spec_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::ID` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::ID::UseCoreKbd`](XCB_XKB_ID_USE_CORE_KBD)
/// - [`xkb::ID::UseCorePtr`](XCB_XKB_ID_USE_CORE_PTR)
/// - [`xkb::ID::DfltXIClass`](XCB_XKB_ID_DFLT_XI_CLASS)
/// - [`xkb::ID::DfltXIId`](XCB_XKB_ID_DFLT_XI_ID)
/// - [`xkb::ID::AllXIClass`](XCB_XKB_ID_ALL_XI_CLASS)
/// - [`xkb::ID::AllXIId`](XCB_XKB_ID_ALL_XI_ID)
/// - [`xkb::ID::XINone`](XCB_XKB_ID_XI_NONE)
pub type xcb_xkb_id_t = u32;
/// The `xkb::ID::UseCoreKbd` enum variant.
///
/// This is a variant of [`xcb_xkb_id_t`].
pub const XCB_XKB_ID_USE_CORE_KBD: xcb_xkb_id_t = 256;
/// The `xkb::ID::UseCorePtr` enum variant.
///
/// This is a variant of [`xcb_xkb_id_t`].
pub const XCB_XKB_ID_USE_CORE_PTR: xcb_xkb_id_t = 512;
/// The `xkb::ID::DfltXIClass` enum variant.
///
/// This is a variant of [`xcb_xkb_id_t`].
pub const XCB_XKB_ID_DFLT_XI_CLASS: xcb_xkb_id_t = 768;
/// The `xkb::ID::DfltXIId` enum variant.
///
/// This is a variant of [`xcb_xkb_id_t`].
pub const XCB_XKB_ID_DFLT_XI_ID: xcb_xkb_id_t = 1024;
/// The `xkb::ID::AllXIClass` enum variant.
///
/// This is a variant of [`xcb_xkb_id_t`].
pub const XCB_XKB_ID_ALL_XI_CLASS: xcb_xkb_id_t = 1280;
/// The `xkb::ID::AllXIId` enum variant.
///
/// This is a variant of [`xcb_xkb_id_t`].
pub const XCB_XKB_ID_ALL_XI_ID: xcb_xkb_id_t = 1536;
/// The `xkb::ID::XINone` enum variant.
///
/// This is a variant of [`xcb_xkb_id_t`].
pub const XCB_XKB_ID_XI_NONE: xcb_xkb_id_t = 65280;

/// The `xkb::IDSpec` type.
pub type xcb_xkb_id_spec_t = u16;

/// An iterator over `xkb::IDSpec` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_id_spec_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_id_spec_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_id_spec_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::Group` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::Group::1`](XCB_XKB_GROUP_1)
/// - [`xkb::Group::2`](XCB_XKB_GROUP_2)
/// - [`xkb::Group::3`](XCB_XKB_GROUP_3)
/// - [`xkb::Group::4`](XCB_XKB_GROUP_4)
pub type xcb_xkb_group_t = u32;
/// The `xkb::Group::1` enum variant.
///
/// This is a variant of [`xcb_xkb_group_t`].
pub const XCB_XKB_GROUP_1: xcb_xkb_group_t = 0;
/// The `xkb::Group::2` enum variant.
///
/// This is a variant of [`xcb_xkb_group_t`].
pub const XCB_XKB_GROUP_2: xcb_xkb_group_t = 1;
/// The `xkb::Group::3` enum variant.
///
/// This is a variant of [`xcb_xkb_group_t`].
pub const XCB_XKB_GROUP_3: xcb_xkb_group_t = 2;
/// The `xkb::Group::4` enum variant.
///
/// This is a variant of [`xcb_xkb_group_t`].
pub const XCB_XKB_GROUP_4: xcb_xkb_group_t = 3;

/// The `xkb::Groups` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::Groups::Any`](XCB_XKB_GROUPS_ANY)
/// - [`xkb::Groups::All`](XCB_XKB_GROUPS_ALL)
pub type xcb_xkb_groups_t = u32;
/// The `xkb::Groups::Any` enum variant.
///
/// This is a variant of [`xcb_xkb_groups_t`].
pub const XCB_XKB_GROUPS_ANY: xcb_xkb_groups_t = 254;
/// The `xkb::Groups::All` enum variant.
///
/// This is a variant of [`xcb_xkb_groups_t`].
pub const XCB_XKB_GROUPS_ALL: xcb_xkb_groups_t = 255;

/// The `xkb::SetOfGroup` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::SetOfGroup::Group1`](XCB_XKB_SET_OF_GROUP_GROUP_1)
/// - [`xkb::SetOfGroup::Group2`](XCB_XKB_SET_OF_GROUP_GROUP_2)
/// - [`xkb::SetOfGroup::Group3`](XCB_XKB_SET_OF_GROUP_GROUP_3)
/// - [`xkb::SetOfGroup::Group4`](XCB_XKB_SET_OF_GROUP_GROUP_4)
pub type xcb_xkb_set_of_group_t = u32;
/// The `xkb::SetOfGroup::Group1` enum variant.
///
/// This is a variant of [`xcb_xkb_set_of_group_t`].
pub const XCB_XKB_SET_OF_GROUP_GROUP_1: xcb_xkb_set_of_group_t = 1;
/// The `xkb::SetOfGroup::Group2` enum variant.
///
/// This is a variant of [`xcb_xkb_set_of_group_t`].
pub const XCB_XKB_SET_OF_GROUP_GROUP_2: xcb_xkb_set_of_group_t = 2;
/// The `xkb::SetOfGroup::Group3` enum variant.
///
/// This is a variant of [`xcb_xkb_set_of_group_t`].
pub const XCB_XKB_SET_OF_GROUP_GROUP_3: xcb_xkb_set_of_group_t = 4;
/// The `xkb::SetOfGroup::Group4` enum variant.
///
/// This is a variant of [`xcb_xkb_set_of_group_t`].
pub const XCB_XKB_SET_OF_GROUP_GROUP_4: xcb_xkb_set_of_group_t = 8;

/// The `xkb::SetOfGroups` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::SetOfGroups::Any`](XCB_XKB_SET_OF_GROUPS_ANY)
pub type xcb_xkb_set_of_groups_t = u32;
/// The `xkb::SetOfGroups::Any` enum variant.
///
/// This is a variant of [`xcb_xkb_set_of_groups_t`].
pub const XCB_XKB_SET_OF_GROUPS_ANY: xcb_xkb_set_of_groups_t = 128;

/// The `xkb::GroupsWrap` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::GroupsWrap::WrapIntoRange`](XCB_XKB_GROUPS_WRAP_WRAP_INTO_RANGE)
/// - [`xkb::GroupsWrap::ClampIntoRange`](XCB_XKB_GROUPS_WRAP_CLAMP_INTO_RANGE)
/// - [`xkb::GroupsWrap::RedirectIntoRange`](XCB_XKB_GROUPS_WRAP_REDIRECT_INTO_RANGE)
pub type xcb_xkb_groups_wrap_t = u32;
/// The `xkb::GroupsWrap::WrapIntoRange` enum variant.
///
/// This is a variant of [`xcb_xkb_groups_wrap_t`].
pub const XCB_XKB_GROUPS_WRAP_WRAP_INTO_RANGE: xcb_xkb_groups_wrap_t = 0;
/// The `xkb::GroupsWrap::ClampIntoRange` enum variant.
///
/// This is a variant of [`xcb_xkb_groups_wrap_t`].
pub const XCB_XKB_GROUPS_WRAP_CLAMP_INTO_RANGE: xcb_xkb_groups_wrap_t = 64;
/// The `xkb::GroupsWrap::RedirectIntoRange` enum variant.
///
/// This is a variant of [`xcb_xkb_groups_wrap_t`].
pub const XCB_XKB_GROUPS_WRAP_REDIRECT_INTO_RANGE: xcb_xkb_groups_wrap_t = 128;

/// The `xkb::VModsHigh` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::VModsHigh::15`](XCB_XKB_V_MODS_HIGH_15)
/// - [`xkb::VModsHigh::14`](XCB_XKB_V_MODS_HIGH_14)
/// - [`xkb::VModsHigh::13`](XCB_XKB_V_MODS_HIGH_13)
/// - [`xkb::VModsHigh::12`](XCB_XKB_V_MODS_HIGH_12)
/// - [`xkb::VModsHigh::11`](XCB_XKB_V_MODS_HIGH_11)
/// - [`xkb::VModsHigh::10`](XCB_XKB_V_MODS_HIGH_10)
/// - [`xkb::VModsHigh::9`](XCB_XKB_V_MODS_HIGH_9)
/// - [`xkb::VModsHigh::8`](XCB_XKB_V_MODS_HIGH_8)
pub type xcb_xkb_v_mods_high_t = u32;
/// The `xkb::VModsHigh::15` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mods_high_t`].
pub const XCB_XKB_V_MODS_HIGH_15: xcb_xkb_v_mods_high_t = 128;
/// The `xkb::VModsHigh::14` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mods_high_t`].
pub const XCB_XKB_V_MODS_HIGH_14: xcb_xkb_v_mods_high_t = 64;
/// The `xkb::VModsHigh::13` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mods_high_t`].
pub const XCB_XKB_V_MODS_HIGH_13: xcb_xkb_v_mods_high_t = 32;
/// The `xkb::VModsHigh::12` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mods_high_t`].
pub const XCB_XKB_V_MODS_HIGH_12: xcb_xkb_v_mods_high_t = 16;
/// The `xkb::VModsHigh::11` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mods_high_t`].
pub const XCB_XKB_V_MODS_HIGH_11: xcb_xkb_v_mods_high_t = 8;
/// The `xkb::VModsHigh::10` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mods_high_t`].
pub const XCB_XKB_V_MODS_HIGH_10: xcb_xkb_v_mods_high_t = 4;
/// The `xkb::VModsHigh::9` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mods_high_t`].
pub const XCB_XKB_V_MODS_HIGH_9: xcb_xkb_v_mods_high_t = 2;
/// The `xkb::VModsHigh::8` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mods_high_t`].
pub const XCB_XKB_V_MODS_HIGH_8: xcb_xkb_v_mods_high_t = 1;

/// The `xkb::VModsLow` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::VModsLow::7`](XCB_XKB_V_MODS_LOW_7)
/// - [`xkb::VModsLow::6`](XCB_XKB_V_MODS_LOW_6)
/// - [`xkb::VModsLow::5`](XCB_XKB_V_MODS_LOW_5)
/// - [`xkb::VModsLow::4`](XCB_XKB_V_MODS_LOW_4)
/// - [`xkb::VModsLow::3`](XCB_XKB_V_MODS_LOW_3)
/// - [`xkb::VModsLow::2`](XCB_XKB_V_MODS_LOW_2)
/// - [`xkb::VModsLow::1`](XCB_XKB_V_MODS_LOW_1)
/// - [`xkb::VModsLow::0`](XCB_XKB_V_MODS_LOW_0)
pub type xcb_xkb_v_mods_low_t = u32;
/// The `xkb::VModsLow::7` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mods_low_t`].
pub const XCB_XKB_V_MODS_LOW_7: xcb_xkb_v_mods_low_t = 128;
/// The `xkb::VModsLow::6` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mods_low_t`].
pub const XCB_XKB_V_MODS_LOW_6: xcb_xkb_v_mods_low_t = 64;
/// The `xkb::VModsLow::5` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mods_low_t`].
pub const XCB_XKB_V_MODS_LOW_5: xcb_xkb_v_mods_low_t = 32;
/// The `xkb::VModsLow::4` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mods_low_t`].
pub const XCB_XKB_V_MODS_LOW_4: xcb_xkb_v_mods_low_t = 16;
/// The `xkb::VModsLow::3` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mods_low_t`].
pub const XCB_XKB_V_MODS_LOW_3: xcb_xkb_v_mods_low_t = 8;
/// The `xkb::VModsLow::2` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mods_low_t`].
pub const XCB_XKB_V_MODS_LOW_2: xcb_xkb_v_mods_low_t = 4;
/// The `xkb::VModsLow::1` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mods_low_t`].
pub const XCB_XKB_V_MODS_LOW_1: xcb_xkb_v_mods_low_t = 2;
/// The `xkb::VModsLow::0` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mods_low_t`].
pub const XCB_XKB_V_MODS_LOW_0: xcb_xkb_v_mods_low_t = 1;

/// The `xkb::VMod` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::VMod::15`](XCB_XKB_V_MOD_15)
/// - [`xkb::VMod::14`](XCB_XKB_V_MOD_14)
/// - [`xkb::VMod::13`](XCB_XKB_V_MOD_13)
/// - [`xkb::VMod::12`](XCB_XKB_V_MOD_12)
/// - [`xkb::VMod::11`](XCB_XKB_V_MOD_11)
/// - [`xkb::VMod::10`](XCB_XKB_V_MOD_10)
/// - [`xkb::VMod::9`](XCB_XKB_V_MOD_9)
/// - [`xkb::VMod::8`](XCB_XKB_V_MOD_8)
/// - [`xkb::VMod::7`](XCB_XKB_V_MOD_7)
/// - [`xkb::VMod::6`](XCB_XKB_V_MOD_6)
/// - [`xkb::VMod::5`](XCB_XKB_V_MOD_5)
/// - [`xkb::VMod::4`](XCB_XKB_V_MOD_4)
/// - [`xkb::VMod::3`](XCB_XKB_V_MOD_3)
/// - [`xkb::VMod::2`](XCB_XKB_V_MOD_2)
/// - [`xkb::VMod::1`](XCB_XKB_V_MOD_1)
/// - [`xkb::VMod::0`](XCB_XKB_V_MOD_0)
pub type xcb_xkb_v_mod_t = u32;
/// The `xkb::VMod::15` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mod_t`].
pub const XCB_XKB_V_MOD_15: xcb_xkb_v_mod_t = 32768;
/// The `xkb::VMod::14` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mod_t`].
pub const XCB_XKB_V_MOD_14: xcb_xkb_v_mod_t = 16384;
/// The `xkb::VMod::13` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mod_t`].
pub const XCB_XKB_V_MOD_13: xcb_xkb_v_mod_t = 8192;
/// The `xkb::VMod::12` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mod_t`].
pub const XCB_XKB_V_MOD_12: xcb_xkb_v_mod_t = 4096;
/// The `xkb::VMod::11` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mod_t`].
pub const XCB_XKB_V_MOD_11: xcb_xkb_v_mod_t = 2048;
/// The `xkb::VMod::10` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mod_t`].
pub const XCB_XKB_V_MOD_10: xcb_xkb_v_mod_t = 1024;
/// The `xkb::VMod::9` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mod_t`].
pub const XCB_XKB_V_MOD_9: xcb_xkb_v_mod_t = 512;
/// The `xkb::VMod::8` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mod_t`].
pub const XCB_XKB_V_MOD_8: xcb_xkb_v_mod_t = 256;
/// The `xkb::VMod::7` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mod_t`].
pub const XCB_XKB_V_MOD_7: xcb_xkb_v_mod_t = 128;
/// The `xkb::VMod::6` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mod_t`].
pub const XCB_XKB_V_MOD_6: xcb_xkb_v_mod_t = 64;
/// The `xkb::VMod::5` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mod_t`].
pub const XCB_XKB_V_MOD_5: xcb_xkb_v_mod_t = 32;
/// The `xkb::VMod::4` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mod_t`].
pub const XCB_XKB_V_MOD_4: xcb_xkb_v_mod_t = 16;
/// The `xkb::VMod::3` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mod_t`].
pub const XCB_XKB_V_MOD_3: xcb_xkb_v_mod_t = 8;
/// The `xkb::VMod::2` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mod_t`].
pub const XCB_XKB_V_MOD_2: xcb_xkb_v_mod_t = 4;
/// The `xkb::VMod::1` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mod_t`].
pub const XCB_XKB_V_MOD_1: xcb_xkb_v_mod_t = 2;
/// The `xkb::VMod::0` enum variant.
///
/// This is a variant of [`xcb_xkb_v_mod_t`].
pub const XCB_XKB_V_MOD_0: xcb_xkb_v_mod_t = 1;

/// The `xkb::Explicit` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::Explicit::VModMap`](XCB_XKB_EXPLICIT_V_MOD_MAP)
/// - [`xkb::Explicit::Behavior`](XCB_XKB_EXPLICIT_BEHAVIOR)
/// - [`xkb::Explicit::AutoRepeat`](XCB_XKB_EXPLICIT_AUTO_REPEAT)
/// - [`xkb::Explicit::Interpret`](XCB_XKB_EXPLICIT_INTERPRET)
/// - [`xkb::Explicit::KeyType4`](XCB_XKB_EXPLICIT_KEY_TYPE_4)
/// - [`xkb::Explicit::KeyType3`](XCB_XKB_EXPLICIT_KEY_TYPE_3)
/// - [`xkb::Explicit::KeyType2`](XCB_XKB_EXPLICIT_KEY_TYPE_2)
/// - [`xkb::Explicit::KeyType1`](XCB_XKB_EXPLICIT_KEY_TYPE_1)
pub type xcb_xkb_explicit_t = u32;
/// The `xkb::Explicit::VModMap` enum variant.
///
/// This is a variant of [`xcb_xkb_explicit_t`].
pub const XCB_XKB_EXPLICIT_V_MOD_MAP: xcb_xkb_explicit_t = 128;
/// The `xkb::Explicit::Behavior` enum variant.
///
/// This is a variant of [`xcb_xkb_explicit_t`].
pub const XCB_XKB_EXPLICIT_BEHAVIOR: xcb_xkb_explicit_t = 64;
/// The `xkb::Explicit::AutoRepeat` enum variant.
///
/// This is a variant of [`xcb_xkb_explicit_t`].
pub const XCB_XKB_EXPLICIT_AUTO_REPEAT: xcb_xkb_explicit_t = 32;
/// The `xkb::Explicit::Interpret` enum variant.
///
/// This is a variant of [`xcb_xkb_explicit_t`].
pub const XCB_XKB_EXPLICIT_INTERPRET: xcb_xkb_explicit_t = 16;
/// The `xkb::Explicit::KeyType4` enum variant.
///
/// This is a variant of [`xcb_xkb_explicit_t`].
pub const XCB_XKB_EXPLICIT_KEY_TYPE_4: xcb_xkb_explicit_t = 8;
/// The `xkb::Explicit::KeyType3` enum variant.
///
/// This is a variant of [`xcb_xkb_explicit_t`].
pub const XCB_XKB_EXPLICIT_KEY_TYPE_3: xcb_xkb_explicit_t = 4;
/// The `xkb::Explicit::KeyType2` enum variant.
///
/// This is a variant of [`xcb_xkb_explicit_t`].
pub const XCB_XKB_EXPLICIT_KEY_TYPE_2: xcb_xkb_explicit_t = 2;
/// The `xkb::Explicit::KeyType1` enum variant.
///
/// This is a variant of [`xcb_xkb_explicit_t`].
pub const XCB_XKB_EXPLICIT_KEY_TYPE_1: xcb_xkb_explicit_t = 1;

/// The `xkb::SymInterpretMatch` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::SymInterpretMatch::NoneOf`](XCB_XKB_SYM_INTERPRET_MATCH_NONE_OF)
/// - [`xkb::SymInterpretMatch::AnyOfOrNone`](XCB_XKB_SYM_INTERPRET_MATCH_ANY_OF_OR_NONE)
/// - [`xkb::SymInterpretMatch::AnyOf`](XCB_XKB_SYM_INTERPRET_MATCH_ANY_OF)
/// - [`xkb::SymInterpretMatch::AllOf`](XCB_XKB_SYM_INTERPRET_MATCH_ALL_OF)
/// - [`xkb::SymInterpretMatch::Exactly`](XCB_XKB_SYM_INTERPRET_MATCH_EXACTLY)
pub type xcb_xkb_sym_interpret_match_t = u32;
/// The `xkb::SymInterpretMatch::NoneOf` enum variant.
///
/// This is a variant of [`xcb_xkb_sym_interpret_match_t`].
pub const XCB_XKB_SYM_INTERPRET_MATCH_NONE_OF: xcb_xkb_sym_interpret_match_t = 0;
/// The `xkb::SymInterpretMatch::AnyOfOrNone` enum variant.
///
/// This is a variant of [`xcb_xkb_sym_interpret_match_t`].
pub const XCB_XKB_SYM_INTERPRET_MATCH_ANY_OF_OR_NONE: xcb_xkb_sym_interpret_match_t = 1;
/// The `xkb::SymInterpretMatch::AnyOf` enum variant.
///
/// This is a variant of [`xcb_xkb_sym_interpret_match_t`].
pub const XCB_XKB_SYM_INTERPRET_MATCH_ANY_OF: xcb_xkb_sym_interpret_match_t = 2;
/// The `xkb::SymInterpretMatch::AllOf` enum variant.
///
/// This is a variant of [`xcb_xkb_sym_interpret_match_t`].
pub const XCB_XKB_SYM_INTERPRET_MATCH_ALL_OF: xcb_xkb_sym_interpret_match_t = 3;
/// The `xkb::SymInterpretMatch::Exactly` enum variant.
///
/// This is a variant of [`xcb_xkb_sym_interpret_match_t`].
pub const XCB_XKB_SYM_INTERPRET_MATCH_EXACTLY: xcb_xkb_sym_interpret_match_t = 4;

/// The `xkb::SymInterpMatch` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::SymInterpMatch::LevelOneOnly`](XCB_XKB_SYM_INTERP_MATCH_LEVEL_ONE_ONLY)
/// - [`xkb::SymInterpMatch::OpMask`](XCB_XKB_SYM_INTERP_MATCH_OP_MASK)
pub type xcb_xkb_sym_interp_match_t = u32;
/// The `xkb::SymInterpMatch::LevelOneOnly` enum variant.
///
/// This is a variant of [`xcb_xkb_sym_interp_match_t`].
pub const XCB_XKB_SYM_INTERP_MATCH_LEVEL_ONE_ONLY: xcb_xkb_sym_interp_match_t = 128;
/// The `xkb::SymInterpMatch::OpMask` enum variant.
///
/// This is a variant of [`xcb_xkb_sym_interp_match_t`].
pub const XCB_XKB_SYM_INTERP_MATCH_OP_MASK: xcb_xkb_sym_interp_match_t = 127;

/// The `xkb::IMFlag` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::IMFlag::NoExplicit`](XCB_XKB_IM_FLAG_NO_EXPLICIT)
/// - [`xkb::IMFlag::NoAutomatic`](XCB_XKB_IM_FLAG_NO_AUTOMATIC)
/// - [`xkb::IMFlag::LEDDrivesKB`](XCB_XKB_IM_FLAG_LED_DRIVES_KB)
pub type xcb_xkb_im_flag_t = u32;
/// The `xkb::IMFlag::NoExplicit` enum variant.
///
/// This is a variant of [`xcb_xkb_im_flag_t`].
pub const XCB_XKB_IM_FLAG_NO_EXPLICIT: xcb_xkb_im_flag_t = 128;
/// The `xkb::IMFlag::NoAutomatic` enum variant.
///
/// This is a variant of [`xcb_xkb_im_flag_t`].
pub const XCB_XKB_IM_FLAG_NO_AUTOMATIC: xcb_xkb_im_flag_t = 64;
/// The `xkb::IMFlag::LEDDrivesKB` enum variant.
///
/// This is a variant of [`xcb_xkb_im_flag_t`].
pub const XCB_XKB_IM_FLAG_LED_DRIVES_KB: xcb_xkb_im_flag_t = 32;

/// The `xkb::IMModsWhich` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::IMModsWhich::UseCompat`](XCB_XKB_IM_MODS_WHICH_USE_COMPAT)
/// - [`xkb::IMModsWhich::UseEffective`](XCB_XKB_IM_MODS_WHICH_USE_EFFECTIVE)
/// - [`xkb::IMModsWhich::UseLocked`](XCB_XKB_IM_MODS_WHICH_USE_LOCKED)
/// - [`xkb::IMModsWhich::UseLatched`](XCB_XKB_IM_MODS_WHICH_USE_LATCHED)
/// - [`xkb::IMModsWhich::UseBase`](XCB_XKB_IM_MODS_WHICH_USE_BASE)
pub type xcb_xkb_im_mods_which_t = u32;
/// The `xkb::IMModsWhich::UseCompat` enum variant.
///
/// This is a variant of [`xcb_xkb_im_mods_which_t`].
pub const XCB_XKB_IM_MODS_WHICH_USE_COMPAT: xcb_xkb_im_mods_which_t = 16;
/// The `xkb::IMModsWhich::UseEffective` enum variant.
///
/// This is a variant of [`xcb_xkb_im_mods_which_t`].
pub const XCB_XKB_IM_MODS_WHICH_USE_EFFECTIVE: xcb_xkb_im_mods_which_t = 8;
/// The `xkb::IMModsWhich::UseLocked` enum variant.
///
/// This is a variant of [`xcb_xkb_im_mods_which_t`].
pub const XCB_XKB_IM_MODS_WHICH_USE_LOCKED: xcb_xkb_im_mods_which_t = 4;
/// The `xkb::IMModsWhich::UseLatched` enum variant.
///
/// This is a variant of [`xcb_xkb_im_mods_which_t`].
pub const XCB_XKB_IM_MODS_WHICH_USE_LATCHED: xcb_xkb_im_mods_which_t = 2;
/// The `xkb::IMModsWhich::UseBase` enum variant.
///
/// This is a variant of [`xcb_xkb_im_mods_which_t`].
pub const XCB_XKB_IM_MODS_WHICH_USE_BASE: xcb_xkb_im_mods_which_t = 1;

/// The `xkb::IMGroupsWhich` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::IMGroupsWhich::UseCompat`](XCB_XKB_IM_GROUPS_WHICH_USE_COMPAT)
/// - [`xkb::IMGroupsWhich::UseEffective`](XCB_XKB_IM_GROUPS_WHICH_USE_EFFECTIVE)
/// - [`xkb::IMGroupsWhich::UseLocked`](XCB_XKB_IM_GROUPS_WHICH_USE_LOCKED)
/// - [`xkb::IMGroupsWhich::UseLatched`](XCB_XKB_IM_GROUPS_WHICH_USE_LATCHED)
/// - [`xkb::IMGroupsWhich::UseBase`](XCB_XKB_IM_GROUPS_WHICH_USE_BASE)
pub type xcb_xkb_im_groups_which_t = u32;
/// The `xkb::IMGroupsWhich::UseCompat` enum variant.
///
/// This is a variant of [`xcb_xkb_im_groups_which_t`].
pub const XCB_XKB_IM_GROUPS_WHICH_USE_COMPAT: xcb_xkb_im_groups_which_t = 16;
/// The `xkb::IMGroupsWhich::UseEffective` enum variant.
///
/// This is a variant of [`xcb_xkb_im_groups_which_t`].
pub const XCB_XKB_IM_GROUPS_WHICH_USE_EFFECTIVE: xcb_xkb_im_groups_which_t = 8;
/// The `xkb::IMGroupsWhich::UseLocked` enum variant.
///
/// This is a variant of [`xcb_xkb_im_groups_which_t`].
pub const XCB_XKB_IM_GROUPS_WHICH_USE_LOCKED: xcb_xkb_im_groups_which_t = 4;
/// The `xkb::IMGroupsWhich::UseLatched` enum variant.
///
/// This is a variant of [`xcb_xkb_im_groups_which_t`].
pub const XCB_XKB_IM_GROUPS_WHICH_USE_LATCHED: xcb_xkb_im_groups_which_t = 2;
/// The `xkb::IMGroupsWhich::UseBase` enum variant.
///
/// This is a variant of [`xcb_xkb_im_groups_which_t`].
pub const XCB_XKB_IM_GROUPS_WHICH_USE_BASE: xcb_xkb_im_groups_which_t = 1;

/// The `xkb::IndicatorMap` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_indicator_map_t {
    pub flags: u8,
    pub which_groups: u8,
    pub groups: u8,
    pub which_mods: u8,
    pub mods: u8,
    pub real_mods: u8,
    pub vmods: u16,
    pub ctrls: u32,
}

impl Default for xcb_xkb_indicator_map_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::IndicatorMap` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_indicator_map_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_indicator_map_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_indicator_map_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::CMDetail` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::CMDetail::SymInterp`](XCB_XKB_CM_DETAIL_SYM_INTERP)
/// - [`xkb::CMDetail::GroupCompat`](XCB_XKB_CM_DETAIL_GROUP_COMPAT)
pub type xcb_xkb_cm_detail_t = u32;
/// The `xkb::CMDetail::SymInterp` enum variant.
///
/// This is a variant of [`xcb_xkb_cm_detail_t`].
pub const XCB_XKB_CM_DETAIL_SYM_INTERP: xcb_xkb_cm_detail_t = 1;
/// The `xkb::CMDetail::GroupCompat` enum variant.
///
/// This is a variant of [`xcb_xkb_cm_detail_t`].
pub const XCB_XKB_CM_DETAIL_GROUP_COMPAT: xcb_xkb_cm_detail_t = 2;

/// The `xkb::NameDetail` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::NameDetail::Keycodes`](XCB_XKB_NAME_DETAIL_KEYCODES)
/// - [`xkb::NameDetail::Geometry`](XCB_XKB_NAME_DETAIL_GEOMETRY)
/// - [`xkb::NameDetail::Symbols`](XCB_XKB_NAME_DETAIL_SYMBOLS)
/// - [`xkb::NameDetail::PhysSymbols`](XCB_XKB_NAME_DETAIL_PHYS_SYMBOLS)
/// - [`xkb::NameDetail::Types`](XCB_XKB_NAME_DETAIL_TYPES)
/// - [`xkb::NameDetail::Compat`](XCB_XKB_NAME_DETAIL_COMPAT)
/// - [`xkb::NameDetail::KeyTypeNames`](XCB_XKB_NAME_DETAIL_KEY_TYPE_NAMES)
/// - [`xkb::NameDetail::KTLevelNames`](XCB_XKB_NAME_DETAIL_KT_LEVEL_NAMES)
/// - [`xkb::NameDetail::IndicatorNames`](XCB_XKB_NAME_DETAIL_INDICATOR_NAMES)
/// - [`xkb::NameDetail::KeyNames`](XCB_XKB_NAME_DETAIL_KEY_NAMES)
/// - [`xkb::NameDetail::KeyAliases`](XCB_XKB_NAME_DETAIL_KEY_ALIASES)
/// - [`xkb::NameDetail::VirtualModNames`](XCB_XKB_NAME_DETAIL_VIRTUAL_MOD_NAMES)
/// - [`xkb::NameDetail::GroupNames`](XCB_XKB_NAME_DETAIL_GROUP_NAMES)
/// - [`xkb::NameDetail::RGNames`](XCB_XKB_NAME_DETAIL_RG_NAMES)
pub type xcb_xkb_name_detail_t = u32;
/// The `xkb::NameDetail::Keycodes` enum variant.
///
/// This is a variant of [`xcb_xkb_name_detail_t`].
pub const XCB_XKB_NAME_DETAIL_KEYCODES: xcb_xkb_name_detail_t = 1;
/// The `xkb::NameDetail::Geometry` enum variant.
///
/// This is a variant of [`xcb_xkb_name_detail_t`].
pub const XCB_XKB_NAME_DETAIL_GEOMETRY: xcb_xkb_name_detail_t = 2;
/// The `xkb::NameDetail::Symbols` enum variant.
///
/// This is a variant of [`xcb_xkb_name_detail_t`].
pub const XCB_XKB_NAME_DETAIL_SYMBOLS: xcb_xkb_name_detail_t = 4;
/// The `xkb::NameDetail::PhysSymbols` enum variant.
///
/// This is a variant of [`xcb_xkb_name_detail_t`].
pub const XCB_XKB_NAME_DETAIL_PHYS_SYMBOLS: xcb_xkb_name_detail_t = 8;
/// The `xkb::NameDetail::Types` enum variant.
///
/// This is a variant of [`xcb_xkb_name_detail_t`].
pub const XCB_XKB_NAME_DETAIL_TYPES: xcb_xkb_name_detail_t = 16;
/// The `xkb::NameDetail::Compat` enum variant.
///
/// This is a variant of [`xcb_xkb_name_detail_t`].
pub const XCB_XKB_NAME_DETAIL_COMPAT: xcb_xkb_name_detail_t = 32;
/// The `xkb::NameDetail::KeyTypeNames` enum variant.
///
/// This is a variant of [`xcb_xkb_name_detail_t`].
pub const XCB_XKB_NAME_DETAIL_KEY_TYPE_NAMES: xcb_xkb_name_detail_t = 64;
/// The `xkb::NameDetail::KTLevelNames` enum variant.
///
/// This is a variant of [`xcb_xkb_name_detail_t`].
pub const XCB_XKB_NAME_DETAIL_KT_LEVEL_NAMES: xcb_xkb_name_detail_t = 128;
/// The `xkb::NameDetail::IndicatorNames` enum variant.
///
/// This is a variant of [`xcb_xkb_name_detail_t`].
pub const XCB_XKB_NAME_DETAIL_INDICATOR_NAMES: xcb_xkb_name_detail_t = 256;
/// The `xkb::NameDetail::KeyNames` enum variant.
///
/// This is a variant of [`xcb_xkb_name_detail_t`].
pub const XCB_XKB_NAME_DETAIL_KEY_NAMES: xcb_xkb_name_detail_t = 512;
/// The `xkb::NameDetail::KeyAliases` enum variant.
///
/// This is a variant of [`xcb_xkb_name_detail_t`].
pub const XCB_XKB_NAME_DETAIL_KEY_ALIASES: xcb_xkb_name_detail_t = 1024;
/// The `xkb::NameDetail::VirtualModNames` enum variant.
///
/// This is a variant of [`xcb_xkb_name_detail_t`].
pub const XCB_XKB_NAME_DETAIL_VIRTUAL_MOD_NAMES: xcb_xkb_name_detail_t = 2048;
/// The `xkb::NameDetail::GroupNames` enum variant.
///
/// This is a variant of [`xcb_xkb_name_detail_t`].
pub const XCB_XKB_NAME_DETAIL_GROUP_NAMES: xcb_xkb_name_detail_t = 4096;
/// The `xkb::NameDetail::RGNames` enum variant.
///
/// This is a variant of [`xcb_xkb_name_detail_t`].
pub const XCB_XKB_NAME_DETAIL_RG_NAMES: xcb_xkb_name_detail_t = 8192;

/// The `xkb::GBNDetail` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::GBNDetail::Types`](XCB_XKB_GBN_DETAIL_TYPES)
/// - [`xkb::GBNDetail::CompatMap`](XCB_XKB_GBN_DETAIL_COMPAT_MAP)
/// - [`xkb::GBNDetail::ClientSymbols`](XCB_XKB_GBN_DETAIL_CLIENT_SYMBOLS)
/// - [`xkb::GBNDetail::ServerSymbols`](XCB_XKB_GBN_DETAIL_SERVER_SYMBOLS)
/// - [`xkb::GBNDetail::IndicatorMaps`](XCB_XKB_GBN_DETAIL_INDICATOR_MAPS)
/// - [`xkb::GBNDetail::KeyNames`](XCB_XKB_GBN_DETAIL_KEY_NAMES)
/// - [`xkb::GBNDetail::Geometry`](XCB_XKB_GBN_DETAIL_GEOMETRY)
/// - [`xkb::GBNDetail::OtherNames`](XCB_XKB_GBN_DETAIL_OTHER_NAMES)
pub type xcb_xkb_gbn_detail_t = u32;
/// The `xkb::GBNDetail::Types` enum variant.
///
/// This is a variant of [`xcb_xkb_gbn_detail_t`].
pub const XCB_XKB_GBN_DETAIL_TYPES: xcb_xkb_gbn_detail_t = 1;
/// The `xkb::GBNDetail::CompatMap` enum variant.
///
/// This is a variant of [`xcb_xkb_gbn_detail_t`].
pub const XCB_XKB_GBN_DETAIL_COMPAT_MAP: xcb_xkb_gbn_detail_t = 2;
/// The `xkb::GBNDetail::ClientSymbols` enum variant.
///
/// This is a variant of [`xcb_xkb_gbn_detail_t`].
pub const XCB_XKB_GBN_DETAIL_CLIENT_SYMBOLS: xcb_xkb_gbn_detail_t = 4;
/// The `xkb::GBNDetail::ServerSymbols` enum variant.
///
/// This is a variant of [`xcb_xkb_gbn_detail_t`].
pub const XCB_XKB_GBN_DETAIL_SERVER_SYMBOLS: xcb_xkb_gbn_detail_t = 8;
/// The `xkb::GBNDetail::IndicatorMaps` enum variant.
///
/// This is a variant of [`xcb_xkb_gbn_detail_t`].
pub const XCB_XKB_GBN_DETAIL_INDICATOR_MAPS: xcb_xkb_gbn_detail_t = 16;
/// The `xkb::GBNDetail::KeyNames` enum variant.
///
/// This is a variant of [`xcb_xkb_gbn_detail_t`].
pub const XCB_XKB_GBN_DETAIL_KEY_NAMES: xcb_xkb_gbn_detail_t = 32;
/// The `xkb::GBNDetail::Geometry` enum variant.
///
/// This is a variant of [`xcb_xkb_gbn_detail_t`].
pub const XCB_XKB_GBN_DETAIL_GEOMETRY: xcb_xkb_gbn_detail_t = 64;
/// The `xkb::GBNDetail::OtherNames` enum variant.
///
/// This is a variant of [`xcb_xkb_gbn_detail_t`].
pub const XCB_XKB_GBN_DETAIL_OTHER_NAMES: xcb_xkb_gbn_detail_t = 128;

/// The `xkb::XIFeature` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::XIFeature::Keyboards`](XCB_XKB_XI_FEATURE_KEYBOARDS)
/// - [`xkb::XIFeature::ButtonActions`](XCB_XKB_XI_FEATURE_BUTTON_ACTIONS)
/// - [`xkb::XIFeature::IndicatorNames`](XCB_XKB_XI_FEATURE_INDICATOR_NAMES)
/// - [`xkb::XIFeature::IndicatorMaps`](XCB_XKB_XI_FEATURE_INDICATOR_MAPS)
/// - [`xkb::XIFeature::IndicatorState`](XCB_XKB_XI_FEATURE_INDICATOR_STATE)
pub type xcb_xkb_xi_feature_t = u32;
/// The `xkb::XIFeature::Keyboards` enum variant.
///
/// This is a variant of [`xcb_xkb_xi_feature_t`].
pub const XCB_XKB_XI_FEATURE_KEYBOARDS: xcb_xkb_xi_feature_t = 1;
/// The `xkb::XIFeature::ButtonActions` enum variant.
///
/// This is a variant of [`xcb_xkb_xi_feature_t`].
pub const XCB_XKB_XI_FEATURE_BUTTON_ACTIONS: xcb_xkb_xi_feature_t = 2;
/// The `xkb::XIFeature::IndicatorNames` enum variant.
///
/// This is a variant of [`xcb_xkb_xi_feature_t`].
pub const XCB_XKB_XI_FEATURE_INDICATOR_NAMES: xcb_xkb_xi_feature_t = 4;
/// The `xkb::XIFeature::IndicatorMaps` enum variant.
///
/// This is a variant of [`xcb_xkb_xi_feature_t`].
pub const XCB_XKB_XI_FEATURE_INDICATOR_MAPS: xcb_xkb_xi_feature_t = 8;
/// The `xkb::XIFeature::IndicatorState` enum variant.
///
/// This is a variant of [`xcb_xkb_xi_feature_t`].
pub const XCB_XKB_XI_FEATURE_INDICATOR_STATE: xcb_xkb_xi_feature_t = 16;

/// The `xkb::PerClientFlag` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::PerClientFlag::DetectableAutoRepeat`](XCB_XKB_PER_CLIENT_FLAG_DETECTABLE_AUTO_REPEAT)
/// - [`xkb::PerClientFlag::GrabsUseXKBState`](XCB_XKB_PER_CLIENT_FLAG_GRABS_USE_XKB_STATE)
/// - [`xkb::PerClientFlag::AutoResetControls`](XCB_XKB_PER_CLIENT_FLAG_AUTO_RESET_CONTROLS)
/// - [`xkb::PerClientFlag::LookupStateWhenGrabbed`](XCB_XKB_PER_CLIENT_FLAG_LOOKUP_STATE_WHEN_GRABBED)
/// - [`xkb::PerClientFlag::SendEventUsesXKBState`](XCB_XKB_PER_CLIENT_FLAG_SEND_EVENT_USES_XKB_STATE)
pub type xcb_xkb_per_client_flag_t = u32;
/// The `xkb::PerClientFlag::DetectableAutoRepeat` enum variant.
///
/// This is a variant of [`xcb_xkb_per_client_flag_t`].
pub const XCB_XKB_PER_CLIENT_FLAG_DETECTABLE_AUTO_REPEAT: xcb_xkb_per_client_flag_t = 1;
/// The `xkb::PerClientFlag::GrabsUseXKBState` enum variant.
///
/// This is a variant of [`xcb_xkb_per_client_flag_t`].
pub const XCB_XKB_PER_CLIENT_FLAG_GRABS_USE_XKB_STATE: xcb_xkb_per_client_flag_t = 2;
/// The `xkb::PerClientFlag::AutoResetControls` enum variant.
///
/// This is a variant of [`xcb_xkb_per_client_flag_t`].
pub const XCB_XKB_PER_CLIENT_FLAG_AUTO_RESET_CONTROLS: xcb_xkb_per_client_flag_t = 4;
/// The `xkb::PerClientFlag::LookupStateWhenGrabbed` enum variant.
///
/// This is a variant of [`xcb_xkb_per_client_flag_t`].
pub const XCB_XKB_PER_CLIENT_FLAG_LOOKUP_STATE_WHEN_GRABBED: xcb_xkb_per_client_flag_t = 8;
/// The `xkb::PerClientFlag::SendEventUsesXKBState` enum variant.
///
/// This is a variant of [`xcb_xkb_per_client_flag_t`].
pub const XCB_XKB_PER_CLIENT_FLAG_SEND_EVENT_USES_XKB_STATE: xcb_xkb_per_client_flag_t = 16;

/// The `xkb::ModDef` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_mod_def_t {
    pub mask: u8,
    pub real_mods: u8,
    pub vmods: u16,
}

impl Default for xcb_xkb_mod_def_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::ModDef` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_mod_def_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_mod_def_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_mod_def_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::KeyName` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_key_name_t {
    pub name: [c_char; 4],
}

impl Default for xcb_xkb_key_name_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::KeyName` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_key_name_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_key_name_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_key_name_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::KeyAlias` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_key_alias_t {
    pub real: [c_char; 4],
    pub alias: [c_char; 4],
}

impl Default for xcb_xkb_key_alias_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::KeyAlias` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_key_alias_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_key_alias_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_key_alias_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::CountedString16` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `string`
/// - `alignment_pad`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_counted_string_16_t {
    pub length: u16,
}

impl Default for xcb_xkb_counted_string_16_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::CountedString16` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_counted_string_16_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_counted_string_16_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_counted_string_16_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::KTMapEntry` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_kt_map_entry_t {
    pub active: u8,
    pub mods_mask: u8,
    pub level: u8,
    pub mods_mods: u8,
    pub mods_vmods: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_xkb_kt_map_entry_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::KTMapEntry` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_kt_map_entry_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_kt_map_entry_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_kt_map_entry_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::KeyType` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `map`
/// - `preserve`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_key_type_t {
    pub mods_mask: u8,
    pub mods_mods: u8,
    pub mods_vmods: u16,
    pub num_levels: u8,
    pub n_map_entries: u8,
    pub has_preserve: u8,
    pub pad0: u8,
}

impl Default for xcb_xkb_key_type_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::KeyType` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_key_type_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_key_type_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_key_type_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::KeySymMap` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `syms`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_key_sym_map_t {
    pub kt_index: [u8; 4],
    pub group_info: u8,
    pub width: u8,
    pub n_syms: u16,
}

impl Default for xcb_xkb_key_sym_map_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::KeySymMap` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_key_sym_map_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_key_sym_map_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_key_sym_map_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::CommonBehavior` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_common_behavior_t {
    pub type_: u8,
    pub data: u8,
}

impl Default for xcb_xkb_common_behavior_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::CommonBehavior` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_common_behavior_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_common_behavior_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_common_behavior_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::DefaultBehavior` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_default_behavior_t {
    pub type_: u8,
    pub pad0: u8,
}

impl Default for xcb_xkb_default_behavior_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::DefaultBehavior` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_default_behavior_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_default_behavior_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_default_behavior_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::LockBehavior` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_lock_behavior_t {
    pub type_: u8,
    pub pad0: u8,
}

impl Default for xcb_xkb_lock_behavior_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::LockBehavior` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_lock_behavior_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_lock_behavior_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_lock_behavior_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::RadioGroupBehavior` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_radio_group_behavior_t {
    pub type_: u8,
    pub group: u8,
}

impl Default for xcb_xkb_radio_group_behavior_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::RadioGroupBehavior` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_radio_group_behavior_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_radio_group_behavior_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_radio_group_behavior_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::OverlayBehavior` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_overlay_behavior_t {
    pub type_: u8,
    pub key: xcb_keycode_t,
}

impl Default for xcb_xkb_overlay_behavior_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::OverlayBehavior` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_overlay_behavior_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_overlay_behavior_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_overlay_behavior_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::PermamentLockBehavior` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_permament_lock_behavior_t {
    pub type_: u8,
    pub pad0: u8,
}

impl Default for xcb_xkb_permament_lock_behavior_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::PermamentLockBehavior` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_permament_lock_behavior_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_permament_lock_behavior_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_permament_lock_behavior_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::PermamentRadioGroupBehavior` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_permament_radio_group_behavior_t {
    pub type_: u8,
    pub group: u8,
}

impl Default for xcb_xkb_permament_radio_group_behavior_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::PermamentRadioGroupBehavior` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_permament_radio_group_behavior_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_permament_radio_group_behavior_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_permament_radio_group_behavior_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::PermamentOverlayBehavior` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_permament_overlay_behavior_t {
    pub type_: u8,
    pub key: xcb_keycode_t,
}

impl Default for xcb_xkb_permament_overlay_behavior_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::PermamentOverlayBehavior` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_permament_overlay_behavior_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_permament_overlay_behavior_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_permament_overlay_behavior_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::Behavior` union.
#[derive(Copy, Clone)]
#[repr(C)]
pub union xcb_xkb_behavior_t {
    pub common: xcb_xkb_common_behavior_t,
    pub default: xcb_xkb_default_behavior_t,
    pub lock: xcb_xkb_lock_behavior_t,
    pub radio_group: xcb_xkb_radio_group_behavior_t,
    pub overlay1: xcb_xkb_overlay_behavior_t,
    pub overlay2: xcb_xkb_overlay_behavior_t,
    pub permament_lock: xcb_xkb_permament_lock_behavior_t,
    pub permament_radio_group: xcb_xkb_permament_radio_group_behavior_t,
    pub permament_overlay1: xcb_xkb_permament_overlay_behavior_t,
    pub permament_overlay2: xcb_xkb_permament_overlay_behavior_t,
    pub type_: u8,
}

impl Default for xcb_xkb_behavior_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::Behavior` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_behavior_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_behavior_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_behavior_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::BehaviorType` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::BehaviorType::Default`](XCB_XKB_BEHAVIOR_TYPE_DEFAULT)
/// - [`xkb::BehaviorType::Lock`](XCB_XKB_BEHAVIOR_TYPE_LOCK)
/// - [`xkb::BehaviorType::RadioGroup`](XCB_XKB_BEHAVIOR_TYPE_RADIO_GROUP)
/// - [`xkb::BehaviorType::Overlay1`](XCB_XKB_BEHAVIOR_TYPE_OVERLAY_1)
/// - [`xkb::BehaviorType::Overlay2`](XCB_XKB_BEHAVIOR_TYPE_OVERLAY_2)
/// - [`xkb::BehaviorType::PermamentLock`](XCB_XKB_BEHAVIOR_TYPE_PERMAMENT_LOCK)
/// - [`xkb::BehaviorType::PermamentRadioGroup`](XCB_XKB_BEHAVIOR_TYPE_PERMAMENT_RADIO_GROUP)
/// - [`xkb::BehaviorType::PermamentOverlay1`](XCB_XKB_BEHAVIOR_TYPE_PERMAMENT_OVERLAY_1)
/// - [`xkb::BehaviorType::PermamentOverlay2`](XCB_XKB_BEHAVIOR_TYPE_PERMAMENT_OVERLAY_2)
pub type xcb_xkb_behavior_type_t = u32;
/// The `xkb::BehaviorType::Default` enum variant.
///
/// This is a variant of [`xcb_xkb_behavior_type_t`].
pub const XCB_XKB_BEHAVIOR_TYPE_DEFAULT: xcb_xkb_behavior_type_t = 0;
/// The `xkb::BehaviorType::Lock` enum variant.
///
/// This is a variant of [`xcb_xkb_behavior_type_t`].
pub const XCB_XKB_BEHAVIOR_TYPE_LOCK: xcb_xkb_behavior_type_t = 1;
/// The `xkb::BehaviorType::RadioGroup` enum variant.
///
/// This is a variant of [`xcb_xkb_behavior_type_t`].
pub const XCB_XKB_BEHAVIOR_TYPE_RADIO_GROUP: xcb_xkb_behavior_type_t = 2;
/// The `xkb::BehaviorType::Overlay1` enum variant.
///
/// This is a variant of [`xcb_xkb_behavior_type_t`].
pub const XCB_XKB_BEHAVIOR_TYPE_OVERLAY_1: xcb_xkb_behavior_type_t = 3;
/// The `xkb::BehaviorType::Overlay2` enum variant.
///
/// This is a variant of [`xcb_xkb_behavior_type_t`].
pub const XCB_XKB_BEHAVIOR_TYPE_OVERLAY_2: xcb_xkb_behavior_type_t = 4;
/// The `xkb::BehaviorType::PermamentLock` enum variant.
///
/// This is a variant of [`xcb_xkb_behavior_type_t`].
pub const XCB_XKB_BEHAVIOR_TYPE_PERMAMENT_LOCK: xcb_xkb_behavior_type_t = 129;
/// The `xkb::BehaviorType::PermamentRadioGroup` enum variant.
///
/// This is a variant of [`xcb_xkb_behavior_type_t`].
pub const XCB_XKB_BEHAVIOR_TYPE_PERMAMENT_RADIO_GROUP: xcb_xkb_behavior_type_t = 130;
/// The `xkb::BehaviorType::PermamentOverlay1` enum variant.
///
/// This is a variant of [`xcb_xkb_behavior_type_t`].
pub const XCB_XKB_BEHAVIOR_TYPE_PERMAMENT_OVERLAY_1: xcb_xkb_behavior_type_t = 131;
/// The `xkb::BehaviorType::PermamentOverlay2` enum variant.
///
/// This is a variant of [`xcb_xkb_behavior_type_t`].
pub const XCB_XKB_BEHAVIOR_TYPE_PERMAMENT_OVERLAY_2: xcb_xkb_behavior_type_t = 132;

/// The `xkb::SetBehavior` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_behavior_t {
    pub keycode: xcb_keycode_t,
    pub behavior: xcb_xkb_behavior_t,
    pub pad0: u8,
}

impl Default for xcb_xkb_set_behavior_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::SetBehavior` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_behavior_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_set_behavior_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_set_behavior_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::SetExplicit` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_explicit_t {
    pub keycode: xcb_keycode_t,
    pub explicit: u8,
}

impl Default for xcb_xkb_set_explicit_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::SetExplicit` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_explicit_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_set_explicit_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_set_explicit_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::KeyModMap` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_key_mod_map_t {
    pub keycode: xcb_keycode_t,
    pub mods: u8,
}

impl Default for xcb_xkb_key_mod_map_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::KeyModMap` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_key_mod_map_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_key_mod_map_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_key_mod_map_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::KeyVModMap` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_key_v_mod_map_t {
    pub keycode: xcb_keycode_t,
    pub pad0: u8,
    pub vmods: u16,
}

impl Default for xcb_xkb_key_v_mod_map_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::KeyVModMap` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_key_v_mod_map_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_key_v_mod_map_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_key_v_mod_map_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::KTSetMapEntry` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_kt_set_map_entry_t {
    pub level: u8,
    pub real_mods: u8,
    pub virtual_mods: u16,
}

impl Default for xcb_xkb_kt_set_map_entry_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::KTSetMapEntry` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_kt_set_map_entry_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_kt_set_map_entry_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_kt_set_map_entry_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::SetKeyType` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `entries`
/// - `preserve_entries`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_key_type_t {
    pub mask: u8,
    pub real_mods: u8,
    pub virtual_mods: u16,
    pub num_levels: u8,
    pub n_map_entries: u8,
    pub preserve: u8,
    pub pad0: u8,
}

impl Default for xcb_xkb_set_key_type_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::SetKeyType` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_key_type_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_set_key_type_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_set_key_type_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::STRING8` type.
pub type xcb_xkb_string8_t = c_char;

/// An iterator over `xkb::STRING8` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_string8_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_string8_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_string8_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::Outline` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `points`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_outline_t {
    pub n_points: u8,
    pub corner_radius: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_xkb_outline_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::Outline` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_outline_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_outline_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_outline_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::Shape` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `outlines`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_shape_t {
    pub name: xcb_atom_t,
    pub n_outlines: u8,
    pub primary_ndx: u8,
    pub approx_ndx: u8,
    pub pad0: u8,
}

impl Default for xcb_xkb_shape_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::Shape` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_shape_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_shape_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_shape_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::Key` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_key_t {
    pub name: [xcb_xkb_string8_t; 4],
    pub gap: i16,
    pub shape_ndx: u8,
    pub color_ndx: u8,
}

impl Default for xcb_xkb_key_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::Key` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_key_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_key_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_key_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::OverlayKey` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_overlay_key_t {
    pub over: [xcb_xkb_string8_t; 4],
    pub under: [xcb_xkb_string8_t; 4],
}

impl Default for xcb_xkb_overlay_key_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::OverlayKey` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_overlay_key_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_overlay_key_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_overlay_key_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::OverlayRow` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `keys`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_overlay_row_t {
    pub row_under: u8,
    pub n_keys: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_xkb_overlay_row_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::OverlayRow` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_overlay_row_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_overlay_row_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_overlay_row_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::Overlay` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `rows`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_overlay_t {
    pub name: xcb_atom_t,
    pub n_rows: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_xkb_overlay_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::Overlay` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_overlay_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_overlay_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_overlay_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::Row` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `keys`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_row_t {
    pub top: i16,
    pub left: i16,
    pub n_keys: u8,
    pub vertical: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_xkb_row_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::Row` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_row_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_row_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_row_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::DoodadType` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::DoodadType::Outline`](XCB_XKB_DOODAD_TYPE_OUTLINE)
/// - [`xkb::DoodadType::Solid`](XCB_XKB_DOODAD_TYPE_SOLID)
/// - [`xkb::DoodadType::Text`](XCB_XKB_DOODAD_TYPE_TEXT)
/// - [`xkb::DoodadType::Indicator`](XCB_XKB_DOODAD_TYPE_INDICATOR)
/// - [`xkb::DoodadType::Logo`](XCB_XKB_DOODAD_TYPE_LOGO)
pub type xcb_xkb_doodad_type_t = u32;
/// The `xkb::DoodadType::Outline` enum variant.
///
/// This is a variant of [`xcb_xkb_doodad_type_t`].
pub const XCB_XKB_DOODAD_TYPE_OUTLINE: xcb_xkb_doodad_type_t = 1;
/// The `xkb::DoodadType::Solid` enum variant.
///
/// This is a variant of [`xcb_xkb_doodad_type_t`].
pub const XCB_XKB_DOODAD_TYPE_SOLID: xcb_xkb_doodad_type_t = 2;
/// The `xkb::DoodadType::Text` enum variant.
///
/// This is a variant of [`xcb_xkb_doodad_type_t`].
pub const XCB_XKB_DOODAD_TYPE_TEXT: xcb_xkb_doodad_type_t = 3;
/// The `xkb::DoodadType::Indicator` enum variant.
///
/// This is a variant of [`xcb_xkb_doodad_type_t`].
pub const XCB_XKB_DOODAD_TYPE_INDICATOR: xcb_xkb_doodad_type_t = 4;
/// The `xkb::DoodadType::Logo` enum variant.
///
/// This is a variant of [`xcb_xkb_doodad_type_t`].
pub const XCB_XKB_DOODAD_TYPE_LOGO: xcb_xkb_doodad_type_t = 5;

/// The `xkb::Listing` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `string`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_listing_t {
    pub flags: u16,
    pub length: u16,
}

impl Default for xcb_xkb_listing_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::Listing` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_listing_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_listing_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_listing_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::DeviceLedInfo` struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `names`
/// - `maps`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_device_led_info_t {
    pub led_class: xcb_xkb_led_class_spec_t,
    pub led_id: xcb_xkb_id_spec_t,
    pub names_present: u32,
    pub maps_present: u32,
    pub phys_indicators: u32,
    pub state: u32,
}

impl Default for xcb_xkb_device_led_info_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::DeviceLedInfo` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_device_led_info_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_device_led_info_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_device_led_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::Error` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::Error::BadDevice`](XCB_XKB_ERROR_BAD_DEVICE)
/// - [`xkb::Error::BadClass`](XCB_XKB_ERROR_BAD_CLASS)
/// - [`xkb::Error::BadId`](XCB_XKB_ERROR_BAD_ID)
pub type xcb_xkb_error_t = u32;
/// The `xkb::Error::BadDevice` enum variant.
///
/// This is a variant of [`xcb_xkb_error_t`].
pub const XCB_XKB_ERROR_BAD_DEVICE: xcb_xkb_error_t = 255;
/// The `xkb::Error::BadClass` enum variant.
///
/// This is a variant of [`xcb_xkb_error_t`].
pub const XCB_XKB_ERROR_BAD_CLASS: xcb_xkb_error_t = 254;
/// The `xkb::Error::BadId` enum variant.
///
/// This is a variant of [`xcb_xkb_error_t`].
pub const XCB_XKB_ERROR_BAD_ID: xcb_xkb_error_t = 253;

/// The opcode for `xkb::Keyboard` errors.
///
/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],
/// then the type of the error is [`xcb_xkb_keyboard_error_t`].
pub const XCB_XKB_KEYBOARD: u8 = 0i32 as u8;

/// The `xkb::Keyboard` error.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_keyboard_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
    pub pad0: [u8; 21],
}

impl Default for xcb_xkb_keyboard_error_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::SA` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::SA::ClearLocks`](XCB_XKB_SA_CLEAR_LOCKS)
/// - [`xkb::SA::LatchToLock`](XCB_XKB_SA_LATCH_TO_LOCK)
/// - [`xkb::SA::UseModMapMods`](XCB_XKB_SA_USE_MOD_MAP_MODS)
/// - [`xkb::SA::GroupAbsolute`](XCB_XKB_SA_GROUP_ABSOLUTE)
pub type xcb_xkb_sa_t = u32;
/// The `xkb::SA::ClearLocks` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_t`].
pub const XCB_XKB_SA_CLEAR_LOCKS: xcb_xkb_sa_t = 1;
/// The `xkb::SA::LatchToLock` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_t`].
pub const XCB_XKB_SA_LATCH_TO_LOCK: xcb_xkb_sa_t = 2;
/// The `xkb::SA::UseModMapMods` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_t`].
pub const XCB_XKB_SA_USE_MOD_MAP_MODS: xcb_xkb_sa_t = 4;
/// The `xkb::SA::GroupAbsolute` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_t`].
pub const XCB_XKB_SA_GROUP_ABSOLUTE: xcb_xkb_sa_t = 4;

/// The `xkb::SAType` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::SAType::NoAction`](XCB_XKB_SA_TYPE_NO_ACTION)
/// - [`xkb::SAType::SetMods`](XCB_XKB_SA_TYPE_SET_MODS)
/// - [`xkb::SAType::LatchMods`](XCB_XKB_SA_TYPE_LATCH_MODS)
/// - [`xkb::SAType::LockMods`](XCB_XKB_SA_TYPE_LOCK_MODS)
/// - [`xkb::SAType::SetGroup`](XCB_XKB_SA_TYPE_SET_GROUP)
/// - [`xkb::SAType::LatchGroup`](XCB_XKB_SA_TYPE_LATCH_GROUP)
/// - [`xkb::SAType::LockGroup`](XCB_XKB_SA_TYPE_LOCK_GROUP)
/// - [`xkb::SAType::MovePtr`](XCB_XKB_SA_TYPE_MOVE_PTR)
/// - [`xkb::SAType::PtrBtn`](XCB_XKB_SA_TYPE_PTR_BTN)
/// - [`xkb::SAType::LockPtrBtn`](XCB_XKB_SA_TYPE_LOCK_PTR_BTN)
/// - [`xkb::SAType::SetPtrDflt`](XCB_XKB_SA_TYPE_SET_PTR_DFLT)
/// - [`xkb::SAType::ISOLock`](XCB_XKB_SA_TYPE_ISO_LOCK)
/// - [`xkb::SAType::Terminate`](XCB_XKB_SA_TYPE_TERMINATE)
/// - [`xkb::SAType::SwitchScreen`](XCB_XKB_SA_TYPE_SWITCH_SCREEN)
/// - [`xkb::SAType::SetControls`](XCB_XKB_SA_TYPE_SET_CONTROLS)
/// - [`xkb::SAType::LockControls`](XCB_XKB_SA_TYPE_LOCK_CONTROLS)
/// - [`xkb::SAType::ActionMessage`](XCB_XKB_SA_TYPE_ACTION_MESSAGE)
/// - [`xkb::SAType::RedirectKey`](XCB_XKB_SA_TYPE_REDIRECT_KEY)
/// - [`xkb::SAType::DeviceBtn`](XCB_XKB_SA_TYPE_DEVICE_BTN)
/// - [`xkb::SAType::LockDeviceBtn`](XCB_XKB_SA_TYPE_LOCK_DEVICE_BTN)
/// - [`xkb::SAType::DeviceValuator`](XCB_XKB_SA_TYPE_DEVICE_VALUATOR)
pub type xcb_xkb_sa_type_t = u32;
/// The `xkb::SAType::NoAction` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_type_t`].
pub const XCB_XKB_SA_TYPE_NO_ACTION: xcb_xkb_sa_type_t = 0;
/// The `xkb::SAType::SetMods` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_type_t`].
pub const XCB_XKB_SA_TYPE_SET_MODS: xcb_xkb_sa_type_t = 1;
/// The `xkb::SAType::LatchMods` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_type_t`].
pub const XCB_XKB_SA_TYPE_LATCH_MODS: xcb_xkb_sa_type_t = 2;
/// The `xkb::SAType::LockMods` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_type_t`].
pub const XCB_XKB_SA_TYPE_LOCK_MODS: xcb_xkb_sa_type_t = 3;
/// The `xkb::SAType::SetGroup` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_type_t`].
pub const XCB_XKB_SA_TYPE_SET_GROUP: xcb_xkb_sa_type_t = 4;
/// The `xkb::SAType::LatchGroup` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_type_t`].
pub const XCB_XKB_SA_TYPE_LATCH_GROUP: xcb_xkb_sa_type_t = 5;
/// The `xkb::SAType::LockGroup` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_type_t`].
pub const XCB_XKB_SA_TYPE_LOCK_GROUP: xcb_xkb_sa_type_t = 6;
/// The `xkb::SAType::MovePtr` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_type_t`].
pub const XCB_XKB_SA_TYPE_MOVE_PTR: xcb_xkb_sa_type_t = 7;
/// The `xkb::SAType::PtrBtn` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_type_t`].
pub const XCB_XKB_SA_TYPE_PTR_BTN: xcb_xkb_sa_type_t = 8;
/// The `xkb::SAType::LockPtrBtn` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_type_t`].
pub const XCB_XKB_SA_TYPE_LOCK_PTR_BTN: xcb_xkb_sa_type_t = 9;
/// The `xkb::SAType::SetPtrDflt` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_type_t`].
pub const XCB_XKB_SA_TYPE_SET_PTR_DFLT: xcb_xkb_sa_type_t = 10;
/// The `xkb::SAType::ISOLock` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_type_t`].
pub const XCB_XKB_SA_TYPE_ISO_LOCK: xcb_xkb_sa_type_t = 11;
/// The `xkb::SAType::Terminate` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_type_t`].
pub const XCB_XKB_SA_TYPE_TERMINATE: xcb_xkb_sa_type_t = 12;
/// The `xkb::SAType::SwitchScreen` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_type_t`].
pub const XCB_XKB_SA_TYPE_SWITCH_SCREEN: xcb_xkb_sa_type_t = 13;
/// The `xkb::SAType::SetControls` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_type_t`].
pub const XCB_XKB_SA_TYPE_SET_CONTROLS: xcb_xkb_sa_type_t = 14;
/// The `xkb::SAType::LockControls` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_type_t`].
pub const XCB_XKB_SA_TYPE_LOCK_CONTROLS: xcb_xkb_sa_type_t = 15;
/// The `xkb::SAType::ActionMessage` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_type_t`].
pub const XCB_XKB_SA_TYPE_ACTION_MESSAGE: xcb_xkb_sa_type_t = 16;
/// The `xkb::SAType::RedirectKey` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_type_t`].
pub const XCB_XKB_SA_TYPE_REDIRECT_KEY: xcb_xkb_sa_type_t = 17;
/// The `xkb::SAType::DeviceBtn` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_type_t`].
pub const XCB_XKB_SA_TYPE_DEVICE_BTN: xcb_xkb_sa_type_t = 18;
/// The `xkb::SAType::LockDeviceBtn` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_type_t`].
pub const XCB_XKB_SA_TYPE_LOCK_DEVICE_BTN: xcb_xkb_sa_type_t = 19;
/// The `xkb::SAType::DeviceValuator` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_type_t`].
pub const XCB_XKB_SA_TYPE_DEVICE_VALUATOR: xcb_xkb_sa_type_t = 20;

/// The `xkb::SANoAction` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_no_action_t {
    pub type_: u8,
    pub pad0: [u8; 7],
}

impl Default for xcb_xkb_sa_no_action_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::SANoAction` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_no_action_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_sa_no_action_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_sa_no_action_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::SASetMods` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_set_mods_t {
    pub type_: u8,
    pub flags: u8,
    pub mask: u8,
    pub real_mods: u8,
    pub vmods_high: u8,
    pub vmods_low: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_xkb_sa_set_mods_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::SASetMods` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_set_mods_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_sa_set_mods_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_sa_set_mods_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::SALatchMods` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_latch_mods_t {
    pub type_: u8,
    pub flags: u8,
    pub mask: u8,
    pub real_mods: u8,
    pub vmods_high: u8,
    pub vmods_low: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_xkb_sa_latch_mods_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::SALatchMods` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_latch_mods_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_sa_latch_mods_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_sa_latch_mods_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::SALockMods` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_lock_mods_t {
    pub type_: u8,
    pub flags: u8,
    pub mask: u8,
    pub real_mods: u8,
    pub vmods_high: u8,
    pub vmods_low: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_xkb_sa_lock_mods_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::SALockMods` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_lock_mods_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_sa_lock_mods_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_sa_lock_mods_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::SASetGroup` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_set_group_t {
    pub type_: u8,
    pub flags: u8,
    pub group: i8,
    pub pad0: [u8; 5],
}

impl Default for xcb_xkb_sa_set_group_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::SASetGroup` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_set_group_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_sa_set_group_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_sa_set_group_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::SALatchGroup` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_latch_group_t {
    pub type_: u8,
    pub flags: u8,
    pub group: i8,
    pub pad0: [u8; 5],
}

impl Default for xcb_xkb_sa_latch_group_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::SALatchGroup` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_latch_group_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_sa_latch_group_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_sa_latch_group_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::SALockGroup` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_lock_group_t {
    pub type_: u8,
    pub flags: u8,
    pub group: i8,
    pub pad0: [u8; 5],
}

impl Default for xcb_xkb_sa_lock_group_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::SALockGroup` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_lock_group_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_sa_lock_group_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_sa_lock_group_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::SAMovePtrFlag` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::SAMovePtrFlag::NoAcceleration`](XCB_XKB_SA_MOVE_PTR_FLAG_NO_ACCELERATION)
/// - [`xkb::SAMovePtrFlag::MoveAbsoluteX`](XCB_XKB_SA_MOVE_PTR_FLAG_MOVE_ABSOLUTE_X)
/// - [`xkb::SAMovePtrFlag::MoveAbsoluteY`](XCB_XKB_SA_MOVE_PTR_FLAG_MOVE_ABSOLUTE_Y)
pub type xcb_xkb_sa_move_ptr_flag_t = u32;
/// The `xkb::SAMovePtrFlag::NoAcceleration` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_move_ptr_flag_t`].
pub const XCB_XKB_SA_MOVE_PTR_FLAG_NO_ACCELERATION: xcb_xkb_sa_move_ptr_flag_t = 1;
/// The `xkb::SAMovePtrFlag::MoveAbsoluteX` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_move_ptr_flag_t`].
pub const XCB_XKB_SA_MOVE_PTR_FLAG_MOVE_ABSOLUTE_X: xcb_xkb_sa_move_ptr_flag_t = 2;
/// The `xkb::SAMovePtrFlag::MoveAbsoluteY` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_move_ptr_flag_t`].
pub const XCB_XKB_SA_MOVE_PTR_FLAG_MOVE_ABSOLUTE_Y: xcb_xkb_sa_move_ptr_flag_t = 4;

/// The `xkb::SAMovePtr` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_move_ptr_t {
    pub type_: u8,
    pub flags: u8,
    pub x_high: i8,
    pub x_low: u8,
    pub y_high: i8,
    pub y_low: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_xkb_sa_move_ptr_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::SAMovePtr` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_move_ptr_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_sa_move_ptr_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_sa_move_ptr_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::SAPtrBtn` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_ptr_btn_t {
    pub type_: u8,
    pub flags: u8,
    pub count: u8,
    pub button: u8,
    pub pad0: [u8; 4],
}

impl Default for xcb_xkb_sa_ptr_btn_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::SAPtrBtn` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_ptr_btn_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_sa_ptr_btn_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_sa_ptr_btn_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::SALockPtrBtn` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_lock_ptr_btn_t {
    pub type_: u8,
    pub flags: u8,
    pub pad0: u8,
    pub button: u8,
    pub pad1: [u8; 4],
}

impl Default for xcb_xkb_sa_lock_ptr_btn_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::SALockPtrBtn` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_lock_ptr_btn_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_sa_lock_ptr_btn_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_sa_lock_ptr_btn_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::SASetPtrDfltFlag` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::SASetPtrDfltFlag::DfltBtnAbsolute`](XCB_XKB_SA_SET_PTR_DFLT_FLAG_DFLT_BTN_ABSOLUTE)
/// - [`xkb::SASetPtrDfltFlag::AffectDfltButton`](XCB_XKB_SA_SET_PTR_DFLT_FLAG_AFFECT_DFLT_BUTTON)
pub type xcb_xkb_sa_set_ptr_dflt_flag_t = u32;
/// The `xkb::SASetPtrDfltFlag::DfltBtnAbsolute` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_set_ptr_dflt_flag_t`].
pub const XCB_XKB_SA_SET_PTR_DFLT_FLAG_DFLT_BTN_ABSOLUTE: xcb_xkb_sa_set_ptr_dflt_flag_t = 4;
/// The `xkb::SASetPtrDfltFlag::AffectDfltButton` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_set_ptr_dflt_flag_t`].
pub const XCB_XKB_SA_SET_PTR_DFLT_FLAG_AFFECT_DFLT_BUTTON: xcb_xkb_sa_set_ptr_dflt_flag_t = 1;

/// The `xkb::SASetPtrDflt` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_set_ptr_dflt_t {
    pub type_: u8,
    pub flags: u8,
    pub affect: u8,
    pub value: i8,
    pub pad0: [u8; 4],
}

impl Default for xcb_xkb_sa_set_ptr_dflt_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::SASetPtrDflt` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_set_ptr_dflt_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_sa_set_ptr_dflt_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_sa_set_ptr_dflt_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::SAIsoLockFlag` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::SAIsoLockFlag::NoLock`](XCB_XKB_SA_ISO_LOCK_FLAG_NO_LOCK)
/// - [`xkb::SAIsoLockFlag::NoUnlock`](XCB_XKB_SA_ISO_LOCK_FLAG_NO_UNLOCK)
/// - [`xkb::SAIsoLockFlag::UseModMapMods`](XCB_XKB_SA_ISO_LOCK_FLAG_USE_MOD_MAP_MODS)
/// - [`xkb::SAIsoLockFlag::GroupAbsolute`](XCB_XKB_SA_ISO_LOCK_FLAG_GROUP_ABSOLUTE)
/// - [`xkb::SAIsoLockFlag::ISODfltIsGroup`](XCB_XKB_SA_ISO_LOCK_FLAG_ISO_DFLT_IS_GROUP)
pub type xcb_xkb_sa_iso_lock_flag_t = u32;
/// The `xkb::SAIsoLockFlag::NoLock` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_iso_lock_flag_t`].
pub const XCB_XKB_SA_ISO_LOCK_FLAG_NO_LOCK: xcb_xkb_sa_iso_lock_flag_t = 1;
/// The `xkb::SAIsoLockFlag::NoUnlock` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_iso_lock_flag_t`].
pub const XCB_XKB_SA_ISO_LOCK_FLAG_NO_UNLOCK: xcb_xkb_sa_iso_lock_flag_t = 2;
/// The `xkb::SAIsoLockFlag::UseModMapMods` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_iso_lock_flag_t`].
pub const XCB_XKB_SA_ISO_LOCK_FLAG_USE_MOD_MAP_MODS: xcb_xkb_sa_iso_lock_flag_t = 4;
/// The `xkb::SAIsoLockFlag::GroupAbsolute` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_iso_lock_flag_t`].
pub const XCB_XKB_SA_ISO_LOCK_FLAG_GROUP_ABSOLUTE: xcb_xkb_sa_iso_lock_flag_t = 4;
/// The `xkb::SAIsoLockFlag::ISODfltIsGroup` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_iso_lock_flag_t`].
pub const XCB_XKB_SA_ISO_LOCK_FLAG_ISO_DFLT_IS_GROUP: xcb_xkb_sa_iso_lock_flag_t = 8;

/// The `xkb::SAIsoLockNoAffect` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::SAIsoLockNoAffect::Ctrls`](XCB_XKB_SA_ISO_LOCK_NO_AFFECT_CTRLS)
/// - [`xkb::SAIsoLockNoAffect::Ptr`](XCB_XKB_SA_ISO_LOCK_NO_AFFECT_PTR)
/// - [`xkb::SAIsoLockNoAffect::Group`](XCB_XKB_SA_ISO_LOCK_NO_AFFECT_GROUP)
/// - [`xkb::SAIsoLockNoAffect::Mods`](XCB_XKB_SA_ISO_LOCK_NO_AFFECT_MODS)
pub type xcb_xkb_sa_iso_lock_no_affect_t = u32;
/// The `xkb::SAIsoLockNoAffect::Ctrls` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_iso_lock_no_affect_t`].
pub const XCB_XKB_SA_ISO_LOCK_NO_AFFECT_CTRLS: xcb_xkb_sa_iso_lock_no_affect_t = 8;
/// The `xkb::SAIsoLockNoAffect::Ptr` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_iso_lock_no_affect_t`].
pub const XCB_XKB_SA_ISO_LOCK_NO_AFFECT_PTR: xcb_xkb_sa_iso_lock_no_affect_t = 16;
/// The `xkb::SAIsoLockNoAffect::Group` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_iso_lock_no_affect_t`].
pub const XCB_XKB_SA_ISO_LOCK_NO_AFFECT_GROUP: xcb_xkb_sa_iso_lock_no_affect_t = 32;
/// The `xkb::SAIsoLockNoAffect::Mods` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_iso_lock_no_affect_t`].
pub const XCB_XKB_SA_ISO_LOCK_NO_AFFECT_MODS: xcb_xkb_sa_iso_lock_no_affect_t = 64;

/// The `xkb::SAIsoLock` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_iso_lock_t {
    pub type_: u8,
    pub flags: u8,
    pub mask: u8,
    pub real_mods: u8,
    pub group: i8,
    pub affect: u8,
    pub vmods_high: u8,
    pub vmods_low: u8,
}

impl Default for xcb_xkb_sa_iso_lock_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::SAIsoLock` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_iso_lock_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_sa_iso_lock_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_sa_iso_lock_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::SATerminate` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_terminate_t {
    pub type_: u8,
    pub pad0: [u8; 7],
}

impl Default for xcb_xkb_sa_terminate_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::SATerminate` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_terminate_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_sa_terminate_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_sa_terminate_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::SwitchScreenFlag` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::SwitchScreenFlag::Application`](XCB_XKB_SWITCH_SCREEN_FLAG_APPLICATION)
/// - [`xkb::SwitchScreenFlag::Absolute`](XCB_XKB_SWITCH_SCREEN_FLAG_ABSOLUTE)
pub type xcb_xkb_switch_screen_flag_t = u32;
/// The `xkb::SwitchScreenFlag::Application` enum variant.
///
/// This is a variant of [`xcb_xkb_switch_screen_flag_t`].
pub const XCB_XKB_SWITCH_SCREEN_FLAG_APPLICATION: xcb_xkb_switch_screen_flag_t = 1;
/// The `xkb::SwitchScreenFlag::Absolute` enum variant.
///
/// This is a variant of [`xcb_xkb_switch_screen_flag_t`].
pub const XCB_XKB_SWITCH_SCREEN_FLAG_ABSOLUTE: xcb_xkb_switch_screen_flag_t = 4;

/// The `xkb::SASwitchScreen` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_switch_screen_t {
    pub type_: u8,
    pub flags: u8,
    pub new_screen: i8,
    pub pad0: [u8; 5],
}

impl Default for xcb_xkb_sa_switch_screen_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::SASwitchScreen` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_switch_screen_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_sa_switch_screen_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_sa_switch_screen_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::BoolCtrlsHigh` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::BoolCtrlsHigh::AccessXFeedback`](XCB_XKB_BOOL_CTRLS_HIGH_ACCESS_X_FEEDBACK)
/// - [`xkb::BoolCtrlsHigh::AudibleBell`](XCB_XKB_BOOL_CTRLS_HIGH_AUDIBLE_BELL)
/// - [`xkb::BoolCtrlsHigh::Overlay1`](XCB_XKB_BOOL_CTRLS_HIGH_OVERLAY_1)
/// - [`xkb::BoolCtrlsHigh::Overlay2`](XCB_XKB_BOOL_CTRLS_HIGH_OVERLAY_2)
/// - [`xkb::BoolCtrlsHigh::IgnoreGroupLock`](XCB_XKB_BOOL_CTRLS_HIGH_IGNORE_GROUP_LOCK)
pub type xcb_xkb_bool_ctrls_high_t = u32;
/// The `xkb::BoolCtrlsHigh::AccessXFeedback` enum variant.
///
/// This is a variant of [`xcb_xkb_bool_ctrls_high_t`].
pub const XCB_XKB_BOOL_CTRLS_HIGH_ACCESS_X_FEEDBACK: xcb_xkb_bool_ctrls_high_t = 1;
/// The `xkb::BoolCtrlsHigh::AudibleBell` enum variant.
///
/// This is a variant of [`xcb_xkb_bool_ctrls_high_t`].
pub const XCB_XKB_BOOL_CTRLS_HIGH_AUDIBLE_BELL: xcb_xkb_bool_ctrls_high_t = 2;
/// The `xkb::BoolCtrlsHigh::Overlay1` enum variant.
///
/// This is a variant of [`xcb_xkb_bool_ctrls_high_t`].
pub const XCB_XKB_BOOL_CTRLS_HIGH_OVERLAY_1: xcb_xkb_bool_ctrls_high_t = 4;
/// The `xkb::BoolCtrlsHigh::Overlay2` enum variant.
///
/// This is a variant of [`xcb_xkb_bool_ctrls_high_t`].
pub const XCB_XKB_BOOL_CTRLS_HIGH_OVERLAY_2: xcb_xkb_bool_ctrls_high_t = 8;
/// The `xkb::BoolCtrlsHigh::IgnoreGroupLock` enum variant.
///
/// This is a variant of [`xcb_xkb_bool_ctrls_high_t`].
pub const XCB_XKB_BOOL_CTRLS_HIGH_IGNORE_GROUP_LOCK: xcb_xkb_bool_ctrls_high_t = 16;

/// The `xkb::BoolCtrlsLow` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::BoolCtrlsLow::RepeatKeys`](XCB_XKB_BOOL_CTRLS_LOW_REPEAT_KEYS)
/// - [`xkb::BoolCtrlsLow::SlowKeys`](XCB_XKB_BOOL_CTRLS_LOW_SLOW_KEYS)
/// - [`xkb::BoolCtrlsLow::BounceKeys`](XCB_XKB_BOOL_CTRLS_LOW_BOUNCE_KEYS)
/// - [`xkb::BoolCtrlsLow::StickyKeys`](XCB_XKB_BOOL_CTRLS_LOW_STICKY_KEYS)
/// - [`xkb::BoolCtrlsLow::MouseKeys`](XCB_XKB_BOOL_CTRLS_LOW_MOUSE_KEYS)
/// - [`xkb::BoolCtrlsLow::MouseKeysAccel`](XCB_XKB_BOOL_CTRLS_LOW_MOUSE_KEYS_ACCEL)
/// - [`xkb::BoolCtrlsLow::AccessXKeys`](XCB_XKB_BOOL_CTRLS_LOW_ACCESS_X_KEYS)
/// - [`xkb::BoolCtrlsLow::AccessXTimeout`](XCB_XKB_BOOL_CTRLS_LOW_ACCESS_X_TIMEOUT)
pub type xcb_xkb_bool_ctrls_low_t = u32;
/// The `xkb::BoolCtrlsLow::RepeatKeys` enum variant.
///
/// This is a variant of [`xcb_xkb_bool_ctrls_low_t`].
pub const XCB_XKB_BOOL_CTRLS_LOW_REPEAT_KEYS: xcb_xkb_bool_ctrls_low_t = 1;
/// The `xkb::BoolCtrlsLow::SlowKeys` enum variant.
///
/// This is a variant of [`xcb_xkb_bool_ctrls_low_t`].
pub const XCB_XKB_BOOL_CTRLS_LOW_SLOW_KEYS: xcb_xkb_bool_ctrls_low_t = 2;
/// The `xkb::BoolCtrlsLow::BounceKeys` enum variant.
///
/// This is a variant of [`xcb_xkb_bool_ctrls_low_t`].
pub const XCB_XKB_BOOL_CTRLS_LOW_BOUNCE_KEYS: xcb_xkb_bool_ctrls_low_t = 4;
/// The `xkb::BoolCtrlsLow::StickyKeys` enum variant.
///
/// This is a variant of [`xcb_xkb_bool_ctrls_low_t`].
pub const XCB_XKB_BOOL_CTRLS_LOW_STICKY_KEYS: xcb_xkb_bool_ctrls_low_t = 8;
/// The `xkb::BoolCtrlsLow::MouseKeys` enum variant.
///
/// This is a variant of [`xcb_xkb_bool_ctrls_low_t`].
pub const XCB_XKB_BOOL_CTRLS_LOW_MOUSE_KEYS: xcb_xkb_bool_ctrls_low_t = 16;
/// The `xkb::BoolCtrlsLow::MouseKeysAccel` enum variant.
///
/// This is a variant of [`xcb_xkb_bool_ctrls_low_t`].
pub const XCB_XKB_BOOL_CTRLS_LOW_MOUSE_KEYS_ACCEL: xcb_xkb_bool_ctrls_low_t = 32;
/// The `xkb::BoolCtrlsLow::AccessXKeys` enum variant.
///
/// This is a variant of [`xcb_xkb_bool_ctrls_low_t`].
pub const XCB_XKB_BOOL_CTRLS_LOW_ACCESS_X_KEYS: xcb_xkb_bool_ctrls_low_t = 64;
/// The `xkb::BoolCtrlsLow::AccessXTimeout` enum variant.
///
/// This is a variant of [`xcb_xkb_bool_ctrls_low_t`].
pub const XCB_XKB_BOOL_CTRLS_LOW_ACCESS_X_TIMEOUT: xcb_xkb_bool_ctrls_low_t = 128;

/// The `xkb::SASetControls` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_set_controls_t {
    pub type_: u8,
    pub pad0: [u8; 3],
    pub bool_ctrls_high: u8,
    pub bool_ctrls_low: u8,
    pub pad1: [u8; 2],
}

impl Default for xcb_xkb_sa_set_controls_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::SASetControls` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_set_controls_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_sa_set_controls_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_sa_set_controls_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::SALockControls` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_lock_controls_t {
    pub type_: u8,
    pub pad0: [u8; 3],
    pub bool_ctrls_high: u8,
    pub bool_ctrls_low: u8,
    pub pad1: [u8; 2],
}

impl Default for xcb_xkb_sa_lock_controls_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::SALockControls` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_lock_controls_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_sa_lock_controls_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_sa_lock_controls_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::ActionMessageFlag` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::ActionMessageFlag::OnPress`](XCB_XKB_ACTION_MESSAGE_FLAG_ON_PRESS)
/// - [`xkb::ActionMessageFlag::OnRelease`](XCB_XKB_ACTION_MESSAGE_FLAG_ON_RELEASE)
/// - [`xkb::ActionMessageFlag::GenKeyEvent`](XCB_XKB_ACTION_MESSAGE_FLAG_GEN_KEY_EVENT)
pub type xcb_xkb_action_message_flag_t = u32;
/// The `xkb::ActionMessageFlag::OnPress` enum variant.
///
/// This is a variant of [`xcb_xkb_action_message_flag_t`].
pub const XCB_XKB_ACTION_MESSAGE_FLAG_ON_PRESS: xcb_xkb_action_message_flag_t = 1;
/// The `xkb::ActionMessageFlag::OnRelease` enum variant.
///
/// This is a variant of [`xcb_xkb_action_message_flag_t`].
pub const XCB_XKB_ACTION_MESSAGE_FLAG_ON_RELEASE: xcb_xkb_action_message_flag_t = 2;
/// The `xkb::ActionMessageFlag::GenKeyEvent` enum variant.
///
/// This is a variant of [`xcb_xkb_action_message_flag_t`].
pub const XCB_XKB_ACTION_MESSAGE_FLAG_GEN_KEY_EVENT: xcb_xkb_action_message_flag_t = 4;

/// The `xkb::SAActionMessage` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_action_message_t {
    pub type_: u8,
    pub flags: u8,
    pub message: [u8; 6],
}

impl Default for xcb_xkb_sa_action_message_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::SAActionMessage` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_action_message_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_sa_action_message_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_sa_action_message_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::SARedirectKey` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_redirect_key_t {
    pub type_: u8,
    pub newkey: xcb_keycode_t,
    pub mask: u8,
    pub real_modifiers: u8,
    pub vmods_mask_high: u8,
    pub vmods_mask_low: u8,
    pub vmods_high: u8,
    pub vmods_low: u8,
}

impl Default for xcb_xkb_sa_redirect_key_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::SARedirectKey` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_redirect_key_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_sa_redirect_key_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_sa_redirect_key_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::SADeviceBtn` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_device_btn_t {
    pub type_: u8,
    pub flags: u8,
    pub count: u8,
    pub button: u8,
    pub device: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_xkb_sa_device_btn_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::SADeviceBtn` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_device_btn_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_sa_device_btn_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_sa_device_btn_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::LockDeviceFlags` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::LockDeviceFlags::NoLock`](XCB_XKB_LOCK_DEVICE_FLAGS_NO_LOCK)
/// - [`xkb::LockDeviceFlags::NoUnlock`](XCB_XKB_LOCK_DEVICE_FLAGS_NO_UNLOCK)
pub type xcb_xkb_lock_device_flags_t = u32;
/// The `xkb::LockDeviceFlags::NoLock` enum variant.
///
/// This is a variant of [`xcb_xkb_lock_device_flags_t`].
pub const XCB_XKB_LOCK_DEVICE_FLAGS_NO_LOCK: xcb_xkb_lock_device_flags_t = 1;
/// The `xkb::LockDeviceFlags::NoUnlock` enum variant.
///
/// This is a variant of [`xcb_xkb_lock_device_flags_t`].
pub const XCB_XKB_LOCK_DEVICE_FLAGS_NO_UNLOCK: xcb_xkb_lock_device_flags_t = 2;

/// The `xkb::SALockDeviceBtn` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_lock_device_btn_t {
    pub type_: u8,
    pub flags: u8,
    pub pad0: u8,
    pub button: u8,
    pub device: u8,
    pub pad1: [u8; 3],
}

impl Default for xcb_xkb_sa_lock_device_btn_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::SALockDeviceBtn` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_lock_device_btn_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_sa_lock_device_btn_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_sa_lock_device_btn_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::SAValWhat` enum.
///
/// This enum has the following variants:
///
/// - [`xkb::SAValWhat::IgnoreVal`](XCB_XKB_SA_VAL_WHAT_IGNORE_VAL)
/// - [`xkb::SAValWhat::SetValMin`](XCB_XKB_SA_VAL_WHAT_SET_VAL_MIN)
/// - [`xkb::SAValWhat::SetValCenter`](XCB_XKB_SA_VAL_WHAT_SET_VAL_CENTER)
/// - [`xkb::SAValWhat::SetValMax`](XCB_XKB_SA_VAL_WHAT_SET_VAL_MAX)
/// - [`xkb::SAValWhat::SetValRelative`](XCB_XKB_SA_VAL_WHAT_SET_VAL_RELATIVE)
/// - [`xkb::SAValWhat::SetValAbsolute`](XCB_XKB_SA_VAL_WHAT_SET_VAL_ABSOLUTE)
pub type xcb_xkb_sa_val_what_t = u32;
/// The `xkb::SAValWhat::IgnoreVal` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_val_what_t`].
pub const XCB_XKB_SA_VAL_WHAT_IGNORE_VAL: xcb_xkb_sa_val_what_t = 0;
/// The `xkb::SAValWhat::SetValMin` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_val_what_t`].
pub const XCB_XKB_SA_VAL_WHAT_SET_VAL_MIN: xcb_xkb_sa_val_what_t = 1;
/// The `xkb::SAValWhat::SetValCenter` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_val_what_t`].
pub const XCB_XKB_SA_VAL_WHAT_SET_VAL_CENTER: xcb_xkb_sa_val_what_t = 2;
/// The `xkb::SAValWhat::SetValMax` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_val_what_t`].
pub const XCB_XKB_SA_VAL_WHAT_SET_VAL_MAX: xcb_xkb_sa_val_what_t = 3;
/// The `xkb::SAValWhat::SetValRelative` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_val_what_t`].
pub const XCB_XKB_SA_VAL_WHAT_SET_VAL_RELATIVE: xcb_xkb_sa_val_what_t = 4;
/// The `xkb::SAValWhat::SetValAbsolute` enum variant.
///
/// This is a variant of [`xcb_xkb_sa_val_what_t`].
pub const XCB_XKB_SA_VAL_WHAT_SET_VAL_ABSOLUTE: xcb_xkb_sa_val_what_t = 5;

/// The `xkb::SADeviceValuator` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_device_valuator_t {
    pub type_: u8,
    pub device: u8,
    pub val1what: u8,
    pub val1index: u8,
    pub val1value: u8,
    pub val2what: u8,
    pub val2index: u8,
    pub val2value: u8,
}

impl Default for xcb_xkb_sa_device_valuator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::SADeviceValuator` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_device_valuator_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_sa_device_valuator_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_sa_device_valuator_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::SIAction` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_si_action_t {
    pub type_: u8,
    pub data: [u8; 7],
}

impl Default for xcb_xkb_si_action_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::SIAction` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_si_action_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_si_action_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_si_action_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::SymInterpret` struct.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sym_interpret_t {
    pub sym: xcb_keysym_t,
    pub mods: u8,
    pub match_: u8,
    pub virtual_mod: u8,
    pub flags: u8,
    pub action: xcb_xkb_si_action_t,
}

impl Default for xcb_xkb_sym_interpret_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::SymInterpret` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sym_interpret_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_sym_interpret_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_sym_interpret_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::Action` union.
#[derive(Copy, Clone)]
#[repr(C)]
pub union xcb_xkb_action_t {
    pub noaction: xcb_xkb_sa_no_action_t,
    pub setmods: xcb_xkb_sa_set_mods_t,
    pub latchmods: xcb_xkb_sa_latch_mods_t,
    pub lockmods: xcb_xkb_sa_lock_mods_t,
    pub setgroup: xcb_xkb_sa_set_group_t,
    pub latchgroup: xcb_xkb_sa_latch_group_t,
    pub lockgroup: xcb_xkb_sa_lock_group_t,
    pub moveptr: xcb_xkb_sa_move_ptr_t,
    pub ptrbtn: xcb_xkb_sa_ptr_btn_t,
    pub lockptrbtn: xcb_xkb_sa_lock_ptr_btn_t,
    pub setptrdflt: xcb_xkb_sa_set_ptr_dflt_t,
    pub isolock: xcb_xkb_sa_iso_lock_t,
    pub terminate: xcb_xkb_sa_terminate_t,
    pub switchscreen: xcb_xkb_sa_switch_screen_t,
    pub setcontrols: xcb_xkb_sa_set_controls_t,
    pub lockcontrols: xcb_xkb_sa_lock_controls_t,
    pub message: xcb_xkb_sa_action_message_t,
    pub redirect: xcb_xkb_sa_redirect_key_t,
    pub devbtn: xcb_xkb_sa_device_btn_t,
    pub lockdevbtn: xcb_xkb_sa_lock_device_btn_t,
    pub devval: xcb_xkb_sa_device_valuator_t,
    pub type_: u8,
}

impl Default for xcb_xkb_action_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// An iterator over `xkb::Action` objects.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_action_iterator_t {
    /// The value of the current iteration.
    pub data: *mut xcb_xkb_action_t,
    /// The number of elements remaining including this one.
    pub rem: c_int,
    /// The offset of `data`, in bytes, from the start of the containing object.
    pub index: c_int,
}

impl Default for xcb_xkb_action_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `xkb::UseExtension` request.
///
/// Pass this cookie to [`xcb_xkb_use_extension_reply`] to retrieve the reply.
///
/// [`xcb_xkb_use_extension_reply`]: XcbXkb::xcb_xkb_use_extension_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_use_extension_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xkb_use_extension_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::UseExtension` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXkb::xcb_xkb_id()`], then the type of the request is
/// [`xcb_xkb_use_extension_request_t`].
pub const XCB_XKB_USE_EXTENSION: u8 = 0i32 as u8;

/// The `xkb::UseExtension` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_use_extension_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub wanted_major: u16,
    pub wanted_minor: u16,
}

impl Default for xcb_xkb_use_extension_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::UseExtension` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_use_extension_reply_t {
    pub response_type: u8,
    pub supported: u8,
    pub sequence: u16,
    pub length: u32,
    pub server_major: u16,
    pub server_minor: u16,
    pub pad0: [u8; 20],
}

impl Default for xcb_xkb_use_extension_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::details` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_select_events_details_t {
    pub affect_new_keyboard: u16,
    pub new_keyboard_details: u16,
    pub affect_state: u16,
    pub state_details: u16,
    pub affect_ctrls: u32,
    pub ctrl_details: u32,
    pub affect_indicator_state: u32,
    pub indicator_state_details: u32,
    pub affect_indicator_map: u32,
    pub indicator_map_details: u32,
    pub affect_names: u16,
    pub names_details: u16,
    pub affect_compat: u8,
    pub compat_details: u8,
    pub affect_bell: u8,
    pub bell_details: u8,
    pub affect_msg_details: u8,
    pub msg_details: u8,
    pub affect_access_x: u16,
    pub access_xdetails: u16,
    pub affect_ext_dev: u16,
    pub extdev_details: u16,
}

impl Default for xcb_xkb_select_events_details_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::SelectEvents` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXkb::xcb_xkb_id()`], then the type of the request is
/// [`xcb_xkb_select_events_request_t`].
pub const XCB_XKB_SELECT_EVENTS: u8 = 1i32 as u8;

/// The `xkb::SelectEvents` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `details`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_select_events_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub affect_which: u16,
    pub clear: u16,
    pub select_all: u16,
    pub affect_map: u16,
    pub map: u16,
}

impl Default for xcb_xkb_select_events_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::Bell` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXkb::xcb_xkb_id()`], then the type of the request is
/// [`xcb_xkb_bell_request_t`].
pub const XCB_XKB_BELL: u8 = 3i32 as u8;

/// The `xkb::Bell` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_bell_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub bell_class: xcb_xkb_bell_class_spec_t,
    pub bell_id: xcb_xkb_id_spec_t,
    pub percent: i8,
    pub force_sound: u8,
    pub event_only: u8,
    pub pad0: u8,
    pub pitch: i16,
    pub duration: i16,
    pub pad1: [u8; 2],
    pub name: xcb_atom_t,
    pub window: xcb_window_t,
}

impl Default for xcb_xkb_bell_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `xkb::GetState` request.
///
/// Pass this cookie to [`xcb_xkb_get_state_reply`] to retrieve the reply.
///
/// [`xcb_xkb_get_state_reply`]: XcbXkb::xcb_xkb_get_state_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_state_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xkb_get_state_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::GetState` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXkb::xcb_xkb_id()`], then the type of the request is
/// [`xcb_xkb_get_state_request_t`].
pub const XCB_XKB_GET_STATE: u8 = 4i32 as u8;

/// The `xkb::GetState` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_state_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub pad0: [u8; 2],
}

impl Default for xcb_xkb_get_state_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::GetState` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_state_reply_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub mods: u8,
    pub base_mods: u8,
    pub latched_mods: u8,
    pub locked_mods: u8,
    pub group: u8,
    pub locked_group: u8,
    pub base_group: i16,
    pub latched_group: i16,
    pub compat_state: u8,
    pub grab_mods: u8,
    pub compat_grab_mods: u8,
    pub lookup_mods: u8,
    pub compat_lookup_mods: u8,
    pub pad0: u8,
    pub ptr_btn_state: u16,
    pub pad1: [u8; 6],
}

impl Default for xcb_xkb_get_state_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::LatchLockState` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXkb::xcb_xkb_id()`], then the type of the request is
/// [`xcb_xkb_latch_lock_state_request_t`].
pub const XCB_XKB_LATCH_LOCK_STATE: u8 = 5i32 as u8;

/// The `xkb::LatchLockState` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_latch_lock_state_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub affect_mod_locks: u8,
    pub mod_locks: u8,
    pub lock_group: u8,
    pub group_lock: u8,
    pub affect_mod_latches: u8,
    pub pad0: u8,
    pub pad1: u8,
    pub latch_group: u8,
    pub group_latch: u16,
}

impl Default for xcb_xkb_latch_lock_state_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `xkb::GetControls` request.
///
/// Pass this cookie to [`xcb_xkb_get_controls_reply`] to retrieve the reply.
///
/// [`xcb_xkb_get_controls_reply`]: XcbXkb::xcb_xkb_get_controls_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_controls_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xkb_get_controls_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::GetControls` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXkb::xcb_xkb_id()`], then the type of the request is
/// [`xcb_xkb_get_controls_request_t`].
pub const XCB_XKB_GET_CONTROLS: u8 = 6i32 as u8;

/// The `xkb::GetControls` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_controls_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub pad0: [u8; 2],
}

impl Default for xcb_xkb_get_controls_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::GetControls` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_controls_reply_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub mouse_keys_dflt_btn: u8,
    pub num_groups: u8,
    pub groups_wrap: u8,
    pub internal_mods_mask: u8,
    pub ignore_lock_mods_mask: u8,
    pub internal_mods_real_mods: u8,
    pub ignore_lock_mods_real_mods: u8,
    pub pad0: u8,
    pub internal_mods_vmods: u16,
    pub ignore_lock_mods_vmods: u16,
    pub repeat_delay: u16,
    pub repeat_interval: u16,
    pub slow_keys_delay: u16,
    pub debounce_delay: u16,
    pub mouse_keys_delay: u16,
    pub mouse_keys_interval: u16,
    pub mouse_keys_time_to_max: u16,
    pub mouse_keys_max_speed: u16,
    pub mouse_keys_curve: i16,
    pub access_xoption: u16,
    pub access_xtimeout: u16,
    pub access_xtimeout_options_mask: u16,
    pub access_xtimeout_options_values: u16,
    pub pad1: [u8; 2],
    pub access_xtimeout_mask: u32,
    pub access_xtimeout_values: u32,
    pub enabled_controls: u32,
    pub per_key_repeat: [u8; 32],
}

impl Default for xcb_xkb_get_controls_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::SetControls` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXkb::xcb_xkb_id()`], then the type of the request is
/// [`xcb_xkb_set_controls_request_t`].
pub const XCB_XKB_SET_CONTROLS: u8 = 7i32 as u8;

/// The `xkb::SetControls` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_controls_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub affect_internal_real_mods: u8,
    pub internal_real_mods: u8,
    pub affect_ignore_lock_real_mods: u8,
    pub ignore_lock_real_mods: u8,
    pub affect_internal_virtual_mods: u16,
    pub internal_virtual_mods: u16,
    pub affect_ignore_lock_virtual_mods: u16,
    pub ignore_lock_virtual_mods: u16,
    pub mouse_keys_dflt_btn: u8,
    pub groups_wrap: u8,
    pub access_xoptions: u16,
    pub pad0: [u8; 2],
    pub affect_enabled_controls: u32,
    pub enabled_controls: u32,
    pub change_controls: u32,
    pub repeat_delay: u16,
    pub repeat_interval: u16,
    pub slow_keys_delay: u16,
    pub debounce_delay: u16,
    pub mouse_keys_delay: u16,
    pub mouse_keys_interval: u16,
    pub mouse_keys_time_to_max: u16,
    pub mouse_keys_max_speed: u16,
    pub mouse_keys_curve: i16,
    pub access_xtimeout: u16,
    pub access_xtimeout_mask: u32,
    pub access_xtimeout_values: u32,
    pub access_xtimeout_options_mask: u16,
    pub access_xtimeout_options_values: u16,
    pub per_key_repeat: [u8; 32],
}

impl Default for xcb_xkb_set_controls_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `xkb::GetMap` request.
///
/// Pass this cookie to [`xcb_xkb_get_map_reply`] to retrieve the reply.
///
/// [`xcb_xkb_get_map_reply`]: XcbXkb::xcb_xkb_get_map_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_map_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xkb_get_map_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::GetMap` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXkb::xcb_xkb_id()`], then the type of the request is
/// [`xcb_xkb_get_map_request_t`].
pub const XCB_XKB_GET_MAP: u8 = 8i32 as u8;

/// The `xkb::GetMap` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_map_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub full: u16,
    pub partial: u16,
    pub first_type: u8,
    pub n_types: u8,
    pub first_key_sym: xcb_keycode_t,
    pub n_key_syms: u8,
    pub first_key_action: xcb_keycode_t,
    pub n_key_actions: u8,
    pub first_key_behavior: xcb_keycode_t,
    pub n_key_behaviors: u8,
    pub virtual_mods: u16,
    pub first_key_explicit: xcb_keycode_t,
    pub n_key_explicit: u8,
    pub first_mod_map_key: xcb_keycode_t,
    pub n_mod_map_keys: u8,
    pub first_vmod_map_key: xcb_keycode_t,
    pub n_vmod_map_keys: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_xkb_get_map_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::map` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_map_map_t {
    pub types_rtrn: *mut xcb_xkb_key_type_t,
    pub syms_rtrn: *mut xcb_xkb_key_sym_map_t,
    pub acts_rtrn_count: *mut u8,
    pub pad2: *mut u8,
    pub acts_rtrn_acts: *mut xcb_xkb_action_t,
    pub behaviors_rtrn: *mut xcb_xkb_set_behavior_t,
    pub vmods_rtrn: *mut u8,
    pub pad3: *mut u8,
    pub explicit_rtrn: *mut xcb_xkb_set_explicit_t,
    pub pad4: *mut u8,
    pub modmap_rtrn: *mut xcb_xkb_key_mod_map_t,
    pub pad5: *mut u8,
    pub vmodmap_rtrn: *mut xcb_xkb_key_v_mod_map_t,
}

impl Default for xcb_xkb_get_map_map_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::GetMap` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `map`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_map_reply_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad0: [u8; 2],
    pub min_key_code: xcb_keycode_t,
    pub max_key_code: xcb_keycode_t,
    pub present: u16,
    pub first_type: u8,
    pub n_types: u8,
    pub total_types: u8,
    pub first_key_sym: xcb_keycode_t,
    pub total_syms: u16,
    pub n_key_syms: u8,
    pub first_key_action: xcb_keycode_t,
    pub total_actions: u16,
    pub n_key_actions: u8,
    pub first_key_behavior: xcb_keycode_t,
    pub n_key_behaviors: u8,
    pub total_key_behaviors: u8,
    pub first_key_explicit: xcb_keycode_t,
    pub n_key_explicit: u8,
    pub total_key_explicit: u8,
    pub first_mod_map_key: xcb_keycode_t,
    pub n_mod_map_keys: u8,
    pub total_mod_map_keys: u8,
    pub first_vmod_map_key: xcb_keycode_t,
    pub n_vmod_map_keys: u8,
    pub total_vmod_map_keys: u8,
    pub pad1: u8,
    pub virtual_mods: u16,
}

impl Default for xcb_xkb_get_map_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::values` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_map_values_t {
    pub types: *mut xcb_xkb_set_key_type_t,
    pub syms: *mut xcb_xkb_key_sym_map_t,
    pub actions_count: *mut u8,
    pub actions: *mut xcb_xkb_action_t,
    pub behaviors: *mut xcb_xkb_set_behavior_t,
    pub vmods: *mut u8,
    pub explicit: *mut xcb_xkb_set_explicit_t,
    pub modmap: *mut xcb_xkb_key_mod_map_t,
    pub vmodmap: *mut xcb_xkb_key_v_mod_map_t,
}

impl Default for xcb_xkb_set_map_values_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::SetMap` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXkb::xcb_xkb_id()`], then the type of the request is
/// [`xcb_xkb_set_map_request_t`].
pub const XCB_XKB_SET_MAP: u8 = 9i32 as u8;

/// The `xkb::SetMap` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `values`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_map_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub present: u16,
    pub flags: u16,
    pub min_key_code: xcb_keycode_t,
    pub max_key_code: xcb_keycode_t,
    pub first_type: u8,
    pub n_types: u8,
    pub first_key_sym: xcb_keycode_t,
    pub n_key_syms: u8,
    pub total_syms: u16,
    pub first_key_action: xcb_keycode_t,
    pub n_key_actions: u8,
    pub total_actions: u16,
    pub first_key_behavior: xcb_keycode_t,
    pub n_key_behaviors: u8,
    pub total_key_behaviors: u8,
    pub first_key_explicit: xcb_keycode_t,
    pub n_key_explicit: u8,
    pub total_key_explicit: u8,
    pub first_mod_map_key: xcb_keycode_t,
    pub n_mod_map_keys: u8,
    pub total_mod_map_keys: u8,
    pub first_vmod_map_key: xcb_keycode_t,
    pub n_vmod_map_keys: u8,
    pub total_vmod_map_keys: u8,
    pub virtual_mods: u16,
}

impl Default for xcb_xkb_set_map_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `xkb::GetCompatMap` request.
///
/// Pass this cookie to [`xcb_xkb_get_compat_map_reply`] to retrieve the reply.
///
/// [`xcb_xkb_get_compat_map_reply`]: XcbXkb::xcb_xkb_get_compat_map_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_compat_map_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xkb_get_compat_map_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::GetCompatMap` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXkb::xcb_xkb_id()`], then the type of the request is
/// [`xcb_xkb_get_compat_map_request_t`].
pub const XCB_XKB_GET_COMPAT_MAP: u8 = 10i32 as u8;

/// The `xkb::GetCompatMap` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_compat_map_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub groups: u8,
    pub get_all_si: u8,
    pub first_si: u16,
    pub n_si: u16,
}

impl Default for xcb_xkb_get_compat_map_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::GetCompatMap` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `si_rtrn`
/// - `group_rtrn`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_compat_map_reply_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub groups_rtrn: u8,
    pub pad0: u8,
    pub first_sirtrn: u16,
    pub n_sirtrn: u16,
    pub n_total_si: u16,
    pub pad1: [u8; 16],
}

impl Default for xcb_xkb_get_compat_map_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::SetCompatMap` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXkb::xcb_xkb_id()`], then the type of the request is
/// [`xcb_xkb_set_compat_map_request_t`].
pub const XCB_XKB_SET_COMPAT_MAP: u8 = 11i32 as u8;

/// The `xkb::SetCompatMap` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `si`
/// - `group_maps`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_compat_map_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub pad0: u8,
    pub recompute_actions: u8,
    pub truncate_si: u8,
    pub groups: u8,
    pub first_si: u16,
    pub n_si: u16,
    pub pad1: [u8; 2],
}

impl Default for xcb_xkb_set_compat_map_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `xkb::GetIndicatorState` request.
///
/// Pass this cookie to [`xcb_xkb_get_indicator_state_reply`] to retrieve the reply.
///
/// [`xcb_xkb_get_indicator_state_reply`]: XcbXkb::xcb_xkb_get_indicator_state_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_indicator_state_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xkb_get_indicator_state_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::GetIndicatorState` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXkb::xcb_xkb_id()`], then the type of the request is
/// [`xcb_xkb_get_indicator_state_request_t`].
pub const XCB_XKB_GET_INDICATOR_STATE: u8 = 12i32 as u8;

/// The `xkb::GetIndicatorState` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_indicator_state_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub pad0: [u8; 2],
}

impl Default for xcb_xkb_get_indicator_state_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::GetIndicatorState` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_indicator_state_reply_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub state: u32,
    pub pad0: [u8; 20],
}

impl Default for xcb_xkb_get_indicator_state_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `xkb::GetIndicatorMap` request.
///
/// Pass this cookie to [`xcb_xkb_get_indicator_map_reply`] to retrieve the reply.
///
/// [`xcb_xkb_get_indicator_map_reply`]: XcbXkb::xcb_xkb_get_indicator_map_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_indicator_map_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xkb_get_indicator_map_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::GetIndicatorMap` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXkb::xcb_xkb_id()`], then the type of the request is
/// [`xcb_xkb_get_indicator_map_request_t`].
pub const XCB_XKB_GET_INDICATOR_MAP: u8 = 13i32 as u8;

/// The `xkb::GetIndicatorMap` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_indicator_map_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub pad0: [u8; 2],
    pub which: u32,
}

impl Default for xcb_xkb_get_indicator_map_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::GetIndicatorMap` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `maps`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_indicator_map_reply_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub which: u32,
    pub real_indicators: u32,
    pub n_indicators: u8,
    pub pad0: [u8; 15],
}

impl Default for xcb_xkb_get_indicator_map_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::SetIndicatorMap` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXkb::xcb_xkb_id()`], then the type of the request is
/// [`xcb_xkb_set_indicator_map_request_t`].
pub const XCB_XKB_SET_INDICATOR_MAP: u8 = 14i32 as u8;

/// The `xkb::SetIndicatorMap` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `maps`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_indicator_map_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub pad0: [u8; 2],
    pub which: u32,
}

impl Default for xcb_xkb_set_indicator_map_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `xkb::GetNamedIndicator` request.
///
/// Pass this cookie to [`xcb_xkb_get_named_indicator_reply`] to retrieve the reply.
///
/// [`xcb_xkb_get_named_indicator_reply`]: XcbXkb::xcb_xkb_get_named_indicator_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_named_indicator_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xkb_get_named_indicator_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::GetNamedIndicator` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXkb::xcb_xkb_id()`], then the type of the request is
/// [`xcb_xkb_get_named_indicator_request_t`].
pub const XCB_XKB_GET_NAMED_INDICATOR: u8 = 15i32 as u8;

/// The `xkb::GetNamedIndicator` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_named_indicator_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub led_class: xcb_xkb_led_class_spec_t,
    pub led_id: xcb_xkb_id_spec_t,
    pub pad0: [u8; 2],
    pub indicator: xcb_atom_t,
}

impl Default for xcb_xkb_get_named_indicator_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::GetNamedIndicator` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_named_indicator_reply_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub indicator: xcb_atom_t,
    pub found: u8,
    pub on: u8,
    pub real_indicator: u8,
    pub ndx: u8,
    pub map_flags: u8,
    pub map_which_groups: u8,
    pub map_groups: u8,
    pub map_which_mods: u8,
    pub map_mods: u8,
    pub map_real_mods: u8,
    pub map_vmod: u16,
    pub map_ctrls: u32,
    pub supported: u8,
    pub pad0: [u8; 3],
}

impl Default for xcb_xkb_get_named_indicator_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::SetNamedIndicator` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXkb::xcb_xkb_id()`], then the type of the request is
/// [`xcb_xkb_set_named_indicator_request_t`].
pub const XCB_XKB_SET_NAMED_INDICATOR: u8 = 16i32 as u8;

/// The `xkb::SetNamedIndicator` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_named_indicator_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub led_class: xcb_xkb_led_class_spec_t,
    pub led_id: xcb_xkb_id_spec_t,
    pub pad0: [u8; 2],
    pub indicator: xcb_atom_t,
    pub set_state: u8,
    pub on: u8,
    pub set_map: u8,
    pub create_map: u8,
    pub pad1: u8,
    pub map_flags: u8,
    pub map_which_groups: u8,
    pub map_groups: u8,
    pub map_which_mods: u8,
    pub map_real_mods: u8,
    pub map_vmods: u16,
    pub map_ctrls: u32,
}

impl Default for xcb_xkb_set_named_indicator_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `xkb::GetNames` request.
///
/// Pass this cookie to [`xcb_xkb_get_names_reply`] to retrieve the reply.
///
/// [`xcb_xkb_get_names_reply`]: XcbXkb::xcb_xkb_get_names_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_names_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xkb_get_names_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::GetNames` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXkb::xcb_xkb_id()`], then the type of the request is
/// [`xcb_xkb_get_names_request_t`].
pub const XCB_XKB_GET_NAMES: u8 = 17i32 as u8;

/// The `xkb::GetNames` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_names_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub pad0: [u8; 2],
    pub which: u32,
}

impl Default for xcb_xkb_get_names_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::valueList` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_names_value_list_t {
    pub keycodes_name: xcb_atom_t,
    pub geometry_name: xcb_atom_t,
    pub symbols_name: xcb_atom_t,
    pub phys_symbols_name: xcb_atom_t,
    pub types_name: xcb_atom_t,
    pub compat_name: xcb_atom_t,
    pub type_names: *mut xcb_atom_t,
    pub n_levels_per_type: *mut u8,
    pub pad1: *mut u8,
    pub kt_level_names: *mut xcb_atom_t,
    pub indicator_names: *mut xcb_atom_t,
    pub virtual_mod_names: *mut xcb_atom_t,
    pub groups: *mut xcb_atom_t,
    pub key_names: *mut xcb_xkb_key_name_t,
    pub key_aliases: *mut xcb_xkb_key_alias_t,
    pub radio_group_names: *mut xcb_atom_t,
}

impl Default for xcb_xkb_get_names_value_list_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::GetNames` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `value_list`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_names_reply_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub which: u32,
    pub min_key_code: xcb_keycode_t,
    pub max_key_code: xcb_keycode_t,
    pub n_types: u8,
    pub group_names: u8,
    pub virtual_mods: u16,
    pub first_key: xcb_keycode_t,
    pub n_keys: u8,
    pub indicators: u32,
    pub n_radio_groups: u8,
    pub n_key_aliases: u8,
    pub n_ktlevels: u16,
    pub pad0: [u8; 4],
}

impl Default for xcb_xkb_get_names_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::values` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_names_values_t {
    pub keycodes_name: xcb_atom_t,
    pub geometry_name: xcb_atom_t,
    pub symbols_name: xcb_atom_t,
    pub phys_symbols_name: xcb_atom_t,
    pub types_name: xcb_atom_t,
    pub compat_name: xcb_atom_t,
    pub type_names: *mut xcb_atom_t,
    pub n_levels_per_type: *mut u8,
    pub kt_level_names: *mut xcb_atom_t,
    pub indicator_names: *mut xcb_atom_t,
    pub virtual_mod_names: *mut xcb_atom_t,
    pub groups: *mut xcb_atom_t,
    pub key_names: *mut xcb_xkb_key_name_t,
    pub key_aliases: *mut xcb_xkb_key_alias_t,
    pub radio_group_names: *mut xcb_atom_t,
}

impl Default for xcb_xkb_set_names_values_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::SetNames` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXkb::xcb_xkb_id()`], then the type of the request is
/// [`xcb_xkb_set_names_request_t`].
pub const XCB_XKB_SET_NAMES: u8 = 18i32 as u8;

/// The `xkb::SetNames` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `values`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_names_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub virtual_mods: u16,
    pub which: u32,
    pub first_type: u8,
    pub n_types: u8,
    pub first_ktlevelt: u8,
    pub n_ktlevels: u8,
    pub indicators: u32,
    pub group_names: u8,
    pub n_radio_groups: u8,
    pub first_key: xcb_keycode_t,
    pub n_keys: u8,
    pub n_key_aliases: u8,
    pub pad0: u8,
    pub total_ktlevel_names: u16,
}

impl Default for xcb_xkb_set_names_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `xkb::PerClientFlags` request.
///
/// Pass this cookie to [`xcb_xkb_per_client_flags_reply`] to retrieve the reply.
///
/// [`xcb_xkb_per_client_flags_reply`]: XcbXkb::xcb_xkb_per_client_flags_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_per_client_flags_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xkb_per_client_flags_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::PerClientFlags` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXkb::xcb_xkb_id()`], then the type of the request is
/// [`xcb_xkb_per_client_flags_request_t`].
pub const XCB_XKB_PER_CLIENT_FLAGS: u8 = 21i32 as u8;

/// The `xkb::PerClientFlags` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_per_client_flags_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub pad0: [u8; 2],
    pub change: u32,
    pub value: u32,
    pub ctrls_to_change: u32,
    pub auto_ctrls: u32,
    pub auto_ctrls_values: u32,
}

impl Default for xcb_xkb_per_client_flags_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::PerClientFlags` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_per_client_flags_reply_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub supported: u32,
    pub value: u32,
    pub auto_ctrls: u32,
    pub auto_ctrls_values: u32,
    pub pad0: [u8; 8],
}

impl Default for xcb_xkb_per_client_flags_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `xkb::ListComponents` request.
///
/// Pass this cookie to [`xcb_xkb_list_components_reply`] to retrieve the reply.
///
/// [`xcb_xkb_list_components_reply`]: XcbXkb::xcb_xkb_list_components_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_list_components_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xkb_list_components_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::ListComponents` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXkb::xcb_xkb_id()`], then the type of the request is
/// [`xcb_xkb_list_components_request_t`].
pub const XCB_XKB_LIST_COMPONENTS: u8 = 22i32 as u8;

/// The `xkb::ListComponents` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_list_components_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub max_names: u16,
}

impl Default for xcb_xkb_list_components_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::ListComponents` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `keymaps`
/// - `keycodes`
/// - `types`
/// - `compat_maps`
/// - `symbols`
/// - `geometries`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_list_components_reply_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub n_keymaps: u16,
    pub n_keycodes: u16,
    pub n_types: u16,
    pub n_compat_maps: u16,
    pub n_symbols: u16,
    pub n_geometries: u16,
    pub extra: u16,
    pub pad0: [u8; 10],
}

impl Default for xcb_xkb_list_components_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `xkb::GetKbdByName` request.
///
/// Pass this cookie to [`xcb_xkb_get_kbd_by_name_reply`] to retrieve the reply.
///
/// [`xcb_xkb_get_kbd_by_name_reply`]: XcbXkb::xcb_xkb_get_kbd_by_name_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xkb_get_kbd_by_name_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::GetKbdByName` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXkb::xcb_xkb_id()`], then the type of the request is
/// [`xcb_xkb_get_kbd_by_name_request_t`].
pub const XCB_XKB_GET_KBD_BY_NAME: u8 = 23i32 as u8;

/// The `xkb::GetKbdByName` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub need: u16,
    pub want: u16,
    pub load: u8,
    pub pad0: u8,
}

impl Default for xcb_xkb_get_kbd_by_name_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::map` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_replies_types_map_t {
    pub types_rtrn: *mut xcb_xkb_key_type_t,
    pub syms_rtrn: *mut xcb_xkb_key_sym_map_t,
    pub acts_rtrn_count: *mut u8,
    pub acts_rtrn_acts: *mut xcb_xkb_action_t,
    pub behaviors_rtrn: *mut xcb_xkb_set_behavior_t,
    pub vmods_rtrn: *mut u8,
    pub explicit_rtrn: *mut xcb_xkb_set_explicit_t,
    pub modmap_rtrn: *mut xcb_xkb_key_mod_map_t,
    pub vmodmap_rtrn: *mut xcb_xkb_key_v_mod_map_t,
}

impl Default for xcb_xkb_get_kbd_by_name_replies_types_map_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::valueList` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t {
    pub keycodes_name: xcb_atom_t,
    pub geometry_name: xcb_atom_t,
    pub symbols_name: xcb_atom_t,
    pub phys_symbols_name: xcb_atom_t,
    pub types_name: xcb_atom_t,
    pub compat_name: xcb_atom_t,
    pub type_names: *mut xcb_atom_t,
    pub n_levels_per_type: *mut u8,
    pub kt_level_names: *mut xcb_atom_t,
    pub indicator_names: *mut xcb_atom_t,
    pub virtual_mod_names: *mut xcb_atom_t,
    pub groups: *mut xcb_atom_t,
    pub key_names: *mut xcb_xkb_key_name_t,
    pub key_aliases: *mut xcb_xkb_key_alias_t,
    pub radio_group_names: *mut xcb_atom_t,
}

impl Default for xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_xkb_get_kbd_by_name_replies_t::types`].
///
/// In libxcb, this type is an anonymous struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `map`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_replies_t__types {
    pub getmap_type: u8,
    pub type_device_id: u8,
    pub getmap_sequence: u16,
    pub getmap_length: u32,
    pub pad1: [u8; 2],
    pub type_min_key_code: xcb_keycode_t,
    pub type_max_key_code: xcb_keycode_t,
    pub present: u16,
    pub first_type: u8,
    pub n_types: u8,
    pub total_types: u8,
    pub first_key_sym: xcb_keycode_t,
    pub total_syms: u16,
    pub n_key_syms: u8,
    pub first_key_action: xcb_keycode_t,
    pub total_actions: u16,
    pub n_key_actions: u8,
    pub first_key_behavior: xcb_keycode_t,
    pub n_key_behaviors: u8,
    pub total_key_behaviors: u8,
    pub first_key_explicit: xcb_keycode_t,
    pub n_key_explicit: u8,
    pub total_key_explicit: u8,
    pub first_mod_map_key: xcb_keycode_t,
    pub n_mod_map_keys: u8,
    pub total_mod_map_keys: u8,
    pub first_vmod_map_key: xcb_keycode_t,
    pub n_vmod_map_keys: u8,
    pub total_vmod_map_keys: u8,
    pub pad2: u8,
    pub virtual_mods: u16,
    pub map: xcb_xkb_get_kbd_by_name_replies_types_map_t,
}

impl Default for xcb_xkb_get_kbd_by_name_replies_t__types {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_xkb_get_kbd_by_name_replies_t::compat_map`].
///
/// In libxcb, this type is an anonymous struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `si_rtrn`
/// - `group_rtrn`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_replies_t__compat_map {
    pub compatmap_type: u8,
    pub compat_device_id: u8,
    pub compatmap_sequence: u16,
    pub compatmap_length: u32,
    pub groups_rtrn: u8,
    pub pad7: u8,
    pub first_sirtrn: u16,
    pub n_sirtrn: u16,
    pub n_total_si: u16,
    pub pad8: [u8; 16],
    pub si_rtrn: *mut xcb_xkb_sym_interpret_t,
    pub group_rtrn: *mut xcb_xkb_mod_def_t,
}

impl Default for xcb_xkb_get_kbd_by_name_replies_t__compat_map {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_xkb_get_kbd_by_name_replies_t::indicator_maps`].
///
/// In libxcb, this type is an anonymous struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `maps`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_replies_t__indicator_maps {
    pub indicatormap_type: u8,
    pub indicator_device_id: u8,
    pub indicatormap_sequence: u16,
    pub indicatormap_length: u32,
    pub which: u32,
    pub real_indicators: u32,
    pub n_indicators: u8,
    pub pad9: [u8; 15],
    pub maps: *mut xcb_xkb_indicator_map_t,
}

impl Default for xcb_xkb_get_kbd_by_name_replies_t__indicator_maps {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_xkb_get_kbd_by_name_replies_t::key_names`].
///
/// In libxcb, this type is an anonymous struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `value_list`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_replies_t__key_names {
    pub keyname_type: u8,
    pub key_device_id: u8,
    pub keyname_sequence: u16,
    pub keyname_length: u32,
    pub which: u32,
    pub key_min_key_code: xcb_keycode_t,
    pub key_max_key_code: xcb_keycode_t,
    pub n_types: u8,
    pub group_names: u8,
    pub virtual_mods: u16,
    pub first_key: xcb_keycode_t,
    pub n_keys: u8,
    pub indicators: u32,
    pub n_radio_groups: u8,
    pub n_key_aliases: u8,
    pub n_ktlevels: u16,
    pub pad10: [u8; 4],
    pub value_list: xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t,
}

impl Default for xcb_xkb_get_kbd_by_name_replies_t__key_names {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The type of [`xcb_xkb_get_kbd_by_name_replies_t::geometry`].
///
/// In libxcb, this type is an anonymous struct.
///
/// The following fields can be accessed via accessor functions:
///
/// - `label_font`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_replies_t__geometry {
    pub geometry_type: u8,
    pub geometry_device_id: u8,
    pub geometry_sequence: u16,
    pub geometry_length: u32,
    pub name: xcb_atom_t,
    pub geometry_found: u8,
    pub pad12: u8,
    pub width_mm: u16,
    pub height_mm: u16,
    pub n_properties: u16,
    pub n_colors: u16,
    pub n_shapes: u16,
    pub n_sections: u16,
    pub n_doodads: u16,
    pub n_key_aliases: u16,
    pub base_color_ndx: u8,
    pub label_color_ndx: u8,
    pub label_font: *mut xcb_xkb_counted_string_16_t,
}

impl Default for xcb_xkb_get_kbd_by_name_replies_t__geometry {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::replies` switch.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_replies_t {
    pub types: xcb_xkb_get_kbd_by_name_replies_t__types,
    pub compat_map: xcb_xkb_get_kbd_by_name_replies_t__compat_map,
    pub indicator_maps: xcb_xkb_get_kbd_by_name_replies_t__indicator_maps,
    pub key_names: xcb_xkb_get_kbd_by_name_replies_t__key_names,
    pub geometry: xcb_xkb_get_kbd_by_name_replies_t__geometry,
}

impl Default for xcb_xkb_get_kbd_by_name_replies_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::GetKbdByName` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `replies`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_reply_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub min_key_code: xcb_keycode_t,
    pub max_key_code: xcb_keycode_t,
    pub loaded: u8,
    pub new_keyboard: u8,
    pub found: u16,
    pub reported: u16,
    pub pad0: [u8; 16],
}

impl Default for xcb_xkb_get_kbd_by_name_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `xkb::GetDeviceInfo` request.
///
/// Pass this cookie to [`xcb_xkb_get_device_info_reply`] to retrieve the reply.
///
/// [`xcb_xkb_get_device_info_reply`]: XcbXkb::xcb_xkb_get_device_info_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_device_info_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xkb_get_device_info_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::GetDeviceInfo` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXkb::xcb_xkb_id()`], then the type of the request is
/// [`xcb_xkb_get_device_info_request_t`].
pub const XCB_XKB_GET_DEVICE_INFO: u8 = 24i32 as u8;

/// The `xkb::GetDeviceInfo` request.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_device_info_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub wanted: u16,
    pub all_buttons: u8,
    pub first_button: u8,
    pub n_buttons: u8,
    pub pad0: u8,
    pub led_class: xcb_xkb_led_class_spec_t,
    pub led_id: xcb_xkb_id_spec_t,
}

impl Default for xcb_xkb_get_device_info_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::GetDeviceInfo` reply.
///
/// The following fields can be accessed via accessor functions:
///
/// - `name`
/// - `btn_actions`
/// - `leds`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_device_info_reply_t {
    pub response_type: u8,
    pub device_id: u8,
    pub sequence: u16,
    pub length: u32,
    pub present: u16,
    pub supported: u16,
    pub unsupported: u16,
    pub n_device_led_fbs: u16,
    pub first_btn_wanted: u8,
    pub n_btns_wanted: u8,
    pub first_btn_rtrn: u8,
    pub n_btns_rtrn: u8,
    pub total_btns: u8,
    pub has_own_state: u8,
    pub dflt_kbd_fb: u16,
    pub dflt_led_fb: u16,
    pub pad0: [u8; 2],
    pub dev_type: xcb_atom_t,
    pub name_len: u16,
}

impl Default for xcb_xkb_get_device_info_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::SetDeviceInfo` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXkb::xcb_xkb_id()`], then the type of the request is
/// [`xcb_xkb_set_device_info_request_t`].
pub const XCB_XKB_SET_DEVICE_INFO: u8 = 25i32 as u8;

/// The `xkb::SetDeviceInfo` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `btn_actions`
/// - `leds`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_device_info_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub first_btn: u8,
    pub n_btns: u8,
    pub change: u16,
    pub n_device_led_fbs: u16,
}

impl Default for xcb_xkb_set_device_info_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The cookie for the reply to a `xkb::SetDebuggingFlags` request.
///
/// Pass this cookie to [`xcb_xkb_set_debugging_flags_reply`] to retrieve the reply.
///
/// [`xcb_xkb_set_debugging_flags_reply`]: XcbXkb::xcb_xkb_set_debugging_flags_reply
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_debugging_flags_cookie_t {
    /// The sequence number of the request.
    pub sequence: c_uint,
}

impl Default for xcb_xkb_set_debugging_flags_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::SetDebuggingFlags` requests.
///
/// If this value appears in [`xcb_protocol_request_t::opcode`], and
/// [`xcb_protocol_request_t::ext`] is [`XcbXkb::xcb_xkb_id()`], then the type of the request is
/// [`xcb_xkb_set_debugging_flags_request_t`].
pub const XCB_XKB_SET_DEBUGGING_FLAGS: u8 = 101i32 as u8;

/// The `xkb::SetDebuggingFlags` request.
///
/// The following fields can be accessed via accessor functions:
///
/// - `message`
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_debugging_flags_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub msg_length: u16,
    pub pad0: [u8; 2],
    pub affect_flags: u32,
    pub flags: u32,
    pub affect_ctrls: u32,
    pub ctrls: u32,
}

impl Default for xcb_xkb_set_debugging_flags_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The `xkb::SetDebuggingFlags` reply.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_debugging_flags_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub current_flags: u32,
    pub current_ctrls: u32,
    pub supported_flags: u32,
    pub supported_ctrls: u32,
    pub pad1: [u8; 8],
}

impl Default for xcb_xkb_set_debugging_flags_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::NewKeyboardNotify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_xkb_new_keyboard_notify_event_t`].
pub const XCB_XKB_NEW_KEYBOARD_NOTIFY: u8 = 0i32 as u8;

/// The `xkb::NewKeyboardNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_new_keyboard_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_id: u8,
    pub old_device_id: u8,
    pub min_key_code: xcb_keycode_t,
    pub max_key_code: xcb_keycode_t,
    pub old_min_key_code: xcb_keycode_t,
    pub old_max_key_code: xcb_keycode_t,
    pub request_major: u8,
    pub request_minor: u8,
    pub changed: u16,
    pub pad0: [u8; 14],
}

impl Default for xcb_xkb_new_keyboard_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::MapNotify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_xkb_map_notify_event_t`].
pub const XCB_XKB_MAP_NOTIFY: u8 = 1i32 as u8;

/// The `xkb::MapNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_map_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_id: u8,
    pub ptr_btn_actions: u8,
    pub changed: u16,
    pub min_key_code: xcb_keycode_t,
    pub max_key_code: xcb_keycode_t,
    pub first_type: u8,
    pub n_types: u8,
    pub first_key_sym: xcb_keycode_t,
    pub n_key_syms: u8,
    pub first_key_act: xcb_keycode_t,
    pub n_key_acts: u8,
    pub first_key_behavior: xcb_keycode_t,
    pub n_key_behavior: u8,
    pub first_key_explicit: xcb_keycode_t,
    pub n_key_explicit: u8,
    pub first_mod_map_key: xcb_keycode_t,
    pub n_mod_map_keys: u8,
    pub first_vmod_map_key: xcb_keycode_t,
    pub n_vmod_map_keys: u8,
    pub virtual_mods: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_xkb_map_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::StateNotify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_xkb_state_notify_event_t`].
pub const XCB_XKB_STATE_NOTIFY: u8 = 2i32 as u8;

/// The `xkb::StateNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_state_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_id: u8,
    pub mods: u8,
    pub base_mods: u8,
    pub latched_mods: u8,
    pub locked_mods: u8,
    pub group: u8,
    pub base_group: i16,
    pub latched_group: i16,
    pub locked_group: u8,
    pub compat_state: u8,
    pub grab_mods: u8,
    pub compat_grab_mods: u8,
    pub lookup_mods: u8,
    pub compat_loockup_mods: u8,
    pub ptr_btn_state: u16,
    pub changed: u16,
    pub keycode: xcb_keycode_t,
    pub event_type: u8,
    pub request_major: u8,
    pub request_minor: u8,
}

impl Default for xcb_xkb_state_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::ControlsNotify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_xkb_controls_notify_event_t`].
pub const XCB_XKB_CONTROLS_NOTIFY: u8 = 3i32 as u8;

/// The `xkb::ControlsNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_controls_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_id: u8,
    pub num_groups: u8,
    pub pad0: [u8; 2],
    pub changed_controls: u32,
    pub enabled_controls: u32,
    pub enabled_control_changes: u32,
    pub keycode: xcb_keycode_t,
    pub event_type: u8,
    pub request_major: u8,
    pub request_minor: u8,
    pub pad1: [u8; 4],
}

impl Default for xcb_xkb_controls_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::IndicatorStateNotify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_xkb_indicator_state_notify_event_t`].
pub const XCB_XKB_INDICATOR_STATE_NOTIFY: u8 = 4i32 as u8;

/// The `xkb::IndicatorStateNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_indicator_state_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_id: u8,
    pub pad0: [u8; 3],
    pub state: u32,
    pub state_changed: u32,
    pub pad1: [u8; 12],
}

impl Default for xcb_xkb_indicator_state_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::IndicatorMapNotify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_xkb_indicator_map_notify_event_t`].
pub const XCB_XKB_INDICATOR_MAP_NOTIFY: u8 = 5i32 as u8;

/// The `xkb::IndicatorMapNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_indicator_map_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_id: u8,
    pub pad0: [u8; 3],
    pub state: u32,
    pub map_changed: u32,
    pub pad1: [u8; 12],
}

impl Default for xcb_xkb_indicator_map_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::NamesNotify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_xkb_names_notify_event_t`].
pub const XCB_XKB_NAMES_NOTIFY: u8 = 6i32 as u8;

/// The `xkb::NamesNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_names_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_id: u8,
    pub pad0: u8,
    pub changed: u16,
    pub first_type: u8,
    pub n_types: u8,
    pub first_level_name: u8,
    pub n_level_names: u8,
    pub pad1: u8,
    pub n_radio_groups: u8,
    pub n_key_aliases: u8,
    pub changed_group_names: u8,
    pub changed_virtual_mods: u16,
    pub first_key: xcb_keycode_t,
    pub n_keys: u8,
    pub changed_indicators: u32,
    pub pad2: [u8; 4],
}

impl Default for xcb_xkb_names_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::CompatMapNotify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_xkb_compat_map_notify_event_t`].
pub const XCB_XKB_COMPAT_MAP_NOTIFY: u8 = 7i32 as u8;

/// The `xkb::CompatMapNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_compat_map_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_id: u8,
    pub changed_groups: u8,
    pub first_si: u16,
    pub n_si: u16,
    pub n_total_si: u16,
    pub pad0: [u8; 16],
}

impl Default for xcb_xkb_compat_map_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::BellNotify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_xkb_bell_notify_event_t`].
pub const XCB_XKB_BELL_NOTIFY: u8 = 8i32 as u8;

/// The `xkb::BellNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_bell_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_id: u8,
    pub bell_class: u8,
    pub bell_id: u8,
    pub percent: u8,
    pub pitch: u16,
    pub duration: u16,
    pub name: xcb_atom_t,
    pub window: xcb_window_t,
    pub event_only: u8,
    pub pad0: [u8; 7],
}

impl Default for xcb_xkb_bell_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::ActionMessage` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_xkb_action_message_event_t`].
pub const XCB_XKB_ACTION_MESSAGE: u8 = 9i32 as u8;

/// The `xkb::ActionMessage` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_action_message_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_id: u8,
    pub keycode: xcb_keycode_t,
    pub press: u8,
    pub key_event_follows: u8,
    pub mods: u8,
    pub group: u8,
    pub message: [xcb_xkb_string8_t; 8],
    pub pad0: [u8; 10],
}

impl Default for xcb_xkb_action_message_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::AccessXNotify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_xkb_access_x_notify_event_t`].
pub const XCB_XKB_ACCESS_X_NOTIFY: u8 = 10i32 as u8;

/// The `xkb::AccessXNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_access_x_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_id: u8,
    pub keycode: xcb_keycode_t,
    pub detailt: u16,
    pub slow_keys_delay: u16,
    pub debounce_delay: u16,
    pub pad0: [u8; 16],
}

impl Default for xcb_xkb_access_x_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// The opcode for `xkb::ExtensionDeviceNotify` events.
///
/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],
/// then the type of the event is [`xcb_xkb_extension_device_notify_event_t`].
pub const XCB_XKB_EXTENSION_DEVICE_NOTIFY: u8 = 11i32 as u8;

/// The `xkb::ExtensionDeviceNotify` event.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_extension_device_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_id: u8,
    pub pad0: u8,
    pub reason: u16,
    pub led_class: u16,
    pub led_id: u16,
    pub leds_defined: u32,
    pub led_state: u32,
    pub first_button: u8,
    pub n_buttons: u8,
    pub supported: u16,
    pub unsupported: u16,
    pub pad1: [u8; 2],
}

impl Default for xcb_xkb_extension_device_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[cfg(feature = "xcb_xkb")]
pub(crate) struct XcbXkbXkb {
    xcb_xkb_id: LazySymbol<*mut xcb_extension_t>,
    xcb_xkb_device_spec_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_device_spec_iterator_t)>,
    xcb_xkb_device_spec_end:
        LazySymbol<unsafe fn(i: xcb_xkb_device_spec_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_led_class_spec_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_led_class_spec_iterator_t)>,
    xcb_xkb_led_class_spec_end:
        LazySymbol<unsafe fn(i: xcb_xkb_led_class_spec_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_bell_class_spec_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_bell_class_spec_iterator_t)>,
    xcb_xkb_bell_class_spec_end:
        LazySymbol<unsafe fn(i: xcb_xkb_bell_class_spec_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_id_spec_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_id_spec_iterator_t)>,
    xcb_xkb_id_spec_end:
        LazySymbol<unsafe fn(i: xcb_xkb_id_spec_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_indicator_map_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_indicator_map_iterator_t)>,
    xcb_xkb_indicator_map_end:
        LazySymbol<unsafe fn(i: xcb_xkb_indicator_map_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_mod_def_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_mod_def_iterator_t)>,
    xcb_xkb_mod_def_end:
        LazySymbol<unsafe fn(i: xcb_xkb_mod_def_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_key_name_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_key_name_iterator_t)>,
    xcb_xkb_key_name_end:
        LazySymbol<unsafe fn(i: xcb_xkb_key_name_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_key_alias_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_key_alias_iterator_t)>,
    xcb_xkb_key_alias_end:
        LazySymbol<unsafe fn(i: xcb_xkb_key_alias_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_counted_string_16_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xkb_counted_string_16_string:
        LazySymbol<unsafe fn(r: *const xcb_xkb_counted_string_16_t) -> *mut c_char>,
    xcb_xkb_counted_string_16_string_length:
        LazySymbol<unsafe fn(r: *const xcb_xkb_counted_string_16_t) -> c_int>,
    xcb_xkb_counted_string_16_string_end:
        LazySymbol<unsafe fn(r: *const xcb_xkb_counted_string_16_t) -> xcb_generic_iterator_t>,
    xcb_xkb_counted_string_16_alignment_pad:
        LazySymbol<unsafe fn(r: *const xcb_xkb_counted_string_16_t) -> *mut c_void>,
    xcb_xkb_counted_string_16_alignment_pad_length:
        LazySymbol<unsafe fn(r: *const xcb_xkb_counted_string_16_t) -> c_int>,
    xcb_xkb_counted_string_16_alignment_pad_end:
        LazySymbol<unsafe fn(r: *const xcb_xkb_counted_string_16_t) -> xcb_generic_iterator_t>,
    xcb_xkb_counted_string_16_next:
        LazySymbol<unsafe fn(i: *mut xcb_xkb_counted_string_16_iterator_t)>,
    xcb_xkb_counted_string_16_end:
        LazySymbol<unsafe fn(i: xcb_xkb_counted_string_16_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_kt_map_entry_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_kt_map_entry_iterator_t)>,
    xcb_xkb_kt_map_entry_end:
        LazySymbol<unsafe fn(i: xcb_xkb_kt_map_entry_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_key_type_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xkb_key_type_map:
        LazySymbol<unsafe fn(r: *const xcb_xkb_key_type_t) -> *mut xcb_xkb_kt_map_entry_t>,
    xcb_xkb_key_type_map_length: LazySymbol<unsafe fn(r: *const xcb_xkb_key_type_t) -> c_int>,
    xcb_xkb_key_type_map_iterator:
        LazySymbol<unsafe fn(r: *const xcb_xkb_key_type_t) -> xcb_xkb_kt_map_entry_iterator_t>,
    xcb_xkb_key_type_preserve:
        LazySymbol<unsafe fn(r: *const xcb_xkb_key_type_t) -> *mut xcb_xkb_mod_def_t>,
    xcb_xkb_key_type_preserve_length: LazySymbol<unsafe fn(r: *const xcb_xkb_key_type_t) -> c_int>,
    xcb_xkb_key_type_preserve_iterator:
        LazySymbol<unsafe fn(r: *const xcb_xkb_key_type_t) -> xcb_xkb_mod_def_iterator_t>,
    xcb_xkb_key_type_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_key_type_iterator_t)>,
    xcb_xkb_key_type_end:
        LazySymbol<unsafe fn(i: xcb_xkb_key_type_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_key_sym_map_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xkb_key_sym_map_syms:
        LazySymbol<unsafe fn(r: *const xcb_xkb_key_sym_map_t) -> *mut xcb_keysym_t>,
    xcb_xkb_key_sym_map_syms_length:
        LazySymbol<unsafe fn(r: *const xcb_xkb_key_sym_map_t) -> c_int>,
    xcb_xkb_key_sym_map_syms_end:
        LazySymbol<unsafe fn(r: *const xcb_xkb_key_sym_map_t) -> xcb_generic_iterator_t>,
    xcb_xkb_key_sym_map_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_key_sym_map_iterator_t)>,
    xcb_xkb_key_sym_map_end:
        LazySymbol<unsafe fn(i: xcb_xkb_key_sym_map_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_common_behavior_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_common_behavior_iterator_t)>,
    xcb_xkb_common_behavior_end:
        LazySymbol<unsafe fn(i: xcb_xkb_common_behavior_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_default_behavior_next:
        LazySymbol<unsafe fn(i: *mut xcb_xkb_default_behavior_iterator_t)>,
    xcb_xkb_default_behavior_end:
        LazySymbol<unsafe fn(i: xcb_xkb_default_behavior_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_lock_behavior_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_lock_behavior_iterator_t)>,
    xcb_xkb_lock_behavior_end:
        LazySymbol<unsafe fn(i: xcb_xkb_lock_behavior_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_radio_group_behavior_next:
        LazySymbol<unsafe fn(i: *mut xcb_xkb_radio_group_behavior_iterator_t)>,
    xcb_xkb_radio_group_behavior_end:
        LazySymbol<unsafe fn(i: xcb_xkb_radio_group_behavior_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_overlay_behavior_next:
        LazySymbol<unsafe fn(i: *mut xcb_xkb_overlay_behavior_iterator_t)>,
    xcb_xkb_overlay_behavior_end:
        LazySymbol<unsafe fn(i: xcb_xkb_overlay_behavior_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_permament_lock_behavior_next:
        LazySymbol<unsafe fn(i: *mut xcb_xkb_permament_lock_behavior_iterator_t)>,
    xcb_xkb_permament_lock_behavior_end: LazySymbol<
        unsafe fn(i: xcb_xkb_permament_lock_behavior_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_permament_radio_group_behavior_next:
        LazySymbol<unsafe fn(i: *mut xcb_xkb_permament_radio_group_behavior_iterator_t)>,
    xcb_xkb_permament_radio_group_behavior_end: LazySymbol<
        unsafe fn(i: xcb_xkb_permament_radio_group_behavior_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_permament_overlay_behavior_next:
        LazySymbol<unsafe fn(i: *mut xcb_xkb_permament_overlay_behavior_iterator_t)>,
    xcb_xkb_permament_overlay_behavior_end: LazySymbol<
        unsafe fn(i: xcb_xkb_permament_overlay_behavior_iterator_t) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_behavior_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_behavior_iterator_t)>,
    xcb_xkb_behavior_end:
        LazySymbol<unsafe fn(i: xcb_xkb_behavior_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_set_behavior_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_set_behavior_iterator_t)>,
    xcb_xkb_set_behavior_end:
        LazySymbol<unsafe fn(i: xcb_xkb_set_behavior_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_set_explicit_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_set_explicit_iterator_t)>,
    xcb_xkb_set_explicit_end:
        LazySymbol<unsafe fn(i: xcb_xkb_set_explicit_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_key_mod_map_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_key_mod_map_iterator_t)>,
    xcb_xkb_key_mod_map_end:
        LazySymbol<unsafe fn(i: xcb_xkb_key_mod_map_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_key_v_mod_map_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_key_v_mod_map_iterator_t)>,
    xcb_xkb_key_v_mod_map_end:
        LazySymbol<unsafe fn(i: xcb_xkb_key_v_mod_map_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_kt_set_map_entry_next:
        LazySymbol<unsafe fn(i: *mut xcb_xkb_kt_set_map_entry_iterator_t)>,
    xcb_xkb_kt_set_map_entry_end:
        LazySymbol<unsafe fn(i: xcb_xkb_kt_set_map_entry_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_set_key_type_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xkb_set_key_type_entries:
        LazySymbol<unsafe fn(r: *const xcb_xkb_set_key_type_t) -> *mut xcb_xkb_kt_set_map_entry_t>,
    xcb_xkb_set_key_type_entries_length:
        LazySymbol<unsafe fn(r: *const xcb_xkb_set_key_type_t) -> c_int>,
    xcb_xkb_set_key_type_entries_iterator: LazySymbol<
        unsafe fn(r: *const xcb_xkb_set_key_type_t) -> xcb_xkb_kt_set_map_entry_iterator_t,
    >,
    xcb_xkb_set_key_type_preserve_entries:
        LazySymbol<unsafe fn(r: *const xcb_xkb_set_key_type_t) -> *mut xcb_xkb_kt_set_map_entry_t>,
    xcb_xkb_set_key_type_preserve_entries_length:
        LazySymbol<unsafe fn(r: *const xcb_xkb_set_key_type_t) -> c_int>,
    xcb_xkb_set_key_type_preserve_entries_iterator: LazySymbol<
        unsafe fn(r: *const xcb_xkb_set_key_type_t) -> xcb_xkb_kt_set_map_entry_iterator_t,
    >,
    xcb_xkb_set_key_type_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_set_key_type_iterator_t)>,
    xcb_xkb_set_key_type_end:
        LazySymbol<unsafe fn(i: xcb_xkb_set_key_type_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_string8_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_string8_iterator_t)>,
    xcb_xkb_string8_end:
        LazySymbol<unsafe fn(i: xcb_xkb_string8_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_outline_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xkb_outline_points: LazySymbol<unsafe fn(r: *const xcb_xkb_outline_t) -> *mut xcb_point_t>,
    xcb_xkb_outline_points_length: LazySymbol<unsafe fn(r: *const xcb_xkb_outline_t) -> c_int>,
    xcb_xkb_outline_points_iterator:
        LazySymbol<unsafe fn(r: *const xcb_xkb_outline_t) -> xcb_point_iterator_t>,
    xcb_xkb_outline_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_outline_iterator_t)>,
    xcb_xkb_outline_end:
        LazySymbol<unsafe fn(i: xcb_xkb_outline_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_shape_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xkb_shape_outlines_length: LazySymbol<unsafe fn(r: *const xcb_xkb_shape_t) -> c_int>,
    xcb_xkb_shape_outlines_iterator:
        LazySymbol<unsafe fn(r: *const xcb_xkb_shape_t) -> xcb_xkb_outline_iterator_t>,
    xcb_xkb_shape_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_shape_iterator_t)>,
    xcb_xkb_shape_end: LazySymbol<unsafe fn(i: xcb_xkb_shape_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_key_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_key_iterator_t)>,
    xcb_xkb_key_end: LazySymbol<unsafe fn(i: xcb_xkb_key_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_overlay_key_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_overlay_key_iterator_t)>,
    xcb_xkb_overlay_key_end:
        LazySymbol<unsafe fn(i: xcb_xkb_overlay_key_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_overlay_row_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xkb_overlay_row_keys:
        LazySymbol<unsafe fn(r: *const xcb_xkb_overlay_row_t) -> *mut xcb_xkb_overlay_key_t>,
    xcb_xkb_overlay_row_keys_length:
        LazySymbol<unsafe fn(r: *const xcb_xkb_overlay_row_t) -> c_int>,
    xcb_xkb_overlay_row_keys_iterator:
        LazySymbol<unsafe fn(r: *const xcb_xkb_overlay_row_t) -> xcb_xkb_overlay_key_iterator_t>,
    xcb_xkb_overlay_row_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_overlay_row_iterator_t)>,
    xcb_xkb_overlay_row_end:
        LazySymbol<unsafe fn(i: xcb_xkb_overlay_row_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_overlay_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xkb_overlay_rows_length: LazySymbol<unsafe fn(r: *const xcb_xkb_overlay_t) -> c_int>,
    xcb_xkb_overlay_rows_iterator:
        LazySymbol<unsafe fn(r: *const xcb_xkb_overlay_t) -> xcb_xkb_overlay_row_iterator_t>,
    xcb_xkb_overlay_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_overlay_iterator_t)>,
    xcb_xkb_overlay_end:
        LazySymbol<unsafe fn(i: xcb_xkb_overlay_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_row_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xkb_row_keys: LazySymbol<unsafe fn(r: *const xcb_xkb_row_t) -> *mut xcb_xkb_key_t>,
    xcb_xkb_row_keys_length: LazySymbol<unsafe fn(r: *const xcb_xkb_row_t) -> c_int>,
    xcb_xkb_row_keys_iterator:
        LazySymbol<unsafe fn(r: *const xcb_xkb_row_t) -> xcb_xkb_key_iterator_t>,
    xcb_xkb_row_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_row_iterator_t)>,
    xcb_xkb_row_end: LazySymbol<unsafe fn(i: xcb_xkb_row_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_listing_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xkb_listing_string:
        LazySymbol<unsafe fn(r: *const xcb_xkb_listing_t) -> *mut xcb_xkb_string8_t>,
    xcb_xkb_listing_string_length: LazySymbol<unsafe fn(r: *const xcb_xkb_listing_t) -> c_int>,
    xcb_xkb_listing_string_end:
        LazySymbol<unsafe fn(r: *const xcb_xkb_listing_t) -> xcb_generic_iterator_t>,
    xcb_xkb_listing_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_listing_iterator_t)>,
    xcb_xkb_listing_end:
        LazySymbol<unsafe fn(i: xcb_xkb_listing_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_device_led_info_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xkb_device_led_info_names:
        LazySymbol<unsafe fn(r: *const xcb_xkb_device_led_info_t) -> *mut xcb_atom_t>,
    xcb_xkb_device_led_info_names_length:
        LazySymbol<unsafe fn(r: *const xcb_xkb_device_led_info_t) -> c_int>,
    xcb_xkb_device_led_info_names_end:
        LazySymbol<unsafe fn(r: *const xcb_xkb_device_led_info_t) -> xcb_generic_iterator_t>,
    xcb_xkb_device_led_info_maps:
        LazySymbol<unsafe fn(r: *const xcb_xkb_device_led_info_t) -> *mut xcb_xkb_indicator_map_t>,
    xcb_xkb_device_led_info_maps_length:
        LazySymbol<unsafe fn(r: *const xcb_xkb_device_led_info_t) -> c_int>,
    xcb_xkb_device_led_info_maps_iterator: LazySymbol<
        unsafe fn(r: *const xcb_xkb_device_led_info_t) -> xcb_xkb_indicator_map_iterator_t,
    >,
    xcb_xkb_device_led_info_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_device_led_info_iterator_t)>,
    xcb_xkb_device_led_info_end:
        LazySymbol<unsafe fn(i: xcb_xkb_device_led_info_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_sa_no_action_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_sa_no_action_iterator_t)>,
    xcb_xkb_sa_no_action_end:
        LazySymbol<unsafe fn(i: xcb_xkb_sa_no_action_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_sa_set_mods_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_sa_set_mods_iterator_t)>,
    xcb_xkb_sa_set_mods_end:
        LazySymbol<unsafe fn(i: xcb_xkb_sa_set_mods_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_sa_latch_mods_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_sa_latch_mods_iterator_t)>,
    xcb_xkb_sa_latch_mods_end:
        LazySymbol<unsafe fn(i: xcb_xkb_sa_latch_mods_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_sa_lock_mods_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_sa_lock_mods_iterator_t)>,
    xcb_xkb_sa_lock_mods_end:
        LazySymbol<unsafe fn(i: xcb_xkb_sa_lock_mods_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_sa_set_group_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_sa_set_group_iterator_t)>,
    xcb_xkb_sa_set_group_end:
        LazySymbol<unsafe fn(i: xcb_xkb_sa_set_group_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_sa_latch_group_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_sa_latch_group_iterator_t)>,
    xcb_xkb_sa_latch_group_end:
        LazySymbol<unsafe fn(i: xcb_xkb_sa_latch_group_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_sa_lock_group_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_sa_lock_group_iterator_t)>,
    xcb_xkb_sa_lock_group_end:
        LazySymbol<unsafe fn(i: xcb_xkb_sa_lock_group_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_sa_move_ptr_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_sa_move_ptr_iterator_t)>,
    xcb_xkb_sa_move_ptr_end:
        LazySymbol<unsafe fn(i: xcb_xkb_sa_move_ptr_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_sa_ptr_btn_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_sa_ptr_btn_iterator_t)>,
    xcb_xkb_sa_ptr_btn_end:
        LazySymbol<unsafe fn(i: xcb_xkb_sa_ptr_btn_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_sa_lock_ptr_btn_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_sa_lock_ptr_btn_iterator_t)>,
    xcb_xkb_sa_lock_ptr_btn_end:
        LazySymbol<unsafe fn(i: xcb_xkb_sa_lock_ptr_btn_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_sa_set_ptr_dflt_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_sa_set_ptr_dflt_iterator_t)>,
    xcb_xkb_sa_set_ptr_dflt_end:
        LazySymbol<unsafe fn(i: xcb_xkb_sa_set_ptr_dflt_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_sa_iso_lock_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_sa_iso_lock_iterator_t)>,
    xcb_xkb_sa_iso_lock_end:
        LazySymbol<unsafe fn(i: xcb_xkb_sa_iso_lock_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_sa_terminate_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_sa_terminate_iterator_t)>,
    xcb_xkb_sa_terminate_end:
        LazySymbol<unsafe fn(i: xcb_xkb_sa_terminate_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_sa_switch_screen_next:
        LazySymbol<unsafe fn(i: *mut xcb_xkb_sa_switch_screen_iterator_t)>,
    xcb_xkb_sa_switch_screen_end:
        LazySymbol<unsafe fn(i: xcb_xkb_sa_switch_screen_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_sa_set_controls_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_sa_set_controls_iterator_t)>,
    xcb_xkb_sa_set_controls_end:
        LazySymbol<unsafe fn(i: xcb_xkb_sa_set_controls_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_sa_lock_controls_next:
        LazySymbol<unsafe fn(i: *mut xcb_xkb_sa_lock_controls_iterator_t)>,
    xcb_xkb_sa_lock_controls_end:
        LazySymbol<unsafe fn(i: xcb_xkb_sa_lock_controls_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_sa_action_message_next:
        LazySymbol<unsafe fn(i: *mut xcb_xkb_sa_action_message_iterator_t)>,
    xcb_xkb_sa_action_message_end:
        LazySymbol<unsafe fn(i: xcb_xkb_sa_action_message_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_sa_redirect_key_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_sa_redirect_key_iterator_t)>,
    xcb_xkb_sa_redirect_key_end:
        LazySymbol<unsafe fn(i: xcb_xkb_sa_redirect_key_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_sa_device_btn_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_sa_device_btn_iterator_t)>,
    xcb_xkb_sa_device_btn_end:
        LazySymbol<unsafe fn(i: xcb_xkb_sa_device_btn_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_sa_lock_device_btn_next:
        LazySymbol<unsafe fn(i: *mut xcb_xkb_sa_lock_device_btn_iterator_t)>,
    xcb_xkb_sa_lock_device_btn_end:
        LazySymbol<unsafe fn(i: xcb_xkb_sa_lock_device_btn_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_sa_device_valuator_next:
        LazySymbol<unsafe fn(i: *mut xcb_xkb_sa_device_valuator_iterator_t)>,
    xcb_xkb_sa_device_valuator_end:
        LazySymbol<unsafe fn(i: xcb_xkb_sa_device_valuator_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_si_action_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_si_action_iterator_t)>,
    xcb_xkb_si_action_end:
        LazySymbol<unsafe fn(i: xcb_xkb_si_action_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_sym_interpret_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_sym_interpret_iterator_t)>,
    xcb_xkb_sym_interpret_end:
        LazySymbol<unsafe fn(i: xcb_xkb_sym_interpret_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_action_next: LazySymbol<unsafe fn(i: *mut xcb_xkb_action_iterator_t)>,
    xcb_xkb_action_end:
        LazySymbol<unsafe fn(i: xcb_xkb_action_iterator_t) -> xcb_generic_iterator_t>,
    xcb_xkb_use_extension: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            wanted_major: u16,
            wanted_minor: u16,
        ) -> xcb_xkb_use_extension_cookie_t,
    >,
    xcb_xkb_use_extension_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            wanted_major: u16,
            wanted_minor: u16,
        ) -> xcb_xkb_use_extension_cookie_t,
    >,
    xcb_xkb_use_extension_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xkb_use_extension_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xkb_use_extension_reply_t,
    >,
    xcb_xkb_select_events_details_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            affectWhich: u16,
            clear: u16,
            selectAll: u16,
            _aux: *const xcb_xkb_select_events_details_t,
        ) -> c_int,
    >,
    xcb_xkb_select_events_details_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            affectWhich: u16,
            clear: u16,
            selectAll: u16,
            _aux: *mut xcb_xkb_select_events_details_t,
        ) -> c_int,
    >,
    xcb_xkb_select_events_details_sizeof: LazySymbol<
        unsafe fn(_buffer: *const c_void, affectWhich: u16, clear: u16, selectAll: u16) -> c_int,
    >,
    xcb_xkb_select_events_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xkb_select_events_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            affect_which: u16,
            clear: u16,
            select_all: u16,
            affect_map: u16,
            map: u16,
            details: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xkb_select_events: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            affect_which: u16,
            clear: u16,
            select_all: u16,
            affect_map: u16,
            map: u16,
            details: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xkb_select_events_aux_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            affect_which: u16,
            clear: u16,
            select_all: u16,
            affect_map: u16,
            map: u16,
            details: *const xcb_xkb_select_events_details_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xkb_select_events_aux: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            affect_which: u16,
            clear: u16,
            select_all: u16,
            affect_map: u16,
            map: u16,
            details: *const xcb_xkb_select_events_details_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xkb_select_events_details:
        LazySymbol<unsafe fn(r: *const xcb_xkb_select_events_request_t) -> *mut c_void>,
    xcb_xkb_bell_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            bell_class: xcb_xkb_bell_class_spec_t,
            bell_id: xcb_xkb_id_spec_t,
            percent: i8,
            force_sound: u8,
            event_only: u8,
            pitch: i16,
            duration: i16,
            name: xcb_atom_t,
            window: xcb_window_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xkb_bell: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            bell_class: xcb_xkb_bell_class_spec_t,
            bell_id: xcb_xkb_id_spec_t,
            percent: i8,
            force_sound: u8,
            event_only: u8,
            pitch: i16,
            duration: i16,
            name: xcb_atom_t,
            window: xcb_window_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xkb_get_state: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
        ) -> xcb_xkb_get_state_cookie_t,
    >,
    xcb_xkb_get_state_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
        ) -> xcb_xkb_get_state_cookie_t,
    >,
    xcb_xkb_get_state_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xkb_get_state_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xkb_get_state_reply_t,
    >,
    xcb_xkb_latch_lock_state_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            affect_mod_locks: u8,
            mod_locks: u8,
            lock_group: u8,
            group_lock: u8,
            affect_mod_latches: u8,
            latch_group: u8,
            group_latch: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xkb_latch_lock_state: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            affect_mod_locks: u8,
            mod_locks: u8,
            lock_group: u8,
            group_lock: u8,
            affect_mod_latches: u8,
            latch_group: u8,
            group_latch: u16,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xkb_get_controls: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
        ) -> xcb_xkb_get_controls_cookie_t,
    >,
    xcb_xkb_get_controls_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
        ) -> xcb_xkb_get_controls_cookie_t,
    >,
    xcb_xkb_get_controls_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xkb_get_controls_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xkb_get_controls_reply_t,
    >,
    xcb_xkb_set_controls_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            affect_internal_real_mods: u8,
            internal_real_mods: u8,
            affect_ignore_lock_real_mods: u8,
            ignore_lock_real_mods: u8,
            affect_internal_virtual_mods: u16,
            internal_virtual_mods: u16,
            affect_ignore_lock_virtual_mods: u16,
            ignore_lock_virtual_mods: u16,
            mouse_keys_dflt_btn: u8,
            groups_wrap: u8,
            access_xoptions: u16,
            affect_enabled_controls: u32,
            enabled_controls: u32,
            change_controls: u32,
            repeat_delay: u16,
            repeat_interval: u16,
            slow_keys_delay: u16,
            debounce_delay: u16,
            mouse_keys_delay: u16,
            mouse_keys_interval: u16,
            mouse_keys_time_to_max: u16,
            mouse_keys_max_speed: u16,
            mouse_keys_curve: i16,
            access_xtimeout: u16,
            access_xtimeout_mask: u32,
            access_xtimeout_values: u32,
            access_xtimeout_options_mask: u16,
            access_xtimeout_options_values: u16,
            per_key_repeat: *const [u8; 32],
        ) -> xcb_void_cookie_t,
    >,
    xcb_xkb_set_controls: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            affect_internal_real_mods: u8,
            internal_real_mods: u8,
            affect_ignore_lock_real_mods: u8,
            ignore_lock_real_mods: u8,
            affect_internal_virtual_mods: u16,
            internal_virtual_mods: u16,
            affect_ignore_lock_virtual_mods: u16,
            ignore_lock_virtual_mods: u16,
            mouse_keys_dflt_btn: u8,
            groups_wrap: u8,
            access_xoptions: u16,
            affect_enabled_controls: u32,
            enabled_controls: u32,
            change_controls: u32,
            repeat_delay: u16,
            repeat_interval: u16,
            slow_keys_delay: u16,
            debounce_delay: u16,
            mouse_keys_delay: u16,
            mouse_keys_interval: u16,
            mouse_keys_time_to_max: u16,
            mouse_keys_max_speed: u16,
            mouse_keys_curve: i16,
            access_xtimeout: u16,
            access_xtimeout_mask: u32,
            access_xtimeout_values: u32,
            access_xtimeout_options_mask: u16,
            access_xtimeout_options_values: u16,
            per_key_repeat: *const [u8; 32],
        ) -> xcb_void_cookie_t,
    >,
    xcb_xkb_get_map_map_types_rtrn_length: LazySymbol<
        unsafe fn(r: *const xcb_xkb_get_map_reply_t, s: *const xcb_xkb_get_map_map_t) -> c_int,
    >,
    xcb_xkb_get_map_map_types_rtrn_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_map_reply_t,
            s: *const xcb_xkb_get_map_map_t,
        ) -> xcb_xkb_key_type_iterator_t,
    >,
    xcb_xkb_get_map_map_syms_rtrn_length: LazySymbol<
        unsafe fn(r: *const xcb_xkb_get_map_reply_t, s: *const xcb_xkb_get_map_map_t) -> c_int,
    >,
    xcb_xkb_get_map_map_syms_rtrn_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_map_reply_t,
            s: *const xcb_xkb_get_map_map_t,
        ) -> xcb_xkb_key_sym_map_iterator_t,
    >,
    xcb_xkb_get_map_map_acts_rtrn_count:
        LazySymbol<unsafe fn(s: *const xcb_xkb_get_map_map_t) -> *mut u8>,
    xcb_xkb_get_map_map_acts_rtrn_count_length: LazySymbol<
        unsafe fn(r: *const xcb_xkb_get_map_reply_t, s: *const xcb_xkb_get_map_map_t) -> c_int,
    >,
    xcb_xkb_get_map_map_acts_rtrn_count_end: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_map_reply_t,
            s: *const xcb_xkb_get_map_map_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_get_map_map_acts_rtrn_acts:
        LazySymbol<unsafe fn(s: *const xcb_xkb_get_map_map_t) -> *mut xcb_xkb_action_t>,
    xcb_xkb_get_map_map_acts_rtrn_acts_length: LazySymbol<
        unsafe fn(r: *const xcb_xkb_get_map_reply_t, s: *const xcb_xkb_get_map_map_t) -> c_int,
    >,
    xcb_xkb_get_map_map_acts_rtrn_acts_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_map_reply_t,
            s: *const xcb_xkb_get_map_map_t,
        ) -> xcb_xkb_action_iterator_t,
    >,
    xcb_xkb_get_map_map_behaviors_rtrn:
        LazySymbol<unsafe fn(s: *const xcb_xkb_get_map_map_t) -> *mut xcb_xkb_set_behavior_t>,
    xcb_xkb_get_map_map_behaviors_rtrn_length: LazySymbol<
        unsafe fn(r: *const xcb_xkb_get_map_reply_t, s: *const xcb_xkb_get_map_map_t) -> c_int,
    >,
    xcb_xkb_get_map_map_behaviors_rtrn_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_map_reply_t,
            s: *const xcb_xkb_get_map_map_t,
        ) -> xcb_xkb_set_behavior_iterator_t,
    >,
    xcb_xkb_get_map_map_vmods_rtrn:
        LazySymbol<unsafe fn(s: *const xcb_xkb_get_map_map_t) -> *mut u8>,
    xcb_xkb_get_map_map_vmods_rtrn_length: LazySymbol<
        unsafe fn(r: *const xcb_xkb_get_map_reply_t, s: *const xcb_xkb_get_map_map_t) -> c_int,
    >,
    xcb_xkb_get_map_map_vmods_rtrn_end: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_map_reply_t,
            s: *const xcb_xkb_get_map_map_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_get_map_map_explicit_rtrn:
        LazySymbol<unsafe fn(s: *const xcb_xkb_get_map_map_t) -> *mut xcb_xkb_set_explicit_t>,
    xcb_xkb_get_map_map_explicit_rtrn_length: LazySymbol<
        unsafe fn(r: *const xcb_xkb_get_map_reply_t, s: *const xcb_xkb_get_map_map_t) -> c_int,
    >,
    xcb_xkb_get_map_map_explicit_rtrn_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_map_reply_t,
            s: *const xcb_xkb_get_map_map_t,
        ) -> xcb_xkb_set_explicit_iterator_t,
    >,
    xcb_xkb_get_map_map_modmap_rtrn:
        LazySymbol<unsafe fn(s: *const xcb_xkb_get_map_map_t) -> *mut xcb_xkb_key_mod_map_t>,
    xcb_xkb_get_map_map_modmap_rtrn_length: LazySymbol<
        unsafe fn(r: *const xcb_xkb_get_map_reply_t, s: *const xcb_xkb_get_map_map_t) -> c_int,
    >,
    xcb_xkb_get_map_map_modmap_rtrn_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_map_reply_t,
            s: *const xcb_xkb_get_map_map_t,
        ) -> xcb_xkb_key_mod_map_iterator_t,
    >,
    xcb_xkb_get_map_map_vmodmap_rtrn:
        LazySymbol<unsafe fn(s: *const xcb_xkb_get_map_map_t) -> *mut xcb_xkb_key_v_mod_map_t>,
    xcb_xkb_get_map_map_vmodmap_rtrn_length: LazySymbol<
        unsafe fn(r: *const xcb_xkb_get_map_reply_t, s: *const xcb_xkb_get_map_map_t) -> c_int,
    >,
    xcb_xkb_get_map_map_vmodmap_rtrn_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_map_reply_t,
            s: *const xcb_xkb_get_map_map_t,
        ) -> xcb_xkb_key_v_mod_map_iterator_t,
    >,
    xcb_xkb_get_map_map_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            nTypes: u8,
            nKeySyms: u8,
            nKeyActions: u8,
            totalActions: u16,
            totalKeyBehaviors: u8,
            virtualMods: u16,
            totalKeyExplicit: u8,
            totalModMapKeys: u8,
            totalVModMapKeys: u8,
            present: u16,
            _aux: *const xcb_xkb_get_map_map_t,
        ) -> c_int,
    >,
    xcb_xkb_get_map_map_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            nTypes: u8,
            nKeySyms: u8,
            nKeyActions: u8,
            totalActions: u16,
            totalKeyBehaviors: u8,
            virtualMods: u16,
            totalKeyExplicit: u8,
            totalModMapKeys: u8,
            totalVModMapKeys: u8,
            present: u16,
            _aux: *mut xcb_xkb_get_map_map_t,
        ) -> c_int,
    >,
    xcb_xkb_get_map_map_sizeof: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            nTypes: u8,
            nKeySyms: u8,
            nKeyActions: u8,
            totalActions: u16,
            totalKeyBehaviors: u8,
            virtualMods: u16,
            totalKeyExplicit: u8,
            totalModMapKeys: u8,
            totalVModMapKeys: u8,
            present: u16,
        ) -> c_int,
    >,
    xcb_xkb_get_map_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xkb_get_map: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            full: u16,
            partial: u16,
            first_type: u8,
            n_types: u8,
            first_key_sym: xcb_keycode_t,
            n_key_syms: u8,
            first_key_action: xcb_keycode_t,
            n_key_actions: u8,
            first_key_behavior: xcb_keycode_t,
            n_key_behaviors: u8,
            virtual_mods: u16,
            first_key_explicit: xcb_keycode_t,
            n_key_explicit: u8,
            first_mod_map_key: xcb_keycode_t,
            n_mod_map_keys: u8,
            first_vmod_map_key: xcb_keycode_t,
            n_vmod_map_keys: u8,
        ) -> xcb_xkb_get_map_cookie_t,
    >,
    xcb_xkb_get_map_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            full: u16,
            partial: u16,
            first_type: u8,
            n_types: u8,
            first_key_sym: xcb_keycode_t,
            n_key_syms: u8,
            first_key_action: xcb_keycode_t,
            n_key_actions: u8,
            first_key_behavior: xcb_keycode_t,
            n_key_behaviors: u8,
            virtual_mods: u16,
            first_key_explicit: xcb_keycode_t,
            n_key_explicit: u8,
            first_mod_map_key: xcb_keycode_t,
            n_mod_map_keys: u8,
            first_vmod_map_key: xcb_keycode_t,
            n_vmod_map_keys: u8,
        ) -> xcb_xkb_get_map_cookie_t,
    >,
    xcb_xkb_get_map_map: LazySymbol<unsafe fn(r: *const xcb_xkb_get_map_reply_t) -> *mut c_void>,
    xcb_xkb_get_map_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xkb_get_map_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xkb_get_map_reply_t,
    >,
    xcb_xkb_set_map_values_types_length: LazySymbol<
        unsafe fn(r: *const xcb_xkb_set_map_request_t, s: *const xcb_xkb_set_map_values_t) -> c_int,
    >,
    xcb_xkb_set_map_values_types_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_map_request_t,
            s: *const xcb_xkb_set_map_values_t,
        ) -> xcb_xkb_set_key_type_iterator_t,
    >,
    xcb_xkb_set_map_values_syms_length: LazySymbol<
        unsafe fn(r: *const xcb_xkb_set_map_request_t, s: *const xcb_xkb_set_map_values_t) -> c_int,
    >,
    xcb_xkb_set_map_values_syms_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_map_request_t,
            s: *const xcb_xkb_set_map_values_t,
        ) -> xcb_xkb_key_sym_map_iterator_t,
    >,
    xcb_xkb_set_map_values_actions_count:
        LazySymbol<unsafe fn(s: *const xcb_xkb_set_map_values_t) -> *mut u8>,
    xcb_xkb_set_map_values_actions_count_length: LazySymbol<
        unsafe fn(r: *const xcb_xkb_set_map_request_t, s: *const xcb_xkb_set_map_values_t) -> c_int,
    >,
    xcb_xkb_set_map_values_actions_count_end: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_map_request_t,
            s: *const xcb_xkb_set_map_values_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_set_map_values_actions:
        LazySymbol<unsafe fn(s: *const xcb_xkb_set_map_values_t) -> *mut xcb_xkb_action_t>,
    xcb_xkb_set_map_values_actions_length: LazySymbol<
        unsafe fn(r: *const xcb_xkb_set_map_request_t, s: *const xcb_xkb_set_map_values_t) -> c_int,
    >,
    xcb_xkb_set_map_values_actions_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_map_request_t,
            s: *const xcb_xkb_set_map_values_t,
        ) -> xcb_xkb_action_iterator_t,
    >,
    xcb_xkb_set_map_values_behaviors:
        LazySymbol<unsafe fn(s: *const xcb_xkb_set_map_values_t) -> *mut xcb_xkb_set_behavior_t>,
    xcb_xkb_set_map_values_behaviors_length: LazySymbol<
        unsafe fn(r: *const xcb_xkb_set_map_request_t, s: *const xcb_xkb_set_map_values_t) -> c_int,
    >,
    xcb_xkb_set_map_values_behaviors_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_map_request_t,
            s: *const xcb_xkb_set_map_values_t,
        ) -> xcb_xkb_set_behavior_iterator_t,
    >,
    xcb_xkb_set_map_values_vmods:
        LazySymbol<unsafe fn(s: *const xcb_xkb_set_map_values_t) -> *mut u8>,
    xcb_xkb_set_map_values_vmods_length: LazySymbol<
        unsafe fn(r: *const xcb_xkb_set_map_request_t, s: *const xcb_xkb_set_map_values_t) -> c_int,
    >,
    xcb_xkb_set_map_values_vmods_end: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_map_request_t,
            s: *const xcb_xkb_set_map_values_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_set_map_values_explicit:
        LazySymbol<unsafe fn(s: *const xcb_xkb_set_map_values_t) -> *mut xcb_xkb_set_explicit_t>,
    xcb_xkb_set_map_values_explicit_length: LazySymbol<
        unsafe fn(r: *const xcb_xkb_set_map_request_t, s: *const xcb_xkb_set_map_values_t) -> c_int,
    >,
    xcb_xkb_set_map_values_explicit_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_map_request_t,
            s: *const xcb_xkb_set_map_values_t,
        ) -> xcb_xkb_set_explicit_iterator_t,
    >,
    xcb_xkb_set_map_values_modmap:
        LazySymbol<unsafe fn(s: *const xcb_xkb_set_map_values_t) -> *mut xcb_xkb_key_mod_map_t>,
    xcb_xkb_set_map_values_modmap_length: LazySymbol<
        unsafe fn(r: *const xcb_xkb_set_map_request_t, s: *const xcb_xkb_set_map_values_t) -> c_int,
    >,
    xcb_xkb_set_map_values_modmap_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_map_request_t,
            s: *const xcb_xkb_set_map_values_t,
        ) -> xcb_xkb_key_mod_map_iterator_t,
    >,
    xcb_xkb_set_map_values_vmodmap:
        LazySymbol<unsafe fn(s: *const xcb_xkb_set_map_values_t) -> *mut xcb_xkb_key_v_mod_map_t>,
    xcb_xkb_set_map_values_vmodmap_length: LazySymbol<
        unsafe fn(r: *const xcb_xkb_set_map_request_t, s: *const xcb_xkb_set_map_values_t) -> c_int,
    >,
    xcb_xkb_set_map_values_vmodmap_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_map_request_t,
            s: *const xcb_xkb_set_map_values_t,
        ) -> xcb_xkb_key_v_mod_map_iterator_t,
    >,
    xcb_xkb_set_map_values_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            nTypes: u8,
            nKeySyms: u8,
            nKeyActions: u8,
            totalActions: u16,
            totalKeyBehaviors: u8,
            virtualMods: u16,
            totalKeyExplicit: u8,
            totalModMapKeys: u8,
            totalVModMapKeys: u8,
            present: u16,
            _aux: *const xcb_xkb_set_map_values_t,
        ) -> c_int,
    >,
    xcb_xkb_set_map_values_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            nTypes: u8,
            nKeySyms: u8,
            nKeyActions: u8,
            totalActions: u16,
            totalKeyBehaviors: u8,
            virtualMods: u16,
            totalKeyExplicit: u8,
            totalModMapKeys: u8,
            totalVModMapKeys: u8,
            present: u16,
            _aux: *mut xcb_xkb_set_map_values_t,
        ) -> c_int,
    >,
    xcb_xkb_set_map_values_sizeof: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            nTypes: u8,
            nKeySyms: u8,
            nKeyActions: u8,
            totalActions: u16,
            totalKeyBehaviors: u8,
            virtualMods: u16,
            totalKeyExplicit: u8,
            totalModMapKeys: u8,
            totalVModMapKeys: u8,
            present: u16,
        ) -> c_int,
    >,
    xcb_xkb_set_map_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xkb_set_map_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            present: u16,
            flags: u16,
            min_key_code: xcb_keycode_t,
            max_key_code: xcb_keycode_t,
            first_type: u8,
            n_types: u8,
            first_key_sym: xcb_keycode_t,
            n_key_syms: u8,
            total_syms: u16,
            first_key_action: xcb_keycode_t,
            n_key_actions: u8,
            total_actions: u16,
            first_key_behavior: xcb_keycode_t,
            n_key_behaviors: u8,
            total_key_behaviors: u8,
            first_key_explicit: xcb_keycode_t,
            n_key_explicit: u8,
            total_key_explicit: u8,
            first_mod_map_key: xcb_keycode_t,
            n_mod_map_keys: u8,
            total_mod_map_keys: u8,
            first_vmod_map_key: xcb_keycode_t,
            n_vmod_map_keys: u8,
            total_vmod_map_keys: u8,
            virtual_mods: u16,
            values: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xkb_set_map: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            present: u16,
            flags: u16,
            min_key_code: xcb_keycode_t,
            max_key_code: xcb_keycode_t,
            first_type: u8,
            n_types: u8,
            first_key_sym: xcb_keycode_t,
            n_key_syms: u8,
            total_syms: u16,
            first_key_action: xcb_keycode_t,
            n_key_actions: u8,
            total_actions: u16,
            first_key_behavior: xcb_keycode_t,
            n_key_behaviors: u8,
            total_key_behaviors: u8,
            first_key_explicit: xcb_keycode_t,
            n_key_explicit: u8,
            total_key_explicit: u8,
            first_mod_map_key: xcb_keycode_t,
            n_mod_map_keys: u8,
            total_mod_map_keys: u8,
            first_vmod_map_key: xcb_keycode_t,
            n_vmod_map_keys: u8,
            total_vmod_map_keys: u8,
            virtual_mods: u16,
            values: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xkb_set_map_aux_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            present: u16,
            flags: u16,
            min_key_code: xcb_keycode_t,
            max_key_code: xcb_keycode_t,
            first_type: u8,
            n_types: u8,
            first_key_sym: xcb_keycode_t,
            n_key_syms: u8,
            total_syms: u16,
            first_key_action: xcb_keycode_t,
            n_key_actions: u8,
            total_actions: u16,
            first_key_behavior: xcb_keycode_t,
            n_key_behaviors: u8,
            total_key_behaviors: u8,
            first_key_explicit: xcb_keycode_t,
            n_key_explicit: u8,
            total_key_explicit: u8,
            first_mod_map_key: xcb_keycode_t,
            n_mod_map_keys: u8,
            total_mod_map_keys: u8,
            first_vmod_map_key: xcb_keycode_t,
            n_vmod_map_keys: u8,
            total_vmod_map_keys: u8,
            virtual_mods: u16,
            values: *const xcb_xkb_set_map_values_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xkb_set_map_aux: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            present: u16,
            flags: u16,
            min_key_code: xcb_keycode_t,
            max_key_code: xcb_keycode_t,
            first_type: u8,
            n_types: u8,
            first_key_sym: xcb_keycode_t,
            n_key_syms: u8,
            total_syms: u16,
            first_key_action: xcb_keycode_t,
            n_key_actions: u8,
            total_actions: u16,
            first_key_behavior: xcb_keycode_t,
            n_key_behaviors: u8,
            total_key_behaviors: u8,
            first_key_explicit: xcb_keycode_t,
            n_key_explicit: u8,
            total_key_explicit: u8,
            first_mod_map_key: xcb_keycode_t,
            n_mod_map_keys: u8,
            total_mod_map_keys: u8,
            first_vmod_map_key: xcb_keycode_t,
            n_vmod_map_keys: u8,
            total_vmod_map_keys: u8,
            virtual_mods: u16,
            values: *const xcb_xkb_set_map_values_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xkb_set_map_values:
        LazySymbol<unsafe fn(r: *const xcb_xkb_set_map_request_t) -> *mut c_void>,
    xcb_xkb_get_compat_map_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xkb_get_compat_map: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            groups: u8,
            get_all_si: u8,
            first_si: u16,
            n_si: u16,
        ) -> xcb_xkb_get_compat_map_cookie_t,
    >,
    xcb_xkb_get_compat_map_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            groups: u8,
            get_all_si: u8,
            first_si: u16,
            n_si: u16,
        ) -> xcb_xkb_get_compat_map_cookie_t,
    >,
    xcb_xkb_get_compat_map_si_rtrn: LazySymbol<
        unsafe fn(r: *const xcb_xkb_get_compat_map_reply_t) -> *mut xcb_xkb_sym_interpret_t,
    >,
    xcb_xkb_get_compat_map_si_rtrn_length:
        LazySymbol<unsafe fn(r: *const xcb_xkb_get_compat_map_reply_t) -> c_int>,
    xcb_xkb_get_compat_map_si_rtrn_iterator: LazySymbol<
        unsafe fn(r: *const xcb_xkb_get_compat_map_reply_t) -> xcb_xkb_sym_interpret_iterator_t,
    >,
    xcb_xkb_get_compat_map_group_rtrn:
        LazySymbol<unsafe fn(r: *const xcb_xkb_get_compat_map_reply_t) -> *mut xcb_xkb_mod_def_t>,
    xcb_xkb_get_compat_map_group_rtrn_length:
        LazySymbol<unsafe fn(r: *const xcb_xkb_get_compat_map_reply_t) -> c_int>,
    xcb_xkb_get_compat_map_group_rtrn_iterator: LazySymbol<
        unsafe fn(r: *const xcb_xkb_get_compat_map_reply_t) -> xcb_xkb_mod_def_iterator_t,
    >,
    xcb_xkb_get_compat_map_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xkb_get_compat_map_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xkb_get_compat_map_reply_t,
    >,
    xcb_xkb_set_compat_map_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xkb_set_compat_map_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            recompute_actions: u8,
            truncate_si: u8,
            groups: u8,
            first_si: u16,
            n_si: u16,
            si: *const xcb_xkb_sym_interpret_t,
            group_maps: *const xcb_xkb_mod_def_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xkb_set_compat_map: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            recompute_actions: u8,
            truncate_si: u8,
            groups: u8,
            first_si: u16,
            n_si: u16,
            si: *const xcb_xkb_sym_interpret_t,
            group_maps: *const xcb_xkb_mod_def_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xkb_set_compat_map_si: LazySymbol<
        unsafe fn(r: *const xcb_xkb_set_compat_map_request_t) -> *mut xcb_xkb_sym_interpret_t,
    >,
    xcb_xkb_set_compat_map_si_length:
        LazySymbol<unsafe fn(r: *const xcb_xkb_set_compat_map_request_t) -> c_int>,
    xcb_xkb_set_compat_map_si_iterator: LazySymbol<
        unsafe fn(r: *const xcb_xkb_set_compat_map_request_t) -> xcb_xkb_sym_interpret_iterator_t,
    >,
    xcb_xkb_set_compat_map_group_maps:
        LazySymbol<unsafe fn(r: *const xcb_xkb_set_compat_map_request_t) -> *mut xcb_xkb_mod_def_t>,
    xcb_xkb_set_compat_map_group_maps_length:
        LazySymbol<unsafe fn(r: *const xcb_xkb_set_compat_map_request_t) -> c_int>,
    xcb_xkb_set_compat_map_group_maps_iterator: LazySymbol<
        unsafe fn(r: *const xcb_xkb_set_compat_map_request_t) -> xcb_xkb_mod_def_iterator_t,
    >,
    xcb_xkb_get_indicator_state: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
        ) -> xcb_xkb_get_indicator_state_cookie_t,
    >,
    xcb_xkb_get_indicator_state_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
        ) -> xcb_xkb_get_indicator_state_cookie_t,
    >,
    xcb_xkb_get_indicator_state_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xkb_get_indicator_state_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xkb_get_indicator_state_reply_t,
    >,
    xcb_xkb_get_indicator_map_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xkb_get_indicator_map: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            which: u32,
        ) -> xcb_xkb_get_indicator_map_cookie_t,
    >,
    xcb_xkb_get_indicator_map_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            which: u32,
        ) -> xcb_xkb_get_indicator_map_cookie_t,
    >,
    xcb_xkb_get_indicator_map_maps: LazySymbol<
        unsafe fn(r: *const xcb_xkb_get_indicator_map_reply_t) -> *mut xcb_xkb_indicator_map_t,
    >,
    xcb_xkb_get_indicator_map_maps_length:
        LazySymbol<unsafe fn(r: *const xcb_xkb_get_indicator_map_reply_t) -> c_int>,
    xcb_xkb_get_indicator_map_maps_iterator: LazySymbol<
        unsafe fn(r: *const xcb_xkb_get_indicator_map_reply_t) -> xcb_xkb_indicator_map_iterator_t,
    >,
    xcb_xkb_get_indicator_map_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xkb_get_indicator_map_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xkb_get_indicator_map_reply_t,
    >,
    xcb_xkb_set_indicator_map_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xkb_set_indicator_map_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            which: u32,
            maps: *const xcb_xkb_indicator_map_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xkb_set_indicator_map: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            which: u32,
            maps: *const xcb_xkb_indicator_map_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xkb_set_indicator_map_maps: LazySymbol<
        unsafe fn(r: *const xcb_xkb_set_indicator_map_request_t) -> *mut xcb_xkb_indicator_map_t,
    >,
    xcb_xkb_set_indicator_map_maps_length:
        LazySymbol<unsafe fn(r: *const xcb_xkb_set_indicator_map_request_t) -> c_int>,
    xcb_xkb_set_indicator_map_maps_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_indicator_map_request_t,
        ) -> xcb_xkb_indicator_map_iterator_t,
    >,
    xcb_xkb_get_named_indicator: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            led_class: xcb_xkb_led_class_spec_t,
            led_id: xcb_xkb_id_spec_t,
            indicator: xcb_atom_t,
        ) -> xcb_xkb_get_named_indicator_cookie_t,
    >,
    xcb_xkb_get_named_indicator_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            led_class: xcb_xkb_led_class_spec_t,
            led_id: xcb_xkb_id_spec_t,
            indicator: xcb_atom_t,
        ) -> xcb_xkb_get_named_indicator_cookie_t,
    >,
    xcb_xkb_get_named_indicator_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xkb_get_named_indicator_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xkb_get_named_indicator_reply_t,
    >,
    xcb_xkb_set_named_indicator_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            led_class: xcb_xkb_led_class_spec_t,
            led_id: xcb_xkb_id_spec_t,
            indicator: xcb_atom_t,
            set_state: u8,
            on: u8,
            set_map: u8,
            create_map: u8,
            map_flags: u8,
            map_which_groups: u8,
            map_groups: u8,
            map_which_mods: u8,
            map_real_mods: u8,
            map_vmods: u16,
            map_ctrls: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xkb_set_named_indicator: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            led_class: xcb_xkb_led_class_spec_t,
            led_id: xcb_xkb_id_spec_t,
            indicator: xcb_atom_t,
            set_state: u8,
            on: u8,
            set_map: u8,
            create_map: u8,
            map_flags: u8,
            map_which_groups: u8,
            map_groups: u8,
            map_which_mods: u8,
            map_real_mods: u8,
            map_vmods: u16,
            map_ctrls: u32,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xkb_get_names_value_list_type_names:
        LazySymbol<unsafe fn(s: *const xcb_xkb_get_names_value_list_t) -> *mut xcb_atom_t>,
    xcb_xkb_get_names_value_list_type_names_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_names_reply_t,
            s: *const xcb_xkb_get_names_value_list_t,
        ) -> c_int,
    >,
    xcb_xkb_get_names_value_list_type_names_end: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_names_reply_t,
            s: *const xcb_xkb_get_names_value_list_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_get_names_value_list_n_levels_per_type:
        LazySymbol<unsafe fn(s: *const xcb_xkb_get_names_value_list_t) -> *mut u8>,
    xcb_xkb_get_names_value_list_n_levels_per_type_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_names_reply_t,
            s: *const xcb_xkb_get_names_value_list_t,
        ) -> c_int,
    >,
    xcb_xkb_get_names_value_list_n_levels_per_type_end: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_names_reply_t,
            s: *const xcb_xkb_get_names_value_list_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_get_names_value_list_kt_level_names:
        LazySymbol<unsafe fn(s: *const xcb_xkb_get_names_value_list_t) -> *mut xcb_atom_t>,
    xcb_xkb_get_names_value_list_kt_level_names_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_names_reply_t,
            s: *const xcb_xkb_get_names_value_list_t,
        ) -> c_int,
    >,
    xcb_xkb_get_names_value_list_kt_level_names_end: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_names_reply_t,
            s: *const xcb_xkb_get_names_value_list_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_get_names_value_list_indicator_names:
        LazySymbol<unsafe fn(s: *const xcb_xkb_get_names_value_list_t) -> *mut xcb_atom_t>,
    xcb_xkb_get_names_value_list_indicator_names_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_names_reply_t,
            s: *const xcb_xkb_get_names_value_list_t,
        ) -> c_int,
    >,
    xcb_xkb_get_names_value_list_indicator_names_end: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_names_reply_t,
            s: *const xcb_xkb_get_names_value_list_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_get_names_value_list_virtual_mod_names:
        LazySymbol<unsafe fn(s: *const xcb_xkb_get_names_value_list_t) -> *mut xcb_atom_t>,
    xcb_xkb_get_names_value_list_virtual_mod_names_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_names_reply_t,
            s: *const xcb_xkb_get_names_value_list_t,
        ) -> c_int,
    >,
    xcb_xkb_get_names_value_list_virtual_mod_names_end: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_names_reply_t,
            s: *const xcb_xkb_get_names_value_list_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_get_names_value_list_groups:
        LazySymbol<unsafe fn(s: *const xcb_xkb_get_names_value_list_t) -> *mut xcb_atom_t>,
    xcb_xkb_get_names_value_list_groups_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_names_reply_t,
            s: *const xcb_xkb_get_names_value_list_t,
        ) -> c_int,
    >,
    xcb_xkb_get_names_value_list_groups_end: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_names_reply_t,
            s: *const xcb_xkb_get_names_value_list_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_get_names_value_list_key_names:
        LazySymbol<unsafe fn(s: *const xcb_xkb_get_names_value_list_t) -> *mut xcb_xkb_key_name_t>,
    xcb_xkb_get_names_value_list_key_names_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_names_reply_t,
            s: *const xcb_xkb_get_names_value_list_t,
        ) -> c_int,
    >,
    xcb_xkb_get_names_value_list_key_names_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_names_reply_t,
            s: *const xcb_xkb_get_names_value_list_t,
        ) -> xcb_xkb_key_name_iterator_t,
    >,
    xcb_xkb_get_names_value_list_key_aliases:
        LazySymbol<unsafe fn(s: *const xcb_xkb_get_names_value_list_t) -> *mut xcb_xkb_key_alias_t>,
    xcb_xkb_get_names_value_list_key_aliases_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_names_reply_t,
            s: *const xcb_xkb_get_names_value_list_t,
        ) -> c_int,
    >,
    xcb_xkb_get_names_value_list_key_aliases_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_names_reply_t,
            s: *const xcb_xkb_get_names_value_list_t,
        ) -> xcb_xkb_key_alias_iterator_t,
    >,
    xcb_xkb_get_names_value_list_radio_group_names:
        LazySymbol<unsafe fn(s: *const xcb_xkb_get_names_value_list_t) -> *mut xcb_atom_t>,
    xcb_xkb_get_names_value_list_radio_group_names_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_names_reply_t,
            s: *const xcb_xkb_get_names_value_list_t,
        ) -> c_int,
    >,
    xcb_xkb_get_names_value_list_radio_group_names_end: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_names_reply_t,
            s: *const xcb_xkb_get_names_value_list_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_get_names_value_list_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            nTypes: u8,
            indicators: u32,
            virtualMods: u16,
            groupNames: u8,
            nKeys: u8,
            nKeyAliases: u8,
            nRadioGroups: u8,
            which: u32,
            _aux: *const xcb_xkb_get_names_value_list_t,
        ) -> c_int,
    >,
    xcb_xkb_get_names_value_list_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            nTypes: u8,
            indicators: u32,
            virtualMods: u16,
            groupNames: u8,
            nKeys: u8,
            nKeyAliases: u8,
            nRadioGroups: u8,
            which: u32,
            _aux: *mut xcb_xkb_get_names_value_list_t,
        ) -> c_int,
    >,
    xcb_xkb_get_names_value_list_sizeof: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            nTypes: u8,
            indicators: u32,
            virtualMods: u16,
            groupNames: u8,
            nKeys: u8,
            nKeyAliases: u8,
            nRadioGroups: u8,
            which: u32,
        ) -> c_int,
    >,
    xcb_xkb_get_names_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xkb_get_names: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            which: u32,
        ) -> xcb_xkb_get_names_cookie_t,
    >,
    xcb_xkb_get_names_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            which: u32,
        ) -> xcb_xkb_get_names_cookie_t,
    >,
    xcb_xkb_get_names_value_list:
        LazySymbol<unsafe fn(r: *const xcb_xkb_get_names_reply_t) -> *mut c_void>,
    xcb_xkb_get_names_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xkb_get_names_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xkb_get_names_reply_t,
    >,
    xcb_xkb_set_names_values_type_names:
        LazySymbol<unsafe fn(s: *const xcb_xkb_set_names_values_t) -> *mut xcb_atom_t>,
    xcb_xkb_set_names_values_type_names_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_names_request_t,
            s: *const xcb_xkb_set_names_values_t,
        ) -> c_int,
    >,
    xcb_xkb_set_names_values_type_names_end: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_names_request_t,
            s: *const xcb_xkb_set_names_values_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_set_names_values_n_levels_per_type:
        LazySymbol<unsafe fn(s: *const xcb_xkb_set_names_values_t) -> *mut u8>,
    xcb_xkb_set_names_values_n_levels_per_type_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_names_request_t,
            s: *const xcb_xkb_set_names_values_t,
        ) -> c_int,
    >,
    xcb_xkb_set_names_values_n_levels_per_type_end: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_names_request_t,
            s: *const xcb_xkb_set_names_values_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_set_names_values_kt_level_names:
        LazySymbol<unsafe fn(s: *const xcb_xkb_set_names_values_t) -> *mut xcb_atom_t>,
    xcb_xkb_set_names_values_kt_level_names_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_names_request_t,
            s: *const xcb_xkb_set_names_values_t,
        ) -> c_int,
    >,
    xcb_xkb_set_names_values_kt_level_names_end: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_names_request_t,
            s: *const xcb_xkb_set_names_values_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_set_names_values_indicator_names:
        LazySymbol<unsafe fn(s: *const xcb_xkb_set_names_values_t) -> *mut xcb_atom_t>,
    xcb_xkb_set_names_values_indicator_names_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_names_request_t,
            s: *const xcb_xkb_set_names_values_t,
        ) -> c_int,
    >,
    xcb_xkb_set_names_values_indicator_names_end: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_names_request_t,
            s: *const xcb_xkb_set_names_values_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_set_names_values_virtual_mod_names:
        LazySymbol<unsafe fn(s: *const xcb_xkb_set_names_values_t) -> *mut xcb_atom_t>,
    xcb_xkb_set_names_values_virtual_mod_names_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_names_request_t,
            s: *const xcb_xkb_set_names_values_t,
        ) -> c_int,
    >,
    xcb_xkb_set_names_values_virtual_mod_names_end: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_names_request_t,
            s: *const xcb_xkb_set_names_values_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_set_names_values_groups:
        LazySymbol<unsafe fn(s: *const xcb_xkb_set_names_values_t) -> *mut xcb_atom_t>,
    xcb_xkb_set_names_values_groups_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_names_request_t,
            s: *const xcb_xkb_set_names_values_t,
        ) -> c_int,
    >,
    xcb_xkb_set_names_values_groups_end: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_names_request_t,
            s: *const xcb_xkb_set_names_values_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_set_names_values_key_names:
        LazySymbol<unsafe fn(s: *const xcb_xkb_set_names_values_t) -> *mut xcb_xkb_key_name_t>,
    xcb_xkb_set_names_values_key_names_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_names_request_t,
            s: *const xcb_xkb_set_names_values_t,
        ) -> c_int,
    >,
    xcb_xkb_set_names_values_key_names_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_names_request_t,
            s: *const xcb_xkb_set_names_values_t,
        ) -> xcb_xkb_key_name_iterator_t,
    >,
    xcb_xkb_set_names_values_key_aliases:
        LazySymbol<unsafe fn(s: *const xcb_xkb_set_names_values_t) -> *mut xcb_xkb_key_alias_t>,
    xcb_xkb_set_names_values_key_aliases_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_names_request_t,
            s: *const xcb_xkb_set_names_values_t,
        ) -> c_int,
    >,
    xcb_xkb_set_names_values_key_aliases_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_names_request_t,
            s: *const xcb_xkb_set_names_values_t,
        ) -> xcb_xkb_key_alias_iterator_t,
    >,
    xcb_xkb_set_names_values_radio_group_names:
        LazySymbol<unsafe fn(s: *const xcb_xkb_set_names_values_t) -> *mut xcb_atom_t>,
    xcb_xkb_set_names_values_radio_group_names_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_names_request_t,
            s: *const xcb_xkb_set_names_values_t,
        ) -> c_int,
    >,
    xcb_xkb_set_names_values_radio_group_names_end: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_names_request_t,
            s: *const xcb_xkb_set_names_values_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_set_names_values_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            nTypes: u8,
            indicators: u32,
            virtualMods: u16,
            groupNames: u8,
            nKeys: u8,
            nKeyAliases: u8,
            nRadioGroups: u8,
            which: u32,
            _aux: *const xcb_xkb_set_names_values_t,
        ) -> c_int,
    >,
    xcb_xkb_set_names_values_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            nTypes: u8,
            indicators: u32,
            virtualMods: u16,
            groupNames: u8,
            nKeys: u8,
            nKeyAliases: u8,
            nRadioGroups: u8,
            which: u32,
            _aux: *mut xcb_xkb_set_names_values_t,
        ) -> c_int,
    >,
    xcb_xkb_set_names_values_sizeof: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            nTypes: u8,
            indicators: u32,
            virtualMods: u16,
            groupNames: u8,
            nKeys: u8,
            nKeyAliases: u8,
            nRadioGroups: u8,
            which: u32,
        ) -> c_int,
    >,
    xcb_xkb_set_names_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xkb_set_names_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            virtual_mods: u16,
            which: u32,
            first_type: u8,
            n_types: u8,
            first_ktlevelt: u8,
            n_ktlevels: u8,
            indicators: u32,
            group_names: u8,
            n_radio_groups: u8,
            first_key: xcb_keycode_t,
            n_keys: u8,
            n_key_aliases: u8,
            total_ktlevel_names: u16,
            values: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xkb_set_names: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            virtual_mods: u16,
            which: u32,
            first_type: u8,
            n_types: u8,
            first_ktlevelt: u8,
            n_ktlevels: u8,
            indicators: u32,
            group_names: u8,
            n_radio_groups: u8,
            first_key: xcb_keycode_t,
            n_keys: u8,
            n_key_aliases: u8,
            total_ktlevel_names: u16,
            values: *const c_void,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xkb_set_names_aux_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            virtual_mods: u16,
            which: u32,
            first_type: u8,
            n_types: u8,
            first_ktlevelt: u8,
            n_ktlevels: u8,
            indicators: u32,
            group_names: u8,
            n_radio_groups: u8,
            first_key: xcb_keycode_t,
            n_keys: u8,
            n_key_aliases: u8,
            total_ktlevel_names: u16,
            values: *const xcb_xkb_set_names_values_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xkb_set_names_aux: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            virtual_mods: u16,
            which: u32,
            first_type: u8,
            n_types: u8,
            first_ktlevelt: u8,
            n_ktlevels: u8,
            indicators: u32,
            group_names: u8,
            n_radio_groups: u8,
            first_key: xcb_keycode_t,
            n_keys: u8,
            n_key_aliases: u8,
            total_ktlevel_names: u16,
            values: *const xcb_xkb_set_names_values_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xkb_set_names_values:
        LazySymbol<unsafe fn(r: *const xcb_xkb_set_names_request_t) -> *mut c_void>,
    xcb_xkb_per_client_flags: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            change: u32,
            value: u32,
            ctrls_to_change: u32,
            auto_ctrls: u32,
            auto_ctrls_values: u32,
        ) -> xcb_xkb_per_client_flags_cookie_t,
    >,
    xcb_xkb_per_client_flags_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            change: u32,
            value: u32,
            ctrls_to_change: u32,
            auto_ctrls: u32,
            auto_ctrls_values: u32,
        ) -> xcb_xkb_per_client_flags_cookie_t,
    >,
    xcb_xkb_per_client_flags_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xkb_per_client_flags_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xkb_per_client_flags_reply_t,
    >,
    xcb_xkb_list_components_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xkb_list_components: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            max_names: u16,
        ) -> xcb_xkb_list_components_cookie_t,
    >,
    xcb_xkb_list_components_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            max_names: u16,
        ) -> xcb_xkb_list_components_cookie_t,
    >,
    xcb_xkb_list_components_keymaps_length:
        LazySymbol<unsafe fn(r: *const xcb_xkb_list_components_reply_t) -> c_int>,
    xcb_xkb_list_components_keymaps_iterator: LazySymbol<
        unsafe fn(r: *const xcb_xkb_list_components_reply_t) -> xcb_xkb_listing_iterator_t,
    >,
    xcb_xkb_list_components_keycodes_length:
        LazySymbol<unsafe fn(r: *const xcb_xkb_list_components_reply_t) -> c_int>,
    xcb_xkb_list_components_keycodes_iterator: LazySymbol<
        unsafe fn(r: *const xcb_xkb_list_components_reply_t) -> xcb_xkb_listing_iterator_t,
    >,
    xcb_xkb_list_components_types_length:
        LazySymbol<unsafe fn(r: *const xcb_xkb_list_components_reply_t) -> c_int>,
    xcb_xkb_list_components_types_iterator: LazySymbol<
        unsafe fn(r: *const xcb_xkb_list_components_reply_t) -> xcb_xkb_listing_iterator_t,
    >,
    xcb_xkb_list_components_compat_maps_length:
        LazySymbol<unsafe fn(r: *const xcb_xkb_list_components_reply_t) -> c_int>,
    xcb_xkb_list_components_compat_maps_iterator: LazySymbol<
        unsafe fn(r: *const xcb_xkb_list_components_reply_t) -> xcb_xkb_listing_iterator_t,
    >,
    xcb_xkb_list_components_symbols_length:
        LazySymbol<unsafe fn(r: *const xcb_xkb_list_components_reply_t) -> c_int>,
    xcb_xkb_list_components_symbols_iterator: LazySymbol<
        unsafe fn(r: *const xcb_xkb_list_components_reply_t) -> xcb_xkb_listing_iterator_t,
    >,
    xcb_xkb_list_components_geometries_length:
        LazySymbol<unsafe fn(r: *const xcb_xkb_list_components_reply_t) -> c_int>,
    xcb_xkb_list_components_geometries_iterator: LazySymbol<
        unsafe fn(r: *const xcb_xkb_list_components_reply_t) -> xcb_xkb_listing_iterator_t,
    >,
    xcb_xkb_list_components_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xkb_list_components_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xkb_list_components_reply_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_types_map_types_rtrn_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_types_map_types_rtrn_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> xcb_xkb_key_type_iterator_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_types_map_syms_rtrn_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_types_map_syms_rtrn_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> xcb_xkb_key_sym_map_iterator_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count:
        LazySymbol<unsafe fn(s: *const xcb_xkb_get_kbd_by_name_replies_t) -> *mut u8>,
    xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count_end: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts:
        LazySymbol<unsafe fn(s: *const xcb_xkb_get_kbd_by_name_replies_t) -> *mut xcb_xkb_action_t>,
    xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> xcb_xkb_action_iterator_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn: LazySymbol<
        unsafe fn(s: *const xcb_xkb_get_kbd_by_name_replies_t) -> *mut xcb_xkb_set_behavior_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> xcb_xkb_set_behavior_iterator_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn:
        LazySymbol<unsafe fn(s: *const xcb_xkb_get_kbd_by_name_replies_t) -> *mut u8>,
    xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn_end: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn: LazySymbol<
        unsafe fn(s: *const xcb_xkb_get_kbd_by_name_replies_t) -> *mut xcb_xkb_set_explicit_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> xcb_xkb_set_explicit_iterator_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn: LazySymbol<
        unsafe fn(s: *const xcb_xkb_get_kbd_by_name_replies_t) -> *mut xcb_xkb_key_mod_map_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> xcb_xkb_key_mod_map_iterator_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn: LazySymbol<
        unsafe fn(s: *const xcb_xkb_get_kbd_by_name_replies_t) -> *mut xcb_xkb_key_v_mod_map_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> xcb_xkb_key_v_mod_map_iterator_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_types_map_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            nTypes: u8,
            nKeySyms: u8,
            nKeyActions: u8,
            totalActions: u16,
            totalKeyBehaviors: u8,
            virtualMods: u16,
            totalKeyExplicit: u8,
            totalModMapKeys: u8,
            totalVModMapKeys: u8,
            present: u16,
            _aux: *const xcb_xkb_get_kbd_by_name_replies_types_map_t,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_types_map_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            nTypes: u8,
            nKeySyms: u8,
            nKeyActions: u8,
            totalActions: u16,
            totalKeyBehaviors: u8,
            virtualMods: u16,
            totalKeyExplicit: u8,
            totalModMapKeys: u8,
            totalVModMapKeys: u8,
            present: u16,
            _aux: *mut xcb_xkb_get_kbd_by_name_replies_types_map_t,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_types_map_sizeof: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            nTypes: u8,
            nKeySyms: u8,
            nKeyActions: u8,
            totalActions: u16,
            totalKeyBehaviors: u8,
            virtualMods: u16,
            totalKeyExplicit: u8,
            totalModMapKeys: u8,
            totalVModMapKeys: u8,
            present: u16,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names:
        LazySymbol<unsafe fn(s: *const xcb_xkb_get_kbd_by_name_replies_t) -> *mut xcb_atom_t>,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names_end: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type:
        LazySymbol<unsafe fn(s: *const xcb_xkb_get_kbd_by_name_replies_t) -> *mut u8>,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type_end: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names:
        LazySymbol<unsafe fn(s: *const xcb_xkb_get_kbd_by_name_replies_t) -> *mut xcb_atom_t>,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names_end: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names:
        LazySymbol<unsafe fn(s: *const xcb_xkb_get_kbd_by_name_replies_t) -> *mut xcb_atom_t>,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names_end: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names:
        LazySymbol<unsafe fn(s: *const xcb_xkb_get_kbd_by_name_replies_t) -> *mut xcb_atom_t>,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names_end: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups:
        LazySymbol<unsafe fn(s: *const xcb_xkb_get_kbd_by_name_replies_t) -> *mut xcb_atom_t>,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups_end: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names: LazySymbol<
        unsafe fn(s: *const xcb_xkb_get_kbd_by_name_replies_t) -> *mut xcb_xkb_key_name_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> xcb_xkb_key_name_iterator_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases: LazySymbol<
        unsafe fn(s: *const xcb_xkb_get_kbd_by_name_replies_t) -> *mut xcb_xkb_key_alias_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> xcb_xkb_key_alias_iterator_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names:
        LazySymbol<unsafe fn(s: *const xcb_xkb_get_kbd_by_name_replies_t) -> *mut xcb_atom_t>,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names_end: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> xcb_generic_iterator_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            nTypes: u8,
            indicators: u32,
            virtualMods: u16,
            groupNames: u8,
            nKeys: u8,
            nKeyAliases: u8,
            nRadioGroups: u8,
            which: u32,
            _aux: *const xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            nTypes: u8,
            indicators: u32,
            virtualMods: u16,
            groupNames: u8,
            nKeys: u8,
            nKeyAliases: u8,
            nRadioGroups: u8,
            which: u32,
            _aux: *mut xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list_sizeof: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            nTypes: u8,
            indicators: u32,
            virtualMods: u16,
            groupNames: u8,
            nKeys: u8,
            nKeyAliases: u8,
            nRadioGroups: u8,
            which: u32,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_types_map: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> *mut xcb_xkb_get_kbd_by_name_replies_types_map_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn: LazySymbol<
        unsafe fn(s: *const xcb_xkb_get_kbd_by_name_replies_t) -> *mut xcb_xkb_sym_interpret_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> xcb_xkb_sym_interpret_iterator_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn: LazySymbol<
        unsafe fn(s: *const xcb_xkb_get_kbd_by_name_replies_t) -> *mut xcb_xkb_mod_def_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> xcb_xkb_mod_def_iterator_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps: LazySymbol<
        unsafe fn(s: *const xcb_xkb_get_kbd_by_name_replies_t) -> *mut xcb_xkb_indicator_map_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps_length: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_reply_t,
            s: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> xcb_xkb_indicator_map_iterator_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_key_names_value_list: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> *mut xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_geometry_label_font: LazySymbol<
        unsafe fn(r: *const xcb_xkb_get_kbd_by_name_replies_t) -> *mut xcb_xkb_counted_string_16_t,
    >,
    xcb_xkb_get_kbd_by_name_replies_serialize: LazySymbol<
        unsafe fn(
            _buffer: *mut *mut c_void,
            reported: u16,
            _aux: *const xcb_xkb_get_kbd_by_name_replies_t,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_unpack: LazySymbol<
        unsafe fn(
            _buffer: *const c_void,
            reported: u16,
            _aux: *mut xcb_xkb_get_kbd_by_name_replies_t,
        ) -> c_int,
    >,
    xcb_xkb_get_kbd_by_name_replies_sizeof:
        LazySymbol<unsafe fn(_buffer: *const c_void, reported: u16) -> c_int>,
    xcb_xkb_get_kbd_by_name_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xkb_get_kbd_by_name: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            need: u16,
            want: u16,
            load: u8,
        ) -> xcb_xkb_get_kbd_by_name_cookie_t,
    >,
    xcb_xkb_get_kbd_by_name_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            need: u16,
            want: u16,
            load: u8,
        ) -> xcb_xkb_get_kbd_by_name_cookie_t,
    >,
    xcb_xkb_get_kbd_by_name_replies:
        LazySymbol<unsafe fn(r: *const xcb_xkb_get_kbd_by_name_reply_t) -> *mut c_void>,
    xcb_xkb_get_kbd_by_name_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xkb_get_kbd_by_name_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xkb_get_kbd_by_name_reply_t,
    >,
    xcb_xkb_get_device_info_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xkb_get_device_info: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            wanted: u16,
            all_buttons: u8,
            first_button: u8,
            n_buttons: u8,
            led_class: xcb_xkb_led_class_spec_t,
            led_id: xcb_xkb_id_spec_t,
        ) -> xcb_xkb_get_device_info_cookie_t,
    >,
    xcb_xkb_get_device_info_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            wanted: u16,
            all_buttons: u8,
            first_button: u8,
            n_buttons: u8,
            led_class: xcb_xkb_led_class_spec_t,
            led_id: xcb_xkb_id_spec_t,
        ) -> xcb_xkb_get_device_info_cookie_t,
    >,
    xcb_xkb_get_device_info_name:
        LazySymbol<unsafe fn(r: *const xcb_xkb_get_device_info_reply_t) -> *mut xcb_xkb_string8_t>,
    xcb_xkb_get_device_info_name_length:
        LazySymbol<unsafe fn(r: *const xcb_xkb_get_device_info_reply_t) -> c_int>,
    xcb_xkb_get_device_info_name_end:
        LazySymbol<unsafe fn(r: *const xcb_xkb_get_device_info_reply_t) -> xcb_generic_iterator_t>,
    xcb_xkb_get_device_info_btn_actions:
        LazySymbol<unsafe fn(r: *const xcb_xkb_get_device_info_reply_t) -> *mut xcb_xkb_action_t>,
    xcb_xkb_get_device_info_btn_actions_length:
        LazySymbol<unsafe fn(r: *const xcb_xkb_get_device_info_reply_t) -> c_int>,
    xcb_xkb_get_device_info_btn_actions_iterator: LazySymbol<
        unsafe fn(r: *const xcb_xkb_get_device_info_reply_t) -> xcb_xkb_action_iterator_t,
    >,
    xcb_xkb_get_device_info_leds_length:
        LazySymbol<unsafe fn(r: *const xcb_xkb_get_device_info_reply_t) -> c_int>,
    xcb_xkb_get_device_info_leds_iterator: LazySymbol<
        unsafe fn(r: *const xcb_xkb_get_device_info_reply_t) -> xcb_xkb_device_led_info_iterator_t,
    >,
    xcb_xkb_get_device_info_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xkb_get_device_info_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xkb_get_device_info_reply_t,
    >,
    xcb_xkb_set_device_info_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xkb_set_device_info_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            first_btn: u8,
            n_btns: u8,
            change: u16,
            n_device_led_fbs: u16,
            btn_actions: *const xcb_xkb_action_t,
            leds: *const xcb_xkb_device_led_info_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xkb_set_device_info: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            first_btn: u8,
            n_btns: u8,
            change: u16,
            n_device_led_fbs: u16,
            btn_actions: *const xcb_xkb_action_t,
            leds: *const xcb_xkb_device_led_info_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xkb_set_device_info_btn_actions:
        LazySymbol<unsafe fn(r: *const xcb_xkb_set_device_info_request_t) -> *mut xcb_xkb_action_t>,
    xcb_xkb_set_device_info_btn_actions_length:
        LazySymbol<unsafe fn(r: *const xcb_xkb_set_device_info_request_t) -> c_int>,
    xcb_xkb_set_device_info_btn_actions_iterator: LazySymbol<
        unsafe fn(r: *const xcb_xkb_set_device_info_request_t) -> xcb_xkb_action_iterator_t,
    >,
    xcb_xkb_set_device_info_leds_length:
        LazySymbol<unsafe fn(r: *const xcb_xkb_set_device_info_request_t) -> c_int>,
    xcb_xkb_set_device_info_leds_iterator: LazySymbol<
        unsafe fn(
            r: *const xcb_xkb_set_device_info_request_t,
        ) -> xcb_xkb_device_led_info_iterator_t,
    >,
    xcb_xkb_set_debugging_flags_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xkb_set_debugging_flags: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            msg_length: u16,
            affect_flags: u32,
            flags: u32,
            affect_ctrls: u32,
            ctrls: u32,
            message: *const xcb_xkb_string8_t,
        ) -> xcb_xkb_set_debugging_flags_cookie_t,
    >,
    xcb_xkb_set_debugging_flags_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            msg_length: u16,
            affect_flags: u32,
            flags: u32,
            affect_ctrls: u32,
            ctrls: u32,
            message: *const xcb_xkb_string8_t,
        ) -> xcb_xkb_set_debugging_flags_cookie_t,
    >,
    xcb_xkb_set_debugging_flags_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xkb_set_debugging_flags_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_xkb_set_debugging_flags_reply_t,
    >,
}

macro_rules! sym {
    ($self:expr, $f:ident) => {
        $self.xkb.$f.get(&$self.lib, concat!(stringify!($f), "\0"))
    };
}

macro_rules! has_sym {
    ($self:expr, $f:ident) => {
        unsafe {
            $self
                .xkb
                .$f
                .exists(&$self.lib, concat!(stringify!($f), "\0"))
        }
    };
}

#[cfg(feature = "xcb_xkb")]
impl XcbXkb {
    /// The libxcb identifier of the `xkb` extension.
    #[inline]
    pub fn xcb_xkb_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_xkb_id) }
    }

    /// Returns `true` iff the symbol `xcb_xkb_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_id(&self) -> bool {
        has_sym!(self, xcb_xkb_id)
    }

    /// Advances a `xcb_xkb_device_spec_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_device_spec_next(&self, i: *mut xcb_xkb_device_spec_iterator_t) {
        sym!(self, xcb_xkb_device_spec_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_device_spec_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_device_spec_next(&self) -> bool {
        has_sym!(self, xcb_xkb_device_spec_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_device_spec_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_device_spec_end(
        &self,
        i: xcb_xkb_device_spec_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_device_spec_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_device_spec_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_device_spec_end(&self) -> bool {
        has_sym!(self, xcb_xkb_device_spec_end)
    }

    /// Advances a `xcb_xkb_led_class_spec_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_led_class_spec_next(&self, i: *mut xcb_xkb_led_class_spec_iterator_t) {
        sym!(self, xcb_xkb_led_class_spec_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_led_class_spec_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_led_class_spec_next(&self) -> bool {
        has_sym!(self, xcb_xkb_led_class_spec_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_led_class_spec_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_led_class_spec_end(
        &self,
        i: xcb_xkb_led_class_spec_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_led_class_spec_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_led_class_spec_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_led_class_spec_end(&self) -> bool {
        has_sym!(self, xcb_xkb_led_class_spec_end)
    }

    /// Advances a `xcb_xkb_bell_class_spec_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_bell_class_spec_next(&self, i: *mut xcb_xkb_bell_class_spec_iterator_t) {
        sym!(self, xcb_xkb_bell_class_spec_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_bell_class_spec_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_bell_class_spec_next(&self) -> bool {
        has_sym!(self, xcb_xkb_bell_class_spec_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_bell_class_spec_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_bell_class_spec_end(
        &self,
        i: xcb_xkb_bell_class_spec_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_bell_class_spec_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_bell_class_spec_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_bell_class_spec_end(&self) -> bool {
        has_sym!(self, xcb_xkb_bell_class_spec_end)
    }

    /// Advances a `xcb_xkb_id_spec_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_id_spec_next(&self, i: *mut xcb_xkb_id_spec_iterator_t) {
        sym!(self, xcb_xkb_id_spec_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_id_spec_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_id_spec_next(&self) -> bool {
        has_sym!(self, xcb_xkb_id_spec_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_id_spec_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_id_spec_end(
        &self,
        i: xcb_xkb_id_spec_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_id_spec_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_id_spec_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_id_spec_end(&self) -> bool {
        has_sym!(self, xcb_xkb_id_spec_end)
    }

    /// Advances a `xcb_xkb_indicator_map_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_indicator_map_next(&self, i: *mut xcb_xkb_indicator_map_iterator_t) {
        sym!(self, xcb_xkb_indicator_map_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_indicator_map_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_indicator_map_next(&self) -> bool {
        has_sym!(self, xcb_xkb_indicator_map_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_indicator_map_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_indicator_map_end(
        &self,
        i: xcb_xkb_indicator_map_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_indicator_map_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_indicator_map_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_indicator_map_end(&self) -> bool {
        has_sym!(self, xcb_xkb_indicator_map_end)
    }

    /// Advances a `xcb_xkb_mod_def_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_mod_def_next(&self, i: *mut xcb_xkb_mod_def_iterator_t) {
        sym!(self, xcb_xkb_mod_def_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_mod_def_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_mod_def_next(&self) -> bool {
        has_sym!(self, xcb_xkb_mod_def_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_mod_def_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_mod_def_end(
        &self,
        i: xcb_xkb_mod_def_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_mod_def_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_mod_def_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_mod_def_end(&self) -> bool {
        has_sym!(self, xcb_xkb_mod_def_end)
    }

    /// Advances a `xcb_xkb_key_name_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_key_name_next(&self, i: *mut xcb_xkb_key_name_iterator_t) {
        sym!(self, xcb_xkb_key_name_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_name_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_name_next(&self) -> bool {
        has_sym!(self, xcb_xkb_key_name_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_key_name_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_key_name_end(
        &self,
        i: xcb_xkb_key_name_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_key_name_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_name_end(&self) -> bool {
        has_sym!(self, xcb_xkb_key_name_end)
    }

    /// Advances a `xcb_xkb_key_alias_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_key_alias_next(&self, i: *mut xcb_xkb_key_alias_iterator_t) {
        sym!(self, xcb_xkb_key_alias_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_alias_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_alias_next(&self) -> bool {
        has_sym!(self, xcb_xkb_key_alias_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_key_alias_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_key_alias_end(
        &self,
        i: xcb_xkb_key_alias_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_key_alias_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_alias_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_alias_end(&self) -> bool {
        has_sym!(self, xcb_xkb_key_alias_end)
    }

    /// Computes the size of a `xcb_xkb_counted_string_16_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_counted_string_16_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_counted_string_16_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_counted_string_16_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_counted_string_16_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_counted_string_16_sizeof)
    }

    /// Returns a pointer to the `string` field of a `xcb_xkb_counted_string_16_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_counted_string_16_string(
        &self,
        r: *const xcb_xkb_counted_string_16_t,
    ) -> *mut c_char {
        sym!(self, xcb_xkb_counted_string_16_string)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_counted_string_16_string` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_counted_string_16_string(&self) -> bool {
        has_sym!(self, xcb_xkb_counted_string_16_string)
    }

    /// Returns the number of elements of the `string` field of a `xcb_xkb_counted_string_16_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_counted_string_16_string_length(
        &self,
        r: *const xcb_xkb_counted_string_16_t,
    ) -> c_int {
        sym!(self, xcb_xkb_counted_string_16_string_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_counted_string_16_string_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_counted_string_16_string_length(&self) -> bool {
        has_sym!(self, xcb_xkb_counted_string_16_string_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `string` field of a `xcb_xkb_counted_string_16_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_counted_string_16_string_end(
        &self,
        r: *const xcb_xkb_counted_string_16_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_counted_string_16_string_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_counted_string_16_string_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_counted_string_16_string_end(&self) -> bool {
        has_sym!(self, xcb_xkb_counted_string_16_string_end)
    }

    /// Returns a pointer to the `alignment_pad` field of a `xcb_xkb_counted_string_16_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_counted_string_16_alignment_pad(
        &self,
        r: *const xcb_xkb_counted_string_16_t,
    ) -> *mut c_void {
        sym!(self, xcb_xkb_counted_string_16_alignment_pad)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_counted_string_16_alignment_pad` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_counted_string_16_alignment_pad(&self) -> bool {
        has_sym!(self, xcb_xkb_counted_string_16_alignment_pad)
    }

    /// Returns the number of elements of the `alignment_pad` field of a `xcb_xkb_counted_string_16_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_counted_string_16_alignment_pad_length(
        &self,
        r: *const xcb_xkb_counted_string_16_t,
    ) -> c_int {
        sym!(self, xcb_xkb_counted_string_16_alignment_pad_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_counted_string_16_alignment_pad_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_counted_string_16_alignment_pad_length(&self) -> bool {
        has_sym!(self, xcb_xkb_counted_string_16_alignment_pad_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `alignment_pad` field of a `xcb_xkb_counted_string_16_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_counted_string_16_alignment_pad_end(
        &self,
        r: *const xcb_xkb_counted_string_16_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_counted_string_16_alignment_pad_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_counted_string_16_alignment_pad_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_counted_string_16_alignment_pad_end(&self) -> bool {
        has_sym!(self, xcb_xkb_counted_string_16_alignment_pad_end)
    }

    /// Advances a `xcb_xkb_counted_string_16_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_counted_string_16_next(
        &self,
        i: *mut xcb_xkb_counted_string_16_iterator_t,
    ) {
        sym!(self, xcb_xkb_counted_string_16_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_counted_string_16_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_counted_string_16_next(&self) -> bool {
        has_sym!(self, xcb_xkb_counted_string_16_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_counted_string_16_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_counted_string_16_end(
        &self,
        i: xcb_xkb_counted_string_16_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_counted_string_16_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_counted_string_16_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_counted_string_16_end(&self) -> bool {
        has_sym!(self, xcb_xkb_counted_string_16_end)
    }

    /// Advances a `xcb_xkb_kt_map_entry_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_kt_map_entry_next(&self, i: *mut xcb_xkb_kt_map_entry_iterator_t) {
        sym!(self, xcb_xkb_kt_map_entry_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_kt_map_entry_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_kt_map_entry_next(&self) -> bool {
        has_sym!(self, xcb_xkb_kt_map_entry_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_kt_map_entry_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_kt_map_entry_end(
        &self,
        i: xcb_xkb_kt_map_entry_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_kt_map_entry_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_kt_map_entry_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_kt_map_entry_end(&self) -> bool {
        has_sym!(self, xcb_xkb_kt_map_entry_end)
    }

    /// Computes the size of a `xcb_xkb_key_type_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_key_type_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_key_type_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_type_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_type_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_key_type_sizeof)
    }

    /// Returns a pointer to the `map` field of a `xcb_xkb_key_type_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_key_type_map(
        &self,
        r: *const xcb_xkb_key_type_t,
    ) -> *mut xcb_xkb_kt_map_entry_t {
        sym!(self, xcb_xkb_key_type_map)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_type_map` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_type_map(&self) -> bool {
        has_sym!(self, xcb_xkb_key_type_map)
    }

    /// Returns the number of elements of the `map` field of a `xcb_xkb_key_type_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_key_type_map_length(&self, r: *const xcb_xkb_key_type_t) -> c_int {
        sym!(self, xcb_xkb_key_type_map_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_type_map_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_type_map_length(&self) -> bool {
        has_sym!(self, xcb_xkb_key_type_map_length)
    }

    /// Returns an iterator over the elements of the
    /// `map` field of a `xcb_xkb_key_type_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_key_type_map_iterator(
        &self,
        r: *const xcb_xkb_key_type_t,
    ) -> xcb_xkb_kt_map_entry_iterator_t {
        sym!(self, xcb_xkb_key_type_map_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_type_map_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_type_map_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_key_type_map_iterator)
    }

    /// Returns a pointer to the `preserve` field of a `xcb_xkb_key_type_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_key_type_preserve(
        &self,
        r: *const xcb_xkb_key_type_t,
    ) -> *mut xcb_xkb_mod_def_t {
        sym!(self, xcb_xkb_key_type_preserve)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_type_preserve` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_type_preserve(&self) -> bool {
        has_sym!(self, xcb_xkb_key_type_preserve)
    }

    /// Returns the number of elements of the `preserve` field of a `xcb_xkb_key_type_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_key_type_preserve_length(&self, r: *const xcb_xkb_key_type_t) -> c_int {
        sym!(self, xcb_xkb_key_type_preserve_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_type_preserve_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_type_preserve_length(&self) -> bool {
        has_sym!(self, xcb_xkb_key_type_preserve_length)
    }

    /// Returns an iterator over the elements of the
    /// `preserve` field of a `xcb_xkb_key_type_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_key_type_preserve_iterator(
        &self,
        r: *const xcb_xkb_key_type_t,
    ) -> xcb_xkb_mod_def_iterator_t {
        sym!(self, xcb_xkb_key_type_preserve_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_type_preserve_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_type_preserve_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_key_type_preserve_iterator)
    }

    /// Advances a `xcb_xkb_key_type_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_key_type_next(&self, i: *mut xcb_xkb_key_type_iterator_t) {
        sym!(self, xcb_xkb_key_type_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_type_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_type_next(&self) -> bool {
        has_sym!(self, xcb_xkb_key_type_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_key_type_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_key_type_end(
        &self,
        i: xcb_xkb_key_type_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_key_type_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_type_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_type_end(&self) -> bool {
        has_sym!(self, xcb_xkb_key_type_end)
    }

    /// Computes the size of a `xcb_xkb_key_sym_map_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_key_sym_map_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_key_sym_map_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_sym_map_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_sym_map_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_key_sym_map_sizeof)
    }

    /// Returns a pointer to the `syms` field of a `xcb_xkb_key_sym_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_key_sym_map_syms(
        &self,
        r: *const xcb_xkb_key_sym_map_t,
    ) -> *mut xcb_keysym_t {
        sym!(self, xcb_xkb_key_sym_map_syms)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_sym_map_syms` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_sym_map_syms(&self) -> bool {
        has_sym!(self, xcb_xkb_key_sym_map_syms)
    }

    /// Returns the number of elements of the `syms` field of a `xcb_xkb_key_sym_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_key_sym_map_syms_length(&self, r: *const xcb_xkb_key_sym_map_t) -> c_int {
        sym!(self, xcb_xkb_key_sym_map_syms_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_sym_map_syms_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_sym_map_syms_length(&self) -> bool {
        has_sym!(self, xcb_xkb_key_sym_map_syms_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `syms` field of a `xcb_xkb_key_sym_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_key_sym_map_syms_end(
        &self,
        r: *const xcb_xkb_key_sym_map_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_key_sym_map_syms_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_sym_map_syms_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_sym_map_syms_end(&self) -> bool {
        has_sym!(self, xcb_xkb_key_sym_map_syms_end)
    }

    /// Advances a `xcb_xkb_key_sym_map_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_key_sym_map_next(&self, i: *mut xcb_xkb_key_sym_map_iterator_t) {
        sym!(self, xcb_xkb_key_sym_map_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_sym_map_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_sym_map_next(&self) -> bool {
        has_sym!(self, xcb_xkb_key_sym_map_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_key_sym_map_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_key_sym_map_end(
        &self,
        i: xcb_xkb_key_sym_map_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_key_sym_map_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_sym_map_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_sym_map_end(&self) -> bool {
        has_sym!(self, xcb_xkb_key_sym_map_end)
    }

    /// Advances a `xcb_xkb_common_behavior_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_common_behavior_next(&self, i: *mut xcb_xkb_common_behavior_iterator_t) {
        sym!(self, xcb_xkb_common_behavior_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_common_behavior_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_common_behavior_next(&self) -> bool {
        has_sym!(self, xcb_xkb_common_behavior_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_common_behavior_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_common_behavior_end(
        &self,
        i: xcb_xkb_common_behavior_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_common_behavior_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_common_behavior_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_common_behavior_end(&self) -> bool {
        has_sym!(self, xcb_xkb_common_behavior_end)
    }

    /// Advances a `xcb_xkb_default_behavior_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_default_behavior_next(
        &self,
        i: *mut xcb_xkb_default_behavior_iterator_t,
    ) {
        sym!(self, xcb_xkb_default_behavior_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_default_behavior_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_default_behavior_next(&self) -> bool {
        has_sym!(self, xcb_xkb_default_behavior_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_default_behavior_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_default_behavior_end(
        &self,
        i: xcb_xkb_default_behavior_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_default_behavior_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_default_behavior_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_default_behavior_end(&self) -> bool {
        has_sym!(self, xcb_xkb_default_behavior_end)
    }

    /// Advances a `xcb_xkb_lock_behavior_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_lock_behavior_next(&self, i: *mut xcb_xkb_lock_behavior_iterator_t) {
        sym!(self, xcb_xkb_lock_behavior_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_lock_behavior_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_lock_behavior_next(&self) -> bool {
        has_sym!(self, xcb_xkb_lock_behavior_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_lock_behavior_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_lock_behavior_end(
        &self,
        i: xcb_xkb_lock_behavior_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_lock_behavior_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_lock_behavior_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_lock_behavior_end(&self) -> bool {
        has_sym!(self, xcb_xkb_lock_behavior_end)
    }

    /// Advances a `xcb_xkb_radio_group_behavior_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_radio_group_behavior_next(
        &self,
        i: *mut xcb_xkb_radio_group_behavior_iterator_t,
    ) {
        sym!(self, xcb_xkb_radio_group_behavior_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_radio_group_behavior_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_radio_group_behavior_next(&self) -> bool {
        has_sym!(self, xcb_xkb_radio_group_behavior_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_radio_group_behavior_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_radio_group_behavior_end(
        &self,
        i: xcb_xkb_radio_group_behavior_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_radio_group_behavior_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_radio_group_behavior_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_radio_group_behavior_end(&self) -> bool {
        has_sym!(self, xcb_xkb_radio_group_behavior_end)
    }

    /// Advances a `xcb_xkb_overlay_behavior_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_overlay_behavior_next(
        &self,
        i: *mut xcb_xkb_overlay_behavior_iterator_t,
    ) {
        sym!(self, xcb_xkb_overlay_behavior_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_overlay_behavior_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_overlay_behavior_next(&self) -> bool {
        has_sym!(self, xcb_xkb_overlay_behavior_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_overlay_behavior_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_overlay_behavior_end(
        &self,
        i: xcb_xkb_overlay_behavior_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_overlay_behavior_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_overlay_behavior_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_overlay_behavior_end(&self) -> bool {
        has_sym!(self, xcb_xkb_overlay_behavior_end)
    }

    /// Advances a `xcb_xkb_permament_lock_behavior_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_permament_lock_behavior_next(
        &self,
        i: *mut xcb_xkb_permament_lock_behavior_iterator_t,
    ) {
        sym!(self, xcb_xkb_permament_lock_behavior_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_permament_lock_behavior_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_permament_lock_behavior_next(&self) -> bool {
        has_sym!(self, xcb_xkb_permament_lock_behavior_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_permament_lock_behavior_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_permament_lock_behavior_end(
        &self,
        i: xcb_xkb_permament_lock_behavior_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_permament_lock_behavior_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_permament_lock_behavior_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_permament_lock_behavior_end(&self) -> bool {
        has_sym!(self, xcb_xkb_permament_lock_behavior_end)
    }

    /// Advances a `xcb_xkb_permament_radio_group_behavior_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_permament_radio_group_behavior_next(
        &self,
        i: *mut xcb_xkb_permament_radio_group_behavior_iterator_t,
    ) {
        sym!(self, xcb_xkb_permament_radio_group_behavior_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_permament_radio_group_behavior_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_permament_radio_group_behavior_next(&self) -> bool {
        has_sym!(self, xcb_xkb_permament_radio_group_behavior_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_permament_radio_group_behavior_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_permament_radio_group_behavior_end(
        &self,
        i: xcb_xkb_permament_radio_group_behavior_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_permament_radio_group_behavior_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_permament_radio_group_behavior_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_permament_radio_group_behavior_end(&self) -> bool {
        has_sym!(self, xcb_xkb_permament_radio_group_behavior_end)
    }

    /// Advances a `xcb_xkb_permament_overlay_behavior_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_permament_overlay_behavior_next(
        &self,
        i: *mut xcb_xkb_permament_overlay_behavior_iterator_t,
    ) {
        sym!(self, xcb_xkb_permament_overlay_behavior_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_permament_overlay_behavior_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_permament_overlay_behavior_next(&self) -> bool {
        has_sym!(self, xcb_xkb_permament_overlay_behavior_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_permament_overlay_behavior_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_permament_overlay_behavior_end(
        &self,
        i: xcb_xkb_permament_overlay_behavior_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_permament_overlay_behavior_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_permament_overlay_behavior_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_permament_overlay_behavior_end(&self) -> bool {
        has_sym!(self, xcb_xkb_permament_overlay_behavior_end)
    }

    /// Advances a `xcb_xkb_behavior_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_behavior_next(&self, i: *mut xcb_xkb_behavior_iterator_t) {
        sym!(self, xcb_xkb_behavior_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_behavior_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_behavior_next(&self) -> bool {
        has_sym!(self, xcb_xkb_behavior_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_behavior_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_behavior_end(
        &self,
        i: xcb_xkb_behavior_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_behavior_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_behavior_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_behavior_end(&self) -> bool {
        has_sym!(self, xcb_xkb_behavior_end)
    }

    /// Advances a `xcb_xkb_set_behavior_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_set_behavior_next(&self, i: *mut xcb_xkb_set_behavior_iterator_t) {
        sym!(self, xcb_xkb_set_behavior_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_behavior_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_behavior_next(&self) -> bool {
        has_sym!(self, xcb_xkb_set_behavior_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_set_behavior_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_set_behavior_end(
        &self,
        i: xcb_xkb_set_behavior_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_set_behavior_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_behavior_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_behavior_end(&self) -> bool {
        has_sym!(self, xcb_xkb_set_behavior_end)
    }

    /// Advances a `xcb_xkb_set_explicit_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_set_explicit_next(&self, i: *mut xcb_xkb_set_explicit_iterator_t) {
        sym!(self, xcb_xkb_set_explicit_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_explicit_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_explicit_next(&self) -> bool {
        has_sym!(self, xcb_xkb_set_explicit_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_set_explicit_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_set_explicit_end(
        &self,
        i: xcb_xkb_set_explicit_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_set_explicit_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_explicit_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_explicit_end(&self) -> bool {
        has_sym!(self, xcb_xkb_set_explicit_end)
    }

    /// Advances a `xcb_xkb_key_mod_map_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_key_mod_map_next(&self, i: *mut xcb_xkb_key_mod_map_iterator_t) {
        sym!(self, xcb_xkb_key_mod_map_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_mod_map_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_mod_map_next(&self) -> bool {
        has_sym!(self, xcb_xkb_key_mod_map_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_key_mod_map_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_key_mod_map_end(
        &self,
        i: xcb_xkb_key_mod_map_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_key_mod_map_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_mod_map_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_mod_map_end(&self) -> bool {
        has_sym!(self, xcb_xkb_key_mod_map_end)
    }

    /// Advances a `xcb_xkb_key_v_mod_map_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_key_v_mod_map_next(&self, i: *mut xcb_xkb_key_v_mod_map_iterator_t) {
        sym!(self, xcb_xkb_key_v_mod_map_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_v_mod_map_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_v_mod_map_next(&self) -> bool {
        has_sym!(self, xcb_xkb_key_v_mod_map_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_key_v_mod_map_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_key_v_mod_map_end(
        &self,
        i: xcb_xkb_key_v_mod_map_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_key_v_mod_map_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_v_mod_map_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_v_mod_map_end(&self) -> bool {
        has_sym!(self, xcb_xkb_key_v_mod_map_end)
    }

    /// Advances a `xcb_xkb_kt_set_map_entry_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_kt_set_map_entry_next(
        &self,
        i: *mut xcb_xkb_kt_set_map_entry_iterator_t,
    ) {
        sym!(self, xcb_xkb_kt_set_map_entry_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_kt_set_map_entry_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_kt_set_map_entry_next(&self) -> bool {
        has_sym!(self, xcb_xkb_kt_set_map_entry_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_kt_set_map_entry_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_kt_set_map_entry_end(
        &self,
        i: xcb_xkb_kt_set_map_entry_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_kt_set_map_entry_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_kt_set_map_entry_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_kt_set_map_entry_end(&self) -> bool {
        has_sym!(self, xcb_xkb_kt_set_map_entry_end)
    }

    /// Computes the size of a `xcb_xkb_set_key_type_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_set_key_type_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_set_key_type_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_key_type_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_key_type_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_set_key_type_sizeof)
    }

    /// Returns a pointer to the `entries` field of a `xcb_xkb_set_key_type_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_key_type_entries(
        &self,
        r: *const xcb_xkb_set_key_type_t,
    ) -> *mut xcb_xkb_kt_set_map_entry_t {
        sym!(self, xcb_xkb_set_key_type_entries)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_key_type_entries` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_key_type_entries(&self) -> bool {
        has_sym!(self, xcb_xkb_set_key_type_entries)
    }

    /// Returns the number of elements of the `entries` field of a `xcb_xkb_set_key_type_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_key_type_entries_length(
        &self,
        r: *const xcb_xkb_set_key_type_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_key_type_entries_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_key_type_entries_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_key_type_entries_length(&self) -> bool {
        has_sym!(self, xcb_xkb_set_key_type_entries_length)
    }

    /// Returns an iterator over the elements of the
    /// `entries` field of a `xcb_xkb_set_key_type_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_key_type_entries_iterator(
        &self,
        r: *const xcb_xkb_set_key_type_t,
    ) -> xcb_xkb_kt_set_map_entry_iterator_t {
        sym!(self, xcb_xkb_set_key_type_entries_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_key_type_entries_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_key_type_entries_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_set_key_type_entries_iterator)
    }

    /// Returns a pointer to the `preserve_entries` field of a `xcb_xkb_set_key_type_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_key_type_preserve_entries(
        &self,
        r: *const xcb_xkb_set_key_type_t,
    ) -> *mut xcb_xkb_kt_set_map_entry_t {
        sym!(self, xcb_xkb_set_key_type_preserve_entries)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_key_type_preserve_entries` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_key_type_preserve_entries(&self) -> bool {
        has_sym!(self, xcb_xkb_set_key_type_preserve_entries)
    }

    /// Returns the number of elements of the `preserve_entries` field of a `xcb_xkb_set_key_type_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_key_type_preserve_entries_length(
        &self,
        r: *const xcb_xkb_set_key_type_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_key_type_preserve_entries_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_key_type_preserve_entries_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_key_type_preserve_entries_length(&self) -> bool {
        has_sym!(self, xcb_xkb_set_key_type_preserve_entries_length)
    }

    /// Returns an iterator over the elements of the
    /// `preserve_entries` field of a `xcb_xkb_set_key_type_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_key_type_preserve_entries_iterator(
        &self,
        r: *const xcb_xkb_set_key_type_t,
    ) -> xcb_xkb_kt_set_map_entry_iterator_t {
        sym!(self, xcb_xkb_set_key_type_preserve_entries_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_key_type_preserve_entries_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_key_type_preserve_entries_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_set_key_type_preserve_entries_iterator)
    }

    /// Advances a `xcb_xkb_set_key_type_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_set_key_type_next(&self, i: *mut xcb_xkb_set_key_type_iterator_t) {
        sym!(self, xcb_xkb_set_key_type_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_key_type_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_key_type_next(&self) -> bool {
        has_sym!(self, xcb_xkb_set_key_type_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_set_key_type_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_set_key_type_end(
        &self,
        i: xcb_xkb_set_key_type_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_set_key_type_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_key_type_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_key_type_end(&self) -> bool {
        has_sym!(self, xcb_xkb_set_key_type_end)
    }

    /// Advances a `xcb_xkb_string8_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_string8_next(&self, i: *mut xcb_xkb_string8_iterator_t) {
        sym!(self, xcb_xkb_string8_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_string8_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_string8_next(&self) -> bool {
        has_sym!(self, xcb_xkb_string8_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_string8_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_string8_end(
        &self,
        i: xcb_xkb_string8_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_string8_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_string8_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_string8_end(&self) -> bool {
        has_sym!(self, xcb_xkb_string8_end)
    }

    /// Computes the size of a `xcb_xkb_outline_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_outline_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_outline_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_outline_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_outline_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_outline_sizeof)
    }

    /// Returns a pointer to the `points` field of a `xcb_xkb_outline_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_outline_points(&self, r: *const xcb_xkb_outline_t) -> *mut xcb_point_t {
        sym!(self, xcb_xkb_outline_points)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_outline_points` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_outline_points(&self) -> bool {
        has_sym!(self, xcb_xkb_outline_points)
    }

    /// Returns the number of elements of the `points` field of a `xcb_xkb_outline_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_outline_points_length(&self, r: *const xcb_xkb_outline_t) -> c_int {
        sym!(self, xcb_xkb_outline_points_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_outline_points_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_outline_points_length(&self) -> bool {
        has_sym!(self, xcb_xkb_outline_points_length)
    }

    /// Returns an iterator over the elements of the
    /// `points` field of a `xcb_xkb_outline_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_outline_points_iterator(
        &self,
        r: *const xcb_xkb_outline_t,
    ) -> xcb_point_iterator_t {
        sym!(self, xcb_xkb_outline_points_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_outline_points_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_outline_points_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_outline_points_iterator)
    }

    /// Advances a `xcb_xkb_outline_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_outline_next(&self, i: *mut xcb_xkb_outline_iterator_t) {
        sym!(self, xcb_xkb_outline_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_outline_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_outline_next(&self) -> bool {
        has_sym!(self, xcb_xkb_outline_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_outline_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_outline_end(
        &self,
        i: xcb_xkb_outline_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_outline_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_outline_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_outline_end(&self) -> bool {
        has_sym!(self, xcb_xkb_outline_end)
    }

    /// Computes the size of a `xcb_xkb_shape_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_shape_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_shape_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_shape_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_shape_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_shape_sizeof)
    }

    /// Returns the number of elements of the `outlines` field of a `xcb_xkb_shape_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_shape_outlines_length(&self, r: *const xcb_xkb_shape_t) -> c_int {
        sym!(self, xcb_xkb_shape_outlines_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_shape_outlines_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_shape_outlines_length(&self) -> bool {
        has_sym!(self, xcb_xkb_shape_outlines_length)
    }

    /// Returns an iterator over the elements of the
    /// `outlines` field of a `xcb_xkb_shape_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_shape_outlines_iterator(
        &self,
        r: *const xcb_xkb_shape_t,
    ) -> xcb_xkb_outline_iterator_t {
        sym!(self, xcb_xkb_shape_outlines_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_shape_outlines_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_shape_outlines_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_shape_outlines_iterator)
    }

    /// Advances a `xcb_xkb_shape_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_shape_next(&self, i: *mut xcb_xkb_shape_iterator_t) {
        sym!(self, xcb_xkb_shape_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_shape_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_shape_next(&self) -> bool {
        has_sym!(self, xcb_xkb_shape_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_shape_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_shape_end(&self, i: xcb_xkb_shape_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_shape_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_shape_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_shape_end(&self) -> bool {
        has_sym!(self, xcb_xkb_shape_end)
    }

    /// Advances a `xcb_xkb_key_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_key_next(&self, i: *mut xcb_xkb_key_iterator_t) {
        sym!(self, xcb_xkb_key_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_next(&self) -> bool {
        has_sym!(self, xcb_xkb_key_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_key_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_key_end(&self, i: xcb_xkb_key_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_key_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_end(&self) -> bool {
        has_sym!(self, xcb_xkb_key_end)
    }

    /// Advances a `xcb_xkb_overlay_key_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_overlay_key_next(&self, i: *mut xcb_xkb_overlay_key_iterator_t) {
        sym!(self, xcb_xkb_overlay_key_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_overlay_key_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_overlay_key_next(&self) -> bool {
        has_sym!(self, xcb_xkb_overlay_key_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_overlay_key_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_overlay_key_end(
        &self,
        i: xcb_xkb_overlay_key_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_overlay_key_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_overlay_key_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_overlay_key_end(&self) -> bool {
        has_sym!(self, xcb_xkb_overlay_key_end)
    }

    /// Computes the size of a `xcb_xkb_overlay_row_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_overlay_row_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_overlay_row_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_overlay_row_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_overlay_row_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_overlay_row_sizeof)
    }

    /// Returns a pointer to the `keys` field of a `xcb_xkb_overlay_row_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_overlay_row_keys(
        &self,
        r: *const xcb_xkb_overlay_row_t,
    ) -> *mut xcb_xkb_overlay_key_t {
        sym!(self, xcb_xkb_overlay_row_keys)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_overlay_row_keys` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_overlay_row_keys(&self) -> bool {
        has_sym!(self, xcb_xkb_overlay_row_keys)
    }

    /// Returns the number of elements of the `keys` field of a `xcb_xkb_overlay_row_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_overlay_row_keys_length(&self, r: *const xcb_xkb_overlay_row_t) -> c_int {
        sym!(self, xcb_xkb_overlay_row_keys_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_overlay_row_keys_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_overlay_row_keys_length(&self) -> bool {
        has_sym!(self, xcb_xkb_overlay_row_keys_length)
    }

    /// Returns an iterator over the elements of the
    /// `keys` field of a `xcb_xkb_overlay_row_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_overlay_row_keys_iterator(
        &self,
        r: *const xcb_xkb_overlay_row_t,
    ) -> xcb_xkb_overlay_key_iterator_t {
        sym!(self, xcb_xkb_overlay_row_keys_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_overlay_row_keys_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_overlay_row_keys_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_overlay_row_keys_iterator)
    }

    /// Advances a `xcb_xkb_overlay_row_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_overlay_row_next(&self, i: *mut xcb_xkb_overlay_row_iterator_t) {
        sym!(self, xcb_xkb_overlay_row_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_overlay_row_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_overlay_row_next(&self) -> bool {
        has_sym!(self, xcb_xkb_overlay_row_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_overlay_row_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_overlay_row_end(
        &self,
        i: xcb_xkb_overlay_row_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_overlay_row_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_overlay_row_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_overlay_row_end(&self) -> bool {
        has_sym!(self, xcb_xkb_overlay_row_end)
    }

    /// Computes the size of a `xcb_xkb_overlay_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_overlay_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_overlay_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_overlay_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_overlay_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_overlay_sizeof)
    }

    /// Returns the number of elements of the `rows` field of a `xcb_xkb_overlay_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_overlay_rows_length(&self, r: *const xcb_xkb_overlay_t) -> c_int {
        sym!(self, xcb_xkb_overlay_rows_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_overlay_rows_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_overlay_rows_length(&self) -> bool {
        has_sym!(self, xcb_xkb_overlay_rows_length)
    }

    /// Returns an iterator over the elements of the
    /// `rows` field of a `xcb_xkb_overlay_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_overlay_rows_iterator(
        &self,
        r: *const xcb_xkb_overlay_t,
    ) -> xcb_xkb_overlay_row_iterator_t {
        sym!(self, xcb_xkb_overlay_rows_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_overlay_rows_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_overlay_rows_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_overlay_rows_iterator)
    }

    /// Advances a `xcb_xkb_overlay_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_overlay_next(&self, i: *mut xcb_xkb_overlay_iterator_t) {
        sym!(self, xcb_xkb_overlay_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_overlay_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_overlay_next(&self) -> bool {
        has_sym!(self, xcb_xkb_overlay_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_overlay_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_overlay_end(
        &self,
        i: xcb_xkb_overlay_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_overlay_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_overlay_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_overlay_end(&self) -> bool {
        has_sym!(self, xcb_xkb_overlay_end)
    }

    /// Computes the size of a `xcb_xkb_row_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_row_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_row_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_row_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_row_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_row_sizeof)
    }

    /// Returns a pointer to the `keys` field of a `xcb_xkb_row_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_row_keys(&self, r: *const xcb_xkb_row_t) -> *mut xcb_xkb_key_t {
        sym!(self, xcb_xkb_row_keys)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_row_keys` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_row_keys(&self) -> bool {
        has_sym!(self, xcb_xkb_row_keys)
    }

    /// Returns the number of elements of the `keys` field of a `xcb_xkb_row_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_row_keys_length(&self, r: *const xcb_xkb_row_t) -> c_int {
        sym!(self, xcb_xkb_row_keys_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_row_keys_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_row_keys_length(&self) -> bool {
        has_sym!(self, xcb_xkb_row_keys_length)
    }

    /// Returns an iterator over the elements of the
    /// `keys` field of a `xcb_xkb_row_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_row_keys_iterator(
        &self,
        r: *const xcb_xkb_row_t,
    ) -> xcb_xkb_key_iterator_t {
        sym!(self, xcb_xkb_row_keys_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_row_keys_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_row_keys_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_row_keys_iterator)
    }

    /// Advances a `xcb_xkb_row_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_row_next(&self, i: *mut xcb_xkb_row_iterator_t) {
        sym!(self, xcb_xkb_row_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_row_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_row_next(&self) -> bool {
        has_sym!(self, xcb_xkb_row_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_row_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_row_end(&self, i: xcb_xkb_row_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_row_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_row_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_row_end(&self) -> bool {
        has_sym!(self, xcb_xkb_row_end)
    }

    /// Computes the size of a `xcb_xkb_listing_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_listing_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_listing_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_listing_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_listing_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_listing_sizeof)
    }

    /// Returns a pointer to the `string` field of a `xcb_xkb_listing_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_listing_string(
        &self,
        r: *const xcb_xkb_listing_t,
    ) -> *mut xcb_xkb_string8_t {
        sym!(self, xcb_xkb_listing_string)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_listing_string` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_listing_string(&self) -> bool {
        has_sym!(self, xcb_xkb_listing_string)
    }

    /// Returns the number of elements of the `string` field of a `xcb_xkb_listing_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_listing_string_length(&self, r: *const xcb_xkb_listing_t) -> c_int {
        sym!(self, xcb_xkb_listing_string_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_listing_string_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_listing_string_length(&self) -> bool {
        has_sym!(self, xcb_xkb_listing_string_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `string` field of a `xcb_xkb_listing_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_listing_string_end(
        &self,
        r: *const xcb_xkb_listing_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_listing_string_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_listing_string_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_listing_string_end(&self) -> bool {
        has_sym!(self, xcb_xkb_listing_string_end)
    }

    /// Advances a `xcb_xkb_listing_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_listing_next(&self, i: *mut xcb_xkb_listing_iterator_t) {
        sym!(self, xcb_xkb_listing_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_listing_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_listing_next(&self) -> bool {
        has_sym!(self, xcb_xkb_listing_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_listing_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_listing_end(
        &self,
        i: xcb_xkb_listing_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_listing_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_listing_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_listing_end(&self) -> bool {
        has_sym!(self, xcb_xkb_listing_end)
    }

    /// Computes the size of a `xcb_xkb_device_led_info_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_device_led_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_device_led_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_device_led_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_device_led_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_device_led_info_sizeof)
    }

    /// Returns a pointer to the `names` field of a `xcb_xkb_device_led_info_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_device_led_info_names(
        &self,
        r: *const xcb_xkb_device_led_info_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_xkb_device_led_info_names)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_device_led_info_names` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_device_led_info_names(&self) -> bool {
        has_sym!(self, xcb_xkb_device_led_info_names)
    }

    /// Returns the number of elements of the `names` field of a `xcb_xkb_device_led_info_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_device_led_info_names_length(
        &self,
        r: *const xcb_xkb_device_led_info_t,
    ) -> c_int {
        sym!(self, xcb_xkb_device_led_info_names_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_device_led_info_names_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_device_led_info_names_length(&self) -> bool {
        has_sym!(self, xcb_xkb_device_led_info_names_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `names` field of a `xcb_xkb_device_led_info_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_device_led_info_names_end(
        &self,
        r: *const xcb_xkb_device_led_info_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_device_led_info_names_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_device_led_info_names_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_device_led_info_names_end(&self) -> bool {
        has_sym!(self, xcb_xkb_device_led_info_names_end)
    }

    /// Returns a pointer to the `maps` field of a `xcb_xkb_device_led_info_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_device_led_info_maps(
        &self,
        r: *const xcb_xkb_device_led_info_t,
    ) -> *mut xcb_xkb_indicator_map_t {
        sym!(self, xcb_xkb_device_led_info_maps)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_device_led_info_maps` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_device_led_info_maps(&self) -> bool {
        has_sym!(self, xcb_xkb_device_led_info_maps)
    }

    /// Returns the number of elements of the `maps` field of a `xcb_xkb_device_led_info_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_device_led_info_maps_length(
        &self,
        r: *const xcb_xkb_device_led_info_t,
    ) -> c_int {
        sym!(self, xcb_xkb_device_led_info_maps_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_device_led_info_maps_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_device_led_info_maps_length(&self) -> bool {
        has_sym!(self, xcb_xkb_device_led_info_maps_length)
    }

    /// Returns an iterator over the elements of the
    /// `maps` field of a `xcb_xkb_device_led_info_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_device_led_info_maps_iterator(
        &self,
        r: *const xcb_xkb_device_led_info_t,
    ) -> xcb_xkb_indicator_map_iterator_t {
        sym!(self, xcb_xkb_device_led_info_maps_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_device_led_info_maps_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_device_led_info_maps_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_device_led_info_maps_iterator)
    }

    /// Advances a `xcb_xkb_device_led_info_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_device_led_info_next(&self, i: *mut xcb_xkb_device_led_info_iterator_t) {
        sym!(self, xcb_xkb_device_led_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_device_led_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_device_led_info_next(&self) -> bool {
        has_sym!(self, xcb_xkb_device_led_info_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_device_led_info_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_device_led_info_end(
        &self,
        i: xcb_xkb_device_led_info_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_device_led_info_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_device_led_info_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_device_led_info_end(&self) -> bool {
        has_sym!(self, xcb_xkb_device_led_info_end)
    }

    /// Advances a `xcb_xkb_sa_no_action_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_sa_no_action_next(&self, i: *mut xcb_xkb_sa_no_action_iterator_t) {
        sym!(self, xcb_xkb_sa_no_action_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_no_action_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_no_action_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_no_action_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_sa_no_action_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_sa_no_action_end(
        &self,
        i: xcb_xkb_sa_no_action_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_sa_no_action_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_no_action_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_no_action_end(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_no_action_end)
    }

    /// Advances a `xcb_xkb_sa_set_mods_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_sa_set_mods_next(&self, i: *mut xcb_xkb_sa_set_mods_iterator_t) {
        sym!(self, xcb_xkb_sa_set_mods_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_set_mods_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_set_mods_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_set_mods_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_sa_set_mods_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_sa_set_mods_end(
        &self,
        i: xcb_xkb_sa_set_mods_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_sa_set_mods_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_set_mods_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_set_mods_end(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_set_mods_end)
    }

    /// Advances a `xcb_xkb_sa_latch_mods_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_sa_latch_mods_next(&self, i: *mut xcb_xkb_sa_latch_mods_iterator_t) {
        sym!(self, xcb_xkb_sa_latch_mods_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_latch_mods_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_latch_mods_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_latch_mods_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_sa_latch_mods_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_sa_latch_mods_end(
        &self,
        i: xcb_xkb_sa_latch_mods_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_sa_latch_mods_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_latch_mods_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_latch_mods_end(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_latch_mods_end)
    }

    /// Advances a `xcb_xkb_sa_lock_mods_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_sa_lock_mods_next(&self, i: *mut xcb_xkb_sa_lock_mods_iterator_t) {
        sym!(self, xcb_xkb_sa_lock_mods_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_lock_mods_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_lock_mods_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_lock_mods_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_sa_lock_mods_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_sa_lock_mods_end(
        &self,
        i: xcb_xkb_sa_lock_mods_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_sa_lock_mods_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_lock_mods_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_lock_mods_end(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_lock_mods_end)
    }

    /// Advances a `xcb_xkb_sa_set_group_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_sa_set_group_next(&self, i: *mut xcb_xkb_sa_set_group_iterator_t) {
        sym!(self, xcb_xkb_sa_set_group_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_set_group_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_set_group_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_set_group_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_sa_set_group_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_sa_set_group_end(
        &self,
        i: xcb_xkb_sa_set_group_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_sa_set_group_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_set_group_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_set_group_end(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_set_group_end)
    }

    /// Advances a `xcb_xkb_sa_latch_group_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_sa_latch_group_next(&self, i: *mut xcb_xkb_sa_latch_group_iterator_t) {
        sym!(self, xcb_xkb_sa_latch_group_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_latch_group_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_latch_group_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_latch_group_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_sa_latch_group_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_sa_latch_group_end(
        &self,
        i: xcb_xkb_sa_latch_group_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_sa_latch_group_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_latch_group_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_latch_group_end(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_latch_group_end)
    }

    /// Advances a `xcb_xkb_sa_lock_group_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_sa_lock_group_next(&self, i: *mut xcb_xkb_sa_lock_group_iterator_t) {
        sym!(self, xcb_xkb_sa_lock_group_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_lock_group_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_lock_group_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_lock_group_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_sa_lock_group_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_sa_lock_group_end(
        &self,
        i: xcb_xkb_sa_lock_group_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_sa_lock_group_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_lock_group_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_lock_group_end(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_lock_group_end)
    }

    /// Advances a `xcb_xkb_sa_move_ptr_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_sa_move_ptr_next(&self, i: *mut xcb_xkb_sa_move_ptr_iterator_t) {
        sym!(self, xcb_xkb_sa_move_ptr_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_move_ptr_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_move_ptr_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_move_ptr_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_sa_move_ptr_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_sa_move_ptr_end(
        &self,
        i: xcb_xkb_sa_move_ptr_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_sa_move_ptr_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_move_ptr_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_move_ptr_end(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_move_ptr_end)
    }

    /// Advances a `xcb_xkb_sa_ptr_btn_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_sa_ptr_btn_next(&self, i: *mut xcb_xkb_sa_ptr_btn_iterator_t) {
        sym!(self, xcb_xkb_sa_ptr_btn_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_ptr_btn_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_ptr_btn_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_ptr_btn_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_sa_ptr_btn_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_sa_ptr_btn_end(
        &self,
        i: xcb_xkb_sa_ptr_btn_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_sa_ptr_btn_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_ptr_btn_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_ptr_btn_end(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_ptr_btn_end)
    }

    /// Advances a `xcb_xkb_sa_lock_ptr_btn_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_sa_lock_ptr_btn_next(&self, i: *mut xcb_xkb_sa_lock_ptr_btn_iterator_t) {
        sym!(self, xcb_xkb_sa_lock_ptr_btn_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_lock_ptr_btn_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_lock_ptr_btn_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_lock_ptr_btn_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_sa_lock_ptr_btn_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_sa_lock_ptr_btn_end(
        &self,
        i: xcb_xkb_sa_lock_ptr_btn_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_sa_lock_ptr_btn_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_lock_ptr_btn_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_lock_ptr_btn_end(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_lock_ptr_btn_end)
    }

    /// Advances a `xcb_xkb_sa_set_ptr_dflt_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_sa_set_ptr_dflt_next(&self, i: *mut xcb_xkb_sa_set_ptr_dflt_iterator_t) {
        sym!(self, xcb_xkb_sa_set_ptr_dflt_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_set_ptr_dflt_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_set_ptr_dflt_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_set_ptr_dflt_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_sa_set_ptr_dflt_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_sa_set_ptr_dflt_end(
        &self,
        i: xcb_xkb_sa_set_ptr_dflt_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_sa_set_ptr_dflt_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_set_ptr_dflt_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_set_ptr_dflt_end(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_set_ptr_dflt_end)
    }

    /// Advances a `xcb_xkb_sa_iso_lock_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_sa_iso_lock_next(&self, i: *mut xcb_xkb_sa_iso_lock_iterator_t) {
        sym!(self, xcb_xkb_sa_iso_lock_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_iso_lock_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_iso_lock_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_iso_lock_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_sa_iso_lock_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_sa_iso_lock_end(
        &self,
        i: xcb_xkb_sa_iso_lock_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_sa_iso_lock_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_iso_lock_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_iso_lock_end(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_iso_lock_end)
    }

    /// Advances a `xcb_xkb_sa_terminate_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_sa_terminate_next(&self, i: *mut xcb_xkb_sa_terminate_iterator_t) {
        sym!(self, xcb_xkb_sa_terminate_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_terminate_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_terminate_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_terminate_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_sa_terminate_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_sa_terminate_end(
        &self,
        i: xcb_xkb_sa_terminate_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_sa_terminate_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_terminate_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_terminate_end(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_terminate_end)
    }

    /// Advances a `xcb_xkb_sa_switch_screen_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_sa_switch_screen_next(
        &self,
        i: *mut xcb_xkb_sa_switch_screen_iterator_t,
    ) {
        sym!(self, xcb_xkb_sa_switch_screen_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_switch_screen_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_switch_screen_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_switch_screen_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_sa_switch_screen_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_sa_switch_screen_end(
        &self,
        i: xcb_xkb_sa_switch_screen_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_sa_switch_screen_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_switch_screen_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_switch_screen_end(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_switch_screen_end)
    }

    /// Advances a `xcb_xkb_sa_set_controls_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_sa_set_controls_next(&self, i: *mut xcb_xkb_sa_set_controls_iterator_t) {
        sym!(self, xcb_xkb_sa_set_controls_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_set_controls_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_set_controls_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_set_controls_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_sa_set_controls_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_sa_set_controls_end(
        &self,
        i: xcb_xkb_sa_set_controls_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_sa_set_controls_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_set_controls_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_set_controls_end(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_set_controls_end)
    }

    /// Advances a `xcb_xkb_sa_lock_controls_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_sa_lock_controls_next(
        &self,
        i: *mut xcb_xkb_sa_lock_controls_iterator_t,
    ) {
        sym!(self, xcb_xkb_sa_lock_controls_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_lock_controls_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_lock_controls_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_lock_controls_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_sa_lock_controls_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_sa_lock_controls_end(
        &self,
        i: xcb_xkb_sa_lock_controls_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_sa_lock_controls_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_lock_controls_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_lock_controls_end(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_lock_controls_end)
    }

    /// Advances a `xcb_xkb_sa_action_message_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_sa_action_message_next(
        &self,
        i: *mut xcb_xkb_sa_action_message_iterator_t,
    ) {
        sym!(self, xcb_xkb_sa_action_message_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_action_message_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_action_message_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_action_message_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_sa_action_message_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_sa_action_message_end(
        &self,
        i: xcb_xkb_sa_action_message_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_sa_action_message_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_action_message_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_action_message_end(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_action_message_end)
    }

    /// Advances a `xcb_xkb_sa_redirect_key_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_sa_redirect_key_next(&self, i: *mut xcb_xkb_sa_redirect_key_iterator_t) {
        sym!(self, xcb_xkb_sa_redirect_key_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_redirect_key_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_redirect_key_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_redirect_key_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_sa_redirect_key_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_sa_redirect_key_end(
        &self,
        i: xcb_xkb_sa_redirect_key_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_sa_redirect_key_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_redirect_key_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_redirect_key_end(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_redirect_key_end)
    }

    /// Advances a `xcb_xkb_sa_device_btn_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_sa_device_btn_next(&self, i: *mut xcb_xkb_sa_device_btn_iterator_t) {
        sym!(self, xcb_xkb_sa_device_btn_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_device_btn_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_device_btn_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_device_btn_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_sa_device_btn_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_sa_device_btn_end(
        &self,
        i: xcb_xkb_sa_device_btn_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_sa_device_btn_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_device_btn_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_device_btn_end(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_device_btn_end)
    }

    /// Advances a `xcb_xkb_sa_lock_device_btn_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_sa_lock_device_btn_next(
        &self,
        i: *mut xcb_xkb_sa_lock_device_btn_iterator_t,
    ) {
        sym!(self, xcb_xkb_sa_lock_device_btn_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_lock_device_btn_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_lock_device_btn_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_lock_device_btn_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_sa_lock_device_btn_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_sa_lock_device_btn_end(
        &self,
        i: xcb_xkb_sa_lock_device_btn_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_sa_lock_device_btn_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_lock_device_btn_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_lock_device_btn_end(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_lock_device_btn_end)
    }

    /// Advances a `xcb_xkb_sa_device_valuator_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_sa_device_valuator_next(
        &self,
        i: *mut xcb_xkb_sa_device_valuator_iterator_t,
    ) {
        sym!(self, xcb_xkb_sa_device_valuator_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_device_valuator_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_device_valuator_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_device_valuator_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_sa_device_valuator_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_sa_device_valuator_end(
        &self,
        i: xcb_xkb_sa_device_valuator_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_sa_device_valuator_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_device_valuator_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_device_valuator_end(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_device_valuator_end)
    }

    /// Advances a `xcb_xkb_si_action_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_si_action_next(&self, i: *mut xcb_xkb_si_action_iterator_t) {
        sym!(self, xcb_xkb_si_action_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_si_action_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_si_action_next(&self) -> bool {
        has_sym!(self, xcb_xkb_si_action_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_si_action_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_si_action_end(
        &self,
        i: xcb_xkb_si_action_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_si_action_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_si_action_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_si_action_end(&self) -> bool {
        has_sym!(self, xcb_xkb_si_action_end)
    }

    /// Advances a `xcb_xkb_sym_interpret_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_sym_interpret_next(&self, i: *mut xcb_xkb_sym_interpret_iterator_t) {
        sym!(self, xcb_xkb_sym_interpret_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sym_interpret_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sym_interpret_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sym_interpret_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_sym_interpret_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_sym_interpret_end(
        &self,
        i: xcb_xkb_sym_interpret_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_sym_interpret_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sym_interpret_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sym_interpret_end(&self) -> bool {
        has_sym!(self, xcb_xkb_sym_interpret_end)
    }

    /// Advances a `xcb_xkb_action_iterator_t` iterator by 1 element.
    #[inline]
    pub unsafe fn xcb_xkb_action_next(&self, i: *mut xcb_xkb_action_iterator_t) {
        sym!(self, xcb_xkb_action_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_action_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_action_next(&self) -> bool {
        has_sym!(self, xcb_xkb_action_next)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of a `xcb_xkb_action_iterator_t`.
    #[inline]
    pub unsafe fn xcb_xkb_action_end(
        &self,
        i: xcb_xkb_action_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_action_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_action_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_action_end(&self) -> bool {
        has_sym!(self, xcb_xkb_action_end)
    }

    /// Sends a `xkb::UseExtension` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xkb_use_extension_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xkb_use_extension_reply`]: Self::xcb_xkb_use_extension_reply
    #[inline]
    pub unsafe fn xcb_xkb_use_extension(
        &self,
        c: *mut xcb_connection_t,
        wanted_major: u16,
        wanted_minor: u16,
    ) -> xcb_xkb_use_extension_cookie_t {
        sym!(self, xcb_xkb_use_extension)(c, wanted_major, wanted_minor)
    }

    /// Returns `true` iff the symbol `xcb_xkb_use_extension` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_use_extension(&self) -> bool {
        has_sym!(self, xcb_xkb_use_extension)
    }

    /// Sends a `xkb::UseExtension` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xkb_use_extension_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xkb_use_extension_reply`]: Self::xcb_xkb_use_extension_reply
    #[inline]
    pub unsafe fn xcb_xkb_use_extension_unchecked(
        &self,
        c: *mut xcb_connection_t,
        wanted_major: u16,
        wanted_minor: u16,
    ) -> xcb_xkb_use_extension_cookie_t {
        sym!(self, xcb_xkb_use_extension_unchecked)(c, wanted_major, wanted_minor)
    }

    /// Returns `true` iff the symbol `xcb_xkb_use_extension_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_use_extension_unchecked(&self) -> bool {
        has_sym!(self, xcb_xkb_use_extension_unchecked)
    }

    /// Waits for the reply to a `xkb::UseExtension` request.
    #[inline]
    pub unsafe fn xcb_xkb_use_extension_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_use_extension_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_use_extension_reply_t {
        sym!(self, xcb_xkb_use_extension_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xkb_use_extension_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_use_extension_reply(&self) -> bool {
        has_sym!(self, xcb_xkb_use_extension_reply)
    }

    /// Serializes a `xcb_xkb_select_events_details_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_select_events_details_serialize(
        &self,
        _buffer: *mut *mut c_void,
        affectWhich: u16,
        clear: u16,
        selectAll: u16,
        _aux: *const xcb_xkb_select_events_details_t,
    ) -> c_int {
        sym!(self, xcb_xkb_select_events_details_serialize)(
            _buffer,
            affectWhich,
            clear,
            selectAll,
            _aux,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_select_events_details_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_select_events_details_serialize(&self) -> bool {
        has_sym!(self, xcb_xkb_select_events_details_serialize)
    }

    /// Unpacks a `xcb_xkb_select_events_details_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_select_events_details_unpack(
        &self,
        _buffer: *const c_void,
        affectWhich: u16,
        clear: u16,
        selectAll: u16,
        _aux: *mut xcb_xkb_select_events_details_t,
    ) -> c_int {
        sym!(self, xcb_xkb_select_events_details_unpack)(
            _buffer,
            affectWhich,
            clear,
            selectAll,
            _aux,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_select_events_details_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_select_events_details_unpack(&self) -> bool {
        has_sym!(self, xcb_xkb_select_events_details_unpack)
    }

    /// Computes the size of a `xcb_xkb_select_events_details_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_select_events_details_sizeof(
        &self,
        _buffer: *const c_void,
        affectWhich: u16,
        clear: u16,
        selectAll: u16,
    ) -> c_int {
        sym!(self, xcb_xkb_select_events_details_sizeof)(_buffer, affectWhich, clear, selectAll)
    }

    /// Returns `true` iff the symbol `xcb_xkb_select_events_details_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_select_events_details_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_select_events_details_sizeof)
    }

    /// Computes the size of a `xcb_xkb_select_events_request_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_select_events_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_select_events_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_select_events_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_select_events_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_select_events_sizeof)
    }

    /// Sends a `xkb::SelectEvents` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    ///
    /// There is an auxiliary version of this function: [`xcb_xkb_select_events_aux_checked`].
    ///
    /// [`xcb_xkb_select_events_aux_checked`]: Self::xcb_xkb_select_events_aux_checked
    #[inline]
    pub unsafe fn xcb_xkb_select_events_checked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        affect_which: u16,
        clear: u16,
        select_all: u16,
        affect_map: u16,
        map: u16,
        details: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_select_events_checked)(
            c,
            device_spec,
            affect_which,
            clear,
            select_all,
            affect_map,
            map,
            details,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_select_events_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_select_events_checked(&self) -> bool {
        has_sym!(self, xcb_xkb_select_events_checked)
    }

    /// Sends a `xkb::SelectEvents` request (unchecked).
    ///
    /// There is an auxiliary version of this function: [`xcb_xkb_select_events_aux`].
    ///
    /// [`xcb_xkb_select_events_aux`]: Self::xcb_xkb_select_events_aux
    #[inline]
    pub unsafe fn xcb_xkb_select_events(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        affect_which: u16,
        clear: u16,
        select_all: u16,
        affect_map: u16,
        map: u16,
        details: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_select_events)(
            c,
            device_spec,
            affect_which,
            clear,
            select_all,
            affect_map,
            map,
            details,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_select_events` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_select_events(&self) -> bool {
        has_sym!(self, xcb_xkb_select_events)
    }

    /// Sends a `xkb::SelectEvents` request (checked) (aux).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xkb_select_events_aux_checked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        affect_which: u16,
        clear: u16,
        select_all: u16,
        affect_map: u16,
        map: u16,
        details: *const xcb_xkb_select_events_details_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_select_events_aux_checked)(
            c,
            device_spec,
            affect_which,
            clear,
            select_all,
            affect_map,
            map,
            details,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_select_events_aux_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_select_events_aux_checked(&self) -> bool {
        has_sym!(self, xcb_xkb_select_events_aux_checked)
    }

    /// Sends a `xkb::SelectEvents` request (unchecked) (aux).
    #[inline]
    pub unsafe fn xcb_xkb_select_events_aux(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        affect_which: u16,
        clear: u16,
        select_all: u16,
        affect_map: u16,
        map: u16,
        details: *const xcb_xkb_select_events_details_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_select_events_aux)(
            c,
            device_spec,
            affect_which,
            clear,
            select_all,
            affect_map,
            map,
            details,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_select_events_aux` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_select_events_aux(&self) -> bool {
        has_sym!(self, xcb_xkb_select_events_aux)
    }

    /// Returns a pointer to the `details` field of a `xcb_xkb_select_events_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_select_events_details(
        &self,
        r: *const xcb_xkb_select_events_request_t,
    ) -> *mut c_void {
        sym!(self, xcb_xkb_select_events_details)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_select_events_details` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_select_events_details(&self) -> bool {
        has_sym!(self, xcb_xkb_select_events_details)
    }

    /// Sends a `xkb::Bell` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xkb_bell_checked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        bell_class: xcb_xkb_bell_class_spec_t,
        bell_id: xcb_xkb_id_spec_t,
        percent: i8,
        force_sound: u8,
        event_only: u8,
        pitch: i16,
        duration: i16,
        name: xcb_atom_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_bell_checked)(
            c,
            device_spec,
            bell_class,
            bell_id,
            percent,
            force_sound,
            event_only,
            pitch,
            duration,
            name,
            window,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_bell_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_bell_checked(&self) -> bool {
        has_sym!(self, xcb_xkb_bell_checked)
    }

    /// Sends a `xkb::Bell` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xkb_bell(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        bell_class: xcb_xkb_bell_class_spec_t,
        bell_id: xcb_xkb_id_spec_t,
        percent: i8,
        force_sound: u8,
        event_only: u8,
        pitch: i16,
        duration: i16,
        name: xcb_atom_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_bell)(
            c,
            device_spec,
            bell_class,
            bell_id,
            percent,
            force_sound,
            event_only,
            pitch,
            duration,
            name,
            window,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_bell` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_bell(&self) -> bool {
        has_sym!(self, xcb_xkb_bell)
    }

    /// Sends a `xkb::GetState` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xkb_get_state_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xkb_get_state_reply`]: Self::xcb_xkb_get_state_reply
    #[inline]
    pub unsafe fn xcb_xkb_get_state(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
    ) -> xcb_xkb_get_state_cookie_t {
        sym!(self, xcb_xkb_get_state)(c, device_spec)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_state` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_state(&self) -> bool {
        has_sym!(self, xcb_xkb_get_state)
    }

    /// Sends a `xkb::GetState` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xkb_get_state_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xkb_get_state_reply`]: Self::xcb_xkb_get_state_reply
    #[inline]
    pub unsafe fn xcb_xkb_get_state_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
    ) -> xcb_xkb_get_state_cookie_t {
        sym!(self, xcb_xkb_get_state_unchecked)(c, device_spec)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_state_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_state_unchecked(&self) -> bool {
        has_sym!(self, xcb_xkb_get_state_unchecked)
    }

    /// Waits for the reply to a `xkb::GetState` request.
    #[inline]
    pub unsafe fn xcb_xkb_get_state_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_state_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_get_state_reply_t {
        sym!(self, xcb_xkb_get_state_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_state_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_state_reply(&self) -> bool {
        has_sym!(self, xcb_xkb_get_state_reply)
    }

    /// Sends a `xkb::LatchLockState` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xkb_latch_lock_state_checked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        affect_mod_locks: u8,
        mod_locks: u8,
        lock_group: u8,
        group_lock: u8,
        affect_mod_latches: u8,
        latch_group: u8,
        group_latch: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_latch_lock_state_checked)(
            c,
            device_spec,
            affect_mod_locks,
            mod_locks,
            lock_group,
            group_lock,
            affect_mod_latches,
            latch_group,
            group_latch,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_latch_lock_state_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_latch_lock_state_checked(&self) -> bool {
        has_sym!(self, xcb_xkb_latch_lock_state_checked)
    }

    /// Sends a `xkb::LatchLockState` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xkb_latch_lock_state(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        affect_mod_locks: u8,
        mod_locks: u8,
        lock_group: u8,
        group_lock: u8,
        affect_mod_latches: u8,
        latch_group: u8,
        group_latch: u16,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_latch_lock_state)(
            c,
            device_spec,
            affect_mod_locks,
            mod_locks,
            lock_group,
            group_lock,
            affect_mod_latches,
            latch_group,
            group_latch,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_latch_lock_state` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_latch_lock_state(&self) -> bool {
        has_sym!(self, xcb_xkb_latch_lock_state)
    }

    /// Sends a `xkb::GetControls` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xkb_get_controls_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xkb_get_controls_reply`]: Self::xcb_xkb_get_controls_reply
    #[inline]
    pub unsafe fn xcb_xkb_get_controls(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
    ) -> xcb_xkb_get_controls_cookie_t {
        sym!(self, xcb_xkb_get_controls)(c, device_spec)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_controls` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_controls(&self) -> bool {
        has_sym!(self, xcb_xkb_get_controls)
    }

    /// Sends a `xkb::GetControls` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xkb_get_controls_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xkb_get_controls_reply`]: Self::xcb_xkb_get_controls_reply
    #[inline]
    pub unsafe fn xcb_xkb_get_controls_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
    ) -> xcb_xkb_get_controls_cookie_t {
        sym!(self, xcb_xkb_get_controls_unchecked)(c, device_spec)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_controls_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_controls_unchecked(&self) -> bool {
        has_sym!(self, xcb_xkb_get_controls_unchecked)
    }

    /// Waits for the reply to a `xkb::GetControls` request.
    #[inline]
    pub unsafe fn xcb_xkb_get_controls_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_controls_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_get_controls_reply_t {
        sym!(self, xcb_xkb_get_controls_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_controls_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_controls_reply(&self) -> bool {
        has_sym!(self, xcb_xkb_get_controls_reply)
    }

    /// Sends a `xkb::SetControls` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xkb_set_controls_checked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        affect_internal_real_mods: u8,
        internal_real_mods: u8,
        affect_ignore_lock_real_mods: u8,
        ignore_lock_real_mods: u8,
        affect_internal_virtual_mods: u16,
        internal_virtual_mods: u16,
        affect_ignore_lock_virtual_mods: u16,
        ignore_lock_virtual_mods: u16,
        mouse_keys_dflt_btn: u8,
        groups_wrap: u8,
        access_xoptions: u16,
        affect_enabled_controls: u32,
        enabled_controls: u32,
        change_controls: u32,
        repeat_delay: u16,
        repeat_interval: u16,
        slow_keys_delay: u16,
        debounce_delay: u16,
        mouse_keys_delay: u16,
        mouse_keys_interval: u16,
        mouse_keys_time_to_max: u16,
        mouse_keys_max_speed: u16,
        mouse_keys_curve: i16,
        access_xtimeout: u16,
        access_xtimeout_mask: u32,
        access_xtimeout_values: u32,
        access_xtimeout_options_mask: u16,
        access_xtimeout_options_values: u16,
        per_key_repeat: *const [u8; 32],
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_set_controls_checked)(
            c,
            device_spec,
            affect_internal_real_mods,
            internal_real_mods,
            affect_ignore_lock_real_mods,
            ignore_lock_real_mods,
            affect_internal_virtual_mods,
            internal_virtual_mods,
            affect_ignore_lock_virtual_mods,
            ignore_lock_virtual_mods,
            mouse_keys_dflt_btn,
            groups_wrap,
            access_xoptions,
            affect_enabled_controls,
            enabled_controls,
            change_controls,
            repeat_delay,
            repeat_interval,
            slow_keys_delay,
            debounce_delay,
            mouse_keys_delay,
            mouse_keys_interval,
            mouse_keys_time_to_max,
            mouse_keys_max_speed,
            mouse_keys_curve,
            access_xtimeout,
            access_xtimeout_mask,
            access_xtimeout_values,
            access_xtimeout_options_mask,
            access_xtimeout_options_values,
            per_key_repeat,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_controls_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_controls_checked(&self) -> bool {
        has_sym!(self, xcb_xkb_set_controls_checked)
    }

    /// Sends a `xkb::SetControls` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xkb_set_controls(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        affect_internal_real_mods: u8,
        internal_real_mods: u8,
        affect_ignore_lock_real_mods: u8,
        ignore_lock_real_mods: u8,
        affect_internal_virtual_mods: u16,
        internal_virtual_mods: u16,
        affect_ignore_lock_virtual_mods: u16,
        ignore_lock_virtual_mods: u16,
        mouse_keys_dflt_btn: u8,
        groups_wrap: u8,
        access_xoptions: u16,
        affect_enabled_controls: u32,
        enabled_controls: u32,
        change_controls: u32,
        repeat_delay: u16,
        repeat_interval: u16,
        slow_keys_delay: u16,
        debounce_delay: u16,
        mouse_keys_delay: u16,
        mouse_keys_interval: u16,
        mouse_keys_time_to_max: u16,
        mouse_keys_max_speed: u16,
        mouse_keys_curve: i16,
        access_xtimeout: u16,
        access_xtimeout_mask: u32,
        access_xtimeout_values: u32,
        access_xtimeout_options_mask: u16,
        access_xtimeout_options_values: u16,
        per_key_repeat: *const [u8; 32],
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_set_controls)(
            c,
            device_spec,
            affect_internal_real_mods,
            internal_real_mods,
            affect_ignore_lock_real_mods,
            ignore_lock_real_mods,
            affect_internal_virtual_mods,
            internal_virtual_mods,
            affect_ignore_lock_virtual_mods,
            ignore_lock_virtual_mods,
            mouse_keys_dflt_btn,
            groups_wrap,
            access_xoptions,
            affect_enabled_controls,
            enabled_controls,
            change_controls,
            repeat_delay,
            repeat_interval,
            slow_keys_delay,
            debounce_delay,
            mouse_keys_delay,
            mouse_keys_interval,
            mouse_keys_time_to_max,
            mouse_keys_max_speed,
            mouse_keys_curve,
            access_xtimeout,
            access_xtimeout_mask,
            access_xtimeout_values,
            access_xtimeout_options_mask,
            access_xtimeout_options_values,
            per_key_repeat,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_controls` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_controls(&self) -> bool {
        has_sym!(self, xcb_xkb_set_controls)
    }

    /// Returns the number of elements of the `types_rtrn` field of a `xcb_xkb_get_map_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map_types_rtrn_length(
        &self,
        r: *const xcb_xkb_get_map_reply_t,
        s: *const xcb_xkb_get_map_map_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_map_map_types_rtrn_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map_types_rtrn_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map_types_rtrn_length(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map_types_rtrn_length)
    }

    /// Returns an iterator over the elements of the
    /// `types_rtrn` field of a `xcb_xkb_get_map_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map_types_rtrn_iterator(
        &self,
        r: *const xcb_xkb_get_map_reply_t,
        s: *const xcb_xkb_get_map_map_t,
    ) -> xcb_xkb_key_type_iterator_t {
        sym!(self, xcb_xkb_get_map_map_types_rtrn_iterator)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map_types_rtrn_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map_types_rtrn_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map_types_rtrn_iterator)
    }

    /// Returns the number of elements of the `syms_rtrn` field of a `xcb_xkb_get_map_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map_syms_rtrn_length(
        &self,
        r: *const xcb_xkb_get_map_reply_t,
        s: *const xcb_xkb_get_map_map_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_map_map_syms_rtrn_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map_syms_rtrn_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map_syms_rtrn_length(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map_syms_rtrn_length)
    }

    /// Returns an iterator over the elements of the
    /// `syms_rtrn` field of a `xcb_xkb_get_map_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map_syms_rtrn_iterator(
        &self,
        r: *const xcb_xkb_get_map_reply_t,
        s: *const xcb_xkb_get_map_map_t,
    ) -> xcb_xkb_key_sym_map_iterator_t {
        sym!(self, xcb_xkb_get_map_map_syms_rtrn_iterator)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map_syms_rtrn_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map_syms_rtrn_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map_syms_rtrn_iterator)
    }

    /// Returns a pointer to the `acts_rtrn_count` field of a `xcb_xkb_get_map_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map_acts_rtrn_count(
        &self,
        s: *const xcb_xkb_get_map_map_t,
    ) -> *mut u8 {
        sym!(self, xcb_xkb_get_map_map_acts_rtrn_count)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map_acts_rtrn_count` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map_acts_rtrn_count(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map_acts_rtrn_count)
    }

    /// Returns the number of elements of the `acts_rtrn_count` field of a `xcb_xkb_get_map_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map_acts_rtrn_count_length(
        &self,
        r: *const xcb_xkb_get_map_reply_t,
        s: *const xcb_xkb_get_map_map_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_map_map_acts_rtrn_count_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map_acts_rtrn_count_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map_acts_rtrn_count_length(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map_acts_rtrn_count_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `acts_rtrn_count` field of a `xcb_xkb_get_map_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map_acts_rtrn_count_end(
        &self,
        r: *const xcb_xkb_get_map_reply_t,
        s: *const xcb_xkb_get_map_map_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_get_map_map_acts_rtrn_count_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map_acts_rtrn_count_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map_acts_rtrn_count_end(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map_acts_rtrn_count_end)
    }

    /// Returns a pointer to the `acts_rtrn_acts` field of a `xcb_xkb_get_map_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map_acts_rtrn_acts(
        &self,
        s: *const xcb_xkb_get_map_map_t,
    ) -> *mut xcb_xkb_action_t {
        sym!(self, xcb_xkb_get_map_map_acts_rtrn_acts)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map_acts_rtrn_acts` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map_acts_rtrn_acts(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map_acts_rtrn_acts)
    }

    /// Returns the number of elements of the `acts_rtrn_acts` field of a `xcb_xkb_get_map_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map_acts_rtrn_acts_length(
        &self,
        r: *const xcb_xkb_get_map_reply_t,
        s: *const xcb_xkb_get_map_map_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_map_map_acts_rtrn_acts_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map_acts_rtrn_acts_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map_acts_rtrn_acts_length(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map_acts_rtrn_acts_length)
    }

    /// Returns an iterator over the elements of the
    /// `acts_rtrn_acts` field of a `xcb_xkb_get_map_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map_acts_rtrn_acts_iterator(
        &self,
        r: *const xcb_xkb_get_map_reply_t,
        s: *const xcb_xkb_get_map_map_t,
    ) -> xcb_xkb_action_iterator_t {
        sym!(self, xcb_xkb_get_map_map_acts_rtrn_acts_iterator)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map_acts_rtrn_acts_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map_acts_rtrn_acts_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map_acts_rtrn_acts_iterator)
    }

    /// Returns a pointer to the `behaviors_rtrn` field of a `xcb_xkb_get_map_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map_behaviors_rtrn(
        &self,
        s: *const xcb_xkb_get_map_map_t,
    ) -> *mut xcb_xkb_set_behavior_t {
        sym!(self, xcb_xkb_get_map_map_behaviors_rtrn)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map_behaviors_rtrn` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map_behaviors_rtrn(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map_behaviors_rtrn)
    }

    /// Returns the number of elements of the `behaviors_rtrn` field of a `xcb_xkb_get_map_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map_behaviors_rtrn_length(
        &self,
        r: *const xcb_xkb_get_map_reply_t,
        s: *const xcb_xkb_get_map_map_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_map_map_behaviors_rtrn_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map_behaviors_rtrn_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map_behaviors_rtrn_length(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map_behaviors_rtrn_length)
    }

    /// Returns an iterator over the elements of the
    /// `behaviors_rtrn` field of a `xcb_xkb_get_map_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map_behaviors_rtrn_iterator(
        &self,
        r: *const xcb_xkb_get_map_reply_t,
        s: *const xcb_xkb_get_map_map_t,
    ) -> xcb_xkb_set_behavior_iterator_t {
        sym!(self, xcb_xkb_get_map_map_behaviors_rtrn_iterator)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map_behaviors_rtrn_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map_behaviors_rtrn_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map_behaviors_rtrn_iterator)
    }

    /// Returns a pointer to the `vmods_rtrn` field of a `xcb_xkb_get_map_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map_vmods_rtrn(
        &self,
        s: *const xcb_xkb_get_map_map_t,
    ) -> *mut u8 {
        sym!(self, xcb_xkb_get_map_map_vmods_rtrn)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map_vmods_rtrn` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map_vmods_rtrn(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map_vmods_rtrn)
    }

    /// Returns the number of elements of the `vmods_rtrn` field of a `xcb_xkb_get_map_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map_vmods_rtrn_length(
        &self,
        r: *const xcb_xkb_get_map_reply_t,
        s: *const xcb_xkb_get_map_map_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_map_map_vmods_rtrn_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map_vmods_rtrn_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map_vmods_rtrn_length(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map_vmods_rtrn_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `vmods_rtrn` field of a `xcb_xkb_get_map_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map_vmods_rtrn_end(
        &self,
        r: *const xcb_xkb_get_map_reply_t,
        s: *const xcb_xkb_get_map_map_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_get_map_map_vmods_rtrn_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map_vmods_rtrn_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map_vmods_rtrn_end(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map_vmods_rtrn_end)
    }

    /// Returns a pointer to the `explicit_rtrn` field of a `xcb_xkb_get_map_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map_explicit_rtrn(
        &self,
        s: *const xcb_xkb_get_map_map_t,
    ) -> *mut xcb_xkb_set_explicit_t {
        sym!(self, xcb_xkb_get_map_map_explicit_rtrn)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map_explicit_rtrn` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map_explicit_rtrn(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map_explicit_rtrn)
    }

    /// Returns the number of elements of the `explicit_rtrn` field of a `xcb_xkb_get_map_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map_explicit_rtrn_length(
        &self,
        r: *const xcb_xkb_get_map_reply_t,
        s: *const xcb_xkb_get_map_map_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_map_map_explicit_rtrn_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map_explicit_rtrn_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map_explicit_rtrn_length(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map_explicit_rtrn_length)
    }

    /// Returns an iterator over the elements of the
    /// `explicit_rtrn` field of a `xcb_xkb_get_map_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map_explicit_rtrn_iterator(
        &self,
        r: *const xcb_xkb_get_map_reply_t,
        s: *const xcb_xkb_get_map_map_t,
    ) -> xcb_xkb_set_explicit_iterator_t {
        sym!(self, xcb_xkb_get_map_map_explicit_rtrn_iterator)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map_explicit_rtrn_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map_explicit_rtrn_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map_explicit_rtrn_iterator)
    }

    /// Returns a pointer to the `modmap_rtrn` field of a `xcb_xkb_get_map_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map_modmap_rtrn(
        &self,
        s: *const xcb_xkb_get_map_map_t,
    ) -> *mut xcb_xkb_key_mod_map_t {
        sym!(self, xcb_xkb_get_map_map_modmap_rtrn)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map_modmap_rtrn` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map_modmap_rtrn(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map_modmap_rtrn)
    }

    /// Returns the number of elements of the `modmap_rtrn` field of a `xcb_xkb_get_map_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map_modmap_rtrn_length(
        &self,
        r: *const xcb_xkb_get_map_reply_t,
        s: *const xcb_xkb_get_map_map_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_map_map_modmap_rtrn_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map_modmap_rtrn_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map_modmap_rtrn_length(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map_modmap_rtrn_length)
    }

    /// Returns an iterator over the elements of the
    /// `modmap_rtrn` field of a `xcb_xkb_get_map_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map_modmap_rtrn_iterator(
        &self,
        r: *const xcb_xkb_get_map_reply_t,
        s: *const xcb_xkb_get_map_map_t,
    ) -> xcb_xkb_key_mod_map_iterator_t {
        sym!(self, xcb_xkb_get_map_map_modmap_rtrn_iterator)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map_modmap_rtrn_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map_modmap_rtrn_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map_modmap_rtrn_iterator)
    }

    /// Returns a pointer to the `vmodmap_rtrn` field of a `xcb_xkb_get_map_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map_vmodmap_rtrn(
        &self,
        s: *const xcb_xkb_get_map_map_t,
    ) -> *mut xcb_xkb_key_v_mod_map_t {
        sym!(self, xcb_xkb_get_map_map_vmodmap_rtrn)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map_vmodmap_rtrn` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map_vmodmap_rtrn(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map_vmodmap_rtrn)
    }

    /// Returns the number of elements of the `vmodmap_rtrn` field of a `xcb_xkb_get_map_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map_vmodmap_rtrn_length(
        &self,
        r: *const xcb_xkb_get_map_reply_t,
        s: *const xcb_xkb_get_map_map_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_map_map_vmodmap_rtrn_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map_vmodmap_rtrn_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map_vmodmap_rtrn_length(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map_vmodmap_rtrn_length)
    }

    /// Returns an iterator over the elements of the
    /// `vmodmap_rtrn` field of a `xcb_xkb_get_map_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map_vmodmap_rtrn_iterator(
        &self,
        r: *const xcb_xkb_get_map_reply_t,
        s: *const xcb_xkb_get_map_map_t,
    ) -> xcb_xkb_key_v_mod_map_iterator_t {
        sym!(self, xcb_xkb_get_map_map_vmodmap_rtrn_iterator)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map_vmodmap_rtrn_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map_vmodmap_rtrn_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map_vmodmap_rtrn_iterator)
    }

    /// Serializes a `xcb_xkb_get_map_map_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map_serialize(
        &self,
        _buffer: *mut *mut c_void,
        nTypes: u8,
        nKeySyms: u8,
        nKeyActions: u8,
        totalActions: u16,
        totalKeyBehaviors: u8,
        virtualMods: u16,
        totalKeyExplicit: u8,
        totalModMapKeys: u8,
        totalVModMapKeys: u8,
        present: u16,
        _aux: *const xcb_xkb_get_map_map_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_map_map_serialize)(
            _buffer,
            nTypes,
            nKeySyms,
            nKeyActions,
            totalActions,
            totalKeyBehaviors,
            virtualMods,
            totalKeyExplicit,
            totalModMapKeys,
            totalVModMapKeys,
            present,
            _aux,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map_serialize(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map_serialize)
    }

    /// Unpacks a `xcb_xkb_get_map_map_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map_unpack(
        &self,
        _buffer: *const c_void,
        nTypes: u8,
        nKeySyms: u8,
        nKeyActions: u8,
        totalActions: u16,
        totalKeyBehaviors: u8,
        virtualMods: u16,
        totalKeyExplicit: u8,
        totalModMapKeys: u8,
        totalVModMapKeys: u8,
        present: u16,
        _aux: *mut xcb_xkb_get_map_map_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_map_map_unpack)(
            _buffer,
            nTypes,
            nKeySyms,
            nKeyActions,
            totalActions,
            totalKeyBehaviors,
            virtualMods,
            totalKeyExplicit,
            totalModMapKeys,
            totalVModMapKeys,
            present,
            _aux,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map_unpack(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map_unpack)
    }

    /// Computes the size of a `xcb_xkb_get_map_map_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map_sizeof(
        &self,
        _buffer: *const c_void,
        nTypes: u8,
        nKeySyms: u8,
        nKeyActions: u8,
        totalActions: u16,
        totalKeyBehaviors: u8,
        virtualMods: u16,
        totalKeyExplicit: u8,
        totalModMapKeys: u8,
        totalVModMapKeys: u8,
        present: u16,
    ) -> c_int {
        sym!(self, xcb_xkb_get_map_map_sizeof)(
            _buffer,
            nTypes,
            nKeySyms,
            nKeyActions,
            totalActions,
            totalKeyBehaviors,
            virtualMods,
            totalKeyExplicit,
            totalModMapKeys,
            totalVModMapKeys,
            present,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map_sizeof)
    }

    /// Computes the size of a `xcb_xkb_get_map_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_get_map_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_sizeof)
    }

    /// Sends a `xkb::GetMap` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xkb_get_map_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xkb_get_map_reply`]: Self::xcb_xkb_get_map_reply
    #[inline]
    pub unsafe fn xcb_xkb_get_map(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        full: u16,
        partial: u16,
        first_type: u8,
        n_types: u8,
        first_key_sym: xcb_keycode_t,
        n_key_syms: u8,
        first_key_action: xcb_keycode_t,
        n_key_actions: u8,
        first_key_behavior: xcb_keycode_t,
        n_key_behaviors: u8,
        virtual_mods: u16,
        first_key_explicit: xcb_keycode_t,
        n_key_explicit: u8,
        first_mod_map_key: xcb_keycode_t,
        n_mod_map_keys: u8,
        first_vmod_map_key: xcb_keycode_t,
        n_vmod_map_keys: u8,
    ) -> xcb_xkb_get_map_cookie_t {
        sym!(self, xcb_xkb_get_map)(
            c,
            device_spec,
            full,
            partial,
            first_type,
            n_types,
            first_key_sym,
            n_key_syms,
            first_key_action,
            n_key_actions,
            first_key_behavior,
            n_key_behaviors,
            virtual_mods,
            first_key_explicit,
            n_key_explicit,
            first_mod_map_key,
            n_mod_map_keys,
            first_vmod_map_key,
            n_vmod_map_keys,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map)
    }

    /// Sends a `xkb::GetMap` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xkb_get_map_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xkb_get_map_reply`]: Self::xcb_xkb_get_map_reply
    #[inline]
    pub unsafe fn xcb_xkb_get_map_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        full: u16,
        partial: u16,
        first_type: u8,
        n_types: u8,
        first_key_sym: xcb_keycode_t,
        n_key_syms: u8,
        first_key_action: xcb_keycode_t,
        n_key_actions: u8,
        first_key_behavior: xcb_keycode_t,
        n_key_behaviors: u8,
        virtual_mods: u16,
        first_key_explicit: xcb_keycode_t,
        n_key_explicit: u8,
        first_mod_map_key: xcb_keycode_t,
        n_mod_map_keys: u8,
        first_vmod_map_key: xcb_keycode_t,
        n_vmod_map_keys: u8,
    ) -> xcb_xkb_get_map_cookie_t {
        sym!(self, xcb_xkb_get_map_unchecked)(
            c,
            device_spec,
            full,
            partial,
            first_type,
            n_types,
            first_key_sym,
            n_key_syms,
            first_key_action,
            n_key_actions,
            first_key_behavior,
            n_key_behaviors,
            virtual_mods,
            first_key_explicit,
            n_key_explicit,
            first_mod_map_key,
            n_mod_map_keys,
            first_vmod_map_key,
            n_vmod_map_keys,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_unchecked(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_unchecked)
    }

    /// Returns a pointer to the `map` field of a `xcb_xkb_get_map_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_map(&self, r: *const xcb_xkb_get_map_reply_t) -> *mut c_void {
        sym!(self, xcb_xkb_get_map_map)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map)
    }

    /// Waits for the reply to a `xkb::GetMap` request.
    #[inline]
    pub unsafe fn xcb_xkb_get_map_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_map_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_get_map_reply_t {
        sym!(self, xcb_xkb_get_map_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_reply(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_reply)
    }

    /// Returns the number of elements of the `types` field of a `xcb_xkb_set_map_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values_types_length(
        &self,
        r: *const xcb_xkb_set_map_request_t,
        s: *const xcb_xkb_set_map_values_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_map_values_types_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values_types_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values_types_length(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values_types_length)
    }

    /// Returns an iterator over the elements of the
    /// `types` field of a `xcb_xkb_set_map_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values_types_iterator(
        &self,
        r: *const xcb_xkb_set_map_request_t,
        s: *const xcb_xkb_set_map_values_t,
    ) -> xcb_xkb_set_key_type_iterator_t {
        sym!(self, xcb_xkb_set_map_values_types_iterator)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values_types_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values_types_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values_types_iterator)
    }

    /// Returns the number of elements of the `syms` field of a `xcb_xkb_set_map_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values_syms_length(
        &self,
        r: *const xcb_xkb_set_map_request_t,
        s: *const xcb_xkb_set_map_values_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_map_values_syms_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values_syms_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values_syms_length(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values_syms_length)
    }

    /// Returns an iterator over the elements of the
    /// `syms` field of a `xcb_xkb_set_map_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values_syms_iterator(
        &self,
        r: *const xcb_xkb_set_map_request_t,
        s: *const xcb_xkb_set_map_values_t,
    ) -> xcb_xkb_key_sym_map_iterator_t {
        sym!(self, xcb_xkb_set_map_values_syms_iterator)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values_syms_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values_syms_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values_syms_iterator)
    }

    /// Returns a pointer to the `actionsCount` field of a `xcb_xkb_set_map_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values_actions_count(
        &self,
        s: *const xcb_xkb_set_map_values_t,
    ) -> *mut u8 {
        sym!(self, xcb_xkb_set_map_values_actions_count)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values_actions_count` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values_actions_count(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values_actions_count)
    }

    /// Returns the number of elements of the `actionsCount` field of a `xcb_xkb_set_map_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values_actions_count_length(
        &self,
        r: *const xcb_xkb_set_map_request_t,
        s: *const xcb_xkb_set_map_values_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_map_values_actions_count_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values_actions_count_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values_actions_count_length(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values_actions_count_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `actionsCount` field of a `xcb_xkb_set_map_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values_actions_count_end(
        &self,
        r: *const xcb_xkb_set_map_request_t,
        s: *const xcb_xkb_set_map_values_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_set_map_values_actions_count_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values_actions_count_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values_actions_count_end(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values_actions_count_end)
    }

    /// Returns a pointer to the `actions` field of a `xcb_xkb_set_map_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values_actions(
        &self,
        s: *const xcb_xkb_set_map_values_t,
    ) -> *mut xcb_xkb_action_t {
        sym!(self, xcb_xkb_set_map_values_actions)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values_actions` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values_actions(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values_actions)
    }

    /// Returns the number of elements of the `actions` field of a `xcb_xkb_set_map_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values_actions_length(
        &self,
        r: *const xcb_xkb_set_map_request_t,
        s: *const xcb_xkb_set_map_values_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_map_values_actions_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values_actions_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values_actions_length(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values_actions_length)
    }

    /// Returns an iterator over the elements of the
    /// `actions` field of a `xcb_xkb_set_map_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values_actions_iterator(
        &self,
        r: *const xcb_xkb_set_map_request_t,
        s: *const xcb_xkb_set_map_values_t,
    ) -> xcb_xkb_action_iterator_t {
        sym!(self, xcb_xkb_set_map_values_actions_iterator)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values_actions_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values_actions_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values_actions_iterator)
    }

    /// Returns a pointer to the `behaviors` field of a `xcb_xkb_set_map_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values_behaviors(
        &self,
        s: *const xcb_xkb_set_map_values_t,
    ) -> *mut xcb_xkb_set_behavior_t {
        sym!(self, xcb_xkb_set_map_values_behaviors)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values_behaviors` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values_behaviors(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values_behaviors)
    }

    /// Returns the number of elements of the `behaviors` field of a `xcb_xkb_set_map_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values_behaviors_length(
        &self,
        r: *const xcb_xkb_set_map_request_t,
        s: *const xcb_xkb_set_map_values_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_map_values_behaviors_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values_behaviors_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values_behaviors_length(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values_behaviors_length)
    }

    /// Returns an iterator over the elements of the
    /// `behaviors` field of a `xcb_xkb_set_map_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values_behaviors_iterator(
        &self,
        r: *const xcb_xkb_set_map_request_t,
        s: *const xcb_xkb_set_map_values_t,
    ) -> xcb_xkb_set_behavior_iterator_t {
        sym!(self, xcb_xkb_set_map_values_behaviors_iterator)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values_behaviors_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values_behaviors_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values_behaviors_iterator)
    }

    /// Returns a pointer to the `vmods` field of a `xcb_xkb_set_map_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values_vmods(
        &self,
        s: *const xcb_xkb_set_map_values_t,
    ) -> *mut u8 {
        sym!(self, xcb_xkb_set_map_values_vmods)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values_vmods` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values_vmods(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values_vmods)
    }

    /// Returns the number of elements of the `vmods` field of a `xcb_xkb_set_map_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values_vmods_length(
        &self,
        r: *const xcb_xkb_set_map_request_t,
        s: *const xcb_xkb_set_map_values_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_map_values_vmods_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values_vmods_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values_vmods_length(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values_vmods_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `vmods` field of a `xcb_xkb_set_map_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values_vmods_end(
        &self,
        r: *const xcb_xkb_set_map_request_t,
        s: *const xcb_xkb_set_map_values_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_set_map_values_vmods_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values_vmods_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values_vmods_end(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values_vmods_end)
    }

    /// Returns a pointer to the `explicit` field of a `xcb_xkb_set_map_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values_explicit(
        &self,
        s: *const xcb_xkb_set_map_values_t,
    ) -> *mut xcb_xkb_set_explicit_t {
        sym!(self, xcb_xkb_set_map_values_explicit)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values_explicit` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values_explicit(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values_explicit)
    }

    /// Returns the number of elements of the `explicit` field of a `xcb_xkb_set_map_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values_explicit_length(
        &self,
        r: *const xcb_xkb_set_map_request_t,
        s: *const xcb_xkb_set_map_values_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_map_values_explicit_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values_explicit_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values_explicit_length(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values_explicit_length)
    }

    /// Returns an iterator over the elements of the
    /// `explicit` field of a `xcb_xkb_set_map_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values_explicit_iterator(
        &self,
        r: *const xcb_xkb_set_map_request_t,
        s: *const xcb_xkb_set_map_values_t,
    ) -> xcb_xkb_set_explicit_iterator_t {
        sym!(self, xcb_xkb_set_map_values_explicit_iterator)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values_explicit_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values_explicit_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values_explicit_iterator)
    }

    /// Returns a pointer to the `modmap` field of a `xcb_xkb_set_map_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values_modmap(
        &self,
        s: *const xcb_xkb_set_map_values_t,
    ) -> *mut xcb_xkb_key_mod_map_t {
        sym!(self, xcb_xkb_set_map_values_modmap)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values_modmap` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values_modmap(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values_modmap)
    }

    /// Returns the number of elements of the `modmap` field of a `xcb_xkb_set_map_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values_modmap_length(
        &self,
        r: *const xcb_xkb_set_map_request_t,
        s: *const xcb_xkb_set_map_values_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_map_values_modmap_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values_modmap_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values_modmap_length(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values_modmap_length)
    }

    /// Returns an iterator over the elements of the
    /// `modmap` field of a `xcb_xkb_set_map_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values_modmap_iterator(
        &self,
        r: *const xcb_xkb_set_map_request_t,
        s: *const xcb_xkb_set_map_values_t,
    ) -> xcb_xkb_key_mod_map_iterator_t {
        sym!(self, xcb_xkb_set_map_values_modmap_iterator)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values_modmap_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values_modmap_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values_modmap_iterator)
    }

    /// Returns a pointer to the `vmodmap` field of a `xcb_xkb_set_map_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values_vmodmap(
        &self,
        s: *const xcb_xkb_set_map_values_t,
    ) -> *mut xcb_xkb_key_v_mod_map_t {
        sym!(self, xcb_xkb_set_map_values_vmodmap)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values_vmodmap` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values_vmodmap(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values_vmodmap)
    }

    /// Returns the number of elements of the `vmodmap` field of a `xcb_xkb_set_map_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values_vmodmap_length(
        &self,
        r: *const xcb_xkb_set_map_request_t,
        s: *const xcb_xkb_set_map_values_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_map_values_vmodmap_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values_vmodmap_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values_vmodmap_length(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values_vmodmap_length)
    }

    /// Returns an iterator over the elements of the
    /// `vmodmap` field of a `xcb_xkb_set_map_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values_vmodmap_iterator(
        &self,
        r: *const xcb_xkb_set_map_request_t,
        s: *const xcb_xkb_set_map_values_t,
    ) -> xcb_xkb_key_v_mod_map_iterator_t {
        sym!(self, xcb_xkb_set_map_values_vmodmap_iterator)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values_vmodmap_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values_vmodmap_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values_vmodmap_iterator)
    }

    /// Serializes a `xcb_xkb_set_map_values_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values_serialize(
        &self,
        _buffer: *mut *mut c_void,
        nTypes: u8,
        nKeySyms: u8,
        nKeyActions: u8,
        totalActions: u16,
        totalKeyBehaviors: u8,
        virtualMods: u16,
        totalKeyExplicit: u8,
        totalModMapKeys: u8,
        totalVModMapKeys: u8,
        present: u16,
        _aux: *const xcb_xkb_set_map_values_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_map_values_serialize)(
            _buffer,
            nTypes,
            nKeySyms,
            nKeyActions,
            totalActions,
            totalKeyBehaviors,
            virtualMods,
            totalKeyExplicit,
            totalModMapKeys,
            totalVModMapKeys,
            present,
            _aux,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values_serialize(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values_serialize)
    }

    /// Unpacks a `xcb_xkb_set_map_values_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values_unpack(
        &self,
        _buffer: *const c_void,
        nTypes: u8,
        nKeySyms: u8,
        nKeyActions: u8,
        totalActions: u16,
        totalKeyBehaviors: u8,
        virtualMods: u16,
        totalKeyExplicit: u8,
        totalModMapKeys: u8,
        totalVModMapKeys: u8,
        present: u16,
        _aux: *mut xcb_xkb_set_map_values_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_map_values_unpack)(
            _buffer,
            nTypes,
            nKeySyms,
            nKeyActions,
            totalActions,
            totalKeyBehaviors,
            virtualMods,
            totalKeyExplicit,
            totalModMapKeys,
            totalVModMapKeys,
            present,
            _aux,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values_unpack(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values_unpack)
    }

    /// Computes the size of a `xcb_xkb_set_map_values_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values_sizeof(
        &self,
        _buffer: *const c_void,
        nTypes: u8,
        nKeySyms: u8,
        nKeyActions: u8,
        totalActions: u16,
        totalKeyBehaviors: u8,
        virtualMods: u16,
        totalKeyExplicit: u8,
        totalModMapKeys: u8,
        totalVModMapKeys: u8,
        present: u16,
    ) -> c_int {
        sym!(self, xcb_xkb_set_map_values_sizeof)(
            _buffer,
            nTypes,
            nKeySyms,
            nKeyActions,
            totalActions,
            totalKeyBehaviors,
            virtualMods,
            totalKeyExplicit,
            totalModMapKeys,
            totalVModMapKeys,
            present,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values_sizeof)
    }

    /// Computes the size of a `xcb_xkb_set_map_request_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_set_map_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_sizeof)
    }

    /// Sends a `xkb::SetMap` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    ///
    /// There is an auxiliary version of this function: [`xcb_xkb_set_map_aux_checked`].
    ///
    /// [`xcb_xkb_set_map_aux_checked`]: Self::xcb_xkb_set_map_aux_checked
    #[inline]
    pub unsafe fn xcb_xkb_set_map_checked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        present: u16,
        flags: u16,
        min_key_code: xcb_keycode_t,
        max_key_code: xcb_keycode_t,
        first_type: u8,
        n_types: u8,
        first_key_sym: xcb_keycode_t,
        n_key_syms: u8,
        total_syms: u16,
        first_key_action: xcb_keycode_t,
        n_key_actions: u8,
        total_actions: u16,
        first_key_behavior: xcb_keycode_t,
        n_key_behaviors: u8,
        total_key_behaviors: u8,
        first_key_explicit: xcb_keycode_t,
        n_key_explicit: u8,
        total_key_explicit: u8,
        first_mod_map_key: xcb_keycode_t,
        n_mod_map_keys: u8,
        total_mod_map_keys: u8,
        first_vmod_map_key: xcb_keycode_t,
        n_vmod_map_keys: u8,
        total_vmod_map_keys: u8,
        virtual_mods: u16,
        values: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_set_map_checked)(
            c,
            device_spec,
            present,
            flags,
            min_key_code,
            max_key_code,
            first_type,
            n_types,
            first_key_sym,
            n_key_syms,
            total_syms,
            first_key_action,
            n_key_actions,
            total_actions,
            first_key_behavior,
            n_key_behaviors,
            total_key_behaviors,
            first_key_explicit,
            n_key_explicit,
            total_key_explicit,
            first_mod_map_key,
            n_mod_map_keys,
            total_mod_map_keys,
            first_vmod_map_key,
            n_vmod_map_keys,
            total_vmod_map_keys,
            virtual_mods,
            values,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_checked(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_checked)
    }

    /// Sends a `xkb::SetMap` request (unchecked).
    ///
    /// There is an auxiliary version of this function: [`xcb_xkb_set_map_aux`].
    ///
    /// [`xcb_xkb_set_map_aux`]: Self::xcb_xkb_set_map_aux
    #[inline]
    pub unsafe fn xcb_xkb_set_map(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        present: u16,
        flags: u16,
        min_key_code: xcb_keycode_t,
        max_key_code: xcb_keycode_t,
        first_type: u8,
        n_types: u8,
        first_key_sym: xcb_keycode_t,
        n_key_syms: u8,
        total_syms: u16,
        first_key_action: xcb_keycode_t,
        n_key_actions: u8,
        total_actions: u16,
        first_key_behavior: xcb_keycode_t,
        n_key_behaviors: u8,
        total_key_behaviors: u8,
        first_key_explicit: xcb_keycode_t,
        n_key_explicit: u8,
        total_key_explicit: u8,
        first_mod_map_key: xcb_keycode_t,
        n_mod_map_keys: u8,
        total_mod_map_keys: u8,
        first_vmod_map_key: xcb_keycode_t,
        n_vmod_map_keys: u8,
        total_vmod_map_keys: u8,
        virtual_mods: u16,
        values: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_set_map)(
            c,
            device_spec,
            present,
            flags,
            min_key_code,
            max_key_code,
            first_type,
            n_types,
            first_key_sym,
            n_key_syms,
            total_syms,
            first_key_action,
            n_key_actions,
            total_actions,
            first_key_behavior,
            n_key_behaviors,
            total_key_behaviors,
            first_key_explicit,
            n_key_explicit,
            total_key_explicit,
            first_mod_map_key,
            n_mod_map_keys,
            total_mod_map_keys,
            first_vmod_map_key,
            n_vmod_map_keys,
            total_vmod_map_keys,
            virtual_mods,
            values,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map)
    }

    /// Sends a `xkb::SetMap` request (checked) (aux).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xkb_set_map_aux_checked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        present: u16,
        flags: u16,
        min_key_code: xcb_keycode_t,
        max_key_code: xcb_keycode_t,
        first_type: u8,
        n_types: u8,
        first_key_sym: xcb_keycode_t,
        n_key_syms: u8,
        total_syms: u16,
        first_key_action: xcb_keycode_t,
        n_key_actions: u8,
        total_actions: u16,
        first_key_behavior: xcb_keycode_t,
        n_key_behaviors: u8,
        total_key_behaviors: u8,
        first_key_explicit: xcb_keycode_t,
        n_key_explicit: u8,
        total_key_explicit: u8,
        first_mod_map_key: xcb_keycode_t,
        n_mod_map_keys: u8,
        total_mod_map_keys: u8,
        first_vmod_map_key: xcb_keycode_t,
        n_vmod_map_keys: u8,
        total_vmod_map_keys: u8,
        virtual_mods: u16,
        values: *const xcb_xkb_set_map_values_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_set_map_aux_checked)(
            c,
            device_spec,
            present,
            flags,
            min_key_code,
            max_key_code,
            first_type,
            n_types,
            first_key_sym,
            n_key_syms,
            total_syms,
            first_key_action,
            n_key_actions,
            total_actions,
            first_key_behavior,
            n_key_behaviors,
            total_key_behaviors,
            first_key_explicit,
            n_key_explicit,
            total_key_explicit,
            first_mod_map_key,
            n_mod_map_keys,
            total_mod_map_keys,
            first_vmod_map_key,
            n_vmod_map_keys,
            total_vmod_map_keys,
            virtual_mods,
            values,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_aux_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_aux_checked(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_aux_checked)
    }

    /// Sends a `xkb::SetMap` request (unchecked) (aux).
    #[inline]
    pub unsafe fn xcb_xkb_set_map_aux(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        present: u16,
        flags: u16,
        min_key_code: xcb_keycode_t,
        max_key_code: xcb_keycode_t,
        first_type: u8,
        n_types: u8,
        first_key_sym: xcb_keycode_t,
        n_key_syms: u8,
        total_syms: u16,
        first_key_action: xcb_keycode_t,
        n_key_actions: u8,
        total_actions: u16,
        first_key_behavior: xcb_keycode_t,
        n_key_behaviors: u8,
        total_key_behaviors: u8,
        first_key_explicit: xcb_keycode_t,
        n_key_explicit: u8,
        total_key_explicit: u8,
        first_mod_map_key: xcb_keycode_t,
        n_mod_map_keys: u8,
        total_mod_map_keys: u8,
        first_vmod_map_key: xcb_keycode_t,
        n_vmod_map_keys: u8,
        total_vmod_map_keys: u8,
        virtual_mods: u16,
        values: *const xcb_xkb_set_map_values_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_set_map_aux)(
            c,
            device_spec,
            present,
            flags,
            min_key_code,
            max_key_code,
            first_type,
            n_types,
            first_key_sym,
            n_key_syms,
            total_syms,
            first_key_action,
            n_key_actions,
            total_actions,
            first_key_behavior,
            n_key_behaviors,
            total_key_behaviors,
            first_key_explicit,
            n_key_explicit,
            total_key_explicit,
            first_mod_map_key,
            n_mod_map_keys,
            total_mod_map_keys,
            first_vmod_map_key,
            n_vmod_map_keys,
            total_vmod_map_keys,
            virtual_mods,
            values,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_aux` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_aux(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_aux)
    }

    /// Returns a pointer to the `values` field of a `xcb_xkb_set_map_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_map_values(
        &self,
        r: *const xcb_xkb_set_map_request_t,
    ) -> *mut c_void {
        sym!(self, xcb_xkb_set_map_values)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_values` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_values(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_values)
    }

    /// Computes the size of a `xcb_xkb_get_compat_map_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_get_compat_map_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_get_compat_map_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_compat_map_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_compat_map_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_get_compat_map_sizeof)
    }

    /// Sends a `xkb::GetCompatMap` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xkb_get_compat_map_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xkb_get_compat_map_reply`]: Self::xcb_xkb_get_compat_map_reply
    #[inline]
    pub unsafe fn xcb_xkb_get_compat_map(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        groups: u8,
        get_all_si: u8,
        first_si: u16,
        n_si: u16,
    ) -> xcb_xkb_get_compat_map_cookie_t {
        sym!(self, xcb_xkb_get_compat_map)(c, device_spec, groups, get_all_si, first_si, n_si)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_compat_map` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_compat_map(&self) -> bool {
        has_sym!(self, xcb_xkb_get_compat_map)
    }

    /// Sends a `xkb::GetCompatMap` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xkb_get_compat_map_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xkb_get_compat_map_reply`]: Self::xcb_xkb_get_compat_map_reply
    #[inline]
    pub unsafe fn xcb_xkb_get_compat_map_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        groups: u8,
        get_all_si: u8,
        first_si: u16,
        n_si: u16,
    ) -> xcb_xkb_get_compat_map_cookie_t {
        sym!(self, xcb_xkb_get_compat_map_unchecked)(
            c,
            device_spec,
            groups,
            get_all_si,
            first_si,
            n_si,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_compat_map_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_compat_map_unchecked(&self) -> bool {
        has_sym!(self, xcb_xkb_get_compat_map_unchecked)
    }

    /// Returns a pointer to the `si_rtrn` field of a `xcb_xkb_get_compat_map_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_compat_map_si_rtrn(
        &self,
        r: *const xcb_xkb_get_compat_map_reply_t,
    ) -> *mut xcb_xkb_sym_interpret_t {
        sym!(self, xcb_xkb_get_compat_map_si_rtrn)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_compat_map_si_rtrn` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_compat_map_si_rtrn(&self) -> bool {
        has_sym!(self, xcb_xkb_get_compat_map_si_rtrn)
    }

    /// Returns the number of elements of the `si_rtrn` field of a `xcb_xkb_get_compat_map_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_compat_map_si_rtrn_length(
        &self,
        r: *const xcb_xkb_get_compat_map_reply_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_compat_map_si_rtrn_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_compat_map_si_rtrn_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_compat_map_si_rtrn_length(&self) -> bool {
        has_sym!(self, xcb_xkb_get_compat_map_si_rtrn_length)
    }

    /// Returns an iterator over the elements of the
    /// `si_rtrn` field of a `xcb_xkb_get_compat_map_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_compat_map_si_rtrn_iterator(
        &self,
        r: *const xcb_xkb_get_compat_map_reply_t,
    ) -> xcb_xkb_sym_interpret_iterator_t {
        sym!(self, xcb_xkb_get_compat_map_si_rtrn_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_compat_map_si_rtrn_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_compat_map_si_rtrn_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_get_compat_map_si_rtrn_iterator)
    }

    /// Returns a pointer to the `group_rtrn` field of a `xcb_xkb_get_compat_map_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_compat_map_group_rtrn(
        &self,
        r: *const xcb_xkb_get_compat_map_reply_t,
    ) -> *mut xcb_xkb_mod_def_t {
        sym!(self, xcb_xkb_get_compat_map_group_rtrn)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_compat_map_group_rtrn` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_compat_map_group_rtrn(&self) -> bool {
        has_sym!(self, xcb_xkb_get_compat_map_group_rtrn)
    }

    /// Returns the number of elements of the `group_rtrn` field of a `xcb_xkb_get_compat_map_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_compat_map_group_rtrn_length(
        &self,
        r: *const xcb_xkb_get_compat_map_reply_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_compat_map_group_rtrn_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_compat_map_group_rtrn_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_compat_map_group_rtrn_length(&self) -> bool {
        has_sym!(self, xcb_xkb_get_compat_map_group_rtrn_length)
    }

    /// Returns an iterator over the elements of the
    /// `group_rtrn` field of a `xcb_xkb_get_compat_map_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_compat_map_group_rtrn_iterator(
        &self,
        r: *const xcb_xkb_get_compat_map_reply_t,
    ) -> xcb_xkb_mod_def_iterator_t {
        sym!(self, xcb_xkb_get_compat_map_group_rtrn_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_compat_map_group_rtrn_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_compat_map_group_rtrn_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_get_compat_map_group_rtrn_iterator)
    }

    /// Waits for the reply to a `xkb::GetCompatMap` request.
    #[inline]
    pub unsafe fn xcb_xkb_get_compat_map_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_compat_map_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_get_compat_map_reply_t {
        sym!(self, xcb_xkb_get_compat_map_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_compat_map_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_compat_map_reply(&self) -> bool {
        has_sym!(self, xcb_xkb_get_compat_map_reply)
    }

    /// Computes the size of a `xcb_xkb_set_compat_map_request_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_set_compat_map_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_set_compat_map_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_compat_map_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_compat_map_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_set_compat_map_sizeof)
    }

    /// Sends a `xkb::SetCompatMap` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xkb_set_compat_map_checked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        recompute_actions: u8,
        truncate_si: u8,
        groups: u8,
        first_si: u16,
        n_si: u16,
        si: *const xcb_xkb_sym_interpret_t,
        group_maps: *const xcb_xkb_mod_def_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_set_compat_map_checked)(
            c,
            device_spec,
            recompute_actions,
            truncate_si,
            groups,
            first_si,
            n_si,
            si,
            group_maps,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_compat_map_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_compat_map_checked(&self) -> bool {
        has_sym!(self, xcb_xkb_set_compat_map_checked)
    }

    /// Sends a `xkb::SetCompatMap` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xkb_set_compat_map(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        recompute_actions: u8,
        truncate_si: u8,
        groups: u8,
        first_si: u16,
        n_si: u16,
        si: *const xcb_xkb_sym_interpret_t,
        group_maps: *const xcb_xkb_mod_def_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_set_compat_map)(
            c,
            device_spec,
            recompute_actions,
            truncate_si,
            groups,
            first_si,
            n_si,
            si,
            group_maps,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_compat_map` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_compat_map(&self) -> bool {
        has_sym!(self, xcb_xkb_set_compat_map)
    }

    /// Returns a pointer to the `si` field of a `xcb_xkb_set_compat_map_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_compat_map_si(
        &self,
        r: *const xcb_xkb_set_compat_map_request_t,
    ) -> *mut xcb_xkb_sym_interpret_t {
        sym!(self, xcb_xkb_set_compat_map_si)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_compat_map_si` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_compat_map_si(&self) -> bool {
        has_sym!(self, xcb_xkb_set_compat_map_si)
    }

    /// Returns the number of elements of the `si` field of a `xcb_xkb_set_compat_map_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_compat_map_si_length(
        &self,
        r: *const xcb_xkb_set_compat_map_request_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_compat_map_si_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_compat_map_si_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_compat_map_si_length(&self) -> bool {
        has_sym!(self, xcb_xkb_set_compat_map_si_length)
    }

    /// Returns an iterator over the elements of the
    /// `si` field of a `xcb_xkb_set_compat_map_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_compat_map_si_iterator(
        &self,
        r: *const xcb_xkb_set_compat_map_request_t,
    ) -> xcb_xkb_sym_interpret_iterator_t {
        sym!(self, xcb_xkb_set_compat_map_si_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_compat_map_si_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_compat_map_si_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_set_compat_map_si_iterator)
    }

    /// Returns a pointer to the `groupMaps` field of a `xcb_xkb_set_compat_map_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_compat_map_group_maps(
        &self,
        r: *const xcb_xkb_set_compat_map_request_t,
    ) -> *mut xcb_xkb_mod_def_t {
        sym!(self, xcb_xkb_set_compat_map_group_maps)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_compat_map_group_maps` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_compat_map_group_maps(&self) -> bool {
        has_sym!(self, xcb_xkb_set_compat_map_group_maps)
    }

    /// Returns the number of elements of the `groupMaps` field of a `xcb_xkb_set_compat_map_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_compat_map_group_maps_length(
        &self,
        r: *const xcb_xkb_set_compat_map_request_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_compat_map_group_maps_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_compat_map_group_maps_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_compat_map_group_maps_length(&self) -> bool {
        has_sym!(self, xcb_xkb_set_compat_map_group_maps_length)
    }

    /// Returns an iterator over the elements of the
    /// `groupMaps` field of a `xcb_xkb_set_compat_map_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_compat_map_group_maps_iterator(
        &self,
        r: *const xcb_xkb_set_compat_map_request_t,
    ) -> xcb_xkb_mod_def_iterator_t {
        sym!(self, xcb_xkb_set_compat_map_group_maps_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_compat_map_group_maps_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_compat_map_group_maps_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_set_compat_map_group_maps_iterator)
    }

    /// Sends a `xkb::GetIndicatorState` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xkb_get_indicator_state_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xkb_get_indicator_state_reply`]: Self::xcb_xkb_get_indicator_state_reply
    #[inline]
    pub unsafe fn xcb_xkb_get_indicator_state(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
    ) -> xcb_xkb_get_indicator_state_cookie_t {
        sym!(self, xcb_xkb_get_indicator_state)(c, device_spec)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_indicator_state` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_indicator_state(&self) -> bool {
        has_sym!(self, xcb_xkb_get_indicator_state)
    }

    /// Sends a `xkb::GetIndicatorState` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xkb_get_indicator_state_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xkb_get_indicator_state_reply`]: Self::xcb_xkb_get_indicator_state_reply
    #[inline]
    pub unsafe fn xcb_xkb_get_indicator_state_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
    ) -> xcb_xkb_get_indicator_state_cookie_t {
        sym!(self, xcb_xkb_get_indicator_state_unchecked)(c, device_spec)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_indicator_state_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_indicator_state_unchecked(&self) -> bool {
        has_sym!(self, xcb_xkb_get_indicator_state_unchecked)
    }

    /// Waits for the reply to a `xkb::GetIndicatorState` request.
    #[inline]
    pub unsafe fn xcb_xkb_get_indicator_state_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_indicator_state_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_get_indicator_state_reply_t {
        sym!(self, xcb_xkb_get_indicator_state_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_indicator_state_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_indicator_state_reply(&self) -> bool {
        has_sym!(self, xcb_xkb_get_indicator_state_reply)
    }

    /// Computes the size of a `xcb_xkb_get_indicator_map_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_get_indicator_map_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_get_indicator_map_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_indicator_map_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_indicator_map_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_get_indicator_map_sizeof)
    }

    /// Sends a `xkb::GetIndicatorMap` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xkb_get_indicator_map_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xkb_get_indicator_map_reply`]: Self::xcb_xkb_get_indicator_map_reply
    #[inline]
    pub unsafe fn xcb_xkb_get_indicator_map(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        which: u32,
    ) -> xcb_xkb_get_indicator_map_cookie_t {
        sym!(self, xcb_xkb_get_indicator_map)(c, device_spec, which)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_indicator_map` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_indicator_map(&self) -> bool {
        has_sym!(self, xcb_xkb_get_indicator_map)
    }

    /// Sends a `xkb::GetIndicatorMap` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xkb_get_indicator_map_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xkb_get_indicator_map_reply`]: Self::xcb_xkb_get_indicator_map_reply
    #[inline]
    pub unsafe fn xcb_xkb_get_indicator_map_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        which: u32,
    ) -> xcb_xkb_get_indicator_map_cookie_t {
        sym!(self, xcb_xkb_get_indicator_map_unchecked)(c, device_spec, which)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_indicator_map_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_indicator_map_unchecked(&self) -> bool {
        has_sym!(self, xcb_xkb_get_indicator_map_unchecked)
    }

    /// Returns a pointer to the `maps` field of a `xcb_xkb_get_indicator_map_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_indicator_map_maps(
        &self,
        r: *const xcb_xkb_get_indicator_map_reply_t,
    ) -> *mut xcb_xkb_indicator_map_t {
        sym!(self, xcb_xkb_get_indicator_map_maps)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_indicator_map_maps` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_indicator_map_maps(&self) -> bool {
        has_sym!(self, xcb_xkb_get_indicator_map_maps)
    }

    /// Returns the number of elements of the `maps` field of a `xcb_xkb_get_indicator_map_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_indicator_map_maps_length(
        &self,
        r: *const xcb_xkb_get_indicator_map_reply_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_indicator_map_maps_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_indicator_map_maps_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_indicator_map_maps_length(&self) -> bool {
        has_sym!(self, xcb_xkb_get_indicator_map_maps_length)
    }

    /// Returns an iterator over the elements of the
    /// `maps` field of a `xcb_xkb_get_indicator_map_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_indicator_map_maps_iterator(
        &self,
        r: *const xcb_xkb_get_indicator_map_reply_t,
    ) -> xcb_xkb_indicator_map_iterator_t {
        sym!(self, xcb_xkb_get_indicator_map_maps_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_indicator_map_maps_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_indicator_map_maps_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_get_indicator_map_maps_iterator)
    }

    /// Waits for the reply to a `xkb::GetIndicatorMap` request.
    #[inline]
    pub unsafe fn xcb_xkb_get_indicator_map_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_indicator_map_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_get_indicator_map_reply_t {
        sym!(self, xcb_xkb_get_indicator_map_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_indicator_map_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_indicator_map_reply(&self) -> bool {
        has_sym!(self, xcb_xkb_get_indicator_map_reply)
    }

    /// Computes the size of a `xcb_xkb_set_indicator_map_request_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_set_indicator_map_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_set_indicator_map_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_indicator_map_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_indicator_map_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_set_indicator_map_sizeof)
    }

    /// Sends a `xkb::SetIndicatorMap` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xkb_set_indicator_map_checked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        which: u32,
        maps: *const xcb_xkb_indicator_map_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_set_indicator_map_checked)(c, device_spec, which, maps)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_indicator_map_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_indicator_map_checked(&self) -> bool {
        has_sym!(self, xcb_xkb_set_indicator_map_checked)
    }

    /// Sends a `xkb::SetIndicatorMap` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xkb_set_indicator_map(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        which: u32,
        maps: *const xcb_xkb_indicator_map_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_set_indicator_map)(c, device_spec, which, maps)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_indicator_map` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_indicator_map(&self) -> bool {
        has_sym!(self, xcb_xkb_set_indicator_map)
    }

    /// Returns a pointer to the `maps` field of a `xcb_xkb_set_indicator_map_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_indicator_map_maps(
        &self,
        r: *const xcb_xkb_set_indicator_map_request_t,
    ) -> *mut xcb_xkb_indicator_map_t {
        sym!(self, xcb_xkb_set_indicator_map_maps)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_indicator_map_maps` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_indicator_map_maps(&self) -> bool {
        has_sym!(self, xcb_xkb_set_indicator_map_maps)
    }

    /// Returns the number of elements of the `maps` field of a `xcb_xkb_set_indicator_map_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_indicator_map_maps_length(
        &self,
        r: *const xcb_xkb_set_indicator_map_request_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_indicator_map_maps_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_indicator_map_maps_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_indicator_map_maps_length(&self) -> bool {
        has_sym!(self, xcb_xkb_set_indicator_map_maps_length)
    }

    /// Returns an iterator over the elements of the
    /// `maps` field of a `xcb_xkb_set_indicator_map_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_indicator_map_maps_iterator(
        &self,
        r: *const xcb_xkb_set_indicator_map_request_t,
    ) -> xcb_xkb_indicator_map_iterator_t {
        sym!(self, xcb_xkb_set_indicator_map_maps_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_indicator_map_maps_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_indicator_map_maps_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_set_indicator_map_maps_iterator)
    }

    /// Sends a `xkb::GetNamedIndicator` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xkb_get_named_indicator_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xkb_get_named_indicator_reply`]: Self::xcb_xkb_get_named_indicator_reply
    #[inline]
    pub unsafe fn xcb_xkb_get_named_indicator(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        led_class: xcb_xkb_led_class_spec_t,
        led_id: xcb_xkb_id_spec_t,
        indicator: xcb_atom_t,
    ) -> xcb_xkb_get_named_indicator_cookie_t {
        sym!(self, xcb_xkb_get_named_indicator)(c, device_spec, led_class, led_id, indicator)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_named_indicator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_named_indicator(&self) -> bool {
        has_sym!(self, xcb_xkb_get_named_indicator)
    }

    /// Sends a `xkb::GetNamedIndicator` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xkb_get_named_indicator_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xkb_get_named_indicator_reply`]: Self::xcb_xkb_get_named_indicator_reply
    #[inline]
    pub unsafe fn xcb_xkb_get_named_indicator_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        led_class: xcb_xkb_led_class_spec_t,
        led_id: xcb_xkb_id_spec_t,
        indicator: xcb_atom_t,
    ) -> xcb_xkb_get_named_indicator_cookie_t {
        sym!(self, xcb_xkb_get_named_indicator_unchecked)(
            c,
            device_spec,
            led_class,
            led_id,
            indicator,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_named_indicator_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_named_indicator_unchecked(&self) -> bool {
        has_sym!(self, xcb_xkb_get_named_indicator_unchecked)
    }

    /// Waits for the reply to a `xkb::GetNamedIndicator` request.
    #[inline]
    pub unsafe fn xcb_xkb_get_named_indicator_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_named_indicator_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_get_named_indicator_reply_t {
        sym!(self, xcb_xkb_get_named_indicator_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_named_indicator_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_named_indicator_reply(&self) -> bool {
        has_sym!(self, xcb_xkb_get_named_indicator_reply)
    }

    /// Sends a `xkb::SetNamedIndicator` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xkb_set_named_indicator_checked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        led_class: xcb_xkb_led_class_spec_t,
        led_id: xcb_xkb_id_spec_t,
        indicator: xcb_atom_t,
        set_state: u8,
        on: u8,
        set_map: u8,
        create_map: u8,
        map_flags: u8,
        map_which_groups: u8,
        map_groups: u8,
        map_which_mods: u8,
        map_real_mods: u8,
        map_vmods: u16,
        map_ctrls: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_set_named_indicator_checked)(
            c,
            device_spec,
            led_class,
            led_id,
            indicator,
            set_state,
            on,
            set_map,
            create_map,
            map_flags,
            map_which_groups,
            map_groups,
            map_which_mods,
            map_real_mods,
            map_vmods,
            map_ctrls,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_named_indicator_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_named_indicator_checked(&self) -> bool {
        has_sym!(self, xcb_xkb_set_named_indicator_checked)
    }

    /// Sends a `xkb::SetNamedIndicator` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xkb_set_named_indicator(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        led_class: xcb_xkb_led_class_spec_t,
        led_id: xcb_xkb_id_spec_t,
        indicator: xcb_atom_t,
        set_state: u8,
        on: u8,
        set_map: u8,
        create_map: u8,
        map_flags: u8,
        map_which_groups: u8,
        map_groups: u8,
        map_which_mods: u8,
        map_real_mods: u8,
        map_vmods: u16,
        map_ctrls: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_set_named_indicator)(
            c,
            device_spec,
            led_class,
            led_id,
            indicator,
            set_state,
            on,
            set_map,
            create_map,
            map_flags,
            map_which_groups,
            map_groups,
            map_which_mods,
            map_real_mods,
            map_vmods,
            map_ctrls,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_named_indicator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_named_indicator(&self) -> bool {
        has_sym!(self, xcb_xkb_set_named_indicator)
    }

    /// Returns a pointer to the `typeNames` field of a `xcb_xkb_get_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_type_names(
        &self,
        s: *const xcb_xkb_get_names_value_list_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_xkb_get_names_value_list_type_names)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_type_names` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_type_names(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_type_names)
    }

    /// Returns the number of elements of the `typeNames` field of a `xcb_xkb_get_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_type_names_length(
        &self,
        r: *const xcb_xkb_get_names_reply_t,
        s: *const xcb_xkb_get_names_value_list_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_names_value_list_type_names_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_type_names_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_type_names_length(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_type_names_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `typeNames` field of a `xcb_xkb_get_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_type_names_end(
        &self,
        r: *const xcb_xkb_get_names_reply_t,
        s: *const xcb_xkb_get_names_value_list_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_get_names_value_list_type_names_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_type_names_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_type_names_end(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_type_names_end)
    }

    /// Returns a pointer to the `nLevelsPerType` field of a `xcb_xkb_get_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_n_levels_per_type(
        &self,
        s: *const xcb_xkb_get_names_value_list_t,
    ) -> *mut u8 {
        sym!(self, xcb_xkb_get_names_value_list_n_levels_per_type)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_n_levels_per_type` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_n_levels_per_type(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_n_levels_per_type)
    }

    /// Returns the number of elements of the `nLevelsPerType` field of a `xcb_xkb_get_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_n_levels_per_type_length(
        &self,
        r: *const xcb_xkb_get_names_reply_t,
        s: *const xcb_xkb_get_names_value_list_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_names_value_list_n_levels_per_type_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_n_levels_per_type_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_n_levels_per_type_length(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_n_levels_per_type_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `nLevelsPerType` field of a `xcb_xkb_get_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_n_levels_per_type_end(
        &self,
        r: *const xcb_xkb_get_names_reply_t,
        s: *const xcb_xkb_get_names_value_list_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_get_names_value_list_n_levels_per_type_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_n_levels_per_type_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_n_levels_per_type_end(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_n_levels_per_type_end)
    }

    /// Returns a pointer to the `ktLevelNames` field of a `xcb_xkb_get_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_kt_level_names(
        &self,
        s: *const xcb_xkb_get_names_value_list_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_xkb_get_names_value_list_kt_level_names)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_kt_level_names` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_kt_level_names(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_kt_level_names)
    }

    /// Returns the number of elements of the `ktLevelNames` field of a `xcb_xkb_get_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_kt_level_names_length(
        &self,
        r: *const xcb_xkb_get_names_reply_t,
        s: *const xcb_xkb_get_names_value_list_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_names_value_list_kt_level_names_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_kt_level_names_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_kt_level_names_length(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_kt_level_names_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `ktLevelNames` field of a `xcb_xkb_get_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_kt_level_names_end(
        &self,
        r: *const xcb_xkb_get_names_reply_t,
        s: *const xcb_xkb_get_names_value_list_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_get_names_value_list_kt_level_names_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_kt_level_names_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_kt_level_names_end(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_kt_level_names_end)
    }

    /// Returns a pointer to the `indicatorNames` field of a `xcb_xkb_get_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_indicator_names(
        &self,
        s: *const xcb_xkb_get_names_value_list_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_xkb_get_names_value_list_indicator_names)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_indicator_names` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_indicator_names(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_indicator_names)
    }

    /// Returns the number of elements of the `indicatorNames` field of a `xcb_xkb_get_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_indicator_names_length(
        &self,
        r: *const xcb_xkb_get_names_reply_t,
        s: *const xcb_xkb_get_names_value_list_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_names_value_list_indicator_names_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_indicator_names_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_indicator_names_length(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_indicator_names_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `indicatorNames` field of a `xcb_xkb_get_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_indicator_names_end(
        &self,
        r: *const xcb_xkb_get_names_reply_t,
        s: *const xcb_xkb_get_names_value_list_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_get_names_value_list_indicator_names_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_indicator_names_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_indicator_names_end(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_indicator_names_end)
    }

    /// Returns a pointer to the `virtualModNames` field of a `xcb_xkb_get_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_virtual_mod_names(
        &self,
        s: *const xcb_xkb_get_names_value_list_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_xkb_get_names_value_list_virtual_mod_names)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_virtual_mod_names` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_virtual_mod_names(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_virtual_mod_names)
    }

    /// Returns the number of elements of the `virtualModNames` field of a `xcb_xkb_get_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_virtual_mod_names_length(
        &self,
        r: *const xcb_xkb_get_names_reply_t,
        s: *const xcb_xkb_get_names_value_list_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_names_value_list_virtual_mod_names_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_virtual_mod_names_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_virtual_mod_names_length(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_virtual_mod_names_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `virtualModNames` field of a `xcb_xkb_get_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_virtual_mod_names_end(
        &self,
        r: *const xcb_xkb_get_names_reply_t,
        s: *const xcb_xkb_get_names_value_list_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_get_names_value_list_virtual_mod_names_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_virtual_mod_names_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_virtual_mod_names_end(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_virtual_mod_names_end)
    }

    /// Returns a pointer to the `groups` field of a `xcb_xkb_get_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_groups(
        &self,
        s: *const xcb_xkb_get_names_value_list_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_xkb_get_names_value_list_groups)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_groups` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_groups(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_groups)
    }

    /// Returns the number of elements of the `groups` field of a `xcb_xkb_get_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_groups_length(
        &self,
        r: *const xcb_xkb_get_names_reply_t,
        s: *const xcb_xkb_get_names_value_list_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_names_value_list_groups_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_groups_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_groups_length(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_groups_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `groups` field of a `xcb_xkb_get_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_groups_end(
        &self,
        r: *const xcb_xkb_get_names_reply_t,
        s: *const xcb_xkb_get_names_value_list_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_get_names_value_list_groups_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_groups_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_groups_end(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_groups_end)
    }

    /// Returns a pointer to the `keyNames` field of a `xcb_xkb_get_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_key_names(
        &self,
        s: *const xcb_xkb_get_names_value_list_t,
    ) -> *mut xcb_xkb_key_name_t {
        sym!(self, xcb_xkb_get_names_value_list_key_names)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_key_names` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_key_names(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_key_names)
    }

    /// Returns the number of elements of the `keyNames` field of a `xcb_xkb_get_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_key_names_length(
        &self,
        r: *const xcb_xkb_get_names_reply_t,
        s: *const xcb_xkb_get_names_value_list_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_names_value_list_key_names_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_key_names_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_key_names_length(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_key_names_length)
    }

    /// Returns an iterator over the elements of the
    /// `keyNames` field of a `xcb_xkb_get_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_key_names_iterator(
        &self,
        r: *const xcb_xkb_get_names_reply_t,
        s: *const xcb_xkb_get_names_value_list_t,
    ) -> xcb_xkb_key_name_iterator_t {
        sym!(self, xcb_xkb_get_names_value_list_key_names_iterator)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_key_names_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_key_names_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_key_names_iterator)
    }

    /// Returns a pointer to the `keyAliases` field of a `xcb_xkb_get_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_key_aliases(
        &self,
        s: *const xcb_xkb_get_names_value_list_t,
    ) -> *mut xcb_xkb_key_alias_t {
        sym!(self, xcb_xkb_get_names_value_list_key_aliases)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_key_aliases` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_key_aliases(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_key_aliases)
    }

    /// Returns the number of elements of the `keyAliases` field of a `xcb_xkb_get_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_key_aliases_length(
        &self,
        r: *const xcb_xkb_get_names_reply_t,
        s: *const xcb_xkb_get_names_value_list_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_names_value_list_key_aliases_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_key_aliases_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_key_aliases_length(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_key_aliases_length)
    }

    /// Returns an iterator over the elements of the
    /// `keyAliases` field of a `xcb_xkb_get_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_key_aliases_iterator(
        &self,
        r: *const xcb_xkb_get_names_reply_t,
        s: *const xcb_xkb_get_names_value_list_t,
    ) -> xcb_xkb_key_alias_iterator_t {
        sym!(self, xcb_xkb_get_names_value_list_key_aliases_iterator)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_key_aliases_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_key_aliases_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_key_aliases_iterator)
    }

    /// Returns a pointer to the `radioGroupNames` field of a `xcb_xkb_get_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_radio_group_names(
        &self,
        s: *const xcb_xkb_get_names_value_list_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_xkb_get_names_value_list_radio_group_names)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_radio_group_names` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_radio_group_names(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_radio_group_names)
    }

    /// Returns the number of elements of the `radioGroupNames` field of a `xcb_xkb_get_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_radio_group_names_length(
        &self,
        r: *const xcb_xkb_get_names_reply_t,
        s: *const xcb_xkb_get_names_value_list_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_names_value_list_radio_group_names_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_radio_group_names_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_radio_group_names_length(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_radio_group_names_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `radioGroupNames` field of a `xcb_xkb_get_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_radio_group_names_end(
        &self,
        r: *const xcb_xkb_get_names_reply_t,
        s: *const xcb_xkb_get_names_value_list_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_get_names_value_list_radio_group_names_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_radio_group_names_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_radio_group_names_end(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_radio_group_names_end)
    }

    /// Serializes a `xcb_xkb_get_names_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_serialize(
        &self,
        _buffer: *mut *mut c_void,
        nTypes: u8,
        indicators: u32,
        virtualMods: u16,
        groupNames: u8,
        nKeys: u8,
        nKeyAliases: u8,
        nRadioGroups: u8,
        which: u32,
        _aux: *const xcb_xkb_get_names_value_list_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_names_value_list_serialize)(
            _buffer,
            nTypes,
            indicators,
            virtualMods,
            groupNames,
            nKeys,
            nKeyAliases,
            nRadioGroups,
            which,
            _aux,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_serialize(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_serialize)
    }

    /// Unpacks a `xcb_xkb_get_names_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_unpack(
        &self,
        _buffer: *const c_void,
        nTypes: u8,
        indicators: u32,
        virtualMods: u16,
        groupNames: u8,
        nKeys: u8,
        nKeyAliases: u8,
        nRadioGroups: u8,
        which: u32,
        _aux: *mut xcb_xkb_get_names_value_list_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_names_value_list_unpack)(
            _buffer,
            nTypes,
            indicators,
            virtualMods,
            groupNames,
            nKeys,
            nKeyAliases,
            nRadioGroups,
            which,
            _aux,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_unpack(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_unpack)
    }

    /// Computes the size of a `xcb_xkb_get_names_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list_sizeof(
        &self,
        _buffer: *const c_void,
        nTypes: u8,
        indicators: u32,
        virtualMods: u16,
        groupNames: u8,
        nKeys: u8,
        nKeyAliases: u8,
        nRadioGroups: u8,
        which: u32,
    ) -> c_int {
        sym!(self, xcb_xkb_get_names_value_list_sizeof)(
            _buffer,
            nTypes,
            indicators,
            virtualMods,
            groupNames,
            nKeys,
            nKeyAliases,
            nRadioGroups,
            which,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list_sizeof)
    }

    /// Computes the size of a `xcb_xkb_get_names_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_get_names_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_sizeof)
    }

    /// Sends a `xkb::GetNames` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xkb_get_names_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xkb_get_names_reply`]: Self::xcb_xkb_get_names_reply
    #[inline]
    pub unsafe fn xcb_xkb_get_names(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        which: u32,
    ) -> xcb_xkb_get_names_cookie_t {
        sym!(self, xcb_xkb_get_names)(c, device_spec, which)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names)
    }

    /// Sends a `xkb::GetNames` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xkb_get_names_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xkb_get_names_reply`]: Self::xcb_xkb_get_names_reply
    #[inline]
    pub unsafe fn xcb_xkb_get_names_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        which: u32,
    ) -> xcb_xkb_get_names_cookie_t {
        sym!(self, xcb_xkb_get_names_unchecked)(c, device_spec, which)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_unchecked(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_unchecked)
    }

    /// Returns a pointer to the `valueList` field of a `xcb_xkb_get_names_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_value_list(
        &self,
        r: *const xcb_xkb_get_names_reply_t,
    ) -> *mut c_void {
        sym!(self, xcb_xkb_get_names_value_list)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_value_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_value_list(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_value_list)
    }

    /// Waits for the reply to a `xkb::GetNames` request.
    #[inline]
    pub unsafe fn xcb_xkb_get_names_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_names_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_get_names_reply_t {
        sym!(self, xcb_xkb_get_names_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_reply(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_reply)
    }

    /// Returns a pointer to the `typeNames` field of a `xcb_xkb_set_names_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_type_names(
        &self,
        s: *const xcb_xkb_set_names_values_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_xkb_set_names_values_type_names)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_type_names` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_type_names(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_type_names)
    }

    /// Returns the number of elements of the `typeNames` field of a `xcb_xkb_set_names_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_type_names_length(
        &self,
        r: *const xcb_xkb_set_names_request_t,
        s: *const xcb_xkb_set_names_values_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_names_values_type_names_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_type_names_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_type_names_length(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_type_names_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `typeNames` field of a `xcb_xkb_set_names_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_type_names_end(
        &self,
        r: *const xcb_xkb_set_names_request_t,
        s: *const xcb_xkb_set_names_values_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_set_names_values_type_names_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_type_names_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_type_names_end(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_type_names_end)
    }

    /// Returns a pointer to the `nLevelsPerType` field of a `xcb_xkb_set_names_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_n_levels_per_type(
        &self,
        s: *const xcb_xkb_set_names_values_t,
    ) -> *mut u8 {
        sym!(self, xcb_xkb_set_names_values_n_levels_per_type)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_n_levels_per_type` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_n_levels_per_type(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_n_levels_per_type)
    }

    /// Returns the number of elements of the `nLevelsPerType` field of a `xcb_xkb_set_names_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_n_levels_per_type_length(
        &self,
        r: *const xcb_xkb_set_names_request_t,
        s: *const xcb_xkb_set_names_values_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_names_values_n_levels_per_type_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_n_levels_per_type_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_n_levels_per_type_length(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_n_levels_per_type_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `nLevelsPerType` field of a `xcb_xkb_set_names_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_n_levels_per_type_end(
        &self,
        r: *const xcb_xkb_set_names_request_t,
        s: *const xcb_xkb_set_names_values_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_set_names_values_n_levels_per_type_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_n_levels_per_type_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_n_levels_per_type_end(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_n_levels_per_type_end)
    }

    /// Returns a pointer to the `ktLevelNames` field of a `xcb_xkb_set_names_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_kt_level_names(
        &self,
        s: *const xcb_xkb_set_names_values_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_xkb_set_names_values_kt_level_names)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_kt_level_names` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_kt_level_names(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_kt_level_names)
    }

    /// Returns the number of elements of the `ktLevelNames` field of a `xcb_xkb_set_names_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_kt_level_names_length(
        &self,
        r: *const xcb_xkb_set_names_request_t,
        s: *const xcb_xkb_set_names_values_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_names_values_kt_level_names_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_kt_level_names_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_kt_level_names_length(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_kt_level_names_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `ktLevelNames` field of a `xcb_xkb_set_names_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_kt_level_names_end(
        &self,
        r: *const xcb_xkb_set_names_request_t,
        s: *const xcb_xkb_set_names_values_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_set_names_values_kt_level_names_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_kt_level_names_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_kt_level_names_end(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_kt_level_names_end)
    }

    /// Returns a pointer to the `indicatorNames` field of a `xcb_xkb_set_names_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_indicator_names(
        &self,
        s: *const xcb_xkb_set_names_values_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_xkb_set_names_values_indicator_names)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_indicator_names` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_indicator_names(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_indicator_names)
    }

    /// Returns the number of elements of the `indicatorNames` field of a `xcb_xkb_set_names_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_indicator_names_length(
        &self,
        r: *const xcb_xkb_set_names_request_t,
        s: *const xcb_xkb_set_names_values_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_names_values_indicator_names_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_indicator_names_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_indicator_names_length(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_indicator_names_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `indicatorNames` field of a `xcb_xkb_set_names_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_indicator_names_end(
        &self,
        r: *const xcb_xkb_set_names_request_t,
        s: *const xcb_xkb_set_names_values_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_set_names_values_indicator_names_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_indicator_names_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_indicator_names_end(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_indicator_names_end)
    }

    /// Returns a pointer to the `virtualModNames` field of a `xcb_xkb_set_names_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_virtual_mod_names(
        &self,
        s: *const xcb_xkb_set_names_values_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_xkb_set_names_values_virtual_mod_names)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_virtual_mod_names` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_virtual_mod_names(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_virtual_mod_names)
    }

    /// Returns the number of elements of the `virtualModNames` field of a `xcb_xkb_set_names_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_virtual_mod_names_length(
        &self,
        r: *const xcb_xkb_set_names_request_t,
        s: *const xcb_xkb_set_names_values_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_names_values_virtual_mod_names_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_virtual_mod_names_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_virtual_mod_names_length(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_virtual_mod_names_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `virtualModNames` field of a `xcb_xkb_set_names_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_virtual_mod_names_end(
        &self,
        r: *const xcb_xkb_set_names_request_t,
        s: *const xcb_xkb_set_names_values_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_set_names_values_virtual_mod_names_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_virtual_mod_names_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_virtual_mod_names_end(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_virtual_mod_names_end)
    }

    /// Returns a pointer to the `groups` field of a `xcb_xkb_set_names_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_groups(
        &self,
        s: *const xcb_xkb_set_names_values_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_xkb_set_names_values_groups)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_groups` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_groups(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_groups)
    }

    /// Returns the number of elements of the `groups` field of a `xcb_xkb_set_names_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_groups_length(
        &self,
        r: *const xcb_xkb_set_names_request_t,
        s: *const xcb_xkb_set_names_values_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_names_values_groups_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_groups_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_groups_length(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_groups_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `groups` field of a `xcb_xkb_set_names_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_groups_end(
        &self,
        r: *const xcb_xkb_set_names_request_t,
        s: *const xcb_xkb_set_names_values_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_set_names_values_groups_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_groups_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_groups_end(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_groups_end)
    }

    /// Returns a pointer to the `keyNames` field of a `xcb_xkb_set_names_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_key_names(
        &self,
        s: *const xcb_xkb_set_names_values_t,
    ) -> *mut xcb_xkb_key_name_t {
        sym!(self, xcb_xkb_set_names_values_key_names)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_key_names` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_key_names(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_key_names)
    }

    /// Returns the number of elements of the `keyNames` field of a `xcb_xkb_set_names_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_key_names_length(
        &self,
        r: *const xcb_xkb_set_names_request_t,
        s: *const xcb_xkb_set_names_values_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_names_values_key_names_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_key_names_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_key_names_length(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_key_names_length)
    }

    /// Returns an iterator over the elements of the
    /// `keyNames` field of a `xcb_xkb_set_names_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_key_names_iterator(
        &self,
        r: *const xcb_xkb_set_names_request_t,
        s: *const xcb_xkb_set_names_values_t,
    ) -> xcb_xkb_key_name_iterator_t {
        sym!(self, xcb_xkb_set_names_values_key_names_iterator)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_key_names_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_key_names_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_key_names_iterator)
    }

    /// Returns a pointer to the `keyAliases` field of a `xcb_xkb_set_names_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_key_aliases(
        &self,
        s: *const xcb_xkb_set_names_values_t,
    ) -> *mut xcb_xkb_key_alias_t {
        sym!(self, xcb_xkb_set_names_values_key_aliases)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_key_aliases` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_key_aliases(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_key_aliases)
    }

    /// Returns the number of elements of the `keyAliases` field of a `xcb_xkb_set_names_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_key_aliases_length(
        &self,
        r: *const xcb_xkb_set_names_request_t,
        s: *const xcb_xkb_set_names_values_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_names_values_key_aliases_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_key_aliases_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_key_aliases_length(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_key_aliases_length)
    }

    /// Returns an iterator over the elements of the
    /// `keyAliases` field of a `xcb_xkb_set_names_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_key_aliases_iterator(
        &self,
        r: *const xcb_xkb_set_names_request_t,
        s: *const xcb_xkb_set_names_values_t,
    ) -> xcb_xkb_key_alias_iterator_t {
        sym!(self, xcb_xkb_set_names_values_key_aliases_iterator)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_key_aliases_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_key_aliases_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_key_aliases_iterator)
    }

    /// Returns a pointer to the `radioGroupNames` field of a `xcb_xkb_set_names_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_radio_group_names(
        &self,
        s: *const xcb_xkb_set_names_values_t,
    ) -> *mut xcb_atom_t {
        sym!(self, xcb_xkb_set_names_values_radio_group_names)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_radio_group_names` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_radio_group_names(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_radio_group_names)
    }

    /// Returns the number of elements of the `radioGroupNames` field of a `xcb_xkb_set_names_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_radio_group_names_length(
        &self,
        r: *const xcb_xkb_set_names_request_t,
        s: *const xcb_xkb_set_names_values_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_names_values_radio_group_names_length)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_radio_group_names_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_radio_group_names_length(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_radio_group_names_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `radioGroupNames` field of a `xcb_xkb_set_names_values_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_radio_group_names_end(
        &self,
        r: *const xcb_xkb_set_names_request_t,
        s: *const xcb_xkb_set_names_values_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_set_names_values_radio_group_names_end)(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_radio_group_names_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_radio_group_names_end(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_radio_group_names_end)
    }

    /// Serializes a `xcb_xkb_set_names_values_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_serialize(
        &self,
        _buffer: *mut *mut c_void,
        nTypes: u8,
        indicators: u32,
        virtualMods: u16,
        groupNames: u8,
        nKeys: u8,
        nKeyAliases: u8,
        nRadioGroups: u8,
        which: u32,
        _aux: *const xcb_xkb_set_names_values_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_names_values_serialize)(
            _buffer,
            nTypes,
            indicators,
            virtualMods,
            groupNames,
            nKeys,
            nKeyAliases,
            nRadioGroups,
            which,
            _aux,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_serialize(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_serialize)
    }

    /// Unpacks a `xcb_xkb_set_names_values_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_unpack(
        &self,
        _buffer: *const c_void,
        nTypes: u8,
        indicators: u32,
        virtualMods: u16,
        groupNames: u8,
        nKeys: u8,
        nKeyAliases: u8,
        nRadioGroups: u8,
        which: u32,
        _aux: *mut xcb_xkb_set_names_values_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_names_values_unpack)(
            _buffer,
            nTypes,
            indicators,
            virtualMods,
            groupNames,
            nKeys,
            nKeyAliases,
            nRadioGroups,
            which,
            _aux,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_unpack(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_unpack)
    }

    /// Computes the size of a `xcb_xkb_set_names_values_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values_sizeof(
        &self,
        _buffer: *const c_void,
        nTypes: u8,
        indicators: u32,
        virtualMods: u16,
        groupNames: u8,
        nKeys: u8,
        nKeyAliases: u8,
        nRadioGroups: u8,
        which: u32,
    ) -> c_int {
        sym!(self, xcb_xkb_set_names_values_sizeof)(
            _buffer,
            nTypes,
            indicators,
            virtualMods,
            groupNames,
            nKeys,
            nKeyAliases,
            nRadioGroups,
            which,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values_sizeof)
    }

    /// Computes the size of a `xcb_xkb_set_names_request_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_set_names_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_sizeof)
    }

    /// Sends a `xkb::SetNames` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    ///
    /// There is an auxiliary version of this function: [`xcb_xkb_set_names_aux_checked`].
    ///
    /// [`xcb_xkb_set_names_aux_checked`]: Self::xcb_xkb_set_names_aux_checked
    #[inline]
    pub unsafe fn xcb_xkb_set_names_checked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        virtual_mods: u16,
        which: u32,
        first_type: u8,
        n_types: u8,
        first_ktlevelt: u8,
        n_ktlevels: u8,
        indicators: u32,
        group_names: u8,
        n_radio_groups: u8,
        first_key: xcb_keycode_t,
        n_keys: u8,
        n_key_aliases: u8,
        total_ktlevel_names: u16,
        values: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_set_names_checked)(
            c,
            device_spec,
            virtual_mods,
            which,
            first_type,
            n_types,
            first_ktlevelt,
            n_ktlevels,
            indicators,
            group_names,
            n_radio_groups,
            first_key,
            n_keys,
            n_key_aliases,
            total_ktlevel_names,
            values,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_checked(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_checked)
    }

    /// Sends a `xkb::SetNames` request (unchecked).
    ///
    /// There is an auxiliary version of this function: [`xcb_xkb_set_names_aux`].
    ///
    /// [`xcb_xkb_set_names_aux`]: Self::xcb_xkb_set_names_aux
    #[inline]
    pub unsafe fn xcb_xkb_set_names(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        virtual_mods: u16,
        which: u32,
        first_type: u8,
        n_types: u8,
        first_ktlevelt: u8,
        n_ktlevels: u8,
        indicators: u32,
        group_names: u8,
        n_radio_groups: u8,
        first_key: xcb_keycode_t,
        n_keys: u8,
        n_key_aliases: u8,
        total_ktlevel_names: u16,
        values: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_set_names)(
            c,
            device_spec,
            virtual_mods,
            which,
            first_type,
            n_types,
            first_ktlevelt,
            n_ktlevels,
            indicators,
            group_names,
            n_radio_groups,
            first_key,
            n_keys,
            n_key_aliases,
            total_ktlevel_names,
            values,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names)
    }

    /// Sends a `xkb::SetNames` request (checked) (aux).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xkb_set_names_aux_checked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        virtual_mods: u16,
        which: u32,
        first_type: u8,
        n_types: u8,
        first_ktlevelt: u8,
        n_ktlevels: u8,
        indicators: u32,
        group_names: u8,
        n_radio_groups: u8,
        first_key: xcb_keycode_t,
        n_keys: u8,
        n_key_aliases: u8,
        total_ktlevel_names: u16,
        values: *const xcb_xkb_set_names_values_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_set_names_aux_checked)(
            c,
            device_spec,
            virtual_mods,
            which,
            first_type,
            n_types,
            first_ktlevelt,
            n_ktlevels,
            indicators,
            group_names,
            n_radio_groups,
            first_key,
            n_keys,
            n_key_aliases,
            total_ktlevel_names,
            values,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_aux_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_aux_checked(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_aux_checked)
    }

    /// Sends a `xkb::SetNames` request (unchecked) (aux).
    #[inline]
    pub unsafe fn xcb_xkb_set_names_aux(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        virtual_mods: u16,
        which: u32,
        first_type: u8,
        n_types: u8,
        first_ktlevelt: u8,
        n_ktlevels: u8,
        indicators: u32,
        group_names: u8,
        n_radio_groups: u8,
        first_key: xcb_keycode_t,
        n_keys: u8,
        n_key_aliases: u8,
        total_ktlevel_names: u16,
        values: *const xcb_xkb_set_names_values_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_set_names_aux)(
            c,
            device_spec,
            virtual_mods,
            which,
            first_type,
            n_types,
            first_ktlevelt,
            n_ktlevels,
            indicators,
            group_names,
            n_radio_groups,
            first_key,
            n_keys,
            n_key_aliases,
            total_ktlevel_names,
            values,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_aux` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_aux(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_aux)
    }

    /// Returns a pointer to the `values` field of a `xcb_xkb_set_names_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_names_values(
        &self,
        r: *const xcb_xkb_set_names_request_t,
    ) -> *mut c_void {
        sym!(self, xcb_xkb_set_names_values)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_values` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_values(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_values)
    }

    /// Sends a `xkb::PerClientFlags` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xkb_per_client_flags_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xkb_per_client_flags_reply`]: Self::xcb_xkb_per_client_flags_reply
    #[inline]
    pub unsafe fn xcb_xkb_per_client_flags(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        change: u32,
        value: u32,
        ctrls_to_change: u32,
        auto_ctrls: u32,
        auto_ctrls_values: u32,
    ) -> xcb_xkb_per_client_flags_cookie_t {
        sym!(self, xcb_xkb_per_client_flags)(
            c,
            device_spec,
            change,
            value,
            ctrls_to_change,
            auto_ctrls,
            auto_ctrls_values,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_per_client_flags` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_per_client_flags(&self) -> bool {
        has_sym!(self, xcb_xkb_per_client_flags)
    }

    /// Sends a `xkb::PerClientFlags` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xkb_per_client_flags_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xkb_per_client_flags_reply`]: Self::xcb_xkb_per_client_flags_reply
    #[inline]
    pub unsafe fn xcb_xkb_per_client_flags_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        change: u32,
        value: u32,
        ctrls_to_change: u32,
        auto_ctrls: u32,
        auto_ctrls_values: u32,
    ) -> xcb_xkb_per_client_flags_cookie_t {
        sym!(self, xcb_xkb_per_client_flags_unchecked)(
            c,
            device_spec,
            change,
            value,
            ctrls_to_change,
            auto_ctrls,
            auto_ctrls_values,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_per_client_flags_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_per_client_flags_unchecked(&self) -> bool {
        has_sym!(self, xcb_xkb_per_client_flags_unchecked)
    }

    /// Waits for the reply to a `xkb::PerClientFlags` request.
    #[inline]
    pub unsafe fn xcb_xkb_per_client_flags_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_per_client_flags_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_per_client_flags_reply_t {
        sym!(self, xcb_xkb_per_client_flags_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xkb_per_client_flags_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_per_client_flags_reply(&self) -> bool {
        has_sym!(self, xcb_xkb_per_client_flags_reply)
    }

    /// Computes the size of a `xcb_xkb_list_components_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_list_components_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_list_components_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_list_components_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_list_components_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_list_components_sizeof)
    }

    /// Sends a `xkb::ListComponents` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xkb_list_components_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xkb_list_components_reply`]: Self::xcb_xkb_list_components_reply
    #[inline]
    pub unsafe fn xcb_xkb_list_components(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        max_names: u16,
    ) -> xcb_xkb_list_components_cookie_t {
        sym!(self, xcb_xkb_list_components)(c, device_spec, max_names)
    }

    /// Returns `true` iff the symbol `xcb_xkb_list_components` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_list_components(&self) -> bool {
        has_sym!(self, xcb_xkb_list_components)
    }

    /// Sends a `xkb::ListComponents` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xkb_list_components_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xkb_list_components_reply`]: Self::xcb_xkb_list_components_reply
    #[inline]
    pub unsafe fn xcb_xkb_list_components_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        max_names: u16,
    ) -> xcb_xkb_list_components_cookie_t {
        sym!(self, xcb_xkb_list_components_unchecked)(c, device_spec, max_names)
    }

    /// Returns `true` iff the symbol `xcb_xkb_list_components_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_list_components_unchecked(&self) -> bool {
        has_sym!(self, xcb_xkb_list_components_unchecked)
    }

    /// Returns the number of elements of the `keymaps` field of a `xcb_xkb_list_components_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_list_components_keymaps_length(
        &self,
        r: *const xcb_xkb_list_components_reply_t,
    ) -> c_int {
        sym!(self, xcb_xkb_list_components_keymaps_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_list_components_keymaps_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_list_components_keymaps_length(&self) -> bool {
        has_sym!(self, xcb_xkb_list_components_keymaps_length)
    }

    /// Returns an iterator over the elements of the
    /// `keymaps` field of a `xcb_xkb_list_components_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_list_components_keymaps_iterator(
        &self,
        r: *const xcb_xkb_list_components_reply_t,
    ) -> xcb_xkb_listing_iterator_t {
        sym!(self, xcb_xkb_list_components_keymaps_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_list_components_keymaps_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_list_components_keymaps_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_list_components_keymaps_iterator)
    }

    /// Returns the number of elements of the `keycodes` field of a `xcb_xkb_list_components_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_list_components_keycodes_length(
        &self,
        r: *const xcb_xkb_list_components_reply_t,
    ) -> c_int {
        sym!(self, xcb_xkb_list_components_keycodes_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_list_components_keycodes_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_list_components_keycodes_length(&self) -> bool {
        has_sym!(self, xcb_xkb_list_components_keycodes_length)
    }

    /// Returns an iterator over the elements of the
    /// `keycodes` field of a `xcb_xkb_list_components_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_list_components_keycodes_iterator(
        &self,
        r: *const xcb_xkb_list_components_reply_t,
    ) -> xcb_xkb_listing_iterator_t {
        sym!(self, xcb_xkb_list_components_keycodes_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_list_components_keycodes_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_list_components_keycodes_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_list_components_keycodes_iterator)
    }

    /// Returns the number of elements of the `types` field of a `xcb_xkb_list_components_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_list_components_types_length(
        &self,
        r: *const xcb_xkb_list_components_reply_t,
    ) -> c_int {
        sym!(self, xcb_xkb_list_components_types_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_list_components_types_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_list_components_types_length(&self) -> bool {
        has_sym!(self, xcb_xkb_list_components_types_length)
    }

    /// Returns an iterator over the elements of the
    /// `types` field of a `xcb_xkb_list_components_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_list_components_types_iterator(
        &self,
        r: *const xcb_xkb_list_components_reply_t,
    ) -> xcb_xkb_listing_iterator_t {
        sym!(self, xcb_xkb_list_components_types_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_list_components_types_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_list_components_types_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_list_components_types_iterator)
    }

    /// Returns the number of elements of the `compatMaps` field of a `xcb_xkb_list_components_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_list_components_compat_maps_length(
        &self,
        r: *const xcb_xkb_list_components_reply_t,
    ) -> c_int {
        sym!(self, xcb_xkb_list_components_compat_maps_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_list_components_compat_maps_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_list_components_compat_maps_length(&self) -> bool {
        has_sym!(self, xcb_xkb_list_components_compat_maps_length)
    }

    /// Returns an iterator over the elements of the
    /// `compatMaps` field of a `xcb_xkb_list_components_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_list_components_compat_maps_iterator(
        &self,
        r: *const xcb_xkb_list_components_reply_t,
    ) -> xcb_xkb_listing_iterator_t {
        sym!(self, xcb_xkb_list_components_compat_maps_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_list_components_compat_maps_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_list_components_compat_maps_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_list_components_compat_maps_iterator)
    }

    /// Returns the number of elements of the `symbols` field of a `xcb_xkb_list_components_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_list_components_symbols_length(
        &self,
        r: *const xcb_xkb_list_components_reply_t,
    ) -> c_int {
        sym!(self, xcb_xkb_list_components_symbols_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_list_components_symbols_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_list_components_symbols_length(&self) -> bool {
        has_sym!(self, xcb_xkb_list_components_symbols_length)
    }

    /// Returns an iterator over the elements of the
    /// `symbols` field of a `xcb_xkb_list_components_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_list_components_symbols_iterator(
        &self,
        r: *const xcb_xkb_list_components_reply_t,
    ) -> xcb_xkb_listing_iterator_t {
        sym!(self, xcb_xkb_list_components_symbols_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_list_components_symbols_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_list_components_symbols_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_list_components_symbols_iterator)
    }

    /// Returns the number of elements of the `geometries` field of a `xcb_xkb_list_components_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_list_components_geometries_length(
        &self,
        r: *const xcb_xkb_list_components_reply_t,
    ) -> c_int {
        sym!(self, xcb_xkb_list_components_geometries_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_list_components_geometries_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_list_components_geometries_length(&self) -> bool {
        has_sym!(self, xcb_xkb_list_components_geometries_length)
    }

    /// Returns an iterator over the elements of the
    /// `geometries` field of a `xcb_xkb_list_components_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_list_components_geometries_iterator(
        &self,
        r: *const xcb_xkb_list_components_reply_t,
    ) -> xcb_xkb_listing_iterator_t {
        sym!(self, xcb_xkb_list_components_geometries_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_list_components_geometries_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_list_components_geometries_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_list_components_geometries_iterator)
    }

    /// Waits for the reply to a `xkb::ListComponents` request.
    #[inline]
    pub unsafe fn xcb_xkb_list_components_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_list_components_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_list_components_reply_t {
        sym!(self, xcb_xkb_list_components_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xkb_list_components_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_list_components_reply(&self) -> bool {
        has_sym!(self, xcb_xkb_list_components_reply)
    }

    /// Returns the number of elements of the `types_rtrn` field of a `xcb_xkb_get_kbd_by_name_replies_types_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_types_rtrn_length(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_types_rtrn_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map_types_rtrn_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map_types_rtrn_length(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_types_rtrn_length
        )
    }

    /// Returns an iterator over the elements of the
    /// `types_rtrn` field of a `xcb_xkb_get_kbd_by_name_replies_types_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_types_rtrn_iterator(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_xkb_key_type_iterator_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_types_rtrn_iterator
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map_types_rtrn_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map_types_rtrn_iterator(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_types_rtrn_iterator
        )
    }

    /// Returns the number of elements of the `syms_rtrn` field of a `xcb_xkb_get_kbd_by_name_replies_types_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_syms_rtrn_length(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_syms_rtrn_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map_syms_rtrn_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map_syms_rtrn_length(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_syms_rtrn_length
        )
    }

    /// Returns an iterator over the elements of the
    /// `syms_rtrn` field of a `xcb_xkb_get_kbd_by_name_replies_types_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_syms_rtrn_iterator(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_xkb_key_sym_map_iterator_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_syms_rtrn_iterator
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map_syms_rtrn_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map_syms_rtrn_iterator(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_syms_rtrn_iterator
        )
    }

    /// Returns a pointer to the `acts_rtrn_count` field of a `xcb_xkb_get_kbd_by_name_replies_types_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count(
        &self,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut u8 {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count
        )(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count
        )
    }

    /// Returns the number of elements of the `acts_rtrn_count` field of a `xcb_xkb_get_kbd_by_name_replies_types_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count_length(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count_length(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count_length
        )
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `acts_rtrn_count` field of a `xcb_xkb_get_kbd_by_name_replies_types_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count_end(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_generic_iterator_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count_end
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count_end(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count_end
        )
    }

    /// Returns a pointer to the `acts_rtrn_acts` field of a `xcb_xkb_get_kbd_by_name_replies_types_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts(
        &self,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_xkb_action_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts
        )(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts
        )
    }

    /// Returns the number of elements of the `acts_rtrn_acts` field of a `xcb_xkb_get_kbd_by_name_replies_types_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts_length(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts_length(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts_length
        )
    }

    /// Returns an iterator over the elements of the
    /// `acts_rtrn_acts` field of a `xcb_xkb_get_kbd_by_name_replies_types_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts_iterator(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_xkb_action_iterator_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts_iterator
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts_iterator(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts_iterator
        )
    }

    /// Returns a pointer to the `behaviors_rtrn` field of a `xcb_xkb_get_kbd_by_name_replies_types_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn(
        &self,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_xkb_set_behavior_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn
        )(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn
        )
    }

    /// Returns the number of elements of the `behaviors_rtrn` field of a `xcb_xkb_get_kbd_by_name_replies_types_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn_length(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn_length(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn_length
        )
    }

    /// Returns an iterator over the elements of the
    /// `behaviors_rtrn` field of a `xcb_xkb_get_kbd_by_name_replies_types_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn_iterator(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_xkb_set_behavior_iterator_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn_iterator
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn_iterator(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn_iterator
        )
    }

    /// Returns a pointer to the `vmods_rtrn` field of a `xcb_xkb_get_kbd_by_name_replies_types_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn(
        &self,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut u8 {
        sym!(self, xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn(&self) -> bool {
        has_sym!(self, xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn)
    }

    /// Returns the number of elements of the `vmods_rtrn` field of a `xcb_xkb_get_kbd_by_name_replies_types_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn_length(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn_length(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn_length
        )
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `vmods_rtrn` field of a `xcb_xkb_get_kbd_by_name_replies_types_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn_end(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_generic_iterator_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn_end
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn_end(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn_end
        )
    }

    /// Returns a pointer to the `explicit_rtrn` field of a `xcb_xkb_get_kbd_by_name_replies_types_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn(
        &self,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_xkb_set_explicit_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn
        )(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn
        )
    }

    /// Returns the number of elements of the `explicit_rtrn` field of a `xcb_xkb_get_kbd_by_name_replies_types_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn_length(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn_length(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn_length
        )
    }

    /// Returns an iterator over the elements of the
    /// `explicit_rtrn` field of a `xcb_xkb_get_kbd_by_name_replies_types_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn_iterator(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_xkb_set_explicit_iterator_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn_iterator
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn_iterator(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn_iterator
        )
    }

    /// Returns a pointer to the `modmap_rtrn` field of a `xcb_xkb_get_kbd_by_name_replies_types_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn(
        &self,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_xkb_key_mod_map_t {
        sym!(self, xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn(&self) -> bool {
        has_sym!(self, xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn)
    }

    /// Returns the number of elements of the `modmap_rtrn` field of a `xcb_xkb_get_kbd_by_name_replies_types_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn_length(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn_length(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn_length
        )
    }

    /// Returns an iterator over the elements of the
    /// `modmap_rtrn` field of a `xcb_xkb_get_kbd_by_name_replies_types_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn_iterator(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_xkb_key_mod_map_iterator_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn_iterator
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn_iterator(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn_iterator
        )
    }

    /// Returns a pointer to the `vmodmap_rtrn` field of a `xcb_xkb_get_kbd_by_name_replies_types_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn(
        &self,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_xkb_key_v_mod_map_t {
        sym!(self, xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn(&self) -> bool {
        has_sym!(self, xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn)
    }

    /// Returns the number of elements of the `vmodmap_rtrn` field of a `xcb_xkb_get_kbd_by_name_replies_types_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn_length(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn_length(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn_length
        )
    }

    /// Returns an iterator over the elements of the
    /// `vmodmap_rtrn` field of a `xcb_xkb_get_kbd_by_name_replies_types_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn_iterator(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_xkb_key_v_mod_map_iterator_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn_iterator
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn_iterator(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn_iterator
        )
    }

    /// Serializes a `xcb_xkb_get_kbd_by_name_replies_types_map_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_serialize(
        &self,
        _buffer: *mut *mut c_void,
        nTypes: u8,
        nKeySyms: u8,
        nKeyActions: u8,
        totalActions: u16,
        totalKeyBehaviors: u8,
        virtualMods: u16,
        totalKeyExplicit: u8,
        totalModMapKeys: u8,
        totalVModMapKeys: u8,
        present: u16,
        _aux: *const xcb_xkb_get_kbd_by_name_replies_types_map_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_kbd_by_name_replies_types_map_serialize)(
            _buffer,
            nTypes,
            nKeySyms,
            nKeyActions,
            totalActions,
            totalKeyBehaviors,
            virtualMods,
            totalKeyExplicit,
            totalModMapKeys,
            totalVModMapKeys,
            present,
            _aux,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map_serialize(&self) -> bool {
        has_sym!(self, xcb_xkb_get_kbd_by_name_replies_types_map_serialize)
    }

    /// Unpacks a `xcb_xkb_get_kbd_by_name_replies_types_map_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_unpack(
        &self,
        _buffer: *const c_void,
        nTypes: u8,
        nKeySyms: u8,
        nKeyActions: u8,
        totalActions: u16,
        totalKeyBehaviors: u8,
        virtualMods: u16,
        totalKeyExplicit: u8,
        totalModMapKeys: u8,
        totalVModMapKeys: u8,
        present: u16,
        _aux: *mut xcb_xkb_get_kbd_by_name_replies_types_map_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_kbd_by_name_replies_types_map_unpack)(
            _buffer,
            nTypes,
            nKeySyms,
            nKeyActions,
            totalActions,
            totalKeyBehaviors,
            virtualMods,
            totalKeyExplicit,
            totalModMapKeys,
            totalVModMapKeys,
            present,
            _aux,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map_unpack(&self) -> bool {
        has_sym!(self, xcb_xkb_get_kbd_by_name_replies_types_map_unpack)
    }

    /// Computes the size of a `xcb_xkb_get_kbd_by_name_replies_types_map_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map_sizeof(
        &self,
        _buffer: *const c_void,
        nTypes: u8,
        nKeySyms: u8,
        nKeyActions: u8,
        totalActions: u16,
        totalKeyBehaviors: u8,
        virtualMods: u16,
        totalKeyExplicit: u8,
        totalModMapKeys: u8,
        totalVModMapKeys: u8,
        present: u16,
    ) -> c_int {
        sym!(self, xcb_xkb_get_kbd_by_name_replies_types_map_sizeof)(
            _buffer,
            nTypes,
            nKeySyms,
            nKeyActions,
            totalActions,
            totalKeyBehaviors,
            virtualMods,
            totalKeyExplicit,
            totalModMapKeys,
            totalVModMapKeys,
            present,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_get_kbd_by_name_replies_types_map_sizeof)
    }

    /// Returns a pointer to the `typeNames` field of a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names(
        &self,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_atom_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names
        )(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names
        )
    }

    /// Returns the number of elements of the `typeNames` field of a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names_length(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names_length(
        &self,
    ) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names_length
        )
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `typeNames` field of a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names_end(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_generic_iterator_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names_end
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names_end(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names_end
        )
    }

    /// Returns a pointer to the `nLevelsPerType` field of a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type(
        &self,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut u8 {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type
        )(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type(
        &self,
    ) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type
        )
    }

    /// Returns the number of elements of the `nLevelsPerType` field of a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type_length(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type_length(
        &self,
    ) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type_length
        )
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `nLevelsPerType` field of a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type_end(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_generic_iterator_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type_end
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type_end(
        &self,
    ) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type_end
        )
    }

    /// Returns a pointer to the `ktLevelNames` field of a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names(
        &self,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_atom_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names
        )(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names
        )
    }

    /// Returns the number of elements of the `ktLevelNames` field of a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names_length(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names_length(
        &self,
    ) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names_length
        )
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `ktLevelNames` field of a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names_end(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_generic_iterator_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names_end
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names_end(
        &self,
    ) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names_end
        )
    }

    /// Returns a pointer to the `indicatorNames` field of a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names(
        &self,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_atom_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names
        )(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names
        )
    }

    /// Returns the number of elements of the `indicatorNames` field of a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names_length(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names_length(
        &self,
    ) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names_length
        )
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `indicatorNames` field of a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names_end(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_generic_iterator_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names_end
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names_end(
        &self,
    ) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names_end
        )
    }

    /// Returns a pointer to the `virtualModNames` field of a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names(
        &self,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_atom_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names
        )(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names(
        &self,
    ) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names
        )
    }

    /// Returns the number of elements of the `virtualModNames` field of a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names_length(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names_length(
        &self,
    ) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names_length
        )
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `virtualModNames` field of a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names_end(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_generic_iterator_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names_end
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names_end(
        &self,
    ) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names_end
        )
    }

    /// Returns a pointer to the `groups` field of a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups(
        &self,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_atom_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups
        )(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups
        )
    }

    /// Returns the number of elements of the `groups` field of a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups_length(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups_length(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups_length
        )
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `groups` field of a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups_end(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_generic_iterator_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups_end
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups_end(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups_end
        )
    }

    /// Returns a pointer to the `keyNames` field of a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names(
        &self,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_xkb_key_name_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names
        )(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names
        )
    }

    /// Returns the number of elements of the `keyNames` field of a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names_length(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names_length(
        &self,
    ) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names_length
        )
    }

    /// Returns an iterator over the elements of the
    /// `keyNames` field of a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names_iterator(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_xkb_key_name_iterator_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names_iterator
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names_iterator(
        &self,
    ) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names_iterator
        )
    }

    /// Returns a pointer to the `keyAliases` field of a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases(
        &self,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_xkb_key_alias_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases
        )(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases
        )
    }

    /// Returns the number of elements of the `keyAliases` field of a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases_length(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases_length(
        &self,
    ) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases_length
        )
    }

    /// Returns an iterator over the elements of the
    /// `keyAliases` field of a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases_iterator(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_xkb_key_alias_iterator_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases_iterator
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases_iterator(
        &self,
    ) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases_iterator
        )
    }

    /// Returns a pointer to the `radioGroupNames` field of a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names(
        &self,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_atom_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names
        )(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names(
        &self,
    ) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names
        )
    }

    /// Returns the number of elements of the `radioGroupNames` field of a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names_length(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names_length(
        &self,
    ) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names_length
        )
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `radioGroupNames` field of a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names_end(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_generic_iterator_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names_end
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names_end(
        &self,
    ) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names_end
        )
    }

    /// Serializes a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_serialize(
        &self,
        _buffer: *mut *mut c_void,
        nTypes: u8,
        indicators: u32,
        virtualMods: u16,
        groupNames: u8,
        nKeys: u8,
        nKeyAliases: u8,
        nRadioGroups: u8,
        which: u32,
        _aux: *const xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t,
    ) -> c_int {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_serialize
        )(
            _buffer,
            nTypes,
            indicators,
            virtualMods,
            groupNames,
            nKeys,
            nKeyAliases,
            nRadioGroups,
            which,
            _aux,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_serialize(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_serialize
        )
    }

    /// Unpacks a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_unpack(
        &self,
        _buffer: *const c_void,
        nTypes: u8,
        indicators: u32,
        virtualMods: u16,
        groupNames: u8,
        nKeys: u8,
        nKeyAliases: u8,
        nRadioGroups: u8,
        which: u32,
        _aux: *mut xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t,
    ) -> c_int {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_unpack
        )(
            _buffer,
            nTypes,
            indicators,
            virtualMods,
            groupNames,
            nKeys,
            nKeyAliases,
            nRadioGroups,
            which,
            _aux,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_unpack(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_unpack
        )
    }

    /// Computes the size of a `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list_sizeof(
        &self,
        _buffer: *const c_void,
        nTypes: u8,
        indicators: u32,
        virtualMods: u16,
        groupNames: u8,
        nKeys: u8,
        nKeyAliases: u8,
        nRadioGroups: u8,
        which: u32,
    ) -> c_int {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_sizeof
        )(
            _buffer,
            nTypes,
            indicators,
            virtualMods,
            groupNames,
            nKeys,
            nKeyAliases,
            nRadioGroups,
            which,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_sizeof(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_key_names_value_list_sizeof
        )
    }

    /// Returns a pointer to the `map` field of a `xcb_xkb_get_kbd_by_name_replies_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_types_map(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_xkb_get_kbd_by_name_replies_types_map_t {
        sym!(self, xcb_xkb_get_kbd_by_name_replies_types_map)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_types_map` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_types_map(&self) -> bool {
        has_sym!(self, xcb_xkb_get_kbd_by_name_replies_types_map)
    }

    /// Returns a pointer to the `si_rtrn` field of a `xcb_xkb_get_kbd_by_name_replies_compat_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn(
        &self,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_xkb_sym_interpret_t {
        sym!(self, xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn(&self) -> bool {
        has_sym!(self, xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn)
    }

    /// Returns the number of elements of the `si_rtrn` field of a `xcb_xkb_get_kbd_by_name_replies_compat_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn_length(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn_length(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn_length
        )
    }

    /// Returns an iterator over the elements of the
    /// `si_rtrn` field of a `xcb_xkb_get_kbd_by_name_replies_compat_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn_iterator(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_xkb_sym_interpret_iterator_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn_iterator
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn_iterator(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn_iterator
        )
    }

    /// Returns a pointer to the `group_rtrn` field of a `xcb_xkb_get_kbd_by_name_replies_compat_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn(
        &self,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_xkb_mod_def_t {
        sym!(self, xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn(&self) -> bool {
        has_sym!(self, xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn)
    }

    /// Returns the number of elements of the `group_rtrn` field of a `xcb_xkb_get_kbd_by_name_replies_compat_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn_length(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn_length(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn_length
        )
    }

    /// Returns an iterator over the elements of the
    /// `group_rtrn` field of a `xcb_xkb_get_kbd_by_name_replies_compat_map_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn_iterator(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_xkb_mod_def_iterator_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn_iterator
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn_iterator(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn_iterator
        )
    }

    /// Returns a pointer to the `maps` field of a `xcb_xkb_get_kbd_by_name_replies_indicator_maps_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps(
        &self,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_xkb_indicator_map_t {
        sym!(self, xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps)(s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps(&self) -> bool {
        has_sym!(self, xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps)
    }

    /// Returns the number of elements of the `maps` field of a `xcb_xkb_get_kbd_by_name_replies_indicator_maps_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps_length(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps_length
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps_length(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps_length
        )
    }

    /// Returns an iterator over the elements of the
    /// `maps` field of a `xcb_xkb_get_kbd_by_name_replies_indicator_maps_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps_iterator(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
        s: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> xcb_xkb_indicator_map_iterator_t {
        sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps_iterator
        )(r, s)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps_iterator(&self) -> bool {
        has_sym!(
            self,
            xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps_iterator
        )
    }

    /// Returns a pointer to the `valueList` field of a `xcb_xkb_get_kbd_by_name_replies_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_key_names_value_list(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t {
        sym!(self, xcb_xkb_get_kbd_by_name_replies_key_names_value_list)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_key_names_value_list` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list(&self) -> bool {
        has_sym!(self, xcb_xkb_get_kbd_by_name_replies_key_names_value_list)
    }

    /// Returns a pointer to the `labelFont` field of a `xcb_xkb_get_kbd_by_name_replies_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_geometry_label_font(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> *mut xcb_xkb_counted_string_16_t {
        sym!(self, xcb_xkb_get_kbd_by_name_replies_geometry_label_font)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_geometry_label_font` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_geometry_label_font(&self) -> bool {
        has_sym!(self, xcb_xkb_get_kbd_by_name_replies_geometry_label_font)
    }

    /// Serializes a `xcb_xkb_get_kbd_by_name_replies_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_serialize(
        &self,
        _buffer: *mut *mut c_void,
        reported: u16,
        _aux: *const xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_kbd_by_name_replies_serialize)(_buffer, reported, _aux)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_serialize` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_serialize(&self) -> bool {
        has_sym!(self, xcb_xkb_get_kbd_by_name_replies_serialize)
    }

    /// Unpacks a `xcb_xkb_get_kbd_by_name_replies_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_unpack(
        &self,
        _buffer: *const c_void,
        reported: u16,
        _aux: *mut xcb_xkb_get_kbd_by_name_replies_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_kbd_by_name_replies_unpack)(_buffer, reported, _aux)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_unpack` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_unpack(&self) -> bool {
        has_sym!(self, xcb_xkb_get_kbd_by_name_replies_unpack)
    }

    /// Computes the size of a `xcb_xkb_get_kbd_by_name_replies_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies_sizeof(
        &self,
        _buffer: *const c_void,
        reported: u16,
    ) -> c_int {
        sym!(self, xcb_xkb_get_kbd_by_name_replies_sizeof)(_buffer, reported)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_get_kbd_by_name_replies_sizeof)
    }

    /// Computes the size of a `xcb_xkb_get_kbd_by_name_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_get_kbd_by_name_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_get_kbd_by_name_sizeof)
    }

    /// Sends a `xkb::GetKbdByName` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xkb_get_kbd_by_name_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xkb_get_kbd_by_name_reply`]: Self::xcb_xkb_get_kbd_by_name_reply
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        need: u16,
        want: u16,
        load: u8,
    ) -> xcb_xkb_get_kbd_by_name_cookie_t {
        sym!(self, xcb_xkb_get_kbd_by_name)(c, device_spec, need, want, load)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name(&self) -> bool {
        has_sym!(self, xcb_xkb_get_kbd_by_name)
    }

    /// Sends a `xkb::GetKbdByName` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xkb_get_kbd_by_name_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xkb_get_kbd_by_name_reply`]: Self::xcb_xkb_get_kbd_by_name_reply
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        need: u16,
        want: u16,
        load: u8,
    ) -> xcb_xkb_get_kbd_by_name_cookie_t {
        sym!(self, xcb_xkb_get_kbd_by_name_unchecked)(c, device_spec, need, want, load)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_unchecked(&self) -> bool {
        has_sym!(self, xcb_xkb_get_kbd_by_name_unchecked)
    }

    /// Returns a pointer to the `replies` field of a `xcb_xkb_get_kbd_by_name_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_replies(
        &self,
        r: *const xcb_xkb_get_kbd_by_name_reply_t,
    ) -> *mut c_void {
        sym!(self, xcb_xkb_get_kbd_by_name_replies)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_replies` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_replies(&self) -> bool {
        has_sym!(self, xcb_xkb_get_kbd_by_name_replies)
    }

    /// Waits for the reply to a `xkb::GetKbdByName` request.
    #[inline]
    pub unsafe fn xcb_xkb_get_kbd_by_name_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_kbd_by_name_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_get_kbd_by_name_reply_t {
        sym!(self, xcb_xkb_get_kbd_by_name_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_reply(&self) -> bool {
        has_sym!(self, xcb_xkb_get_kbd_by_name_reply)
    }

    /// Computes the size of a `xcb_xkb_get_device_info_reply_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_get_device_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_get_device_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_device_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_device_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_get_device_info_sizeof)
    }

    /// Sends a `xkb::GetDeviceInfo` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xkb_get_device_info_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xkb_get_device_info_reply`]: Self::xcb_xkb_get_device_info_reply
    #[inline]
    pub unsafe fn xcb_xkb_get_device_info(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        wanted: u16,
        all_buttons: u8,
        first_button: u8,
        n_buttons: u8,
        led_class: xcb_xkb_led_class_spec_t,
        led_id: xcb_xkb_id_spec_t,
    ) -> xcb_xkb_get_device_info_cookie_t {
        sym!(self, xcb_xkb_get_device_info)(
            c,
            device_spec,
            wanted,
            all_buttons,
            first_button,
            n_buttons,
            led_class,
            led_id,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_device_info` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_device_info(&self) -> bool {
        has_sym!(self, xcb_xkb_get_device_info)
    }

    /// Sends a `xkb::GetDeviceInfo` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xkb_get_device_info_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xkb_get_device_info_reply`]: Self::xcb_xkb_get_device_info_reply
    #[inline]
    pub unsafe fn xcb_xkb_get_device_info_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        wanted: u16,
        all_buttons: u8,
        first_button: u8,
        n_buttons: u8,
        led_class: xcb_xkb_led_class_spec_t,
        led_id: xcb_xkb_id_spec_t,
    ) -> xcb_xkb_get_device_info_cookie_t {
        sym!(self, xcb_xkb_get_device_info_unchecked)(
            c,
            device_spec,
            wanted,
            all_buttons,
            first_button,
            n_buttons,
            led_class,
            led_id,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_device_info_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_device_info_unchecked(&self) -> bool {
        has_sym!(self, xcb_xkb_get_device_info_unchecked)
    }

    /// Returns a pointer to the `name` field of a `xcb_xkb_get_device_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_device_info_name(
        &self,
        r: *const xcb_xkb_get_device_info_reply_t,
    ) -> *mut xcb_xkb_string8_t {
        sym!(self, xcb_xkb_get_device_info_name)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_device_info_name` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_device_info_name(&self) -> bool {
        has_sym!(self, xcb_xkb_get_device_info_name)
    }

    /// Returns the number of elements of the `name` field of a `xcb_xkb_get_device_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_device_info_name_length(
        &self,
        r: *const xcb_xkb_get_device_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_device_info_name_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_device_info_name_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_device_info_name_length(&self) -> bool {
        has_sym!(self, xcb_xkb_get_device_info_name_length)
    }

    /// Returns a `xcb_generic_iterator_t` pointing just past the end of the
    /// `name` field of a `xcb_xkb_get_device_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_device_info_name_end(
        &self,
        r: *const xcb_xkb_get_device_info_reply_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_get_device_info_name_end)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_device_info_name_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_device_info_name_end(&self) -> bool {
        has_sym!(self, xcb_xkb_get_device_info_name_end)
    }

    /// Returns a pointer to the `btnActions` field of a `xcb_xkb_get_device_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_device_info_btn_actions(
        &self,
        r: *const xcb_xkb_get_device_info_reply_t,
    ) -> *mut xcb_xkb_action_t {
        sym!(self, xcb_xkb_get_device_info_btn_actions)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_device_info_btn_actions` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_device_info_btn_actions(&self) -> bool {
        has_sym!(self, xcb_xkb_get_device_info_btn_actions)
    }

    /// Returns the number of elements of the `btnActions` field of a `xcb_xkb_get_device_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_device_info_btn_actions_length(
        &self,
        r: *const xcb_xkb_get_device_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_device_info_btn_actions_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_device_info_btn_actions_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_device_info_btn_actions_length(&self) -> bool {
        has_sym!(self, xcb_xkb_get_device_info_btn_actions_length)
    }

    /// Returns an iterator over the elements of the
    /// `btnActions` field of a `xcb_xkb_get_device_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_device_info_btn_actions_iterator(
        &self,
        r: *const xcb_xkb_get_device_info_reply_t,
    ) -> xcb_xkb_action_iterator_t {
        sym!(self, xcb_xkb_get_device_info_btn_actions_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_device_info_btn_actions_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_device_info_btn_actions_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_get_device_info_btn_actions_iterator)
    }

    /// Returns the number of elements of the `leds` field of a `xcb_xkb_get_device_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_device_info_leds_length(
        &self,
        r: *const xcb_xkb_get_device_info_reply_t,
    ) -> c_int {
        sym!(self, xcb_xkb_get_device_info_leds_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_device_info_leds_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_device_info_leds_length(&self) -> bool {
        has_sym!(self, xcb_xkb_get_device_info_leds_length)
    }

    /// Returns an iterator over the elements of the
    /// `leds` field of a `xcb_xkb_get_device_info_reply_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_get_device_info_leds_iterator(
        &self,
        r: *const xcb_xkb_get_device_info_reply_t,
    ) -> xcb_xkb_device_led_info_iterator_t {
        sym!(self, xcb_xkb_get_device_info_leds_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_device_info_leds_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_device_info_leds_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_get_device_info_leds_iterator)
    }

    /// Waits for the reply to a `xkb::GetDeviceInfo` request.
    #[inline]
    pub unsafe fn xcb_xkb_get_device_info_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_device_info_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_get_device_info_reply_t {
        sym!(self, xcb_xkb_get_device_info_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_device_info_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_device_info_reply(&self) -> bool {
        has_sym!(self, xcb_xkb_get_device_info_reply)
    }

    /// Computes the size of a `xcb_xkb_set_device_info_request_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_set_device_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_set_device_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_device_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_device_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_set_device_info_sizeof)
    }

    /// Sends a `xkb::SetDeviceInfo` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_request_check`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_request_check`]: crate::Xcb::xcb_request_check
    #[inline]
    pub unsafe fn xcb_xkb_set_device_info_checked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        first_btn: u8,
        n_btns: u8,
        change: u16,
        n_device_led_fbs: u16,
        btn_actions: *const xcb_xkb_action_t,
        leds: *const xcb_xkb_device_led_info_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_set_device_info_checked)(
            c,
            device_spec,
            first_btn,
            n_btns,
            change,
            n_device_led_fbs,
            btn_actions,
            leds,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_device_info_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_device_info_checked(&self) -> bool {
        has_sym!(self, xcb_xkb_set_device_info_checked)
    }

    /// Sends a `xkb::SetDeviceInfo` request (unchecked).
    #[inline]
    pub unsafe fn xcb_xkb_set_device_info(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        first_btn: u8,
        n_btns: u8,
        change: u16,
        n_device_led_fbs: u16,
        btn_actions: *const xcb_xkb_action_t,
        leds: *const xcb_xkb_device_led_info_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_set_device_info)(
            c,
            device_spec,
            first_btn,
            n_btns,
            change,
            n_device_led_fbs,
            btn_actions,
            leds,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_device_info` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_device_info(&self) -> bool {
        has_sym!(self, xcb_xkb_set_device_info)
    }

    /// Returns a pointer to the `btnActions` field of a `xcb_xkb_set_device_info_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_device_info_btn_actions(
        &self,
        r: *const xcb_xkb_set_device_info_request_t,
    ) -> *mut xcb_xkb_action_t {
        sym!(self, xcb_xkb_set_device_info_btn_actions)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_device_info_btn_actions` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_device_info_btn_actions(&self) -> bool {
        has_sym!(self, xcb_xkb_set_device_info_btn_actions)
    }

    /// Returns the number of elements of the `btnActions` field of a `xcb_xkb_set_device_info_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_device_info_btn_actions_length(
        &self,
        r: *const xcb_xkb_set_device_info_request_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_device_info_btn_actions_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_device_info_btn_actions_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_device_info_btn_actions_length(&self) -> bool {
        has_sym!(self, xcb_xkb_set_device_info_btn_actions_length)
    }

    /// Returns an iterator over the elements of the
    /// `btnActions` field of a `xcb_xkb_set_device_info_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_device_info_btn_actions_iterator(
        &self,
        r: *const xcb_xkb_set_device_info_request_t,
    ) -> xcb_xkb_action_iterator_t {
        sym!(self, xcb_xkb_set_device_info_btn_actions_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_device_info_btn_actions_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_device_info_btn_actions_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_set_device_info_btn_actions_iterator)
    }

    /// Returns the number of elements of the `leds` field of a `xcb_xkb_set_device_info_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_device_info_leds_length(
        &self,
        r: *const xcb_xkb_set_device_info_request_t,
    ) -> c_int {
        sym!(self, xcb_xkb_set_device_info_leds_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_device_info_leds_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_device_info_leds_length(&self) -> bool {
        has_sym!(self, xcb_xkb_set_device_info_leds_length)
    }

    /// Returns an iterator over the elements of the
    /// `leds` field of a `xcb_xkb_set_device_info_request_t` struct.
    #[inline]
    pub unsafe fn xcb_xkb_set_device_info_leds_iterator(
        &self,
        r: *const xcb_xkb_set_device_info_request_t,
    ) -> xcb_xkb_device_led_info_iterator_t {
        sym!(self, xcb_xkb_set_device_info_leds_iterator)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_device_info_leds_iterator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_device_info_leds_iterator(&self) -> bool {
        has_sym!(self, xcb_xkb_set_device_info_leds_iterator)
    }

    /// Computes the size of a `xcb_xkb_set_debugging_flags_request_t` object.
    #[inline]
    pub unsafe fn xcb_xkb_set_debugging_flags_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_set_debugging_flags_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_debugging_flags_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_debugging_flags_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_set_debugging_flags_sizeof)
    }

    /// Sends a `xkb::SetDebuggingFlags` request (checked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xkb_set_debugging_flags_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xkb_set_debugging_flags_reply`]: Self::xcb_xkb_set_debugging_flags_reply
    #[inline]
    pub unsafe fn xcb_xkb_set_debugging_flags(
        &self,
        c: *mut xcb_connection_t,
        msg_length: u16,
        affect_flags: u32,
        flags: u32,
        affect_ctrls: u32,
        ctrls: u32,
        message: *const xcb_xkb_string8_t,
    ) -> xcb_xkb_set_debugging_flags_cookie_t {
        sym!(self, xcb_xkb_set_debugging_flags)(
            c,
            msg_length,
            affect_flags,
            flags,
            affect_ctrls,
            ctrls,
            message,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_debugging_flags` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_debugging_flags(&self) -> bool {
        has_sym!(self, xcb_xkb_set_debugging_flags)
    }

    /// Sends a `xkb::SetDebuggingFlags` request (unchecked).
    ///
    /// This request generates a reply. You must either discard it with
    /// [`discard_reply`] or retrieve it with [`xcb_xkb_set_debugging_flags_reply`].
    ///
    /// [`discard_reply`]: crate::Xcb::xcb_discard_reply
    /// [`xcb_xkb_set_debugging_flags_reply`]: Self::xcb_xkb_set_debugging_flags_reply
    #[inline]
    pub unsafe fn xcb_xkb_set_debugging_flags_unchecked(
        &self,
        c: *mut xcb_connection_t,
        msg_length: u16,
        affect_flags: u32,
        flags: u32,
        affect_ctrls: u32,
        ctrls: u32,
        message: *const xcb_xkb_string8_t,
    ) -> xcb_xkb_set_debugging_flags_cookie_t {
        sym!(self, xcb_xkb_set_debugging_flags_unchecked)(
            c,
            msg_length,
            affect_flags,
            flags,
            affect_ctrls,
            ctrls,
            message,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_debugging_flags_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_debugging_flags_unchecked(&self) -> bool {
        has_sym!(self, xcb_xkb_set_debugging_flags_unchecked)
    }

    /// Waits for the reply to a `xkb::SetDebuggingFlags` request.
    #[inline]
    pub unsafe fn xcb_xkb_set_debugging_flags_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_set_debugging_flags_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_xkb_set_debugging_flags_reply_t {
        sym!(self, xcb_xkb_set_debugging_flags_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_debugging_flags_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_debugging_flags_reply(&self) -> bool {
        has_sym!(self, xcb_xkb_set_debugging_flags_reply)
    }
}

#[cfg(feature = "xcb_xkb")]
#[cfg(all(test, feature = "has_symbol"))]
mod test {
    #[test]
    fn has_all() {
        let lib = unsafe { crate::XcbXkb::load().unwrap() };
        assert!(lib.has_xcb_xkb_id());
        assert!(lib.has_xcb_xkb_device_spec_next());
        assert!(lib.has_xcb_xkb_device_spec_end());
        assert!(lib.has_xcb_xkb_led_class_spec_next());
        assert!(lib.has_xcb_xkb_led_class_spec_end());
        assert!(lib.has_xcb_xkb_bell_class_spec_next());
        assert!(lib.has_xcb_xkb_bell_class_spec_end());
        assert!(lib.has_xcb_xkb_id_spec_next());
        assert!(lib.has_xcb_xkb_id_spec_end());
        assert!(lib.has_xcb_xkb_indicator_map_next());
        assert!(lib.has_xcb_xkb_indicator_map_end());
        assert!(lib.has_xcb_xkb_mod_def_next());
        assert!(lib.has_xcb_xkb_mod_def_end());
        assert!(lib.has_xcb_xkb_key_name_next());
        assert!(lib.has_xcb_xkb_key_name_end());
        assert!(lib.has_xcb_xkb_key_alias_next());
        assert!(lib.has_xcb_xkb_key_alias_end());
        assert!(lib.has_xcb_xkb_counted_string_16_sizeof());
        assert!(lib.has_xcb_xkb_counted_string_16_string());
        assert!(lib.has_xcb_xkb_counted_string_16_string_length());
        assert!(lib.has_xcb_xkb_counted_string_16_string_end());
        assert!(lib.has_xcb_xkb_counted_string_16_alignment_pad());
        assert!(lib.has_xcb_xkb_counted_string_16_alignment_pad_length());
        assert!(lib.has_xcb_xkb_counted_string_16_alignment_pad_end());
        assert!(lib.has_xcb_xkb_counted_string_16_next());
        assert!(lib.has_xcb_xkb_counted_string_16_end());
        assert!(lib.has_xcb_xkb_kt_map_entry_next());
        assert!(lib.has_xcb_xkb_kt_map_entry_end());
        assert!(lib.has_xcb_xkb_key_type_sizeof());
        assert!(lib.has_xcb_xkb_key_type_map());
        assert!(lib.has_xcb_xkb_key_type_map_length());
        assert!(lib.has_xcb_xkb_key_type_map_iterator());
        assert!(lib.has_xcb_xkb_key_type_preserve());
        assert!(lib.has_xcb_xkb_key_type_preserve_length());
        assert!(lib.has_xcb_xkb_key_type_preserve_iterator());
        assert!(lib.has_xcb_xkb_key_type_next());
        assert!(lib.has_xcb_xkb_key_type_end());
        assert!(lib.has_xcb_xkb_key_sym_map_sizeof());
        assert!(lib.has_xcb_xkb_key_sym_map_syms());
        assert!(lib.has_xcb_xkb_key_sym_map_syms_length());
        assert!(lib.has_xcb_xkb_key_sym_map_syms_end());
        assert!(lib.has_xcb_xkb_key_sym_map_next());
        assert!(lib.has_xcb_xkb_key_sym_map_end());
        assert!(lib.has_xcb_xkb_common_behavior_next());
        assert!(lib.has_xcb_xkb_common_behavior_end());
        assert!(lib.has_xcb_xkb_default_behavior_next());
        assert!(lib.has_xcb_xkb_default_behavior_end());
        assert!(lib.has_xcb_xkb_lock_behavior_next());
        assert!(lib.has_xcb_xkb_lock_behavior_end());
        assert!(lib.has_xcb_xkb_radio_group_behavior_next());
        assert!(lib.has_xcb_xkb_radio_group_behavior_end());
        assert!(lib.has_xcb_xkb_overlay_behavior_next());
        assert!(lib.has_xcb_xkb_overlay_behavior_end());
        assert!(lib.has_xcb_xkb_permament_lock_behavior_next());
        assert!(lib.has_xcb_xkb_permament_lock_behavior_end());
        assert!(lib.has_xcb_xkb_permament_radio_group_behavior_next());
        assert!(lib.has_xcb_xkb_permament_radio_group_behavior_end());
        assert!(lib.has_xcb_xkb_permament_overlay_behavior_next());
        assert!(lib.has_xcb_xkb_permament_overlay_behavior_end());
        assert!(lib.has_xcb_xkb_behavior_next());
        assert!(lib.has_xcb_xkb_behavior_end());
        assert!(lib.has_xcb_xkb_set_behavior_next());
        assert!(lib.has_xcb_xkb_set_behavior_end());
        assert!(lib.has_xcb_xkb_set_explicit_next());
        assert!(lib.has_xcb_xkb_set_explicit_end());
        assert!(lib.has_xcb_xkb_key_mod_map_next());
        assert!(lib.has_xcb_xkb_key_mod_map_end());
        assert!(lib.has_xcb_xkb_key_v_mod_map_next());
        assert!(lib.has_xcb_xkb_key_v_mod_map_end());
        assert!(lib.has_xcb_xkb_kt_set_map_entry_next());
        assert!(lib.has_xcb_xkb_kt_set_map_entry_end());
        assert!(lib.has_xcb_xkb_set_key_type_sizeof());
        assert!(lib.has_xcb_xkb_set_key_type_entries());
        assert!(lib.has_xcb_xkb_set_key_type_entries_length());
        assert!(lib.has_xcb_xkb_set_key_type_entries_iterator());
        assert!(lib.has_xcb_xkb_set_key_type_preserve_entries());
        assert!(lib.has_xcb_xkb_set_key_type_preserve_entries_length());
        assert!(lib.has_xcb_xkb_set_key_type_preserve_entries_iterator());
        assert!(lib.has_xcb_xkb_set_key_type_next());
        assert!(lib.has_xcb_xkb_set_key_type_end());
        assert!(lib.has_xcb_xkb_string8_next());
        assert!(lib.has_xcb_xkb_string8_end());
        assert!(lib.has_xcb_xkb_outline_sizeof());
        assert!(lib.has_xcb_xkb_outline_points());
        assert!(lib.has_xcb_xkb_outline_points_length());
        assert!(lib.has_xcb_xkb_outline_points_iterator());
        assert!(lib.has_xcb_xkb_outline_next());
        assert!(lib.has_xcb_xkb_outline_end());
        assert!(lib.has_xcb_xkb_shape_sizeof());
        assert!(lib.has_xcb_xkb_shape_outlines_length());
        assert!(lib.has_xcb_xkb_shape_outlines_iterator());
        assert!(lib.has_xcb_xkb_shape_next());
        assert!(lib.has_xcb_xkb_shape_end());
        assert!(lib.has_xcb_xkb_key_next());
        assert!(lib.has_xcb_xkb_key_end());
        assert!(lib.has_xcb_xkb_overlay_key_next());
        assert!(lib.has_xcb_xkb_overlay_key_end());
        assert!(lib.has_xcb_xkb_overlay_row_sizeof());
        assert!(lib.has_xcb_xkb_overlay_row_keys());
        assert!(lib.has_xcb_xkb_overlay_row_keys_length());
        assert!(lib.has_xcb_xkb_overlay_row_keys_iterator());
        assert!(lib.has_xcb_xkb_overlay_row_next());
        assert!(lib.has_xcb_xkb_overlay_row_end());
        assert!(lib.has_xcb_xkb_overlay_sizeof());
        assert!(lib.has_xcb_xkb_overlay_rows_length());
        assert!(lib.has_xcb_xkb_overlay_rows_iterator());
        assert!(lib.has_xcb_xkb_overlay_next());
        assert!(lib.has_xcb_xkb_overlay_end());
        assert!(lib.has_xcb_xkb_row_sizeof());
        assert!(lib.has_xcb_xkb_row_keys());
        assert!(lib.has_xcb_xkb_row_keys_length());
        assert!(lib.has_xcb_xkb_row_keys_iterator());
        assert!(lib.has_xcb_xkb_row_next());
        assert!(lib.has_xcb_xkb_row_end());
        assert!(lib.has_xcb_xkb_listing_sizeof());
        assert!(lib.has_xcb_xkb_listing_string());
        assert!(lib.has_xcb_xkb_listing_string_length());
        assert!(lib.has_xcb_xkb_listing_string_end());
        assert!(lib.has_xcb_xkb_listing_next());
        assert!(lib.has_xcb_xkb_listing_end());
        assert!(lib.has_xcb_xkb_device_led_info_sizeof());
        assert!(lib.has_xcb_xkb_device_led_info_names());
        assert!(lib.has_xcb_xkb_device_led_info_names_length());
        assert!(lib.has_xcb_xkb_device_led_info_names_end());
        assert!(lib.has_xcb_xkb_device_led_info_maps());
        assert!(lib.has_xcb_xkb_device_led_info_maps_length());
        assert!(lib.has_xcb_xkb_device_led_info_maps_iterator());
        assert!(lib.has_xcb_xkb_device_led_info_next());
        assert!(lib.has_xcb_xkb_device_led_info_end());
        assert!(lib.has_xcb_xkb_sa_no_action_next());
        assert!(lib.has_xcb_xkb_sa_no_action_end());
        assert!(lib.has_xcb_xkb_sa_set_mods_next());
        assert!(lib.has_xcb_xkb_sa_set_mods_end());
        assert!(lib.has_xcb_xkb_sa_latch_mods_next());
        assert!(lib.has_xcb_xkb_sa_latch_mods_end());
        assert!(lib.has_xcb_xkb_sa_lock_mods_next());
        assert!(lib.has_xcb_xkb_sa_lock_mods_end());
        assert!(lib.has_xcb_xkb_sa_set_group_next());
        assert!(lib.has_xcb_xkb_sa_set_group_end());
        assert!(lib.has_xcb_xkb_sa_latch_group_next());
        assert!(lib.has_xcb_xkb_sa_latch_group_end());
        assert!(lib.has_xcb_xkb_sa_lock_group_next());
        assert!(lib.has_xcb_xkb_sa_lock_group_end());
        assert!(lib.has_xcb_xkb_sa_move_ptr_next());
        assert!(lib.has_xcb_xkb_sa_move_ptr_end());
        assert!(lib.has_xcb_xkb_sa_ptr_btn_next());
        assert!(lib.has_xcb_xkb_sa_ptr_btn_end());
        assert!(lib.has_xcb_xkb_sa_lock_ptr_btn_next());
        assert!(lib.has_xcb_xkb_sa_lock_ptr_btn_end());
        assert!(lib.has_xcb_xkb_sa_set_ptr_dflt_next());
        assert!(lib.has_xcb_xkb_sa_set_ptr_dflt_end());
        assert!(lib.has_xcb_xkb_sa_iso_lock_next());
        assert!(lib.has_xcb_xkb_sa_iso_lock_end());
        assert!(lib.has_xcb_xkb_sa_terminate_next());
        assert!(lib.has_xcb_xkb_sa_terminate_end());
        assert!(lib.has_xcb_xkb_sa_switch_screen_next());
        assert!(lib.has_xcb_xkb_sa_switch_screen_end());
        assert!(lib.has_xcb_xkb_sa_set_controls_next());
        assert!(lib.has_xcb_xkb_sa_set_controls_end());
        assert!(lib.has_xcb_xkb_sa_lock_controls_next());
        assert!(lib.has_xcb_xkb_sa_lock_controls_end());
        assert!(lib.has_xcb_xkb_sa_action_message_next());
        assert!(lib.has_xcb_xkb_sa_action_message_end());
        assert!(lib.has_xcb_xkb_sa_redirect_key_next());
        assert!(lib.has_xcb_xkb_sa_redirect_key_end());
        assert!(lib.has_xcb_xkb_sa_device_btn_next());
        assert!(lib.has_xcb_xkb_sa_device_btn_end());
        assert!(lib.has_xcb_xkb_sa_lock_device_btn_next());
        assert!(lib.has_xcb_xkb_sa_lock_device_btn_end());
        assert!(lib.has_xcb_xkb_sa_device_valuator_next());
        assert!(lib.has_xcb_xkb_sa_device_valuator_end());
        assert!(lib.has_xcb_xkb_si_action_next());
        assert!(lib.has_xcb_xkb_si_action_end());
        assert!(lib.has_xcb_xkb_sym_interpret_next());
        assert!(lib.has_xcb_xkb_sym_interpret_end());
        assert!(lib.has_xcb_xkb_action_next());
        assert!(lib.has_xcb_xkb_action_end());
        assert!(lib.has_xcb_xkb_use_extension());
        assert!(lib.has_xcb_xkb_use_extension_unchecked());
        assert!(lib.has_xcb_xkb_use_extension_reply());
        assert!(lib.has_xcb_xkb_select_events_details_serialize());
        assert!(lib.has_xcb_xkb_select_events_details_unpack());
        assert!(lib.has_xcb_xkb_select_events_details_sizeof());
        assert!(lib.has_xcb_xkb_select_events_sizeof());
        assert!(lib.has_xcb_xkb_select_events_checked());
        assert!(lib.has_xcb_xkb_select_events());
        assert!(lib.has_xcb_xkb_select_events_aux_checked());
        assert!(lib.has_xcb_xkb_select_events_aux());
        assert!(lib.has_xcb_xkb_select_events_details());
        assert!(lib.has_xcb_xkb_bell_checked());
        assert!(lib.has_xcb_xkb_bell());
        assert!(lib.has_xcb_xkb_get_state());
        assert!(lib.has_xcb_xkb_get_state_unchecked());
        assert!(lib.has_xcb_xkb_get_state_reply());
        assert!(lib.has_xcb_xkb_latch_lock_state_checked());
        assert!(lib.has_xcb_xkb_latch_lock_state());
        assert!(lib.has_xcb_xkb_get_controls());
        assert!(lib.has_xcb_xkb_get_controls_unchecked());
        assert!(lib.has_xcb_xkb_get_controls_reply());
        assert!(lib.has_xcb_xkb_set_controls_checked());
        assert!(lib.has_xcb_xkb_set_controls());
        assert!(lib.has_xcb_xkb_get_map_map_types_rtrn_length());
        assert!(lib.has_xcb_xkb_get_map_map_types_rtrn_iterator());
        assert!(lib.has_xcb_xkb_get_map_map_syms_rtrn_length());
        assert!(lib.has_xcb_xkb_get_map_map_syms_rtrn_iterator());
        assert!(lib.has_xcb_xkb_get_map_map_acts_rtrn_count());
        assert!(lib.has_xcb_xkb_get_map_map_acts_rtrn_count_length());
        assert!(lib.has_xcb_xkb_get_map_map_acts_rtrn_count_end());
        assert!(lib.has_xcb_xkb_get_map_map_acts_rtrn_acts());
        assert!(lib.has_xcb_xkb_get_map_map_acts_rtrn_acts_length());
        assert!(lib.has_xcb_xkb_get_map_map_acts_rtrn_acts_iterator());
        assert!(lib.has_xcb_xkb_get_map_map_behaviors_rtrn());
        assert!(lib.has_xcb_xkb_get_map_map_behaviors_rtrn_length());
        assert!(lib.has_xcb_xkb_get_map_map_behaviors_rtrn_iterator());
        assert!(lib.has_xcb_xkb_get_map_map_vmods_rtrn());
        assert!(lib.has_xcb_xkb_get_map_map_vmods_rtrn_length());
        assert!(lib.has_xcb_xkb_get_map_map_vmods_rtrn_end());
        assert!(lib.has_xcb_xkb_get_map_map_explicit_rtrn());
        assert!(lib.has_xcb_xkb_get_map_map_explicit_rtrn_length());
        assert!(lib.has_xcb_xkb_get_map_map_explicit_rtrn_iterator());
        assert!(lib.has_xcb_xkb_get_map_map_modmap_rtrn());
        assert!(lib.has_xcb_xkb_get_map_map_modmap_rtrn_length());
        assert!(lib.has_xcb_xkb_get_map_map_modmap_rtrn_iterator());
        assert!(lib.has_xcb_xkb_get_map_map_vmodmap_rtrn());
        assert!(lib.has_xcb_xkb_get_map_map_vmodmap_rtrn_length());
        assert!(lib.has_xcb_xkb_get_map_map_vmodmap_rtrn_iterator());
        assert!(lib.has_xcb_xkb_get_map_map_serialize());
        assert!(lib.has_xcb_xkb_get_map_map_unpack());
        assert!(lib.has_xcb_xkb_get_map_map_sizeof());
        assert!(lib.has_xcb_xkb_get_map_sizeof());
        assert!(lib.has_xcb_xkb_get_map());
        assert!(lib.has_xcb_xkb_get_map_unchecked());
        assert!(lib.has_xcb_xkb_get_map_map());
        assert!(lib.has_xcb_xkb_get_map_reply());
        assert!(lib.has_xcb_xkb_set_map_values_types_length());
        assert!(lib.has_xcb_xkb_set_map_values_types_iterator());
        assert!(lib.has_xcb_xkb_set_map_values_syms_length());
        assert!(lib.has_xcb_xkb_set_map_values_syms_iterator());
        assert!(lib.has_xcb_xkb_set_map_values_actions_count());
        assert!(lib.has_xcb_xkb_set_map_values_actions_count_length());
        assert!(lib.has_xcb_xkb_set_map_values_actions_count_end());
        assert!(lib.has_xcb_xkb_set_map_values_actions());
        assert!(lib.has_xcb_xkb_set_map_values_actions_length());
        assert!(lib.has_xcb_xkb_set_map_values_actions_iterator());
        assert!(lib.has_xcb_xkb_set_map_values_behaviors());
        assert!(lib.has_xcb_xkb_set_map_values_behaviors_length());
        assert!(lib.has_xcb_xkb_set_map_values_behaviors_iterator());
        assert!(lib.has_xcb_xkb_set_map_values_vmods());
        assert!(lib.has_xcb_xkb_set_map_values_vmods_length());
        assert!(lib.has_xcb_xkb_set_map_values_vmods_end());
        assert!(lib.has_xcb_xkb_set_map_values_explicit());
        assert!(lib.has_xcb_xkb_set_map_values_explicit_length());
        assert!(lib.has_xcb_xkb_set_map_values_explicit_iterator());
        assert!(lib.has_xcb_xkb_set_map_values_modmap());
        assert!(lib.has_xcb_xkb_set_map_values_modmap_length());
        assert!(lib.has_xcb_xkb_set_map_values_modmap_iterator());
        assert!(lib.has_xcb_xkb_set_map_values_vmodmap());
        assert!(lib.has_xcb_xkb_set_map_values_vmodmap_length());
        assert!(lib.has_xcb_xkb_set_map_values_vmodmap_iterator());
        assert!(lib.has_xcb_xkb_set_map_values_serialize());
        assert!(lib.has_xcb_xkb_set_map_values_unpack());
        assert!(lib.has_xcb_xkb_set_map_values_sizeof());
        assert!(lib.has_xcb_xkb_set_map_sizeof());
        assert!(lib.has_xcb_xkb_set_map_checked());
        assert!(lib.has_xcb_xkb_set_map());
        assert!(lib.has_xcb_xkb_set_map_aux_checked());
        assert!(lib.has_xcb_xkb_set_map_aux());
        assert!(lib.has_xcb_xkb_set_map_values());
        assert!(lib.has_xcb_xkb_get_compat_map_sizeof());
        assert!(lib.has_xcb_xkb_get_compat_map());
        assert!(lib.has_xcb_xkb_get_compat_map_unchecked());
        assert!(lib.has_xcb_xkb_get_compat_map_si_rtrn());
        assert!(lib.has_xcb_xkb_get_compat_map_si_rtrn_length());
        assert!(lib.has_xcb_xkb_get_compat_map_si_rtrn_iterator());
        assert!(lib.has_xcb_xkb_get_compat_map_group_rtrn());
        assert!(lib.has_xcb_xkb_get_compat_map_group_rtrn_length());
        assert!(lib.has_xcb_xkb_get_compat_map_group_rtrn_iterator());
        assert!(lib.has_xcb_xkb_get_compat_map_reply());
        assert!(lib.has_xcb_xkb_set_compat_map_sizeof());
        assert!(lib.has_xcb_xkb_set_compat_map_checked());
        assert!(lib.has_xcb_xkb_set_compat_map());
        assert!(lib.has_xcb_xkb_set_compat_map_si());
        assert!(lib.has_xcb_xkb_set_compat_map_si_length());
        assert!(lib.has_xcb_xkb_set_compat_map_si_iterator());
        assert!(lib.has_xcb_xkb_set_compat_map_group_maps());
        assert!(lib.has_xcb_xkb_set_compat_map_group_maps_length());
        assert!(lib.has_xcb_xkb_set_compat_map_group_maps_iterator());
        assert!(lib.has_xcb_xkb_get_indicator_state());
        assert!(lib.has_xcb_xkb_get_indicator_state_unchecked());
        assert!(lib.has_xcb_xkb_get_indicator_state_reply());
        assert!(lib.has_xcb_xkb_get_indicator_map_sizeof());
        assert!(lib.has_xcb_xkb_get_indicator_map());
        assert!(lib.has_xcb_xkb_get_indicator_map_unchecked());
        assert!(lib.has_xcb_xkb_get_indicator_map_maps());
        assert!(lib.has_xcb_xkb_get_indicator_map_maps_length());
        assert!(lib.has_xcb_xkb_get_indicator_map_maps_iterator());
        assert!(lib.has_xcb_xkb_get_indicator_map_reply());
        assert!(lib.has_xcb_xkb_set_indicator_map_sizeof());
        assert!(lib.has_xcb_xkb_set_indicator_map_checked());
        assert!(lib.has_xcb_xkb_set_indicator_map());
        assert!(lib.has_xcb_xkb_set_indicator_map_maps());
        assert!(lib.has_xcb_xkb_set_indicator_map_maps_length());
        assert!(lib.has_xcb_xkb_set_indicator_map_maps_iterator());
        assert!(lib.has_xcb_xkb_get_named_indicator());
        assert!(lib.has_xcb_xkb_get_named_indicator_unchecked());
        assert!(lib.has_xcb_xkb_get_named_indicator_reply());
        assert!(lib.has_xcb_xkb_set_named_indicator_checked());
        assert!(lib.has_xcb_xkb_set_named_indicator());
        assert!(lib.has_xcb_xkb_get_names_value_list_type_names());
        assert!(lib.has_xcb_xkb_get_names_value_list_type_names_length());
        assert!(lib.has_xcb_xkb_get_names_value_list_type_names_end());
        assert!(lib.has_xcb_xkb_get_names_value_list_n_levels_per_type());
        assert!(lib.has_xcb_xkb_get_names_value_list_n_levels_per_type_length());
        assert!(lib.has_xcb_xkb_get_names_value_list_n_levels_per_type_end());
        assert!(lib.has_xcb_xkb_get_names_value_list_kt_level_names());
        assert!(lib.has_xcb_xkb_get_names_value_list_kt_level_names_length());
        assert!(lib.has_xcb_xkb_get_names_value_list_kt_level_names_end());
        assert!(lib.has_xcb_xkb_get_names_value_list_indicator_names());
        assert!(lib.has_xcb_xkb_get_names_value_list_indicator_names_length());
        assert!(lib.has_xcb_xkb_get_names_value_list_indicator_names_end());
        assert!(lib.has_xcb_xkb_get_names_value_list_virtual_mod_names());
        assert!(lib.has_xcb_xkb_get_names_value_list_virtual_mod_names_length());
        assert!(lib.has_xcb_xkb_get_names_value_list_virtual_mod_names_end());
        assert!(lib.has_xcb_xkb_get_names_value_list_groups());
        assert!(lib.has_xcb_xkb_get_names_value_list_groups_length());
        assert!(lib.has_xcb_xkb_get_names_value_list_groups_end());
        assert!(lib.has_xcb_xkb_get_names_value_list_key_names());
        assert!(lib.has_xcb_xkb_get_names_value_list_key_names_length());
        assert!(lib.has_xcb_xkb_get_names_value_list_key_names_iterator());
        assert!(lib.has_xcb_xkb_get_names_value_list_key_aliases());
        assert!(lib.has_xcb_xkb_get_names_value_list_key_aliases_length());
        assert!(lib.has_xcb_xkb_get_names_value_list_key_aliases_iterator());
        assert!(lib.has_xcb_xkb_get_names_value_list_radio_group_names());
        assert!(lib.has_xcb_xkb_get_names_value_list_radio_group_names_length());
        assert!(lib.has_xcb_xkb_get_names_value_list_radio_group_names_end());
        assert!(lib.has_xcb_xkb_get_names_value_list_serialize());
        assert!(lib.has_xcb_xkb_get_names_value_list_unpack());
        assert!(lib.has_xcb_xkb_get_names_value_list_sizeof());
        assert!(lib.has_xcb_xkb_get_names_sizeof());
        assert!(lib.has_xcb_xkb_get_names());
        assert!(lib.has_xcb_xkb_get_names_unchecked());
        assert!(lib.has_xcb_xkb_get_names_value_list());
        assert!(lib.has_xcb_xkb_get_names_reply());
        assert!(lib.has_xcb_xkb_set_names_values_type_names());
        assert!(lib.has_xcb_xkb_set_names_values_type_names_length());
        assert!(lib.has_xcb_xkb_set_names_values_type_names_end());
        assert!(lib.has_xcb_xkb_set_names_values_n_levels_per_type());
        assert!(lib.has_xcb_xkb_set_names_values_n_levels_per_type_length());
        assert!(lib.has_xcb_xkb_set_names_values_n_levels_per_type_end());
        assert!(lib.has_xcb_xkb_set_names_values_kt_level_names());
        assert!(lib.has_xcb_xkb_set_names_values_kt_level_names_length());
        assert!(lib.has_xcb_xkb_set_names_values_kt_level_names_end());
        assert!(lib.has_xcb_xkb_set_names_values_indicator_names());
        assert!(lib.has_xcb_xkb_set_names_values_indicator_names_length());
        assert!(lib.has_xcb_xkb_set_names_values_indicator_names_end());
        assert!(lib.has_xcb_xkb_set_names_values_virtual_mod_names());
        assert!(lib.has_xcb_xkb_set_names_values_virtual_mod_names_length());
        assert!(lib.has_xcb_xkb_set_names_values_virtual_mod_names_end());
        assert!(lib.has_xcb_xkb_set_names_values_groups());
        assert!(lib.has_xcb_xkb_set_names_values_groups_length());
        assert!(lib.has_xcb_xkb_set_names_values_groups_end());
        assert!(lib.has_xcb_xkb_set_names_values_key_names());
        assert!(lib.has_xcb_xkb_set_names_values_key_names_length());
        assert!(lib.has_xcb_xkb_set_names_values_key_names_iterator());
        assert!(lib.has_xcb_xkb_set_names_values_key_aliases());
        assert!(lib.has_xcb_xkb_set_names_values_key_aliases_length());
        assert!(lib.has_xcb_xkb_set_names_values_key_aliases_iterator());
        assert!(lib.has_xcb_xkb_set_names_values_radio_group_names());
        assert!(lib.has_xcb_xkb_set_names_values_radio_group_names_length());
        assert!(lib.has_xcb_xkb_set_names_values_radio_group_names_end());
        assert!(lib.has_xcb_xkb_set_names_values_serialize());
        assert!(lib.has_xcb_xkb_set_names_values_unpack());
        assert!(lib.has_xcb_xkb_set_names_values_sizeof());
        assert!(lib.has_xcb_xkb_set_names_sizeof());
        assert!(lib.has_xcb_xkb_set_names_checked());
        assert!(lib.has_xcb_xkb_set_names());
        assert!(lib.has_xcb_xkb_set_names_aux_checked());
        assert!(lib.has_xcb_xkb_set_names_aux());
        assert!(lib.has_xcb_xkb_set_names_values());
        assert!(lib.has_xcb_xkb_per_client_flags());
        assert!(lib.has_xcb_xkb_per_client_flags_unchecked());
        assert!(lib.has_xcb_xkb_per_client_flags_reply());
        assert!(lib.has_xcb_xkb_list_components_sizeof());
        assert!(lib.has_xcb_xkb_list_components());
        assert!(lib.has_xcb_xkb_list_components_unchecked());
        assert!(lib.has_xcb_xkb_list_components_keymaps_length());
        assert!(lib.has_xcb_xkb_list_components_keymaps_iterator());
        assert!(lib.has_xcb_xkb_list_components_keycodes_length());
        assert!(lib.has_xcb_xkb_list_components_keycodes_iterator());
        assert!(lib.has_xcb_xkb_list_components_types_length());
        assert!(lib.has_xcb_xkb_list_components_types_iterator());
        assert!(lib.has_xcb_xkb_list_components_compat_maps_length());
        assert!(lib.has_xcb_xkb_list_components_compat_maps_iterator());
        assert!(lib.has_xcb_xkb_list_components_symbols_length());
        assert!(lib.has_xcb_xkb_list_components_symbols_iterator());
        assert!(lib.has_xcb_xkb_list_components_geometries_length());
        assert!(lib.has_xcb_xkb_list_components_geometries_iterator());
        assert!(lib.has_xcb_xkb_list_components_reply());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map_types_rtrn_length());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map_types_rtrn_iterator());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map_syms_rtrn_length());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map_syms_rtrn_iterator());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count_length());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_count_end());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts_length());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map_acts_rtrn_acts_iterator());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn_length());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map_behaviors_rtrn_iterator());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn_length());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map_vmods_rtrn_end());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn_length());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map_explicit_rtrn_iterator());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn_length());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map_modmap_rtrn_iterator());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn_length());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map_vmodmap_rtrn_iterator());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map_serialize());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map_unpack());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map_sizeof());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names_length());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_type_names_end());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type());
        assert!(
            lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type_length()
        );
        assert!(
            lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_n_levels_per_type_end()
        );
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names());
        assert!(
            lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names_length()
        );
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_kt_level_names_end());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names());
        assert!(
            lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names_length()
        );
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_indicator_names_end());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names());
        assert!(
            lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names_length()
        );
        assert!(
            lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_virtual_mod_names_end()
        );
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups_length());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_groups_end());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names_length());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_names_iterator());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases_length());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_key_aliases_iterator());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names());
        assert!(
            lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names_length()
        );
        assert!(
            lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_radio_group_names_end()
        );
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_serialize());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_unpack());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list_sizeof());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_types_map());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn_length());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_compat_map_si_rtrn_iterator());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn_length());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_compat_map_group_rtrn_iterator());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps_length());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_indicator_maps_maps_iterator());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_key_names_value_list());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_geometry_label_font());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_serialize());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_unpack());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies_sizeof());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_sizeof());
        assert!(lib.has_xcb_xkb_get_kbd_by_name());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_unchecked());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_replies());
        assert!(lib.has_xcb_xkb_get_kbd_by_name_reply());
        assert!(lib.has_xcb_xkb_get_device_info_sizeof());
        assert!(lib.has_xcb_xkb_get_device_info());
        assert!(lib.has_xcb_xkb_get_device_info_unchecked());
        assert!(lib.has_xcb_xkb_get_device_info_name());
        assert!(lib.has_xcb_xkb_get_device_info_name_length());
        assert!(lib.has_xcb_xkb_get_device_info_name_end());
        assert!(lib.has_xcb_xkb_get_device_info_btn_actions());
        assert!(lib.has_xcb_xkb_get_device_info_btn_actions_length());
        assert!(lib.has_xcb_xkb_get_device_info_btn_actions_iterator());
        assert!(lib.has_xcb_xkb_get_device_info_leds_length());
        assert!(lib.has_xcb_xkb_get_device_info_leds_iterator());
        assert!(lib.has_xcb_xkb_get_device_info_reply());
        assert!(lib.has_xcb_xkb_set_device_info_sizeof());
        assert!(lib.has_xcb_xkb_set_device_info_checked());
        assert!(lib.has_xcb_xkb_set_device_info());
        assert!(lib.has_xcb_xkb_set_device_info_btn_actions());
        assert!(lib.has_xcb_xkb_set_device_info_btn_actions_length());
        assert!(lib.has_xcb_xkb_set_device_info_btn_actions_iterator());
        assert!(lib.has_xcb_xkb_set_device_info_leds_length());
        assert!(lib.has_xcb_xkb_set_device_info_leds_iterator());
        assert!(lib.has_xcb_xkb_set_debugging_flags_sizeof());
        assert!(lib.has_xcb_xkb_set_debugging_flags());
        assert!(lib.has_xcb_xkb_set_debugging_flags_unchecked());
        assert!(lib.has_xcb_xkb_set_debugging_flags_reply());
    }
}
