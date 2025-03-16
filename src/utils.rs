use std::fs;

pub trait VecStringTrim {
    fn foreach_trim(&self) -> Self;
}

pub trait StrArrVecString {
    // allow dead code. `into_vecstring` used only in test modules
    #![allow(dead_code)]
    fn into_vecstring(&self) -> Vec<String>;
}

impl<const N: usize> StrArrVecString for [&str; N] {
    fn into_vecstring(&self) -> Vec<String> {
        self.iter().map(|row| row.to_string()).collect()
    }
}

impl VecStringTrim for Vec<String> {
    fn foreach_trim(&self) -> Vec<String> {
        self.into_iter().map(|row| row.trim().to_string()).collect()
    }
}

pub fn get_file_content(path: &str) -> Vec<String> {
    let file = fs::read_to_string(path).expect(&format!("Unable to open file ({})", path));
    file.split("\n")
        .filter(|row| !row.is_empty())
        .map(|row| row.to_string())
        .collect()
}
