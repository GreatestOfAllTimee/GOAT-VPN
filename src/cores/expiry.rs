use crate::{
    cores::{
        calculate::calculate_date,
        proccessing::{delete_line_regex, get_user_index, lets_read},
    },
    utils::user_files::*,
};
use anyhow::Result;
use chrono::NaiveDate;

// use super::proccessing::find_words_string;

#[allow(dead_code)]
pub fn clear_user_data(file: &str, user_name: &str, user_date: NaiveDate) -> Result<()> {
    let find_regex =
        regex::Regex::new(format!(r"(?s)#([A-Z]+)(?m) {} {}+\n", user_name, user_date).as_str())?;

    delete_line_regex(file, find_regex)?;
    Ok(())
}

#[allow(dead_code)]
pub fn clear_user_data_two(file: &str, username_date: &str) -> Result<()> {
    let find_regex =
        regex::Regex::new(format!(r"(?s)#([A-Z]+)(?m) {}+\n", username_date).as_str())?;

    delete_line_regex(file, find_regex)?;
    Ok(())
}

#[allow(dead_code)]
pub fn auto_run() -> Result<()> {
    let files: Vec<&str> = vec![SSH_OVPN, SS, SSR, V2RAY, TROJAN];

    for file in files.iter() {
        let mut reader = lets_read::Reader::open(file)?;
        let mut buffer = String::new();

        while let Some(line) = reader.read_line(&mut buffer) {
            let line = line?.trim();

            if !line.is_empty() {
                let user_date = get_user_index(line, 2);
                let user_date = NaiveDate::parse_from_str(&user_date, "%Y-%m-%d")?;
                if !calculate_date(user_date) {
                    let user = get_user_index(line, 1);
                    clear_user_data(file, &user, user_date)?;
                }
            }
        }
    }

    Ok(())
}

#[allow(dead_code)]
pub fn manual_run(file: &str, user: &str, only: bool) -> Result<()> {
    // if only is true then that means user variable is only have username
    // if only is false then that means user variable is username and date
    let mut reader = lets_read::Reader::open(file)?;
    let mut buffer = String::new();

    while let Some(line) = reader.read_line(&mut buffer) {
        let line = line?.trim();
        if !line.is_empty() && line.contains(user) {
            if only {
                let date = get_user_index(line, 2);
                let user_date = NaiveDate::parse_from_str(date.as_str(), "%Y-%m-%d")?;
                clear_user_data(file, user, user_date)?;
            } else {
                clear_user_data_two(file, user)?;
            }
        }
    }

    Ok(())
}