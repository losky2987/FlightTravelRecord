use chrono::{Datelike, Local, Offset, Timelike};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TimePoint {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
    pub second: i32,
    pub utc_offset: i32
}

impl TimePoint {
    pub fn new(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: i32, utc_offset: i32) -> Self {
        if TimePoint::valid_timepoint(year, month, day, hour, minute, second, utc_offset) == false {
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

    pub fn to_utc(&mut self, new_utc_zone: i32) {
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

        self.year = ntz_year;
        self.month = ntz_month;
        self.day = ntz_day;
        self.hour = ntz_hour;
        self.utc_offset = new_utc_zone;
    }

    pub fn after(&mut self, year: i32, month: i32, day: i32, hour: i32, minute: i32, second: i32){
        self.second += if second >= 0 {second} else {0};
        self.minute += if minute >= 0 {minute} else {0};
        self.hour += if hour >= 0 {hour} else {0};
        self.day += if day >= 0 {day} else {0};
        self.month += if month >= 0 {month} else {0};
        self.year += if year >= 0 {year} else {0};

        if self.second >= 60 {
            self.minute += self.second / 60;
            self.second %= 60;
        }

        if self.minute >= 60 {
            self.hour += self.minute / 60;
            self.minute %= 60;
        }

        if self.hour >= 24 {
            self.day += self.hour / 24;
            self.hour %= 24;
        }

        if self.month > 12 {
            self.year += (self.month - 1) / 12;
            self.month = (self.month - 1) % 12 + 1;
        }

        while self.day > TimePoint::month_to_days(self.month, self.year) {
            let days_in_month = TimePoint::month_to_days(self.month, self.year);
            self.day -= days_in_month;
            self.month += 1;

            if self.month > 12 {
                self.year += (self.month - 1) / 12;
                self.month = (self.month - 1) % 12 + 1;
            }
        }
    }

    pub fn before(&mut self, year: i32, month: i32, day: i32, hour: i32, minute: i32, second: i32){
        self.second -= if second >= 0 { second } else { 0 };
        self.minute -= if minute >= 0 { minute } else { 0 };
        self.hour -= if hour >= 0 { hour } else { 0 };
        self.day -= if day >= 0 { day } else { 0 };
        self.month -= if month >= 0 { month } else { 0 };
        self.year -= if year >= 0 { year } else { 0 };

        if self.second < 0 {
            self.minute -= 1;
            self.second += 60;
        }

        if self.minute < 0 {
            self.hour -= 1;
            self.minute += 60;
        }

        if self.hour < 0 {
            self.day -= 1;
            self.hour += 24;
        }

        while self.month < 1 {
            self.year -= 1;
            self.month += 12;
        }

        while self.day < 1 {
            self.month -= 1;
            if self.month < 1 {
                self.year -= 1;
                self.month = 12;
            }
            self.day += TimePoint::month_to_days(self.month, self.year);
        }
    }

    pub fn get_now(&mut self) {
        let now = Local::now();
        let utc = now.offset().fix().local_minus_utc() / 3600;
        self.year = now.year();
        self.month = now.month() as i32;
        self.day = now.day() as i32;
        self.hour = now.hour() as i32;
        self.minute = now.minute() as i32;
        self.second = now.second() as i32;
        self.utc_offset = utc;
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

    fn valid_timepoint(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: i32, utc_offset: i32) -> bool {
        if utc_offset > 13 || utc_offset < -13 {
            return false;
        }

        return TimePoint::valid_date(year, month, day) && TimePoint::valid_time(hour, minute, second);
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
        let tp1 = TimePoint::new(-2023, 3, 15, 12, 0, 0, 1);
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
        let mut origin: TimePoint = TimePoint::new(2020, 1, 1, 23, 0, 0, 8);
        origin.to_utc(9);
        assert_eq!(origin.to_string(), "2020-01-02 00:00:00 UTC+9");
        origin.to_utc(0);
        assert_eq!(origin.to_string(), "2020-01-01 15:00:00 UTC+0");
        origin.to_utc(1);
        assert_eq!(origin.to_string(), "2020-01-01 16:00:00 UTC+1");
        origin.to_utc(-5);
        assert_eq!(origin.to_string(), "2020-01-01 10:00:00 UTC-5");
    }

    #[test]
    fn test_to_utc_a() {
        let mut origin: TimePoint = TimePoint::new(2020, 1, 1, 1, 0, 0, 8);
        origin.to_utc(9);
        assert_eq!(origin.to_string(), "2020-01-01 02:00:00 UTC+9");
        origin.to_utc(0);
        assert_eq!(origin.to_string(), "2019-12-31 17:00:00 UTC+0");
        origin.to_utc(1);
        assert_eq!(origin.to_string(), "2019-12-31 18:00:00 UTC+1");
        origin.to_utc(-5);
        assert_eq!(origin.to_string(), "2019-12-31 12:00:00 UTC-5");
    }

    #[test]
    fn test_after() {
        let mut origin: TimePoint = TimePoint::new(2025, 10, 1, 10, 32, 17, 8);
        origin.after(0, 5, 0, 0, 0, 0); // test month overflow
        assert_eq!(origin.to_string(), "2026-03-01 10:32:17 UTC+8");
        origin.after(0, 0, 0, 15, 0, 0); // test hour overflow
        assert_eq!(origin.to_string(), "2026-03-02 01:32:17 UTC+8");
        origin.after(0, 0, 40, 0, 0, 0); // test day overflow
        assert_eq!(origin.to_string(), "2026-04-11 01:32:17 UTC+8");
        origin.after(0, 0, 0, 0, 30, 0); // test minute overflow
        assert_eq!(origin.to_string(), "2026-04-11 02:02:17 UTC+8");
        origin.after(0, 0, 0, 0, 0, 50); // test second overflow
        assert_eq!(origin.to_string(), "2026-04-11 02:03:07 UTC+8");
        origin.after(0, 0, 0, 0, 0, 0); // test nothing changed
        assert_eq!(origin.to_string(), "2026-04-11 02:03:07 UTC+8");
        origin.after(-1, -1, -1, -1, -1, -1); // test error exception
        assert_eq!(origin.to_string(), "2026-04-11 02:03:07 UTC+8");
    }

    #[test]
    fn test_before() {
        let mut origin: TimePoint = TimePoint::new(2025, 10, 1, 10, 32, 17, 8);
        origin.before(0, 12, 0, 0, 0, 0); // test month overflow
        assert_eq!(origin.to_string(), "2024-10-01 10:32:17 UTC+8");
        origin.before(0, 0, 2, 0, 0, 0); // test day overflow
        assert_eq!(origin.to_string(), "2024-09-29 10:32:17 UTC+8");
        origin.before(0, 0, 0, 12, 0, 0); // test hour overflow
        assert_eq!(origin.to_string(), "2024-09-28 22:32:17 UTC+8");
        origin.before(0, 0, 0, 0, 40, 0); // test minute overflow
        assert_eq!(origin.to_string(), "2024-09-28 21:52:17 UTC+8");
        origin.before(0, 0, 0, 0, 0, 20); // test second overflow
        assert_eq!(origin.to_string(), "2024-09-28 21:51:57 UTC+8");
    }

    #[test] // this test only valid when your system time zone at UTC+8
    fn test_get_now_utc() {
        let mut test = TimePoint::new(0, 0, 0, 0, 0, 0, 0);
        test.get_now();
        assert_eq!(test.utc_offset, 8);
    }
}
