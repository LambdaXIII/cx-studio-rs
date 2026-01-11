use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[doc = include_str!("doc_Time.md")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
    millisecond: i64,
}

impl Time {
    /// Directly constructs a Time object with 0 milliseconds.
    /// [default()](Self::default) will call this method.
    pub fn zero() -> Self {
        Self::from_milliseconds(0)
    }

    /// Directly constructs a Time object with given milliseconds.
    ///
    /// *Actually, Time internally stores this millisecond value.*
    ///
    /// ```rust
    /// let time = Time::from_milliseconds(1500); // will generate a time length of 1500 milliseconds   
    /// let time2 = Time::from_milliseconds(-1500); // this is also illegal
    /// ```
    pub fn from_milliseconds(milliseconds: i64) -> Self {
        Self {
            millisecond: milliseconds,
        }
    }

    /// Directly constructs a Time object with given seconds.
    ///
    /// *Although you can input a floating-point number,
    /// Time class only stores millisecond accuracy.*
    /// ```rust
    /// let time = Time::from_seconds(1.5); // represents a time object of 1500 milliseconds
    /// let time2 = Time::from_seconds(1.500001); // actually equals time
    /// ```
    pub fn from_seconds(seconds: f64) -> Self {
        Self {
            millisecond: (seconds * 1000.0).round() as i64,
        }
    }

    /// Directly constructs a Time object with given minutes.
    ///
    /// *See also [Self::from_seconds].*
    pub fn from_minutes(minutes: f64) -> Self {
        Self {
            millisecond: (minutes * 60.0 * 1000.0).round() as i64,
        }
    }

    /// Directly constructs a Time object with given hours.
    ///
    /// *See also [Self::from_seconds].*
    pub fn from_hours(hours: f64) -> Self {
        Self {
            millisecond: (hours * 60.0 * 60.0 * 1000.0).round() as i64,
        }
    }

    /// Converts the time object to milliseconds.
    pub fn to_milliseconds(&self) -> i64 {
        self.millisecond
    }

    /// Converts the time object to seconds.
    ///
    /// *Note that the precision of Time class is only millisecond,
    /// so the converted seconds may be different from the input seconds.*
    /// ```rust
    /// let time = Time::from_seconds(1.5555);
    /// let seconds = time.to_seconds(); // 1.556
    /// ```
    pub fn to_seconds(&self) -> f64 {
        self.millisecond as f64 / 1000.0
    }

    /// Converts the time object to minutes.
    ///
    /// *See also [Self::to_seconds].*
    pub fn to_minutes(&self) -> f64 {
        self.millisecond as f64 / 60.0 / 1000.0
    }

    /// Converts the time object to hours.
    ///
    /// *See also [Self::to_seconds].*
    pub fn to_hours(&self) -> f64 {
        self.millisecond as f64 / 60.0 / 60.0 / 1000.0
    }

    /// Normalizes the time object to a duration within one day.    
    /// Returns a new time object.
    ///
    /// ```rust
    /// let time = Time::from_hours(25); // stores a time with 25 hours
    /// let normalized_time = time.normalized(); // normalized as 1 hour
    /// let one_hour = Time::from_hours(1); // normalized_time equals one_hour
    /// ```
    ///
    /// - If the time is larger than one day,
    ///   the normalized time will represent the time in next day.
    ///
    /// - If the time is smaller than zero,
    ///   the normalized time will represent the time in previous day.
    ///
    /// - If the time is still out of the range of one day,
    ///   the normalization will continue to try.
    ///
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
