struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }
}

#[cfg(test)]
mod tests {
    use crate::no292::Solution;

    #[test]
    fn test_can_win_nim() {
        assert_eq!(false, Solution::can_win_nim(4));
        assert_eq!(true, Solution::can_win_nim(1));
        assert_eq!(true, Solution::can_win_nim(3));
    }
}
