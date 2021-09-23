use std::ops::Index;
use std::ops::Rem;

#[derive(Debug)]
pub struct Bits {
    arr: Vec<u8>,
    size: usize,
}

static TRUE: bool = true;
static FALSE: bool = false;

impl Bits {
    pub fn new(size: usize) -> Bits {
        let len = Bits::len_for_size(size);
        Bits {
            arr: vec![0; len],
            size: size,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    fn len_for_size(size: usize) -> usize {
        if size >= u16::max_value() as usize {
            panic!("Bits size cannot exceed {}", u16::max_value())
        }
        let rem = size.rem(8);
        match rem {
            0 => size / 8,
            _ => size / 8 + 1,
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
        if i >= self.size {
            panic!("Bit index must be less than {}", i);
        }
        let index = i / 8;
        let shift = i.rem(8);
        (index, shift)
    }
}

impl Index<usize> for Bits {
    type Output = bool;

    fn index(&self, i: usize) -> &bool {
        if self.get(i) {
            &TRUE
        } else {
            &FALSE
        }
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
    fn check_len() {
        let b = Bits::new(16);
        assert_eq!(b.arr.len(), 2);
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
    #[should_panic(expected = "Bits size cannot exceed 65535")]
    fn max_size() {
        Bits::new(usize::max_value());
    }

    #[test]
    fn full() {
        let mut b = Bits::new(128);
        for i in 0..128 {
            b.set(i)
        }
        for i in 0..128 {
            assert!(b.get(i))
        }
        assert_eq!(
            b.arr,
            vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]
        );
    }

    #[test]
    #[should_panic(expected = "Bit index must be less than 1")]
    fn out_of_bounds() {
        let mut b = Bits::new(1);
        b.set(1);
    }

    #[test]
    fn index_get() {
        let b = Bits::new(1);
        assert_eq!(b[0], false);
    }

    #[test]
    fn index_get_set() {
        let mut b = Bits::new(1);
        b.set(0);
        assert_eq!(b[0], true);
    }

    mod quickcheck {
        use super::*;
        use quickcheck::TestResult;

        quickcheck! {
            fn creatable(size: usize) -> bool {
                Bits::new(size).size == size
            }

            fn set_get(size: usize, i: usize) -> TestResult {
                if i >= size {
                    TestResult::discard()
                } else {
                    let mut b = Bits::new(size);
                    b.set(i);
                    TestResult::from_bool(b.get(i))
                }
            }

            fn set_index_get(size: usize, i: usize) -> TestResult {
                if i >= size {
                    TestResult::discard()
                } else {
                    let mut b = Bits::new(size);
                    b.set(i);
                    TestResult::from_bool(b[i])
                }
            }

            fn get_not_set(size: usize, i: usize) -> TestResult {
                if i >= size {
                    TestResult::discard()
                } else {
                    let b = Bits::new(size);
                    TestResult::from_bool(!b.get(i))
                }
            }

            fn index_get_not_set(size: usize, i: usize) -> TestResult {
                if i >= size {
                    TestResult::discard()
                } else {
                    let b = Bits::new(size);
                    TestResult::from_bool(!b[i])
                }
            }
        }
    }
}
