use std::fs::File;
use std::io::{self, Read, Write};
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};
fn main() -> io::Result<()> {
    // 1. Create a file and inspect its raw fd (just for display/logging)
    let mut file = File::create("/tmp/fd_example.txt")?;
    let raw_fd: RawFd = file.as_raw_fd();
    println!("File opened with fd: {}", raw_fd);

    file.write_all(b"Hello, fd world!\n")?;

    // 2. Convert File -> OwnedFd (safe, no libc needed)
    // This *consumes* the File and hands you an owning fd wrapper
    let owned_fd: OwnedFd = file.into();
    println!("Now holding OwnedFd: {}", owned_fd.as_raw_fd());

    // 3. Convert OwnedFd back -> File (also safe)
    let mut file2: File = owned_fd.into();
    file2.write_all(b"Wrote more after round-trip\n")?;

    // 4. Borrow a fd without taking ownership, using AsFd/BorrwedFd
    print_fd_into(&file2);

    // 5. Read back the file's contents to confirm everything worked
    let mut file3 = File::open("/tmp/fd_example.txt")?;
    let mut contents = String::new();
    file3.read_to_string(&mut contents)?;
    println!("File contents: \n{}", contents);


    Ok(())
}

// A function generic over anything that can hand out a BorrowedFd -
// this mirrors how you'd write low-level fd based APIs without owning the file.
fn print_fd_into<F: AsFd>(f: &F) {
    let borrowed: BorrowedFd = f.as_fd();
    println!("Borrowed fd: {}", borrowed.as_raw_fd());
}