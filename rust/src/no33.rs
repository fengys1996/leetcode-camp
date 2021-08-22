use std::cmp::Ordering;

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        for (i, &item) in nums.iter().enumerate() {
            if item == target {
                return i as i32;
            }
        }
        -1
    }

    #[allow(dead_code)]
    pub fn search_great(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = nums.len() as i32 - 1;
        while left <= right {
            let mid = left + ((right - left) >> 1);
            let left_val = nums[left as usize];
            let right_val = nums[right as usize];
            let mid_val = nums[mid as usize];
            if mid_val == target {
                return mid;
            }
            match left_val.cmp(&mid_val) {
                Ordering::Less => {
                    if left_val <= target && target < mid_val {
                        right = mid - 1;
                    } else {
                        left = mid + 1;
                    }
                }
                Ordering::Equal => left = mid + 1,
                Ordering::Greater => {
                    if mid_val < target && target <= right_val {
                        left = mid + 1;
                    } else {
                        right = mid - 1;
                    }
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use crate::no33::Solution;

    #[test]
    fn test_search() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        assert_eq!(4, Solution::search(nums, target));

        let nums = vec![0];
        let target = 0;
        assert_eq!(0, Solution::search(nums, target));

        let nums = vec![1];
        let target = 2;
        assert_eq!(-1, Solution::search(nums, target));
    }

    #[test]
    fn test_search_great() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        assert_eq!(4, Solution::search_great(nums, target));

        let nums = vec![0];
        let target = 0;
        assert_eq!(0, Solution::search_great(nums, target));

        let nums = vec![1];
        let target = 2;
        assert_eq!(-1, Solution::search_great(nums, target));

        let nums = vec![4, 5, 6, 7, 8, 1, 2, 3];
        let target = 8;
        assert_eq!(4, Solution::search_great(nums, target));

        let nums = vec![3, 1];
        let target = 1;
        assert_eq!(1, Solution::search_great(nums, target));
    }
}
