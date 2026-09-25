use std::fmt::Formatter;

fn main() {

}

#[derive(Debug)]
struct AppError {
    source: std::io::Error,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not load configuration")
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}