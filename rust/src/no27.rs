struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut fp = 0;
        let mut bp = nums.len() as i32 - 1;
        while fp <= bp {
            if nums[fp as usize] == val {
                nums[fp as usize] = nums[bp as usize];
                bp -= 1;
            } else {
                fp += 1;
            }
        }
        bp + 1
    }
}

#[cfg(test)]
mod test {
    use crate::no27::Solution;

    #[test]
    fn test_no_21_solution_1() {
        let mut nums = Vec::new();
        nums.push(1);
        nums.push(2);
        nums.push(3);
        nums.push(4);
        nums.push(2);
        nums.push(6);
        assert_eq!(4, Solution::remove_element(&mut nums, 2));
    }

    #[test]
    fn test_no_21_solution_2() {
        let mut nums = Vec::new();
        nums.push(2);
        assert_eq!(0, Solution::remove_element(&mut nums, 2));
    }

    #[test]
    fn test_no_21_solution_3() {
        let mut nums = Vec::new();
        assert_eq!(0, Solution::remove_element(&mut nums, 2));
    }
}
