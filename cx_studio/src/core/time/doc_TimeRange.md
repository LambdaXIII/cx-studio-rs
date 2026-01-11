TimeRange stores a pair of time points,
which represents the start and end time of a time range.

TimeRange implemented From and Into traits,
which allows you to convert between TimeRange and `(Time, Time)`.

It is a very simple struct,
designed for convenience of time range operations.
If you need more advanced time range operations,
check out [TimeRangeSupport](super::TimeRangeSupport) and [TimeRangeMutableSupport](super::TimeRangeMutableSupport) traits.
