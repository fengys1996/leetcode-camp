struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut strs_vec = Vec::new();
        let mut shortest_len = 0;
        strs.iter().enumerate().for_each(|(index, str)| {
            let str: Vec<char> = str.chars().collect();
            if index == 0 || str.len() < shortest_len {
                shortest_len = str.len();
            }
            strs_vec.push(str);
        });

        let mut result = "".to_string();
        for i in 0..shortest_len {
            let mut tmp_char = '_';
            for (index, str_vec) in (&strs_vec).iter().enumerate() {
                if index == 0 {
                    tmp_char = str_vec[i];
                } else if tmp_char != str_vec[i] {
                    return result;
                }
            }
            result.push(tmp_char);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::no14::Solution;

    #[test]
    fn test_longest_common_prefix() {
        let strs = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        assert_eq!("fl".to_string(), Solution::longest_common_prefix(strs));
        let strs = vec!["dogs".to_string(), "cars".to_string(), "tour".to_string()];
        assert_eq!("".to_string(), Solution::longest_common_prefix(strs));
    }
}
