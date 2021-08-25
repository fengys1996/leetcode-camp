struct Solution;

impl Solution {
    #[allow(dead_code)]
    fn match_brackets(c1: &char, c2: &char) -> bool {
        let mut is_match: bool = false;
        match (c1, c2) {
            ('{', '}') => {
                is_match = true;
            }
            ('[', ']') => {
                is_match = true;
            }
            ('(', ')') => {
                is_match = true;
            }
            _ => {}
        }
        is_match
    }

    #[allow(dead_code)]
    pub fn is_valid(s: String) -> bool {
        let length = s.len();
        if length == 0 {
            return true;
        }
        if length % 2 == 1 {
            return false;
        }
        let mut stack = vec![];
        for c in s.chars() {
            if stack.is_empty() || !Solution::match_brackets(stack.last().unwrap(), &c) {
                stack.push(c);
            } else {
                stack.pop();
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::no20::Solution;

    #[test]
    fn test_no_20_solution() {
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
        assert_eq!(Solution::is_valid("(}".to_string()), false);
    }
}
