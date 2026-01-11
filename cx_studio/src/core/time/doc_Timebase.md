Timebase struct saves timebase information.

It constructs from a simple frame-per-second value,
calculate actual framerate and determine whether frames are dropped automatically.

```rust
let timebase = Timebase::new(24.0); // 24 frames per seconds without drop frames.
let timebase2 = Timebase::new(23.976); // 24 fps but drops frames.
```

**IMPORTANT**

- Negative frame rate is forbidden.
- Frame rate less than 1 is not supported.
