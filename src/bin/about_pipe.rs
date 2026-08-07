use std::io::{pipe, Read, Write};
fn main() -> std::io::Result<()> {
    let (mut reader, mut writer) = pipe()?;

    writer.write_all(b"hello through the pipe")?;
    drop(writer); // close write end so read doesn't block forever

    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    println!("{buf}");
    Ok(())
}