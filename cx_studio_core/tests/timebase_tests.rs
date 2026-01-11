use cx_studio_core::time::{Time, Timebase};

/// 测试创建整数帧率的Timebase对象
#[test]
fn test_new_integer_fps() {
    let timebase = Timebase::new(24.0);
    assert_eq!(timebase.fps(), 24.0);
    assert_eq!(timebase.framerate(), 24);
    assert_eq!(timebase.dropframe(), true);
}

/// 测试创建分数帧率的Timebase对象
#[test]
fn test_new_fractional_fps() {
    let timebase = Timebase::new(23.976);
    assert_eq!(timebase.fps(), 23.976);
    assert_eq!(timebase.framerate(), 24);
    assert_eq!(timebase.dropframe(), false);
}

/// 测试创建30fps帧率的Timebase对象
#[test]
fn test_new_30_fps() {
    let timebase = Timebase::new(30.0);
    assert_eq!(timebase.fps(), 30.0);
    assert_eq!(timebase.framerate(), 30);
    assert_eq!(timebase.dropframe(), true);
}

/// 测试创建29.97fps帧率的Timebase对象
#[test]
fn test_new_29_97_fps() {
    let timebase = Timebase::new(29.97);
    assert_eq!(timebase.fps(), 29.97);
    assert_eq!(timebase.framerate(), 30);
    assert_eq!(timebase.dropframe(), false);
}

/// 测试创建60fps帧率的Timebase对象
#[test]
fn test_new_60_fps() {
    let timebase = Timebase::new(60.0);
    assert_eq!(timebase.fps(), 60.0);
    assert_eq!(timebase.framerate(), 60);
    assert_eq!(timebase.dropframe(), true);
}

/// 测试创建59.94fps帧率的Timebase对象
#[test]
fn test_new_59_94_fps() {
    let timebase = Timebase::new(59.94);
    assert_eq!(timebase.fps(), 59.94);
    assert_eq!(timebase.framerate(), 60);
    assert_eq!(timebase.dropframe(), false);
}

/// 测试获取Timebase对象的fps值
#[test]
fn test_fps() {
    let timebase = Timebase::new(24.0);
    assert_eq!(timebase.fps(), 24.0);
}

/// 测试获取Timebase对象的帧率值
#[test]
fn test_framerate() {
    let timebase = Timebase::new(24.0);
    assert_eq!(timebase.framerate(), 24);
}

/// 测试整数帧率的dropframe属性
#[test]
fn test_dropframe_integer() {
    let timebase = Timebase::new(24.0);
    assert_eq!(timebase.dropframe(), true);
}

/// 测试分数帧率的dropframe属性
#[test]
fn test_dropframe_fractional() {
    let timebase = Timebase::new(23.976);
    assert_eq!(timebase.dropframe(), false);
}

/// 测试24fps帧率下每帧的毫秒数
#[test]
fn test_milliseconds_per_frame_24fps() {
    let timebase = Timebase::new(24.0);
    assert_eq!(timebase.milliseconds_per_frame(), 41);
}

/// 测试30fps帧率下每帧的毫秒数
#[test]
fn test_milliseconds_per_frame_30fps() {
    let timebase = Timebase::new(30.0);
    assert_eq!(timebase.milliseconds_per_frame(), 33);
}

/// 测试60fps帧率下每帧的毫秒数
#[test]
fn test_milliseconds_per_frame_60fps() {
    let timebase = Timebase::new(60.0);
    assert_eq!(timebase.milliseconds_per_frame(), 16);
}

/// 测试24fps帧率下将1秒转换为帧数
#[test]
fn test_frames_from_time_one_second_24fps() {
    let timebase = Timebase::new(24.0);
    let time = Time::from_seconds(1.0);
    let frames = timebase.frames_from_time(&time);
    assert_eq!(frames, 24);
}

/// 测试30fps帧率下将1秒转换为帧数
#[test]
fn test_frames_from_time_one_second_30fps() {
    let timebase = Timebase::new(30.0);
    let time = Time::from_seconds(1.0);
    let frames = timebase.frames_from_time(&time);
    assert_eq!(frames, 30);
}

/// 测试24fps帧率下将0.5秒转换为帧数
#[test]
fn test_frames_from_time_half_second_24fps() {
    let timebase = Timebase::new(24.0);
    let time = Time::from_seconds(0.5);
    let frames = timebase.frames_from_time(&time);
    assert_eq!(frames, 12);
}

/// 测试24fps帧率下将1分钟转换为帧数
#[test]
fn test_frames_from_time_one_minute_24fps() {
    let timebase = Timebase::new(24.0);
    let time = Time::from_minutes(1.0);
    let frames = timebase.frames_from_time(&time);
    assert_eq!(frames, 1440);
}

