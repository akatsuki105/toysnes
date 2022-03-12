use std::{any::TypeId, mem};

pub unsafe fn malloc(len: usize) -> *mut u8 {
    let mut vec = Vec::<u8>::with_capacity(len);
    vec.set_len(len);
    Box::into_raw(vec.into_boxed_slice()) as *mut u8
}

pub unsafe fn free(raw: *mut u8, len: usize) {
    let s = std::slice::from_raw_parts_mut(raw, len);
    let _ = Box::from_raw(s);
}

pub struct EvilPointer {
    ptr: *mut u8,
    type_id: TypeId,
    destroy: fn(*mut u8) -> (),
}

impl EvilPointer {
    pub fn new<T: 'static>(value: T) -> EvilPointer {
        EvilPointer {
            ptr: unsafe { mem::transmute(Box::new(value)) },
            type_id: TypeId::of::<T>(),
            destroy: |ptr| {
                unsafe { mem::transmute::<*mut u8, Box<T>>(ptr) };
            },
        }
    }

    pub fn rob<T: 'static>(&mut self) -> Option<T> {
        match self.ptr.is_null() {
            true => None, // When ptr is null return None
            false => match TypeId::of::<T>() == self.type_id {
                true => {
                    // When types match

                    // Transmute into returned value and set internal pointer to
                    // null, so we avoid owning same value in several places.

                    let result: Box<T> = unsafe { mem::transmute(self.ptr) };
                    self.ptr = std::ptr::null_mut();

                    Some(*result) // Unbox and return Some
                }
                false => None, // When types do not match return None
            },
        }
    }
}

impl Drop for EvilPointer {
    fn drop(&mut self) {
        match self.ptr.is_null() {
            true => (),
            false => (self.destroy)(self.ptr),
        }
    }
}
