use crate::ffi::*;
use crate::*;
use std::os::raw::*;

pub const XCB_PRESENT_MAJOR_VERSION: u32 = 1;
pub const XCB_PRESENT_MINOR_VERSION: u32 = 2;

pub type xcb_present_event_enum_t = u32;
pub const XCB_PRESENT_EVENT_CONFIGURE_NOTIFY: xcb_present_event_enum_t = 0x00;
pub const XCB_PRESENT_EVENT_COMPLETE_NOTIFY: xcb_present_event_enum_t = 0x01;
pub const XCB_PRESENT_EVENT_IDLE_NOTIFY: xcb_present_event_enum_t = 0x02;
pub const XCB_PRESENT_EVENT_REDIRECT_NOTIFY: xcb_present_event_enum_t = 0x03;

pub type xcb_present_event_mask_t = u32;
pub const XCB_PRESENT_EVENT_MASK_NO_EVENT: xcb_present_event_mask_t = 0x00;
pub const XCB_PRESENT_EVENT_MASK_CONFIGURE_NOTIFY: xcb_present_event_mask_t = 0x01;
pub const XCB_PRESENT_EVENT_MASK_COMPLETE_NOTIFY: xcb_present_event_mask_t = 0x02;
pub const XCB_PRESENT_EVENT_MASK_IDLE_NOTIFY: xcb_present_event_mask_t = 0x04;
pub const XCB_PRESENT_EVENT_MASK_REDIRECT_NOTIFY: xcb_present_event_mask_t = 0x08;

pub type xcb_present_option_t = u32;
pub const XCB_PRESENT_OPTION_NONE: xcb_present_option_t = 0x00;
pub const XCB_PRESENT_OPTION_ASYNC: xcb_present_option_t = 0x01;
pub const XCB_PRESENT_OPTION_COPY: xcb_present_option_t = 0x02;
pub const XCB_PRESENT_OPTION_UST: xcb_present_option_t = 0x04;
pub const XCB_PRESENT_OPTION_SUBOPTIMAL: xcb_present_option_t = 0x08;

pub type xcb_present_capability_t = u32;
pub const XCB_PRESENT_CAPABILITY_NONE: xcb_present_capability_t = 0x00;
pub const XCB_PRESENT_CAPABILITY_ASYNC: xcb_present_capability_t = 0x01;
pub const XCB_PRESENT_CAPABILITY_FENCE: xcb_present_capability_t = 0x02;
pub const XCB_PRESENT_CAPABILITY_UST: xcb_present_capability_t = 0x04;

pub type xcb_present_complete_kind_t = u32;
pub const XCB_PRESENT_COMPLETE_KIND_PIXMAP: xcb_present_complete_kind_t = 0x00;
pub const XCB_PRESENT_COMPLETE_KIND_NOTIFY_MSC: xcb_present_complete_kind_t = 0x01;

pub type xcb_present_complete_mode_t = u32;
pub const XCB_PRESENT_COMPLETE_MODE_COPY: xcb_present_complete_mode_t = 0x00;
pub const XCB_PRESENT_COMPLETE_MODE_FLIP: xcb_present_complete_mode_t = 0x01;
pub const XCB_PRESENT_COMPLETE_MODE_SKIP: xcb_present_complete_mode_t = 0x02;
pub const XCB_PRESENT_COMPLETE_MODE_SUBOPTIMAL_COPY: xcb_present_complete_mode_t = 0x03;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_notify_t {
    pub window: xcb_window_t,
    pub serial: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_notify_iterator_t {
    pub data: *mut xcb_present_notify_t,
    pub rem: c_int,
    pub index: c_int,
}

pub const XCB_PRESENT_QUERY_VERSION: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub major_version: u32,
    pub minor_version: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_query_version_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u32,
    pub minor_version: u32,
}

pub const XCB_PRESENT_PIXMAP: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_pixmap_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub pixmap: xcb_pixmap_t,
    pub serial: u32,
    pub valid: xcb_xfixes_region_t,
    pub update: xcb_xfixes_region_t,
    pub x_off: i16,
    pub y_off: i16,
    pub target_crtc: xcb_randr_crtc_t,
    pub wait_fence: xcb_sync_fence_t,
    pub idle_fence: xcb_sync_fence_t,
    pub options: u32,
    pub pad0: [u8; 4],
    pub target_msc: u64,
    pub divisor: u64,
    pub remainder: u64,
}

