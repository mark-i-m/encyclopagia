//! Machinery for interpretting kpageflags on a few different kernels.

use std::{
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
    str::FromStr,
};

/// All the different KPF implementations are `Flaggy`.
pub trait Flaggy:
    Sized
    + FromStr
    + Copy
    + std::fmt::Debug
    + std::hash::Hash
    + Ord
    + Eq
    + Into<u64>
    + From<u64>
    + BitOr<Output = Self>
    + BitOrAssign
    + BitAnd<Output = Self>
    + BitAndAssign
    + BitXor<Output = Self>
    + BitXorAssign
    + Not<Output = Self>
    + 'static
{
    // Some flags that should be present in all kernel versions.
    const NOPAGE: Self;
    const COMPOUND_HEAD: Self;
    const COMPOUND_TAIL: Self;
    const PGTABLE: Option<Self>;
    const BUDDY: Self;
    const SLAB: Self;
    const RESERVED: Self;
    const MMAP: Self;
    const LRU: Self;
    const ANON: Self;
    const THP: Self;
    const PRIVATE: Self;
    const PRIVATE2: Self;
    const OWNERPRIVATE1: Self;

    fn empty() -> Self;
    fn values() -> &'static [Self];

    fn valid_mask() -> Self {
        Self::values().iter().fold(Self::empty(), |a, b| a | *b)
    }
}

/// Easier to derive `Flaggy` and a bunch of other stuff...
macro_rules! kpf {
    ($kpfname:ident { $($name:ident = $val:literal),+ $(,)? } $($c:ident: $t:ty = $v:expr;)+) => {
        #[allow(non_snake_case)]
        pub mod $kpfname {
            use std::{
                ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
                str::FromStr,
            };
            use crate::kpageflags::Flaggy;

            #[allow(dead_code)]
            #[derive(Copy, Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
            #[repr(transparent)]
            pub struct Flags (u64);

            $(
                #[allow(non_upper_case_globals)]
                pub const $name : Flags = Flags(1 << $val);
            )+

            const _SIZE_CHECK: () = if std::mem::size_of::<u64>() != std::mem::size_of::<Flags>() {
                panic!("KPF size > sizeof(u64)");
            } else { };

            impl FromStr for Flags {
                type Err = String;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    match s {
                        $(
                            stringify!($name) => Ok($name),
                        )+

                        other => Err(format!("unknown flag: {}", other)),
                    }
                }
            }

            impl Flaggy for Flags {
                $(const $c: $t = $v;)+

                fn empty() -> Self {
                    Flags(0)
                }

                fn values() -> &'static [Self] {
                    &[ $($name),* ]
                }
            }

            impl From<Flags> for u64 {
                fn from(kpf: Flags) -> u64 {
                    kpf.0
                }
            }

            impl From<u64> for Flags {
                fn from(val: u64) -> Self {
                    assert_eq!(Self::valid_mask().0 & val, val);
                    unsafe { std::mem::transmute(val) }
                }
            }

            impl BitOr for Flags {
                type Output = Self;
                fn bitor(self, rhs: Self) -> Self {
                    Flags(self.0 | rhs.0)
                }
            }

            impl BitOrAssign for Flags {
                fn bitor_assign(&mut self, rhs: Self) {
                    *self = *self | rhs;
                }
            }

            impl BitAnd for Flags {
                type Output = Self;
                fn bitand(self, rhs: Self) -> Self {
                    Flags(self.0 & rhs.0)
                }
            }

            impl BitAndAssign for Flags {
                fn bitand_assign(&mut self, rhs: Self)  {
                    *self = *self & rhs;
                }
            }

            impl BitXor for Flags {
                type Output = Self;
                fn bitxor(self, rhs: Self) -> Self {
                    Flags(self.0 ^ rhs.0)
                }
            }

            impl BitXorAssign for Flags {
                fn bitxor_assign(&mut self, rhs: Self)  {
                    *self = *self ^ rhs;
                }
            }

            impl Not for Flags {
                type Output = Self;
                fn not(self) -> Self {
                    Flags(!self.0)
                }
            }
        }
    };
}

/////////////////////////////////////////////////////////////////////////////////////////
// Actual definitions of the different flags...

