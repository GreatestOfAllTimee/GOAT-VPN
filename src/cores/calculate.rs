#![allow(dead_code)]
use chrono::{Datelike, Local, NaiveDate, TimeZone, Utc};

/// ### EXAMPLE
/// ```
/// let date = add_user_date(ask_date);
///
/// if !calculate_date(date) {
///     println!("Your account has expired");
///     return Ok(());
/// }
/// ```
pub fn calculate_date(user_date: NaiveDate) -> bool {
    let date = Local::now().date();
    let extract_date = Utc.ymd(user_date.year(), user_date.month(), user_date.day());
    let dt = extract_date.signed_duration_since(date);

    dt.num_days() >= 0
}

pub fn add_user_date(user_date: i64) -> NaiveDate {
    let date = Local::now() + chrono::Duration::days(user_date);

    date.naive_utc().date()
}
