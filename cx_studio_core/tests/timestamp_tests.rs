use cx_studio_core::time::{Time, Timestamp};

/// 测试创建Timestamp对象
#[test]
fn test_new() {
    let timestamp = Timestamp::new(1, 2, 3, 4);
    let expected_time = Time::from_hours(1.0)
        + Time::from_minutes(2.0)
        + Time::from_seconds(3.0)
        + Time::from_milliseconds(4);
    assert_eq!(timestamp.to_time(), expected_time);
}

/// 测试创建零值Timestamp对象
#[test]
fn test_new_zero() {
    let timestamp = Timestamp::new(0, 0, 0, 0);
    assert_eq!(timestamp.to_time(), Time::zero());
}

/// 测试创建最大值Timestamp对象
#[test]
fn test_new_max_values() {
    let timestamp = Timestamp::new(23, 59, 59, 999);
    let expected_time = Time::from_hours(23.0)
        + Time::from_minutes(59.0)
        + Time::from_seconds(59.0)
        + Time::from_milliseconds(999);
    assert_eq!(timestamp.to_time(), expected_time);
}

/// 测试从零值Time创建Timestamp对象
#[test]
fn test_from_time_zero() {
    let time = Time::zero();
    let timestamp = Timestamp::from_time(time);
    assert_eq!(timestamp.to_time(), Time::zero());
}

/// 测试从1秒Time创建Timestamp对象
#[test]
fn test_from_time_one_second() {
    let time = Time::from_seconds(1.0);
    let timestamp = Timestamp::from_time(time);
    assert_eq!(timestamp.to_time(), time);
}

/// 测试从1分钟Time创建Timestamp对象
#[test]
fn test_from_time_one_minute() {
    let time = Time::from_minutes(1.0);
    let timestamp = Timestamp::from_time(time);
    assert_eq!(timestamp.to_time(), time);
}

/// 测试从1小时Time创建Timestamp对象
#[test]
fn test_from_time_one_hour() {
    let time = Time::from_hours(1.0);
    let timestamp = Timestamp::from_time(time);
    assert_eq!(timestamp.to_time(), time);
}

/// 测试从带毫秒的Time创建Timestamp对象
#[test]
fn test_from_time_with_milliseconds() {
    let time = Time::from_milliseconds(1234);
    let timestamp = Timestamp::from_time(time);
    assert_eq!(timestamp.to_time(), time);
}

/// 测试从复杂Time创建Timestamp对象
#[test]
fn test_from_time_complex() {
    let time = Time::from_hours(1.0)
        + Time::from_minutes(30.0)
        + Time::from_seconds(15.0)
        + Time::from_milliseconds(500);
    let timestamp = Timestamp::from_time(time);
    assert_eq!(timestamp.to_time(), time);
}

/// 测试从规范化时间创建Timestamp对象
#[test]
fn test_from_time_normalized() {
    let time = Time::from_hours(25.0);
    let timestamp = Timestamp::from_time(time);
    assert_eq!(timestamp.to_time(), time.normalized());
}

/// 测试从负时间创建规范化的Timestamp对象
#[test]
fn test_from_time_negative_normalized() {
    let time = Time::from_hours(-1.0);
    let timestamp = Timestamp::from_time(time);
    assert_eq!(timestamp.to_time(), time.normalized());
}

/// 测试零值Timestamp转换为Time
#[test]
fn test_to_time_zero() {
    let timestamp = Timestamp::new(0, 0, 0, 0);
    let time = timestamp.to_time();
    assert_eq!(time.to_milliseconds(), 0);
}

/// 测试1秒Timestamp转换为Time
#[test]
fn test_to_time_one_second() {
    let timestamp = Timestamp::new(0, 0, 1, 0);
    let time = timestamp.to_time();
    assert_eq!(time.to_seconds(), 1.0);
}

/// 测试1分钟Timestamp转换为Time
#[test]
fn test_to_time_one_minute() {
    let timestamp = Timestamp::new(0, 1, 0, 0);
    let time = timestamp.to_time();
    assert_eq!(time.to_minutes(), 1.0);
}

