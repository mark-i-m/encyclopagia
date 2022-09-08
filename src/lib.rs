//! Tools for reading `/proc/kpageflags` and `/proc/[self]/pagemap`.

pub mod kpageflags;
pub mod pagemap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
