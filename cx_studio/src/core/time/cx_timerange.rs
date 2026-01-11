use std::cmp::{max, min};

use super::cx_time::Time;

#[doc = include_str!("doc_TimeRange.md")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeRange {
    pub start: Time,
    pub end: Time,
}

impl Default for TimeRange {
    /// Default time range is (0, 0).
    fn default() -> Self {
        Self {
            start: Time::zero(),
            end: Time::zero(),
        }
    }
}

impl TimeRange {
    /// Creates a new time range with the given start and end times.
    ///
    /// *Note that the start time will be the minimum of the two given times,
    /// and the end time will be the maximum of the two given times.*
    pub fn new(start: Time, end: Time) -> Self {
        Self {
            start: min(start, end),
            end: max(start, end),
        }
    }

    /// Gets the duration of the time range.
    pub fn duration(&self) -> Time {
        self.end - self.start
    }

    /// Checks if the time range is overlapped with the other time range.
    pub fn is_overlapped_with(&self, other: &Self) -> bool {
        self.start < other.end && other.start < self.end
    }

    /// Checks if the time range contains the given time.
    pub fn contains(&self, time: Time) -> bool {
        self.start <= time && time < self.end
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
