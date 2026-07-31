use std::alloc::{alloc, dealloc, Layout};
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::ptr::NonNull;

pub struct DenseVec<T: Copy> {
    ptr: NonNull<T>,
    capacity: usize,
    len: usize,
    alignment: usize,
    _phantom: PhantomData<T>,
}

unsafe impl<T: Copy + Send> Send for DenseVec<T> {}
unsafe impl<T: Copy + Send + Sync> Sync for DenseVec<T> {}

impl<T: Copy> DenseVec<T> {
    const MIN_ALIGNMENT: usize = 64;

    pub fn new(capacity: usize) -> Result<Self, String> {
        Self::with_alignment(capacity, Self::MIN_ALIGNMENT)
    }

    pub fn with_alignment(capacity: usize, alignment: usize) -> Result<Self, String> {
        if capacity == 0 {
            return Ok(DenseVec {
                ptr: NonNull::dangling(),
                capacity: 0,
                len: 0,
                alignment,
                _phantom: PhantomData,
            });
        }

        let layout = Layout::from_size_align(
            capacity * std::mem::size_of::<T>(),
            alignment.max(std::mem::align_of::<T>()),
        )
        .map_err(|e| format!("Layout error: {}", e))?;

        let ptr = unsafe { alloc(layout) } as *mut T;
        let ptr = NonNull::new(ptr).ok_or_else(|| "Allocation failed".to_string())?;

        Ok(DenseVec {
            ptr,
            capacity,
            len: 0,
            alignment,
            _phantom: PhantomData,
        })
    }

    pub fn push(&mut self, value: T) -> Result<(), String> {
        if self.len >= self.capacity {
            return Err("Vector full".to_string());
        }

        unsafe {
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
            let layout = Layout::from_size_align(
                self.capacity * std::mem::size_of::<T>(),
                self.alignment.max(std::mem::align_of::<T>()),
            )
            .unwrap_or_else(|_| std::process::abort());

            unsafe {
                dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl<T: Copy> Clone for DenseVec<T> {
    fn clone(&self) -> Self {
        let mut new_vec = Self::with_alignment(self.capacity, self.alignment)
            .unwrap_or_else(|_| std::process::abort());
        new_vec.len = self.len;
        new_vec.as_mut_slice().copy_from_slice(self.as_slice());
        new_vec
    }
}

#[repr(C, align(64))]
pub struct CacheAlignedData {
    pub data: u64,
    pub padding: [u64; 7],
}

impl CacheAlignedData {
    pub fn new(value: u64) -> Self {
        CacheAlignedData {
            data: value,
            padding: [0u64; 7],
        }
    }
}

impl<T: Copy> DenseVec<T> {
    pub fn new_cache_aligned(capacity: usize) -> Result<Self, String> {
        Self::with_alignment(capacity, 64)
    }
}
