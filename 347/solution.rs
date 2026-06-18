use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::<i32, u32>::new();

        for n in nums {
            *map.entry(n).or_insert(0) += 1;
        }

        let mut kv_list = map.into_iter().collect::<Vec<(i32, u32)>>();
        kv_list.sort_by_key(|(_, v)| std::cmp::Reverse(*v));

        let mut res = Vec::new();

        for i in 0..k {
            res.push(kv_list[i as usize].0);
        }
        res
    }
}
