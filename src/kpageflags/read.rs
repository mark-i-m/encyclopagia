//! Abstractions for reading kpageflags and producing a stream of flags.

use std::{
    io::{self, BufRead, BufReader, Read},
    marker::PhantomData,
};

use super::{flags::Flaggy, KPageFlags, KPF_SIZE};

/// Wrapper around a `Read` type that for the `/proc/kpageflags` file.
pub struct KPageFlagsReader<R: Read, K: Flaggy> {
    reader: BufReader<R>,
    _phantom: PhantomData<K>,
}

impl<R: Read, K: Flaggy> KPageFlagsReader<R, K> {
    pub fn new(reader: BufReader<R>) -> Self {
        KPageFlagsReader {
            reader,
            _phantom: PhantomData,
        }
    }

    /// Similar to `Read::read`, but reads the bytes as `KPageFlags`, and returns the number of
    /// flags in the buffer, rather than the number of bytes.
    pub fn read(&mut self, orig_buf: &mut [KPageFlags<K>]) -> io::Result<usize> {
        // Cast as an array of bytes to do the read.
        let mut buf: &mut [u8] = unsafe {
            let ptr: *mut u8 = orig_buf.as_mut_ptr() as *mut u8;
            let len = orig_buf.len() * KPF_SIZE;
            std::slice::from_raw_parts_mut(ptr, len)
        };

        // Manually read from the buffer so that we can stop at a proper KPF boundary.
        let mut total_bytes_read = 0;
        let mut filled_buf = if self.reader.buffer().is_empty() {
            self.reader.fill_buf()?
        } else {
            self.reader.buffer()
        };

        // Until we read enough...
        loop {
            match filled_buf.len() {
                // Reached EOF
                0 => break,

                // Doesn't contain enough data for even one flag.
                len if len < KPF_SIZE => {
                    // Copy what we have...
                    for i in 0..len {
                        buf[i] = filled_buf[i];
                    }

                    // ... and refill.
                    self.reader.consume(len);
                    filled_buf = self.reader.fill_buf()?;
                    buf = &mut buf[len..];
                    total_bytes_read += len;
                }

                // Enough for at least one flag.
                len => {
                    // Figure out how many complete `KPageFlags` we have, and copy them to the `orig_buf`.
                    let max_bytes_to_copy = std::cmp::min(len, buf.len());
                    let complete_flags = max_bytes_to_copy / KPF_SIZE; // round (integer division)

                    // We account for any partially read flags from previous iterations...
                    let bytes_to_copy = complete_flags * KPF_SIZE - (total_bytes_read % KPF_SIZE);

                    for i in 0..bytes_to_copy {
                        buf[i] = filled_buf[i];
                    }
                    total_bytes_read += bytes_to_copy;

                    // Tell the `BufReader` how much we consumed.
                    self.reader.consume(bytes_to_copy);

                    break;
                }
            }
        }

        assert_eq!(total_bytes_read % KPF_SIZE, 0);
        Ok(total_bytes_read / KPF_SIZE)
    }
}

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

                for f in ignored_flags.into_iter() {
                    mask |= 1 << (*f).into();
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

        item.clear(self.ignored_flags);

        self.nflags -= 1;
        self.idx += 1;

        Some(item)
    }
}
