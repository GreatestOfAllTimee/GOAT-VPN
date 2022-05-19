pub fn display_error_convert(method: &str, types: &str, file: &str, line: u32) -> String {
    format!(
        r#"Error: failed while convert {} to {} - {}:{}"#,
        method, types, file, line
    )
}

pub fn display_error(method: &str, file: &str, line: u32) -> String {
    format!(
        r#"Error: can't get "{}" from prompt - {}:{}"#,
        method, file, line
    )
}