/// 测试1小时Timestamp转换为Time
#[test]
fn test_to_time_one_hour() {
    let timestamp = Timestamp::new(1, 0, 0, 0);
    let time = timestamp.to_time();
    assert_eq!(time.to_hours(), 1.0);
}

/// 测试带毫秒的Timestamp转换为Time
#[test]
fn test_to_time_with_milliseconds() {
    let timestamp = Timestamp::new(0, 0, 1, 500);
    let time = timestamp.to_time();
    assert_eq!(time.to_milliseconds(), 1500);
}

/// 测试复杂Timestamp转换为Time
#[test]
fn test_to_time_complex() {
    let timestamp = Timestamp::new(1, 30, 15, 500);
    let time = timestamp.to_time();
    let expected = Time::from_hours(1.0)
        + Time::from_minutes(30.0)
        + Time::from_seconds(15.0)
        + Time::from_milliseconds(500);
    assert_eq!(time, expected);
}

/// 测试Time与Timestamp的往返转换
#[test]
fn test_roundtrip_time() {
    let original_time = Time::from_hours(1.0)
        + Time::from_minutes(30.0)
        + Time::from_seconds(15.0)
        + Time::from_milliseconds(500);
    let timestamp = Timestamp::from_time(original_time);
    let result_time = timestamp.to_time();
    assert_eq!(original_time, result_time);
}

/// 测试规范化时间与Timestamp的往返转换
#[test]
fn test_roundtrip_time_normalized() {
    let original_time =
        Time::from_hours(25.0) + Time::from_minutes(70.0) + Time::from_seconds(80.0);
    let timestamp = Timestamp::from_time(original_time);
    let result_time = timestamp.to_time();
    let normalized = original_time.normalized();
    assert_eq!(result_time, normalized);
}

/// 测试从冒号分隔的字符串创建Timestamp对象
#[test]
fn test_from_string_colon_separator() {
    let timestamp = Timestamp::from_string("01:02:03.004").unwrap();
    let expected_time = Time::from_hours(1.0)
        + Time::from_minutes(2.0)
        + Time::from_seconds(3.0)
        + Time::from_milliseconds(4);
    assert_eq!(timestamp.to_time(), expected_time);
}

/// 测试从分号分隔的字符串创建Timestamp对象
#[test]
fn test_from_string_semicolon_separator() {
    let timestamp = Timestamp::from_string("01;02;03;004").unwrap();
    let expected_time = Time::from_hours(1.0)
        + Time::from_minutes(2.0)
        + Time::from_seconds(3.0)
        + Time::from_milliseconds(4);
    assert_eq!(timestamp.to_time(), expected_time);
}

/// 测试从破折号分隔的字符串创建Timestamp对象
#[test]
fn test_from_string_dash_separator() {
    let timestamp = Timestamp::from_string("01-02-03-004").unwrap();
    let expected_time = Time::from_hours(1.0)
        + Time::from_minutes(2.0)
        + Time::from_seconds(3.0)
        + Time::from_milliseconds(4);
    assert_eq!(timestamp.to_time(), expected_time);
}

/// 测试从空格分隔的字符串创建Timestamp对象
#[test]
fn test_from_string_space_separator() {
    let timestamp = Timestamp::from_string("01 02 03 004").unwrap();
    let expected_time = Time::from_hours(1.0)
        + Time::from_minutes(2.0)
        + Time::from_seconds(3.0)
        + Time::from_milliseconds(4);
    assert_eq!(timestamp.to_time(), expected_time);
}

/// 测试从零值字符串创建Timestamp对象
#[test]
fn test_from_string_zero() {
    let timestamp = Timestamp::from_string("00:00:00.000").unwrap();
    assert_eq!(timestamp.to_time(), Time::zero());
}

/// 测试从最大值字符串创建Timestamp对象
#[test]
fn test_from_string_max_values() {
    let timestamp = Timestamp::from_string("23:59:59.999").unwrap();
    let expected_time = Time::from_hours(23.0)
        + Time::from_minutes(59.0)
        + Time::from_seconds(59.0)
        + Time::from_milliseconds(999);
    assert_eq!(timestamp.to_time(), expected_time);
}

