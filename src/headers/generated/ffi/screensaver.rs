use crate::ffi::*;
use crate::*;
use std::os::raw::*;

pub const XCB_SCREENSAVER_MAJOR_VERSION: u32 = 1;
pub const XCB_SCREENSAVER_MINOR_VERSION: u32 = 1;

pub type xcb_screensaver_kind_t = u32;
pub const XCB_SCREENSAVER_KIND_BLANKED: xcb_screensaver_kind_t = 0x00;
pub const XCB_SCREENSAVER_KIND_INTERNAL: xcb_screensaver_kind_t = 0x01;
pub const XCB_SCREENSAVER_KIND_EXTERNAL: xcb_screensaver_kind_t = 0x02;

pub type xcb_screensaver_event_t = u32;
pub const XCB_SCREENSAVER_EVENT_NOTIFY_MASK: xcb_screensaver_event_t = 0x01;
pub const XCB_SCREENSAVER_EVENT_CYCLE_MASK: xcb_screensaver_event_t = 0x02;

pub type xcb_screensaver_state_t = u32;
pub const XCB_SCREENSAVER_STATE_OFF: xcb_screensaver_state_t = 0x00;
pub const XCB_SCREENSAVER_STATE_ON: xcb_screensaver_state_t = 0x01;
pub const XCB_SCREENSAVER_STATE_CYCLE: xcb_screensaver_state_t = 0x02;
pub const XCB_SCREENSAVER_STATE_DISABLED: xcb_screensaver_state_t = 0x03;

pub const XCB_SCREENSAVER_QUERY_VERSION: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screensaver_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub client_major_version: u8,
    pub client_minor_version: u8,
    pub pad0: [u8; 2],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screensaver_query_version_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screensaver_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub server_major_version: u16,
    pub server_minor_version: u16,
    pub pad1: [u8; 20],
}

pub const XCB_SCREENSAVER_QUERY_INFO: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screensaver_query_info_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screensaver_query_info_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screensaver_query_info_reply_t {
    pub response_type: u8,
    pub state: u8,
    pub sequence: u16,
    pub length: u32,
    pub saver_window: xcb_window_t,
    pub ms_until_server: u32,
    pub ms_since_user_input: u32,
    pub event_mask: u32,
    pub kind: u8,
    pub pad0: [u8; 7],
}

pub const XCB_SCREENSAVER_SELECT_INPUT: u8 = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screensaver_select_input_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub event_mask: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screensaver_set_attributes_value_list_t {
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

pub const XCB_SCREENSAVER_SET_ATTRIBUTES: u8 = 3;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screensaver_set_attributes_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub class: u8,
    pub depth: u8,
    pub visual: xcb_visualid_t,
    pub value_mask: u32,
}

pub const XCB_SCREENSAVER_UNSET_ATTRIBUTES: u8 = 4;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screensaver_unset_attributes_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
}

pub const XCB_SCREENSAVER_SUSPEND: u8 = 5;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screensaver_suspend_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub suspend: u32,
}

pub const XCB_SCREENSAVER_NOTIFY: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_screensaver_notify_event_t {
    pub response_type: u8,
    pub state: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub window: xcb_window_t,
    pub kind: u8,
    pub forced: u8,
    pub pad0: [u8; 14],
}

impl XcbScreensaver {
    #[inline]
    pub unsafe fn xcb_screensaver_id(&self) -> *mut xcb_extension_t {
        sym!(self, xcb_screensaver_id)
    }

    #[inline]
    pub unsafe fn xcb_screensaver_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_screensaver_query_version_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_screensaver_query_version_reply_t {
        sym!(self, xcb_screensaver_query_version_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_screensaver_query_version(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u8,
        client_minor_version: u8,
    ) -> xcb_screensaver_query_version_cookie_t {
        sym!(self, xcb_screensaver_query_version)(c, client_major_version, client_minor_version)
    }

    #[inline]
    pub unsafe fn xcb_screensaver_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u8,
        client_minor_version: u8,
    ) -> xcb_screensaver_query_version_cookie_t {
        sym!(self, xcb_screensaver_query_version_unchecked)(
            c,
            client_major_version,
            client_minor_version,
        )
    }

    #[inline]
    pub unsafe fn xcb_screensaver_query_info_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_screensaver_query_info_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_screensaver_query_info_reply_t {
        sym!(self, xcb_screensaver_query_info_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_screensaver_query_info(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_screensaver_query_info_cookie_t {
        sym!(self, xcb_screensaver_query_info)(c, drawable)
    }

    #[inline]
    pub unsafe fn xcb_screensaver_query_info_unchecked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_screensaver_query_info_cookie_t {
        sym!(self, xcb_screensaver_query_info_unchecked)(c, drawable)
    }

    #[inline]
    pub unsafe fn xcb_screensaver_select_input(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        event_mask: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_screensaver_select_input)(c, drawable, event_mask)
    }

    #[inline]
    pub unsafe fn xcb_screensaver_select_input_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        event_mask: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_screensaver_select_input_checked)(c, drawable, event_mask)
    }

    #[inline]
    pub unsafe fn xcb_screensaver_set_attributes(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        class: u8,
        depth: u8,
        visual: xcb_visualid_t,
        value_mask: u32,
        value_list: *const xcb_screensaver_set_attributes_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_screensaver_set_attributes)(
            c,
            drawable,
            x,
            y,
            width,
            height,
            border_width,
            class,
            depth,
            visual,
            value_mask,
            value_list,
        )
    }

    #[inline]
    pub unsafe fn xcb_screensaver_set_attributes_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        class: u8,
        depth: u8,
        visual: xcb_visualid_t,
        value_mask: u32,
        value_list: *const xcb_screensaver_set_attributes_value_list_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_screensaver_set_attributes_checked)(
            c,
            drawable,
            x,
            y,
            width,
            height,
            border_width,
            class,
            depth,
            visual,
            value_mask,
            value_list,
        )
    }

    #[inline]
    pub unsafe fn xcb_screensaver_unset_attributes(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_screensaver_unset_attributes)(c, drawable)
    }

    #[inline]
    pub unsafe fn xcb_screensaver_unset_attributes_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_screensaver_unset_attributes_checked)(c, drawable)
    }

    #[inline]
    pub unsafe fn xcb_screensaver_suspend(
        &self,
        c: *mut xcb_connection_t,
        suspend: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_screensaver_suspend)(c, suspend)
    }

    #[inline]
    pub unsafe fn xcb_screensaver_suspend_checked(
        &self,
        c: *mut xcb_connection_t,
        suspend: u32,
    ) -> xcb_void_cookie_t {
        sym!(self, xcb_screensaver_suspend_checked)(c, suspend)
    }
}
