use std::array;
use std::iter::Iterator;
use std::vec;
struct Solution;

impl Solution {
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        let un = n as usize;
        // fixed size array is ok;
        let mut v = vec![0; 2 * un - 1];
        let mut state = vec![false; un];
        v[0] = n;
        v[un] = n;
        backtrack(&mut v, &mut state);
        v.iter_mut().map(|num| *num as i32).collect()
    }
}

fn place_one(nums: &mut Vec<i32>) {
    let it = nums.iter_mut();
    it.filter(|n| **n == 0).map(|n| *n = 1).next(); // next to force consume.
}
// return true if finish base case.
fn backtrack(nums: &mut Vec<i32>, state: &mut Vec<bool>) -> bool {
    // may panic?
    match nums.iter().position(|n| *n == 0) {
        None => {
            return true;
        }
        Some(idx) => {
            // [n-1...1]
            for i in (1..state.len()).rev() {
                if !state[i] {
                    if idx + i >= nums.len() {
                        // current filling will fail
                        continue;
                    }

                    state[i] = true;
                    if i == 1 {
                        nums[idx] = 1;
                        if backtrack(nums, state) {
                            return true;
                        }
                        nums[idx] = 0;
                    } else {
                        let v = i as i32;
                        //println!("{:?}", nums);
                        if nums[idx] == 0 && nums[idx + i] == 0 {
                            nums[idx] = v;
                            nums[idx + i] = v;
                            if backtrack(nums, state) {
                                return true;
                            }
                            nums[idx] = 0;
                            nums[idx + i] = 0;
                        }
                    }
                    state[i] = false;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn sample1() {
        assert_eq!(
            Solution::construct_distanced_sequence(3),
            vec![3, 1, 2, 3, 2]
        )
    }

    #[test]
    fn sample2() {
        assert_eq!(
            Solution::construct_distanced_sequence(5),
            vec![5, 3, 1, 4, 3, 5, 2, 4, 2]
        )
    }
    #[test]
    fn sample3() {
        assert_eq!(
            Solution::construct_distanced_sequence(4),
            vec![4,2,3,2,4,3,1]
        )
    }
}