// kpageflags for kernel 3.10.0
kpf! {
    KPF3_10_0 {
        Locked = 0,
        Error = 1,
        Referenced = 2,
        Uptodate = 3,
        Dirty = 4,
        Lru = 5,
        Active = 6,
        Slab = 7,
        Writeback = 8,
        Reclaim = 9,
        Buddy = 10,
        Mmap = 11,
        Anon = 12,
        Swapcache = 13,
        Swapbacked = 14,
        CompoundHead = 15,
        CompoundTail = 16,
        Huge = 17,
        Unevictable = 18,
        Hwpoison = 19,
        Nopage = 20,
        Ksm = 21,
        Thp = 22,
        Balloon = 23,
        ZeroPage = 24,
        Idle = 25,

        Reserved = 32,
        Mlocked = 33,
        Mappedtodisk = 34,
        Private = 35,
        Private2 = 36,
        OwnerPrivate = 37,
        Arch = 38,
        Uncached = 39,

        Readahead = 48,
        Slobfree = 49,
        Slubfrozen = 50,
        Slubdebug = 51,
    }

    NOPAGE: Self = Nopage;
    COMPOUND_HEAD: Self = CompoundHead;
    COMPOUND_TAIL: Self = CompoundTail;
    PGTABLE: Option<Self> = None;
    BUDDY: Self = Buddy;
    SLAB: Self = Slab;
    RESERVED: Self = Reserved;
    MMAP: Self = Mmap;
    LRU: Self = Lru;
    ANON: Self = Anon;
    THP: Self = Thp;
    PRIVATE: Self = Private;
    PRIVATE2: Self = Private2;
    OWNERPRIVATE1: Self = OwnerPrivate;
}

// kpageflags for kernel 4.15.0
kpf! {
    KPF4_15_0 {
        Locked = 0,
        Error = 1,
        Referenced = 2,
        Uptodate = 3,
        Dirty = 4,
        Lru = 5,
        Active = 6,
        Slab = 7,
        Writeback = 8,
        Reclaim = 9,
        Buddy = 10,
        Mmap = 11,
        Anon = 12,
        Swapcache = 13,
        Swapbacked = 14,
        CompoundHead = 15,
        CompoundTail = 16,
        Huge = 17,
        Unevictable = 18,
        Hwpoison = 19,
        Nopage = 20,
        Ksm = 21,
        Thp = 22,
        Balloon = 23,
        ZeroPage = 24,
        Idle = 25,

        Reserved = 32,
        Mlocked = 33,
        Mappedtodisk = 34,
        Private = 35,
        Private2 = 36,
        OwnerPrivate = 37,
        Arch = 38,
        Uncached = 39,
        Softdirty = 40,

        Readahead = 48,
        Slobfree = 49,
        Slubfrozen = 50,
        Slubdebug = 51,

        File = 61,
        Swap = 62,
        MmapExclusive = 63,
    }

    NOPAGE: Self = Nopage;
    COMPOUND_HEAD: Self = CompoundHead;
    COMPOUND_TAIL: Self = CompoundTail;
    PGTABLE: Option<Self> = None;
    BUDDY: Self = Buddy;
    SLAB: Self = Slab;
    RESERVED: Self = Reserved;
    MMAP: Self = Mmap;
    LRU: Self = Lru;
    ANON: Self = Anon;
    THP: Self = Thp;
    PRIVATE: Self = Private;
    PRIVATE2: Self = Private2;
    OWNERPRIVATE1: Self = OwnerPrivate;
}

// kpageflags for kernel 5.0.8
kpf! {
    KPF5_0_8 {
        Locked = 0,
        Error = 1,
        Referenced = 2,
        Uptodate = 3,
        Dirty = 4,
        Lru = 5,
        Active = 6,
        Slab = 7,
        Writeback = 8,
        Reclaim = 9,
        Buddy = 10,
        Mmap = 11,
        Anon = 12,
        Swapcache = 13,
        Swapbacked = 14,
        CompoundHead = 15,
        CompoundTail = 16,
        Huge = 17,
        Unevictable = 18,
        Hwpoison = 19,
        Nopage = 20,
        Ksm = 21,
        Thp = 22,
        Offline = 23,
        ZeroPage = 24,
        Idle = 25,
        Pgtable = 26,

        Reserved = 32,
        Mlocked = 33,
        Mappedtodisk = 34,
        Private = 35,
        Private2 = 36,
        OwnerPrivate = 37,
        Arch = 38,
        Uncached = 39,
        Softdirty = 40,

        Readahead = 48,
        Slobfree = 49,
        Slubfrozen = 50,
        Slubdebug = 51,

        File = 61,
        Swap = 62,
        MmapExclusive = 63,
    }

    NOPAGE: Self = Nopage;
    COMPOUND_HEAD: Self = CompoundHead;
    COMPOUND_TAIL: Self = CompoundTail;
    PGTABLE: Option<Self> = Some(Pgtable);
    BUDDY: Self = Buddy;
    SLAB: Self = Slab;
    RESERVED: Self = Reserved;
    MMAP: Self = Mmap;
    LRU: Self = Lru;
    ANON: Self = Anon;
    THP: Self = Thp;
    PRIVATE: Self = Private;
    PRIVATE2: Self = Private2;
    OWNERPRIVATE1: Self = OwnerPrivate;
}

