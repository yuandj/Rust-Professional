#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

impl Date {
    fn format(&self) -> String {
        format!("{}-{:02}-{:02}", self.year, self.month, self.day)
    }

    pub fn from_ymd(year: i32, month: i32, day: i32) -> Option<Date> {
        if month < 1 || month > 12 || day < 1 || day > 31 {
            return None;
        }
        let max_day = Self::days_in_month(year, month);
        if day > max_day {
            return None;
        }
        Some(Date { year, month, day })
    }

    fn days_in_month(year: i32, month: i32) -> i32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if Self::is_leap_year(year) { 29 } else { 28 }
            }
            _ => 0,
        }
    }

    fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
    }

    pub fn add_days(&self, days: i32) -> Date {
        let mut date = *self;
        let mut days = days;
        while days > 0 {
            let max_day = Self::days_in_month(date.year, date.month);
            let remaining = max_day - date.day + 1;
            if days >= remaining {
                days -= remaining;
                date.day = 1;
                date.month += 1;
                if date.month > 12 {
                    date.month = 1;
                    date.year += 1;
                }
            } else {
                date.day += days;
                days = 0;
            }
        }
        date
    }

    pub fn sub_days(&self, days: i32) -> Date {
        let mut date = *self;
        let mut days = days;
        while days > 0 {
            if date.day > days {
                date.day -= days;
                days = 0;
            } else {
                days -= date.day;
                date.month -= 1;
                if date.month < 1 {
                    date.month = 12;
                    date.year -= 1;
                }
                date.day = Self::days_in_month(date.year, date.month);
            }
        }
        date
    }

    pub fn days_since(&self, other: &Date) -> i32 {
        let mut days = 0;
        let mut date = *other;
        while date < *self {
            date = date.add_days(1);
            days += 1;
        }
        days
    }

    // 在Date的weekday方法中修正计算公式
// 在 calc_time.rs 中修改 weekday 方法
pub fn weekday(&self) -> i32 {
    let (m, y) = if self.month < 3 {
        (self.month + 12, self.year - 1)
    } else {
        (self.month, self.year)
    };
    let q = self.day;
    let k = y % 100;
    let j = y / 100;
    let h = (q + (13 * (m + 1) / 5) + k + (k / 4) + (j / 4) + 5 * j) % 7;
    (h + 6) % 7 // 修正映射：0=周日，1=周一...6=周六
}

// 修改 iso_weekday 转换逻辑
pub fn iso_weekday(&self) -> i32 {
    (self.weekday() + 6) % 7 + 1 // 1=周一，7=周日
}

    pub fn day_of_year(&self) -> i32 {
        let mut day_count = self.day;
        for m in 1..self.month {
            day_count += Self::days_in_month(self.year, m);
        }
        day_count
    }
}

fn get_first_thursday(year: i32) -> Date {
    // 寻找第一个ISO周四（即实际周四）
    for day in 1..=7 {
        let date = Date::from_ymd(year, 1, day).unwrap();
        if date.weekday() == 4 { // 使用原始周四定义（Zeller计算结果4对应周四）
            return date;
        }
    }
    unreachable!()
}

fn iso_week_number(date: Date) -> i32 {
    let thursday = date_to_thursday(date);
    let year = thursday.year;
    let first_thurs = get_first_thursday(year);
    let days = thursday.days_since(&first_thurs);
    days / 7 + 1
}

fn date_to_thursday(date: Date) -> Date {
    let iso_weekday = date.iso_weekday();
    let delta = if iso_weekday >= 4 {
        iso_weekday - 4
    } else {
        4 - iso_weekday
    };
    if iso_weekday >= 4 {
        date.sub_days(delta)
    } else {
        date.add_days(4 - iso_weekday)
    }
}

fn get_spring_festival(year: i32) -> Date {
    match year {
        2025 => Date::from_ymd(2025, 1, 29).unwrap(), // 正确春节日期
        2026 => Date::from_ymd(2026, 2, 17).unwrap(),
        2027 => Date::from_ymd(2027, 2, 6).unwrap(),
        2028 => Date::from_ymd(2028, 1, 26).unwrap(),
        _ => panic!("Spring festival not predefined for year {}", year),
    }
}

fn days_until_spring(current: Date) -> i32 {
    let current_year = current.year;
    let spring = get_spring_festival(current_year);
    if current < spring {
        spring.days_since(&current)
    } else {
        let next_spring = get_spring_festival(current_year + 1);
        next_spring.days_since(&current)
    }
}

pub fn next_a_stock_opening(current: Date) -> Date {
    let mut date = current.add_days(1);
    loop {
        // 跳过周末（周一至周五为交易日）
        while date.iso_weekday() > 5 { // 6=周六,7=周日
            date = date.add_days(1);
        }

        // 新增劳动节假期处理（假设放假5天：5月1日-5月5日）
        if date.month == 5 && date.day >= 1 && date.day <= 5 {
            date = Date::from_ymd(date.year, 5, 6).unwrap(); // 直接跳到5月6日
            continue;
        }
        
        // 新增元旦假期处理（1月1日）
        if date.month == 1 && date.day == 1 {
            date = date.add_days(1);
            continue;
        }
        
        // 检查春节假期（当前年份）
        let spring_start = get_spring_festival(date.year);
        let spring_end = spring_start.add_days(6);
        if date >= spring_start && date <= spring_end {
            date = spring_end.add_days(1);
            continue;
        }
        
        // 检查下一年春节（跨年情况）
        let next_spring = get_spring_festival(date.year + 1);
        let next_spring_end = next_spring.add_days(6);
        if date >= next_spring && date <= next_spring_end {
            date = next_spring_end.add_days(1);
            continue;
        }
        
        break;
    }
    date
}

pub fn time_info(time: &str) -> String {
    let parts: Vec<&str> = time.split('-').collect();
    let year = parts[0].parse().unwrap();
    let month = parts[1].parse().unwrap();
    let day = parts[2].parse().unwrap();
    let date = Date::from_ymd(year, month, day).unwrap();

    let day_of_year = date.day_of_year();
    let days_left = if Date::is_leap_year(year) { 366 - day_of_year } else { 365 - day_of_year };
    let iso_week = iso_week_number(date);
    let weekday = date.iso_weekday();
    let spring_days = days_until_spring(date);
    let next_open_date = next_a_stock_opening(date);
    let next_open_days = next_open_date.days_since(&date) - 1; // 保持原逻辑正确

    // 调试输出
    println!(
        "Input: {} => 第{}周,周{},年天数{},剩余{},春节{},开盘日{} (差{}天)",
        time, iso_week, weekday, day_of_year, days_left, spring_days, 
        next_open_date.format(), next_open_days
    );

    format!("{},{},{},{},{},{}", iso_week, weekday, day_of_year, days_left, spring_days, next_open_days)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2025_01_18() {
        let result = time_info("2025-01-18");
        assert_eq!(result, "3,6,18,347,11,1");
    }
}