use std::vec;
use std::{array, iter::Enumerate};
use std::{cmp::max, iter::Iterator};
struct Solution;

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let mut total = 0;
        let mut stack = vec![];
        let mut stack1 = vec![];

        let ab = ['a', 'b'];
        let ba = ['b', 'a'];
        let (first, second) = match x > y {
            true => (ab, ba),
            false => (ba, ab),
        };
        let (h, l) = match x > y {
            true => (x, y),
            false => (y, x),
        };
        //println!["{:?}", dp];
        for ch in s.chars() {
            stack.push(ch);
            while stack.len() >= 2 && stack[stack.len() - 2..] == first {
                total += h;
                stack.pop();
                stack.pop();
            }
        }
        for ch in stack {
            stack1.push(ch);
            while stack1.len() >= 2 && stack1[stack1.len() - 2..] == second {
                total += l;
                stack1.pop();
                stack1.pop();
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn sample1() {
        assert_eq!(
            Solution::maximum_gain(String::from("aabbaaxybbaabb"), 4, 5),
            20
        )
    }
    #[test]
    fn sample2() {
        assert_eq!(
            Solution::maximum_gain(String::from("cdbcbbaaabab"), 4, 5),
            19
        )
    }
}
