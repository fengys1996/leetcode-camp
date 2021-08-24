use std::collections::HashMap;

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map2: HashMap<i32, i32> = HashMap::new();
        nums2.iter().for_each(|&item| {
            map2.entry(item).and_modify(|e| *e += 1).or_insert(1);
        });

        nums1
            .into_iter()
            .filter(|&item| {
                let mut res = false;
                map2.entry(item).and_modify(|e| {
                    *e -= 1;
                    res = *e >= 0;
                });
                res
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::no350::Solution;

    #[test]
    fn test_intersection() {
        let nums1 = vec![1, 2, 2, 5];
        let nums2 = vec![2, 2];
        let vec_expect = vec![2, 2];
        assert_eq!(vec_expect, Solution::intersection(nums1, nums2));
    }
}
