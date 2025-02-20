pub fn new_birthday_probability(n: u32) -> f64 {
    if n >= 365 {
        return 1.0;
    }
    let mut prob = 1.0;
    for i in 0..n {
        prob *= (365.0 - i as f64) / 365.0;
    }
    let result = 1.0 - prob;
    (result * 10000.0).round() / 10000.0
}