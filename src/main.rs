use requestty::{Answer, Question};
use std::{fs::File, io::Read};

fn read_file() -> Vec<String> {
    let mut file = File::open("input.txt")
        .map_err(|_| {
            println!("Could not open file");
            std::process::exit(1);
        })
        .unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|_| {
            println!("Could not read file");
            std::process::exit(1);
        })
        .unwrap();

    // split whitespace and only take the second and third
    contents
        .lines()
        .map(|s| s.split_whitespace().collect::<Vec<&str>>()[1..3].join(" "))
        .collect()
}

fn main() {
    let err_msg: &str = "Name cannot be empty!";
    let err_msg2: &str = "Name must be at least 3 characters long!";

    let question_1: Question = Question::input("name")
        .message("Enter your name")
        .validate(|s, _| {
            if s.is_empty() {
                Err(err_msg.to_string())
            } else if s.len() < 3 {
                Err(err_msg2.to_string())
            } else {
                Ok(())
            }
        })
        .build();

    let maria: Answer = requestty::prompt_one(question_1).unwrap();

    let question_2: Question = Question::raw_select("Multi Select")
        .message("Choose your favorite authors")
        .choices(read_file())
        .build();

    let authors = requestty::prompt_one(question_2).unwrap();
    let authors = authors.try_into_list_item().unwrap();
    let authors = authors.text.as_str();

    let client_name = authors.split_whitespace().collect::<Vec<&str>>()[0];

    println!("{}", maria.as_string().unwrap_or(""));
    println!("{}", client_name);
}