pub const XCB_PRESENT_NOTIFY_MSC: u8 = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_notify_msc_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub window: xcb_window_t,
    pub serial: u32,
    pub pad0: [u8; 4],
    pub target_msc: u64,
    pub divisor: u64,
    pub remainder: u64,
}

pub type xcb_present_event_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_event_iterator_t {
    pub data: *mut xcb_present_event_t,
    pub rem: c_int,
    pub index: c_int,
}

pub const XCB_PRESENT_SELECT_INPUT: u8 = 3;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_select_input_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub eid: xcb_present_event_t,
    pub window: xcb_window_t,
    pub event_mask: u32,
}

pub const XCB_PRESENT_QUERY_CAPABILITIES: u8 = 4;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_query_capabilities_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub target: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_query_capabilities_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_query_capabilities_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub capabilities: u32,
}

pub const XCB_PRESENT_GENERIC: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_generic_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub evtype: u16,
    pub pad0: [u8; 2],
    pub event: xcb_present_event_t,
}

pub const XCB_PRESENT_CONFIGURE_NOTIFY: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_configure_notify_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub pad0: [u8; 2],
    pub event: xcb_present_event_t,
    pub window: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub off_x: i16,
    pub off_y: i16,
    pub full_sequence: u32,
    pub pixmap_width: u16,
    pub pixmap_height: u16,
    pub pixmap_flags: u32,
}

pub const XCB_PRESENT_COMPLETE_NOTIFY: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct xcb_present_complete_notify_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub kind: u8,
    pub mode: u8,
    pub event: xcb_present_event_t,
    pub window: xcb_window_t,
    pub serial: u32,
    pub ust: u64,
    pub full_sequence: u32,
    pub msc: u64,
}

pub const XCB_PRESENT_IDLE_NOTIFY: u8 = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_present_idle_notify_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub pad0: [u8; 2],
    pub event: xcb_present_event_t,
    pub window: xcb_window_t,
    pub serial: u32,
    pub pixmap: xcb_pixmap_t,
    pub idle_fence: xcb_sync_fence_t,
    pub full_sequence: u32,
}

pub const XCB_PRESENT_REDIRECT_NOTIFY: u8 = 3;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct xcb_present_redirect_notify_event_t {
    pub response_type: u8,
    pub extension: u8,
    pub sequence: u16,
    pub length: u32,
    pub event_type: u16,
    pub update_window: u8,
    pub pad0: u8,
    pub event: xcb_present_event_t,
    pub event_window: xcb_window_t,
    pub window: xcb_window_t,
    pub pixmap: xcb_pixmap_t,
    pub serial: u32,
    pub full_sequence: u32,
    pub valid_region: xcb_xfixes_region_t,
    pub update_region: xcb_xfixes_region_t,
    pub valid_rect: xcb_rectangle_t,
    pub update_rect: xcb_rectangle_t,
    pub x_off: i16,
    pub y_off: i16,
    pub target_crtc: xcb_randr_crtc_t,
    pub wait_fence: xcb_sync_fence_t,
    pub idle_fence: xcb_sync_fence_t,
    pub options: u32,
    pub pad1: [u8; 4],
    pub target_msc: u64,
    pub divisor: u64,
    pub remainder: u64,
}

impl XcbPresent {
    #[inline]
    pub unsafe fn xcb_present_id(&self) -> *mut xcb_extension_t {
        sym!(self, xcb_present_id)
    }

