use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;

pub const STICKY_REF_COUNT: u32 = u32::MAX;

#[repr(C)]
struct PerceusBox<T> {
    ref_count: u32,
    value: T,
}

pub struct PerceusPtr<T> {
    ptr: NonNull<PerceusBox<T>>,
}

impl<T> PerceusPtr<T> {
    /// Allocate a new PerceusPtr.
    pub fn new(value: T) -> Self {
        let layout = Layout::new::<PerceusBox<T>>();
        unsafe {
            let ptr = alloc(layout) as *mut PerceusBox<T>;
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            std::ptr::write(
                ptr,
                PerceusBox {
                    ref_count: 1,
                    value,
                },
            );
            PerceusPtr {
                ptr: NonNull::new_unchecked(ptr),
            }
        }
    }

    /// Check if we are the unique owner.
    pub fn is_unique(&self) -> bool {
        unsafe { (*self.ptr.as_ptr()).ref_count == 1 }
    }

    /// Check if the reference counter has reached the sticky threshold.
    pub fn is_sticky(&self) -> bool {
        unsafe { (*self.ptr.as_ptr()).ref_count == STICKY_REF_COUNT }
    }

    /// Duplicate the pointer (Perceus `dup` instruction).
    pub fn dup(&self) -> Self {
        unsafe {
            let box_ptr = self.ptr.as_ptr();
            let ref_count = (*box_ptr).ref_count;
            if ref_count != STICKY_REF_COUNT {
                (*box_ptr).ref_count = ref_count.saturating_add(1);
            }
        }
        PerceusPtr { ptr: self.ptr }
    }

    /// Make mutable (FBIP logic).
    /// If unique, returns a mutable reference to the data,
    /// allowing in-place mutation without allocation.
    pub fn make_mut(&mut self) -> Option<&mut T> {
        if self.is_unique() {
            unsafe { Some(&mut (*self.ptr.as_ptr()).value) }
        } else {
            None
        }
    }

    /// Get a shared reference to the inner value.
    pub fn get(&self) -> &T {
        unsafe { &(*self.ptr.as_ptr()).value }
    }

    /// Intentionally leak memory by making this pointer sticky.
    /// Useful for static data or when we want to bypass the RC overhead
    /// completely.
    pub fn make_sticky(&mut self) {
        unsafe {
            (*self.ptr.as_ptr()).ref_count = STICKY_REF_COUNT;
        }
    }
}

impl<T> Drop for PerceusPtr<T> {
    fn drop(&mut self) {
        unsafe {
            let box_ptr = self.ptr.as_ptr();
            let ref_count = (*box_ptr).ref_count;

            if ref_count != STICKY_REF_COUNT {
                let new_count = ref_count - 1;
                (*box_ptr).ref_count = new_count;

                if new_count == 0 {
                    // Drop the inner value
                    std::ptr::drop_in_place(&mut (*box_ptr).value);
                    // Deallocate the memory
                    let layout = Layout::new::<PerceusBox<T>>();
                    dealloc(box_ptr as *mut u8, layout);
                }
            }
        }
    }
}

impl<T> Clone for PerceusPtr<T> {
    fn clone(&self) -> Self {
        self.dup()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocation_and_drop() {
        let ptr = PerceusPtr::new(42);
        assert!(ptr.is_unique());
        assert_eq!(*ptr.get(), 42);
        // ptr is dropped here automatically
    }

    #[test]
    fn test_dup() {
        let ptr1 = PerceusPtr::new(10);
        let ptr2 = ptr1.dup();
        assert!(!ptr1.is_unique());
        assert!(!ptr2.is_unique());
        assert_eq!(*ptr1.get(), 10);

        drop(ptr1);
        assert!(ptr2.is_unique());
        assert_eq!(*ptr2.get(), 10);
    }

    #[test]
    fn test_fbip_mutation() {
        let mut ptr1 = PerceusPtr::new(100);

        // Is unique, can mutate in place
        if let Some(val) = ptr1.make_mut() {
            *val = 200;
        }
        assert_eq!(*ptr1.get(), 200);

        let mut ptr2 = ptr1.dup();
        // Not unique, cannot mutate in place
        assert!(ptr1.make_mut().is_none());
        assert!(ptr2.make_mut().is_none());
        assert_eq!(*ptr2.get(), 200);
        
        drop(ptr1);
        
        // ptr2 is now unique again
        assert!(ptr2.make_mut().is_some());
    }

    #[test]
    fn test_sticky_sharing() {
        let mut ptr = PerceusPtr::new(42);
        
        // Artificially saturate the counter
        ptr.make_sticky();
        assert!(ptr.is_sticky());

        let ptr2 = ptr.dup();
        assert!(ptr2.is_sticky());
        assert!(ptr.is_sticky());

        drop(ptr2);
        // Should still be sticky and not deallocated
        assert!(ptr.is_sticky());
        assert_eq!(*ptr.get(), 42);

        // Reset so it doesn't actually leak during unit tests
        unsafe {
            (*ptr.ptr.as_ptr()).ref_count = 1;
        }
    }
}
