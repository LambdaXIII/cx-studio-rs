use cx_studio::core::Time;

/// 测试创建零值时间对象
#[test]
fn test_zero() {
    let time = Time::zero();
    assert_eq!(time.to_milliseconds(), 0);
}

/// 测试从毫秒创建时间对象
#[test]
fn test_from_milliseconds() {
    let time = Time::from_milliseconds(1000);
    assert_eq!(time.to_milliseconds(), 1000);
}

/// 测试从负毫秒创建时间对象
#[test]
fn test_from_milliseconds_negative() {
    let time = Time::from_milliseconds(-1000);
    assert_eq!(time.to_milliseconds(), -1000);
}

/// 测试从秒创建时间对象
#[test]
fn test_from_seconds() {
    let time = Time::from_seconds(1.5);
    assert_eq!(time.to_milliseconds(), 1500);
    assert_eq!(time.to_seconds(), 1.5);
}

/// 测试从负秒创建时间对象
#[test]
fn test_from_seconds_negative() {
    let time = Time::from_seconds(-1.5);
    assert_eq!(time.to_milliseconds(), -1500);
    assert_eq!(time.to_seconds(), -1.5);
}

/// 测试从分钟创建时间对象
#[test]
fn test_from_minutes() {
    let time = Time::from_minutes(1.5);
    assert_eq!(time.to_milliseconds(), 90000);
    assert_eq!(time.to_minutes(), 1.5);
}

/// 测试从负分钟创建时间对象
#[test]
fn test_from_minutes_negative() {
    let time = Time::from_minutes(-1.5);
    assert_eq!(time.to_milliseconds(), -90000);
    assert_eq!(time.to_minutes(), -1.5);
}

/// 测试从小时创建时间对象
#[test]
fn test_from_hours() {
    let time = Time::from_hours(1.5);
    assert_eq!(time.to_milliseconds(), 5400000);
    assert_eq!(time.to_hours(), 1.5);
}

/// 测试从负小时创建时间对象
#[test]
fn test_from_hours_negative() {
    let time = Time::from_hours(-1.5);
    assert_eq!(time.to_milliseconds(), -5400000);
    assert_eq!(time.to_hours(), -1.5);
}

/// 测试将时间转换为毫秒
#[test]
fn test_to_milliseconds() {
    let time = Time::from_milliseconds(12345);
    assert_eq!(time.to_milliseconds(), 12345);
}

/// 测试将时间转换为秒
#[test]
fn test_to_seconds() {
    let time = Time::from_milliseconds(2500);
    assert_eq!(time.to_seconds(), 2.5);
}

/// 测试将时间转换为分钟
#[test]
fn test_to_minutes() {
    let time = Time::from_milliseconds(180000);
    assert_eq!(time.to_minutes(), 3.0);
}

/// 测试将时间转换为小时
#[test]
fn test_to_hours() {
    let time = Time::from_milliseconds(7200000);
    assert_eq!(time.to_hours(), 2.0);
}

/// 测试正时间值的规范化
#[test]
fn test_normalized_positive() {
    let time = Time::from_hours(25.0);
    let normalized = time.normalized();
    assert_eq!(normalized.to_hours(), 1.0);
}

/// 测试负时间值的规范化
#[test]
fn test_normalized_negative() {
    let time = Time::from_hours(-1.0);
    let normalized = time.normalized();
    assert_eq!(normalized.to_hours(), 23.0);
}

/// 测试零值时间的规范化
#[test]
fn test_normalized_zero() {
    let time = Time::zero();
    let normalized = time.normalized();
    assert_eq!(normalized.to_milliseconds(), 0);
}

/// 测试恰好一天的时间规范化
#[test]
fn test_normalized_exactly_one_day() {
    let time = Time::from_hours(24.0);
    let normalized = time.normalized();
    assert_eq!(normalized.to_milliseconds(), 0);
}

/// 测试时间加法运算
#[test]
fn test_add() {
    let time1 = Time::from_seconds(1.0);
    let time2 = Time::from_seconds(2.0);
    let result = time1 + time2;
    assert_eq!(result.to_seconds(), 3.0);
}

/// 测试包含负时间的加法运算
#[test]
fn test_add_negative() {
    let time1 = Time::from_seconds(5.0);
    let time2 = Time::from_seconds(-2.0);
    let result = time1 + time2;
    assert_eq!(result.to_seconds(), 3.0);
}

/// 测试时间减法运算
#[test]
fn test_sub() {
    let time1 = Time::from_seconds(5.0);
    let time2 = Time::from_seconds(2.0);
    let result = time1 - time2;
    assert_eq!(result.to_seconds(), 3.0);
}

/// 测试包含负时间的减法运算
#[test]
fn test_sub_negative() {
    let time1 = Time::from_seconds(2.0);
    let time2 = Time::from_seconds(5.0);
    let result = time1 - time2;
    assert_eq!(result.to_seconds(), -3.0);
}

/// 测试时间加法赋值运算
#[test]
fn test_add_assign() {
    let mut time = Time::from_seconds(1.0);
    time += Time::from_seconds(2.0);
    assert_eq!(time.to_seconds(), 3.0);
}

