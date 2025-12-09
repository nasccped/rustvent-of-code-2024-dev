pub struct ErrReport {
    title: Box<str>,
    content: Option<Box<str>>,
}

trait Sealed: Into<Box<str>> {}
impl Sealed for &str {}
impl Sealed for String {}

impl<T: Sealed> From<T> for ErrReport {
    fn from(value: T) -> Self {
        Self {
            title: value.into(),
            content: None,
        }
    }
}

impl<T: Sealed, U: Sealed> From<(T, U)> for ErrReport {
    fn from(value: (T, U)) -> Self {
        Self {
            title: value.0.into(),
            content: Some(value.1.into()),
        }
    }
}

impl ErrReport {
    pub fn print_and_exit(self) -> ! {
        println!("\x1b[91merror\x1b[0m: {}", self.title);
        if let Some(content) = self.content {
            println!("\n{}", content);
        }
        std::process::exit(1)
    }
}
