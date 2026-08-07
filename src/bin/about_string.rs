fn main() {
    println!("size_of::<char>() = {} bytes", std::mem::size_of::<char>());

    let s = String::from("hello world, this is a test string");
    let as_bytes_len = s.len();
    let as_chars: Vec<char> = s.chars().collect();
    let as_chars_len: usize = as_chars.len() * std::mem::size_of::<char>();

    println!("\"{}\"", s);
    println!("stored as UTF-8 bytes: {as_bytes_len} bytes");
    println!("stored as Vec<char> (4 bytes each): {as_chars_len} byates");
    println!("overhead: {:.1}x more memory", as_chars_len as f64 / as_bytes_len as f64);
}