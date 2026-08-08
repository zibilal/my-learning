use std::io::{copy, pipe, stdin, stdout};
use std::thread;

fn main() -> std::io::Result<()> {
    let (mut reader, mut writer) = pipe()?;

    let writer_task = thread::spawn(move || -> std::io::Result<()> {
        let mut input = stdin();
        copy(&mut input, &mut writer)?;
        Ok(())
    });

    let reader_task = thread::spawn(move || -> std::io::Result<()> {
        let mut output = stdout();
        copy(&mut reader, &mut output)?;
        Ok(())
    });

    writer_task.join().unwrap()?;
    reader_task.join().unwrap()?;

    Ok(())
}