/// 测试24fps帧率下将1小时转换为帧数
#[test]
fn test_frames_from_time_one_hour_24fps() {
    let timebase = Timebase::new(24.0);
    let time = Time::from_hours(1.0);
    let frames = timebase.frames_from_time(&time);
    assert_eq!(frames, 86400);
}

/// 测试将负时间转换为帧数
#[test]
fn test_frames_from_time_negative() {
    let timebase = Timebase::new(24.0);
    let time = Time::from_seconds(-1.0);
    let frames = timebase.frames_from_time(&time);
    assert_eq!(frames, -24);
}

/// 测试24fps帧率下将24帧转换为时间
#[test]
fn test_time_from_frames_24fps() {
    let timebase = Timebase::new(24.0);
    let time = timebase.time_from_frames(24);
    assert_eq!(time.to_seconds(), 1.0);
}

/// 测试30fps帧率下将30帧转换为时间
#[test]
fn test_time_from_frames_30fps() {
    let timebase = Timebase::new(30.0);
    let time = timebase.time_from_frames(30);
    assert_eq!(time.to_seconds(), 1.0);
}

/// 测试24fps帧率下将12帧转换为时间
#[test]
fn test_time_from_frames_half_second_24fps() {
    let timebase = Timebase::new(24.0);
    let time = timebase.time_from_frames(12);
    assert_eq!(time.to_seconds(), 0.5);
}

/// 测试将负帧数转换为时间
#[test]
fn test_time_from_frames_negative() {
    let timebase = Timebase::new(24.0);
    let time = timebase.time_from_frames(-24);
    assert_eq!(time.to_seconds(), -1.0);
}

/// 测试将0帧转换为时间
#[test]
fn test_time_from_frames_zero() {
    let timebase = Timebase::new(24.0);
    let time = timebase.time_from_frames(0);
    assert_eq!(time.to_seconds(), 0.0);
}

/// 测试24fps帧率下时间与帧数的往返转换
#[test]
fn test_roundtrip_frames_time_24fps() {
    let timebase = Timebase::new(24.0);
    let original_time = Time::from_seconds(10.5);
    let frames = timebase.frames_from_time(&original_time);
    let result_time = timebase.time_from_frames(frames);
    assert_eq!(original_time, result_time);
}

/// 测试30fps帧率下时间与帧数的往返转换
#[test]
fn test_roundtrip_frames_time_30fps() {
    let timebase = Timebase::new(30.0);
    let original_time = Time::from_seconds(10.5);
    let frames = timebase.frames_from_time(&original_time);
    let result_time = timebase.time_from_frames(frames);
    assert_eq!(original_time, result_time);
}

/// 测试60fps帧率下时间与帧数的往返转换
#[test]
fn test_roundtrip_frames_time_60fps() {
    let timebase = Timebase::new(60.0);
    let original_time = Time::from_seconds(10.5);
    let frames = timebase.frames_from_time(&original_time);
    let result_time = timebase.time_from_frames(frames);
    assert_eq!(original_time, result_time);
}

/// 测试Timebase类型的默认值
#[test]
fn test_default() {
    let timebase = Timebase::default();
    assert_eq!(timebase.fps(), 24.0);
    assert_eq!(timebase.framerate(), 24);
    assert_eq!(timebase.dropframe(), true);
}

/// 测试Timebase对象的克隆
#[test]
fn test_clone() {
    let timebase1 = Timebase::new(24.0);
    let timebase2 = timebase1;
    assert_eq!(timebase1.fps(), timebase2.fps());
    assert_eq!(timebase1.framerate(), timebase2.framerate());
}

/// 测试Timebase对象的复制
#[test]
fn test_copy() {
    let timebase1 = Timebase::new(24.0);
    let timebase2 = timebase1;
    assert_eq!(timebase1.fps(), 24.0);
    assert_eq!(timebase2.fps(), 24.0);
}

/// 测试Timebase对象的调试输出
#[test]
fn test_debug() {
    let timebase = Timebase::new(24.0);
    let debug_str = format!("{:?}", timebase);
    assert!(debug_str.contains("Timebase"));
}

/// 测试复杂时间值到帧数的转换
#[test]
fn test_complex_conversion() {
    let timebase = Timebase::new(24.0);
    let time = Time::from_hours(1.0) + Time::from_minutes(30.0) + Time::from_seconds(15.0);
    let frames = timebase.frames_from_time(&time);
    assert_eq!(frames, 129960);
}

/// 测试分数帧率下时间到帧数的转换
#[test]
fn test_fractional_fps_frames_calculation() {
    let timebase = Timebase::new(23.976);
    let time = Time::from_seconds(1.0);
    let frames = timebase.frames_from_time(&time);
    assert_eq!(frames, 24);
}

/// 测试分数帧率下帧数到时间的转换
#[test]
fn test_fractional_fps_time_calculation() {
    let timebase = Timebase::new(23.976);
    let time = timebase.time_from_frames(24);
    assert_eq!(time.to_seconds(), 1.0);
}
