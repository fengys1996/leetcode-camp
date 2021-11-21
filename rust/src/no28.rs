struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let h_len = haystack.len();
        let n_len = needle.len();
        if n_len == 0 {
            return 0;
        }
        if n_len > h_len {
            return -1;
        }
        let needle_chars: Vec<char> = needle.chars().collect();
        let haystack_chars: Vec<char> = haystack.chars().collect();
        for i in 0..(h_len - n_len + 1) {
            let mut flag = true;
            let mut k = i;
            for needle_char in needle_chars.iter().take(n_len) {
                if needle_char != &haystack_chars[k] {
                    flag = false;
                }
                k += 1;
            }
            if flag {
                return i as i32;
            }
        }
        -1
    }

    // todo: implement with use kmp
    #[allow(dead_code)]
    pub fn str_str_kmp(_haystack: String, _needle: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::no28::Solution;

    #[test]
    fn test_str_str() {
        let haystack = "".to_owned();
        let needle = "".to_owned();
        assert_eq!(0, Solution::str_str(haystack, needle));

        let haystack = "hello".to_owned();
        let needle = "ll".to_owned();
        assert_eq!(2, Solution::str_str(haystack, needle));

        let haystack = "aaaa".to_owned();
        let needle = "bba".to_owned();
        assert_eq!(-1, Solution::str_str(haystack, needle));
 
        let haystack = "".to_owned();
        let needle = "bba".to_owned();
        assert_eq!(-1, Solution::str_str(haystack, needle));

        let haystack = "a".to_owned();
        let needle = "a".to_owned();
        assert_eq!(0, Solution::str_str(haystack, needle));
    }
}
