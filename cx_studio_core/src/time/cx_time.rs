use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
    millisecond: i64,
}

/**
 * Time 是一个用于表示时间的结构体。既可以用于表示时间点，也可以表示一段时间。
 * 内部保存的是毫秒数字，也就是说精确到毫秒。
 *
 * 支持负数时间。
 */
impl Time {
    pub fn from_milliseconds(milliseconds: i64) -> Self {
        Self {
            millisecond: milliseconds,
        }
    }

    pub fn from_seconds(seconds: f64) -> Self {
        Self {
            millisecond: (seconds * 1000.0).round() as i64,
        }
    }

    pub fn from_minutes(minutes: f64) -> Self {
        Self {
            millisecond: (minutes * 60.0 * 1000.0).round() as i64,
        }
    }

    pub fn from_hours(hours: f64) -> Self {
        Self {
            millisecond: (hours * 60.0 * 60.0 * 1000.0).round() as i64,
        }
    }

    pub fn to_milliseconds(&self) -> i64 {
        self.millisecond
    }

    pub fn to_seconds(&self) -> f64 {
        self.millisecond as f64 / 1000.0
    }

    pub fn to_minutes(&self) -> f64 {
        self.millisecond as f64 / 60.0 / 1000.0
    }

    pub fn to_hours(&self) -> f64 {
        self.millisecond as f64 / 60.0 / 60.0 / 1000.0
    }

    /**
     * 将时间锁定在一天之内。
     *
     * 超出部分将重新计算。
     * 负数部分则反向计算。
     */
    pub fn normalized(&self) -> Self {
        const DAY_MILLISECONDS: i64 = 24 * 60 * 60 * 1000;
        let normalized_millisecond = self.millisecond.rem_euclid(DAY_MILLISECONDS);
        Self::from_milliseconds(normalized_millisecond)
    }
}

impl Add for Time {
    type Output = Time;

    fn add(self, other: Time) -> Time {
        Time {
            millisecond: self.millisecond + other.millisecond,
        }
    }
}

impl Sub for Time {
    type Output = Time;

    fn sub(self, other: Time) -> Time {
        Time {
            millisecond: self.millisecond - other.millisecond,
        }
    }
}

impl AddAssign for Time {
    fn add_assign(&mut self, other: Time) {
        self.millisecond += other.millisecond;
    }
}

impl SubAssign for Time {
    fn sub_assign(&mut self, other: Time) {
        self.millisecond -= other.millisecond;
    }
}

impl Mul<f64> for Time {
    type Output = Time;

    fn mul(self, other: f64) -> Time {
        Time {
            millisecond: (self.millisecond as f64 * other).round() as i64,
        }
    }
}

impl Div<f64> for Time {
    type Output = Time;

    fn div(self, other: f64) -> Time {
        Time {
            millisecond: (self.millisecond as f64 / other).round() as i64,
        }
    }
}

impl MulAssign<f64> for Time {
    fn mul_assign(&mut self, other: f64) {
        self.millisecond = (self.millisecond as f64 * other).round() as i64;
    }
}

impl DivAssign<f64> for Time {
    fn div_assign(&mut self, other: f64) {
        self.millisecond = (self.millisecond as f64 / other).round() as i64;
    }
}

impl From<i64> for Time {
    fn from(milliseconds: i64) -> Self {
        Self::from_milliseconds(milliseconds)
    }
}

impl Into<f64> for Time {
    fn into(self) -> f64 {
        self.to_seconds()
    }
}

impl From<f64> for Time {
    fn from(seconds: f64) -> Self {
        Self::from_seconds(seconds)
    }
}

impl Into<i64> for Time {
    fn into(self) -> i64 {
        self.to_milliseconds()
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::from_milliseconds(0)
    }
}
