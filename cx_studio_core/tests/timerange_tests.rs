use cx_studio_core::time::{Time, TimeRange};

/// 测试创建有序的TimeRange对象
#[test]
fn test_new_ordered() {
    let start = Time::from_seconds(1.0);
    let end = Time::from_seconds(5.0);
    let range = TimeRange::new(start, end);
    assert_eq!(range.start, start);
    assert_eq!(range.end, end);
}

/// 测试创建反向的TimeRange对象
#[test]
fn test_new_reversed() {
    let start = Time::from_seconds(5.0);
    let end = Time::from_seconds(1.0);
    let range = TimeRange::new(start, end);
    assert_eq!(range.start, end);
    assert_eq!(range.end, start);
}

/// 测试创建起止时间相等的TimeRange对象
#[test]
fn test_new_equal() {
    let time = Time::from_seconds(3.0);
    let range = TimeRange::new(time, time);
    assert_eq!(range.start, time);
    assert_eq!(range.end, time);
}

/// 测试创建零值的TimeRange对象
#[test]
fn test_new_zero() {
    let start = Time::zero();
    let end = Time::zero();
    let range = TimeRange::new(start, end);
    assert_eq!(range.start, Time::zero());
    assert_eq!(range.end, Time::zero());
}

/// 测试创建负时间的TimeRange对象
#[test]
fn test_new_negative() {
    let start = Time::from_seconds(-5.0);
    let end = Time::from_seconds(-1.0);
    let range = TimeRange::new(start, end);
    assert_eq!(range.start, start);
    assert_eq!(range.end, end);
}

/// 测试创建混合符号的TimeRange对象
#[test]
fn test_new_mixed_signs() {
    let start = Time::from_seconds(-1.0);
    let end = Time::from_seconds(1.0);
    let range = TimeRange::new(start, end);
    assert_eq!(range.start, start);
    assert_eq!(range.end, end);
}

/// 测试计算正向TimeRange的持续时间
#[test]
fn test_duration_positive() {
    let start = Time::from_seconds(1.0);
    let end = Time::from_seconds(5.0);
    let range = TimeRange::new(start, end);
    let duration = range.duration();
    assert_eq!(duration.to_seconds(), 4.0);
}

/// 测试计算反向TimeRange的持续时间
#[test]
fn test_duration_reversed() {
    let start = Time::from_seconds(5.0);
    let end = Time::from_seconds(1.0);
    let range = TimeRange::new(start, end);
    let duration = range.duration();
    assert_eq!(duration.to_seconds(), 4.0);
}

/// 测试计算零持续时间的TimeRange
#[test]
fn test_duration_zero() {
    let time = Time::from_seconds(3.0);
    let range = TimeRange::new(time, time);
    let duration = range.duration();
    assert_eq!(duration.to_seconds(), 0.0);
}

/// 测试计算小时单位的TimeRange持续时间
#[test]
fn test_duration_hours() {
    let start = Time::from_hours(1.0);
    let end = Time::from_hours(3.0);
    let range = TimeRange::new(start, end);
    let duration = range.duration();
    assert_eq!(duration.to_hours(), 2.0);
}

/// 测试计算分钟单位的TimeRange持续时间
#[test]
fn test_duration_minutes() {
    let start = Time::from_minutes(10.0);
    let end = Time::from_minutes(30.0);
    let range = TimeRange::new(start, end);
    let duration = range.duration();
    assert_eq!(duration.to_minutes(), 20.0);
}

/// 测试两个不重叠的TimeRange
#[test]
fn test_is_overlapped_with_no_overlap() {
    let range1 = TimeRange::new(Time::from_seconds(0.0), Time::from_seconds(5.0));
    let range2 = TimeRange::new(Time::from_seconds(6.0), Time::from_seconds(10.0));
    assert!(!range1.is_overlapped_with(&range2));
}

/// 测试两个相邻的TimeRange
#[test]
fn test_is_overlapped_with_adjacent() {
    let range1 = TimeRange::new(Time::from_seconds(0.0), Time::from_seconds(5.0));
    let range2 = TimeRange::new(Time::from_seconds(5.0), Time::from_seconds(10.0));
    assert!(!range1.is_overlapped_with(&range2));
}

/// 测试两个部分重叠的TimeRange
#[test]
fn test_is_overlapped_with_partial_overlap() {
    let range1 = TimeRange::new(Time::from_seconds(0.0), Time::from_seconds(5.0));
    let range2 = TimeRange::new(Time::from_seconds(3.0), Time::from_seconds(8.0));
    assert!(range1.is_overlapped_with(&range2));
}

/// 测试一个TimeRange完全包含另一个TimeRange
#[test]
fn test_is_overlapped_with_complete_overlap() {
    let range1 = TimeRange::new(Time::from_seconds(0.0), Time::from_seconds(10.0));
    let range2 = TimeRange::new(Time::from_seconds(3.0), Time::from_seconds(7.0));
    assert!(range1.is_overlapped_with(&range2));
}

/// 测试两个完全相同的TimeRange
#[test]
fn test_is_overlapped_with_identical() {
    let range1 = TimeRange::new(Time::from_seconds(0.0), Time::from_seconds(5.0));
    let range2 = TimeRange::new(Time::from_seconds(0.0), Time::from_seconds(5.0));
    assert!(range1.is_overlapped_with(&range2));
}

/// 测试一个TimeRange被另一个TimeRange包含
#[test]
fn test_is_overlapped_with_contained() {
    let range1 = TimeRange::new(Time::from_seconds(3.0), Time::from_seconds(7.0));
    let range2 = TimeRange::new(Time::from_seconds(0.0), Time::from_seconds(10.0));
    assert!(range1.is_overlapped_with(&range2));
}

