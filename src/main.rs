// #![allow(unused_variables, unused_imports)]
mod cores;
mod utils;

use anyhow::{anyhow, Context, Result};
use cores::calculate::add_user_date;
use cores::proccessing::{append_line, display_user_data};

#[allow(unused_imports)]
use utils::game::call_prompt;

use utils::services::ovpn::ovpn_test;
use utils::structer::UserData;
#[allow(unused_imports)]
use utils::{
    display_interface::prompt_ssh_ovpn,
    prompt_interface::{ask_user_date, password_prompt, user_prompt, user_prompt_index},
    user_files::SSH_OVPN,
};

#[allow(unused_doc_comments)]
/// TODO: remove all of this comments after testing
fn main() -> Result<()> {
    ovpn_test()?;
    // call_prompt();
    // create_user()?;
    // auto_run()?;
    // delete_user()?;
    // prompt_ssh_ovpn()?;

    // println!("{}", utils::display_interface::get_public_ip()?);

    /// V2RAY : append & remove json values
    // append_json("v2ray.json", data)?;
    // remove_json_value("v2ray.json", email)?;

    /// TROJAN-VPN : append & remove json values
    // let new_data: serde_json::Value = "John-Smith".into();
    // cores::proccessing::append_json_trojan("trojan.json", new_data)?;
    // cores::proccessing::remove_json_trojan("trojan.json", "John-Smith")?;
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

    let user_data = UserData::new(&user_name, &user_password, ask_date, date);

    append_line(
        SSH_OVPN,
        format!("#USER {} {}\n", user_data.name, user_data.date),
    )?;

    Ok(())
}
