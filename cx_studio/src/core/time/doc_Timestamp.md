Timestamp represents a single time point,
in a format of `HH:MM:SS.mmm`,
where `HH` is the hour, `MM` is the minute, `SS` is the second,
and `mmm` is the millisecond.

Converting between Timestamp and [Time] doesn't require timebase,
since Timestamp is just a string representation of [Time].
So the conversion is can be done via From and Into traits.
