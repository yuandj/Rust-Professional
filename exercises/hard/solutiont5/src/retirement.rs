// src/retirement.rs
pub fn retire_time(time: &str, tp: &str) -> String {
    // 解析输入年月
    let (y, m) = {
        let bytes = time.as_bytes();
        let dash = bytes.iter().position(|&b| b == b'-').unwrap();
        let y = parse_num(&bytes[..dash]);
        let m = parse_num(&bytes[dash+1..]);
        (y, m)
    };

    // 获取基准参数
    let (base, delay) = match tp.as_bytes() {
        b"\xe7\x94\xb7\xe8\x81\x8c\xe5\xb7\xa5" => (60, (y.saturating_sub(1963) * 3).min(36)),  // 男职工
        b"\xe5\x8e\x9f" => {  // 原开头的类型
            if tp.ends_with("55周岁女职工") {
                (55, (y.saturating_sub(1970) * 4).min(36))
            } else {
                (50, 60)
            }
        },
        _ => panic!("无效类型"),
    };

    // 计算总月份
    let total = (y + base) * 12 + m - 1 + delay;
    let (ry, rm) = (total / 12, (total % 12) + 1);

    // 预分配字符串缓冲区
    let mut buf = [b' '; 21];  // 最大长度测试用例为"2063-12,63,36"
    let mut pos = 0;

    // 写入年份
    pos += write_num(&mut buf[pos..], ry);
    buf[pos] = b'-';
    pos += 1;
    
    // 写入月份（两位）
    let (m1, m2) = (rm / 10, rm % 10);
    buf[pos] = b'0' + m1 as u8;
    buf[pos+1] = b'0' + m2 as u8;
    pos += 2;

    // 写入年龄
    buf[pos] = b',';
    pos += 1;
    let whole = base + delay / 12;
    let rem = delay % 12;
    pos += write_num(&mut buf[pos..], whole);
    if rem != 0 {
        buf[pos] = b'.';
        pos += 1;
        let cents = (rem * 100) / 12;
        let c1 = cents / 10;
        let c2 = cents % 10;
        buf[pos] = b'0' + c1 as u8;
        buf[pos+1] = b'0' + c2 as u8;
        pos += 2;
    }

    // 写入延迟月数
    buf[pos] = b',';
    pos += 1;
    pos += write_num(&mut buf[pos..], delay);

    // 转换为字符串
    String::from_utf8(buf[..pos].to_vec()).unwrap()
}

#[inline]
fn parse_num(b: &[u8]) -> i32 {
    b.iter().fold(0, |n, &c| n * 10 + (c - b'0') as i32)
}

#[inline]
fn write_num(buf: &mut [u8], mut n: i32) -> usize {
    let mut i = 0;
    let mut started = false;
    for d in (0..=4).rev() {
        let p = 10i32.pow(d);
        let digit = n / p;
        if digit != 0 || started || p == 1 {
            buf[i] = b'0' + digit as u8;
            i += 1;
            started = true;
        }
        n %= p;
    }
    i
}