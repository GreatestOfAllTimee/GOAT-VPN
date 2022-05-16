pub struct User {
    pub name: String,
    pub password: String,
    pub date: chrono::NaiveDate,
}

pub enum UserSSH {
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
}

pub enum ShadowServices {
    Add,
    Delete,
    Renew,
    List,
}
