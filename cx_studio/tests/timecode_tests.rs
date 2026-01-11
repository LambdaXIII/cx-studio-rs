use cx_studio::core::{Time, Timebase, Timecode};

/// 测试创建Timecode对象
#[test]
fn test_new() {
    let timebase = Timebase::new(24.0);
    let timecode = Timecode::new(1, 2, 3, 4, timebase);
    let expected_time = Time::from_hours(1.0)
        + Time::from_minutes(2.0)
        + Time::from_seconds(3.0)
        + Time::from_seconds(4.0 / 24.0);
    assert_eq!(timecode.to_time(), expected_time);
}

/// 测试创建零值Timecode对象
#[test]
fn test_new_zero() {
    let timebase = Timebase::new(24.0);
    let timecode = Timecode::new(0, 0, 0, 0, timebase);
    assert_eq!(timecode.to_time(), Time::zero());
}

/// 测试创建最大值Timecode对象
#[test]
fn test_new_max_values() {
    let timebase = Timebase::new(24.0);
    let timecode = Timecode::new(23, 59, 59, 23, timebase);
    let expected_time = Time::from_hours(23.0)
        + Time::from_minutes(59.0)
        + Time::from_seconds(59.0)
        + Time::from_seconds(23.0 / 24.0);
    assert_eq!(timecode.to_time(), expected_time);
}

/// 测试从零值Time创建Timecode对象
#[test]
fn test_from_time_zero() {
    let timebase = Timebase::new(24.0);
    let time = Time::zero();
    let timecode = Timecode::from_time(time, timebase);
    assert_eq!(timecode.to_time(), Time::zero());
}

/// 测试24fps帧率下从1秒Time创建Timecode对象
#[test]
fn test_from_time_one_second_24fps() {
    let timebase = Timebase::new(24.0);
    let time = Time::from_seconds(1.0);
    let timecode = Timecode::from_time(time, timebase);
    assert_eq!(timecode.to_time(), time);
}

/// 测试24fps帧率下从1分钟Time创建Timecode对象
#[test]
fn test_from_time_one_minute_24fps() {
    let timebase = Timebase::new(24.0);
    let time = Time::from_minutes(1.0);
    let timecode = Timecode::from_time(time, timebase);
    assert_eq!(timecode.to_time(), time);
}

/// 测试24fps帧率下从1小时Time创建Timecode对象
#[test]
fn test_from_time_one_hour_24fps() {
    let timebase = Timebase::new(24.0);
    let time = Time::from_hours(1.0);
    let timecode = Timecode::from_time(time, timebase);
    assert_eq!(timecode.to_time(), time);
}

/// 测试24fps帧率下从复杂Time创建Timecode对象
#[test]
fn test_from_time_complex_24fps() {
    let timebase = Timebase::new(24.0);
    let time = Time::from_hours(1.0)
        + Time::from_minutes(30.0)
        + Time::from_seconds(15.0)
        + Time::from_seconds(1.0 / 24.0);
    let timecode = Timecode::from_time(time, timebase);
    assert_eq!(timecode.to_time(), time);
}

/// 测试30fps帧率下从Time创建Timecode对象
#[test]
fn test_from_time_30fps() {
    let timebase = Timebase::new(30.0);
    let time = Time::from_seconds(1.5);
    let timecode = Timecode::from_time(time, timebase);
    assert_eq!(timecode.to_time(), time);
}

/// 测试从规范化时间创建Timecode对象
#[test]
fn test_from_time_normalized() {
    let timebase = Timebase::new(24.0);
    let time = Time::from_hours(25.0);
    let timecode = Timecode::from_time(time, timebase);
    assert_eq!(timecode.to_time(), time.normalized());
}

/// 测试零值Timecode转换为Time
#[test]
fn test_to_time_zero() {
    let timebase = Timebase::new(24.0);
    let timecode = Timecode::new(0, 0, 0, 0, timebase);
    let time = timecode.to_time();
    assert_eq!(time.to_seconds(), 0.0);
}

/// 测试24fps帧率下1秒Timecode转换为Time
#[test]
fn test_to_time_one_second_24fps() {
    let timebase = Timebase::new(24.0);
    let timecode = Timecode::new(0, 0, 1, 0, timebase);
    let time = timecode.to_time();
    assert_eq!(time.to_seconds(), 1.0);
}

/// 测试24fps帧率下1分钟Timecode转换为Time
#[test]
fn test_to_time_one_minute_24fps() {
    let timebase = Timebase::new(24.0);
    let timecode = Timecode::new(0, 1, 0, 0, timebase);
    let time = timecode.to_time();
    assert_eq!(time.to_seconds(), 60.0);
}

/// 测试24fps帧率下1小时Timecode转换为Time
#[test]
fn test_to_time_one_hour_24fps() {
    let timebase = Timebase::new(24.0);
    let timecode = Timecode::new(1, 0, 0, 0, timebase);
    let time = timecode.to_time();
    assert_eq!(time.to_seconds(), 3600.0);
}

