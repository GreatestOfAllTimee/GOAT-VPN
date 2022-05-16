use anyhow::{Context, Result};
use regex::Regex;
use ruplacer::{FilePatcher, Query};
use serde_json::json;
use std::path::PathBuf;
use std::{fs::File, io::Read};

// let file = PathBuf::from(file);
// let query = Query::substring(line, "");
// let patcher = FilePatcher::new(&file, &query).unwrap();
// patcher.unwrap().run().unwrap();

/// ### EXAMPLE
/// ```
/// let file: &str = "input.txt";
/// let name = vec_prompt(read_file(file)?, 2).unwrap();
/// ```
#[allow(dead_code)]
pub fn read_file(file: &str) -> Result<Vec<String>> {
    let mut file = File::open(file)
        .with_context(|| format!("Failed to open file: {file} - {}:{}", file!(), line!()))?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    // split whitespace and only take the second and third
    let new_content: Vec<String> = contents
        .lines()
        .map(|s| s.split_whitespace().collect::<Vec<&str>>()[1..3].join(" "))
        .collect();

    Ok(new_content)
}

#[allow(dead_code)]
pub fn delete_line(file: &str, line: &str) -> Result<()> {
    let file = PathBuf::from(file);
    let query = Query::substring(line, "");
    let patcher = FilePatcher::new(&file, &query)?;

    patcher
        .with_context(|| {
            format!(
                "Could not open file {} - {}:{}",
                file.display(),
                file!(),
                line!()
            )
        })?
        .run()
        .with_context(|| format!("Could not delete line {line} - {}:{}", file!(), line!()))?;

    Ok(())
}

/// ### EXAMPLE
/// ```
/// let file: &str = "input.txt";
/// let name = vec_prompt(read_file(file).unwrap(), 2);
/// let test = Regex::new(format!(r"(?s)#([A-Z]+)(?m) {}+\n", &name).as_str()).unwrap();
/// delete_line_regex(file, test).unwrap();
/// ```
#[allow(dead_code)]
pub fn delete_line_regex(file: &str, pattern: Regex) -> Result<()> {
    let file = PathBuf::from(file);
    let query = Query::regex(pattern.clone(), "");
    let patcher = FilePatcher::new(&file, &query)?;

    patcher
        .with_context(|| {
            format!(
                "Could not open file {} - {}:{}",
                file.display(),
                file!(),
                line!()
            )
        })?
        .run()
        .with_context(|| format!("Could not delete line {pattern} - {}:{}", file!(), line!()))?;

    Ok(())
}

/// ### EXAMPLE
/// ```
/// let test_ssh = user_prompt_index("Select a services", menu_ovpn_ssh());
/// let test_ssh = test_ssh.as_list_item().context("Error").unwrap();
/// let user_name = get_user_index(&test_ssh.text, 1);
/// println!("{}", user_name);
/// ```
#[allow(dead_code)]
pub fn get_user_index(value: &str, index: usize) -> String {
    value.split_whitespace().collect::<Vec<&str>>()[index].to_string()
}

/// ### EXAMPLE
/// ```
/// let f = read_file(file).unwrap();
/// let found: bool = find_words(&f, "Zayne");
///
/// if found {
///     println!("Found");
/// } else {
///     println!("Not found");
/// }
/// ```
#[allow(dead_code)]
pub fn find_words(string: &[String], word: &str) -> bool {
    string.iter().any(|s| s.contains(word))
}

/// ### EXAMPLE
/// ```
/// let f = read_file(file).unwrap();
/// let found: bool = find_words_string(f, "Zayne");
///
/// if found {
///     println!("Found");
/// } else {
///     println!("Not found");
/// }
/// ```
pub fn find_words_string(string: &str, word: &str) -> bool {
    string.split_whitespace().any(|s| s.contains(word))
}

/// ### EXAMPLE
/// ```
/// let username: String = "Zayne".to_owned();
/// let date: String = "2022-01-01".to_owned();
///
/// append_line( &file_module, format!("#USER {} {}\n", username, date) .unwrap();
/// ```
pub fn append_line(file_module: &str, new_line: String) -> Result<()> {
    use std::fs::OpenOptions;
    use std::io::Write;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_module)
        .with_context(|| {
            format!(
                "Failed to open file: {file_module} - {}:{}",
                file!(),
                line!()
            )
        })?;

    file.write_all(new_line.as_bytes()).with_context(|| {
        format!(
            "Failed to write to file: {file_module} - {}:{}",
            file!(),
            line!()
        )
    })?;

    Ok(())
}

#[allow(dead_code)]
pub fn get_user_data(file: &str) -> Result<()> {
    let mut reader = lets_read::Reader::open(file)?;
    let mut buffer = String::new();

    while let Some(line) = reader.read_line(&mut buffer) {
        println!("{}", line?.trim());
    }

    Ok(())
}

