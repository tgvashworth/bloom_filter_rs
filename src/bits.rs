use std::ops::Rem;

#[derive(Debug)]
pub struct Bits {
    arr: Vec<u32>,
    size: usize
}

impl Bits {
    pub fn new(size: usize) -> Bits {
        let len = size / 32 + 1;
        Bits {
            arr: vec![0; len],
            size: size
        }
    }

    pub fn set(&mut self, i: usize) {
        let (index, shift) = self.loc(i);
        self.arr[index] = self.arr[index] | (1 << shift);
    }

    pub fn unset(&mut self, i: usize) {
        let (index, shift) = self.loc(i);
        self.arr[index] = self.arr[index] & !(1 << shift);
    }

    pub fn get(&self, i: usize) -> bool {
        let (index, shift) = self.loc(i);
        (self.arr[index] >> shift) & 1 == 1
    }

    fn loc(&self, i: usize) -> (usize, usize) {
        if (i >= self.size) {
            panic!("Bit index must be less than {}", i);
        }
        let index = i / 32;
        let shift = i.rem(32);
        (index, shift)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creatable() {
        Bits::new(10);
    }

    #[test]
    fn set() {
        let mut b = Bits::new(10);
        b.set(0)
    }

    #[test]
    fn set_check_bits() {
        let mut b = Bits::new(8);
        assert_eq!(b.arr.len(), 1);
        b.set(0);
        assert_eq!(b.arr, vec![1]);
    }

    #[test]
    fn set_non_zero_check_bits() {
        let mut b = Bits::new(8);
        assert_eq!(b.arr.len(), 1);
        b.set(1);
        assert_eq!(b.arr, vec![2]);
    }

    #[test]
    fn get() {
        let mut b = Bits::new(10);
        b.set(0);
        assert_eq!(b.get(0), true)
    }

    #[test]
    fn get_not_set() {
        let b = Bits::new(1);
        assert_eq!(b.get(0), false)
    }

    #[test]
    fn unset() {
        let mut b = Bits::new(1);
        b.set(0);
        assert_eq!(b.get(0), true);
        b.unset(0);
        assert_eq!(b.get(0), false);
    }

    #[test]
    fn large_size() {
        let mut b = Bits::new(128);
        b.set(0);
        b.set(8);
        b.set(127);
        assert_eq!(b.get(0), true);
        assert_eq!(b.get(8), true);
        assert_eq!(b.get(127), true);
    }

    #[test]
    #[should_panic(expected = "Bit index must be less than 1")]
    fn out_of_bounds() {
        let mut b = Bits::new(1);
        b.set(1);
    }
}