/// 测试24fps帧率下复杂Timecode转换为Time
#[test]
fn test_to_time_complex_24fps() {
    let timebase = Timebase::new(24.0);
    let timecode = Timecode::new(1, 30, 15, 12, timebase);
    let time = timecode.to_time();
    let expected = Time::from_hours(1.0)
        + Time::from_minutes(30.0)
        + Time::from_seconds(15.0)
        + Time::from_milliseconds(500);
    assert_eq!(time, expected);
}

/// 测试30fps帧率下Timecode转换为Time
#[test]
fn test_to_time_30fps() {
    let timebase = Timebase::new(30.0);
    let timecode = Timecode::new(0, 0, 1, 15, timebase);
    let time = timecode.to_time();
    assert_eq!(time.to_seconds(), 1.5);
}

/// 测试24fps帧率下Time与Timecode的往返转换
#[test]
fn test_roundtrip_time_24fps() {
    let timebase = Timebase::new(24.0);
    let original_time = Time::from_hours(1.0)
        + Time::from_minutes(30.0)
        + Time::from_seconds(15.0)
        + Time::from_milliseconds(500);
    let timecode = Timecode::from_time(original_time, timebase);
    let result_time = timecode.to_time();
    assert_eq!(original_time, result_time);
}

/// 测试30fps帧率下Time与Timecode的往返转换
#[test]
fn test_roundtrip_time_30fps() {
    let timebase = Timebase::new(30.0);
    let original_time = Time::from_hours(1.0)
        + Time::from_minutes(30.0)
        + Time::from_seconds(15.0)
        + Time::from_milliseconds(500);
    let timecode = Timecode::from_time(original_time, timebase);
    let result_time = timecode.to_time();
    assert_eq!(original_time, result_time);
}

/// 测试60fps帧率下Time与Timecode的往返转换
#[test]
fn test_roundtrip_time_60fps() {
    let timebase = Timebase::new(60.0);
    let original_time = Time::from_hours(1.0)
        + Time::from_minutes(30.0)
        + Time::from_seconds(15.0)
        + Time::from_milliseconds(500);
    let timecode = Timecode::from_time(original_time, timebase);
    let result_time = timecode.to_time();
    assert_eq!(original_time, result_time);
}

/// 测试从冒号分隔的字符串创建Timecode对象
#[test]
fn test_from_string_colon_separator() {
    let timebase = Timebase::new(24.0);
    let timecode = Timecode::from_string("01:02:03:04", timebase).unwrap();
    let expected_time = Time::from_hours(1.0)
        + Time::from_minutes(2.0)
        + Time::from_seconds(3.0)
        + Time::from_seconds(4.0 / 24.0);
    assert_eq!(timecode.to_time(), expected_time);
}

/// 测试从分号分隔的字符串创建Timecode对象
#[test]
fn test_from_string_semicolon_separator() {
    let timebase = Timebase::new(24.0);
    let timecode = Timecode::from_string("01;02;03;04", timebase).unwrap();
    let expected_time = Time::from_hours(1.0)
        + Time::from_minutes(2.0)
        + Time::from_seconds(3.0)
        + Time::from_seconds(4.0 / 24.0);
    assert_eq!(timecode.to_time(), expected_time);
}

/// 测试从破折号分隔的字符串创建Timecode对象
#[test]
fn test_from_string_dash_separator() {
    let timebase = Timebase::new(24.0);
    let timecode = Timecode::from_string("01-02-03-04", timebase).unwrap();
    let expected_time = Time::from_hours(1.0)
        + Time::from_minutes(2.0)
        + Time::from_seconds(3.0)
        + Time::from_seconds(4.0 / 24.0);
    assert_eq!(timecode.to_time(), expected_time);
}

/// 测试从空格分隔的字符串创建Timecode对象
#[test]
fn test_from_string_space_separator() {
    let timebase = Timebase::new(24.0);
    let timecode = Timecode::from_string("01 02 03 04", timebase).unwrap();
    let expected_time = Time::from_hours(1.0)
        + Time::from_minutes(2.0)
        + Time::from_seconds(3.0)
        + Time::from_seconds(4.0 / 24.0);
    assert_eq!(timecode.to_time(), expected_time);
}

/// 测试从零值字符串创建Timecode对象
#[test]
fn test_from_string_zero() {
    let timebase = Timebase::new(24.0);
    let timecode = Timecode::from_string("00:00:00:00", timebase).unwrap();
    assert_eq!(timecode.to_time(), Time::zero());
}

/// 测试从最大值字符串创建Timecode对象
#[test]
fn test_from_string_max_values() {
    let timebase = Timebase::new(24.0);
    let timecode = Timecode::from_string("23:59:59:23", timebase).unwrap();
    let expected_time = Time::from_hours(23.0)
        + Time::from_minutes(59.0)
        + Time::from_seconds(59.0)
        + Time::from_seconds(23.0 / 24.0);
    assert_eq!(timecode.to_time(), expected_time);
}

