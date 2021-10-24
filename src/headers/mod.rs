pub use {
    big_requests::*, composite::*, damage::*, dpms::*, dri2::*, dri3::*, ext::*, genericevent::*,
    glx::*, present::*, randr::*, record::*, render::*, res::*, screensaver::*, selinux::*,
    shape::*, shm::*, sync::*, test::*, x_print::*, xc_misc::*, xcb::*, xevie::*, xf86dri::*,
    xfixes::*, xinerama::*, xinput::*, xkb::*, xproto::*, xv::*, xvmc::*,
};

mod ext;
mod xcb;

#[path = "generated/ffi/big_requests.rs"]
mod big_requests;
#[path = "generated/ffi/composite.rs"]
mod composite;
#[path = "generated/ffi/damage.rs"]
mod damage;
#[path = "generated/ffi/dpms.rs"]
mod dpms;
#[path = "generated/ffi/dri2.rs"]
mod dri2;
#[path = "generated/ffi/dri3.rs"]
mod dri3;
#[path = "generated/ffi/genericevent.rs"]
mod genericevent;
#[path = "generated/ffi/glx.rs"]
mod glx;
#[path = "generated/ffi/present.rs"]
mod present;
#[path = "generated/ffi/randr.rs"]
mod randr;
#[path = "generated/ffi/record.rs"]
mod record;
#[path = "generated/ffi/render.rs"]
mod render;
#[path = "generated/ffi/res.rs"]
mod res;
#[path = "generated/ffi/screensaver.rs"]
mod screensaver;
#[path = "generated/ffi/selinux.rs"]
mod selinux;
#[path = "generated/ffi/shape.rs"]
mod shape;
#[path = "generated/ffi/shm.rs"]
mod shm;
#[path = "generated/ffi/sync.rs"]
mod sync;
#[path = "generated/ffi/test.rs"]
mod test;
#[path = "generated/ffi/x_print.rs"]
mod x_print;
#[path = "generated/ffi/xc_misc.rs"]
mod xc_misc;
#[path = "generated/ffi/xevie.rs"]
mod xevie;
#[path = "generated/ffi/xf86dri.rs"]
mod xf86dri;
#[path = "generated/ffi/xfixes.rs"]
mod xfixes;
#[path = "generated/ffi/xinerama.rs"]
mod xinerama;
#[path = "generated/ffi/xinput.rs"]
mod xinput;
#[path = "generated/ffi/xkb.rs"]
mod xkb;
#[path = "generated/ffi/xproto.rs"]
mod xproto;
#[path = "generated/ffi/xv.rs"]
mod xv;
#[path = "generated/ffi/xvmc.rs"]
mod xvmc;
