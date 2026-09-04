use std::error::Error;
use std::net::TcpListener;
use std::os::fd::AsFd;
use nix::poll::{poll, PollFd, PollFlags, PollTimeout};

fn main() -> Result<(), Box<dyn Error>> {
    let listener1 = TcpListener::bind("127.0.0.1:8081")?;
    let listener2 = TcpListener::bind("127.0.0.1:8082")?;

    let mut fds = [
        PollFd::new(listener1.as_fd(), PollFlags::POLLIN),
        PollFd::new(listener2.as_fd(), PollFlags::POLLIN),
    ];

    loop {
        poll(&mut fds, PollTimeout::NONE)?;
        if fds[0].revents().unwrap_or(PollFlags::empty()).contains(PollFlags::POLLIN) {
            let (_conn, addr) = listener1.accept()?;
            println!("Listener1 accepted connection from {addr}");
        }
        if fds[1].revents().unwrap_or(PollFlags::empty()).contains(PollFlags::POLLIN) {
            let (_conn, addr) = listener2.accept()?;
            println!("Listener2 accepted connection from {addr}");
        }
    }
}