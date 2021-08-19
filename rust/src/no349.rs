use std::collections::HashSet;

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set2: HashSet<i32> = nums2.into_iter().collect();
        nums1
            .into_iter()
            .filter(|item| set2.contains(item))
            .collect::<HashSet<i32>>()
            .into_iter()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::no349::Solution;

    #[test]
    fn test_intersection() {
        let mut nums1 = Vec::new();
        nums1.push(1);
        nums1.push(2);
        nums1.push(2);
        nums1.push(5);
        let mut nums2 = Vec::new();
        nums2.push(2);
        nums2.push(2);
        let mut vec_expect = Vec::new();
        vec_expect.push(2);
        assert_eq!(vec_expect, Solution::intersection(nums1, nums2));
    }
}
