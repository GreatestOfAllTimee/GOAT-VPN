pub struct User {
    pub name: String,
    pub password: String,
    pub date: chrono::NaiveDate,
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
