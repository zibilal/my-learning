use std::io::{pipe, BufRead, BufReader};
use std::process::Command;
use std::thread;

fn main() -> std::io::Result<()> {
    let (reader, writer) = pipe()?;
    let writer_clone = writer.try_clone()?;

    let mut child = Command::new("ping")
        .arg("-c").arg("3").arg("127.0.0.1")
        .stdout(writer)
        .stderr(writer_clone)
        .spawn()?;

    let handle = thread::spawn(move || {
        let buf_reader = BufReader::new(reader);
        for line in buf_reader.lines() {
            println!("Child says {}", line.unwrap())
        }
    });

    child.wait()?;
    handle.join().unwrap();

    Ok(())
}