use anyhow::{Context, Result};
use requestty::Question;

use crate::{
    cores::proccessing::append_line,
    utils::{
        banner::BANNER_SS, display_interface::print_lines, game::clear_screen,
        structer::ShadowServices, user_files::SS, MENU_SHADOW,
    },
};

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
            Ok(ss_main()?)
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
        _ => unreachable!(),
    }

    Ok(())
}

pub fn ss_add() -> Result<()> {
    let answer = requestty::Answers::default();
    let adoi = requestty::PromptModule::new(vec![
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

    let ss_details = adoi.prompt_all()?;

    let username = ss_details.get("user").with_context(|| {
        format!(
            r#"Error: can't get "user" from prompt - {}:{}"#,
            file!(),
            line!()
        )
    })?;

    let username = username.as_string().with_context(|| {
        format!(
            r#"Error: failed while convert username to string - {}:{}"#,
            file!(),
            line!()
        )
    })?;

    let date = ss_details.get("date").with_context(|| {
        format!(
            r#"Error: can't get "date" from prompt - {}:{}"#,
            file!(),
            line!()
        )
    })?;
    let total_days = date.as_int().with_context(|| {
        format!(
            r#"Error: failed while convert date as integer - {}:{}"#,
            file!(),
            line!()
        )
    })?;

    let date = crate::cores::calculate::add_user_date(total_days);

    append_line(SS, format!("#USER_SS {} {}\n", username, date))?;

    print_lines(username.len());
    println!("Password   : {}", username);
    println!("Date       : {}", date);
    println!("Total Days : {}", total_days);
    print_lines(username.len());

    Ok(())
}
