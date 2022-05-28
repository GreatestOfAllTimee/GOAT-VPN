use crate::{
    cores,
    utils::error_msg::{EMPTY, NAME_LENGTH},
};
use anyhow::{Context, Result};
use requestty::{Answer, Question};

#[allow(dead_code)]
pub fn vec_prompt(list: Vec<String>, index: usize) -> Result<String> {
    let question = Question::raw_select("React Select")
        .message("Enter your favorite author")
        .choices(list)
        .build();

    let authors = requestty::prompt_one(question)?;
    let authors = authors.try_into_list_item().unwrap();
    let authors = authors.text.as_str();

    Ok(authors.split_whitespace().collect::<Vec<&str>>()[..index].join(" "))
}

/// ### EXAMPLE
/// ```
/// let maria = user_prompt("Enter your name", 3, "FILE_SSH_OPENVPN.txt").unwrap();
/// println!("{}", maria);
/// ```
pub fn user_prompt(msg: &str, length: usize, user_file: &str) -> Result<String> {
    let question = Question::input("input")
        .message(msg.to_string())
        .validate(|s, _| match s {
            s if s.is_empty() => Err(EMPTY.to_owned()),
            s if s.len() < length => Err(NAME_LENGTH.to_owned()),
            s if cores::proccessing::get_test(s, user_file).unwrap() => {
                Err(format!("{} already exists", s))
            }
            _ => Ok(()),
        })
        .build();

    let answers: Answer = requestty::prompt_one(question)?;

    Ok(answers
        .as_string()
        .unwrap_or(&random_string(4, false))
        .replace(&['\"', '\'', '.', ' '][..], "-"))
}

pub fn random_string(total: usize, username_or_password: bool) -> String {
    // if username_or_password true then it will be a username otherwise it will be a password
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    let random = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(total)
        .map(char::from)
        .collect();

    if username_or_password {
        format!("GOAT-{}", random)
    } else {
        random
    }
}

/// ### EXAMPLE
/// ```
/// let test_ssh = user_prompt_index("Select a services", MENU_OVPN_SSH).unwrap();
/// let test_ssh = test_ssh.as_list_item().context("Error").unwrap();
/// println!("{}", test_ssh.index);
/// ```
pub fn user_prompt_index<T>(msg: &str, raw: Vec<T>) -> Result<Answer>
where
    T: Into<String> + std::fmt::Debug,
{
    let question = Question::raw_select("prompt_two")
        .message(msg)
        .choices(raw)
        .build();

    requestty::prompt_one(question).context(format!(
        r#"ERROR: while run "user_prompt_index" {}:{}"#,
        file!(),
        line!()
    ))
}

/// ### EXAMPLE
/// ```
/// let user_password = password_prompt("Confirm your password").unwrap();
/// let user_password = user_password.as_string().context("Error").unwrap();
/// println!("{}", user_password);
/// ```
pub fn password_prompt(msg: &str) -> Result<String> {
    let question = Question::password("password")
        .message(msg)
        .mask('*')
        .validate(|s, _| {
            if s.is_empty() || s.len() < 4 {
                Err("Password cannot be empty or password to short".to_owned())
            } else {
                Ok(())
            }
        })
        .build();

    let password = requestty::prompt_one(question)?;

    Ok(password
        .as_string()
        .unwrap_or(&random_string(5, true))
        .replace(&['\"', '\'', '.', ' '][..], "-"))
}

/// ### EXAMPLE
/// ```
/// let user_date = ask_user_date("Enter your date").unwrap();
/// let date = user.as_int().context("Error").unwrap();
/// println!("{}", date);
/// ```
pub fn ask_user_date(msg: &str) -> Result<Answer> {
    let question = Question::int("date")
        .message(msg)
        .default(1)
        .validate_on_key(|d, _| d > 0)
        .validate(|d, _| {
            if d <= 0 {
                Err("Date cannot be 0 or less".to_owned())
            } else {
                Ok(())
            }
        })
        .build();

    requestty::prompt_one(question).with_context(|| "ask_user_date")
}
