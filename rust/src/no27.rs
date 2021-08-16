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