/// 测试从不正确格式的字符串创建Timecode对象
#[test]
fn test_from_string_invalid_format() {
    let timebase = Timebase::new(24.0);
    let timecode = Timecode::from_string("01:02:03", timebase);
    assert!(timecode.is_none());
}

/// 测试从无分隔符的字符串创建Timecode对象
#[test]
fn test_from_string_invalid_separator() {
    let timebase = Timebase::new(24.0);
    let timecode = Timecode::from_string("01020304", timebase);
    assert!(timecode.is_none());
}

/// 测试从无效数值的字符串创建Timecode对象
#[test]
fn test_from_string_invalid_values() {
    let timebase = Timebase::new(24.0);
    let timecode = Timecode::from_string("24:60:60:24", timebase);
    assert!(timecode.is_some());
}

/// 测试从非数字字符串创建Timecode对象
#[test]
fn test_from_string_non_numeric() {
    let timebase = Timebase::new(24.0);
    let timecode = Timecode::from_string("ab:cd:ef:gh", timebase);
    assert!(timecode.is_none());
}

/// 测试从混合分隔符的字符串创建Timecode对象
#[test]
fn test_from_string_mixed_separators() {
    let timebase = Timebase::new(24.0);
    let timecode = Timecode::from_string("01:02-03:04", timebase).unwrap();
    let expected_time = Time::from_hours(1.0)
        + Time::from_minutes(2.0)
        + Time::from_seconds(3.0)
        + Time::from_seconds(4.0 / 24.0);
    assert_eq!(timecode.to_time(), expected_time);
}

/// 测试从大帧数的字符串创建Timecode对象
#[test]
fn test_from_string_large_frame_number() {
    let timebase = Timebase::new(24.0);
    let timecode = Timecode::from_string("00:00:00:100", timebase).unwrap();
    let expected_time = Time::from_seconds(100.0 / 24.0);
    assert_eq!(timecode.to_time(), expected_time);
}

/// 测试Timecode类型的默认值
#[test]
fn test_default() {
    let timecode = Timecode::default();
    assert_eq!(timecode.to_time(), Time::zero());
}

/// 测试Timebase对象在Timecode中的保留
#[test]
fn test_timebase_preservation() {
    let timebase1 = Timebase::new(24.0);
    let timebase2 = Timebase::new(30.0);
    let timecode1 = Timecode::new(1, 2, 3, 4, timebase1);
    let timecode2 = Timecode::new(1, 2, 3, 4, timebase2);
    assert_ne!(timecode1.to_time(), timecode2.to_time());
}

/// 测试24fps帧率下复杂时间值与Timecode的转换
#[test]
fn test_complex_conversion_24fps() {
    let timebase = Timebase::new(24.0);
    let time = Time::from_hours(12.0)
        + Time::from_minutes(34.0)
        + Time::from_seconds(56.0)
        + Time::from_seconds(12.0 / 24.0);
    let timecode = Timecode::from_time(time, timebase);
    assert_eq!(timecode.to_time(), time);
}

/// 测试Timecode与字符串的往返转换
#[test]
fn test_roundtrip_string() {
    let timebase = Timebase::new(24.0);
    let original_time = Time::from_hours(1.0)
        + Time::from_minutes(30.0)
        + Time::from_seconds(15.0)
        + Time::from_seconds(12.0 / 24.0);
    let timecode1 = Timecode::from_time(original_time, timebase);
    let timecode2 = Timecode::from_string("01:30:15:12", timebase).unwrap();
    assert_eq!(timecode1.to_time(), timecode2.to_time());
}

/// 测试午夜边缘情况的Timecode
#[test]
fn test_edge_case_midnight() {
    let timebase = Timebase::new(24.0);
    let timecode = Timecode::new(0, 0, 0, 0, timebase);
    assert_eq!(timecode.to_time(), Time::zero());
}

/// 测试单帧边缘情况的Timecode
#[test]
fn test_edge_case_one_frame() {
    let timebase = Timebase::new(24.0);
    let timecode = Timecode::new(0, 0, 0, 1, timebase);
    let expected_time = Time::from_seconds(1.0 / 24.0);
    assert_eq!(timecode.to_time(), expected_time);
}

/// 测试接近午夜边缘情况的Timecode
#[test]
fn test_edge_case_almost_midnight() {
    let timebase = Timebase::new(24.0);
    let timecode = Timecode::new(23, 59, 59, 23, timebase);
    let expected_time = Time::from_hours(23.0)
        + Time::from_minutes(59.0)
        + Time::from_seconds(59.0)
        + Time::from_seconds(23.0 / 24.0);
    assert_eq!(timecode.to_time(), expected_time);
}

/// 测试分数帧率下Time与Timecode的转换
#[test]
fn test_fractional_fps() {
    let timebase = Timebase::new(23.976);
    let time = Time::from_seconds(1.0);
    let timecode = Timecode::from_time(time, timebase);
    assert_eq!(timecode.to_time(), time);
}
