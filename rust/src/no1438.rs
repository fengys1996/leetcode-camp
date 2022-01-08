use std::{
    cmp::max,
    collections::{BTreeMap, VecDeque},
};
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

    #[allow(dead_code)]
    pub fn longest_subarray_1(nums: Vec<i32>, limit: i32) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut res = 0;
        let mut min_queue: VecDeque<i32> = VecDeque::new();
        let mut max_queue: VecDeque<i32> = VecDeque::new();
        let len = nums.len();
        while right < len {
            while !min_queue.is_empty() && nums[right] < *min_queue.back().unwrap() {
                min_queue.pop_back();
            }
            while !max_queue.is_empty() && nums[right] > *max_queue.back().unwrap() {
                max_queue.pop_back();
            }
            min_queue.push_back(nums[right]);
            max_queue.push_back(nums[right]);
            while max_queue.front().unwrap() - min_queue.front().unwrap() > limit {
                if nums[left] == *max_queue.front().unwrap() {
                    max_queue.pop_front();
                }
                if nums[left] == *min_queue.front().unwrap() {
                    min_queue.pop_front();
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
        assert_eq!(3, Solution::longest_subarray(vec![8, 7, 6, 5], 2));
    }

    #[test]
    fn test_longest_subarray_1() {
        assert_eq!(2, Solution::longest_subarray_1(vec![8, 2, 4, 7], 4));
        assert_eq!(4, Solution::longest_subarray_1(vec![10, 1, 2, 4, 7, 2], 5));
        assert_eq!(
            3,
            Solution::longest_subarray_1(vec![2, 2, 2, 4, 4, 2, 2], 0)
        );
        assert_eq!(3, Solution::longest_subarray_1(vec![8, 7, 6, 5], 2));
    }
}
