use std::cmp::{max, min};

use super::cx_time::Time;

pub struct TimeRange {
    pub start: Time,
    pub end: Time,
}

impl Default for TimeRange {
    fn default() -> Self {
        Self {
            start: Time::zero(),
            end: Time::zero(),
        }
    }
}

impl TimeRange {
    pub fn new(start: Time, end: Time) -> Self {
        Self {
            start: min(start, end),
            end: max(start, end),
        }
    }

    pub fn duration(&self) -> Time {
        self.end - self.start
    }

    pub fn is_overlapped_with(&self, other: &Self) -> bool {
        self.start < other.end && other.start < self.end
    }
}

impl From<(Time, Time)> for TimeRange {
    fn from(value: (Time, Time)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl Into<(Time, Time)> for TimeRange {
    fn into(self) -> (Time, Time) {
        (self.start, self.end)
    }
}
