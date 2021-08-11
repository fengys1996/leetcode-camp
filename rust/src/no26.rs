struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let length_of_nums = nums.len();
        if length_of_nums < 2 {
            return length_of_nums as i32;
        }
        let mut first = 0;
        let mut next = first + 1;
        while next < length_of_nums {
            if nums[first] != nums[next] {
                if first + 1 != next {
                    nums[first + 1] = nums[next];
                }
                first += 1;
            }
            next += 1;
        }
        (first + 1) as i32
    }

    #[allow(dead_code)]
    pub fn remove_duplicates1(nums: &mut Vec<i32>) -> i32 {
        // todo learn vec.dedup
        nums.dedup();
        nums.len() as i32
    }
}

#[allow(dead_code)]
fn do_test_no_26_solution(func: fn(&mut Vec<i32>) -> i32) {
    let mut test_case_1 = vec![1, 1, 2, 3, 3];
    let length = func(&mut test_case_1);
    assert_eq!(length, 3);
    assert_eq!(test_case_1[0..length as usize], vec![1, 2, 3][..]);

    let mut test_case_2 = vec![];
    let length = func(&mut test_case_2);
    assert_eq!(length, 0);
    assert_eq!(test_case_2[0..length as usize], vec![][..]);
}

#[test]
fn test_no_26_solution() {
    do_test_no_26_solution(Solution::remove_duplicates);
    do_test_no_26_solution(Solution::remove_duplicates1);
}