    #[inline]
    pub unsafe fn xcb_present_notify_next(&self, i: *mut xcb_present_notify_iterator_t) {
        sym!(self, xcb_present_notify_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_present_notify_end(
        &self,
        i: *mut xcb_present_notify_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_present_notify_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_present_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_present_query_version_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_present_query_version_reply_t {
        sym!(self, xcb_present_query_version_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_present_query_version(
        &self,
        c: *mut xcb_connection_t,
        major_version: u32,
        minor_version: u32,
    ) -> xcb_present_query_version_cookie_t {
        sym!(self, xcb_present_query_version)(c, major_version, minor_version)
    }

    #[inline]
    pub unsafe fn xcb_present_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        major_version: u32,
        minor_version: u32,
    ) -> xcb_present_query_version_cookie_t {
        sym!(self, xcb_present_query_version_unchecked)(c, major_version, minor_version)
    }

    #[inline]
    pub unsafe fn xcb_present_pixmap(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        pixmap: xcb_pixmap_t,
        serial: u32,
        valid: xcb_xfixes_region_t,
        update: xcb_xfixes_region_t,
        x_off: i16,
        y_off: i16,
        target_crtc: xcb_randr_crtc_t,
        wait_fence: xcb_sync_fence_t,
        idle_fence: xcb_sync_fence_t,
        options: u32,
        target_msc: u64,
        divisor: u64,
        remainder: u64,
        notifies_len: u32,
        notifies: *const xcb_present_notify_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_present_pixmap)(
            c,
            window,
            pixmap,
            serial,
            valid,
            update,
            x_off,
            y_off,
            target_crtc,
            wait_fence,
            idle_fence,
            options,
            target_msc,
            divisor,
            remainder,
            notifies_len,
            notifies,
        )
    }

    #[inline]
    pub unsafe fn xcb_present_pixmap_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        pixmap: xcb_pixmap_t,
        serial: u32,
        valid: xcb_xfixes_region_t,
        update: xcb_xfixes_region_t,
        x_off: i16,
        y_off: i16,
        target_crtc: xcb_randr_crtc_t,
        wait_fence: xcb_sync_fence_t,
        idle_fence: xcb_sync_fence_t,
        options: u32,
        target_msc: u64,
        divisor: u64,
        remainder: u64,
        notifies_len: u32,
        notifies: *const xcb_present_notify_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_present_pixmap_checked)(
            c,
            window,
            pixmap,
            serial,
            valid,
            update,
            x_off,
            y_off,
            target_crtc,
            wait_fence,
            idle_fence,
            options,
            target_msc,
            divisor,
            remainder,
            notifies_len,
            notifies,
        )
    }

    #[inline]
    pub unsafe fn xcb_present_notify_msc(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        serial: u32,
        target_msc: u64,
        divisor: u64,
        remainder: u64,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_present_notify_msc)(c, window, serial, target_msc, divisor, remainder)
    }

    #[inline]
    pub unsafe fn xcb_present_notify_msc_checked(
        &self,
        c: *mut xcb_connection_t,
        window: xcb_window_t,
        serial: u32,
        target_msc: u64,
        divisor: u64,
        remainder: u64,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_present_notify_msc_checked)(
            c, window, serial, target_msc, divisor, remainder,
        )
    }

    #[inline]
    pub unsafe fn xcb_present_event_next(&self, i: *mut xcb_present_event_iterator_t) {
        sym!(self, xcb_present_event_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_present_event_end(
        &self,
        i: *mut xcb_present_event_iterator_t,
    ) -> xcb_generic_iterator_t {
        sym!(self, xcb_present_event_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_present_select_input(
        &self,
        c: *mut xcb_connection_t,
        eid: xcb_present_event_t,
        window: xcb_window_t,
        event_mask: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_present_select_input)(c, eid, window, event_mask)
    }

    #[inline]
    pub unsafe fn xcb_present_select_input_checked(
        &self,
        c: *mut xcb_connection_t,
        eid: xcb_present_event_t,
        window: xcb_window_t,
        event_mask: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_present_select_input_checked)(c, eid, window, event_mask)
    }

    #[inline]
    pub unsafe fn xcb_present_query_capabilities_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_present_query_capabilities_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_present_query_capabilities_reply_t {
        sym!(self, xcb_present_query_capabilities_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_present_query_capabilities(
        &self,
        c: *mut xcb_connection_t,
        target: u32,
    ) -> xcb_present_query_capabilities_cookie_t {
        sym!(self, xcb_present_query_capabilities)(c, target)
    }

    #[inline]
    pub unsafe fn xcb_present_query_capabilities_unchecked(
        &self,
        c: *mut xcb_connection_t,
        target: u32,
    ) -> xcb_present_query_capabilities_cookie_t {
        sym!(self, xcb_present_query_capabilities_unchecked)(c, target)
    }

    #[inline]
    pub unsafe fn xcb_present_redirect_notify_notifies(
        &self,
        R: *const xcb_present_redirect_notify_event_t,
    ) -> *mut xcb_present_notify_t {
        sym!(self, xcb_present_redirect_notify_notifies)(R)
    }

    #[inline]
    pub unsafe fn xcb_present_redirect_notify_notifies_length(
        &self,
        R: *const xcb_present_redirect_notify_event_t,
    ) -> c_int {
        sym!(self, xcb_present_redirect_notify_notifies_length)(R)
    }

    #[inline]
    pub unsafe fn xcb_present_redirect_notify_notifies_iterator(
        &self,
        R: *const xcb_present_redirect_notify_event_t,
    ) -> xcb_present_notify_iterator_t {
        sym!(self, xcb_present_redirect_notify_notifies_iterator)(R)
    }
}
