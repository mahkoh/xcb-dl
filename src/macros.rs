macro_rules! lib_entry {
    ($name:ident, $path:expr) => {
        impl $name {
            /// Loads the library from the default location.
            #[inline]
            pub unsafe fn load() -> Result<Self, libloading::Error> {
                Self::load_from($path)
            }

            /// Loads the library from the specified path.
            #[inline]
            pub unsafe fn load_from(
                path: impl AsRef<std::path::Path>,
            ) -> Result<Self, libloading::Error> {
                let lib = libloading::Library::new(path.as_ref().as_os_str())?;
                let mut slf = std::mem::MaybeUninit::<Self>::zeroed();
                std::ptr::write(
                    std::ptr::addr_of_mut!((*slf.as_mut_ptr()).lib),
                    crate::lazy::NamedLibrary {
                        path: path.as_ref().to_path_buf(),
                        lib,
                    },
                );
                Ok(slf.assume_init())
            }
        }
    };
}
