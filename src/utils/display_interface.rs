use anyhow::Context;
// use crate::ask_user_date;
use crate::utils::MENU;
use requestty::Question;

pub fn prompt_ssh_ovpn() -> anyhow::Result<()> {
    let answer = requestty::Answers::default();

    let questionhuh = requestty::PromptModule::new(vec![
        Question::raw_select("menu")
            .choices(MENU.to_vec())
            .message("select an option")
            .build(),
        Question::input("user")
            .message("Enter username")
            .validate(|n, _| {
                if n.is_empty() {
                    Err("username cannot be empty".to_owned())
                } else {
                    Ok(())
                }
            })
            .build(),
        Question::password("password")
            .message("enter your password")
            .mask('*')
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

    let final_result = questionhuh.prompt_all()?;

    let services_name = final_result.get("menu").context("failed to get menu")?;
    let services_name = services_name.as_list_item().context("failed to get menu")?;

    let username = final_result.get("user").context("failed to get username")?;
    let username = username.as_string().context("failed to get username")?;

    let password = final_result
        .get("password")
        .context("failed to get username")?;
    let password = password.as_string().context("failed to get username")?;

    let date = final_result.get("date").context("failed to get date")?;
    let date = date.as_int().context("failed to get date")?;

    println!(
        "Service Name: {}\nUsername: {}\nPassword: {}\nDate: {}",
        services_name.text, username, password, date
    );

    Ok(())
}

pub fn print_lines(username: usize) {
    if (username + 13) > 23 {
        println!("{sep:━>width$}", sep = "━", width = username + 15);
    } else {
        println!("{sep:━>width$}", sep = "━", width = 25);
    }
}

/// # Example
/// ```
/// const MYIP: &str = get_public_ip().unwrap();
/// println!("{}", MYIP);
/// ```
pub fn get_public_ip() -> anyhow::Result<String> {
    let urls = vec![
        "https://ifconfig.me",
        "http://ipinfo.io/ip",
        "https://checkip.amazonaws.com",
    ];
    let mut ip: String = String::new();
    let client = reqwest::blocking::Client::new();

    for i in urls {
        let res = client.get(i).send();
        if res.is_ok() {
            let res = res?;
            if res.status().is_success() {
                ip = res.text()?.replace(&['\n', ' '], "");
                break;
            }
        }
    }

    Ok(ip)
}
