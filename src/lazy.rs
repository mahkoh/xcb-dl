use libloading::{Error, Library};
use std::marker::PhantomData;
use std::path::PathBuf;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::{mem, ptr};

pub struct NamedLibrary {
    pub path: PathBuf,
    pub lib: Library,
}

pub struct LazySymbol<T> {
    sym: AtomicUsize,
    _phantom: PhantomData<T>,
}

impl<T> Default for LazySymbol<T> {
    fn default() -> Self {
        Self {
            sym: Default::default(),
            _phantom: Default::default(),
        }
    }
}

impl<T> LazySymbol<T> {
    #[inline]
    pub unsafe fn get(&self, lib: &NamedLibrary, name: &str) -> T {
        assert_eq!(mem::size_of::<T>(), mem::size_of::<usize>());
        assert!(mem::align_of::<T>() <= mem::align_of::<usize>());
        let sym = get(&self.sym, lib, name);
        ptr::read(&sym as *const usize as *const T)
    }

    #[inline(never)]
    #[cfg(feature = "has_symbol")]
    pub unsafe fn exists(&self, lib: &NamedLibrary, name: &str) -> bool {
        exists(&self.sym, lib, name)
    }
}

#[inline]
unsafe fn get(asym: &AtomicUsize, lib: &NamedLibrary, name: &str) -> usize {
    let mut sym = asym.load(Relaxed);
    if sym == 0 {
        sym = load_symbol(lib, name);
        asym.store(sym, Relaxed);
    }
    sym
}

#[inline]
#[cfg(feature = "has_symbol")]
unsafe fn exists(asym: &AtomicUsize, lib: &NamedLibrary, name: &str) -> bool {
    if asym.load(Relaxed) != 0 {
        return true;
    }
    match try_load_symbol(lib, name) {
        Ok(sym) => {
            asym.store(sym, Relaxed);
            true
        }
        _ => false,
    }
}

#[cold]
unsafe fn load_symbol(lib: &NamedLibrary, mut name: &str) -> usize {
    let err = match try_load_symbol(lib, name) {
        Ok(sym) => return sym,
        Err(e) => e,
    };
    if name.ends_with('\0') {
        name = &name[..name.len() - 1];
    }
    panic!(
        "Cannot load `{}` from `{}`: {:?}",
        name,
        lib.path.display(),
        err
    );
}

#[cold]
unsafe fn try_load_symbol(lib: &NamedLibrary, name: &str) -> Result<usize, Error> {
    match lib.lib.get::<usize>(name.as_bytes()) {
        Ok(sym) => Ok(sym.into_raw().into_raw() as usize),
        Err(e) => Err(e),
    }
}
