#![allow(dead_code)]
pub mod display_interface;
pub mod game;
pub mod prompt_interface;
pub mod structer;
pub mod user_files;
pub mod services;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref MENU: Vec<&'static str> = vec![
        "SSH & OpenVPN Menu",
        "Shadowsocks Menu",
        "Shadowsocks-R Menu",
        "V2ray Menu",
        "Trojan Menu",
    ];

    pub static ref MENU_OVPN_SSH: Vec<&'static str> = vec![
        "Create SSH & OpenVPN Account",
        "Delete SSH & OpenVPN Account",
        "Renew SSH & OpenVPN Account",
        "Change User Password SSH & OpenVPN",
    ];

    pub static ref MENU_SHADOW: Vec<&'static str> = vec![
        "Create Shadowsocks Account",
        "Delete Shadowsocks Account",
        "Renew Shadowsocks Account",
        "List Shadowsocks All Active Account",
    ];

}

pub mod msg {
    pub const OPENVPN_SSH: &str = "SSH & OPENVPN";
}

pub mod error_msg {
    pub const EMPTY: &str = "Name cannot be empty!";
    pub const NAME_LENGTH: &str = "Name must be at least 3 characters long!";
}

pub mod banner {
    pub const BANNER_MAIN: &str = "
        █▀▀ ░ █▀█ ░ ▄▀█ ░ ▀█▀ ▄▄ █▀ ▀█▀ █▀█ █▀█ █▀▀
        █▄█ ▄ █▄█ ▄ █▀█ ▄ ░█░ ░░ ▄█ ░█░ █▄█ █▀▄ ██▄
    ";

    pub const BANNER_OVPN: &str = "
        █▀█ █▀█ █▀▀ █▄░█ █░█ █▀█ █▄░█     █▀ █▀ █░█
        █▄█ █▀▀ ██▄ █░▀█ ▀▄▀ █▀▀ █░▀█     ▄█ ▄█ █▀█
    ";

    pub const BANNER_V2RAY: &str = "
        ╭╮╱╱╭┳━━━┳━━━┳━━━┳╮╱╱╭╮
        ┃╰╮╭╯┃╭━╮┃╭━╮┃╭━╮┃╰╮╭╯┃
        ╰╮┃┃╭┻╯╭╯┃╰━╯┃┃╱┃┣╮╰╯╭╯
        ╱┃╰╯┃╭━╯╭┫╭╮╭┫╰━╯┃╰╮╭╯
        ╱╰╮╭╯┃┃╰━┫┃┃╰┫╭━╮┃╱┃┃
        ╱╱╰╯╱╰━━━┻╯╰━┻╯╱╰╯╱╰╯
    ";

    pub const BANNER_TROJAN: &str = "
        ╭━━━━┳━━━┳━━━╮╱╭┳━━━┳━╮╱╭╮
        ┃╭╮╭╮┃╭━╮┃╭━╮┃╱┃┃╭━╮┃┃╰╮┃┃
        ╰╯┃┃╰┫╰━╯┃┃╱┃┃╱┃┃┃╱┃┃╭╮╰╯┃
        ╱╱┃┃╱┃╭╮╭┫┃╱┃┣╮┃┃╰━╯┃┃╰╮┃┃
        ╱╱┃┃╱┃┃┃╰┫╰━╯┃╰╯┃╭━╮┃┃╱┃┃┃
        ╱╱╰╯╱╰╯╰━┻━━━┻━━┻╯╱╰┻╯╱╰━╯
    ";

    pub const BANNER_SS: &str = "
        █▀ █░█ ▄▀█ █▀▄ █▀█ █░█░█ █▀ █▀█ █▀▀ █▄▀ █▀
        ▄█ █▀█ █▀█ █▄▀ █▄█ ▀▄▀▄▀ ▄█ █▄█ █▄▄ █░█ ▄█
    ";

    pub const BANNER_SSR: &str = "
        █▀ █░█ ▄▀█ █▀▄ █▀█ █░█░█ █▀ █▀█ █▀▀ █▄▀ █▀ ▄▄ █▀█
        ▄█ █▀█ █▀█ █▄▀ █▄█ ▀▄▀▄▀ ▄█ █▄█ █▄▄ █░█ ▄█ ░░ █▀▄
    ";
}
