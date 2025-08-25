pub struct TimePoint {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    utc_offset: i32
}

impl TimePoint {
    pub fn new(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32, utc_offset: i32) -> Self {
        return TimePoint { year, month, day, hour, minute, second, utc_offset };
    }

    pub fn to_string(&self) -> String {
        return format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC{:+02}",
            self.year, self.month, self.day, self.hour, self.minute, self.second, self.utc_offset
        );
    }

    pub fn calc_bias_with(&self, tp2: &TimePoint) -> i64 {
        let total_seconds1 = self.to_seconds();
        let total_seconds2 = tp2.to_seconds();
        return total_seconds1 - total_seconds2;
    }

    pub fn calc_duration_with(&self, timepoint2: &TimePoint) -> i64 {
        return TimePoint::calc_bias_with(self, timepoint2).abs();
    }

    pub fn is_after(&self, other: &TimePoint) -> bool {
        return self.calc_bias_with(other) > 0;
    }

    pub fn is_before(&self, other: &TimePoint) -> bool {
        return self.calc_bias_with(other) < 0;
    }

    pub fn is(&self, other: &TimePoint) -> bool {
        return self.calc_bias_with(other) == 0;
    }

    // TODO: tp + tp, tp - tp, difficault is consider UTC

    // TODO: tp convert, consider UTC

    fn to_seconds(&self) -> i64 {
        fn is_leap_year(year: i32) -> bool {
            return (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
        }

        fn year_to_days(year: i32) -> i64 {
            return if is_leap_year(year) { 366 } else { 365 };
        }

        fn month_to_days(month: u32, year: i32) -> i64 {
            match month {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                4 | 6 | 9 | 11 => 30,
                2 => if is_leap_year(year) { 29 } else { 28 },
                _ => 0,
            }
        }

        let mut total_days: i64 = 0;

        for year in 1970..self.year {
            total_days += year_to_days(year);
        }

        for month in 1..self.month {
            total_days += month_to_days(month, self.year);
        }

        total_days += (self.day - 1) as i64;

        return total_days * 86400 + (self.hour as i64) * 3600 + (self.minute as i64) * 60 + (self.second as i64) - (self.utc_offset as i64) * 3600;
    }
}

#[cfg(test)]
mod test_time_point {
    use super::*;

    #[test]
    fn test_in_same_day() {
        let tp1 = TimePoint::new(2023, 3, 15, 12, 0, 0, 1);
        let tp2 = TimePoint::new(2023, 3, 15, 13, 0, 0, 1);
        assert_eq!(TimePoint::calc_duration_with(&tp1, &tp2), 3600);
    }

    #[test]
    fn test_in_different_day() {
        let tp1 = TimePoint::new(2023, 3, 15, 12, 0, 0, 1);
        let tp2 = TimePoint::new(2023, 3, 16, 12, 0, 0, 1);
        assert_eq!(TimePoint::calc_duration_with(&tp1, &tp2), 86400);
    }

    #[test]
    fn test_leap_year() {
        let tp1 = TimePoint::new(2020, 2, 29, 12, 0, 0, 1);
        let tp2 = TimePoint::new(2020, 3, 1, 12, 0, 0, 1);
        assert_eq!(TimePoint::calc_duration_with(&tp1, &tp2), 86400);
    }

    #[test]
    fn test_with_different_timezone() {
        let tp1 = TimePoint::new(2023, 3, 15, 12, 0, 0, 1);
        let tp2 = TimePoint::new(2023, 3, 15, 12, 0, 0, 2);
        assert_eq!(TimePoint::calc_duration_with(&tp1, &tp2), 3600);
    }

    #[test]
    fn test_signed_timezone() {
        let tp1 = TimePoint::new(2023, 3, 15, 12, 0, 0, 1);
        let tp2 = TimePoint::new(2023, 3, 15, 12, 0, 0, -1);
        assert_eq!(TimePoint::calc_duration_with(&tp1, &tp2), 7200);
    }

    #[test]
    fn test_calc_tp_bias_with_same_utc() {
        let tp1 = TimePoint::new(2023, 3, 15, 12, 0, 0, 1);
        let tp2 = TimePoint::new(2023, 3, 15, 13, 0, 0, 1);
        assert_eq!(TimePoint::calc_bias_with(&tp1, &tp2), -3600);
    }

    #[test]
    fn test_calc_tp_bias_with_different_utc() {
        let tp1 = TimePoint::new(2023, 3, 15, 12, 0, 0, 1);
        let tp2 = TimePoint::new(2023, 3, 15, 12, 0, 0, 2);
        assert_eq!(TimePoint::calc_bias_with(&tp1, &tp2), 3600);
    }

    #[test]
    fn test_to_string() {
        let tp = TimePoint::new(2023, 3, 15, 12, 0, 0, 1);
        assert_eq!(tp.to_string(), "2023-03-15 12:00:00 UTC+1");
        let tp2 = TimePoint::new(2023, 3, 15, 12, 0, 0, -1);
        assert_eq!(tp2.to_string(), "2023-03-15 12:00:00 UTC-1");
    }

    #[test]
    fn test_is_after() {
        let tp1 = TimePoint::new(2023, 3, 15, 12, 0, 0, 1);
        let tp2 = TimePoint::new(2023, 3, 15, 13, 0, 0, 1);
        assert_eq!(tp1.is_after(&tp2), false);
        assert_eq!(tp2.is_after(&tp1), true);
    }

    #[test]
    fn test_is_before() {
        let tp1 = TimePoint::new(2023, 3, 15, 12, 0, 0, 1);
        let tp2 = TimePoint::new(2023, 3, 15, 13, 0, 0, 1);
        assert_eq!(tp1.is_before(&tp2), true);
        assert_eq!(tp2.is_before(&tp1), false);
    }

    #[test]
    fn test_is() {
        let tp1 = TimePoint::new(2023, 3, 15, 12, 0, 0, 1);
        let tp2 = TimePoint::new(2023, 3, 15, 12, 0, 0, 1);
        assert_eq!(tp1.is(&tp2), true);
        assert_eq!(tp2.is(&tp1), true);
    }
}
