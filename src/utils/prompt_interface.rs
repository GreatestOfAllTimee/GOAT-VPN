use crate::utils::msg::{EMPTY, NAME_LENGTH};
use anyhow::{Context, Result};
use requestty::{Answer, Question};

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

pub fn user_prompt(msg: &str, length: usize) -> Result<String> {
    let question = Question::input("input")
        .message(msg.to_string())
        .validate(|s, _| {
            if s.is_empty() {
                Err(EMPTY.to_owned())
            } else if s.len() < length {
                Err(NAME_LENGTH.to_owned())
            } else {
                Ok(())
            }
        })
        .build();

    let answers: Answer = requestty::prompt_one(question)?;

    Ok(answers
        .as_string()
        .unwrap_or("")
        .replace(&['\"', '\'', '.', ' '][..], "-"))
}
