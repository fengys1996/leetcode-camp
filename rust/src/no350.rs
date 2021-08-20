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
                let res = map2.contains_key(&item);
                if res {
                    let val = *map2.get(&item).unwrap();
                    if val > 1 {
                        map2.insert(item, val - 1);
                    } else {
                        map2.remove(&item);
                    }
                }
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
        vec_expect.push(2);
        assert_eq!(vec_expect, Solution::intersection(nums1, nums2));
    }
}
