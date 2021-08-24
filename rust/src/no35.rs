use std::cmp::Ordering;

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let lenght = nums.len();
        if lenght == 0 {
            return 0;
        }
        let mut left: i32 = 0;
        let mut right: i32 = (lenght - 1) as i32;
        let mut mid: i32 = 0;
        while left <= right {
            mid = (left + right) / 2;
            match nums[mid as usize].cmp(&target) {
                Ordering::Equal => return mid,
                Ordering::Greater => right = mid - 1,
                Ordering::Less => left = mid + 1,
            }
        }
        if nums[mid as usize] > target {
            return mid;
        }
        mid + 1
    }
}

#[cfg(test)]
mod tests {
    use crate::no35::Solution;

    #[test]
    fn test_search_insert() {
        let nums = vec![1, 2, 3, 5];
        assert_eq!(1, Solution::search_insert(nums.clone(), 2));
        assert_eq!(3, Solution::search_insert(nums.clone(), 4));

        let nums = vec![1];
        assert_eq!(0, Solution::search_insert(nums.clone(), 0));
        assert_eq!(0, Solution::search_insert(nums.clone(), 1));
        assert_eq!(1, Solution::search_insert(nums.clone(), 2));

        let nums = vec![];
        assert_eq!(0, Solution::search_insert(nums.clone(), 0));
        assert_eq!(0, Solution::search_insert(nums.clone(), 1));
    }
}
