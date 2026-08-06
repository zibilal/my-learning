use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::future::Future;
use std::pin::Pin;
use std::pin::pin;
use std::thread;

use std::time::Duration;
use waker_fn::waker_fn;

// The data shared between the future and whoever completes it
struct SharedState {
    done: bool,
    waker: Option<Waker>,
}

// The future itself just holds a handle to hat shared state
struct MyFuture {
    shared: Arc<Mutex<SharedState>>,
}
impl Future for MyFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let mut state = self.shared.lock().unwrap();

        if state.done {
            // The work is finished -- we can return the result.
            Poll::Ready(())
        } else {
            // Not ready yet. Store the CURRENT waker so we can
            // notify the executor later, then tell it to wait.
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

fn complete(shared: &Arc<Mutex<SharedState>>) {
    let mut state = shared.lock().unwrap();
    state.done = true;

    // This is your snippet, in context
    // If someone is waiting on us, wake them up so their
    // executor polls this future again.
    if let Some(waker) = state.waker.take() {
        waker.wake();
    }
}

fn block_on<F: Future>(future: F) -> F::Output {
    let executor_thread = thread::current();
    let waker = waker_fn(move || executor_thread.unpark());
    let mut context = Context::from_waker(&waker);
    let mut future = pin!(future);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => thread::park(),
        }
    }
}
fn main() {
    let shared = Arc::new(Mutex::new(SharedState {
        done: false,
        waker: None,
    }));

    let fut = MyFuture { shared: shared.clone() };

    // spawn a thread that will "finish the work" after 2 seconds,
    // simulating an I/O event or timer completing
    let shared_for_thread = shared.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        complete(&shared_for_thread);
    });

    // run the future on an executor (e.g tokio or block_on)
    block_on(fut);
    println!("Future completed!");
}