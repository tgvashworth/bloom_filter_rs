#[cfg(test)]
#[macro_use]
extern crate quickcheck;

mod bits;
mod hash;
pub mod bloom_filter;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bloom_filter() {
        bloom_filter::BloomFilter::new(128);
    }
}

