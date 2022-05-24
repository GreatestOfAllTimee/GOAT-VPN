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

// return static string
pub fn privileges() -> &'static str {
    if std::env::var("USER").unwrap() != "root" {
        return "sudo";
    }

    ""
}
