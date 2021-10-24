use crate::*;
use std::os::raw::*;

pub const XCB_DAMAGE_MAJOR_VERSION: u32 = 1;
pub const XCB_DAMAGE_MINOR_VERSION: u32 = 1;

pub type xcb_damage_damage_t = u32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_damage_damage_iterator_t {
    pub data: *mut xcb_damage_damage_t,
    pub rem: c_int,
    pub index: c_int,
}

pub type xcb_damage_report_level_t = u32;
pub const XCB_DAMAGE_REPORT_LEVEL_RAW_RECTANGLES: xcb_damage_report_level_t = 0x00;
pub const XCB_DAMAGE_REPORT_LEVEL_DELTA_RECTANGLES: xcb_damage_report_level_t = 0x01;
pub const XCB_DAMAGE_REPORT_LEVEL_BOUNDING_BOX: xcb_damage_report_level_t = 0x02;
pub const XCB_DAMAGE_REPORT_LEVEL_NON_EMPTY: xcb_damage_report_level_t = 0x03;

pub const XCB_DAMAGE_BAD_DAMAGE: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_damage_bad_damage_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}

pub const XCB_DAMAGE_QUERY_VERSION: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_damage_query_version_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub client_major_version: u32,
    pub client_minor_version: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_damage_query_version_cookie_t {
    pub sequence: c_uint,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_damage_query_version_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u32,
    pub minor_version: u32,
    pub pad1: [u8; 16],
}

pub const XCB_DAMAGE_CREATE: u8 = 1;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_damage_create_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub damage: xcb_damage_damage_t,
    pub drawable: xcb_drawable_t,
    pub level: u8,
    pub pad0: [u8; 3],
}

pub const XCB_DAMAGE_DESTROY: u8 = 2;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_damage_destroy_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub damage: xcb_damage_damage_t,
}

pub const XCB_DAMAGE_SUBTRACT: u8 = 3;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_damage_subtract_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub damage: xcb_damage_damage_t,
    pub repair: xcb_xfixes_region_t,
    pub parts: xcb_xfixes_region_t,
}

pub const XCB_DAMAGE_ADD: u8 = 4;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_damage_add_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length: u16,
    pub drawable: xcb_drawable_t,
    pub region: xcb_xfixes_region_t,
}

pub const XCB_DAMAGE_NOTIFY: u8 = 0;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_damage_notify_event_t {
    pub response_type: u8,
    pub level: u8,
    pub sequence: u16,
    pub drawable: xcb_drawable_t,
    pub damage: xcb_damage_damage_t,
    pub timestamp: xcb_timestamp_t,
    pub area: xcb_rectangle_t,
    pub geometry: xcb_rectangle_t,
}

impl XcbDamage {
    #[inline]
    pub fn xcb_damage_id(&self) -> *mut xcb_extension_t {
        call!(self, xcb_damage_id)
    }

    #[inline]
    pub unsafe fn xcb_damage_damage_next(&self, i: *mut xcb_damage_damage_iterator_t) {
        call!(self, xcb_damage_damage_next)(i);
    }

    #[inline]
    pub unsafe fn xcb_damage_damage_end(
        &self,
        i: *mut xcb_damage_damage_iterator_t,
    ) -> xcb_generic_iterator_t {
        call!(self, xcb_damage_damage_end)(i)
    }

    #[inline]
    pub unsafe fn xcb_damage_query_version_reply(
        &self,
        c: *mut xcb_connection_t,
        cookie: xcb_damage_query_version_cookie_t,
        error: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_damage_query_version_reply_t {
        call!(self, xcb_damage_query_version_reply)(c, cookie, error)
    }

    #[inline]
    pub unsafe fn xcb_damage_query_version(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u32,
        client_minor_version: u32,
    ) -> xcb_damage_query_version_cookie_t {
        call!(self, xcb_damage_query_version)(c, client_major_version, client_minor_version)
    }

    #[inline]
    pub unsafe fn xcb_damage_query_version_unchecked(
        &self,
        c: *mut xcb_connection_t,
        client_major_version: u32,
        client_minor_version: u32,
    ) -> xcb_damage_query_version_cookie_t {
        call!(self, xcb_damage_query_version_unchecked)(
            c,
            client_major_version,
            client_minor_version,
        )
    }

    #[inline]
    pub unsafe fn xcb_damage_create(
        &self,
        c: *mut xcb_connection_t,
        damage: xcb_damage_damage_t,
        drawable: xcb_drawable_t,
        level: u8,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_damage_create)(c, damage, drawable, level)
    }

