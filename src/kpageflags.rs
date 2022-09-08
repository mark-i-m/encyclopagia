//! Tools for reading `/proc/kpageflags`.

mod flags;
mod read;

use std::{
    marker::PhantomData,
    ops::{BitOr, BitOrAssign},
};

pub use flags::{
    Flaggy, KPF3_10_0, KPF4_15_0, KPF5_0_8, KPF5_13_0, KPF5_15_0, KPF5_17_0, KPF5_4_0,
};
pub use read::{KPageFlagsIterator, KPageFlagsReader};

use crate::FileReadable;

/// The file path... `/proc/kpageflags`.
pub const KPAGEFLAGS_PATH: &str = "/proc/kpageflags";

/// Represents the flags for a single physical page frame.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct KPageFlags<K: Flaggy>(u64, PhantomData<K>);

impl<K: Flaggy> KPageFlags<K> {
    /// Returns an empty set of flags.
    pub fn empty() -> Self {
        KPageFlags(0, PhantomData)
    }

    /// Returns `true` if all bits in the given mask are set and `false` if any bits are not set.
    pub fn all(&self, mask: u64) -> bool {
        self.0 & mask == mask
    }

    /// Returns `true` if the given KPF bit is set; `false` otherwise.
    pub fn has(&self, kpf: K) -> bool {
        self.all(1 << kpf.into())
    }

    /// Returns `true` if _consecutive_ regions with flags `first` and then `second` can be
    /// combined into one big region.
    pub fn can_combine(first: Self, second: Self) -> bool {
        // Combine identical sets of pages.
        if first == second {
            return true;
        }

        // Combine compound head and compound tail pages.
        if first.has(K::COMPOUND_HEAD) && second.has(K::COMPOUND_TAIL) {
            return true;
        }

        false
    }

    /// Clear all bits set in the `mask` from this `KPageFlags`.
    pub fn clear(&mut self, mask: u64) {
        self.0 &= !mask;
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }
}

unsafe impl<K: Flaggy> FileReadable for KPageFlags<K> {}

impl<K: Flaggy> BitOr for KPageFlags<K> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        KPageFlags(self.0 | rhs.0, PhantomData)
    }
}

impl<K: Flaggy> BitOrAssign for KPageFlags<K> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl<K: Flaggy> From<K> for KPageFlags<K> {
    fn from(kpf: K) -> Self {
        KPageFlags(1 << kpf.into(), PhantomData)
    }
}

impl<K: Flaggy> std::fmt::Display for KPageFlags<K> {
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