// kpageflags for kernel 5.4.0
kpf! {
    KPF5_4_0 {
        Locked = 0,
        Error = 1,
        Referenced = 2,
        Uptodate = 3,
        Dirty = 4,
        Lru = 5,
        Active = 6,
        Slab = 7,
        Writeback = 8,
        Reclaim = 9,
        Buddy = 10,
        Mmap = 11,
        Anon = 12,
        Swapcache = 13,
        Swapbacked = 14,
        CompoundHead = 15,
        CompoundTail = 16,
        Huge = 17,
        Unevictable = 18,
        Hwpoison = 19,
        Nopage = 20,
        Ksm = 21,
        Thp = 22,
        Offline = 23,
        ZeroPage = 24,
        Idle = 25,
        Pgtable = 26,

        Reserved = 32,
        Mlocked = 33,
        Mappedtodisk = 34,
        Private = 35,
        Private2 = 36,
        OwnerPrivate = 37,
        Arch = 38,
        Uncached = 39,
        Softdirty = 40,

        Readahead = 48,
        Slobfree = 49,
        Slubfrozen = 50,
        Slubdebug = 51,

        File = 61,
        Swap = 62,
        MmapExclusive = 63,
    }

    NOPAGE: Self = Nopage;
    COMPOUND_HEAD: Self = CompoundHead;
    COMPOUND_TAIL: Self = CompoundTail;
    PGTABLE: Option<Self> = Some(Pgtable);
    BUDDY: Self = Buddy;
    SLAB: Self = Slab;
    RESERVED: Self = Reserved;
    MMAP: Self = Mmap;
    LRU: Self = Lru;
    ANON: Self = Anon;
    THP: Self = Thp;
    PRIVATE: Self = Private;
    PRIVATE2: Self = Private2;
    OWNERPRIVATE1: Self = OwnerPrivate;
}

// kpageflags for kernel 5.13.0
kpf! {
    KPF5_13_0 {
        Locked = 0,
        Error = 1,
        Referenced = 2,
        Uptodate = 3,
        Dirty = 4,
        Lru = 5,
        Active = 6,
        Slab = 7,
        Writeback = 8,
        Reclaim = 9,
        Buddy = 10,
        Mmap = 11,
        Anon = 12,
        Swapcache = 13,
        Swapbacked = 14,
        CompoundHead = 15,
        CompoundTail = 16,
        Huge = 17,
        Unevictable = 18,
        Hwpoison = 19,
        Nopage = 20,
        Ksm = 21,
        Thp = 22,
        Offline = 23,
        ZeroPage = 24,
        Idle = 25,
        Pgtable = 26,

        Reserved = 32,
        Mlocked = 33,
        Mappedtodisk = 34,
        Private = 35,
        Private2 = 36,
        OwnerPrivate = 37,
        Arch = 38,
        Uncached = 39,
        Softdirty = 40,
        Arch2 = 41,

        Readahead = 48,
        Slobfree = 49,
        Slubfrozen = 50,
        Slubdebug = 51,

        File = 61,
        Swap = 62,
        MmapExclusive = 63,
    }

    NOPAGE: Self = Nopage;
    COMPOUND_HEAD: Self = CompoundHead;
    COMPOUND_TAIL: Self = CompoundTail;
    PGTABLE: Option<Self> = Some(Pgtable);
    BUDDY: Self = Buddy;
    SLAB: Self = Slab;
    RESERVED: Self = Reserved;
    MMAP: Self = Mmap;
    LRU: Self = Lru;
    ANON: Self = Anon;
    THP: Self = Thp;
    PRIVATE: Self = Private;
    PRIVATE2: Self = Private2;
    OWNERPRIVATE1: Self = OwnerPrivate;
}

