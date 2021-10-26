//#![deny(missing_docs)]

use std::borrow::{Borrow, BorrowMut};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Index, IndexMut};

const U16_ARRAY_SIZE: usize = u16::MAX as usize + 1;

pub struct U16IndexableArray<T> {
    inner: [T; U16_ARRAY_SIZE],
}

impl<T: Copy> U16IndexableArray<T> {
    pub fn new_with_default_value(default_value: T) -> U16IndexableArray<T> {
        Self {
            inner: [default_value; U16_ARRAY_SIZE],
        }
    }
}

impl<T: Copy> Copy for U16IndexableArray<T> {}

impl<T: Debug> Debug for U16IndexableArray<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T: Clone> Clone for U16IndexableArray<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> IntoIterator for U16IndexableArray<T> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, U16_ARRAY_SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a U16IndexableArray<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut U16IndexableArray<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}

impl<T: PartialEq> PartialEq for U16IndexableArray<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<T: Eq> Eq for U16IndexableArray<T> {}

impl<T: PartialOrd> PartialOrd for U16IndexableArray<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<T: Ord> Ord for U16IndexableArray<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T: Hash> Hash for U16IndexableArray<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl<T> AsRef<[T]> for U16IndexableArray<T> {
    fn as_ref(&self) -> &[T] {
        self.inner.as_ref()
    }
}

impl<T> AsMut<[T]> for U16IndexableArray<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.inner.as_mut()
    }
}

impl<T> Borrow<[T]> for U16IndexableArray<T> {
    fn borrow(&self) -> &[T] {
        self.inner.borrow()
    }
}

impl<T> BorrowMut<[T]> for U16IndexableArray<T> {
    fn borrow_mut(&mut self) -> &mut [T] {
        self.inner.borrow_mut()
    }
}

impl<T> Index<u16> for U16IndexableArray<T> {
    type Output = <[T] as Index<usize>>::Output;

    fn index(&self, index: u16) -> &Self::Output {
        // SAFETY: We know that the inner array has u16::MAX+1 items, which means it has an item
        // for every possible index of type `u8`.
        unsafe { self.inner.get_unchecked(index as usize) }
    }
}

impl<T> IndexMut<u16> for U16IndexableArray<T> {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        // SAFETY: We know that the inner array has u16::MAX+1 items, which means it has an item
        // for every possible index of type `u16`.
        unsafe { self.inner.get_unchecked_mut(index as usize) }
    }
}

// TODO: Get rid of the Copy contraint
impl<T> Default for U16IndexableArray<T>
where
    T: Default,
    T: Copy,
{
    fn default() -> Self {
        Self::new_with_default_value(T::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u16_works() {
        let mut array = U16IndexableArray::new_with_default_value(0);
        for i in 0..=u16::MAX {
            array[i] = i;
        }
        for i in 0..=u16::MAX {
            assert_eq!(i, array[i]);
        }
    }
}
