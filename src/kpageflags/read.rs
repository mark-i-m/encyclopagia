//! Abstractions for reading kpageflags and producing a stream of flags.

use std::io::Read;

use crate::FileReadableReader;

use super::{flags::Flaggy, KPageFlags};

/// Wrapper around a `Read` type that for the `/proc/kpageflags` file.
pub type KPageFlagsReader<R, K> = FileReadableReader<R, KPageFlags<K>>;

/// Turns a `KPageFlagsReader` into a proper (efficient) iterator over flags.
pub struct KPageFlagsIterator<R: Read, K: Flaggy> {
    /// The reader we are reading from.
    reader: KPageFlagsReader<R, K>,

    /// Temporary buffer for data read but not consumed yet.
    buf: [KPageFlags<K>; 1 << (21 - 3)],
    /// The number of valid flags in the buffer.
    nflags: usize,
    /// The index of the first valid, unconsumed flag in the buffer, if `nflags > 0`.
    idx: usize,

    ignored_flags: u64,
}

impl<R: Read, K: Flaggy> KPageFlagsIterator<R, K> {
    pub fn new(reader: KPageFlagsReader<R, K>, ignored_flags: &[K]) -> Self {
        KPageFlagsIterator {
            reader,
            buf: [KPageFlags::empty(); 1 << (21 - 3)],
            nflags: 0,
            idx: 0,
            ignored_flags: {
                let mut mask: u64 = 0;

                for f in ignored_flags {
                    mask |= <K as Into<u64>>::into(*f);
                }

                mask
            },
        }
    }
}

impl<R: Read, K: Flaggy> Iterator for KPageFlagsIterator<R, K> {
    type Item = KPageFlags<K>;

    fn next(&mut self) -> Option<Self::Item> {
        // Need to read some more?
        if self.nflags == 0 {
            self.nflags = match self.reader.read(&mut self.buf) {
                Err(err) => {
                    panic!("{:?}", err);
                }

                // EOF
                Ok(0) => return None,

                Ok(nflags) => nflags,
            };
            self.idx = 0;
        }

        // Return the first valid flags.
        let mut item = self.buf[self.idx];

        item.clear(self.ignored_flags.into());

        self.nflags -= 1;
        self.idx += 1;

        Some(item)
    }
}
