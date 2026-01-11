use super::cx_time::Time;
use super::cx_timerange::TimeRange;

pub trait TimeRangeSupport {
    fn start_time(&self) -> Time;
    fn duration_time(&self) -> Time;
    fn end_time(&self) -> Time {
        self.start_time() + self.duration_time()
    }
    fn is_overlapped_with(&self, other: &Self) -> bool {
        self.start_time() < other.end_time() && other.start_time() < self.end_time()
    }

    fn time_range(&self) -> TimeRange {
        TimeRange::new(self.start_time(), self.end_time())
    }
}

pub trait TimeRangeMutableSupport: TimeRangeSupport {
    fn set_start_time(&mut self, start_time: Time);
    fn set_duration_time(&mut self, duration_time: Time);
    fn set_end_time(&mut self, end_time: Time) {
        if end_time > self.start_time() {
            self.set_duration_time(end_time - self.start_time())
        } else {
            self.set_duration_time(Time::zero())
        }
    }
    fn set_time_range(&mut self, time_range: TimeRange) {
        self.set_start_time(time_range.start);
        self.set_end_time(time_range.end);
    }
}
