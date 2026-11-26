// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::parse_failure::ParseFailure;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;
use core::fmt::Display;
use core::fmt::Formatter;

/// A civil date, `YYYY-MM-DD`, with the day and month arithmetic the rules
/// read: a line's age in days, a release within so many months; and the
/// day so many days after 1970-01-01, the clock's own count.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    year: i64,
    month: i64,
    day: i64,
}

impl Date {
    fn from_day_number(number: i64) -> Self {
        let shifted = number + 719_468;
        let era = shifted.div_euclid(146_097);
        let day_of_era = shifted - era * 146_097;
        let year_of_era =
            (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
        let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
        let month_index = (5 * day_of_year + 2) / 153;
        let month = if month_index < 10 {
            month_index + 3
        } else {
            month_index - 9
        };
        Self {
            year: year_of_era + era * 400 + i64::from(month <= 2),
            month,
            day: day_of_year - (153 * month_index + 2) / 5 + 1,
        }
    }

    pub fn from_unix_days(days: i64) -> Self {
        Self::from_day_number(days)
    }

    pub fn parse(text: &str) -> Result<Self, ParseFailure> {
        let failure = || ParseFailure {
            text: String::from(text),
            expected: "a date, YYYY-MM-DD",
        };
        let parts = text.split('-').collect::<Vec<&str>>();
        let [year, month, day] = parts.as_slice() else {
            return Err(failure());
        };
        let numbers = [(year, 4), (month, 2), (day, 2)]
            .iter()
            .map(|(part, width)| Self::number(part, *width))
            .collect::<Option<Vec<i64>>>()
            .ok_or_else(failure)?;
        let date = Self {
            year: numbers[0],
            month: numbers[1],
            day: numbers[2],
        };
        let valid = (1..=12).contains(&date.month)
            && (1..=Self::days_in_month(date.year, date.month)).contains(&date.day);
        valid.then_some(date).ok_or_else(failure)
    }

    #[must_use]
    pub fn add_days(&self, days: i64) -> Self {
        Self::from_day_number(self.day_number() + days)
    }

    pub fn days_since(&self, earlier: &Self) -> i64 {
        self.day_number() - earlier.day_number()
    }

    #[must_use]
    pub fn months_before(&self, months: i64) -> Self {
        let index = self.year * 12 + self.month - 1 - months;
        let year = index.div_euclid(12);
        let month = index.rem_euclid(12) + 1;
        Self {
            year,
            month,
            day: self.day.min(Self::days_in_month(year, month)),
        }
    }

    fn day_number(&self) -> i64 {
        let year = if self.month <= 2 {
            self.year - 1
        } else {
            self.year
        };
        let era = year.div_euclid(400);
        let year_of_era = year - era * 400;
        let day_of_year = (153 * ((self.month + 9) % 12) + 2) / 5 + self.day - 1;
        let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
        era * 146_097 + day_of_era - 719_468
    }

    fn days_in_month(year: i64, month: i64) -> i64 {
        let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
        match month {
            2 if leap => 29,
            2 => 28,
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        }
    }

    fn number(part: &str, width: usize) -> Option<i64> {
        if part.len() == width && part.bytes().all(|byte| byte.is_ascii_digit()) {
            part.parse().ok()
        } else {
            None
        }
    }
}

impl Display for Date {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{:04}-{:02}-{:02}",
            self.year, self.month, self.day
        )
    }
}