// kpageflags for kernel 5.15.0
kpf! {
    KPF5_15_0 {
        Locked = 0,
        Error = 1,
        Referenced = 2,
        Uptodate = 3,
        Dirty = 4,
        Lru = 5,
        Active = 6,
        Slab = 7,
        Writeback = 8,
        Reclaim = 9,
        Buddy = 10,
        Mmap = 11,
        Anon = 12,
        Swapcache = 13,
        Swapbacked = 14,
        CompoundHead = 15,
        CompoundTail = 16,
        Huge = 17,
        Unevictable = 18,
        Hwpoison = 19,
        Nopage = 20,
        Ksm = 21,
        Thp = 22,
        Offline = 23,
        ZeroPage = 24,
        Idle = 25,
        Pgtable = 26,

        Reserved = 32,
        Mlocked = 33,
        Mappedtodisk = 34,
        Private = 35,
        Private2 = 36,
        OwnerPrivate = 37,
        Arch = 38,
        Uncached = 39,
        Softdirty = 40,
        Arch2 = 41,

        Readahead = 48,
        Slobfree = 49,
        Slubfrozen = 50,
        Slubdebug = 51,

        File = 61,
        Swap = 62,
        MmapExclusive = 63,
    }

    NOPAGE: Self = Nopage;
    COMPOUND_HEAD: Self = CompoundHead;
    COMPOUND_TAIL: Self = CompoundTail;
    PGTABLE: Option<Self> = Some(Pgtable);
    BUDDY: Self = Buddy;
    SLAB: Self = Slab;
    RESERVED: Self = Reserved;
    MMAP: Self = Mmap;
    LRU: Self = Lru;
    ANON: Self = Anon;
    THP: Self = Thp;
    PRIVATE: Self = Private;
    PRIVATE2: Self = Private2;
    OWNERPRIVATE1: Self = OwnerPrivate;
}

// kpageflags for kernel 5.17.0
kpf! {
    KPF5_17_0 {
        Locked = 0,
        Error = 1,
        Referenced = 2,
        Uptodate = 3,
        Dirty = 4,
        Lru = 5,
        Active = 6,
        Slab = 7,
        Writeback = 8,
        Reclaim = 9,
        Buddy = 10,
        Mmap = 11,
        Anon = 12,
        Swapcache = 13,
        Swapbacked = 14,
        CompoundHead = 15,
        CompoundTail = 16,
        Huge = 17,
        Unevictable = 18,
        Hwpoison = 19,
        Nopage = 20,
        Ksm = 21,
        Thp = 22,
        Offline = 23,
        ZeroPage = 24,
        Idle = 25,
        Pgtable = 26,

        Reserved = 32,
        Mlocked = 33,
        Mappedtodisk = 34,
        Private = 35,
        Private2 = 36,
        OwnerPrivate = 37,
        Arch = 38,
        Uncached = 39,
        Softdirty = 40,
        Arch2 = 41,

        Readahead = 48,
        Slobfree = 49,
        Slubfrozen = 50,
        Slubdebug = 51,

        File = 61,
        Swap = 62,
        MmapExclusive = 63,
    }

    NOPAGE: Self = Nopage;
    COMPOUND_HEAD: Self = CompoundHead;
    COMPOUND_TAIL: Self = CompoundTail;
    PGTABLE: Option<Self> = Some(Pgtable);
    BUDDY: Self = Buddy;
    SLAB: Self = Slab;
    RESERVED: Self = Reserved;
    MMAP: Self = Mmap;
    LRU: Self = Lru;
    ANON: Self = Anon;
    THP: Self = Thp;
    PRIVATE: Self = Private;
    PRIVATE2: Self = Private2;
    OWNERPRIVATE1: Self = OwnerPrivate;
}

// kpageflags for kernel 6.0.0
kpf! {
    KPF6_0_0 {
        Locked = 0,
        Error = 1,
        Referenced = 2,
        Uptodate = 3,
        Dirty = 4,
        Lru = 5,
        Active = 6,
        Slab = 7,
        Writeback = 8,
        Reclaim = 9,
        Buddy = 10,
        Mmap = 11,
        Anon = 12,
        Swapcache = 13,
        Swapbacked = 14,
        CompoundHead = 15,
        CompoundTail = 16,
        Huge = 17,
        Unevictable = 18,
        Hwpoison = 19,
        Nopage = 20,
        Ksm = 21,
        Thp = 22,
        Offline = 23,
        ZeroPage = 24,
        Idle = 25,
        Pgtable = 26,

        Reserved = 32,
        Mlocked = 33,
        Mappedtodisk = 34,
        Private = 35,
        Private2 = 36,
        OwnerPrivate = 37,
        Arch = 38,
        Uncached = 39,
        Softdirty = 40,
        Arch2 = 41,

        AnonExclusive = 47,
        Readahead = 48,
        Slobfree = 49,
        Slubfrozen = 50,
        Slubdebug = 51,

        File = 61,
        Swap = 62,
        MmapExclusive = 63,
    }

    NOPAGE: Self = Nopage;
    COMPOUND_HEAD: Self = CompoundHead;
    COMPOUND_TAIL: Self = CompoundTail;
    PGTABLE: Option<Self> = Some(Pgtable);
    BUDDY: Self = Buddy;
    SLAB: Self = Slab;
    RESERVED: Self = Reserved;
    MMAP: Self = Mmap;
    LRU: Self = Lru;
    ANON: Self = Anon;
    THP: Self = Thp;
    PRIVATE: Self = Private;
    PRIVATE2: Self = Private2;
    OWNERPRIVATE1: Self = OwnerPrivate;
}
