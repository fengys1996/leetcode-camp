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
            match nums[mid as usize].cmp(&target) {
                Ordering::Equal => return mid as i32,
                Ordering::Greater => {
                    if nums[left as usize] <= target {
                        right = mid - 1;
                    } else {
                        left = mid + 1;
                    }
                }
                Ordering::Less => {
                    if nums[right as usize] >= target {
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
    }
}
