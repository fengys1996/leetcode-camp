use std::{cmp::max, collections::BTreeMap};
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut tree_map: BTreeMap<i32, i32> = BTreeMap::new();
        let mut res = 0_usize;
        let mut left = 0_usize;
        let mut right = 0_usize;
        let len = nums.len();
        while right < len {
            *tree_map.entry(nums[right]).or_insert(0) += 1;
            while tree_map.last_key_value().unwrap().0 - tree_map.first_key_value().unwrap().0
                > limit
            {
                let update_no = *tree_map.get(&nums[left]).unwrap() - 1;
                if update_no <= 0 {
                    tree_map.remove(&nums[left]);
                } else {
                    tree_map.insert(nums[left], update_no);
                }
                left += 1;
            }
            res = max(res, right - left + 1);
            right += 1;
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::no1438::Solution;

    #[test]
    fn test_longest_subarray() {
        assert_eq!(2, Solution::longest_subarray(vec![8, 2, 4, 7], 4));
        assert_eq!(4, Solution::longest_subarray(vec![10, 1, 2, 4, 7, 2], 5));
        assert_eq!(3, Solution::longest_subarray(vec![2, 2, 2, 4, 4, 2, 2], 0));
    }
}
