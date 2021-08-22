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
    }
}
