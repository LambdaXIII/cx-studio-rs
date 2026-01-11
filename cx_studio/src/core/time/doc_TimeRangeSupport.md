TimeRangeSupport makes a struct being able to handle time points.

Unlike [TimeRange],
the default implementation of TimeRangeSupport aims to store the start time and the duration.
The end time would be calculated on the fly.

Well, the TimeRangeSupport trait still provides a function to export a TimeRange.

TimeRangeSupport is designed to be mutable,
for mutable functions, see [TimeRangeMutableSupport].
