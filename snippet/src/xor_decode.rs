//use std::{array, iter::Enumerate, vec};
use std::{i32::{MAX, MIN}, iter::Iterator};
use std::cmp::{max};
use std::collections::BinaryHeap;
struct Solution;

// make struct sortalbe
// #[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
impl Solution {
    pub fn decode(encoded: Vec<i32>) -> Vec<i32> {
        let mut total= 0;
        let mut res = vec![];
        for num in 1..=(encoded.len()+1) {
            total = total ^ num;
        }
        let mut first = total as i32;
        for i in 0..encoded.len() {
            if (i & 1) == 0 {
                continue;
            }
            first = first ^ encoded[i];
        }
        res.push(first);
        for code in &encoded{
            // fisrt holds current val
            first = first ^ code;
            res.push(first);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::Solution;

    #[test]
    fn sample1() {
        assert_eq!(
            Solution::decode(vec![3,1]),
            vec![1,2,3]
        )
    }
    #[test]
    fn sample2() {
        assert_eq!(
            Solution::decode(vec![6,5,4,6]),
            vec![2,4,1,5,3]
        )
    }
}
