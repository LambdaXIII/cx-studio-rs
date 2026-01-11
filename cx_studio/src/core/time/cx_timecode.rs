use super::cx_time::Time;
use super::cx_timebase::Timebase;
use std::fmt;

#[doc=include_str!("doc_Timecode.md")]
#[derive(Debug, Clone, PartialEq)]
pub struct Timecode {
    hour_code: u8,
    minute_code: u8,
    second_code: u8,
    frame_code: u16,
    timebase: Timebase,
}

impl Timecode {
    /// Directly constructs a Timecode object with given hour, minute, second, frame and timebase.
    ///
    /// ```rust
    /// use cx_studio::core::{Timebase,Timecode};
    /// let timebase = Timebase::new(24.0);
    /// let timecode = Timecode::new(0, 0, 5, 2, timebase); // 00:00:05:02
    /// ```
    pub fn new(
        hour_code: u8,
        minute_code: u8,
        second_code: u8,
        frame_code: u16,
        timebase: Timebase,
    ) -> Self {
        Self {
            hour_code,
            minute_code,
            second_code,
            frame_code,
            timebase,
        }
    }

    /// Converts a Time object to a Timecode object with given timebase.
    ///
    /// ```rust
    /// use cx_studio::core::{Timebase,Timecode,Time};
    /// let timebase = Timebase::new(24.0);
    /// let time = Time::from_seconds(1.5555);
    /// let timecode = Timecode::from_time(time, timebase); // 00:00:01:12
    /// ```
    pub fn from_time(time: Time, timebase: Timebase) -> Self {
        let rate = timebase.framerate();
        let frames: u64 = (time.normalized().to_seconds() * rate as f64).round() as u64;
        let ff = (frames % (rate as u64)) as u16;
        let seconds = (frames as f64) / (rate as f64);
        let ss = seconds % 60.0;
        let minutes = seconds / 60.0;
        let mm = minutes % 60.0;
        let hours = minutes / 60.0;
        let hh = hours % 24.0;

        Self {
            hour_code: hh as u8,
            minute_code: mm as u8,
            second_code: ss as u8,
            frame_code: ff as u16,
            timebase,
        }
    }

    /// Calculate the time of the timecode.
    ///
    /// Returns a [Time] object.
    ///
    /// ```rust
    /// use cx_studio::core::{Timebase,Timecode};
    /// let timebase = Timebase::new(24.0);
    /// let timecode = Timecode::new(0, 0, 5, 2, timebase); // 00:00:05:02
    /// let time = timecode.to_time(); // 1.5555
    /// ```
    pub fn to_time(&self) -> Time {
        let rate = self.timebase.framerate();
        let frames = (self.hour_code as u64 * 60 * 60 * rate as u64)
            + (self.minute_code as u64 * 60 * rate as u64)
            + (self.second_code as u64 * rate as u64)
            + (self.frame_code as u64);
        Time::from_seconds((frames as f64) / (rate as f64))
    }

    const PATTERN: &'static str = r"^(\d{2})[^\d](\d{2})[^\d](\d{2})[^\d](\d{2,})$";

    /// Converts a timecode string to a Timecode object with given timebase.
    ///
    /// Returns None if the string is invalid.
    ///
    /// ```rust
    /// use cx_studio::core::{Timebase, Timecode};
    /// let timebase = Timebase::new(24.0);
    /// let timecode = Timecode::from_string("00:00:05:02", timebase); // Some(00:00:05:02)
    /// ```
    pub fn from_string(code: &str, timebase: Timebase) -> Option<Self> {
        let pat = regex::Regex::new(Self::PATTERN).ok()?;
        let caps = pat.captures(code)?;
        let hour_code = caps.get(1)?.as_str().parse().ok()?;
        let minute_code = caps.get(2)?.as_str().parse().ok()?;
        let second_code = caps.get(3)?.as_str().parse().ok()?;
        let frame_code = caps.get(4)?.as_str().parse().ok()?;
        Some(Self::new(
            hour_code,
            minute_code,
            second_code,
            frame_code,
            timebase,
        ))
    }

    /// Gets the stored timebase.
    pub fn get_timebase(&self) -> Timebase {
        self.timebase
    }
}

impl fmt::Display for Timecode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sep = if self.timebase.dropframe() { ";" } else { ":" };
        // 计算帧部分的位数：优先取frame_code的实际位数，其次按framerate的位数，最少保证2位
        let framerate = self.timebase.framerate() as u32;
        let frame_digits = if self.frame_code > 0 {
            self.frame_code.to_string().len() as usize
        } else {
            1
        }
        .max(framerate.to_string().len())
        .max(2);
        write!(
            f,
            "{:02}:{:02}:{:02}{}{:0width$}",
            self.hour_code,
            self.minute_code,
            self.second_code,
            sep,
            self.frame_code,
            width = frame_digits
        )
    }
}

impl Default for Timecode {
    /// Default timecode is 00:00:00:00 (24fps).
    fn default() -> Self {
        Self::new(0, 0, 0, 0, Timebase::default())
    }
}
