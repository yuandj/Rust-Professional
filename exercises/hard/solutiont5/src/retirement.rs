// src/retirement.rs
pub fn retire_time(time: &str, tp: &str) -> String {
    let (y, m) = parse_time(time);
    let (base_age, delay) = match tp {
        "男职工" => (60, male_delay(y, m)),
        "原法定退休年龄55周岁女职工" => (55, ((y - 1970).max(0) * 4).min(36)),
        "原法定退休年龄50周岁女职工" => (50, 60),
        _ => panic!("无效人员类型"),
    };

    let total_months = (y + base_age) * 12 + m - 1 + delay;
    let (ry, rm) = (total_months / 12, (total_months % 12) + 1);

    let age = base_age as f32 + delay as f32 / 12.0;
    let formatted_age = format!("{:.2}", age)
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string();

    format!("{}-{:02},{},{}", ry, rm, formatted_age, delay)
}

#[inline]
fn parse_time(s: &str) -> (i32, i32) {
    let (y, m) = s.split_once('-').unwrap();
    (y.parse().unwrap(), m.parse().unwrap())
}

#[inline]
fn male_delay(y: i32, m: i32) -> i32 {
    match y {
        1963 | 1964 => 0, // 1963-1964年出生无延迟
        1965 => {
            // 1965年特殊处理月份
            if m <= 4 { 1 } else { 3 }
        }
        _ => ((y - 1963) * 3).clamp(0, 36), // 其他年份每年+3个月
    }
}