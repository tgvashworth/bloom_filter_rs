#[cfg(test)]
#[macro_use]
extern crate quickcheck;

mod bits;
pub mod bloom_filter;
mod hash;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bloom_filter() {
        bloom_filter::BloomFilter::new(128);
    }
}
