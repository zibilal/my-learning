use std::io::PipeWriter;
use std::os::fd::OwnedFd;
use std::sync::{Arc, Mutex, mpsc};
use std::task::Waker;

#[derive(Clone)]
pub struct ReactorHandle {
    fd_sender: mpsc::Sender<(OwnedFd, Waker)>,
    interrupt_writer: Arc<Mutex<PipeWriter>>,
}

use std::sync::OnceLock;

static REACTOR: OnceLock<ReactorHandle> = OnceLock::new();
pub fn get_reactor() -> ReactorHandle {
    REACTOR
        .get_or_init(|| {
            let (reader, writer) = std::io::pipe().expect("Could not create pipe");
            let (sender, receiver) = mpsc::channel::<(OwnedFd, Waker)>();
            // std::thread::spawn(move || reactor_loop(reader, receiver));

            ReactorHandle {
                fd_sender: sender,
                interrupt_writer: Arc::new(Mutex::new(writer)),
            }
        })
        .clone()
}