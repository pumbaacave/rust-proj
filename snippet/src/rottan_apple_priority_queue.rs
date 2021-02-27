use std::{cmp::max, iter::Iterator};
use std::collections::BinaryHeap;
use std::cmp::{Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq )]
struct State {
    // lexi order
    day: i32, // apple's last edible day.
    num: i32
}
/// next
/// use std::cmp::Reverse;
impl Ord for State {
    // reverse to min heap
    fn cmp(&self, other: &Self) -> Ordering {
        other.day.cmp(&self.day).then_with(|| other.num.cmp(&self.num))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Solution {
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        let mut idx= 0;
        let mut total = 0;
        let mut heap = BinaryHeap::new();
        for (apple, duration) in apples.iter().zip(days.iter()) {
            idx += 1;
            if *apple > 0 {
            heap.push(State{day: idx + duration - 1, num: *apple});
            }

            while let Some(State{day, num})= heap.pop() {
                if day < idx {
                    continue;
                }
                if num <= 0 {
                    continue;
                }
                total += 1;
                //println!["{:?}: {:?}", i, total];
                heap.push(State{day, num: num - 1});
                break;
            }
        }
        println!["{:?}",heap];
        idx += 1;
        for i in idx..40_000 {
            while let Some(State{day, num})= heap.pop() {
                if day < i {
                    continue;
                }
                if num <= 0 {
                    continue;
                }
                total += 1;
                //println!["{:?}: {:?}", i, total];
                heap.push(State{day, num: num - 1});
                break;
            }
        }
        total
    }
}
