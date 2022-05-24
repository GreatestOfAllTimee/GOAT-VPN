use crate::utils::{
    banner::BANNER_SS,
    display_interface::print_lines,
    game::{call_prompt, clear_screen},
    structer::ShadowServices,
    user_files::SS,
    MENU_SHADOW,
};
use crate::{
    cores::{
        proccessing::{append_line, display_user_data},
        types_error::{display_error, display_error_convert},
    },
    utils::prompt_interface::user_prompt_index,
};
use anyhow::{Context, Result};
use colored::*;
use requestty::Question;

pub fn ss_main() -> Result<ShadowServices> {
    let answer = requestty::prompt_one(
        Question::raw_select("user_ss")
            .message("Select Services")
            .choices(MENU_SHADOW.to_vec())
            .default_separator()
            .choices(vec!["Back to Main Menu", "Exit"])
            .build(),
    )?;

    match answer.as_list_item().unwrap().index {
        0 => Ok(ShadowServices::Add),
        1 => Ok(ShadowServices::Delete),
        2 => Ok(ShadowServices::Renew),
        3 => Ok(ShadowServices::List),
        5 => {
            clear_screen()?;
            Ok(ShadowServices::Exit)
        }
        6 => return Err(anyhow::anyhow!("Exit")),
        _ => unreachable!(),
    }
}

pub fn ss_call_prompt() {
    if let Err(e) = ss_exit() {
        println!("{}", e);
    }
}

pub fn ss_exit() -> anyhow::Result<()> {
    clear_screen()?;
    println!("{}", BANNER_SS);

    match ss_main()? {
        ShadowServices::Add => ss_add()?,
        ShadowServices::Exit => call_prompt(),
        _ => unreachable!(),
    }

    Ok(())
}

pub fn ss_add() -> Result<()> {
    let answer = requestty::Answers::default();
    let ask = requestty::PromptModule::new(vec![
        Question::input("user")
            .message("Enter username for OpenVPN")
            .validate(|n, _| {
                if n.is_empty() || n.len() < 3 {
                    Err("username cannot be empty or must be greater than 3.".to_owned())
                } else {
                    Ok(())
                }
            })
            .build(),
        Question::int("date")
            .message("Enter a date")
            .default(1)
            .validate_on_key(|d, _| d > 0)
            .validate(|d, _| {
                if d <= 0 {
                    Err("Date cannot be 0 or less".to_owned())
                } else {
                    Ok(())
                }
            })
            .build(),
    ])
    .with_answers(answer);

    let ss_details = ask.prompt_all()?;

    let username = ss_details
        .get("user")
        .with_context(|| display_error("username", file!(), line!()))?;

    let username = username
        .as_string()
        .with_context(|| display_error_convert("username", "String", file!(), line!()))?;

    let date = ss_details
        .get("date")
        .with_context(|| display_error("date", file!(), line!()))?;

    let total_days = date
        .as_int()
        .with_context(|| display_error_convert("date", "Int", file!(), line!()))?;

    let date = crate::cores::calculate::add_user_date(total_days);
    append_line(SS, format!("#USER_SS {} {}\n", username, date))?;

    print_lines(username.len());
    println!("{}: {}", "Password".bold(), username);
    println!("{}       : {}", "Date".bold(), date);
    println!("{} : {}", "Total Days".bold(), total_days);
    print_lines(username.len());

    Ok(())
}

fn delete_ss_user() -> Result<()> {
    let details = display_user_data(SS)?;
    let display = user_prompt_index("Select user", details)?;
    let display = display.as_list_item().context("Invalid user")?;

    let user = crate::cores::proccessing::get_user_index(&display.text, 0);
    let privs = crate::utils::structer::privileges();
    let userdel = subprocess::Exec::shell(format!("{} userdel {}", privs, user)).join()?;

    if !userdel.success() {
        return Err(anyhow::anyhow!("{}", "Failed to delete user".red().bold()));
    }

    crate::cores::expiry::manual_run(SS, &display.text, false)?;

    Ok(())
}
