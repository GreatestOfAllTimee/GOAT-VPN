use crate::utils::banner::BANNER_MAIN;
use crate::utils::display_interface::print_lines;
use crate::utils::services::ovpn::ovpn_call_prompt;
use crate::utils::services::ss::ss_call_prompt;
use crate::utils::MENU;
use anyhow::Context;
use requestty::{Question, Result as ReqResult};

use super::structer::MainMenu;

pub fn main_prompt() -> anyhow::Result<MainMenu> {
    let answer = requestty::prompt_one(
        Question::raw_select("user_ssh")
            .message("Select Services")
            .choices(MENU.to_vec())
            .default_separator()
            .choices(vec!["Back to Main Menu", "Exit"])
            .build(),
    )?;

    match answer
        .as_list_item()
        .context({
            format!(
                "Error: failed to extract as_list_item {}:{}",
                file!(),
                line!()
            )
        })?
        .index
    {
        0 => Ok(MainMenu::Openvpn),
        1 => Ok(MainMenu::Shadowsocks),
        2 => Ok(MainMenu::ShadowsocksR),
        3 => Ok(MainMenu::V2ray),
        4 => Ok(MainMenu::Trojan),
        6 => {
            clear_screen()?;
            Ok(main_prompt()?)
        }
        7 => Err(anyhow::anyhow!("Exit")),
        _ => unreachable!(),
    }
}

pub fn clear_screen() -> anyhow::Result<()> {
    #[cfg(not(target_os = "windows"))]
    std::process::Command::new("clear")
        .status()
        .context(format!(
            r#"Error: "clear" console failed {}:{}"#,
            file!(),
            line!()
        ))?;

    Ok(())
}

pub fn call_prompt() {
    if let Err(e) = exit_prompt() {
        println!("{}", e);
    }
}

pub fn exit_prompt() -> anyhow::Result<()> {
    println!("{}", BANNER_MAIN);

    match main_prompt()? {
        MainMenu::Openvpn => ovpn_call_prompt(),
        MainMenu::Shadowsocks => ss_call_prompt(),
        MainMenu::ShadowsocksR => ssr_prompt()?,
        MainMenu::V2ray => v2ray_prompt("V2ray")?,
        MainMenu::Trojan => v2ray_prompt("Trojan")?,
    }

    Ok(())
}

pub fn v2ray_prompt(service: &str) -> ReqResult<()> {
    let answer = requestty::Answers::default();
    let adoi = requestty::PromptModule::new(vec![
        Question::input("user")
            .message(format!("Enter Username for {}", service))
            .validate(|n, _| {
                if n.is_empty() || n.len() < 3 {
                    Err("username cannot be empty or must be greater than 3.".to_owned())
                } else {
                    Ok(())
                }
            })
            .build(),
        Question::password("password")
            .message("Enter Password")
            .mask('*')
            .validate(|p, _| {
                if p.is_empty() || p.len() < 4 {
                    Err("password cannot be empty or must be greater than 4.".to_owned())
                } else {
                    Ok(())
                }
            })
            .build(),
        Question::int("date")
            .message("Total Date (days)")
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

    let v2ray = adoi.prompt_all()?;
    let username = v2ray.get("user").context("add user").unwrap();
    let username = username.as_string().unwrap();

    let password = v2ray.get("password").context("password").unwrap();
    let password = password.as_string().unwrap();

    let date = v2ray.get("date").context("date").unwrap();
    let total_days = date.as_int().unwrap();
    let date = crate::cores::calculate::add_user_date(total_days);

    print_lines(username.len());
    println!("Username   : {}", username);
    println!("Password   : {}", password);
    println!("Date       : {}", date);
    println!("Total Days : {}", total_days);
    print_lines(username.len());

    Ok(())
}

pub fn ssr_prompt() -> ReqResult<()> {
    let answer = requestty::Answers::default();
    let adoi = requestty::PromptModule::new(vec![
        Question::input("user")
            .message("Enter Username for ShadowsocksR")
            .validate(|n, _| {
                if n.is_empty() || n.len() < 3 {
                    Err("username cannot be empty or must be greater than 3.".to_owned())
                } else {
                    Ok(())
                }
            })
            .build(),
        Question::password("password")
            .message("Enter Password")
            .mask('*')
            .validate(|p, _| {
                if p.is_empty() || p.len() < 4 {
                    Err("password cannot be empty or must be greater than 4.".to_owned())
                } else {
                    Ok(())
                }
            })
            .build(),
        Question::int("date")
            .message("Total Date (days)")
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

    let ovpn_ssh = adoi.prompt_all()?;
    let username = ovpn_ssh.get("user").context("add user").unwrap();
    let username = username.as_string().unwrap();

    let password = ovpn_ssh.get("password").context("password").unwrap();
    let password = password.as_string().unwrap();

    let date = ovpn_ssh.get("date").context("date").unwrap();
    let total_days = date.as_int().unwrap();
    let date = crate::cores::calculate::add_user_date(total_days);

    print_lines(username.len());
    println!("Username   : {}", username);
    println!("Password   : {}", password);
    println!("Date       : {}", date);
    println!("Total Days : {}", total_days);
    print_lines(username.len());

    Ok(())
}

pub fn confirm_back() -> bool {
    requestty::prompt_one(
        Question::confirm("back_to_previous")
            .message("Do you want to continue?")
            .default(true)
            .build(),
    )
    .ok()
    .and_then(|r| r.as_bool())
    .unwrap_or(false)
}
