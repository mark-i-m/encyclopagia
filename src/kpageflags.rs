//! Tools for reading `/proc/kpageflags`.

mod flags;
mod read;

use std::ops::{BitOr, BitOrAssign};

pub use flags::{
    Flaggy, KPF3_10_0, KPF4_15_0, KPF5_0_8, KPF5_13_0, KPF5_15_0, KPF5_17_0, KPF5_4_0, KPF6_0_0,
};
pub use read::{KPageFlagsIterator, KPageFlagsReader};

use crate::FileReadable;

/// The file path... `/proc/kpageflags`.
pub const KPAGEFLAGS_PATH: &str = "/proc/kpageflags";

/// Represents the flags for a single physical page frame.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct KPageFlags<K: Flaggy>(K);

impl<K: Flaggy> KPageFlags<K> {
    /// Returns an empty set of flags.
    pub fn empty() -> Self {
        KPageFlags(K::from(0))
    }

    /// Returns `true` if all bits in the given mask are set and `false` if any bits are not set.
    pub fn all(&self, mask: K) -> bool {
        self.0 & mask == mask
    }

    /// Returns `true` if any bits in the given mask are set and `false` if all bits are not set.
    pub fn any(&self, mask: K) -> bool {
        self.0 & mask != K::empty()
    }

    /// Returns `true` if _consecutive_ regions with flags `first` and then `second` can be
    /// combined into one big region.
    pub fn can_combine(first: Self, second: Self) -> bool {
        // Combine identical sets of pages.
        if first == second {
            return true;
        }

        // Combine compound head and compound tail pages.
        if first.all(K::COMPOUND_HEAD) && second.all(K::COMPOUND_TAIL) {
            return true;
        }

        false
    }

    /// Clear all bits set in the `mask` from this `KPageFlags`.
    pub fn clear(&mut self, mask: K) {
        self.0 &= !mask;
    }

    pub fn as_u64(self) -> u64 {
        self.0.into()
    }
}

unsafe impl<K: Flaggy> FileReadable for KPageFlags<K> {}

impl<K: Flaggy> BitOr for KPageFlags<K> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        KPageFlags(self.0 | rhs.0)
    }
}

impl<K: Flaggy> BitOrAssign for KPageFlags<K> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl<K: Flaggy> From<K> for KPageFlags<K> {
    fn from(kpf: K) -> Self {
        KPageFlags(kpf)
    }
}

impl<K: Flaggy> std::fmt::Display for KPageFlags<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for fi in K::values() {
            if self.all(*fi) {
                write!(f, "{:?} ", *fi)?;
            }
        }

        let invalid_bits = self.0 & !K::valid_mask();
        if invalid_bits != K::empty() {
            write!(f, "INVALID BITS: {invalid_bits:X?}")?;
        }

        Ok(())
    }
}
