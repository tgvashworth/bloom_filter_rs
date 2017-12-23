#[cfg(test)]
#[macro_use]
extern crate quickcheck;

mod bits;
mod hash;

use std::ops::Rem;
use bits::Bits;

#[derive(Debug)]
pub struct BloomFilter {
    bits: Bits,
}

static ROUNDS: usize = 5;

impl BloomFilter {
    pub fn new(size: usize) -> BloomFilter {
        BloomFilter {
            bits: Bits::new(size)
        }
    }

    pub fn add(&mut self, v: &str) {
        let hashes = hash::hash_rounds(v, ROUNDS);
        self.idxs(hashes).iter().for_each(|i| {
            self.bits.set(*i)
        })
    }

    pub fn contains(&self, v: &str) -> bool {
        let hashes = hash::hash_rounds(v, ROUNDS);
        self.idxs(hashes).iter().all(|i| {
            self.bits.get(*i)
        })
    }

    fn idxs(&self, hashes: Vec<String>) -> Vec<usize> {
        hashes.iter().map(|hash| {
            match usize::from_str_radix(&hash[..8], 16) {
                Ok(v) => v.rem(self.bits.size()),
                Err(error) => {
                    panic!("Failed to parse: {:?}", error)
                },
            }
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creatable() {
        BloomFilter::new(128);
    }

    #[test]
    fn add_string() {
        let mut b = BloomFilter::new(128);
        b.add("a")
    }

    #[test]
    fn contains_string() {
        let mut b = BloomFilter::new(128);
        b.add("a");
        assert_eq!(b.contains("a"), true)
    }

    #[test]
    fn does_not_contain_string() {
        let b = BloomFilter::new(128);
        assert_eq!(b.contains("a"), false)
    }

    #[test]
    fn contains_multiple_strings() {
        let mut b = BloomFilter::new(128);
        b.add("a");
        b.add("b");
        b.add("c");
        assert_eq!(b.contains("a"), true);
        assert_eq!(b.contains("b"), true);
        assert_eq!(b.contains("c"), true)
    }

    #[test]
    fn mixed_use() {
        let mut b = BloomFilter::new(128);
        b.add("a");
        b.add("b");
        b.add("c");
        assert_eq!(b.contains("a"), true);
        assert_eq!(b.contains("b"), true);
        assert_eq!(b.contains("c"), true);
        assert_eq!(b.contains("d"), false);
        assert_eq!(b.contains("e"), false);
        println!("bits: {:?}", b.bits)
    }
}
