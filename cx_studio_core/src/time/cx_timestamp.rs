use super::cx_time::Time;
use regex::Regex;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timestamp {
    hour_code: u8,
    minute_code: u8,
    second_code: u8,
    millisecond_code: u16,
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02}.{:03}",
            self.hour_code, self.minute_code, self.second_code, self.millisecond_code
        )
    }
}

impl Timestamp {
    pub fn new(hour_code: u8, minute_code: u8, second_code: u8, millisecond_code: u16) -> Self {
        Self {
            hour_code,
            minute_code,
            second_code,
            millisecond_code,
        }
    }

    pub fn from_time(time: Time) -> Self {
        let ms = time.normalized().to_milliseconds();
        let mmm = ms % 1000;
        let seconds = ms / 1000;
        let ss = seconds % 60;
        let minutes = seconds / 60;
        let mm = minutes % 60;
        let hours = minutes / 60;
        let hh = hours % 24;
        Self {
            hour_code: hh as u8,
            minute_code: mm as u8,
            second_code: ss as u8,
            millisecond_code: mmm as u16,
        }
    }

    pub fn to_time(&self) -> Time {
        Time::from_milliseconds(
            (self.hour_code as i64 * 60 * 60 * 1000)
                + (self.minute_code as i64 * 60 * 1000)
                + (self.second_code as i64 * 1000)
                + (self.millisecond_code as i64),
        )
    }

    const PATTERN: &'static str = r"^(\d{2})[^\d](\d{2})[^\d](\d{2})[^\d](\d{3})$";
    pub fn from_string(s: &str) -> Option<Self> {
        let re = Regex::new(Self::PATTERN).ok()?;
        let caps = re.captures(s)?;
        let hour_code = caps.get(1)?.as_str().parse().ok()?;
        let minute_code = caps.get(2)?.as_str().parse().ok()?;
        let second_code = caps.get(3)?.as_str().parse().ok()?;
        let millisecond_code = caps.get(4)?.as_str().parse().ok()?;
        Some(Self::new(
            hour_code,
            minute_code,
            second_code,
            millisecond_code,
        ))
    }
}

impl From<Time> for Timestamp {
    fn from(time: Time) -> Self {
        Self::from_time(time)
    }
}

impl Into<Time> for Timestamp {
    fn into(self) -> Time {
        self.to_time()
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}
