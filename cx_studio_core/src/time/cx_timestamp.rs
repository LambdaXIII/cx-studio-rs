use super::cx_time::Time;
use regex::Regex;
use std::fmt;
use std::str::FromStr;

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
        let ms = time.to_milliseconds();
        Self {
            hour_code: (ms / 1000 / 60 / 60) as u8,
            minute_code: (ms / 1000 / 60 % 60) as u8,
            second_code: (ms / 1000 % 60) as u8,
            millisecond_code: (ms % 1000) as u16,
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

impl FromStr for Timestamp {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(\d{2}):(\d{2}):(\d{2})\.(\d{3})$").map_err(|e| e.to_string())?;

        let caps = re
            .captures(s)
            .ok_or("Invalid timestamp format. Expected: HH:MM:SS.mmm")?;

        let hour: u8 = caps
            .get(1)
            .ok_or("Missing hour")?
            .as_str()
            .parse()
            .map_err(|_| "Invalid hour")?;
        let minute: u8 = caps
            .get(2)
            .ok_or("Missing minute")?
            .as_str()
            .parse()
            .map_err(|_| "Invalid minute")?;
        let second: u8 = caps
            .get(3)
            .ok_or("Missing second")?
            .as_str()
            .parse()
            .map_err(|_| "Invalid second")?;
        let millisecond: u16 = caps
            .get(4)
            .ok_or("Missing millisecond")?
            .as_str()
            .parse()
            .map_err(|_| "Invalid millisecond")?;

        if hour > 23 {
            return Err("Hour must be 0-23".to_string());
        }
        if minute > 59 {
            return Err("Minute must be 0-59".to_string());
        }
        if second > 59 {
            return Err("Second must be 0-59".to_string());
        }

        Ok(Self {
            hour_code: hour,
            minute_code: minute,
            second_code: second,
            millisecond_code: millisecond,
        })
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}
