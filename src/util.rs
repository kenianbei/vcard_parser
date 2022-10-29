use time::format_description::well_known::{Iso8601, Rfc2822, Rfc3339};
use time::{format_description, Date, OffsetDateTime, PrimitiveDateTime, UtcOffset};

/// Parse a date string.
///
/// # Examples
/// ```
/// use vcard_parser::util::parse_date;
///
/// let date = parse_date("2000-01-01").expect("Unable to parse date string.");
/// assert_eq!(date, (2000, 01, 01));
/// ```
pub fn parse_date(str: &str) -> Option<(i32, u8, u8)> {
    if let Ok(date) = Date::parse(str, &Rfc3339) {
        return Some((date.year(), date.month().into(), date.day()));
    }
    if let Ok(date) = Date::parse(str, &Rfc2822) {
        return Some((date.year(), date.month().into(), date.day()));
    }
    if let Ok(date) = Date::parse(str, &Iso8601::DEFAULT) {
        return Some((date.year(), date.month().into(), date.day()));
    }
    if let Ok(date) = Date::parse(str, &format_description::parse("[year][month][day]").unwrap()) {
        return Some((date.year(), date.month().into(), date.day()));
    }
    if let Ok(date) = Date::parse(str, &format_description::parse("[year]-[month]-[day]").unwrap()) {
        return Some((date.year(), date.month().into(), date.day()));
    }
    None
}

/// Parse a datetime string.
///
/// # Examples
/// ```
/// use vcard_parser::util::parse_time;
///
/// let date = parse_time("2000-01-01T00:00:00Z").expect("Unable to parse datetime string.");
/// assert_eq!(date, 946684800);
/// ```
pub fn parse_time(str: &str) -> Option<i32> {
    if let Ok(time) = OffsetDateTime::parse(str, &Rfc3339) {
        return Some(time.unix_timestamp() as i32);
    }
    if let Ok(time) = OffsetDateTime::parse(str, &Rfc2822) {
        return Some(time.unix_timestamp() as i32);
    }
    if let Ok(time) = OffsetDateTime::parse(str, &Iso8601::DEFAULT) {
        return Some(time.unix_timestamp() as i32);
    }
    if let Ok(datetime) = PrimitiveDateTime::parse(str, &format_description::parse("[year]-[month]-[day]T[hour]:[minute]:[second]").unwrap()) {
        return Some(datetime.assume_offset(UtcOffset::UTC).unix_timestamp() as i32);
    }
    if let Ok(datetime) = PrimitiveDateTime::parse(str, &format_description::parse("[year]-[month]-[day]T[hour]:[minute]:[second]Z").unwrap()) {
        return Some(datetime.assume_offset(UtcOffset::UTC).unix_timestamp() as i32);
    }
    if let Ok(datetime) = PrimitiveDateTime::parse(str, &format_description::parse("[year][month][day]T[hour][minute][second]").unwrap()) {
        return Some(datetime.assume_offset(UtcOffset::UTC).unix_timestamp() as i32);
    }
    if let Ok(datetime) = PrimitiveDateTime::parse(str, &format_description::parse("[year][month][day]T[hour][minute][second]Z").unwrap()) {
        return Some(datetime.assume_offset(UtcOffset::UTC).unix_timestamp() as i32);
    }
    None
}
