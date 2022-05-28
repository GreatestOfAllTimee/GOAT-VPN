use anyhow::{Context, Result};
use colored::Colorize;
use requestty::Question;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    cores::{
        proccessing::{append_json, append_line},
        types_error::{display_error, display_error_convert},
    },
    utils::{
        banner::BANNER_V2RAY,
        display_interface::print_lines,
        game::{call_prompt, clear_screen},
        structer::V2rayServices,
        user_files::{V2RAY, V2RAY_CONFIG},
        MENU_V2RAY,
    },
};

fn v2ray_main() -> Result<V2rayServices> {
    let answer = requestty::prompt_one(
        Question::raw_select("user_v2ray")
            .message("Select Services")
            .choices(MENU_V2RAY.to_vec())
            .default_separator()
            .choices(vec!["Back to Main Menu", "Exit"])
            .build(),
    )?;

    match answer.as_list_item().unwrap().index {
        0 => Ok(V2rayServices::Add),
        1 => Ok(V2rayServices::Delete),
        2 => Ok(V2rayServices::Renew),
        3 => Ok(V2rayServices::List),
        5 => {
            clear_screen()?;
            Ok(V2rayServices::Exit)
        }
        6 => return Err(anyhow::anyhow!("Exit")),
        _ => unreachable!(),
    }
}

pub fn v2ray_call_prompt() {
    if let Err(e) = v2ray_exit() {
        println!("{}", e);
    }
}

pub fn v2ray_exit() -> Result<()> {
    clear_screen()?;
    println!("{}", BANNER_V2RAY);

    match v2ray_main()? {
        V2rayServices::Add => v2ray_add()?,
        V2rayServices::Delete => todo!(),
        V2rayServices::Renew => todo!(),
        V2rayServices::List => todo!(),
        V2rayServices::Exit => call_prompt(),
        _ => unreachable!(),
    }

    Ok(())
}

pub fn v2ray_add() -> Result<()> {
    let answer = requestty::Answers::default();
    let ask = requestty::PromptModule::new(vec![
        Question::input("user")
            .message("Enter password for V2ray")
            .validate(|n, _| {
                if n.is_empty() || n.len() < 3 {
                    Err("password cannot be empty or must be greater than 3.".to_owned())
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

    let v2ray_details = ask.prompt_all()?;

    let username = v2ray_details
        .get("user")
        .with_context(|| display_error("username", file!(), line!()))?;
    let username = username
        .as_string()
        .with_context(|| display_error_convert("username", "String", file!(), line!()))?;

    let date = v2ray_details
        .get("date")
        .with_context(|| display_error("date", file!(), line!()))?;
    let total_days = date
        .as_int()
        .with_context(|| display_error_convert("date", "Int", file!(), line!()))?;

    let date = crate::cores::calculate::add_user_date(total_days);

    let data: Value = json!({
        "id": Uuid::new_v4().to_string(),
        "alterId": 2,
        "email": format!("{}@v2ray.com", username)
    });

    append_json(V2RAY_CONFIG, data)?;
    append_line(V2RAY, format!("#USER_VMESS {} {}\n", username, date))?;

    print_lines(username.len());
    println!("{}       : {}", "Password".bold(), username);
    println!("{}       : {}", "Date".bold(), date);
    println!("{} : {}", "Total Days".bold(), total_days);
    print_lines(username.len());

    Ok(())
}
