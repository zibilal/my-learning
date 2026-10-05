use time::macros::datetime;
use time::PrimitiveDateTime as DateTime;

fn after(start: DateTime) -> DateTime {
    start + time::Duration::seconds(1_000_000_000)
}

#[test]
fn test_after() {
    let result = after(datetime!(2015-01-01 22:00:00));
    assert_eq!(result, datetime!(2046-09-09 23:46:40));
}