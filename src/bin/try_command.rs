use std::io::pipe;
use std::process::Command;

fn main() -> std::io::Result<()> {
    let (reader, writer) = pipe()?;

    let mut echo = Command::new("echo")
        .arg("hello world")
        .stdout(writer)
        .spawn()?;

    let grep = Command::new("grep")
        .arg("hello")
        .stdin(reader)
        .output()?;

    echo.wait()?;
    println!("{}", String::from_utf8_lossy(&grep.stdout));
    Ok(())
}