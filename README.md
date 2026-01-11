# cx-studio-rs

Rust version of cxalio studio toolkits, a multimedia-focused library for handling time-related functionality.

## Overview

cx-studio-rs provides a comprehensive set of tools for working with time in multimedia applications. Built with precision and performance in mind, it offers robust implementations for time representation, conversion, and manipulation.

## Core Features

### Time

- Represents time points or durations in milliseconds
- Supports negative values for calculation convenience
- Provides methods for creation from various units (milliseconds, seconds, minutes, hours)
- Supports arithmetic operations (addition, subtraction, multiplication, division)
- Offers normalization within a 24-hour period

### Timebase

- Handles frame rate information
- Automatically calculates actual frame rate and dropframe status
- Supports both integer and fractional frame rates (e.g., 24.0, 23.976)
- Provides frame-to-time and time-to-frame conversion

### Timecode

- Represents SMPTE timecode information
- Stores timebase information internally
- Converts between time values and formatted timecode strings
- Supports both standard and dropframe formats
- Automatically handles frame padding based on frame rate

### TimeRange

- Represents time intervals
- Provides methods for overlap detection
- Calculates duration and normalized time points
- Supports creation from tuples and conversion back to tuples

### Timestamp

- Represents timestamps with millisecond precision
- Converts between time values and formatted timestamp strings
- Supports various separators (colon, semicolon, space, dash)
- Handles both positive and normalized negative times

## Basic Usage

```rust
use cx_studio::core::{Time, Timebase, Timecode};

// Create time objects
let time = Time::from_seconds(1.5);
let duration = Time::from_minutes(30.0);

// Create a timebase
let timebase_24fps = Timebase::new(24.0);
let timebase_dropframe = Timebase::new(23.976);

// Create a timecode from time
let timecode = Timecode::from_time(Time::from_seconds(1234.5), timebase_24fps);
println!("Timecode: {}", timecode.to_string()); // "00:20:34:12"
```

## Documentation

Each component includes detailed documentation with usage examples. For more information, refer to the in-code documentation:

- [Time Documentation](cx_studio/src/core/time/doc_Time.md)
- [Timebase Documentation](cx_studio/src/core/time/doc_Timebase.md)
- [Timecode Documentation](cx_studio/src/core/time/doc_Timecode.md)
- [TimeRange Documentation](cx_studio/src/core/time/doc_TimeRange.md)
- [Timestamp Documentation](cx_studio/src/core/time/doc_Timestamp.md)

## Testing

The library includes comprehensive tests for all components:

```bash
cargo test
```

## Installation

Add `cx_studio` to your `Cargo.toml` dependencies:

```toml
[dependencies]
cx_studio = "0.1.0"
```

## Building

To build the project:

```bash
cargo build
```

To build in release mode:

```bash
cargo build --release
```

## Project Structure

```
cx-studio-rs/
├── cx_studio/
│   ├── src/
│   │   ├── core/
│   │   │   ├── time/          # Time-related functionality
│   │   │   │   ├── cx_time.rs
│   │   │   │   ├── cx_timebase.rs
│   │   │   │   ├── cx_timecode.rs
│   │   │   │   ├── cx_timerange.rs
│   │   │   │   ├── cx_timestamp.rs
│   │   │   │   └── mod.rs
│   │   │   └── mod.rs
│   │   └── lib.rs
│   ├── tests/                 # Unit tests
│   └── Cargo.toml
├── Cargo.lock
├── Cargo.toml
├── LICENSE
└── README.md
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Roadmap

- Add more multimedia processing modules
- Enhance timecode functionality
- Add support for more time formats
- Improve performance and documentation

## License

This project is licensed under the GPL-3.0-or-later license. See the [LICENSE](LICENSE) file for details.

## Contact

For questions or support, please open an issue on the [GitHub repository](https://github.com/LambdaXIII/cx-studio-rs).
