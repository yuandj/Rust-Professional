// src/tests.rs
use solution1::count_distinct::new_count_distinct;

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASES: &[(&str, usize)] = &[
        ("a,b,c,a,e,cd", 5),
        ("a,b,a,a,e,cd", 4),
        ("j,a,c,d,e,z", 6),
        ("a,b,c,好,好,爱", 5),
        ("a,b,c,0,e,cd", 6),
    ];

    #[test]
    fn test_count() {
        let mut total_score = 0.0;
        for (input, expected) in TEST_CASES {
            let result = new_count_distinct(*input);
            if result == *expected {
                total_score += 20.0;
            }
        }
        println!("Total score: {:.2}", total_score);
        assert_eq!(total_score, 100.0);
    }
}