/// 测试从不正确格式的字符串创建Timestamp对象
#[test]
fn test_from_string_invalid_format() {
    let timestamp = Timestamp::from_string("01:02:03");
    assert!(timestamp.is_none());
}

/// 测试从无分隔符的字符串创建Timestamp对象
#[test]
fn test_from_string_invalid_separator() {
    let timestamp = Timestamp::from_string("010203004");
    assert!(timestamp.is_none());
}

/// 测试从无效数值的字符串创建Timestamp对象
#[test]
fn test_from_string_invalid_values() {
    let timestamp = Timestamp::from_string("24:60:60.1000");
    assert!(timestamp.is_none());
}

/// 测试从非数字字符串创建Timestamp对象
#[test]
fn test_from_string_non_numeric() {
    let timestamp = Timestamp::from_string("ab:cd:ef.ghi");
    assert!(timestamp.is_none());
}

/// 测试从混合分隔符的字符串创建Timestamp对象
#[test]
fn test_from_string_mixed_separators() {
    let timestamp = Timestamp::from_string("01:02-03.004").unwrap();
    let expected_time = Time::from_hours(1.0)
        + Time::from_minutes(2.0)
        + Time::from_seconds(3.0)
        + Time::from_milliseconds(4);
    assert_eq!(timestamp.to_time(), expected_time);
}

/// 测试从带前导零的字符串创建Timestamp对象
#[test]
fn test_from_string_leading_zeros() {
    let timestamp = Timestamp::from_string("01:02:03.004").unwrap();
    let expected_time = Time::from_hours(1.0)
        + Time::from_minutes(2.0)
        + Time::from_seconds(3.0)
        + Time::from_milliseconds(4);
    assert_eq!(timestamp.to_time(), expected_time);
}

/// 测试Timestamp对象的显示格式
#[test]
fn test_display() {
    let timestamp = Timestamp::new(1, 2, 3, 4);
    let display_str = format!("{}", timestamp);
    assert_eq!(display_str, "01:02:03.004");
}

/// 测试零值Timestamp对象的显示格式
#[test]
fn test_display_zero() {
    let timestamp = Timestamp::new(0, 0, 0, 0);
    let display_str = format!("{}", timestamp);
    assert_eq!(display_str, "00:00:00.000");
}

/// 测试最大值Timestamp对象的显示格式
#[test]
fn test_display_max_values() {
    let timestamp = Timestamp::new(23, 59, 59, 999);
    let display_str = format!("{}", timestamp);
    assert_eq!(display_str, "23:59:59.999");
}

/// 测试复杂Timestamp对象的显示格式
#[test]
fn test_display_complex() {
    let timestamp = Timestamp::new(12, 34, 56, 789);
    let display_str = format!("{}", timestamp);
    assert_eq!(display_str, "12:34:56.789");
}

/// 测试从Time类型转换为Timestamp类型
#[test]
fn test_from_time() {
    let time = Time::from_seconds(1.5);
    let timestamp: Timestamp = time.into();
    assert_eq!(timestamp.to_time(), time);
}

/// 测试从Timestamp类型转换为Time类型
#[test]
fn test_into_time() {
    let timestamp = Timestamp::new(0, 0, 1, 500);
    let time: Time = timestamp.into();
    assert_eq!(time.to_milliseconds(), 1500);
}

/// 测试Timestamp类型的默认值
#[test]
fn test_default() {
    let timestamp = Timestamp::default();
    assert_eq!(timestamp.to_time(), Time::zero());
}

/// 测试Timestamp对象的克隆
#[test]
fn test_clone() {
    let timestamp1 = Timestamp::new(1, 2, 3, 4);
    let timestamp2 = timestamp1.clone();
    assert_eq!(timestamp1.to_time(), timestamp2.to_time());
}