    #[inline]
    pub unsafe fn xcb_damage_create_checked(
        &self,
        c: *mut xcb_connection_t,
        damage: xcb_damage_damage_t,
        drawable: xcb_drawable_t,
        level: u8,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_damage_create_checked)(c, damage, drawable, level)
    }

    #[inline]
    pub unsafe fn xcb_damage_destroy(
        &self,
        c: *mut xcb_connection_t,
        damage: xcb_damage_damage_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_damage_destroy)(c, damage)
    }

    #[inline]
    pub unsafe fn xcb_damage_destroy_checked(
        &self,
        c: *mut xcb_connection_t,
        damage: xcb_damage_damage_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_damage_destroy_checked)(c, damage)
    }

    #[inline]
    pub unsafe fn xcb_damage_subtract(
        &self,
        c: *mut xcb_connection_t,
        damage: xcb_damage_damage_t,
        repair: xcb_xfixes_region_t,
        parts: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_damage_subtract)(c, damage, repair, parts)
    }

    #[inline]
    pub unsafe fn xcb_damage_subtract_checked(
        &self,
        c: *mut xcb_connection_t,
        damage: xcb_damage_damage_t,
        repair: xcb_xfixes_region_t,
        parts: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_damage_subtract_checked)(c, damage, repair, parts)
    }

    #[inline]
    pub unsafe fn xcb_damage_add(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        region: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_damage_add)(c, drawable, region)
    }

    #[inline]
    pub unsafe fn xcb_damage_add_checked(
        &self,
        c: *mut xcb_connection_t,
        drawable: xcb_drawable_t,
        region: xcb_xfixes_region_t,
    ) -> xcb_void_cookie_t {
        call!(self, xcb_damage_add_checked)(c, drawable, region)
    }
}

pub struct XcbDamage {
    pub(crate) lib: NamedLibrary,
    pub(crate) xcb_damage_id: LazySymbol<*mut xcb_extension_t>,
    pub(crate) xcb_damage_damage_next: LazySymbol<unsafe fn(i: *mut xcb_damage_damage_iterator_t)>,
    pub(crate) xcb_damage_damage_end:
        LazySymbol<unsafe fn(i: *mut xcb_damage_damage_iterator_t) -> xcb_generic_iterator_t>,
    pub(crate) xcb_damage_query_version_reply: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            cookie: xcb_damage_query_version_cookie_t,
            error: *mut *mut xcb_generic_error_t,
        ) -> *mut xcb_damage_query_version_reply_t,
    >,
    pub(crate) xcb_damage_query_version: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u32,
            client_minor_version: u32,
        ) -> xcb_damage_query_version_cookie_t,
    >,
    pub(crate) xcb_damage_query_version_unchecked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            client_major_version: u32,
            client_minor_version: u32,
        ) -> xcb_damage_query_version_cookie_t,
    >,
    pub(crate) xcb_damage_create: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            damage: xcb_damage_damage_t,
            drawable: xcb_drawable_t,
            level: u8,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_damage_create_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            damage: xcb_damage_damage_t,
            drawable: xcb_drawable_t,
            level: u8,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_damage_destroy: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, damage: xcb_damage_damage_t) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_damage_destroy_checked: LazySymbol<
        unsafe fn(c: *mut xcb_connection_t, damage: xcb_damage_damage_t) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_damage_subtract: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            damage: xcb_damage_damage_t,
            repair: xcb_xfixes_region_t,
            parts: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_damage_subtract_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            damage: xcb_damage_damage_t,
            repair: xcb_xfixes_region_t,
            parts: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_damage_add: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            region: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
    pub(crate) xcb_damage_add_checked: LazySymbol<
        unsafe fn(
            c: *mut xcb_connection_t,
            drawable: xcb_drawable_t,
            region: xcb_xfixes_region_t,
        ) -> xcb_void_cookie_t,
    >,
}
