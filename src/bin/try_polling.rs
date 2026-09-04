use std::io::{stdin, BufRead};
use std::os::fd::AsFd;
use nix::poll::{PollFd, PollFlags, PollTimeout, poll};

fn main() {
    let stdin = stdin();
    let mut fds = [PollFd::new(stdin.as_fd(), PollFlags::POLLIN)];

    println!("Wait 5 second ...");
    let timeout = PollTimeout::try_from(5000u16).unwrap();
    let n_ready = poll(&mut fds, timeout).unwrap();

    match n_ready {
        0 => println!("Reached timeout without input...."),
        _ if fds[0].revents().map_or(false, |r| r.contains(PollFlags::POLLIN)) => {
            let mut line = String::new();
            stdin.lock().read_line(&mut line).unwrap();
            println!("Reached timeout with input {line}");
        },
        _ => println!("Reached timeout"),
    }
}