/// 测试时间减法赋值运算
#[test]
fn test_sub_assign() {
    let mut time = Time::from_seconds(5.0);
    time -= Time::from_seconds(2.0);
    assert_eq!(time.to_seconds(), 3.0);
}

/// 测试时间与浮点数的乘法运算
#[test]
fn test_mul_f64() {
    let time = Time::from_seconds(2.0);
    let result = time * 3.0;
    assert_eq!(result.to_seconds(), 6.0);
}

/// 测试时间与分数浮点数的乘法运算
#[test]
fn test_mul_f64_fractional() {
    let time = Time::from_seconds(2.0);
    let result = time * 1.5;
    assert_eq!(result.to_seconds(), 3.0);
}

/// 测试时间与负浮点数的乘法运算
#[test]
fn test_mul_f64_negative() {
    let time = Time::from_seconds(2.0);
    let result = time * -2.0;
    assert_eq!(result.to_seconds(), -4.0);
}

/// 测试时间与浮点数的除法运算
#[test]
fn test_div_f64() {
    let time = Time::from_seconds(6.0);
    let result = time / 3.0;
    assert_eq!(result.to_seconds(), 2.0);
}

/// 测试时间与分数浮点数的除法运算
#[test]
fn test_div_f64_fractional() {
    let time = Time::from_seconds(3.0);
    let result = time / 2.0;
    assert_eq!(result.to_seconds(), 1.5);
}

/// 测试时间与负浮点数的除法运算
#[test]
fn test_div_f64_negative() {
    let time = Time::from_seconds(6.0);
    let result = time / -2.0;
    assert_eq!(result.to_seconds(), -3.0);
}

/// 测试时间与浮点数的乘法赋值运算
#[test]
fn test_mul_assign_f64() {
    let mut time = Time::from_seconds(2.0);
    time *= 3.0;
    assert_eq!(time.to_seconds(), 6.0);
}

/// 测试时间与浮点数的除法赋值运算
#[test]
fn test_div_assign_f64() {
    let mut time = Time::from_seconds(6.0);
    time /= 3.0;
    assert_eq!(time.to_seconds(), 2.0);
}

/// 测试从i64类型转换为Time类型
#[test]
fn test_from_i64() {
    let time: Time = 1000.into();
    assert_eq!(time.to_milliseconds(), 1000);
}

/// 测试从Time类型转换为f64类型
#[test]
fn test_into_f64() {
    let time = Time::from_seconds(2.5);
    let seconds: f64 = time.into();
    assert_eq!(seconds, 2.5);
}

/// 测试从f64类型转换为Time类型
#[test]
fn test_from_f64() {
    let time: Time = 2.5.into();
    assert_eq!(time.to_seconds(), 2.5);
}

/// 测试从Time类型转换为i64类型
#[test]
fn test_into_i64() {
    let time = Time::from_milliseconds(12345);
    let milliseconds: i64 = time.into();
    assert_eq!(milliseconds, 12345);
}

/// 测试Time类型的默认值
#[test]
fn test_default() {
    let time = Time::default();
    assert_eq!(time.to_milliseconds(), 0);
}

/// 测试Time类型的相等性比较
#[test]
fn test_equality() {
    let time1 = Time::from_seconds(1.0);
    let time2 = Time::from_seconds(1.0);
    let time3 = Time::from_seconds(2.0);
    assert_eq!(time1, time2);
    assert_ne!(time1, time3);
}

/// 测试Time类型的排序比较
#[test]
fn test_ordering() {
    let time1 = Time::from_seconds(1.0);
    let time2 = Time::from_seconds(2.0);
    let time3 = Time::from_seconds(2.0);
    assert!(time1 < time2);
    assert!(time2 > time1);
    assert!(time2 <= time3);
    assert!(time2 >= time3);
}

/// 测试毫秒值的往返转换
#[test]
fn test_roundtrip_milliseconds() {
    let original = 12345;
    let time = Time::from_milliseconds(original);
    let result = time.to_milliseconds();
    assert_eq!(original, result);
}

/// 测试秒值的往返转换
#[test]
fn test_roundtrip_seconds() {
    let original = 1.234;
    let time = Time::from_seconds(original);
    let result = time.to_seconds();
    assert!((original - result).abs() < 0.001);
}

/// 测试分钟值的往返转换
#[test]
fn test_roundtrip_minutes() {
    let original = 1.5;
    let time = Time::from_minutes(original);
    let result = time.to_minutes();
    assert!((original - result).abs() < 0.001);
}

/// 测试小时值的往返转换
#[test]
fn test_roundtrip_hours() {
    let original = 2.5;
    let time = Time::from_hours(original);
    let result = time.to_hours();
    assert!((original - result).abs() < 0.001);
}

/// 测试复杂的时间计算
#[test]
fn test_complex_calculation() {
    let time1 = Time::from_hours(1.5);
    let time2 = Time::from_minutes(30.0);
    let time3 = Time::from_seconds(15.0);
    let total = time1 + time2 + time3;
    assert_eq!(total.to_seconds(), 5400.0 + 1800.0 + 15.0);
}

/// 测试链式时间运算
#[test]
fn test_chained_operations() {
    let time = Time::from_seconds(10.0);
    let result = (time * 2.0) / 5.0 + Time::from_seconds(1.0);
    assert_eq!(result.to_seconds(), 5.0);
}
