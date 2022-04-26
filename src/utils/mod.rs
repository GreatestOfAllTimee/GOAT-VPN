pub mod prompt_interface;

#[allow(dead_code)]
pub mod msg {
    pub const EMPTY: &str = "Name cannot be empty!";
    pub const SSH: &str = "SSH";
    pub const V2RAY: &str = "V2RAY";
    pub const TROJAN: &str = "TROJAN";
    pub const OPENVPN: &str = "OPENVPN";
    pub const OPENVPN_SSH: &str = "SSH & OPENVPN";
    pub const NAME_LENGTH: &str = "Name must be at least 3 characters long!";
}

#[allow(dead_code)]
pub mod services {
    pub const MENU_OVPN_SSH: [&str; 5] = [
        "Create SSH & OpenVPN Account",
        "Delete SSH & OpenVPN Account",
        "Renew SSH & OpenVPN Account",
        "Change User Password SSH & OpenVPN",
        "Generate Account SSH & OPENVPN",
    ];
}
