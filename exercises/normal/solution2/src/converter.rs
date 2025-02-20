pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // 解析输入字符串，分离数字部分和原进制
    let (num_part, base_part) = num_str.split_once('(').expect("Invalid input format");
    let base_from = base_part.strip_suffix(')').unwrap().parse::<u32>().unwrap();

    // 将原数字转换为十进制
    let mut decimal: u64 = 0;
    for c in num_part.chars() {
        let digit = c.to_digit(base_from).expect("Invalid digit for base");
        decimal = decimal * u64::from(base_from) + u64::from(digit);
    }

    // 处理十进制为0的情况
    if decimal == 0 {
        return "0".to_string();
    }

    // 将十进制转换为目标进制
    let to_base = to_base as u64;
    let mut result = Vec::new();
    while decimal > 0 {
        let rem = decimal % to_base;
        decimal /= to_base;
        let c = match rem {
            0..=9 => (b'0' + rem as u8) as char,
            _ => (b'a' + (rem - 10) as u8) as char,
        };
        result.push(c);
    }
    result.reverse();
    result.into_iter().collect()
}