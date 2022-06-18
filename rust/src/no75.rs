use std::usize;

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let len = nums.len();
        let mut ptr = 0;
        for i in 0..2 {
            for j in 0..len {
                if nums[j] == i {
                    Self::swap(nums, j, ptr);
                    ptr += 1;
                }
            }
        }
    }

    fn swap(nums: &mut [i32], i: usize, j: usize) {
        nums.swap(i, j);
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_sort_colors() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);

        let mut nums = vec![2, 0, 1];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);
        
        let mut nums = vec![2, 2, 2];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![2, 2, 2]);

        let mut nums = vec![];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![]);
    }
}
