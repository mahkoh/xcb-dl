// This file was generated using generate.py. Do not edit.

use crate::ffi::*;
use crate::lazy::*;
use crate::*;
use std::os::raw::*;

pub type xcb_xkb_const_t = u32;
pub const XCB_XKB_CONST_MAX_LEGAL_KEY_CODE: xcb_xkb_const_t = 255;
pub const XCB_XKB_CONST_PER_KEY_BIT_ARRAY_SIZE: xcb_xkb_const_t = 32;
pub const XCB_XKB_CONST_KEY_NAME_LENGTH: xcb_xkb_const_t = 4;

pub type xcb_xkb_event_type_t = u32;
pub const XCB_XKB_EVENT_TYPE_NEW_KEYBOARD_NOTIFY: xcb_xkb_event_type_t = 1;
pub const XCB_XKB_EVENT_TYPE_MAP_NOTIFY: xcb_xkb_event_type_t = 2;
pub const XCB_XKB_EVENT_TYPE_STATE_NOTIFY: xcb_xkb_event_type_t = 4;
pub const XCB_XKB_EVENT_TYPE_CONTROLS_NOTIFY: xcb_xkb_event_type_t = 8;
pub const XCB_XKB_EVENT_TYPE_INDICATOR_STATE_NOTIFY: xcb_xkb_event_type_t = 16;
pub const XCB_XKB_EVENT_TYPE_INDICATOR_MAP_NOTIFY: xcb_xkb_event_type_t = 32;
pub const XCB_XKB_EVENT_TYPE_NAMES_NOTIFY: xcb_xkb_event_type_t = 64;
pub const XCB_XKB_EVENT_TYPE_COMPAT_MAP_NOTIFY: xcb_xkb_event_type_t = 128;
pub const XCB_XKB_EVENT_TYPE_BELL_NOTIFY: xcb_xkb_event_type_t = 256;
pub const XCB_XKB_EVENT_TYPE_ACTION_MESSAGE: xcb_xkb_event_type_t = 512;
pub const XCB_XKB_EVENT_TYPE_ACCESS_X_NOTIFY: xcb_xkb_event_type_t = 1024;
pub const XCB_XKB_EVENT_TYPE_EXTENSION_DEVICE_NOTIFY: xcb_xkb_event_type_t = 2048;

pub type xcb_xkb_nkn_detail_t = u32;
pub const XCB_XKB_NKN_DETAIL_KEYCODES: xcb_xkb_nkn_detail_t = 1;
pub const XCB_XKB_NKN_DETAIL_GEOMETRY: xcb_xkb_nkn_detail_t = 2;
pub const XCB_XKB_NKN_DETAIL_DEVICE_ID: xcb_xkb_nkn_detail_t = 4;

pub type xcb_xkb_axn_detail_t = u32;
pub const XCB_XKB_AXN_DETAIL_SK_PRESS: xcb_xkb_axn_detail_t = 1;
pub const XCB_XKB_AXN_DETAIL_SK_ACCEPT: xcb_xkb_axn_detail_t = 2;
pub const XCB_XKB_AXN_DETAIL_SK_REJECT: xcb_xkb_axn_detail_t = 4;
pub const XCB_XKB_AXN_DETAIL_SK_RELEASE: xcb_xkb_axn_detail_t = 8;
pub const XCB_XKB_AXN_DETAIL_BK_ACCEPT: xcb_xkb_axn_detail_t = 16;
pub const XCB_XKB_AXN_DETAIL_BK_REJECT: xcb_xkb_axn_detail_t = 32;
pub const XCB_XKB_AXN_DETAIL_AXK_WARNING: xcb_xkb_axn_detail_t = 64;

pub type xcb_xkb_map_part_t = u32;
pub const XCB_XKB_MAP_PART_KEY_TYPES: xcb_xkb_map_part_t = 1;
pub const XCB_XKB_MAP_PART_KEY_SYMS: xcb_xkb_map_part_t = 2;
pub const XCB_XKB_MAP_PART_MODIFIER_MAP: xcb_xkb_map_part_t = 4;
pub const XCB_XKB_MAP_PART_EXPLICIT_COMPONENTS: xcb_xkb_map_part_t = 8;
pub const XCB_XKB_MAP_PART_KEY_ACTIONS: xcb_xkb_map_part_t = 16;
pub const XCB_XKB_MAP_PART_KEY_BEHAVIORS: xcb_xkb_map_part_t = 32;
pub const XCB_XKB_MAP_PART_VIRTUAL_MODS: xcb_xkb_map_part_t = 64;
pub const XCB_XKB_MAP_PART_VIRTUAL_MOD_MAP: xcb_xkb_map_part_t = 128;

pub type xcb_xkb_set_map_flags_t = u32;
pub const XCB_XKB_SET_MAP_FLAGS_RESIZE_TYPES: xcb_xkb_set_map_flags_t = 1;
pub const XCB_XKB_SET_MAP_FLAGS_RECOMPUTE_ACTIONS: xcb_xkb_set_map_flags_t = 2;

pub type xcb_xkb_state_part_t = u32;
pub const XCB_XKB_STATE_PART_MODIFIER_STATE: xcb_xkb_state_part_t = 1;
pub const XCB_XKB_STATE_PART_MODIFIER_BASE: xcb_xkb_state_part_t = 2;
pub const XCB_XKB_STATE_PART_MODIFIER_LATCH: xcb_xkb_state_part_t = 4;
pub const XCB_XKB_STATE_PART_MODIFIER_LOCK: xcb_xkb_state_part_t = 8;
pub const XCB_XKB_STATE_PART_GROUP_STATE: xcb_xkb_state_part_t = 16;
pub const XCB_XKB_STATE_PART_GROUP_BASE: xcb_xkb_state_part_t = 32;
pub const XCB_XKB_STATE_PART_GROUP_LATCH: xcb_xkb_state_part_t = 64;
pub const XCB_XKB_STATE_PART_GROUP_LOCK: xcb_xkb_state_part_t = 128;
pub const XCB_XKB_STATE_PART_COMPAT_STATE: xcb_xkb_state_part_t = 256;
pub const XCB_XKB_STATE_PART_GRAB_MODS: xcb_xkb_state_part_t = 512;
pub const XCB_XKB_STATE_PART_COMPAT_GRAB_MODS: xcb_xkb_state_part_t = 1024;
pub const XCB_XKB_STATE_PART_LOOKUP_MODS: xcb_xkb_state_part_t = 2048;
pub const XCB_XKB_STATE_PART_COMPAT_LOOKUP_MODS: xcb_xkb_state_part_t = 4096;
pub const XCB_XKB_STATE_PART_POINTER_BUTTONS: xcb_xkb_state_part_t = 8192;

pub type xcb_xkb_bool_ctrl_t = u32;
pub const XCB_XKB_BOOL_CTRL_REPEAT_KEYS: xcb_xkb_bool_ctrl_t = 1;
pub const XCB_XKB_BOOL_CTRL_SLOW_KEYS: xcb_xkb_bool_ctrl_t = 2;
pub const XCB_XKB_BOOL_CTRL_BOUNCE_KEYS: xcb_xkb_bool_ctrl_t = 4;
pub const XCB_XKB_BOOL_CTRL_STICKY_KEYS: xcb_xkb_bool_ctrl_t = 8;
pub const XCB_XKB_BOOL_CTRL_MOUSE_KEYS: xcb_xkb_bool_ctrl_t = 16;
pub const XCB_XKB_BOOL_CTRL_MOUSE_KEYS_ACCEL: xcb_xkb_bool_ctrl_t = 32;
pub const XCB_XKB_BOOL_CTRL_ACCESS_X_KEYS: xcb_xkb_bool_ctrl_t = 64;
pub const XCB_XKB_BOOL_CTRL_ACCESS_X_TIMEOUT_MASK: xcb_xkb_bool_ctrl_t = 128;
pub const XCB_XKB_BOOL_CTRL_ACCESS_X_FEEDBACK_MASK: xcb_xkb_bool_ctrl_t = 256;
pub const XCB_XKB_BOOL_CTRL_AUDIBLE_BELL_MASK: xcb_xkb_bool_ctrl_t = 512;
pub const XCB_XKB_BOOL_CTRL_OVERLAY_1_MASK: xcb_xkb_bool_ctrl_t = 1024;
pub const XCB_XKB_BOOL_CTRL_OVERLAY_2_MASK: xcb_xkb_bool_ctrl_t = 2048;
pub const XCB_XKB_BOOL_CTRL_IGNORE_GROUP_LOCK_MASK: xcb_xkb_bool_ctrl_t = 4096;

pub type xcb_xkb_control_t = u32;
pub const XCB_XKB_CONTROL_GROUPS_WRAP: xcb_xkb_control_t = 134217728;
pub const XCB_XKB_CONTROL_INTERNAL_MODS: xcb_xkb_control_t = 268435456;
pub const XCB_XKB_CONTROL_IGNORE_LOCK_MODS: xcb_xkb_control_t = 536870912;
pub const XCB_XKB_CONTROL_PER_KEY_REPEAT: xcb_xkb_control_t = 1073741824;
pub const XCB_XKB_CONTROL_CONTROLS_ENABLED: xcb_xkb_control_t = 2147483648;

pub type xcb_xkb_ax_option_t = u32;
pub const XCB_XKB_AX_OPTION_SK_PRESS_FB: xcb_xkb_ax_option_t = 1;
pub const XCB_XKB_AX_OPTION_SK_ACCEPT_FB: xcb_xkb_ax_option_t = 2;
pub const XCB_XKB_AX_OPTION_FEATURE_FB: xcb_xkb_ax_option_t = 4;
pub const XCB_XKB_AX_OPTION_SLOW_WARN_FB: xcb_xkb_ax_option_t = 8;
pub const XCB_XKB_AX_OPTION_INDICATOR_FB: xcb_xkb_ax_option_t = 16;
pub const XCB_XKB_AX_OPTION_STICKY_KEYS_FB: xcb_xkb_ax_option_t = 32;
pub const XCB_XKB_AX_OPTION_TWO_KEYS: xcb_xkb_ax_option_t = 64;
pub const XCB_XKB_AX_OPTION_LATCH_TO_LOCK: xcb_xkb_ax_option_t = 128;
pub const XCB_XKB_AX_OPTION_SK_RELEASE_FB: xcb_xkb_ax_option_t = 256;
pub const XCB_XKB_AX_OPTION_SK_REJECT_FB: xcb_xkb_ax_option_t = 512;
pub const XCB_XKB_AX_OPTION_BK_REJECT_FB: xcb_xkb_ax_option_t = 1024;
pub const XCB_XKB_AX_OPTION_DUMB_BELL: xcb_xkb_ax_option_t = 2048;

