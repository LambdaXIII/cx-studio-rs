Timecode represents time code information.

Timecode doesn't store the time value.
It stores only the numbers to show in a time code string.
Also, a timecode depends on a timebase,
so it stores the timebase completely inside.

Timecode can be constructed from a time value and a timebase.
Then it can be formatted to a time code string properly.

For example:

```rust
let timebase = Timebase::new(24.0);
let timecode = Timecode::from_time(Time::from_seconds(1.5), &timebase);
let timecode_str = timecode.to_string(); // "00:00:01:12"
```

Note:

- The last section -- witch represents frames --
  would be formated depends on the framerate of timebase,
  but always has 2 digits at least.

- If the timebase is dropframe,
  the last separator would be a colon.

Examples:

| Framerate | Dropframe | Timecode     |
| --------- | --------- | ------------ |
| 24.0      | No        | 00:00:05:02  |
| 23.976    | Yes       | 00:00:05;02  |
| 120.0     | No        | 00:00:01:078 |

---

See also: [Time], [Timebase] and [Timestamp](super::Timestamp).
