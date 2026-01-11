Time struct represents 'time'.
It can be either a time point or a duration.

This struct is supposed to be used in multimedia context,
so it's time unit is **milliseconds**.

For conveniency of calculation,
Time can be a negative value.
The value stored as a i64 integer,
so it also shouldn't be used to represent a very long time.
_(But I think i64 is long enough thou ~)_

### Construction

Time struct provides multiple construction methods,
for creating time object from different numerical values.

For example:

```rust
use cx_studio::core::Time;
let time0 = Time::zero();
let time1 = Time::from_seconds(1.5);
let time2 = Time::from_minutes(2.5);
let time3 = Time::from_hours(0.5);
```

Well, Time actually stores a milliseond value,
so larger accuracy will be ignored.

```rust
use cx_studio::core::Time;
let time1 = Time::from_milliseconds(1234);
let time2 = Time::from_seconds(1.2341);
// time1 and time2 are equal.
```

For more details,
see also [Time::from_seconds], [Time::from_minutes], [Time::from_seconds].

### Extraction

Time struct provides multiple methods,
for extracting numerical values from time object.

For example:

```rust
use cx_studio::core::Time;
let time = Time::from_seconds(1.5);
let ms = time.to_milliseconds(); // 1500
let hours = time.to_hours();
```

See also [Time::to_milliseconds], [Time::to_seconds], [Time::to_minutes], [Time::to_hours].

### Operation

Time struct provides multiple methods,
for performing arithmetic operations on time object.

For example:

```rust
use cx_studio::core::Time;
let time1 = Time::from_seconds(1.5);
let time2 = Time::from_minutes(2.5);
let time3 = time1 + time2; // 9000ms
let time4 = time3 * 2.0; // 18000ms
```

### Note

- Time struct's operation results are also time objects,
  not numerical values.
- Time object is immutable.
