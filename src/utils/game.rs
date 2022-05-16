use crate::utils::banner::{BANNER_MAIN, BANNER_OVPN};
use crate::utils::display_interface::print_lines;
use crate::utils::services::ovpn::ovpn_call_prompt;
use crate::utils::services::ss::ss_call_prompt;
#[allow(unused_imports)]
use crate::utils::{MENU, MENU_OVPN_SSH};
use anyhow::Context;
use requestty::{Question, Result as ReqResult};

use super::structer::UserSSH;

pub fn main_prompt() -> anyhow::Result<UserSSH> {
    let answer = requestty::prompt_one(
        Question::raw_select("user_ssh")
            .message("Select Services")
            .choices(MENU.to_vec())
            .default_separator()
            .choices(vec!["Back to Main Menu", "Exit"])
            .build(),
    )?;

    match answer.as_list_item().unwrap().index {
        0 => Ok(UserSSH::Openvpn),
        1 => Ok(UserSSH::Shadowsocks),
        2 => Ok(UserSSH::ShadowsocksR),
        3 => Ok(UserSSH::V2ray),
        4 => Ok(UserSSH::Trojan),
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
        .context(format!("Clear console failed {}:{}", file!(), line!()))?;

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
        UserSSH::Openvpn => ovpn_call_prompt(),
        UserSSH::Shadowsocks => ss_call_prompt(),
        UserSSH::ShadowsocksR => ssr_prompt()?,
        UserSSH::V2ray => v2ray_prompt("V2ray")?,
        UserSSH::Trojan => v2ray_prompt("Trojan")?,
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

pub fn ss_prompt() -> ReqResult<()> {
    let answer = requestty::Answers::default();
    let adoi = requestty::PromptModule::new(vec![
        Question::input("user")
            .message("Enter Username for Shadowsocks")
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

pub fn ovpn_prompt() -> ReqResult<()> {
    clear_screen().unwrap();
    println!("{}", BANNER_OVPN);

    let answer = requestty::Answers::default();
    let adoi = requestty::PromptModule::new(vec![
        Question::input("user")
            .message("Enter Username for OpenVPN")
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
            .build(),
        Question::int("date")
            .message("Enter Date (days)")
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
    println!("Username: {}", username);
    println!("Password: {}", password);
    println!("Date: {}", date);
    println!("Total Days: {}", total_days);
    print_lines(username.len());

    Ok(())
}
