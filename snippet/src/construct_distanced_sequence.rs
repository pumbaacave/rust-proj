use std::array;
use std::iter::Iterator;
use std::vec;
struct Solution;

impl Solution {
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        let un = n as usize;
        // fixed size array is ok;
        let mut v = vec![0; 2 * un - 1];
        let mut state = vec![false; un + 1];
        backtrack(&mut v, &mut state);
        v.iter_mut().map(|num| *num as i32).collect()
    }
}

// return true if finish base case.
fn backtrack(nums: &mut Vec<i32>, state: &mut Vec<bool>) -> bool {
    // may panic?
    println!("{:?}", nums);
    println!("{:?}", state);
    match nums.iter().position(|n| *n == 0) {
        None => {
            return true;
        }
        Some(idx) => {
            let mut done = true;
            for s in &state[1..] {
                if !*s {
                    done = false;
                    break;
                }
            }
            if done {
                return true;
            }
            // [n..1]
            for i in (1..state.len()).rev() {
                if !state[i] {
                    state[i] = true;
                    if i == 1 {
                        nums[idx] = 1;
                        if backtrack(nums, state) {
                            return true;
                        }
                        nums[idx] = 0;
                    } else {
                        if idx + i >= nums.len() {
                            // current filling will fail
                            state[i] = false;
                            continue;
                        }

                        if nums[idx] == 0 && nums[idx + i] == 0 {
                            let v = i as i32;
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

    //#[test]
    fn sample1() {
        assert_eq!(
            Solution::construct_distanced_sequence(3),
            vec![3, 1, 2, 3, 2]
        )
    }

    //#[test]
    fn sample2() {
        assert_eq!(
            Solution::construct_distanced_sequence(5),
            vec![5, 3, 1, 4, 3, 5, 2, 4, 2]
        )
    }
    //#[test]
    fn sample3() {
        assert_eq!(
            Solution::construct_distanced_sequence(4),
            vec![4, 2, 3, 2, 4, 3, 1]
        )
    }
}
