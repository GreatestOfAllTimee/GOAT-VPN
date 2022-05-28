use anyhow::Context;

/// TODO: using UserData instead of User
pub struct User {
    pub name: String,
    pub password: String,
    pub date: chrono::NaiveDate,
}

pub struct UserData<'a> {
    pub name: &'a str,
    pub password: &'a str,
    pub total_days: i64,
    pub date: chrono::NaiveDate,
}

impl<'a> UserData<'a> {
    pub fn new(name: &'a str, password: &'a str, total_days: i64, date: chrono::NaiveDate) -> Self {
        Self {
            name,
            password,
            total_days,
            date,
        }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let privs = crate::utils::structer::privileges();

        let useradd = subprocess::Exec::shell(format!(
            "{} useradd -M -N -s /bin/false -e {} {}",
            privs, self.date, self.name
        ))
        .join()
        .context(format!(
            r#"ERROR: failed to add user "run" {}:{}"#,
            file!(),
            line!()
        ))?;

        let user_pass = subprocess::Exec::shell(format!(
            "echo \"{password}\n{password}\n\" | {} passwd {} 2>/dev/null",
            privs,
            self.name,
            password = self.password,
        ))
        .join()
        .context(format!(
            r#"ERROR: failed to add "{}" password "run" {}:{}"#,
            self.name,
            file!(),
            line!()
        ))?;

        if useradd.success() && user_pass.success() {
            self.print_user_details();
        } else {
            return {
                Err(anyhow::anyhow!(format!(
                    r#"ERROR: failed to add user "{}" password "{}" {}:{}"#,
                    self.name,
                    self.password,
                    file!(),
                    line!()
                )))
            };
        }

        Ok(())
    }

    pub fn print_user_details(&self) {
        use colored::Colorize;

        println!("{}   : {}", "username".bold(), self.name.green());
        println!("{}   : {}", "Password".bold(), self.password.green());
        println!("{}       : {}", "Date".bold(), self.date);
        println!("{} : {}", "Total Days".bold(), self.total_days);
    }
}

pub enum MainMenu {
    Openvpn,
    Shadowsocks,
    ShadowsocksR,
    V2ray,
    Trojan,
}

pub enum OvpnServices {
    Add,
    Delete,
    Renew,
    List,
    Back,
    Exit,
}

pub enum ShadowServices {
    Add,
    Delete,
    Renew,
    List,
    Back,
    Exit,
}

pub enum V2rayServices {
    Add,
    Delete,
    Renew,
    List,
    Back,
    Exit,
}

pub fn privileges() -> &'static str {
    if std::env::var("USER").unwrap() != "root" {
        return "sudo";
    }

    ""
}

pub mod lets_read {
    use std::{
        fs::File,
        io::{self, prelude::*},
    };

    pub struct Reader {
        reader: io::BufReader<File>,
    }

    impl Reader {
        pub fn open(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);

            Ok(Self { reader })
        }

        pub fn read_line<'buf>(
            &mut self,
            buffer: &'buf mut String,
        ) -> Option<io::Result<&'buf mut String>> {
            buffer.clear();

            self.reader
                .read_line(buffer)
                .map(|u| if u == 0 { None } else { Some(buffer) })
                .transpose()
        }
    }
}
