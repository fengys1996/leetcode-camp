use std::collections::HashMap;

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_map: HashMap<i32, i32> = HashMap::new();
        let mut index = 0;
        let mut vec_result: Vec<i32> = Vec::with_capacity(2);
        nums.iter().for_each(|num| {
            let other_number = target - num;
            if hash_map.contains_key(&other_number) {
                vec_result.push(*hash_map.get(&other_number).unwrap());
                vec_result.push(index);
            }
            hash_map.insert(*num, index);
            index += 1;
        });
        vec_result
    }
}

#[test]
fn test_no_1_solution() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(Solution::two_sum(nums, target), vec![0, 1])
}
