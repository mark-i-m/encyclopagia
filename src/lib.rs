//! Tools for reading `/proc/kpageflags` and `/proc/[self]/pagemap`.

use std::{
    io::{self, BufRead, BufReader, Read},
    marker::PhantomData,
};

pub mod kpageflags;
pub mod pagemap;

/// Indicates that the implementing type can be cast directly from the contents of a file.
///
/// # Safety
/// The trait is `unsafe` because the implementor needs to guarantee that such a cast doesn't cause
/// UB.
pub unsafe trait FileReadable {}

/// A reader for `FileReadable` types.
pub struct FileReadableReader<R: Read, T: FileReadable> {
    reader: BufReader<R>,
    _phantom: PhantomData<T>,
}

impl<R: Read, T: FileReadable> FileReadableReader<R, T> {
    pub fn new(reader: BufReader<R>) -> Self {
        FileReadableReader {
            reader,
            _phantom: PhantomData,
        }
    }

    /// Similar to `Read::read`, but reads the bytes as `PageMapPage`, and returns the number of
    /// flags in the buffer, rather than the number of bytes.
    pub fn read(&mut self, orig_buf: &mut [T]) -> io::Result<usize> {
        let size = std::mem::size_of::<T>();

        // Cast as an array of bytes to do the read.
        let mut buf: &mut [u8] = unsafe {
            let ptr: *mut u8 = orig_buf.as_mut_ptr() as *mut u8;
            let len = orig_buf.len() * size;
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
                len if len < size => {
                    // Copy what we have...
                    buf[..len].copy_from_slice(&filled_buf[..len]);

                    // ... and refill.
                    self.reader.consume(len);
                    filled_buf = self.reader.fill_buf()?;
                    buf = &mut buf[len..];
                    total_bytes_read += len;
                }

                // Enough for at least one flag.
                len => {
                    // Figure out how many complete `PageMapPage` we have, and copy them to the `orig_buf`.
                    let max_bytes_to_copy = std::cmp::min(len, buf.len());
                    let complete_flags = max_bytes_to_copy / size; // round (integer division)

                    // We account for any partially read flags from previous iterations...
                    let bytes_to_copy = complete_flags * size - (total_bytes_read % size);

                    buf[..bytes_to_copy].copy_from_slice(&filled_buf[..bytes_to_copy]);
                    total_bytes_read += bytes_to_copy;

                    // Tell the `BufReader` how much we consumed.
                    self.reader.consume(bytes_to_copy);

                    break;
                }
            }
        }

        // Error: maybe the file was gzipped? Check for the sake of error reporting.
        if total_bytes_read % size != 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Total number of bytes read is not a multiple of struct size. \
                 Perhaps the data is compressed?",
            ));
        }

        Ok(total_bytes_read / size)
    }
}