/// 测试两个负时间的TimeRange重叠
#[test]
fn test_is_overlapped_with_negative() {
    let range1 = TimeRange::new(Time::from_seconds(-5.0), Time::from_seconds(-1.0));
    let range2 = TimeRange::new(Time::from_seconds(-3.0), Time::from_seconds(0.0));
    assert!(range1.is_overlapped_with(&range2));
}

/// 测试混合符号的TimeRange重叠
#[test]
fn test_is_overlapped_with_mixed_signs() {
    let range1 = TimeRange::new(Time::from_seconds(-2.0), Time::from_seconds(2.0));
    let range2 = TimeRange::new(Time::from_seconds(1.0), Time::from_seconds(5.0));
    assert!(range1.is_overlapped_with(&range2));
}

/// 测试零持续时间的TimeRange重叠
#[test]
fn test_is_overlapped_with_zero_duration() {
    let range1 = TimeRange::new(Time::from_seconds(3.0), Time::from_seconds(3.0));
    let range2 = TimeRange::new(Time::from_seconds(2.0), Time::from_seconds(4.0));
    assert!(range1.is_overlapped_with(&range2));
}

/// 测试TimeRange类型的默认值
#[test]
fn test_default() {
    let range = TimeRange::default();
    assert_eq!(range.start, Time::zero());
    assert_eq!(range.end, Time::zero());
}

/// 测试从有序元组创建TimeRange对象
#[test]
fn test_from_tuple_ordered() {
    let tuple = (Time::from_seconds(1.0), Time::from_seconds(5.0));
    let range: TimeRange = tuple.into();
    assert_eq!(range.start, Time::from_seconds(1.0));
    assert_eq!(range.end, Time::from_seconds(5.0));
}

/// 测试从反向元组创建TimeRange对象
#[test]
fn test_from_tuple_reversed() {
    let tuple = (Time::from_seconds(5.0), Time::from_seconds(1.0));
    let range: TimeRange = tuple.into();
    assert_eq!(range.start, Time::from_seconds(1.0));
    assert_eq!(range.end, Time::from_seconds(5.0));
}

/// 测试将TimeRange对象转换为元组
#[test]
fn test_into_tuple() {
    let range = TimeRange::new(Time::from_seconds(1.0), Time::from_seconds(5.0));
    let tuple: (Time, Time) = range.into();
    assert_eq!(tuple.0, Time::from_seconds(1.0));
    assert_eq!(tuple.1, Time::from_seconds(5.0));
}

/// 测试TimeRange与元组的往返转换
#[test]
fn test_roundtrip_tuple() {
    let original = (Time::from_seconds(1.0), Time::from_seconds(5.0));
    let range: TimeRange = original.into();
    let result: (Time, Time) = range.into();
    assert_eq!(original, result);
}

/// 测试计算复杂时间值的TimeRange持续时间
#[test]
fn test_complex_duration() {
    let start = Time::from_hours(1.0) + Time::from_minutes(30.0) + Time::from_seconds(15.0);
    let end = Time::from_hours(2.0) + Time::from_minutes(45.0) + Time::from_seconds(30.0);
    let range = TimeRange::new(start, end);
    let duration = range.duration();
    assert_eq!(duration.to_seconds(), 4515.0);
}

/// 测试大数值的TimeRange持续时间
#[test]
fn test_large_values() {
    let start = Time::from_hours(100.0);
    let end = Time::from_hours(200.0);
    let range = TimeRange::new(start, end);
    let duration = range.duration();
    assert_eq!(duration.to_hours(), 100.0);
}

/// 测试负时间值的TimeRange持续时间
#[test]
fn test_negative_duration() {
    let start = Time::from_seconds(-10.0);
    let end = Time::from_seconds(-5.0);
    let range = TimeRange::new(start, end);
    let duration = range.duration();
    assert_eq!(duration.to_seconds(), 5.0);
}

/// 测试多个TimeRange之间的重叠关系
#[test]
fn test_multiple_overlaps() {
    let range1 = TimeRange::new(Time::from_seconds(0.0), Time::from_seconds(10.0));
    let range2 = TimeRange::new(Time::from_seconds(2.0), Time::from_seconds(4.0));
    let range3 = TimeRange::new(Time::from_seconds(6.0), Time::from_seconds(8.0));
    let range4 = TimeRange::new(Time::from_seconds(12.0), Time::from_seconds(14.0));

    assert!(range1.is_overlapped_with(&range2));
    assert!(range1.is_overlapped_with(&range3));
    assert!(!range1.is_overlapped_with(&range4));
    assert!(!range2.is_overlapped_with(&range3));
    assert!(!range2.is_overlapped_with(&range4));
    assert!(!range3.is_overlapped_with(&range4));
}

/// 测试零到零的边缘情况
#[test]
fn test_edge_case_zero_to_zero() {
    let range = TimeRange::new(Time::zero(), Time::zero());
    assert_eq!(range.start, Time::zero());
    assert_eq!(range.end, Time::zero());
    assert_eq!(range.duration(), Time::zero());
}

/// 测试单毫秒的边缘情况
#[test]
fn test_edge_case_single_millisecond() {
    let start = Time::from_milliseconds(0);
    let end = Time::from_milliseconds(1);
    let range = TimeRange::new(start, end);
    assert_eq!(range.duration().to_milliseconds(), 1);
}

/// 测试从负到正的边缘情况
#[test]
fn test_edge_case_negative_to_positive() {
    let start = Time::from_seconds(-1.0);
    let end = Time::from_seconds(1.0);
    let range = TimeRange::new(start, end);
    assert_eq!(range.duration().to_seconds(), 2.0);
}
