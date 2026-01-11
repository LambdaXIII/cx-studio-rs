use super::cx_time::Time;
use super::cx_timerange::TimeRange;

#[doc = include_str!("doc_TimeRangeSupport.md")]
pub trait TimeRangeSupport {
    /// Gets the start time of the time range.
    fn start_time(&self) -> Time;

    /// Gets the duration time of the time range.
    fn duration_time(&self) -> Time;

    /// Gets the end time of the time range.
    fn end_time(&self) -> Time {
        self.start_time() + self.duration_time()
    }

    /// Checks if the time range is overlapped with another time range.
    fn is_overlapped_with(&self, other: &Self) -> bool {
        self.start_time() < other.end_time() && other.start_time() < self.end_time()
    }

    /// Gets the time range.
    fn time_range(&self) -> TimeRange {
        TimeRange::new(self.start_time(), self.end_time())
    }
}

#[doc = include_str!("doc_TimeRangeMutableSupport.md")]
pub trait TimeRangeMutableSupport: TimeRangeSupport {
    /// Sets the start time of the time range.
    ///
    /// Setting the start time may change the end time of the time range
    /// without property implementation.
    /// In this case, this functions behaves like
    /// shifting the whole object on the timeline.
    fn set_start_time(&mut self, start_time: Time);

    /// Sets the duration time of the time range.
    fn set_duration_time(&mut self, duration_time: Time);

    /// Sets the end time of the time range.    
    ///
    /// If the end time is less than the start time,
    /// the duration time would be set to zero.
    fn set_end_time(&mut self, end_time: Time) {
        if end_time > self.start_time() {
            self.set_duration_time(end_time - self.start_time())
        } else {
            self.set_duration_time(Time::zero())
        }
    }

    /// Sets the time range.
    fn set_time_range(&mut self, time_range: TimeRange) {
        self.set_start_time(time_range.start);
        self.set_end_time(time_range.end);
    }
}
