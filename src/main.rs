#![allow(unused_variables, unused_imports)]
// use ruplacer::{FilePatcher, Query};
mod core;
mod utils;
use crate::core::proccessing::{delete_line, delete_line_regex, find_words, read_file};
use crate::utils::prompt_interface::{user_prompt, vec_prompt};
use crate::utils::services::MENU_OVPN_SSH;

use anyhow::{Context, Result};
use regex::Regex;
use requestty::Question;
use std::path::PathBuf;

fn main() -> Result<()> {
    let file: &str = "input.txt";
    let maria = user_prompt("Enter your name", 3)?;
    let name = vec_prompt(read_file(file)?, 2)?;
    println!("{:?}", name);

    println!("{}", maria);
    println!("{}", name);

    let f = read_file(file)?;
    let found: bool = find_words(&f, "Zayne");

    if found {
        println!("Found");
    } else {
        println!("Not found");
    }

    let openvpn = Question::raw_select("service")
        .message("Select a service")
        .choices(MENU_OVPN_SSH);

    let openvpn = requestty::prompt_one(openvpn)?;

    // get openvpn value
    let openvpn = openvpn
        .as_list_item()
        .context("Error getting openvpn value")?;

    match openvpn.index {
        0 => println!("OpenVPN"),
        1 => println!("SSH"),
        2 => println!("V2ray"),
        3 => println!("Trojan"),
        4 => println!("Shadowsocks"),
        _ => println!("Error"),
    }

    // println!("index: {}\nvec_length: {}", openvpn.index, MENU_OVPN_SSH.len());

    /* second_name is a for take a second word in name variable */
    // let second_name = &name.split_whitespace().collect::<Vec<&str>>()[1];
    // let full_name = format!("{}\n", &name);
    // let pattern = Regex::new(r"#[A-Z] Zayne 16-06-2021")?;
    // let pattern = Regex::new(format!(r"(?i) {}", &full_name).as_str())?;
    // delete_line(file, &full_name)?;
    // delete_line_regex(file, pattern)?;

    // let test = Regex::new(format!(r"(?s)#([A-Z]+)(?m) {}+", &name).as_str())?;
    // remove test from the file including the line
    // let before = "#USER Zayne 16-06-2021";
    // replace before without new line

    // let test = Regex::new(format!(r"(?s)#([A-Z]+)(?m) {}+\n", &name).as_str())?;
    // delete_line_regex(file, test)?;

    // let find_i = core::proccessing::find_words(&f, "mari");
    // let find_i = find_i.split(' ').next().unwrap(); /* only take the first word found */
    /*
    let file = PathBuf::from("input.txt");
    let query = Query::substring(find_i, "Mari");
    let patcher = FilePatcher::new(&file, &query).unwrap();
    patcher.unwrap().run().unwrap();
    */

    Ok(())
}
