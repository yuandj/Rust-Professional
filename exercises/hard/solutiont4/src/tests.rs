// src/tests.rs
use solutiont4::calc_time::{Date, time_info, next_a_stock_opening};

#[cfg(test)]
mod tests {
    use super::*;
    
    const TEST_CASES: &[(&str, &str)] = &[
        ("2025-01-01", "1,3,1,364,28,0"),
        ("2025-01-18", "3,6,18,347,11,1"),
        // ...其他测试用例...
    ];

    #[test]
    fn test_all_cases() {
        let mut score = 0.0;
        for (input, expected) in TEST_CASES {
            let result = time_info(input);
            if &result == expected {
                score += 10.0;
            } else {
                println!("Test failed: {} => {}", input, result);
            }
        }
        assert_eq!(score, 100.0);
    }

    #[test]
    fn test_next_opening() {
        let date = Date::from_ymd(2025, 2, 9).unwrap();
        let next = next_a_stock_opening(date);
        assert_eq!(next, Date::from_ymd(2025, 2, 10).unwrap());
    }
}