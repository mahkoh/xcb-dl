use std::time::Duration;
use std::{mem, ptr};
use xcb_dl::ffi::{
    xcb_atom_t, xcb_connection_t, xcb_generic_error_t, xcb_window_t, XCB_ATOM_STRING,
    XCB_PROP_MODE_REPLACE,
};
use xcb_dl::Xcb;

unsafe fn check_err(err: *mut xcb_generic_error_t) {
    if !err.is_null() {
        panic!("{:#?}", *err);
    }
}

unsafe fn get_atom(lib: &Xcb, con: *mut xcb_connection_t, atom: &str) -> xcb_atom_t {
    let mut err = ptr::null_mut();
    let atom_reply = lib.xcb_intern_atom_reply(
        con,
        lib.xcb_intern_atom(con, 0, atom.len() as u16, atom.as_ptr() as _),
        &mut err,
    );
    check_err(err);
    assert!(!atom_reply.is_null());
    let atom = (*atom_reply).atom;
    libc::free(atom_reply as _);
    atom
}

unsafe fn replace_property<T: ?Sized>(
    lib: &Xcb,
    con: *mut xcb_connection_t,
    window: xcb_window_t,
    prop: xcb_atom_t,
    ty: xcb_atom_t,
    t: &T,
) {
    let cookie = lib.xcb_change_property_checked(
        con,
        XCB_PROP_MODE_REPLACE as _,
        window,
        prop,
        ty,
        8,
        mem::size_of_val(t) as _,
        t as *const _ as _,
    );
    check_err(lib.xcb_request_check(con, cookie));
}

unsafe fn create_window(lib: &Xcb, con: *mut xcb_connection_t) -> xcb_window_t {
    let setup = lib.xcb_get_setup(con);
    let screen = &*lib.xcb_setup_roots_iterator(setup).data;
    let window_id = lib.xcb_generate_id(con);
    let cookie = lib.xcb_create_window_checked(
        con,
        screen.root_depth,
        window_id,
        screen.root,
        0,
        0,
        200,
        200,
        0,
        0,
        screen.root_visual,
        0,
        ptr::null(),
    );
    check_err(lib.xcb_request_check(con, cookie));
    window_id
}

fn main() {
    unsafe {
        let lib = Xcb::load().unwrap();
        let con = lib.xcb_connect(ptr::null(), ptr::null_mut());
        assert_eq!(lib.xcb_connection_has_error(con), 0);
        let window_id = create_window(&lib, con);
        let wm_name = get_atom(&lib, con, "WM_NAME");
        replace_property(&lib, con, window_id, wm_name, XCB_ATOM_STRING, "Hello XCB");
        check_err(lib.xcb_request_check(con, lib.xcb_map_window_checked(con, window_id)));
        assert!(lib.xcb_flush(con) > 0);
        std::thread::sleep(Duration::from_secs(5));
    }
}
