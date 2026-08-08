use std::io::{pipe, Read, Write};

fn main() -> std::io::Result<()> {
    let (mut reader, mut writer) = pipe()?;

    writer.write_all(b"Hello World")?;
    drop(writer);

    let mut str = String::new();
    reader.read_to_string(&mut str)?;

    println!("{}", str);
    Ok(())
}