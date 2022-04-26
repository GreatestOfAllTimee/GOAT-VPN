use anyhow::{Context, Result};
use regex::Regex;
use ruplacer::{FilePatcher, Query};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::{fs::File, io::Read};

// let file = PathBuf::from(file);
// let query = Query::substring(line, "");
// let patcher = FilePatcher::new(&file, &query).unwrap();
// patcher.unwrap().run().unwrap();

pub fn read_file(file: &str) -> Result<Vec<String>> {
    let mut file = File::open(file).with_context(|| format!("Failed to open file: {}", file))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // split whitespace and only take the second and third
    let new_content: Vec<String> = contents
        .lines()
        .map(|s| s.split_whitespace().collect::<Vec<&str>>()[1..3].join(" "))
        .collect();

    Ok(new_content)
}

pub fn find_words(string: &[String], word: &str) -> bool {
    string.iter().any(|s| s.contains(word))
}

#[allow(dead_code)]
pub fn delete_line(file: &str, line: &str) -> Result<()> {
    let file = PathBuf::from(file);
    let query = Query::substring(line, "");
    let patcher = FilePatcher::new(&file, &query)?;
    patcher
        .with_context(|| format!("Could not open file {}", file.display()))?
        .run()
        .with_context(|| format!("Could not delete line {}", line))?;
    Ok(())
}

#[allow(dead_code)]
pub fn delete_line_regex(file: &str, pattern: Regex) -> Result<()> {
    /*
    NOTE: Example how to use:
    let file: &str = "input.txt";
    let name = vec_prompt(read_file(file)?, 2);
    let test = Regex::new(format!(r"(?s)#([A-Z]+)(?m) {}+\n", &name).as_str())?;
    delete_line_regex(file, test)?;
    */

    let file = PathBuf::from(file);
    let query = Query::regex(pattern.clone(), "");
    let patcher = FilePatcher::new(&file, &query)?;

    patcher
        .with_context(|| format!("Could not open file {}", file.display()))?
        .run()
        .with_context(|| format!("Could not delete line {}", &pattern))?;

    Ok(())
}
