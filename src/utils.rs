use std::fs;

pub fn get_file_content(path: &str) -> Vec<String> {
    let file = fs::read_to_string(path).expect(&format!("Unable to open file ({})", path));
    file.split("\n")
        .filter(|row| !row.is_empty())
        .map(|row| row.to_string())
        .collect()
}
