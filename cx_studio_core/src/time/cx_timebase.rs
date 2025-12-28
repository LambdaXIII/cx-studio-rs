use super::cx_time::Time;

/**
 * 自动处理时基信息的结构体
 *
 * 使用理想的帧速率初始化，将自动识别运算时使用的帧速率和是否丢帧。
 */

#[derive(Debug, Clone, Copy)]
pub struct Timebase {
    fps: f32,
    framerate: u16,
    dropframe: bool,
}

impl Timebase {
    pub fn new(fps: f32) -> Self {
        let rounded_fps = fps.round() as u16;
        Self {
            fps,
            framerate: rounded_fps,
            dropframe: (rounded_fps as f32) == fps,
        }
    }

    pub fn fps(&self) -> f32 {
        self.fps
    }

    pub fn framerate(&self) -> u16 {
        self.framerate
    }

    pub fn dropframe(&self) -> bool {
        self.dropframe
    }

    pub fn milliseconds_per_frame(&self) -> u32 {
        (1000.0 / self.fps) as u32
    }

    pub fn frames_from_time(&self, time: &Time) -> i64 {
        let seconds = time.to_seconds();
        (seconds * self.framerate as f64) as i64
    }

    pub fn time_from_frames(&self, frames: i64) -> Time {
        Time::from_seconds((frames as f64) / self.framerate as f64)
    }
}

impl Default for Timebase {
    fn default() -> Self {
        Self::new(24.0)
    }
}
