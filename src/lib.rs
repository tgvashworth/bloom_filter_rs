mod bits;
use bits::Bits;

#[derive(Debug)]
pub struct BloomFilter {
    bits: Bits,
}

impl BloomFilter {
    pub fn new(size: usize) -> BloomFilter {
        BloomFilter {
            bits: Bits::new(size)
        }
    }

    pub fn add(&mut self, v: &str) {
    }

    pub fn contains(&self, v: &str) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creatable() {
        BloomFilter::new(10);
    }

    #[test]
    fn add_string() {
        let mut b = BloomFilter::new(10);
        b.add("a")
    }

    #[test]
    fn contains_string() {
        let mut b = BloomFilter::new(10);
        b.add("a");
        assert!(b.contains("a"))
    }

    #[test]
    fn does_not_contain_string() {
        let b = BloomFilter::new(10);
        assert!(!b.contains("a"))
    }
}
