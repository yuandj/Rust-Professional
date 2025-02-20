// src/retirement.rs
pub fn retire_time(time: &str, tp: &str) -> String {
    let (y, m) = parse_time(time);
    let (base, delay) = get_params(y, tp);

    let total = (y + base) * 12 + m - 1 + delay;
    let ry = total / 12;
    let rm = (total % 12) + 1;

    let (whole, rem) = (delay / 12, delay % 12);
    let age = if rem == 0 {
        (base + whole).to_string()
    } else {
        let cents = (rem * 100) / 12;
        format!("{}.{:02}", base + whole, cents)
    };

    format!("{}-{:02},{},{}", ry, rm, age, delay)
}

#[inline]
fn parse_time(s: &str) -> (i32, i32) {
    let bytes = s.as_bytes();
    let dash = bytes.iter().position(|&b| b == b'-').unwrap();
    let y = parse_num(&bytes[..dash]);
    let m = parse_num(&bytes[dash+1..]);
    (y, m)
}

#[inline]
fn parse_num(b: &[u8]) -> i32 {
    b.iter().fold(0, |n, &c| n * 10 + (c - b'0') as i32)
}

#[inline]
fn get_params(y: i32, tp: &str) -> (i32, i32) {
    match tp {
        "男职工" => (60, (y.saturating_sub(1963) * 3).min(36)),
        "原法定退休年龄55周岁女职工" => (55, (y.saturating_sub(1970) * 4).min(36)),
        "原法定退休年龄50周岁女职工" => (50, 60),
        _ => panic!("无效类型"),
    }
}