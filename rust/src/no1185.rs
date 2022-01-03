struct Solution;

const WEEK: [&str; 7] = [
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
];

const DAY_OF_MONTH: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

impl Solution {
    #[inline]
    fn leap_year(year: i32) -> bool {
        assert!(year >= 1971 || year <= 2100);
        year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
    }

    #[allow(dead_code)]
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let mut days = (year - 1971) * 365 + (year - 1969) / 4;
        for i in 0..month - 1 {
            days += DAY_OF_MONTH[i as usize];
        }
        if Self::leap_year(year) && month >= 3 {
            days += 1;
        }
        days += day;
        WEEK[((days + 3) % 7) as usize].to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::no1185::Solution;

    #[test]
    fn test_leap_year() {
        assert_eq!(true, Solution::leap_year(2000));
        assert_eq!(true, Solution::leap_year(2004));
        assert_eq!(false, Solution::leap_year(2001));
        assert_eq!(false, Solution::leap_year(2100));
    }

    #[test]
    fn test_day_of_the_week() {
        assert_eq!("Friday", Solution::day_of_the_week(1, 1, 1971));
        assert_eq!("Sunday", Solution::day_of_the_week(18, 7, 1999));
        assert_eq!("Sunday", Solution::day_of_the_week(15, 8, 1993));
        assert_eq!("Monday", Solution::day_of_the_week(29, 2, 2016));
    }
}