pub fn get_test(user: &str, user_data: &str) -> Result<bool> {
    let mut reader = lets_read::Reader::open(user_data)?;
    let mut buffer = String::new();

    while let Some(line) = reader.read_line(&mut buffer) {
        let line = line?.trim();
        if find_words_string(line, user) {
            return Ok(true);
        }
    }

    Ok(false)
}

pub fn display_user_data(file: &str) -> Result<Vec<String>> {
    let mut reader = lets_read::Reader::open(file)?;
    let mut buffer = String::new();
    // restore all data at vector
    let mut data: Vec<String> = Vec::new();

    while let Some(line) = reader.read_line(&mut buffer) {
        let line = line?.trim();
        if !line.is_empty() {
            // get second and third word
            let line = line.split_whitespace().collect::<Vec<&str>>()[1..3].join(" ");
            data.push(line);
        }
    }

    Ok(data)
}

/// ### EXAMPLE
/// ```
/// use serde_json::{json, Value};
///
/// let id: &str = "999";
/// let mut data: Value = json!({
///    "id": id,
///    "alterId": 4
/// });
///
/// append_json("config.json", id).unwrap();
/// ```
#[allow(dead_code)]
pub fn append_json(file: &str, new_data: serde_json::Value) -> Result<()> {
    let data = std::fs::read_to_string(file)?;
    let mut serde_data: serde_json::Value = serde_json::from_str(&data)?;

    let array_mut = serde_data["inbounds"][0]["settings"]["clients"]
        .as_array_mut()
        .context(format!("Error: while run {}:{}", file!(), line!()))?;

    array_mut.push(json!(new_data));

    // write data in file
    let data = serde_json::to_string_pretty(&serde_data)?;
    std::fs::write(file, data)?;

    Ok(())
}

/// ### EXAMPLE
/// ```
/// // value to delete
/// let user_email: &str = "user@gmail.com";
/// remove_json_value("config.json", user_email).unwrap();
/// ```
#[allow(dead_code)]
pub fn remove_json_value(file: &str, need_to_remove: &str) -> Result<()> {
    let data = std::fs::read_to_string(file)?;
    let mut serde_data: serde_json::Value = serde_json::from_str(&data)?;

    let array_mut = serde_data["inbounds"][0]["settings"]["clients"]
        .as_array_mut()
        .context(format!("Error: while run {}:{}", file!(), line!()))?;

    // find the index of the value to remove.
    // NOTE: we find the index of EMAIL not ID
    let index = array_mut
        .iter()
        .position(|x| x["email"].as_str().unwrap() == need_to_remove)
        .with_context(|| {
            format!(
                "Could not find {need_to_remove} in {file} - {}:{}",
                file!(),
                line!()
            )
        })?;

    // remove the value
    array_mut.remove(index);

    // write data in file
    let data = serde_json::to_string_pretty(&serde_data)?;
    std::fs::write(file, data)?;

    Ok(())
}

/// ### EXAMPLE
/// ```
/// use serde_json::{json, Value};
///
/// let password: Value = "new_password".into();
/// remove_json_value("config.json", id).unwrap();
/// ```
#[allow(dead_code)]
pub fn append_json_trojan(file: &str, new_data: serde_json::Value) -> Result<()> {
    let data = std::fs::read_to_string(file)?;
    let mut serde_data: serde_json::Value = serde_json::from_str(&data)?;

    let array_mut = serde_data["password"].as_array_mut().context(format!(
        "Error: while run {}:{}",
        file!(),
        line!()
    ))?;

    array_mut.push(new_data);

    // write data in file
    let data = serde_json::to_string_pretty(&serde_data)?;
    std::fs::write(file, data)?;

    Ok(())
}

/// # EXAMPLE
/// ```
/// // value to delete
/// let first_name: &str = "Johnathan";
/// remove_json_value("config.json", first_name).unwrap();
/// ```
#[allow(dead_code)]
pub fn remove_json_trojan(file: &str, need_to_remove: &str) -> Result<()> {
    let data = std::fs::read_to_string(file)?;
    let mut serde_data: serde_json::Value = serde_json::from_str(&data)?;

    let array_mut = serde_data["password"].as_array_mut().context(format!(
        "Error: while run {}:{}",
        file!(),
        line!()
    ))?;

    // find need_to_remove in array_mut and remove it
    let index = array_mut
        .iter()
        .position(|x| x.as_str().unwrap() == need_to_remove)
        .with_context(|| {
            format!(
                "Could not find {need_to_remove} in {file} - {}:{}",
                file!(),
                line!()
            )
        })?;

    array_mut.remove(index);

    // write data in file
    let data = serde_json::to_string_pretty(&serde_data)?;
    std::fs::write(file, data)?;

    Ok(())
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
