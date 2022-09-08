//! Machinery for interpretting kpageflags on a few different kernels.

use std::str::FromStr;

/// All the different KPF implementations are `Flaggy`.
pub trait Flaggy:
    Sized + FromStr + Copy + std::fmt::Debug + std::hash::Hash + Ord + Eq + Into<u64> + From<u64>
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

    fn valid(val: u64) -> bool;
    fn values() -> &'static [u64];

    fn valid_mask() -> u64 {
        let mut v = 0;
        for b in Self::values() {
            v |= 1 << b;
        }
        v
    }
}

/// Easier to derive `Flaggy` and a bunch of other stuff...
macro_rules! kpf {
    (pub enum $kpfname:ident { $($name:ident = $val:literal),+ $(,)? } $($c:ident: $t:ty = $v:expr;)+) => {
        // It's not actually dead code... the `KPF::from` function allows constructing all of them...
        #[allow(dead_code)]
        #[derive(Copy, Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
        #[repr(u64)]
        pub enum $kpfname {
            $($name = $val),+
        }

        impl $kpfname {
            const _SIZE_CHECK: () = if std::mem::size_of::<u64>() != std::mem::size_of::<$kpfname>() {
                panic!("KPF size > sizeof(u64)");
            } else {
                ()
            };
        }

        impl FromStr for $kpfname {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        stringify!($name) => Ok($kpfname::$name),
                    )+

                    other => Err(format!("unknown flag: {}", other)),
                }
            }
        }

        impl Flaggy for $kpfname {
            $(const $c: $t = $v;)+

            fn valid(val: u64) -> bool {
                match val {
                    $($val => true,)*
                    _ => false,
                }
            }

            fn values() -> &'static [u64] {
                &[ $($kpfname::$name as u64),* ]
            }
        }

        impl From<$kpfname> for u64 {
            fn from(kpf: $kpfname) -> u64 {
                kpf as u64
            }
        }

        impl From<u64> for $kpfname {
            fn from(val: u64) -> Self {
                assert!(Self::valid(val));
                unsafe { std::mem::transmute(val) }
            }
        }
    };
}

/////////////////////////////////////////////////////////////////////////////////////////
// Actual definitions of the different flags...

