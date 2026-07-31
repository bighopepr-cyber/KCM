use std::alloc::{alloc, dealloc, Layout};
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::ptr::NonNull;

use crate::types::KcmError;

pub struct DenseVec<T: Copy> {
    ptr: NonNull<T>,
    capacity: usize,
    len: usize,
    alignment: usize,
    _phantom: PhantomData<T>,
}

// SAFETY: DenseVec manages its own memory via Layout-based alloc/dealloc.
// DenseVec is Send when T is Send: owned memory with no shared references.
// DenseVec is Sync when T is Send+Sync: safe to share references across threads.
unsafe impl<T: Copy + Send> Send for DenseVec<T> {}
unsafe impl<T: Copy + Send + Sync> Sync for DenseVec<T> {}

impl<T: Copy> DenseVec<T> {
    const MIN_ALIGNMENT: usize = 64;

    pub fn new(capacity: usize) -> Result<Self, KcmError> {
        Self::with_alignment(capacity, Self::MIN_ALIGNMENT)
    }

    pub fn with_alignment(capacity: usize, alignment: usize) -> Result<Self, KcmError> {
        if capacity == 0 {
            return Ok(DenseVec {
                ptr: NonNull::dangling(),
                capacity: 0,
                len: 0,
                alignment,
                _phantom: PhantomData,
            });
        }

        let byte_size = capacity
            .checked_mul(std::mem::size_of::<T>())
            .ok_or_else(|| {
                KcmError::InvalidArgument(format!(
                    "Capacity overflow: {} * {} exceeds usize::MAX",
                    capacity,
                    std::mem::size_of::<T>()
                ))
            })?;

        let layout = Layout::from_size_align(byte_size, alignment.max(std::mem::align_of::<T>()))
            .map_err(|e| KcmError::InvalidArgument(format!("Layout error: {}", e)))?;

        let ptr = unsafe { alloc(layout) } as *mut T;
        let ptr = NonNull::new(ptr).ok_or(KcmError::OutOfMemory)?;

        Ok(DenseVec {
            ptr,
            capacity,
            len: 0,
            alignment,
            _phantom: PhantomData,
        })
    }

    pub fn push(&mut self, value: T) -> Result<(), KcmError> {
        if self.len >= self.capacity {
            return Err(KcmError::OutOfMemory);
        }

        unsafe {
            // SAFETY: DenseVec is contiguous memory. push() has already verified
            // that self.len < self.capacity. Writing at self.len is within bounds.
            *self.ptr.as_ptr().add(self.len) = value;
        }
        self.len += 1;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.as_slice().iter()
    }
}

impl<T: Copy> Index<usize> for DenseVec<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.as_slice()[idx]
    }
}

impl<T: Copy> IndexMut<usize> for DenseVec<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.as_mut_slice()[idx]
    }
}

impl<T: Copy> Drop for DenseVec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            // SAFETY: The layout is reconstructed from the same values used during allocation.
            // capacity and size_of::<T>() are the same as when allocated. The multiplication
            // cannot overflow because it succeeded during construction with the same values.
            let byte_size = self.capacity * std::mem::size_of::<T>();
            let layout =
                Layout::from_size_align(byte_size, self.alignment.max(std::mem::align_of::<T>()));
            // SAFETY: Layout::from_size_align can only fail if align is not a power of two
            // or if byte_size overflows. Both are invariant: alignment is stored from a
            // valid construction, and byte_size is the same multiplication that succeeded before.
            if let Ok(layout) = layout {
                unsafe {
                    dealloc(self.ptr.as_ptr() as *mut u8, layout);
                }
            }
            // If layout reconstruction somehow fails, we leak the memory rather than abort.
            // This is the safest behavior: no crash, no UB, just a leak on impossible invariant violation.
        }
    }
}

impl<T: Copy> Clone for DenseVec<T> {
    fn clone(&self) -> Self {
        let mut new_vec = Self::with_alignment(self.capacity, self.alignment)
            .expect("DenseVec::clone: allocation must succeed with same parameters as original");
        new_vec.len = self.len;
        new_vec.as_mut_slice().copy_from_slice(self.as_slice());
        new_vec
    }
}
