use std::cmp::Ordering;

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let n = matrix.len();
        let m = matrix[0].len();
        let length = m * n;

        let mut start: i32 = 0;
        let mut end: i32 = length as i32 - 1;

        while start <= end {
            let mid = (start + end) / 2;
            let mid_val = matrix[mid as usize / m][mid as usize % m];
            match mid_val.cmp(&target) {
                Ordering::Equal => {
                    return true;
                }
                Ordering::Less => {
                    start = mid + 1;
                }
                Ordering::Greater => {
                    end = mid - 1;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::no74::Solution;

    #[test]
    fn test_search_matrix_1() {
        let mut vec = Vec::new();
        let mut vec1 = Vec::new();
        let mut vec2 = Vec::new();
        vec1.push(1);
        vec1.push(2);
        vec1.push(3);
        vec2.push(4);
        vec2.push(5);
        vec2.push(6);
        vec.push(vec1);
        vec.push(vec2);
        assert_eq!(false, Solution::search_matrix(vec.clone(), 7));
        assert_eq!(true, Solution::search_matrix(vec, 2))
    }

    #[test]
    fn test_search_matrix_2() {
        let mut vec = Vec::new();
        let mut vec1 = Vec::new();
        vec1.push(1);
        vec.push(vec1);
        assert_eq!(false, Solution::search_matrix(vec.clone(), 0));
        assert_eq!(true, Solution::search_matrix(vec, 1));
    }
}
