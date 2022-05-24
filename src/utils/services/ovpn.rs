use crate::cores::proccessing::{append_line, display_user_data, get_user_index};
use crate::cores::types_error::{display_error, display_error_convert};
use crate::utils::display_interface::print_lines;
use crate::utils::game::{call_prompt, clear_screen};
use crate::utils::prompt_interface::user_prompt_index;
use crate::utils::structer::OvpnServices;
use crate::utils::user_files::SSH_OVPN;
use crate::utils::{banner::BANNER_OVPN, MENU_OVPN_SSH};
use anyhow::{Context, Result};
use colored::*;
use requestty::Question;

fn ovpn_main() -> Result<OvpnServices> {
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
            Ok(OvpnServices::Exit)
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

fn ovpn_exit() -> anyhow::Result<()> {
    clear_screen()?;
    println!("{}", BANNER_OVPN);

    match ovpn_main()? {
        OvpnServices::Add => {
            ovpn_add()?;
            confirm_back_ovpn()?
        }
        OvpnServices::Delete if std::fs::metadata(SSH_OVPN)?.len() == 0 => {
            println!("{}", "No Data Found".yellow().bold());
            confirm_back_ovpn()?
        }
        OvpnServices::Delete => {
            if delete_ovpn_user().is_ok() {
                println!("{}", "Sucessfully Deleted".green().bold());
            } else {
                println!("{}", "User Not Found".red().bold());
            }

            confirm_back_ovpn()?
        }
        OvpnServices::Exit => call_prompt(),
        _ => unreachable!(),
    }

    Ok(())
}

fn ovpn_add() -> Result<()> {
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
        Question::password("password")
            .message("Enter Password")
            .mask('*')
            .validate(|p, _| {
                if p.is_empty() || p.len() < 3 {
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

    let ovpn_ssh = ask.prompt_all()?;
    let username = ovpn_ssh
        .get("user")
        .with_context(|| display_error("username", file!(), line!()))?;
    let username = username
        .as_string()
        .with_context(|| display_error_convert("username", "String", file!(), line!()))?;

    let password = ovpn_ssh
        .get("password")
        .with_context(|| display_error("password", file!(), line!()))?;
    let password = password
        .as_string()
        .with_context(|| display_error_convert("password", "String", file!(), line!()))?;

    let date = ovpn_ssh
        .get("date")
        .with_context(|| display_error("date", file!(), line!()))?;
    let total_days = date
        .as_int()
        .with_context(|| display_error_convert("date", "Int", file!(), line!()))?;

    let date = crate::cores::calculate::add_user_date(total_days);
    let privs = crate::utils::structer::privileges();
    let useradd = subprocess::Exec::shell(format!(
        "{} useradd -M -N -s /bin/false -e {} {}",
        privs, date, username
    ))
    .join()
    .context("Failed to add user")?;

    let user_pass = subprocess::Exec::shell(format!(
        "echo \"{password}\n{password}\n\" | {} passwd {username} 2>/dev/null",
        privs
    ))
    .join()?;

    if useradd.success() && user_pass.success() {
        append_line(SSH_OVPN, format!("#USER {username} {date}\n"))?;
        print_lines(username.len());
        println!("{}   : {}", "username".bold(), username.green());
        println!("{}   : {}", "Password".bold(), password.green());
        println!("{}       : {date}", "Date".bold());
        println!("{} : {total_days}", "Total Days".bold());
        print_lines(username.len());
    } else {
        println!("{}", "Failed to add user".red().bold());
    }

    Ok(())
}

fn delete_ovpn_user() -> Result<()> {
    let details = display_user_data(SSH_OVPN)?;
    let display = user_prompt_index("Select user", details)?;
    let display = display.as_list_item().context("Invalid user")?;

    let user = get_user_index(&display.text, 0);
    let privs = crate::utils::structer::privileges();
    let userdel = subprocess::Exec::shell(format!("{} userdel {}", privs, user)).join()?;

    if !userdel.success() {
        return Err(anyhow::anyhow!("{}", "Failed to delete user".red().bold()));
    }

    crate::cores::expiry::manual_run(SSH_OVPN, &display.text, false)?;

    Ok(())
}

fn confirm_back_ovpn() -> Result<()> {
    if !requestty::prompt_one(
        Question::confirm("back_to_previous")
            .message("Do you want to Continue?")
            .default(true)
            .build(),
    )
    .ok()
    .and_then(|r| r.as_bool())
    .unwrap_or(false)
    {
        println!("{}", "Exiting".yellow().bold());
        std::process::exit(0)
    }

    ovpn_exit()
}
