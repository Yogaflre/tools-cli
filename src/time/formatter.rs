use chrono::{DateTime, Local, TimeZone};

const NORMAL_DATETIME: &str = "%Y-%m-%d %H:%M:%S";
const STANDARD_DATETIME: &str = "%Y-%m-%dT%H:%M:%SZ";

pub fn now() -> String {
    return time_format(Local::now());
}

pub fn convert(time: &str) -> String {
    let datetime;
    if let Ok(number) = time.parse::<i64>() {
        datetime = Local.timestamp_millis_opt(number).unwrap(); // FIXME unwrap
    } else if let Ok(t) = Local.datetime_from_str(&time, STANDARD_DATETIME) {
        datetime = t.with_timezone(&Local);
    } else if let Ok(t) = Local.datetime_from_str(&time, NORMAL_DATETIME) {
        datetime = t;
    } else {
        panic!("Unknown time format: {}", time);
    }
    return time_format(datetime);
}

fn time_format(time: DateTime<Local>) -> String {
    return format!(
        "{}\n{}\n{}\n{}",
        time.timestamp(),
        time.timestamp_millis(),
        time.format(NORMAL_DATETIME),
        time.format(STANDARD_DATETIME),
    );
}
