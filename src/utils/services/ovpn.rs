use crate::utils::display_interface::print_lines;
use crate::utils::game::clear_screen;
use crate::utils::structer::OvpnServices;
use crate::utils::{banner::BANNER_OVPN, MENU_OVPN_SSH};
use anyhow::{Context, Result};
use requestty::{Question, Result as ReqResult};

pub fn ovpn_main() -> Result<OvpnServices> {
    let answer = requestty::prompt_one(
        Question::raw_select("user_ssh")
            .message("Select Services")
            .choices(MENU_OVPN_SSH.to_vec())
            .default_separator()
            .choices(vec!["Back to Main Menu", "Exit"])
            .build(),
    )?;

    match answer.as_list_item().unwrap().index {
        0 => Ok(OvpnServices::Add),
        1 => Ok(OvpnServices::Delete),
        2 => Ok(OvpnServices::Renew),
        3 => Ok(OvpnServices::List),
        5 => {
            clear_screen()?;
            Ok(ovpn_main()?)
        }
        6 => return Err(anyhow::anyhow!("Exit")),
        _ => unreachable!(),
    }
}

pub fn ovpn_call_prompt() {
    if let Err(e) = ovpn_exit() {
        println!("{}", e);
    }
}

pub fn ovpn_exit() -> anyhow::Result<()> {
    clear_screen()?;
    println!("{}", BANNER_OVPN);

    match ovpn_main()? {
        OvpnServices::Add => ovpn_add()?,
        _ => unreachable!(),
    }

    Ok(())
}

pub fn ovpn_add() -> ReqResult<()> {
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
