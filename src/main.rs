// #![allow(unused_variables, unused_imports)]
mod cores;
mod utils;

use anyhow::{anyhow, Context, Result};
use cores::calculate::add_user_date;
use cores::proccessing::{append_line, display_user_data};

#[allow(unused_imports)]
use utils::game::call_prompt;

#[allow(unused_imports)]
use utils::{
    display_interface::prompt_ssh_ovpn,
    prompt_interface::{ask_user_date, password_prompt, user_prompt, user_prompt_index},
    structer::User,
    user_files::SSH_OVPN,
};

/// TODO: remove all of this comments after testing
fn main() -> Result<()> {
    // create_user()?;
    // auto_run()?;
    // delete_user()?;
    // prompt_ssh_ovpn()?;
    // call_prompt();
    println!("{}", utils::display_interface::get_public_ip()?);

    // append_json("v2ray.json", data)?;
    // remove_json_value("v2ray.json", email)?;

    // let new_data: serde_json::Value = "Zulaikha".into();
    // append_json_trojan("/etc/v2ray/config.json", new_data)?;
    // remove_json_trojan("data/json/trojan.json", "Zulaikha")?;
    // delete_ovpn_user();

    Ok(())
}

#[allow(dead_code)]
fn delete_user() -> Result<()> {
    let details = display_user_data(SSH_OVPN)?;
    let display = user_prompt_index("Select user", details)?;
    let display = display.as_list_item().context("Invalid user")?;

    cores::expiry::manual_run(SSH_OVPN, &display.text, false)?;
    Ok(())
}

#[allow(dead_code)]
fn create_user() -> Result<()> {
    let user_name = user_prompt("Enter username", 3, SSH_OVPN)?;
    let user_password = password_prompt("Confirm your password")?;
    let user_password_confirm = password_prompt("Confirm your password")?;

    if user_password != user_password_confirm {
        return Err(anyhow!("Password and confirm password does not match"));
    }

    let ask_date = ask_user_date("Total Days To Exp (days):")?;
    let ask_date = ask_date.as_int().context("failed to get date")?;
    let date = add_user_date(ask_date);

    let user_data = User {
        name: user_name,
        password: user_password,
        date,
    };

    append_line(
        SSH_OVPN,
        format!("#USER {} {}\n", user_data.name, user_data.date),
    )?;

    Ok(())
}
