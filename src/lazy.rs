use libloading::Library;
use std::marker::PhantomData;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::{mem, ptr};

pub struct NamedLibrary {
    pub name: &'static str,
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
        let mut sym = self.sym.load(Relaxed);
        if sym == 0 {
            sym = load_symbol::<T>(lib, name);
            self.sym.store(sym, Relaxed);
        }
        ptr::read(&sym as *const usize as *const T)
    }
}

#[cold]
unsafe fn load_symbol<T>(lib: &NamedLibrary, mut name: &str) -> usize {
    assert_eq!(mem::size_of::<T>(), mem::size_of::<usize>());
    assert!(mem::align_of::<T>() <= mem::align_of::<usize>());
    let err = match lib.lib.get::<usize>(name.as_bytes()) {
        Ok(sym) => return sym.into_raw().into_raw() as usize,
        Err(e) => e,
    };
    if name.ends_with('\0') {
        name = &name[..name.len() - 1];
    }
    panic!("Cannot load `{}` from `{}`: {:?}", name, lib.name, err);
}