// kpageflags for kernel 3.10.0
kpf! {
    pub enum KPF3_10_0 {
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

    NOPAGE: Self = KPF3_10_0::Nopage;
    COMPOUND_HEAD: Self = KPF3_10_0::CompoundHead;
    COMPOUND_TAIL: Self = KPF3_10_0::CompoundTail;
    PGTABLE: Option<Self> = None;
    BUDDY: Self = KPF3_10_0::Buddy;
    SLAB: Self = KPF3_10_0::Slab;
    RESERVED: Self = KPF3_10_0::Reserved;
    MMAP: Self = KPF3_10_0::Mmap;
    LRU: Self = KPF3_10_0::Lru;
    ANON: Self = KPF3_10_0::Anon;
    THP: Self = KPF3_10_0::Thp;
}

// kpageflags for kernel 4.15.0
kpf! {
    pub enum KPF4_15_0 {
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

    NOPAGE: Self = KPF4_15_0::Nopage;
    COMPOUND_HEAD: Self = KPF4_15_0::CompoundHead;
    COMPOUND_TAIL: Self = KPF4_15_0::CompoundTail;
    PGTABLE: Option<Self> = None;
    BUDDY: Self = KPF4_15_0::Buddy;
    SLAB: Self = KPF4_15_0::Slab;
    RESERVED: Self = KPF4_15_0::Reserved;
    MMAP: Self = KPF4_15_0::Mmap;
    LRU: Self = KPF4_15_0::Lru;
    ANON: Self = KPF4_15_0::Anon;
    THP: Self = KPF4_15_0::Thp;
}

// kpageflags for kernel 5.0.8
kpf! {
    pub enum KPF5_0_8 {
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

    NOPAGE: Self = KPF5_0_8::Nopage;
    COMPOUND_HEAD: Self = KPF5_0_8::CompoundHead;
    COMPOUND_TAIL: Self = KPF5_0_8::CompoundTail;
    PGTABLE: Option<Self> = Some(KPF5_0_8::Pgtable);
    BUDDY: Self = KPF5_0_8::Buddy;
    SLAB: Self = KPF5_0_8::Slab;
    RESERVED: Self = KPF5_0_8::Reserved;
    MMAP: Self = KPF5_0_8::Mmap;
    LRU: Self = KPF5_0_8::Lru;
    ANON: Self = KPF5_0_8::Anon;
    THP: Self = KPF5_0_8::Thp;
}

// kpageflags for kernel 5.4.0
kpf! {
    pub enum KPF5_4_0 {
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

    NOPAGE: Self = KPF5_4_0::Nopage;
    COMPOUND_HEAD: Self = KPF5_4_0::CompoundHead;
    COMPOUND_TAIL: Self = KPF5_4_0::CompoundTail;
    PGTABLE: Option<Self> = Some(KPF5_4_0::Pgtable);
    BUDDY: Self = KPF5_4_0::Buddy;
    SLAB: Self = KPF5_4_0::Slab;
    RESERVED: Self = KPF5_4_0::Reserved;
    MMAP: Self = KPF5_4_0::Mmap;
    LRU: Self = KPF5_4_0::Lru;
    ANON: Self = KPF5_4_0::Anon;
    THP: Self = KPF5_4_0::Thp;
}

// kpageflags for kernel 5.13.0
kpf! {
    pub enum KPF5_13_0 {
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

    NOPAGE: Self = KPF5_13_0::Nopage;
    COMPOUND_HEAD: Self = KPF5_13_0::CompoundHead;
    COMPOUND_TAIL: Self = KPF5_13_0::CompoundTail;
    PGTABLE: Option<Self> = Some(KPF5_13_0::Pgtable);
    BUDDY: Self = KPF5_13_0::Buddy;
    SLAB: Self = KPF5_13_0::Slab;
    RESERVED: Self = KPF5_13_0::Reserved;
    MMAP: Self = KPF5_13_0::Mmap;
    LRU: Self = KPF5_13_0::Lru;
    ANON: Self = KPF5_13_0::Anon;
    THP: Self = KPF5_13_0::Thp;
}

// kpageflags for kernel 5.15.0
kpf! {
    pub enum KPF5_15_0 {
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

    NOPAGE: Self = KPF5_15_0::Nopage;
    COMPOUND_HEAD: Self = KPF5_15_0::CompoundHead;
    COMPOUND_TAIL: Self = KPF5_15_0::CompoundTail;
    PGTABLE: Option<Self> = Some(KPF5_15_0::Pgtable);
    BUDDY: Self = KPF5_15_0::Buddy;
    SLAB: Self = KPF5_15_0::Slab;
    RESERVED: Self = KPF5_15_0::Reserved;
    MMAP: Self = KPF5_15_0::Mmap;
    LRU: Self = KPF5_15_0::Lru;
    ANON: Self = KPF5_15_0::Anon;
    THP: Self = KPF5_15_0::Thp;
}

// kpageflags for kernel 5.17.0
kpf! {
    pub enum KPF5_17_0 {
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

    NOPAGE: Self = KPF5_17_0::Nopage;
    COMPOUND_HEAD: Self = KPF5_17_0::CompoundHead;
    COMPOUND_TAIL: Self = KPF5_17_0::CompoundTail;
    PGTABLE: Option<Self> = Some(KPF5_17_0::Pgtable);
    BUDDY: Self = KPF5_17_0::Buddy;
    SLAB: Self = KPF5_17_0::Slab;
    RESERVED: Self = KPF5_17_0::Reserved;
    MMAP: Self = KPF5_17_0::Mmap;
    LRU: Self = KPF5_17_0::Lru;
    ANON: Self = KPF5_17_0::Anon;
    THP: Self = KPF5_17_0::Thp;
}
