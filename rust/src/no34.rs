struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res_vec = vec![-1, -1];
        let mut index = 0;
        nums.iter().enumerate().for_each(|(i, &item)| {
            if item == target {
                if index == 0 {
                    res_vec[0] = i as i32;
                    res_vec[1] = i as i32;
                } else {
                    res_vec[1] = i as i32;
                }
                index += 1;
            }
        });
        res_vec
    }
}

#[cfg(test)]
mod tests {
    use crate::no34::Solution;

    #[test]
    fn test_search_range() {
        assert_eq!(vec![1, 1], Solution::search_range(vec![1, 2, 3], 2));
        assert_eq!(vec![-1, -1], Solution::search_range(vec![1, 2, 3], 4));
        assert_eq!(vec![0, 0], Solution::search_range(vec![1], 1));
    }
}
