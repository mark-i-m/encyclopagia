//! Tools for reading `/proc/[pid]/pagemap`.

use std::{
    marker::PhantomData,
    ops::{BitOr, BitOrAssign},
    str::FromStr,
};

use crate::{FileReadable, FileReadableReader};

/// All the different pagemap implementations are `PageMappy`.
pub trait PageMappy:
    Sized + FromStr + Copy + std::fmt::Debug + std::hash::Hash + Ord + Eq + Into<u64> + From<u64>
{
    const PRESENT: Self;
    const SWAPPED: Self;
    const FILE_OR_SHM: Self;
    const EXCLUSIVE: Option<Self>;
    const SOFT_DIRTY: Option<Self>;

    fn valid(val: u64) -> bool;
    fn values() -> &'static [u64];

    fn valid_mask() -> u64 {
        let mut v = 0;
        for b in Self::values() {
            v |= 1 << b;
        }
        v
    }

    /// Returns a mask for the location information of the page.
    fn location_mask() -> u64;
}

/// Represents the flags for a single virtual page in the address space of a process as given by
/// that process's pagemap.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PageMapPage<K: PageMappy>(u64, PhantomData<K>);

impl<K: PageMappy> PageMapPage<K> {
    /// Returns an empty set of flags.
    pub fn empty() -> Self {
        PageMapPage(0, PhantomData)
    }

    /// Returns `true` if all bits in the given mask are set and `false` if any bits are not set.
    pub fn all(&self, mask: u64) -> bool {
        self.0 & mask == mask
    }

    /// Returns `true` if the given KPF bit is set; `false` otherwise.
    pub fn has(&self, flag: K) -> bool {
        self.all(1 << flag.into())
    }

    /// Clear all bits set in the `mask` from this `PageMapPage`.
    pub fn clear(&mut self, mask: u64) {
        self.0 &= !mask;
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }

    pub fn location(self) -> u64 {
        let mask = K::location_mask();
        let shift = mask.trailing_zeros();
        (self.0 & mask) >> shift
    }
}

unsafe impl<K: PageMappy> FileReadable for PageMapPage<K> {}

impl<K: PageMappy> BitOr for PageMapPage<K> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        PageMapPage(self.0 | rhs.0, PhantomData)
    }
}

impl<K: PageMappy> BitOrAssign for PageMapPage<K> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl<K: PageMappy> From<K> for PageMapPage<K> {
    fn from(kpf: K) -> Self {
        PageMapPage(1 << kpf.into(), PhantomData)
    }
}

impl<K: PageMappy> std::fmt::Display for PageMapPage<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for fi in K::values() {
            if self.all(1 << fi) {
                write!(f, "{:?} ", K::from(*fi))?;
            }
        }

        let invalid_bits = self.0 & !K::valid_mask();
        if invalid_bits != 0 {
            write!(f, "INVALID BITS: {invalid_bits:X?}")?;
        }

        Ok(())
    }
}

/// Wrapper around a `Read` type that for the `/proc/[pid]/pagemap` file.
pub type PageMapReader<R, K> = FileReadableReader<R, PageMapPage<K>>;

// TODO: implement PageMappy for a few kernels...
