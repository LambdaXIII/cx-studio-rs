use super::cx_time::Time;
use super::cx_timebase::Timebase;

pub struct Timecode {
    hour_code: u8,
    minute_code: u8,
    second_code: u8,
    frame_code: u16,
    timebase: Timebase,
}

impl Timecode {
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

    pub fn to_time(&self) -> Time {
        let rate = self.timebase.framerate();
        let frames = (self.hour_code as u64 * 60 * 60 * rate as u64)
            + (self.minute_code as u64 * 60 * rate as u64)
            + (self.second_code as u64 * rate as u64)
            + (self.frame_code as u64);
        Time::from_seconds((frames as f64) / (rate as f64))
    }

    const PATTERN: &'static str = r"^(\d{2})[^\d](\d{2})[^\d](\d{2})[^\d](\d{2,})$";
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
}

impl Default for Timecode {
    fn default() -> Self {
        Self::new(0, 0, 0, 0, Timebase::default())
    }
}
