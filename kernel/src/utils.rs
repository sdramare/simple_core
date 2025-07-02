use core::cell::UnsafeCell;

pub struct Global<T> {
    value: UnsafeCell<Option<T>>,
}

impl<T> Global<T> {
    pub const fn uninit() -> Self {
        Global {
            value: UnsafeCell::new(Option::None),
        }
    }

    /// Replaces the value, returning the old without dropping either.
    pub fn set(&self, value: T) -> Option<T> {
        unsafe { self.value.get().replace(Some(value)) }
    }

    pub fn get(&self) -> Option<&mut T> {
        unsafe { self.value.get().as_mut().and_then(|value| value.as_mut()) }
    }
}

unsafe impl<T> Send for Global<T> where T: Send {}
unsafe impl<T> Sync for Global<T> where T: Send + Sync {}
