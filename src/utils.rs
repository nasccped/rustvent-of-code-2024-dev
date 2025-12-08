use std::{fs, path::PathBuf};

/// Input file loader.
pub struct InputFile {
    pub content: String,
}

/// When file loading fails.
#[derive(Debug)]
pub struct InputFileError(pub PathBuf);

impl TryFrom<(&str, u8)> for InputFile {
    type Error = InputFileError;
    fn try_from(value: (&str, u8)) -> Result<Self, Self::Error> {
        Self::try_from((PathBuf::from(value.0), value.1))
    }
}

impl TryFrom<(PathBuf, u8)> for InputFile {
    type Error = InputFileError;
    fn try_from(value: (PathBuf, u8)) -> Result<Self, Self::Error> {
        let (mut buf, day) = value;
        buf.push(format!("day{:02}.txt", day).as_str());
        let mut content = fs::read_to_string(&buf).map_err(|_| InputFileError(buf))?;
        if let Some(c) = content.strip_prefix("\n") {
            content = c.into();
        }
        if let Some(c) = content.strip_suffix("\n") {
            content = c.into();
        }
        Ok(Self { content })
    }
}

trait VecStringTrim {
    fn foreach_trim(&self) -> Self;
}

pub trait StrArrVecString {
    // allow dead code. `into_vecstring` used only in test modules
    #![allow(dead_code)]
    fn into_vecstring(self) -> Vec<String>;
}

impl<const N: usize> StrArrVecString for [&str; N] {
    fn into_vecstring(self) -> Vec<String> {
        self.iter().map(|row| row.to_string()).collect()
    }
}

impl VecStringTrim for Vec<String> {
    fn foreach_trim(&self) -> Vec<String> {
        self.iter().map(|row| row.trim().to_string()).collect()
    }
}

pub fn get_file_content(path: &str) -> Vec<String> {
    let file =
        fs::read_to_string(path).unwrap_or_else(|_| panic!("Unable to open file ({})", path));
    file.split("\n")
        .filter(|row| !row.is_empty())
        .map(|row| row.to_string())
        .collect::<Vec<String>>()
        .foreach_trim()
}
