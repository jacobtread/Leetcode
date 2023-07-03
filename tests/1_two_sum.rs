//! # 1. Two Sum
//! Given an array of integers `nums` and an integer `target`, return indices of the two numbers such that they add up to `target`.
//!
//! You may assume that each input would have exactly one solution, and you may not use the same element twice.
//!
//! You can return the answer in any order.

use std::{
    collections::HashMap,
    hash::{BuildHasherDefault, Hasher},
};

#[derive(Default)]
pub struct IntHasher(u64);

impl Hasher for IntHasher {
    fn write(&mut self, _: &[u8]) {
        panic!("Invalid use of IntHasher")
    }

    fn write_u8(&mut self, n: u8) {
        self.0 = u64::from(n)
    }
    fn write_u16(&mut self, n: u16) {
        self.0 = u64::from(n)
    }
    fn write_u32(&mut self, n: u32) {
        self.0 = u64::from(n)
    }
    fn write_u64(&mut self, n: u64) {
        self.0 = n
    }
    fn write_usize(&mut self, n: usize) {
        self.0 = n as u64
    }

    fn write_i8(&mut self, n: i8) {
        self.0 = n as u64
    }
    fn write_i16(&mut self, n: i16) {
        self.0 = n as u64
    }
    fn write_i32(&mut self, n: i32) {
        self.0 = n as u64
    }
    fn write_i64(&mut self, n: i64) {
        self.0 = n as u64
    }
    fn write_isize(&mut self, n: isize) {
        self.0 = n as u64
    }

    fn finish(&self) -> u64 {
        self.0
    }
}

pub fn two_sum(nums: &[i32], target: i32) -> [i32; 2] {
    let mut map: HashMap<i32, usize, BuildHasherDefault<IntHasher>> = HashMap::default();

    let output = nums
        .iter()
        .enumerate()
        .find_map(|(i, num)| {
            if let Some(index) = map.get(num) {
                Some([i as i32, (*index) as i32])
            } else {
                map.insert(target - num, i);
                None
            }
        })
        .unwrap_or([0, 0]);

    output
}

#[cfg(test)]
mod test {
    use crate::two_sum;

    fn assert_matches(a: [i32; 2], b: [i32; 2]) {
        assert!((a[0] == b[0] && a[1] == b[1]) || (a[1] == b[0] && a[0] == b[1]))
    }

    #[test]
    fn example_1() {
        let target = 9;
        let nums = &[2, 7, 11, 15];
        let output = two_sum(nums, target);

        assert_matches(output, [0, 1]);
    }

    #[test]
    fn example_2() {
        let target = 6;
        let nums = &[3, 2, 4];
        let output = two_sum(nums, target);

        assert_matches(output, [1, 2]);
    }

    #[test]
    fn example_3() {
        let target = 6;
        let nums = &[3, 3];
        let output = two_sum(nums, target);

        assert_matches(output, [0, 1]);
    }
}
