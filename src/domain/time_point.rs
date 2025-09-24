#[derive(PartialEq, Eq, Debug)]
pub struct TimePoint {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    second: i32,
    utc_offset: i32
}

impl TimePoint {
    pub fn new(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: i32, utc_offset: i32) -> Self {
        if TimePoint::valid_date(year, month, day) == false || TimePoint::valid_time(hour, minute, second) == false {
            return TimePoint { year: 0, month: -1, day: -1, hour: -1, minute: -1, second: -1, utc_offset: 99 };
        }
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

    pub fn to_utc(&self, new_utc_zone: i32) -> TimePoint {
        // convert to standard time zone (STZ UTC+0)
        let mut stz_hour: i32 = self.hour - self.utc_offset;
        let mut stz_day: i32 = self.day;
        let mut stz_month: i32 = self.month;
        let mut stz_year: i32 = self.year;

        let days_in_month = |m: i32, y: i32| -> i32 {
            TimePoint::month_to_days(m, y)
        };

        while stz_hour < 0 {
            stz_hour += 24;
            stz_day -= 1;
            if stz_day < 1 {
                stz_month -= 1;
                if stz_month < 1 {
                    stz_month += 12;
                    stz_year -= 1;
                }
                stz_day += days_in_month(stz_month, stz_year);
            }
        }
        while stz_hour >= 24 {
            stz_hour -= 24;
            stz_day += 1;
            let dim = days_in_month(stz_month, stz_year);
            if stz_day > dim {
                stz_day -= dim;
                stz_month += 1;
                if stz_month > 12 {
                    stz_month -= 12;
                    stz_year += 1;
                }
            }
        }


        // convert to new time zone (NTZ, UTC+n)
        let mut ntz_hour: i32 = stz_hour + new_utc_zone;
        let mut ntz_day: i32 = stz_day;
        let mut ntz_month: i32 = stz_month;
        let mut ntz_year: i32 = stz_year;

        while ntz_hour < 0 {
            ntz_hour += 24;
            ntz_day -= 1;
            if ntz_day < 1 {
                ntz_month -= 1;
                if ntz_month < 1 {
                    ntz_month += 12;
                    ntz_year -= 1;
                }
                ntz_day += days_in_month(ntz_month, ntz_year);
            }
        }
        while ntz_hour >= 24 {
            ntz_hour -= 24;
            ntz_day += 1;
            let dim = days_in_month(ntz_month, ntz_year);
            if ntz_day > dim {
                ntz_day -= dim;
                ntz_month += 1;
                if ntz_month > 12 {
                    ntz_month -= 12;
                    ntz_year += 1;
                }
            }
        }

        return TimePoint::new(ntz_year, ntz_month, ntz_day, ntz_hour, self.minute, self.second, new_utc_zone);
    }

    fn is_leap_year(year: i32) -> bool {
        return (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
    }

    fn year_to_days(year: i32) -> i32 {
        return if TimePoint::is_leap_year(year) { 366 } else { 365 };
    }

    fn month_to_days(month: i32, year: i32) -> i32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => if TimePoint::is_leap_year(year) { 29 } else { 28 },
            _ => 0,
        }
    }

    fn to_seconds(&self) -> i64 {
        let mut total_days: i64 = 0;

        for year in 1970..self.year {
            total_days += TimePoint::year_to_days(year) as i64;
        }

        for month in 1..self.month {
            total_days += TimePoint::month_to_days(month, self.year) as i64;
        }

        total_days += (self.day - 1) as i64;

        return total_days * 86400 + (self.hour as i64) * 3600 + (self.minute as i64) * 60 + (self.second as i64) - (self.utc_offset as i64) * 3600;
    }

    fn valid_date(year: i32, month: i32, day: i32) -> bool {
        if month <= 12 && month >= 1 && day <= TimePoint::month_to_days(month, year) && day >= 1 {
            return true;
        }

        return false;
    }

    fn valid_time(hour: i32, minute:i32, second: i32) -> bool {
        if hour <= 23 && hour >= 0 && minute <= 59 && minute >= 0 && second <= 59 && second >= 0 {
            return true;
        }

        return false;
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

    #[test]
    fn test_is_valid_date() {
        assert_eq!(TimePoint::valid_date(2020, 1, 1), true);
        assert_eq!(TimePoint::valid_date(2020, 1, 55), false);
    }

    #[test]
    fn test_is_valid_time() {
        assert_eq!(TimePoint::valid_time(0, 5, 60), false);
        assert_eq!(TimePoint::valid_time(0, 0, 0), true);
    }

    #[test]
    fn test_to_utc() {
        let origin: TimePoint = TimePoint::new(2020, 1, 1, 23, 0, 0, 8);
        assert_eq!(origin.to_utc(9), TimePoint::new(2020, 1, 2, 0, 0, 0, 9));
        assert_eq!(origin.to_utc(0), TimePoint::new(2020, 1, 1, 15, 0, 0, 0));
        assert_eq!(origin.to_utc(1), TimePoint::new(2020, 1, 1, 16, 0, 0, 1));
        assert_eq!(origin.to_utc(-5), TimePoint::new(2020, 1, 1, 10, 0, 0, -5));
    }

    #[test]
    fn test_to_utc_a() {
        let origin: TimePoint = TimePoint::new(2020, 1, 1, 1, 0, 0, 8);
        assert_eq!(origin.to_utc(9), TimePoint::new(2020, 1, 1, 2, 0, 0, 9));
        assert_eq!(origin.to_utc(0), TimePoint::new(2019, 12, 31, 17, 0, 0, 0));
        assert_eq!(origin.to_utc(1), TimePoint::new(2019, 12, 31, 18, 0, 0, 1));
        assert_eq!(origin.to_utc(-5), TimePoint::new(2019, 12, 31, 12, 0, 0, -5));
    }
}