pub type xcb_xkb_device_spec_t = u16;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_device_spec_iterator_t {
    pub data: *mut xcb_xkb_device_spec_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_device_spec_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_xkb_led_class_result_t = u32;
pub const XCB_XKB_LED_CLASS_RESULT_KBD_FEEDBACK_CLASS: xcb_xkb_led_class_result_t = 0;
pub const XCB_XKB_LED_CLASS_RESULT_LED_FEEDBACK_CLASS: xcb_xkb_led_class_result_t = 4;

pub type xcb_xkb_led_class_t = u32;
pub const XCB_XKB_LED_CLASS_KBD_FEEDBACK_CLASS: xcb_xkb_led_class_t = 0;
pub const XCB_XKB_LED_CLASS_LED_FEEDBACK_CLASS: xcb_xkb_led_class_t = 4;
pub const XCB_XKB_LED_CLASS_DFLT_XI_CLASS: xcb_xkb_led_class_t = 768;
pub const XCB_XKB_LED_CLASS_ALL_XI_CLASSES: xcb_xkb_led_class_t = 1280;

pub type xcb_xkb_led_class_spec_t = u16;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_led_class_spec_iterator_t {
    pub data: *mut xcb_xkb_led_class_spec_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_led_class_spec_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_xkb_bell_class_result_t = u32;
pub const XCB_XKB_BELL_CLASS_RESULT_KBD_FEEDBACK_CLASS: xcb_xkb_bell_class_result_t = 0;
pub const XCB_XKB_BELL_CLASS_RESULT_BELL_FEEDBACK_CLASS: xcb_xkb_bell_class_result_t = 5;

pub type xcb_xkb_bell_class_t = u32;
pub const XCB_XKB_BELL_CLASS_KBD_FEEDBACK_CLASS: xcb_xkb_bell_class_t = 0;
pub const XCB_XKB_BELL_CLASS_BELL_FEEDBACK_CLASS: xcb_xkb_bell_class_t = 5;
pub const XCB_XKB_BELL_CLASS_DFLT_XI_CLASS: xcb_xkb_bell_class_t = 768;

pub type xcb_xkb_bell_class_spec_t = u16;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_bell_class_spec_iterator_t {
    pub data: *mut xcb_xkb_bell_class_spec_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_bell_class_spec_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_xkb_id_t = u32;
pub const XCB_XKB_ID_USE_CORE_KBD: xcb_xkb_id_t = 256;
pub const XCB_XKB_ID_USE_CORE_PTR: xcb_xkb_id_t = 512;
pub const XCB_XKB_ID_DFLT_XI_CLASS: xcb_xkb_id_t = 768;
pub const XCB_XKB_ID_DFLT_XI_ID: xcb_xkb_id_t = 1024;
pub const XCB_XKB_ID_ALL_XI_CLASS: xcb_xkb_id_t = 1280;
pub const XCB_XKB_ID_ALL_XI_ID: xcb_xkb_id_t = 1536;
pub const XCB_XKB_ID_XI_NONE: xcb_xkb_id_t = 65280;

pub type xcb_xkb_id_spec_t = u16;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_id_spec_iterator_t {
    pub data: *mut xcb_xkb_id_spec_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_id_spec_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_xkb_group_t = u32;
pub const XCB_XKB_GROUP_1: xcb_xkb_group_t = 0;
pub const XCB_XKB_GROUP_2: xcb_xkb_group_t = 1;
pub const XCB_XKB_GROUP_3: xcb_xkb_group_t = 2;
pub const XCB_XKB_GROUP_4: xcb_xkb_group_t = 3;

pub type xcb_xkb_groups_t = u32;
pub const XCB_XKB_GROUPS_ANY: xcb_xkb_groups_t = 254;
pub const XCB_XKB_GROUPS_ALL: xcb_xkb_groups_t = 255;

pub type xcb_xkb_set_of_group_t = u32;
pub const XCB_XKB_SET_OF_GROUP_GROUP_1: xcb_xkb_set_of_group_t = 1;
pub const XCB_XKB_SET_OF_GROUP_GROUP_2: xcb_xkb_set_of_group_t = 2;
pub const XCB_XKB_SET_OF_GROUP_GROUP_3: xcb_xkb_set_of_group_t = 4;
pub const XCB_XKB_SET_OF_GROUP_GROUP_4: xcb_xkb_set_of_group_t = 8;

pub type xcb_xkb_set_of_groups_t = u32;
pub const XCB_XKB_SET_OF_GROUPS_ANY: xcb_xkb_set_of_groups_t = 128;

pub type xcb_xkb_groups_wrap_t = u32;
pub const XCB_XKB_GROUPS_WRAP_WRAP_INTO_RANGE: xcb_xkb_groups_wrap_t = 0;
pub const XCB_XKB_GROUPS_WRAP_CLAMP_INTO_RANGE: xcb_xkb_groups_wrap_t = 64;
pub const XCB_XKB_GROUPS_WRAP_REDIRECT_INTO_RANGE: xcb_xkb_groups_wrap_t = 128;

pub type xcb_xkb_v_mods_high_t = u32;
pub const XCB_XKB_V_MODS_HIGH_15: xcb_xkb_v_mods_high_t = 128;
pub const XCB_XKB_V_MODS_HIGH_14: xcb_xkb_v_mods_high_t = 64;
pub const XCB_XKB_V_MODS_HIGH_13: xcb_xkb_v_mods_high_t = 32;
pub const XCB_XKB_V_MODS_HIGH_12: xcb_xkb_v_mods_high_t = 16;
pub const XCB_XKB_V_MODS_HIGH_11: xcb_xkb_v_mods_high_t = 8;
pub const XCB_XKB_V_MODS_HIGH_10: xcb_xkb_v_mods_high_t = 4;
pub const XCB_XKB_V_MODS_HIGH_9: xcb_xkb_v_mods_high_t = 2;
pub const XCB_XKB_V_MODS_HIGH_8: xcb_xkb_v_mods_high_t = 1;

pub type xcb_xkb_v_mods_low_t = u32;
pub const XCB_XKB_V_MODS_LOW_7: xcb_xkb_v_mods_low_t = 128;
pub const XCB_XKB_V_MODS_LOW_6: xcb_xkb_v_mods_low_t = 64;
pub const XCB_XKB_V_MODS_LOW_5: xcb_xkb_v_mods_low_t = 32;
pub const XCB_XKB_V_MODS_LOW_4: xcb_xkb_v_mods_low_t = 16;
pub const XCB_XKB_V_MODS_LOW_3: xcb_xkb_v_mods_low_t = 8;
pub const XCB_XKB_V_MODS_LOW_2: xcb_xkb_v_mods_low_t = 4;
pub const XCB_XKB_V_MODS_LOW_1: xcb_xkb_v_mods_low_t = 2;
pub const XCB_XKB_V_MODS_LOW_0: xcb_xkb_v_mods_low_t = 1;

pub type xcb_xkb_v_mod_t = u32;
pub const XCB_XKB_V_MOD_15: xcb_xkb_v_mod_t = 32768;
pub const XCB_XKB_V_MOD_14: xcb_xkb_v_mod_t = 16384;
pub const XCB_XKB_V_MOD_13: xcb_xkb_v_mod_t = 8192;
pub const XCB_XKB_V_MOD_12: xcb_xkb_v_mod_t = 4096;
pub const XCB_XKB_V_MOD_11: xcb_xkb_v_mod_t = 2048;
pub const XCB_XKB_V_MOD_10: xcb_xkb_v_mod_t = 1024;
pub const XCB_XKB_V_MOD_9: xcb_xkb_v_mod_t = 512;
pub const XCB_XKB_V_MOD_8: xcb_xkb_v_mod_t = 256;
pub const XCB_XKB_V_MOD_7: xcb_xkb_v_mod_t = 128;
pub const XCB_XKB_V_MOD_6: xcb_xkb_v_mod_t = 64;
pub const XCB_XKB_V_MOD_5: xcb_xkb_v_mod_t = 32;
pub const XCB_XKB_V_MOD_4: xcb_xkb_v_mod_t = 16;
pub const XCB_XKB_V_MOD_3: xcb_xkb_v_mod_t = 8;
pub const XCB_XKB_V_MOD_2: xcb_xkb_v_mod_t = 4;
pub const XCB_XKB_V_MOD_1: xcb_xkb_v_mod_t = 2;
pub const XCB_XKB_V_MOD_0: xcb_xkb_v_mod_t = 1;

pub type xcb_xkb_explicit_t = u32;
pub const XCB_XKB_EXPLICIT_V_MOD_MAP: xcb_xkb_explicit_t = 128;
pub const XCB_XKB_EXPLICIT_BEHAVIOR: xcb_xkb_explicit_t = 64;
pub const XCB_XKB_EXPLICIT_AUTO_REPEAT: xcb_xkb_explicit_t = 32;
pub const XCB_XKB_EXPLICIT_INTERPRET: xcb_xkb_explicit_t = 16;
pub const XCB_XKB_EXPLICIT_KEY_TYPE_4: xcb_xkb_explicit_t = 8;
pub const XCB_XKB_EXPLICIT_KEY_TYPE_3: xcb_xkb_explicit_t = 4;
pub const XCB_XKB_EXPLICIT_KEY_TYPE_2: xcb_xkb_explicit_t = 2;
pub const XCB_XKB_EXPLICIT_KEY_TYPE_1: xcb_xkb_explicit_t = 1;

pub type xcb_xkb_sym_interpret_match_t = u32;
pub const XCB_XKB_SYM_INTERPRET_MATCH_NONE_OF: xcb_xkb_sym_interpret_match_t = 0;
pub const XCB_XKB_SYM_INTERPRET_MATCH_ANY_OF_OR_NONE: xcb_xkb_sym_interpret_match_t = 1;
pub const XCB_XKB_SYM_INTERPRET_MATCH_ANY_OF: xcb_xkb_sym_interpret_match_t = 2;
pub const XCB_XKB_SYM_INTERPRET_MATCH_ALL_OF: xcb_xkb_sym_interpret_match_t = 3;
pub const XCB_XKB_SYM_INTERPRET_MATCH_EXACTLY: xcb_xkb_sym_interpret_match_t = 4;

pub type xcb_xkb_sym_interp_match_t = u32;
pub const XCB_XKB_SYM_INTERP_MATCH_LEVEL_ONE_ONLY: xcb_xkb_sym_interp_match_t = 128;
pub const XCB_XKB_SYM_INTERP_MATCH_OP_MASK: xcb_xkb_sym_interp_match_t = 127;

pub type xcb_xkb_im_flag_t = u32;
pub const XCB_XKB_IM_FLAG_NO_EXPLICIT: xcb_xkb_im_flag_t = 128;
pub const XCB_XKB_IM_FLAG_NO_AUTOMATIC: xcb_xkb_im_flag_t = 64;
pub const XCB_XKB_IM_FLAG_LED_DRIVES_KB: xcb_xkb_im_flag_t = 32;

pub type xcb_xkb_im_mods_which_t = u32;
pub const XCB_XKB_IM_MODS_WHICH_USE_COMPAT: xcb_xkb_im_mods_which_t = 16;
pub const XCB_XKB_IM_MODS_WHICH_USE_EFFECTIVE: xcb_xkb_im_mods_which_t = 8;
pub const XCB_XKB_IM_MODS_WHICH_USE_LOCKED: xcb_xkb_im_mods_which_t = 4;
pub const XCB_XKB_IM_MODS_WHICH_USE_LATCHED: xcb_xkb_im_mods_which_t = 2;
pub const XCB_XKB_IM_MODS_WHICH_USE_BASE: xcb_xkb_im_mods_which_t = 1;

pub type xcb_xkb_im_groups_which_t = u32;
pub const XCB_XKB_IM_GROUPS_WHICH_USE_COMPAT: xcb_xkb_im_groups_which_t = 16;
pub const XCB_XKB_IM_GROUPS_WHICH_USE_EFFECTIVE: xcb_xkb_im_groups_which_t = 8;
pub const XCB_XKB_IM_GROUPS_WHICH_USE_LOCKED: xcb_xkb_im_groups_which_t = 4;
pub const XCB_XKB_IM_GROUPS_WHICH_USE_LATCHED: xcb_xkb_im_groups_which_t = 2;
pub const XCB_XKB_IM_GROUPS_WHICH_USE_BASE: xcb_xkb_im_groups_which_t = 1;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_indicator_map_iterator_t {
    pub data: *mut xcb_xkb_indicator_map_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_indicator_map_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_xkb_cm_detail_t = u32;
pub const XCB_XKB_CM_DETAIL_SYM_INTERP: xcb_xkb_cm_detail_t = 1;
pub const XCB_XKB_CM_DETAIL_GROUP_COMPAT: xcb_xkb_cm_detail_t = 2;

pub type xcb_xkb_name_detail_t = u32;
pub const XCB_XKB_NAME_DETAIL_KEYCODES: xcb_xkb_name_detail_t = 1;
pub const XCB_XKB_NAME_DETAIL_GEOMETRY: xcb_xkb_name_detail_t = 2;
pub const XCB_XKB_NAME_DETAIL_SYMBOLS: xcb_xkb_name_detail_t = 4;
pub const XCB_XKB_NAME_DETAIL_PHYS_SYMBOLS: xcb_xkb_name_detail_t = 8;
pub const XCB_XKB_NAME_DETAIL_TYPES: xcb_xkb_name_detail_t = 16;
pub const XCB_XKB_NAME_DETAIL_COMPAT: xcb_xkb_name_detail_t = 32;
pub const XCB_XKB_NAME_DETAIL_KEY_TYPE_NAMES: xcb_xkb_name_detail_t = 64;
pub const XCB_XKB_NAME_DETAIL_KT_LEVEL_NAMES: xcb_xkb_name_detail_t = 128;
pub const XCB_XKB_NAME_DETAIL_INDICATOR_NAMES: xcb_xkb_name_detail_t = 256;
pub const XCB_XKB_NAME_DETAIL_KEY_NAMES: xcb_xkb_name_detail_t = 512;
pub const XCB_XKB_NAME_DETAIL_KEY_ALIASES: xcb_xkb_name_detail_t = 1024;
pub const XCB_XKB_NAME_DETAIL_VIRTUAL_MOD_NAMES: xcb_xkb_name_detail_t = 2048;
pub const XCB_XKB_NAME_DETAIL_GROUP_NAMES: xcb_xkb_name_detail_t = 4096;
pub const XCB_XKB_NAME_DETAIL_RG_NAMES: xcb_xkb_name_detail_t = 8192;

pub type xcb_xkb_gbn_detail_t = u32;
pub const XCB_XKB_GBN_DETAIL_TYPES: xcb_xkb_gbn_detail_t = 1;
pub const XCB_XKB_GBN_DETAIL_COMPAT_MAP: xcb_xkb_gbn_detail_t = 2;
pub const XCB_XKB_GBN_DETAIL_CLIENT_SYMBOLS: xcb_xkb_gbn_detail_t = 4;
pub const XCB_XKB_GBN_DETAIL_SERVER_SYMBOLS: xcb_xkb_gbn_detail_t = 8;
pub const XCB_XKB_GBN_DETAIL_INDICATOR_MAPS: xcb_xkb_gbn_detail_t = 16;
pub const XCB_XKB_GBN_DETAIL_KEY_NAMES: xcb_xkb_gbn_detail_t = 32;
pub const XCB_XKB_GBN_DETAIL_GEOMETRY: xcb_xkb_gbn_detail_t = 64;
pub const XCB_XKB_GBN_DETAIL_OTHER_NAMES: xcb_xkb_gbn_detail_t = 128;

pub type xcb_xkb_xi_feature_t = u32;
pub const XCB_XKB_XI_FEATURE_KEYBOARDS: xcb_xkb_xi_feature_t = 1;
pub const XCB_XKB_XI_FEATURE_BUTTON_ACTIONS: xcb_xkb_xi_feature_t = 2;
pub const XCB_XKB_XI_FEATURE_INDICATOR_NAMES: xcb_xkb_xi_feature_t = 4;
pub const XCB_XKB_XI_FEATURE_INDICATOR_MAPS: xcb_xkb_xi_feature_t = 8;
pub const XCB_XKB_XI_FEATURE_INDICATOR_STATE: xcb_xkb_xi_feature_t = 16;

pub type xcb_xkb_per_client_flag_t = u32;
pub const XCB_XKB_PER_CLIENT_FLAG_DETECTABLE_AUTO_REPEAT: xcb_xkb_per_client_flag_t = 1;
pub const XCB_XKB_PER_CLIENT_FLAG_GRABS_USE_XKB_STATE: xcb_xkb_per_client_flag_t = 2;
pub const XCB_XKB_PER_CLIENT_FLAG_AUTO_RESET_CONTROLS: xcb_xkb_per_client_flag_t = 4;
pub const XCB_XKB_PER_CLIENT_FLAG_LOOKUP_STATE_WHEN_GRABBED: xcb_xkb_per_client_flag_t = 8;
pub const XCB_XKB_PER_CLIENT_FLAG_SEND_EVENT_USES_XKB_STATE: xcb_xkb_per_client_flag_t = 16;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_mod_def_iterator_t {
    pub data: *mut xcb_xkb_mod_def_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_mod_def_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_key_name_iterator_t {
    pub data: *mut xcb_xkb_key_name_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_key_name_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_key_alias_iterator_t {
    pub data: *mut xcb_xkb_key_alias_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_key_alias_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_counted_string_16_iterator_t {
    pub data: *mut xcb_xkb_counted_string_16_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_counted_string_16_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_kt_map_entry_iterator_t {
    pub data: *mut xcb_xkb_kt_map_entry_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_kt_map_entry_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_key_type_iterator_t {
    pub data: *mut xcb_xkb_key_type_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_key_type_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_key_sym_map_iterator_t {
    pub data: *mut xcb_xkb_key_sym_map_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_key_sym_map_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_common_behavior_iterator_t {
    pub data: *mut xcb_xkb_common_behavior_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_common_behavior_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_default_behavior_iterator_t {
    pub data: *mut xcb_xkb_default_behavior_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_default_behavior_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_lock_behavior_iterator_t {
    pub data: *mut xcb_xkb_lock_behavior_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_lock_behavior_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_radio_group_behavior_iterator_t {
    pub data: *mut xcb_xkb_radio_group_behavior_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_radio_group_behavior_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_overlay_behavior_iterator_t {
    pub data: *mut xcb_xkb_overlay_behavior_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_overlay_behavior_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_permament_lock_behavior_iterator_t {
    pub data: *mut xcb_xkb_permament_lock_behavior_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_permament_lock_behavior_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_permament_radio_group_behavior_iterator_t {
    pub data: *mut xcb_xkb_permament_radio_group_behavior_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_permament_radio_group_behavior_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_permament_overlay_behavior_iterator_t {
    pub data: *mut xcb_xkb_permament_overlay_behavior_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_permament_overlay_behavior_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_behavior_iterator_t {
    pub data: *mut xcb_xkb_behavior_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_behavior_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_xkb_behavior_type_t = u32;
pub const XCB_XKB_BEHAVIOR_TYPE_DEFAULT: xcb_xkb_behavior_type_t = 0;
pub const XCB_XKB_BEHAVIOR_TYPE_LOCK: xcb_xkb_behavior_type_t = 1;
pub const XCB_XKB_BEHAVIOR_TYPE_RADIO_GROUP: xcb_xkb_behavior_type_t = 2;
pub const XCB_XKB_BEHAVIOR_TYPE_OVERLAY_1: xcb_xkb_behavior_type_t = 3;
pub const XCB_XKB_BEHAVIOR_TYPE_OVERLAY_2: xcb_xkb_behavior_type_t = 4;
pub const XCB_XKB_BEHAVIOR_TYPE_PERMAMENT_LOCK: xcb_xkb_behavior_type_t = 129;
pub const XCB_XKB_BEHAVIOR_TYPE_PERMAMENT_RADIO_GROUP: xcb_xkb_behavior_type_t = 130;
pub const XCB_XKB_BEHAVIOR_TYPE_PERMAMENT_OVERLAY_1: xcb_xkb_behavior_type_t = 131;
pub const XCB_XKB_BEHAVIOR_TYPE_PERMAMENT_OVERLAY_2: xcb_xkb_behavior_type_t = 132;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_behavior_iterator_t {
    pub data: *mut xcb_xkb_set_behavior_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_set_behavior_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_explicit_iterator_t {
    pub data: *mut xcb_xkb_set_explicit_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_set_explicit_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_key_mod_map_iterator_t {
    pub data: *mut xcb_xkb_key_mod_map_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_key_mod_map_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_key_v_mod_map_iterator_t {
    pub data: *mut xcb_xkb_key_v_mod_map_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_key_v_mod_map_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_kt_set_map_entry_iterator_t {
    pub data: *mut xcb_xkb_kt_set_map_entry_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_kt_set_map_entry_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_key_type_iterator_t {
    pub data: *mut xcb_xkb_set_key_type_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_set_key_type_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_xkb_string8_t = c_char;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_string8_iterator_t {
    pub data: *mut xcb_xkb_string8_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_string8_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_outline_iterator_t {
    pub data: *mut xcb_xkb_outline_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_outline_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_shape_iterator_t {
    pub data: *mut xcb_xkb_shape_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_shape_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_key_iterator_t {
    pub data: *mut xcb_xkb_key_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_key_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_overlay_key_iterator_t {
    pub data: *mut xcb_xkb_overlay_key_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_overlay_key_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_overlay_row_iterator_t {
    pub data: *mut xcb_xkb_overlay_row_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_overlay_row_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_overlay_iterator_t {
    pub data: *mut xcb_xkb_overlay_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_overlay_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_row_iterator_t {
    pub data: *mut xcb_xkb_row_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_row_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_xkb_doodad_type_t = u32;
pub const XCB_XKB_DOODAD_TYPE_OUTLINE: xcb_xkb_doodad_type_t = 1;
pub const XCB_XKB_DOODAD_TYPE_SOLID: xcb_xkb_doodad_type_t = 2;
pub const XCB_XKB_DOODAD_TYPE_TEXT: xcb_xkb_doodad_type_t = 3;
pub const XCB_XKB_DOODAD_TYPE_INDICATOR: xcb_xkb_doodad_type_t = 4;
pub const XCB_XKB_DOODAD_TYPE_LOGO: xcb_xkb_doodad_type_t = 5;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_listing_iterator_t {
    pub data: *mut xcb_xkb_listing_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_listing_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_device_led_info_t {
    pub led_class: xcb_xkb_led_class_spec_t,
    pub led_i_d: xcb_xkb_id_spec_t,
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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_device_led_info_iterator_t {
    pub data: *mut xcb_xkb_device_led_info_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_device_led_info_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_xkb_error_t = u32;
pub const XCB_XKB_ERROR_BAD_DEVICE: xcb_xkb_error_t = 255;
pub const XCB_XKB_ERROR_BAD_CLASS: xcb_xkb_error_t = 254;
pub const XCB_XKB_ERROR_BAD_ID: xcb_xkb_error_t = 253;

/// Opcode for xcb_xkb_keyboard.
pub const XCB_XKB_KEYBOARD: u8 = 0i32 as u8;

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

pub type xcb_xkb_sa_t = u32;
pub const XCB_XKB_SA_CLEAR_LOCKS: xcb_xkb_sa_t = 1;
pub const XCB_XKB_SA_LATCH_TO_LOCK: xcb_xkb_sa_t = 2;
pub const XCB_XKB_SA_USE_MOD_MAP_MODS: xcb_xkb_sa_t = 4;
pub const XCB_XKB_SA_GROUP_ABSOLUTE: xcb_xkb_sa_t = 4;

pub type xcb_xkb_sa_type_t = u32;
pub const XCB_XKB_SA_TYPE_NO_ACTION: xcb_xkb_sa_type_t = 0;
pub const XCB_XKB_SA_TYPE_SET_MODS: xcb_xkb_sa_type_t = 1;
pub const XCB_XKB_SA_TYPE_LATCH_MODS: xcb_xkb_sa_type_t = 2;
pub const XCB_XKB_SA_TYPE_LOCK_MODS: xcb_xkb_sa_type_t = 3;
pub const XCB_XKB_SA_TYPE_SET_GROUP: xcb_xkb_sa_type_t = 4;
pub const XCB_XKB_SA_TYPE_LATCH_GROUP: xcb_xkb_sa_type_t = 5;
pub const XCB_XKB_SA_TYPE_LOCK_GROUP: xcb_xkb_sa_type_t = 6;
pub const XCB_XKB_SA_TYPE_MOVE_PTR: xcb_xkb_sa_type_t = 7;
pub const XCB_XKB_SA_TYPE_PTR_BTN: xcb_xkb_sa_type_t = 8;
pub const XCB_XKB_SA_TYPE_LOCK_PTR_BTN: xcb_xkb_sa_type_t = 9;
pub const XCB_XKB_SA_TYPE_SET_PTR_DFLT: xcb_xkb_sa_type_t = 10;
pub const XCB_XKB_SA_TYPE_ISO_LOCK: xcb_xkb_sa_type_t = 11;
pub const XCB_XKB_SA_TYPE_TERMINATE: xcb_xkb_sa_type_t = 12;
pub const XCB_XKB_SA_TYPE_SWITCH_SCREEN: xcb_xkb_sa_type_t = 13;
pub const XCB_XKB_SA_TYPE_SET_CONTROLS: xcb_xkb_sa_type_t = 14;
pub const XCB_XKB_SA_TYPE_LOCK_CONTROLS: xcb_xkb_sa_type_t = 15;
pub const XCB_XKB_SA_TYPE_ACTION_MESSAGE: xcb_xkb_sa_type_t = 16;
pub const XCB_XKB_SA_TYPE_REDIRECT_KEY: xcb_xkb_sa_type_t = 17;
pub const XCB_XKB_SA_TYPE_DEVICE_BTN: xcb_xkb_sa_type_t = 18;
pub const XCB_XKB_SA_TYPE_LOCK_DEVICE_BTN: xcb_xkb_sa_type_t = 19;
pub const XCB_XKB_SA_TYPE_DEVICE_VALUATOR: xcb_xkb_sa_type_t = 20;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_no_action_iterator_t {
    pub data: *mut xcb_xkb_sa_no_action_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_sa_no_action_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_set_mods_iterator_t {
    pub data: *mut xcb_xkb_sa_set_mods_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_sa_set_mods_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_latch_mods_iterator_t {
    pub data: *mut xcb_xkb_sa_latch_mods_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_sa_latch_mods_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_lock_mods_iterator_t {
    pub data: *mut xcb_xkb_sa_lock_mods_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_sa_lock_mods_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_set_group_iterator_t {
    pub data: *mut xcb_xkb_sa_set_group_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_sa_set_group_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_latch_group_iterator_t {
    pub data: *mut xcb_xkb_sa_latch_group_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_sa_latch_group_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_lock_group_iterator_t {
    pub data: *mut xcb_xkb_sa_lock_group_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_sa_lock_group_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_xkb_sa_move_ptr_flag_t = u32;
pub const XCB_XKB_SA_MOVE_PTR_FLAG_NO_ACCELERATION: xcb_xkb_sa_move_ptr_flag_t = 1;
pub const XCB_XKB_SA_MOVE_PTR_FLAG_MOVE_ABSOLUTE_X: xcb_xkb_sa_move_ptr_flag_t = 2;
pub const XCB_XKB_SA_MOVE_PTR_FLAG_MOVE_ABSOLUTE_Y: xcb_xkb_sa_move_ptr_flag_t = 4;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_move_ptr_iterator_t {
    pub data: *mut xcb_xkb_sa_move_ptr_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_sa_move_ptr_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_ptr_btn_iterator_t {
    pub data: *mut xcb_xkb_sa_ptr_btn_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_sa_ptr_btn_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_lock_ptr_btn_iterator_t {
    pub data: *mut xcb_xkb_sa_lock_ptr_btn_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_sa_lock_ptr_btn_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_xkb_sa_set_ptr_dflt_flag_t = u32;
pub const XCB_XKB_SA_SET_PTR_DFLT_FLAG_DFLT_BTN_ABSOLUTE: xcb_xkb_sa_set_ptr_dflt_flag_t = 4;
pub const XCB_XKB_SA_SET_PTR_DFLT_FLAG_AFFECT_DFLT_BUTTON: xcb_xkb_sa_set_ptr_dflt_flag_t = 1;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_set_ptr_dflt_iterator_t {
    pub data: *mut xcb_xkb_sa_set_ptr_dflt_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_sa_set_ptr_dflt_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_xkb_sa_iso_lock_flag_t = u32;
pub const XCB_XKB_SA_ISO_LOCK_FLAG_NO_LOCK: xcb_xkb_sa_iso_lock_flag_t = 1;
pub const XCB_XKB_SA_ISO_LOCK_FLAG_NO_UNLOCK: xcb_xkb_sa_iso_lock_flag_t = 2;
pub const XCB_XKB_SA_ISO_LOCK_FLAG_USE_MOD_MAP_MODS: xcb_xkb_sa_iso_lock_flag_t = 4;
pub const XCB_XKB_SA_ISO_LOCK_FLAG_GROUP_ABSOLUTE: xcb_xkb_sa_iso_lock_flag_t = 4;
pub const XCB_XKB_SA_ISO_LOCK_FLAG_ISO_DFLT_IS_GROUP: xcb_xkb_sa_iso_lock_flag_t = 8;

pub type xcb_xkb_sa_iso_lock_no_affect_t = u32;
pub const XCB_XKB_SA_ISO_LOCK_NO_AFFECT_CTRLS: xcb_xkb_sa_iso_lock_no_affect_t = 8;
pub const XCB_XKB_SA_ISO_LOCK_NO_AFFECT_PTR: xcb_xkb_sa_iso_lock_no_affect_t = 16;
pub const XCB_XKB_SA_ISO_LOCK_NO_AFFECT_GROUP: xcb_xkb_sa_iso_lock_no_affect_t = 32;
pub const XCB_XKB_SA_ISO_LOCK_NO_AFFECT_MODS: xcb_xkb_sa_iso_lock_no_affect_t = 64;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_iso_lock_iterator_t {
    pub data: *mut xcb_xkb_sa_iso_lock_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_sa_iso_lock_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_terminate_iterator_t {
    pub data: *mut xcb_xkb_sa_terminate_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_sa_terminate_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_xkb_switch_screen_flag_t = u32;
pub const XCB_XKB_SWITCH_SCREEN_FLAG_APPLICATION: xcb_xkb_switch_screen_flag_t = 1;
pub const XCB_XKB_SWITCH_SCREEN_FLAG_ABSOLUTE: xcb_xkb_switch_screen_flag_t = 4;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_switch_screen_iterator_t {
    pub data: *mut xcb_xkb_sa_switch_screen_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_sa_switch_screen_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_xkb_bool_ctrls_high_t = u32;
pub const XCB_XKB_BOOL_CTRLS_HIGH_ACCESS_X_FEEDBACK: xcb_xkb_bool_ctrls_high_t = 1;
pub const XCB_XKB_BOOL_CTRLS_HIGH_AUDIBLE_BELL: xcb_xkb_bool_ctrls_high_t = 2;
pub const XCB_XKB_BOOL_CTRLS_HIGH_OVERLAY_1: xcb_xkb_bool_ctrls_high_t = 4;
pub const XCB_XKB_BOOL_CTRLS_HIGH_OVERLAY_2: xcb_xkb_bool_ctrls_high_t = 8;
pub const XCB_XKB_BOOL_CTRLS_HIGH_IGNORE_GROUP_LOCK: xcb_xkb_bool_ctrls_high_t = 16;

pub type xcb_xkb_bool_ctrls_low_t = u32;
pub const XCB_XKB_BOOL_CTRLS_LOW_REPEAT_KEYS: xcb_xkb_bool_ctrls_low_t = 1;
pub const XCB_XKB_BOOL_CTRLS_LOW_SLOW_KEYS: xcb_xkb_bool_ctrls_low_t = 2;
pub const XCB_XKB_BOOL_CTRLS_LOW_BOUNCE_KEYS: xcb_xkb_bool_ctrls_low_t = 4;
pub const XCB_XKB_BOOL_CTRLS_LOW_STICKY_KEYS: xcb_xkb_bool_ctrls_low_t = 8;
pub const XCB_XKB_BOOL_CTRLS_LOW_MOUSE_KEYS: xcb_xkb_bool_ctrls_low_t = 16;
pub const XCB_XKB_BOOL_CTRLS_LOW_MOUSE_KEYS_ACCEL: xcb_xkb_bool_ctrls_low_t = 32;
pub const XCB_XKB_BOOL_CTRLS_LOW_ACCESS_X_KEYS: xcb_xkb_bool_ctrls_low_t = 64;
pub const XCB_XKB_BOOL_CTRLS_LOW_ACCESS_X_TIMEOUT: xcb_xkb_bool_ctrls_low_t = 128;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_set_controls_iterator_t {
    pub data: *mut xcb_xkb_sa_set_controls_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_sa_set_controls_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_lock_controls_iterator_t {
    pub data: *mut xcb_xkb_sa_lock_controls_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_sa_lock_controls_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_xkb_action_message_flag_t = u32;
pub const XCB_XKB_ACTION_MESSAGE_FLAG_ON_PRESS: xcb_xkb_action_message_flag_t = 1;
pub const XCB_XKB_ACTION_MESSAGE_FLAG_ON_RELEASE: xcb_xkb_action_message_flag_t = 2;
pub const XCB_XKB_ACTION_MESSAGE_FLAG_GEN_KEY_EVENT: xcb_xkb_action_message_flag_t = 4;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_action_message_iterator_t {
    pub data: *mut xcb_xkb_sa_action_message_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_sa_action_message_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_redirect_key_iterator_t {
    pub data: *mut xcb_xkb_sa_redirect_key_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_sa_redirect_key_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_device_btn_iterator_t {
    pub data: *mut xcb_xkb_sa_device_btn_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_sa_device_btn_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_xkb_lock_device_flags_t = u32;
pub const XCB_XKB_LOCK_DEVICE_FLAGS_NO_LOCK: xcb_xkb_lock_device_flags_t = 1;
pub const XCB_XKB_LOCK_DEVICE_FLAGS_NO_UNLOCK: xcb_xkb_lock_device_flags_t = 2;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_lock_device_btn_iterator_t {
    pub data: *mut xcb_xkb_sa_lock_device_btn_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_sa_lock_device_btn_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

pub type xcb_xkb_sa_val_what_t = u32;
pub const XCB_XKB_SA_VAL_WHAT_IGNORE_VAL: xcb_xkb_sa_val_what_t = 0;
pub const XCB_XKB_SA_VAL_WHAT_SET_VAL_MIN: xcb_xkb_sa_val_what_t = 1;
pub const XCB_XKB_SA_VAL_WHAT_SET_VAL_CENTER: xcb_xkb_sa_val_what_t = 2;
pub const XCB_XKB_SA_VAL_WHAT_SET_VAL_MAX: xcb_xkb_sa_val_what_t = 3;
pub const XCB_XKB_SA_VAL_WHAT_SET_VAL_RELATIVE: xcb_xkb_sa_val_what_t = 4;
pub const XCB_XKB_SA_VAL_WHAT_SET_VAL_ABSOLUTE: xcb_xkb_sa_val_what_t = 5;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sa_device_valuator_iterator_t {
    pub data: *mut xcb_xkb_sa_device_valuator_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_sa_device_valuator_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_si_action_iterator_t {
    pub data: *mut xcb_xkb_si_action_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_si_action_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_sym_interpret_iterator_t {
    pub data: *mut xcb_xkb_sym_interpret_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_sym_interpret_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_action_iterator_t {
    pub data: *mut xcb_xkb_action_t,
    pub rem: c_int,
    pub index: c_int,
}

impl Default for xcb_xkb_action_iterator_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_use_extension_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xkb_use_extension_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xkb_use_extension.
pub const XCB_XKB_USE_EXTENSION: u8 = 0i32 as u8;

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
    pub access_x_details: u16,
    pub affect_ext_dev: u16,
    pub extdev_details: u16,
}

impl Default for xcb_xkb_select_events_details_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xkb_select_events.
pub const XCB_XKB_SELECT_EVENTS: u8 = 1i32 as u8;

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

/// Opcode for xcb_xkb_bell.
pub const XCB_XKB_BELL: u8 = 3i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_bell_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub bell_class: xcb_xkb_bell_class_spec_t,
    pub bell_i_d: xcb_xkb_id_spec_t,
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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_state_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xkb_get_state_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xkb_get_state.
pub const XCB_XKB_GET_STATE: u8 = 4i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_state_reply_t {
    pub response_type: u8,
    pub device_i_d: u8,
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

/// Opcode for xcb_xkb_latch_lock_state.
pub const XCB_XKB_LATCH_LOCK_STATE: u8 = 5i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_controls_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xkb_get_controls_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xkb_get_controls.
pub const XCB_XKB_GET_CONTROLS: u8 = 6i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_controls_reply_t {
    pub response_type: u8,
    pub device_i_d: u8,
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
    pub access_x_option: u16,
    pub access_x_timeout: u16,
    pub access_x_timeout_options_mask: u16,
    pub access_x_timeout_options_values: u16,
    pub pad1: [u8; 2],
    pub access_x_timeout_mask: u32,
    pub access_x_timeout_values: u32,
    pub enabled_controls: u32,
    pub per_key_repeat: [u8; 32],
}

impl Default for xcb_xkb_get_controls_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xkb_set_controls.
pub const XCB_XKB_SET_CONTROLS: u8 = 7i32 as u8;

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
    pub access_x_options: u16,
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
    pub access_x_timeout: u16,
    pub access_x_timeout_mask: u32,
    pub access_x_timeout_values: u32,
    pub access_x_timeout_options_mask: u16,
    pub access_x_timeout_options_values: u16,
    pub per_key_repeat: [u8; 32],
}

impl Default for xcb_xkb_set_controls_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_map_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xkb_get_map_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xkb_get_map.
pub const XCB_XKB_GET_MAP: u8 = 8i32 as u8;

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
    pub first_v_mod_map_key: xcb_keycode_t,
    pub n_v_mod_map_keys: u8,
    pub pad0: [u8; 2],
}

impl Default for xcb_xkb_get_map_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_map_reply_t {
    pub response_type: u8,
    pub device_i_d: u8,
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
    pub first_v_mod_map_key: xcb_keycode_t,
    pub n_v_mod_map_keys: u8,
    pub total_v_mod_map_keys: u8,
    pub pad1: u8,
    pub virtual_mods: u16,
}

impl Default for xcb_xkb_get_map_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

/// Opcode for xcb_xkb_set_map.
pub const XCB_XKB_SET_MAP: u8 = 9i32 as u8;

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
    pub first_v_mod_map_key: xcb_keycode_t,
    pub n_v_mod_map_keys: u8,
    pub total_v_mod_map_keys: u8,
    pub virtual_mods: u16,
}

impl Default for xcb_xkb_set_map_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_compat_map_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xkb_get_compat_map_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xkb_get_compat_map.
pub const XCB_XKB_GET_COMPAT_MAP: u8 = 10i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_compat_map_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub groups: u8,
    pub get_all_s_i: u8,
    pub first_s_i: u16,
    pub n_s_i: u16,
}

impl Default for xcb_xkb_get_compat_map_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_compat_map_reply_t {
    pub response_type: u8,
    pub device_i_d: u8,
    pub sequence: u16,
    pub length: u32,
    pub groups_rtrn: u8,
    pub pad0: u8,
    pub first_s_i_rtrn: u16,
    pub n_s_i_rtrn: u16,
    pub n_total_s_i: u16,
    pub pad1: [u8; 16],
}

impl Default for xcb_xkb_get_compat_map_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xkb_set_compat_map.
pub const XCB_XKB_SET_COMPAT_MAP: u8 = 11i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_compat_map_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub pad0: u8,
    pub recompute_actions: u8,
    pub truncate_s_i: u8,
    pub groups: u8,
    pub first_s_i: u16,
    pub n_s_i: u16,
    pub pad1: [u8; 2],
}

impl Default for xcb_xkb_set_compat_map_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_indicator_state_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xkb_get_indicator_state_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xkb_get_indicator_state.
pub const XCB_XKB_GET_INDICATOR_STATE: u8 = 12i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_indicator_state_reply_t {
    pub response_type: u8,
    pub device_i_d: u8,
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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_indicator_map_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xkb_get_indicator_map_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xkb_get_indicator_map.
pub const XCB_XKB_GET_INDICATOR_MAP: u8 = 13i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_indicator_map_reply_t {
    pub response_type: u8,
    pub device_i_d: u8,
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

/// Opcode for xcb_xkb_set_indicator_map.
pub const XCB_XKB_SET_INDICATOR_MAP: u8 = 14i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_named_indicator_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xkb_get_named_indicator_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xkb_get_named_indicator.
pub const XCB_XKB_GET_NAMED_INDICATOR: u8 = 15i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_named_indicator_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub led_class: xcb_xkb_led_class_spec_t,
    pub led_i_d: xcb_xkb_id_spec_t,
    pub pad0: [u8; 2],
    pub indicator: xcb_atom_t,
}

impl Default for xcb_xkb_get_named_indicator_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_named_indicator_reply_t {
    pub response_type: u8,
    pub device_i_d: u8,
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

/// Opcode for xcb_xkb_set_named_indicator.
pub const XCB_XKB_SET_NAMED_INDICATOR: u8 = 16i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_named_indicator_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub device_spec: xcb_xkb_device_spec_t,
    pub led_class: xcb_xkb_led_class_spec_t,
    pub led_i_d: xcb_xkb_id_spec_t,
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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_names_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xkb_get_names_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xkb_get_names.
pub const XCB_XKB_GET_NAMES: u8 = 17i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_names_reply_t {
    pub response_type: u8,
    pub device_i_d: u8,
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
    pub n_k_t_levels: u16,
    pub pad0: [u8; 4],
}

impl Default for xcb_xkb_get_names_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

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

/// Opcode for xcb_xkb_set_names.
pub const XCB_XKB_SET_NAMES: u8 = 18i32 as u8;

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
    pub first_k_t_levelt: u8,
    pub n_k_t_levels: u8,
    pub indicators: u32,
    pub group_names: u8,
    pub n_radio_groups: u8,
    pub first_key: xcb_keycode_t,
    pub n_keys: u8,
    pub n_key_aliases: u8,
    pub pad0: u8,
    pub total_k_t_level_names: u16,
}

impl Default for xcb_xkb_set_names_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_per_client_flags_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xkb_per_client_flags_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xkb_per_client_flags.
pub const XCB_XKB_PER_CLIENT_FLAGS: u8 = 21i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_per_client_flags_reply_t {
    pub response_type: u8,
    pub device_i_d: u8,
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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_list_components_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xkb_list_components_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xkb_list_components.
pub const XCB_XKB_LIST_COMPONENTS: u8 = 22i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_list_components_reply_t {
    pub response_type: u8,
    pub device_i_d: u8,
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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xkb_get_kbd_by_name_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xkb_get_kbd_by_name.
pub const XCB_XKB_GET_KBD_BY_NAME: u8 = 23i32 as u8;

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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_replies_t__types {
    pub getmap_type: u8,
    pub type_device_i_d: u8,
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
    pub first_v_mod_map_key: xcb_keycode_t,
    pub n_v_mod_map_keys: u8,
    pub total_v_mod_map_keys: u8,
    pub pad2: u8,
    pub virtual_mods: u16,
    pub map: xcb_xkb_get_kbd_by_name_replies_types_map_t,
}

impl Default for xcb_xkb_get_kbd_by_name_replies_t__types {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_replies_t__compat_map {
    pub compatmap_type: u8,
    pub compat_device_i_d: u8,
    pub compatmap_sequence: u16,
    pub compatmap_length: u32,
    pub groups_rtrn: u8,
    pub pad7: u8,
    pub first_s_i_rtrn: u16,
    pub n_s_i_rtrn: u16,
    pub n_total_s_i: u16,
    pub pad8: [u8; 16],
    pub si_rtrn: *mut xcb_xkb_sym_interpret_t,
    pub group_rtrn: *mut xcb_xkb_mod_def_t,
}

impl Default for xcb_xkb_get_kbd_by_name_replies_t__compat_map {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_replies_t__indicator_maps {
    pub indicatormap_type: u8,
    pub indicator_device_i_d: u8,
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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_replies_t__key_names {
    pub keyname_type: u8,
    pub key_device_i_d: u8,
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
    pub n_k_t_levels: u16,
    pub pad10: [u8; 4],
    pub value_list: xcb_xkb_get_kbd_by_name_replies_key_names_value_list_t,
}

impl Default for xcb_xkb_get_kbd_by_name_replies_t__key_names {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_replies_t__geometry {
    pub geometry_type: u8,
    pub geometry_device_i_d: u8,
    pub geometry_sequence: u16,
    pub geometry_length: u32,
    pub name: xcb_atom_t,
    pub geometry_found: u8,
    pub pad12: u8,
    pub width_m_m: u16,
    pub height_m_m: u16,
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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_kbd_by_name_reply_t {
    pub response_type: u8,
    pub device_i_d: u8,
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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_device_info_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xkb_get_device_info_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xkb_get_device_info.
pub const XCB_XKB_GET_DEVICE_INFO: u8 = 24i32 as u8;

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
    pub led_i_d: xcb_xkb_id_spec_t,
}

impl Default for xcb_xkb_get_device_info_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_get_device_info_reply_t {
    pub response_type: u8,
    pub device_i_d: u8,
    pub sequence: u16,
    pub length: u32,
    pub present: u16,
    pub supported: u16,
    pub unsupported: u16,
    pub n_device_led_f_bs: u16,
    pub first_btn_wanted: u8,
    pub n_btns_wanted: u8,
    pub first_btn_rtrn: u8,
    pub n_btns_rtrn: u8,
    pub total_btns: u8,
    pub has_own_state: u8,
    pub dflt_kbd_f_b: u16,
    pub dflt_led_f_b: u16,
    pub pad0: [u8; 2],
    pub dev_type: xcb_atom_t,
    pub name_len: u16,
}

impl Default for xcb_xkb_get_device_info_reply_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xkb_set_device_info.
pub const XCB_XKB_SET_DEVICE_INFO: u8 = 25i32 as u8;

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
    pub n_device_led_f_bs: u16,
}

impl Default for xcb_xkb_set_device_info_request_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_set_debugging_flags_cookie_t {
    pub sequence: c_uint,
}

impl Default for xcb_xkb_set_debugging_flags_cookie_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xkb_set_debugging_flags.
pub const XCB_XKB_SET_DEBUGGING_FLAGS: u8 = 101i32 as u8;

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

/// Opcode for xcb_xkb_new_keyboard_notify.
pub const XCB_XKB_NEW_KEYBOARD_NOTIFY: u8 = 0i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_new_keyboard_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_i_d: u8,
    pub old_device_i_d: u8,
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

/// Opcode for xcb_xkb_map_notify.
pub const XCB_XKB_MAP_NOTIFY: u8 = 1i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_map_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_i_d: u8,
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
    pub first_v_mod_map_key: xcb_keycode_t,
    pub n_v_mod_map_keys: u8,
    pub virtual_mods: u16,
    pub pad0: [u8; 2],
}

impl Default for xcb_xkb_map_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xkb_state_notify.
pub const XCB_XKB_STATE_NOTIFY: u8 = 2i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_state_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_i_d: u8,
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

/// Opcode for xcb_xkb_controls_notify.
pub const XCB_XKB_CONTROLS_NOTIFY: u8 = 3i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_controls_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_i_d: u8,
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

/// Opcode for xcb_xkb_indicator_state_notify.
pub const XCB_XKB_INDICATOR_STATE_NOTIFY: u8 = 4i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_indicator_state_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_i_d: u8,
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

/// Opcode for xcb_xkb_indicator_map_notify.
pub const XCB_XKB_INDICATOR_MAP_NOTIFY: u8 = 5i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_indicator_map_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_i_d: u8,
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

/// Opcode for xcb_xkb_names_notify.
pub const XCB_XKB_NAMES_NOTIFY: u8 = 6i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_names_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_i_d: u8,
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

/// Opcode for xcb_xkb_compat_map_notify.
pub const XCB_XKB_COMPAT_MAP_NOTIFY: u8 = 7i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_compat_map_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_i_d: u8,
    pub changed_groups: u8,
    pub first_s_i: u16,
    pub n_s_i: u16,
    pub n_total_s_i: u16,
    pub pad0: [u8; 16],
}

impl Default for xcb_xkb_compat_map_notify_event_t {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
    }
}

/// Opcode for xcb_xkb_bell_notify.
pub const XCB_XKB_BELL_NOTIFY: u8 = 8i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_bell_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_i_d: u8,
    pub bell_class: u8,
    pub bell_i_d: u8,
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

/// Opcode for xcb_xkb_action_message.
pub const XCB_XKB_ACTION_MESSAGE: u8 = 9i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_action_message_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_i_d: u8,
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

/// Opcode for xcb_xkb_access_x_notify.
pub const XCB_XKB_ACCESS_X_NOTIFY: u8 = 10i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_access_x_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_i_d: u8,
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

/// Opcode for xcb_xkb_extension_device_notify.
pub const XCB_XKB_EXTENSION_DEVICE_NOTIFY: u8 = 11i32 as u8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_xkb_extension_device_notify_event_t {
    pub response_type: u8,
    pub xkb_type: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub device_i_d: u8,
    pub pad0: u8,
    pub reason: u16,
    pub led_class: u16,
    pub led_i_d: u16,
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
        ) -> xcb_xkb_use_extension_reply_t,
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
            bell_i_d: xcb_xkb_id_spec_t,
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
            bell_i_d: xcb_xkb_id_spec_t,
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
        ) -> xcb_xkb_get_state_reply_t,
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
        ) -> xcb_xkb_get_controls_reply_t,
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
            access_x_options: u16,
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
            access_x_timeout: u16,
            access_x_timeout_mask: u32,
            access_x_timeout_values: u32,
            access_x_timeout_options_mask: u16,
            access_x_timeout_options_values: u16,
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
            access_x_options: u16,
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
            access_x_timeout: u16,
            access_x_timeout_mask: u32,
            access_x_timeout_values: u32,
            access_x_timeout_options_mask: u16,
            access_x_timeout_options_values: u16,
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
            first_v_mod_map_key: xcb_keycode_t,
            n_v_mod_map_keys: u8,
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
            first_v_mod_map_key: xcb_keycode_t,
            n_v_mod_map_keys: u8,
        ) -> xcb_xkb_get_map_cookie_t,
    >,
    xcb_xkb_get_map_map: LazySymbol<unsafe fn(r: *const xcb_xkb_get_map_reply_t) -> *mut c_void>,
    xcb_xkb_get_map_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xkb_get_map_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_xkb_get_map_reply_t,
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
            first_v_mod_map_key: xcb_keycode_t,
            n_v_mod_map_keys: u8,
            total_v_mod_map_keys: u8,
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
            first_v_mod_map_key: xcb_keycode_t,
            n_v_mod_map_keys: u8,
            total_v_mod_map_keys: u8,
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
            first_v_mod_map_key: xcb_keycode_t,
            n_v_mod_map_keys: u8,
            total_v_mod_map_keys: u8,
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
            first_v_mod_map_key: xcb_keycode_t,
            n_v_mod_map_keys: u8,
            total_v_mod_map_keys: u8,
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
            get_all_s_i: u8,
            first_s_i: u16,
            n_s_i: u16,
        ) -> xcb_xkb_get_compat_map_cookie_t,
    >,
    xcb_xkb_get_compat_map_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            groups: u8,
            get_all_s_i: u8,
            first_s_i: u16,
            n_s_i: u16,
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
        ) -> xcb_xkb_get_compat_map_reply_t,
    >,
    xcb_xkb_set_compat_map_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xkb_set_compat_map_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            recompute_actions: u8,
            truncate_s_i: u8,
            groups: u8,
            first_s_i: u16,
            n_s_i: u16,
            si: *const xcb_xkb_sym_interpret_t,
            group_maps: *const xcb_xkb_mod_def_t,
        ) -> xcb_void_cookie_t,
    >,
    xcb_xkb_set_compat_map: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            recompute_actions: u8,
            truncate_s_i: u8,
            groups: u8,
            first_s_i: u16,
            n_s_i: u16,
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
        ) -> xcb_xkb_get_indicator_state_reply_t,
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
        ) -> xcb_xkb_get_indicator_map_reply_t,
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
            led_i_d: xcb_xkb_id_spec_t,
            indicator: xcb_atom_t,
        ) -> xcb_xkb_get_named_indicator_cookie_t,
    >,
    xcb_xkb_get_named_indicator_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            led_class: xcb_xkb_led_class_spec_t,
            led_i_d: xcb_xkb_id_spec_t,
            indicator: xcb_atom_t,
        ) -> xcb_xkb_get_named_indicator_cookie_t,
    >,
    xcb_xkb_get_named_indicator_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_xkb_get_named_indicator_cookie_t,
            e: *mut *mut xcb_generic_error_t,
        ) -> xcb_xkb_get_named_indicator_reply_t,
    >,
    xcb_xkb_set_named_indicator_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            led_class: xcb_xkb_led_class_spec_t,
            led_i_d: xcb_xkb_id_spec_t,
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
            led_i_d: xcb_xkb_id_spec_t,
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
        ) -> xcb_xkb_get_names_reply_t,
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
            first_k_t_levelt: u8,
            n_k_t_levels: u8,
            indicators: u32,
            group_names: u8,
            n_radio_groups: u8,
            first_key: xcb_keycode_t,
            n_keys: u8,
            n_key_aliases: u8,
            total_k_t_level_names: u16,
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
            first_k_t_levelt: u8,
            n_k_t_levels: u8,
            indicators: u32,
            group_names: u8,
            n_radio_groups: u8,
            first_key: xcb_keycode_t,
            n_keys: u8,
            n_key_aliases: u8,
            total_k_t_level_names: u16,
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
            first_k_t_levelt: u8,
            n_k_t_levels: u8,
            indicators: u32,
            group_names: u8,
            n_radio_groups: u8,
            first_key: xcb_keycode_t,
            n_keys: u8,
            n_key_aliases: u8,
            total_k_t_level_names: u16,
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
            first_k_t_levelt: u8,
            n_k_t_levels: u8,
            indicators: u32,
            group_names: u8,
            n_radio_groups: u8,
            first_key: xcb_keycode_t,
            n_keys: u8,
            n_key_aliases: u8,
            total_k_t_level_names: u16,
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
        ) -> xcb_xkb_per_client_flags_reply_t,
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
        ) -> xcb_xkb_list_components_reply_t,
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
        ) -> xcb_xkb_get_kbd_by_name_reply_t,
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
            led_i_d: xcb_xkb_id_spec_t,
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
            led_i_d: xcb_xkb_id_spec_t,
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
        ) -> xcb_xkb_get_device_info_reply_t,
    >,
    xcb_xkb_set_device_info_sizeof: LazySymbol<unsafe fn(_buffer: *const c_void) -> c_int>,
    xcb_xkb_set_device_info_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            device_spec: xcb_xkb_device_spec_t,
            first_btn: u8,
            n_btns: u8,
            change: u16,
            n_device_led_f_bs: u16,
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
            n_device_led_f_bs: u16,
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
        ) -> xcb_xkb_set_debugging_flags_reply_t,
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

impl XcbXkb {
    pub fn xcb_xkb_id(&self) -> *mut xcb_extension_t {
        unsafe { sym!(self, xcb_xkb_id) }
    }

    /// Returns `true` iff the symbol `xcb_xkb_id` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_id(&self) -> bool {
        has_sym!(self, xcb_xkb_id)
    }

    pub unsafe fn xcb_xkb_device_spec_next(&self, i: *mut xcb_xkb_device_spec_iterator_t) {
        sym!(self, xcb_xkb_device_spec_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_device_spec_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_device_spec_next(&self) -> bool {
        has_sym!(self, xcb_xkb_device_spec_next)
    }

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

    pub unsafe fn xcb_xkb_led_class_spec_next(&self, i: *mut xcb_xkb_led_class_spec_iterator_t) {
        sym!(self, xcb_xkb_led_class_spec_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_led_class_spec_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_led_class_spec_next(&self) -> bool {
        has_sym!(self, xcb_xkb_led_class_spec_next)
    }

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

    pub unsafe fn xcb_xkb_bell_class_spec_next(&self, i: *mut xcb_xkb_bell_class_spec_iterator_t) {
        sym!(self, xcb_xkb_bell_class_spec_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_bell_class_spec_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_bell_class_spec_next(&self) -> bool {
        has_sym!(self, xcb_xkb_bell_class_spec_next)
    }

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

    pub unsafe fn xcb_xkb_id_spec_next(&self, i: *mut xcb_xkb_id_spec_iterator_t) {
        sym!(self, xcb_xkb_id_spec_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_id_spec_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_id_spec_next(&self) -> bool {
        has_sym!(self, xcb_xkb_id_spec_next)
    }

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

    pub unsafe fn xcb_xkb_indicator_map_next(&self, i: *mut xcb_xkb_indicator_map_iterator_t) {
        sym!(self, xcb_xkb_indicator_map_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_indicator_map_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_indicator_map_next(&self) -> bool {
        has_sym!(self, xcb_xkb_indicator_map_next)
    }

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

    pub unsafe fn xcb_xkb_mod_def_next(&self, i: *mut xcb_xkb_mod_def_iterator_t) {
        sym!(self, xcb_xkb_mod_def_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_mod_def_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_mod_def_next(&self) -> bool {
        has_sym!(self, xcb_xkb_mod_def_next)
    }

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

    pub unsafe fn xcb_xkb_key_name_next(&self, i: *mut xcb_xkb_key_name_iterator_t) {
        sym!(self, xcb_xkb_key_name_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_name_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_name_next(&self) -> bool {
        has_sym!(self, xcb_xkb_key_name_next)
    }

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

    pub unsafe fn xcb_xkb_key_alias_next(&self, i: *mut xcb_xkb_key_alias_iterator_t) {
        sym!(self, xcb_xkb_key_alias_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_alias_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_alias_next(&self) -> bool {
        has_sym!(self, xcb_xkb_key_alias_next)
    }

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

    pub unsafe fn xcb_xkb_counted_string_16_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_counted_string_16_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_counted_string_16_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_counted_string_16_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_counted_string_16_sizeof)
    }

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

    pub unsafe fn xcb_xkb_kt_map_entry_next(&self, i: *mut xcb_xkb_kt_map_entry_iterator_t) {
        sym!(self, xcb_xkb_kt_map_entry_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_kt_map_entry_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_kt_map_entry_next(&self) -> bool {
        has_sym!(self, xcb_xkb_kt_map_entry_next)
    }

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

    pub unsafe fn xcb_xkb_key_type_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_key_type_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_type_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_type_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_key_type_sizeof)
    }

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

    pub unsafe fn xcb_xkb_key_type_map_length(&self, r: *const xcb_xkb_key_type_t) -> c_int {
        sym!(self, xcb_xkb_key_type_map_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_type_map_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_type_map_length(&self) -> bool {
        has_sym!(self, xcb_xkb_key_type_map_length)
    }

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

    pub unsafe fn xcb_xkb_key_type_preserve_length(&self, r: *const xcb_xkb_key_type_t) -> c_int {
        sym!(self, xcb_xkb_key_type_preserve_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_type_preserve_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_type_preserve_length(&self) -> bool {
        has_sym!(self, xcb_xkb_key_type_preserve_length)
    }

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

    pub unsafe fn xcb_xkb_key_type_next(&self, i: *mut xcb_xkb_key_type_iterator_t) {
        sym!(self, xcb_xkb_key_type_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_type_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_type_next(&self) -> bool {
        has_sym!(self, xcb_xkb_key_type_next)
    }

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

    pub unsafe fn xcb_xkb_key_sym_map_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_key_sym_map_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_sym_map_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_sym_map_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_key_sym_map_sizeof)
    }

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

    pub unsafe fn xcb_xkb_key_sym_map_syms_length(&self, r: *const xcb_xkb_key_sym_map_t) -> c_int {
        sym!(self, xcb_xkb_key_sym_map_syms_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_sym_map_syms_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_sym_map_syms_length(&self) -> bool {
        has_sym!(self, xcb_xkb_key_sym_map_syms_length)
    }

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

    pub unsafe fn xcb_xkb_key_sym_map_next(&self, i: *mut xcb_xkb_key_sym_map_iterator_t) {
        sym!(self, xcb_xkb_key_sym_map_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_sym_map_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_sym_map_next(&self) -> bool {
        has_sym!(self, xcb_xkb_key_sym_map_next)
    }

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

    pub unsafe fn xcb_xkb_common_behavior_next(&self, i: *mut xcb_xkb_common_behavior_iterator_t) {
        sym!(self, xcb_xkb_common_behavior_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_common_behavior_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_common_behavior_next(&self) -> bool {
        has_sym!(self, xcb_xkb_common_behavior_next)
    }

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

    pub unsafe fn xcb_xkb_lock_behavior_next(&self, i: *mut xcb_xkb_lock_behavior_iterator_t) {
        sym!(self, xcb_xkb_lock_behavior_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_lock_behavior_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_lock_behavior_next(&self) -> bool {
        has_sym!(self, xcb_xkb_lock_behavior_next)
    }

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

    pub unsafe fn xcb_xkb_behavior_next(&self, i: *mut xcb_xkb_behavior_iterator_t) {
        sym!(self, xcb_xkb_behavior_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_behavior_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_behavior_next(&self) -> bool {
        has_sym!(self, xcb_xkb_behavior_next)
    }

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

    pub unsafe fn xcb_xkb_set_behavior_next(&self, i: *mut xcb_xkb_set_behavior_iterator_t) {
        sym!(self, xcb_xkb_set_behavior_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_behavior_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_behavior_next(&self) -> bool {
        has_sym!(self, xcb_xkb_set_behavior_next)
    }

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

    pub unsafe fn xcb_xkb_set_explicit_next(&self, i: *mut xcb_xkb_set_explicit_iterator_t) {
        sym!(self, xcb_xkb_set_explicit_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_explicit_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_explicit_next(&self) -> bool {
        has_sym!(self, xcb_xkb_set_explicit_next)
    }

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

    pub unsafe fn xcb_xkb_key_mod_map_next(&self, i: *mut xcb_xkb_key_mod_map_iterator_t) {
        sym!(self, xcb_xkb_key_mod_map_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_mod_map_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_mod_map_next(&self) -> bool {
        has_sym!(self, xcb_xkb_key_mod_map_next)
    }

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

    pub unsafe fn xcb_xkb_key_v_mod_map_next(&self, i: *mut xcb_xkb_key_v_mod_map_iterator_t) {
        sym!(self, xcb_xkb_key_v_mod_map_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_v_mod_map_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_v_mod_map_next(&self) -> bool {
        has_sym!(self, xcb_xkb_key_v_mod_map_next)
    }

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

    pub unsafe fn xcb_xkb_set_key_type_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_set_key_type_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_key_type_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_key_type_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_set_key_type_sizeof)
    }

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

    pub unsafe fn xcb_xkb_set_key_type_next(&self, i: *mut xcb_xkb_set_key_type_iterator_t) {
        sym!(self, xcb_xkb_set_key_type_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_key_type_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_key_type_next(&self) -> bool {
        has_sym!(self, xcb_xkb_set_key_type_next)
    }

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

    pub unsafe fn xcb_xkb_string8_next(&self, i: *mut xcb_xkb_string8_iterator_t) {
        sym!(self, xcb_xkb_string8_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_string8_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_string8_next(&self) -> bool {
        has_sym!(self, xcb_xkb_string8_next)
    }

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

    pub unsafe fn xcb_xkb_outline_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_outline_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_outline_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_outline_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_outline_sizeof)
    }

    pub unsafe fn xcb_xkb_outline_points(&self, r: *const xcb_xkb_outline_t) -> *mut xcb_point_t {
        sym!(self, xcb_xkb_outline_points)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_outline_points` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_outline_points(&self) -> bool {
        has_sym!(self, xcb_xkb_outline_points)
    }

    pub unsafe fn xcb_xkb_outline_points_length(&self, r: *const xcb_xkb_outline_t) -> c_int {
        sym!(self, xcb_xkb_outline_points_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_outline_points_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_outline_points_length(&self) -> bool {
        has_sym!(self, xcb_xkb_outline_points_length)
    }

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

    pub unsafe fn xcb_xkb_outline_next(&self, i: *mut xcb_xkb_outline_iterator_t) {
        sym!(self, xcb_xkb_outline_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_outline_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_outline_next(&self) -> bool {
        has_sym!(self, xcb_xkb_outline_next)
    }

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

    pub unsafe fn xcb_xkb_shape_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_shape_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_shape_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_shape_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_shape_sizeof)
    }

    pub unsafe fn xcb_xkb_shape_outlines_length(&self, r: *const xcb_xkb_shape_t) -> c_int {
        sym!(self, xcb_xkb_shape_outlines_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_shape_outlines_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_shape_outlines_length(&self) -> bool {
        has_sym!(self, xcb_xkb_shape_outlines_length)
    }

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

    pub unsafe fn xcb_xkb_shape_next(&self, i: *mut xcb_xkb_shape_iterator_t) {
        sym!(self, xcb_xkb_shape_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_shape_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_shape_next(&self) -> bool {
        has_sym!(self, xcb_xkb_shape_next)
    }

    pub unsafe fn xcb_xkb_shape_end(&self, i: xcb_xkb_shape_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_shape_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_shape_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_shape_end(&self) -> bool {
        has_sym!(self, xcb_xkb_shape_end)
    }

    pub unsafe fn xcb_xkb_key_next(&self, i: *mut xcb_xkb_key_iterator_t) {
        sym!(self, xcb_xkb_key_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_next(&self) -> bool {
        has_sym!(self, xcb_xkb_key_next)
    }

    pub unsafe fn xcb_xkb_key_end(&self, i: xcb_xkb_key_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_key_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_key_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_key_end(&self) -> bool {
        has_sym!(self, xcb_xkb_key_end)
    }

    pub unsafe fn xcb_xkb_overlay_key_next(&self, i: *mut xcb_xkb_overlay_key_iterator_t) {
        sym!(self, xcb_xkb_overlay_key_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_overlay_key_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_overlay_key_next(&self) -> bool {
        has_sym!(self, xcb_xkb_overlay_key_next)
    }

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

    pub unsafe fn xcb_xkb_overlay_row_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_overlay_row_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_overlay_row_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_overlay_row_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_overlay_row_sizeof)
    }

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

    pub unsafe fn xcb_xkb_overlay_row_keys_length(&self, r: *const xcb_xkb_overlay_row_t) -> c_int {
        sym!(self, xcb_xkb_overlay_row_keys_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_overlay_row_keys_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_overlay_row_keys_length(&self) -> bool {
        has_sym!(self, xcb_xkb_overlay_row_keys_length)
    }

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

    pub unsafe fn xcb_xkb_overlay_row_next(&self, i: *mut xcb_xkb_overlay_row_iterator_t) {
        sym!(self, xcb_xkb_overlay_row_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_overlay_row_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_overlay_row_next(&self) -> bool {
        has_sym!(self, xcb_xkb_overlay_row_next)
    }

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

    pub unsafe fn xcb_xkb_overlay_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_overlay_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_overlay_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_overlay_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_overlay_sizeof)
    }

    pub unsafe fn xcb_xkb_overlay_rows_length(&self, r: *const xcb_xkb_overlay_t) -> c_int {
        sym!(self, xcb_xkb_overlay_rows_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_overlay_rows_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_overlay_rows_length(&self) -> bool {
        has_sym!(self, xcb_xkb_overlay_rows_length)
    }

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

    pub unsafe fn xcb_xkb_overlay_next(&self, i: *mut xcb_xkb_overlay_iterator_t) {
        sym!(self, xcb_xkb_overlay_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_overlay_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_overlay_next(&self) -> bool {
        has_sym!(self, xcb_xkb_overlay_next)
    }

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

    pub unsafe fn xcb_xkb_row_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_row_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_row_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_row_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_row_sizeof)
    }

    pub unsafe fn xcb_xkb_row_keys(&self, r: *const xcb_xkb_row_t) -> *mut xcb_xkb_key_t {
        sym!(self, xcb_xkb_row_keys)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_row_keys` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_row_keys(&self) -> bool {
        has_sym!(self, xcb_xkb_row_keys)
    }

    pub unsafe fn xcb_xkb_row_keys_length(&self, r: *const xcb_xkb_row_t) -> c_int {
        sym!(self, xcb_xkb_row_keys_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_row_keys_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_row_keys_length(&self) -> bool {
        has_sym!(self, xcb_xkb_row_keys_length)
    }

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

    pub unsafe fn xcb_xkb_row_next(&self, i: *mut xcb_xkb_row_iterator_t) {
        sym!(self, xcb_xkb_row_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_row_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_row_next(&self) -> bool {
        has_sym!(self, xcb_xkb_row_next)
    }

    pub unsafe fn xcb_xkb_row_end(&self, i: xcb_xkb_row_iterator_t) -> xcb_generic_iterator_t {
        sym!(self, xcb_xkb_row_end)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_row_end` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_row_end(&self) -> bool {
        has_sym!(self, xcb_xkb_row_end)
    }

    pub unsafe fn xcb_xkb_listing_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_listing_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_listing_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_listing_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_listing_sizeof)
    }

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

    pub unsafe fn xcb_xkb_listing_string_length(&self, r: *const xcb_xkb_listing_t) -> c_int {
        sym!(self, xcb_xkb_listing_string_length)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_listing_string_length` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_listing_string_length(&self) -> bool {
        has_sym!(self, xcb_xkb_listing_string_length)
    }

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

    pub unsafe fn xcb_xkb_listing_next(&self, i: *mut xcb_xkb_listing_iterator_t) {
        sym!(self, xcb_xkb_listing_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_listing_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_listing_next(&self) -> bool {
        has_sym!(self, xcb_xkb_listing_next)
    }

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

    pub unsafe fn xcb_xkb_device_led_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_device_led_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_device_led_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_device_led_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_device_led_info_sizeof)
    }

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

    pub unsafe fn xcb_xkb_device_led_info_next(&self, i: *mut xcb_xkb_device_led_info_iterator_t) {
        sym!(self, xcb_xkb_device_led_info_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_device_led_info_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_device_led_info_next(&self) -> bool {
        has_sym!(self, xcb_xkb_device_led_info_next)
    }

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

    pub unsafe fn xcb_xkb_sa_no_action_next(&self, i: *mut xcb_xkb_sa_no_action_iterator_t) {
        sym!(self, xcb_xkb_sa_no_action_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_no_action_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_no_action_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_no_action_next)
    }

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

    pub unsafe fn xcb_xkb_sa_set_mods_next(&self, i: *mut xcb_xkb_sa_set_mods_iterator_t) {
        sym!(self, xcb_xkb_sa_set_mods_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_set_mods_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_set_mods_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_set_mods_next)
    }

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

    pub unsafe fn xcb_xkb_sa_latch_mods_next(&self, i: *mut xcb_xkb_sa_latch_mods_iterator_t) {
        sym!(self, xcb_xkb_sa_latch_mods_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_latch_mods_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_latch_mods_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_latch_mods_next)
    }

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

    pub unsafe fn xcb_xkb_sa_lock_mods_next(&self, i: *mut xcb_xkb_sa_lock_mods_iterator_t) {
        sym!(self, xcb_xkb_sa_lock_mods_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_lock_mods_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_lock_mods_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_lock_mods_next)
    }

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

    pub unsafe fn xcb_xkb_sa_set_group_next(&self, i: *mut xcb_xkb_sa_set_group_iterator_t) {
        sym!(self, xcb_xkb_sa_set_group_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_set_group_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_set_group_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_set_group_next)
    }

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

    pub unsafe fn xcb_xkb_sa_latch_group_next(&self, i: *mut xcb_xkb_sa_latch_group_iterator_t) {
        sym!(self, xcb_xkb_sa_latch_group_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_latch_group_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_latch_group_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_latch_group_next)
    }

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

    pub unsafe fn xcb_xkb_sa_lock_group_next(&self, i: *mut xcb_xkb_sa_lock_group_iterator_t) {
        sym!(self, xcb_xkb_sa_lock_group_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_lock_group_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_lock_group_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_lock_group_next)
    }

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

    pub unsafe fn xcb_xkb_sa_move_ptr_next(&self, i: *mut xcb_xkb_sa_move_ptr_iterator_t) {
        sym!(self, xcb_xkb_sa_move_ptr_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_move_ptr_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_move_ptr_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_move_ptr_next)
    }

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

    pub unsafe fn xcb_xkb_sa_ptr_btn_next(&self, i: *mut xcb_xkb_sa_ptr_btn_iterator_t) {
        sym!(self, xcb_xkb_sa_ptr_btn_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_ptr_btn_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_ptr_btn_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_ptr_btn_next)
    }

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

    pub unsafe fn xcb_xkb_sa_lock_ptr_btn_next(&self, i: *mut xcb_xkb_sa_lock_ptr_btn_iterator_t) {
        sym!(self, xcb_xkb_sa_lock_ptr_btn_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_lock_ptr_btn_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_lock_ptr_btn_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_lock_ptr_btn_next)
    }

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

    pub unsafe fn xcb_xkb_sa_set_ptr_dflt_next(&self, i: *mut xcb_xkb_sa_set_ptr_dflt_iterator_t) {
        sym!(self, xcb_xkb_sa_set_ptr_dflt_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_set_ptr_dflt_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_set_ptr_dflt_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_set_ptr_dflt_next)
    }

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

    pub unsafe fn xcb_xkb_sa_iso_lock_next(&self, i: *mut xcb_xkb_sa_iso_lock_iterator_t) {
        sym!(self, xcb_xkb_sa_iso_lock_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_iso_lock_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_iso_lock_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_iso_lock_next)
    }

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

    pub unsafe fn xcb_xkb_sa_terminate_next(&self, i: *mut xcb_xkb_sa_terminate_iterator_t) {
        sym!(self, xcb_xkb_sa_terminate_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_terminate_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_terminate_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_terminate_next)
    }

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

    pub unsafe fn xcb_xkb_sa_set_controls_next(&self, i: *mut xcb_xkb_sa_set_controls_iterator_t) {
        sym!(self, xcb_xkb_sa_set_controls_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_set_controls_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_set_controls_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_set_controls_next)
    }

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

    pub unsafe fn xcb_xkb_sa_redirect_key_next(&self, i: *mut xcb_xkb_sa_redirect_key_iterator_t) {
        sym!(self, xcb_xkb_sa_redirect_key_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_redirect_key_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_redirect_key_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_redirect_key_next)
    }

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

    pub unsafe fn xcb_xkb_sa_device_btn_next(&self, i: *mut xcb_xkb_sa_device_btn_iterator_t) {
        sym!(self, xcb_xkb_sa_device_btn_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sa_device_btn_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sa_device_btn_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sa_device_btn_next)
    }

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

    pub unsafe fn xcb_xkb_si_action_next(&self, i: *mut xcb_xkb_si_action_iterator_t) {
        sym!(self, xcb_xkb_si_action_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_si_action_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_si_action_next(&self) -> bool {
        has_sym!(self, xcb_xkb_si_action_next)
    }

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

    pub unsafe fn xcb_xkb_sym_interpret_next(&self, i: *mut xcb_xkb_sym_interpret_iterator_t) {
        sym!(self, xcb_xkb_sym_interpret_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_sym_interpret_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_sym_interpret_next(&self) -> bool {
        has_sym!(self, xcb_xkb_sym_interpret_next)
    }

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

    pub unsafe fn xcb_xkb_action_next(&self, i: *mut xcb_xkb_action_iterator_t) {
        sym!(self, xcb_xkb_action_next)(i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_action_next` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_action_next(&self) -> bool {
        has_sym!(self, xcb_xkb_action_next)
    }

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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_xkb_use_extension_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xkb_use_extension_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_use_extension_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xkb_use_extension_reply_t {
        sym!(self, xcb_xkb_use_extension_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xkb_use_extension_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_use_extension_reply(&self) -> bool {
        has_sym!(self, xcb_xkb_use_extension_reply)
    }

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

    pub unsafe fn xcb_xkb_select_events_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_select_events_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_select_events_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_select_events_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_select_events_sizeof)
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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
    pub unsafe fn xcb_xkb_bell_checked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        bell_class: xcb_xkb_bell_class_spec_t,
        bell_i_d: xcb_xkb_id_spec_t,
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
            bell_i_d,
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xkb_bell(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        bell_class: xcb_xkb_bell_class_spec_t,
        bell_i_d: xcb_xkb_id_spec_t,
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
            bell_i_d,
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_xkb_get_state_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xkb_get_state_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_state_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xkb_get_state_reply_t {
        sym!(self, xcb_xkb_get_state_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_state_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_state_reply(&self) -> bool {
        has_sym!(self, xcb_xkb_get_state_reply)
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_xkb_get_controls_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xkb_get_controls_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_controls_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xkb_get_controls_reply_t {
        sym!(self, xcb_xkb_get_controls_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_controls_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_controls_reply(&self) -> bool {
        has_sym!(self, xcb_xkb_get_controls_reply)
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
        access_x_options: u16,
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
        access_x_timeout: u16,
        access_x_timeout_mask: u32,
        access_x_timeout_values: u32,
        access_x_timeout_options_mask: u16,
        access_x_timeout_options_values: u16,
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
            access_x_options,
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
            access_x_timeout,
            access_x_timeout_mask,
            access_x_timeout_values,
            access_x_timeout_options_mask,
            access_x_timeout_options_values,
            per_key_repeat,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_controls_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_controls_checked(&self) -> bool {
        has_sym!(self, xcb_xkb_set_controls_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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
        access_x_options: u16,
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
        access_x_timeout: u16,
        access_x_timeout_mask: u32,
        access_x_timeout_values: u32,
        access_x_timeout_options_mask: u16,
        access_x_timeout_options_values: u16,
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
            access_x_options,
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
            access_x_timeout,
            access_x_timeout_mask,
            access_x_timeout_values,
            access_x_timeout_options_mask,
            access_x_timeout_options_values,
            per_key_repeat,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_controls` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_controls(&self) -> bool {
        has_sym!(self, xcb_xkb_set_controls)
    }

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

    pub unsafe fn xcb_xkb_get_map_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_get_map_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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
        first_v_mod_map_key: xcb_keycode_t,
        n_v_mod_map_keys: u8,
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
            first_v_mod_map_key,
            n_v_mod_map_keys,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map)
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
        first_v_mod_map_key: xcb_keycode_t,
        n_v_mod_map_keys: u8,
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
            first_v_mod_map_key,
            n_v_mod_map_keys,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_unchecked(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_unchecked)
    }

    pub unsafe fn xcb_xkb_get_map_map(&self, r: *const xcb_xkb_get_map_reply_t) -> *mut c_void {
        sym!(self, xcb_xkb_get_map_map)(r)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_map` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_map(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_map)
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
     * xcb_xkb_get_map_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xkb_get_map_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_map_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xkb_get_map_reply_t {
        sym!(self, xcb_xkb_get_map_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_map_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_map_reply(&self) -> bool {
        has_sym!(self, xcb_xkb_get_map_reply)
    }

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

    pub unsafe fn xcb_xkb_set_map_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_set_map_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_sizeof)
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
        first_v_mod_map_key: xcb_keycode_t,
        n_v_mod_map_keys: u8,
        total_v_mod_map_keys: u8,
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
            first_v_mod_map_key,
            n_v_mod_map_keys,
            total_v_mod_map_keys,
            virtual_mods,
            values,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_checked(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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
        first_v_mod_map_key: xcb_keycode_t,
        n_v_mod_map_keys: u8,
        total_v_mod_map_keys: u8,
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
            first_v_mod_map_key,
            n_v_mod_map_keys,
            total_v_mod_map_keys,
            virtual_mods,
            values,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map)
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
        first_v_mod_map_key: xcb_keycode_t,
        n_v_mod_map_keys: u8,
        total_v_mod_map_keys: u8,
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
            first_v_mod_map_key,
            n_v_mod_map_keys,
            total_v_mod_map_keys,
            virtual_mods,
            values,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_aux_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_aux_checked(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_aux_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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
        first_v_mod_map_key: xcb_keycode_t,
        n_v_mod_map_keys: u8,
        total_v_mod_map_keys: u8,
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
            first_v_mod_map_key,
            n_v_mod_map_keys,
            total_v_mod_map_keys,
            virtual_mods,
            values,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_map_aux` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_map_aux(&self) -> bool {
        has_sym!(self, xcb_xkb_set_map_aux)
    }

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

    pub unsafe fn xcb_xkb_get_compat_map_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_get_compat_map_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_compat_map_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_compat_map_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_get_compat_map_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xkb_get_compat_map(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        groups: u8,
        get_all_s_i: u8,
        first_s_i: u16,
        n_s_i: u16,
    ) -> xcb_xkb_get_compat_map_cookie_t {
        sym!(self, xcb_xkb_get_compat_map)(c, device_spec, groups, get_all_s_i, first_s_i, n_s_i)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_compat_map` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_compat_map(&self) -> bool {
        has_sym!(self, xcb_xkb_get_compat_map)
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
    pub unsafe fn xcb_xkb_get_compat_map_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        groups: u8,
        get_all_s_i: u8,
        first_s_i: u16,
        n_s_i: u16,
    ) -> xcb_xkb_get_compat_map_cookie_t {
        sym!(self, xcb_xkb_get_compat_map_unchecked)(
            c,
            device_spec,
            groups,
            get_all_s_i,
            first_s_i,
            n_s_i,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_compat_map_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_compat_map_unchecked(&self) -> bool {
        has_sym!(self, xcb_xkb_get_compat_map_unchecked)
    }

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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_xkb_get_compat_map_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xkb_get_compat_map_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_compat_map_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xkb_get_compat_map_reply_t {
        sym!(self, xcb_xkb_get_compat_map_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_compat_map_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_compat_map_reply(&self) -> bool {
        has_sym!(self, xcb_xkb_get_compat_map_reply)
    }

    pub unsafe fn xcb_xkb_set_compat_map_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_set_compat_map_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_compat_map_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_compat_map_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_set_compat_map_sizeof)
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
    pub unsafe fn xcb_xkb_set_compat_map_checked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        recompute_actions: u8,
        truncate_s_i: u8,
        groups: u8,
        first_s_i: u16,
        n_s_i: u16,
        si: *const xcb_xkb_sym_interpret_t,
        group_maps: *const xcb_xkb_mod_def_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_set_compat_map_checked)(
            c,
            device_spec,
            recompute_actions,
            truncate_s_i,
            groups,
            first_s_i,
            n_s_i,
            si,
            group_maps,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_compat_map_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_compat_map_checked(&self) -> bool {
        has_sym!(self, xcb_xkb_set_compat_map_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xkb_set_compat_map(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        recompute_actions: u8,
        truncate_s_i: u8,
        groups: u8,
        first_s_i: u16,
        n_s_i: u16,
        si: *const xcb_xkb_sym_interpret_t,
        group_maps: *const xcb_xkb_mod_def_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_set_compat_map)(
            c,
            device_spec,
            recompute_actions,
            truncate_s_i,
            groups,
            first_s_i,
            n_s_i,
            si,
            group_maps,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_compat_map` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_compat_map(&self) -> bool {
        has_sym!(self, xcb_xkb_set_compat_map)
    }

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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_xkb_get_indicator_state_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xkb_get_indicator_state_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_indicator_state_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xkb_get_indicator_state_reply_t {
        sym!(self, xcb_xkb_get_indicator_state_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_indicator_state_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_indicator_state_reply(&self) -> bool {
        has_sym!(self, xcb_xkb_get_indicator_state_reply)
    }

    pub unsafe fn xcb_xkb_get_indicator_map_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_get_indicator_map_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_indicator_map_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_indicator_map_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_get_indicator_map_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_xkb_get_indicator_map_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xkb_get_indicator_map_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_indicator_map_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xkb_get_indicator_map_reply_t {
        sym!(self, xcb_xkb_get_indicator_map_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_indicator_map_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_indicator_map_reply(&self) -> bool {
        has_sym!(self, xcb_xkb_get_indicator_map_reply)
    }

    pub unsafe fn xcb_xkb_set_indicator_map_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_set_indicator_map_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_indicator_map_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_indicator_map_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_set_indicator_map_sizeof)
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xkb_get_named_indicator(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        led_class: xcb_xkb_led_class_spec_t,
        led_i_d: xcb_xkb_id_spec_t,
        indicator: xcb_atom_t,
    ) -> xcb_xkb_get_named_indicator_cookie_t {
        sym!(self, xcb_xkb_get_named_indicator)(c, device_spec, led_class, led_i_d, indicator)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_named_indicator` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_named_indicator(&self) -> bool {
        has_sym!(self, xcb_xkb_get_named_indicator)
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
    pub unsafe fn xcb_xkb_get_named_indicator_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        led_class: xcb_xkb_led_class_spec_t,
        led_i_d: xcb_xkb_id_spec_t,
        indicator: xcb_atom_t,
    ) -> xcb_xkb_get_named_indicator_cookie_t {
        sym!(self, xcb_xkb_get_named_indicator_unchecked)(
            c,
            device_spec,
            led_class,
            led_i_d,
            indicator,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_named_indicator_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_named_indicator_unchecked(&self) -> bool {
        has_sym!(self, xcb_xkb_get_named_indicator_unchecked)
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
     * xcb_xkb_get_named_indicator_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xkb_get_named_indicator_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_named_indicator_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xkb_get_named_indicator_reply_t {
        sym!(self, xcb_xkb_get_named_indicator_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_named_indicator_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_named_indicator_reply(&self) -> bool {
        has_sym!(self, xcb_xkb_get_named_indicator_reply)
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
    pub unsafe fn xcb_xkb_set_named_indicator_checked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        led_class: xcb_xkb_led_class_spec_t,
        led_i_d: xcb_xkb_id_spec_t,
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
            led_i_d,
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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xkb_set_named_indicator(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        led_class: xcb_xkb_led_class_spec_t,
        led_i_d: xcb_xkb_id_spec_t,
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
            led_i_d,
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

    pub unsafe fn xcb_xkb_get_names_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_get_names_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_xkb_get_names_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xkb_get_names_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_names_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xkb_get_names_reply_t {
        sym!(self, xcb_xkb_get_names_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_names_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_names_reply(&self) -> bool {
        has_sym!(self, xcb_xkb_get_names_reply)
    }

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

    pub unsafe fn xcb_xkb_set_names_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_set_names_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_sizeof)
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
    pub unsafe fn xcb_xkb_set_names_checked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        virtual_mods: u16,
        which: u32,
        first_type: u8,
        n_types: u8,
        first_k_t_levelt: u8,
        n_k_t_levels: u8,
        indicators: u32,
        group_names: u8,
        n_radio_groups: u8,
        first_key: xcb_keycode_t,
        n_keys: u8,
        n_key_aliases: u8,
        total_k_t_level_names: u16,
        values: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_set_names_checked)(
            c,
            device_spec,
            virtual_mods,
            which,
            first_type,
            n_types,
            first_k_t_levelt,
            n_k_t_levels,
            indicators,
            group_names,
            n_radio_groups,
            first_key,
            n_keys,
            n_key_aliases,
            total_k_t_level_names,
            values,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_checked(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xkb_set_names(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        virtual_mods: u16,
        which: u32,
        first_type: u8,
        n_types: u8,
        first_k_t_levelt: u8,
        n_k_t_levels: u8,
        indicators: u32,
        group_names: u8,
        n_radio_groups: u8,
        first_key: xcb_keycode_t,
        n_keys: u8,
        n_key_aliases: u8,
        total_k_t_level_names: u16,
        values: *const c_void,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_set_names)(
            c,
            device_spec,
            virtual_mods,
            which,
            first_type,
            n_types,
            first_k_t_levelt,
            n_k_t_levels,
            indicators,
            group_names,
            n_radio_groups,
            first_key,
            n_keys,
            n_key_aliases,
            total_k_t_level_names,
            values,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names)
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
    pub unsafe fn xcb_xkb_set_names_aux_checked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        virtual_mods: u16,
        which: u32,
        first_type: u8,
        n_types: u8,
        first_k_t_levelt: u8,
        n_k_t_levels: u8,
        indicators: u32,
        group_names: u8,
        n_radio_groups: u8,
        first_key: xcb_keycode_t,
        n_keys: u8,
        n_key_aliases: u8,
        total_k_t_level_names: u16,
        values: *const xcb_xkb_set_names_values_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_set_names_aux_checked)(
            c,
            device_spec,
            virtual_mods,
            which,
            first_type,
            n_types,
            first_k_t_levelt,
            n_k_t_levels,
            indicators,
            group_names,
            n_radio_groups,
            first_key,
            n_keys,
            n_key_aliases,
            total_k_t_level_names,
            values,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_aux_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_aux_checked(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_aux_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xkb_set_names_aux(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        virtual_mods: u16,
        which: u32,
        first_type: u8,
        n_types: u8,
        first_k_t_levelt: u8,
        n_k_t_levels: u8,
        indicators: u32,
        group_names: u8,
        n_radio_groups: u8,
        first_key: xcb_keycode_t,
        n_keys: u8,
        n_key_aliases: u8,
        total_k_t_level_names: u16,
        values: *const xcb_xkb_set_names_values_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_set_names_aux)(
            c,
            device_spec,
            virtual_mods,
            which,
            first_type,
            n_types,
            first_k_t_levelt,
            n_k_t_levels,
            indicators,
            group_names,
            n_radio_groups,
            first_key,
            n_keys,
            n_key_aliases,
            total_k_t_level_names,
            values,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_names_aux` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_names_aux(&self) -> bool {
        has_sym!(self, xcb_xkb_set_names_aux)
    }

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

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_xkb_per_client_flags_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xkb_per_client_flags_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_per_client_flags_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xkb_per_client_flags_reply_t {
        sym!(self, xcb_xkb_per_client_flags_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xkb_per_client_flags_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_per_client_flags_reply(&self) -> bool {
        has_sym!(self, xcb_xkb_per_client_flags_reply)
    }

    pub unsafe fn xcb_xkb_list_components_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_list_components_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_list_components_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_list_components_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_list_components_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_xkb_list_components_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xkb_list_components_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_list_components_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xkb_list_components_reply_t {
        sym!(self, xcb_xkb_list_components_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xkb_list_components_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_list_components_reply(&self) -> bool {
        has_sym!(self, xcb_xkb_list_components_reply)
    }

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

    pub unsafe fn xcb_xkb_get_kbd_by_name_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_get_kbd_by_name_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_get_kbd_by_name_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_xkb_get_kbd_by_name_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xkb_get_kbd_by_name_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_kbd_by_name_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xkb_get_kbd_by_name_reply_t {
        sym!(self, xcb_xkb_get_kbd_by_name_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_kbd_by_name_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_kbd_by_name_reply(&self) -> bool {
        has_sym!(self, xcb_xkb_get_kbd_by_name_reply)
    }

    pub unsafe fn xcb_xkb_get_device_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_get_device_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_device_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_device_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_get_device_info_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xkb_get_device_info(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        wanted: u16,
        all_buttons: u8,
        first_button: u8,
        n_buttons: u8,
        led_class: xcb_xkb_led_class_spec_t,
        led_i_d: xcb_xkb_id_spec_t,
    ) -> xcb_xkb_get_device_info_cookie_t {
        sym!(self, xcb_xkb_get_device_info)(
            c,
            device_spec,
            wanted,
            all_buttons,
            first_button,
            n_buttons,
            led_class,
            led_i_d,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_device_info` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_device_info(&self) -> bool {
        has_sym!(self, xcb_xkb_get_device_info)
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
    pub unsafe fn xcb_xkb_get_device_info_unchecked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        wanted: u16,
        all_buttons: u8,
        first_button: u8,
        n_buttons: u8,
        led_class: xcb_xkb_led_class_spec_t,
        led_i_d: xcb_xkb_id_spec_t,
    ) -> xcb_xkb_get_device_info_cookie_t {
        sym!(self, xcb_xkb_get_device_info_unchecked)(
            c,
            device_spec,
            wanted,
            all_buttons,
            first_button,
            n_buttons,
            led_class,
            led_i_d,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_device_info_unchecked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_device_info_unchecked(&self) -> bool {
        has_sym!(self, xcb_xkb_get_device_info_unchecked)
    }

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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_xkb_get_device_info_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xkb_get_device_info_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_get_device_info_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xkb_get_device_info_reply_t {
        sym!(self, xcb_xkb_get_device_info_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xkb_get_device_info_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_get_device_info_reply(&self) -> bool {
        has_sym!(self, xcb_xkb_get_device_info_reply)
    }

    pub unsafe fn xcb_xkb_set_device_info_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_set_device_info_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_device_info_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_device_info_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_set_device_info_sizeof)
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
    pub unsafe fn xcb_xkb_set_device_info_checked(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        first_btn: u8,
        n_btns: u8,
        change: u16,
        n_device_led_f_bs: u16,
        btn_actions: *const xcb_xkb_action_t,
        leds: *const xcb_xkb_device_led_info_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_set_device_info_checked)(
            c,
            device_spec,
            first_btn,
            n_btns,
            change,
            n_device_led_f_bs,
            btn_actions,
            leds,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_device_info_checked` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_device_info_checked(&self) -> bool {
        has_sym!(self, xcb_xkb_set_device_info_checked)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
    pub unsafe fn xcb_xkb_set_device_info(
        &self,
        c: *mut xcb_connection_t,
        device_spec: xcb_xkb_device_spec_t,
        first_btn: u8,
        n_btns: u8,
        change: u16,
        n_device_led_f_bs: u16,
        btn_actions: *const xcb_xkb_action_t,
        leds: *const xcb_xkb_device_led_info_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_xkb_set_device_info)(
            c,
            device_spec,
            first_btn,
            n_btns,
            change,
            n_device_led_f_bs,
            btn_actions,
            leds,
        )
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_device_info` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_device_info(&self) -> bool {
        has_sym!(self, xcb_xkb_set_device_info)
    }

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

    pub unsafe fn xcb_xkb_set_debugging_flags_sizeof(&self, _buffer: *const c_void) -> c_int {
        sym!(self, xcb_xkb_set_debugging_flags_sizeof)(_buffer)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_debugging_flags_sizeof` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_debugging_flags_sizeof(&self) -> bool {
        has_sym!(self, xcb_xkb_set_debugging_flags_sizeof)
    }

    /**
     *
     * @param c The connection
     * @return A cookie
     *
     * Delivers a request to the X server.
     *
     */
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

    /**
     * Return the reply
     * @param c      The connection
     * @param cookie The cookie
     * @param e      The xcb_generic_error_t supplied
     *
     * Returns the reply of the request asked by
     *
     * The parameter @p e supplied to this function must be NULL if
     * xcb_xkb_set_debugging_flags_unchecked(). is used.
     * Otherwise, it stores the error if any.
     *
     * The returned value must be freed by the caller using free().
     */
    pub unsafe fn xcb_xkb_set_debugging_flags_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_xkb_set_debugging_flags_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> xcb_xkb_set_debugging_flags_reply_t {
        sym!(self, xcb_xkb_set_debugging_flags_reply)(c, cookie, e)
    }

    /// Returns `true` iff the symbol `xcb_xkb_set_debugging_flags_reply` could be loaded.
    #[cfg(feature = "has_symbol")]
    pub fn has_xcb_xkb_set_debugging_flags_reply(&self) -> bool {
        has_sym!(self, xcb_xkb_set_debugging_flags_reply)
    }
}

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