/// 测试Timestamp对象的复制
#[test]
fn test_copy() {
    let timestamp1 = Timestamp::new(1, 2, 3, 4);
    let timestamp2 = timestamp1.clone();
    assert_eq!(
        timestamp1.to_time(),
        Time::from_hours(1.0)
            + Time::from_minutes(2.0)
            + Time::from_seconds(3.0)
            + Time::from_milliseconds(4)
    );
    assert_eq!(
        timestamp2.to_time(),
        Time::from_hours(1.0)
            + Time::from_minutes(2.0)
            + Time::from_seconds(3.0)
            + Time::from_milliseconds(4)
    );
}

/// 测试Timestamp对象的相等性比较
#[test]
fn test_equality() {
    let timestamp1 = Timestamp::new(1, 2, 3, 4);
    let timestamp2 = Timestamp::new(1, 2, 3, 4);
    let timestamp3 = Timestamp::new(1, 2, 3, 5);
    assert_eq!(timestamp1, timestamp2);
    assert_ne!(timestamp1, timestamp3);
}

/// 测试复杂时间值与Timestamp的转换
#[test]
fn test_complex_conversion() {
    let time = Time::from_hours(12.0)
        + Time::from_minutes(34.0)
        + Time::from_seconds(56.0)
        + Time::from_milliseconds(789);
    let timestamp = Timestamp::from_time(time);
    assert_eq!(timestamp.to_time(), time);
}

/// 测试Timestamp与字符串的往返转换
#[test]
fn test_roundtrip_string() {
    let original = "12:34:56.789";
    let timestamp = Timestamp::from_string(original).unwrap();
    let display_str = format!("{}", timestamp);
    assert_eq!(original, display_str);
}

/// 测试午夜边缘情况的Timestamp
#[test]
fn test_edge_case_midnight() {
    let timestamp = Timestamp::new(0, 0, 0, 0);
    let time = timestamp.to_time();
    assert_eq!(time.to_milliseconds(), 0);
}

/// 测试单毫秒边缘情况的Timestamp
#[test]
fn test_edge_case_one_millisecond() {
    let timestamp = Timestamp::new(0, 0, 0, 1);
    let time = timestamp.to_time();
    assert_eq!(time.to_milliseconds(), 1);
}

/// 测试接近午夜边缘情况的Timestamp
#[test]
fn test_edge_case_almost_midnight() {
    let timestamp = Timestamp::new(23, 59, 59, 999);
    let time = timestamp.to_time();
    assert_eq!(time.to_milliseconds(), 86399999);
}

/// 测试负时间规范化为Timestamp
#[test]
fn test_negative_time_normalized() {
    let time = Time::from_seconds(-1.0);
    let timestamp = Timestamp::from_time(time);
    let expected_time =
        Time::from_hours(23.0) + Time::from_minutes(59.0) + Time::from_seconds(59.0);
    assert_eq!(timestamp.to_time(), expected_time);
}

/// 测试大毫秒值的Timestamp
#[test]
fn test_large_millisecond_value() {
    let timestamp = Timestamp::new(0, 0, 1, 999);
    let time = timestamp.to_time();
    assert_eq!(time.to_milliseconds(), 1999);
}

/// 测试从逗号分隔的字符串创建Timestamp对象
#[test]
fn test_from_string_with_comma_separator() {
    let timestamp = Timestamp::from_string("01,02,03,004").unwrap();
    let expected_time = Time::from_hours(1.0)
        + Time::from_minutes(2.0)
        + Time::from_seconds(3.0)
        + Time::from_milliseconds(4);
    assert_eq!(timestamp.to_time(), expected_time);
}

/// 测试从点分隔的字符串创建Timestamp对象
#[test]
fn test_from_string_with_dot_separator() {
    let timestamp = Timestamp::from_string("01.02.03.004").unwrap();
    let expected_time = Time::from_hours(1.0)
        + Time::from_minutes(2.0)
        + Time::from_seconds(3.0)
        + Time::from_milliseconds(4);
    assert_eq!(timestamp.to_time(), expected_time);
}

/// 测试Timestamp显示格式的往返转换
#[test]
fn test_display_roundtrip() {
    let timestamp1 = Timestamp::new(12, 34, 56, 789);
    let display_str = format!("{}", timestamp1);
    let timestamp2 = Timestamp::from_string(&display_str).unwrap();
    assert_eq!(timestamp1, timestamp2);
}
