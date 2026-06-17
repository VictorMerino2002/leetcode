use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::<i32, usize>::new();

        for i in 0..nums.len() {
            let n = nums[i];
            if let Some(idx) = map.get(&n) {
                return vec![*idx as i32, i as i32];
            }
            map.insert(target - n, i);
        }
        unreachable!();
    }
}
