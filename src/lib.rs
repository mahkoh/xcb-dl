#![allow(non_camel_case_types)]

pub use headers::{
    big_requests::*, composite::*, damage::*, dpms::*, dri2::*, dri3::*, ext::*, genericevent::*,
    glx::*, present::*, randr::*, record::*, render::*, res::*, screensaver::*, selinux::*,
    shape::*, shm::*, sync::*, test::*, x_print::*, xc_misc::*, xcb::*, xevie::*, xf86dri::*,
    xf86vidmode::*, xfixes::*, xinerama::*, xinput::*, xkb::*, xproto::*, xv::*, xvmc::*,
};
pub use libs::xcb::Xcb;

#[macro_use]
mod macros;
mod headers;
mod lazy;
mod libs;
