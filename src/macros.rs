macro_rules! call {
    ($self:expr, $f:ident) => {
        ($self.$f.get(&$self.lib, concat!(stringify!($f), "\0")))
    };
}

macro_rules! init_lib {
    ($lib:expr, $name:expr) => {
        unsafe {
            let slf = std::mem::MaybeUninit::zeroed();
            std::ptr::write(
                std::ptr::addr_of_mut!((*slf.as_mut_ptr()).lib),
                crate::lazy::NamedLibrary {
                    name: $name,
                    lib: $lib,
                },
            );
            slf.assume_init()
        }
    };
}
