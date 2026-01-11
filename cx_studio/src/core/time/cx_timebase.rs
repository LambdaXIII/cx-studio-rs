use super::cx_time::Time;

#[doc=include_str!("doc_Timebase.md")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Timebase {
    fps: f32,
    framerate: u16,
    dropframe: bool,
}

impl Timebase {
    /// Creates a new Timebase object with the given frame rate.
    ///
    /// If the given fps is an integer,
    /// framerate would be the same as fps,
    /// and dropframe would be `false`.
    ///
    /// If the given fps is not an integer,
    /// framerate would be rounded to the nearest integer,
    /// and dropframe would be `true`.
    ///
    /// ```rust
    /// let tb_24 = Timebase::new(24); // Standard 24 frame rate
    /// let tb_23976 = Timebase::new(23.976); // Dropframe 24 frame rate
    /// let strange_timebase = Timebase::new(789.830) // Dropframe 790 fps, strange but acceptable
    /// ```
    ///
    /// Note that the frame rate would always be larger than or equal to the given fps.
    /// Because the frames can only be 'dropped'.
    pub fn new(fps: f32) -> Self {
        assert!(fps >= 1.0, "fps must be greater than or equal to 1");
        let rounded_fps = fps.ceil() as u16;
        Self {
            fps,
            framerate: rounded_fps,
            dropframe: (rounded_fps as f32) == fps,
        }
    }

    /// Returns the fps of the timebase.
    /// This is the same value from the constructor -- [Self::new].
    pub fn fps(&self) -> f32 {
        self.fps
    }

    /// Returns the actual frame rate which will be used for calculations.
    ///
    /// It will always be an integer value,
    /// even if it is drop frame.
    pub fn framerate(&self) -> u16 {
        self.framerate
    }

    /// Returns whether the timebase is drop frame.
    pub fn dropframe(&self) -> bool {
        self.dropframe
    }

    /// Returns the duration of one frame in milliseconds.
    ///
    /// It can be used to construct a [Time] object.
    pub fn milliseconds_per_frame(&self) -> u32 {
        (1000.0 / self.fps) as u32
    }

    /// Returns the number of frames in the given [Time] duration.
    pub fn frames_from_time(&self, time: &Time) -> i64 {
        let seconds = time.to_seconds();
        (seconds * self.framerate as f64) as i64
    }

    /// Returns the [Time] duration of the given number of frames.
    ///
    /// Although this functions uses seconds to calculate [Time],
    /// it still would only be accurate to the precision of **milliseconds**.
    pub fn time_from_frames(&self, frames: i64) -> Time {
        Time::from_seconds((frames as f64) / self.framerate as f64)
    }
}

impl Default for Timebase {
    /// Constructs a default value of Timebase with 24 fps (no frame drops).
    fn default() -> Self {
        Self::new(24.0)
    }
}
