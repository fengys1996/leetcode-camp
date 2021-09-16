use std::cmp::Ordering;

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
                }
                res_vec[1] = i as i32;
                index += 1;
            }
        });
        res_vec
    }

    #[allow(dead_code)]
    pub fn search_range_great(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        let mut position = -1;
        while left <= right {
            let mid = left + ((right - left) >> 1);
            match target.cmp(&nums[mid as usize]) {
                Ordering::Equal => {
                    let pre = mid - 1;
                    if pre >= 0 && nums[pre as usize] == target {
                        right = mid - 1;
                    } else {
                        position = mid;
                        break;
                    }
                }
                Ordering::Less => {
                    right = mid - 1;
                }
                Ordering::Greater => {
                    left = mid + 1;
                }
            }
        }
        let mut res_vec = vec![position, position];
        if position == -1 {
            return res_vec;
        }
        for i in (position + 1)..(nums.len() as i32) {
            if nums[i as usize] != target {
                res_vec[1] = i - 1;
                break;
            }
            if i == nums.len() as i32 - 1 {
                res_vec[1] = nums.len() as i32 - 1;
            }
        }
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
        assert_eq!(
            vec![3, 4],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8)
        );
    }

    #[test]
    fn test_search_range_great() {
        assert_eq!(vec![1, 1], Solution::search_range_great(vec![1, 2, 3], 2));
        assert_eq!(vec![-1, -1], Solution::search_range_great(vec![1, 2, 3], 4));
        assert_eq!(vec![0, 0], Solution::search_range_great(vec![1], 1));
        assert_eq!(
            vec![3, 4],
            Solution::search_range_great(vec![5, 7, 7, 8, 8, 10], 8)
        );
    }
}
