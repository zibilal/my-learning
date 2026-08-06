use std::thread;
use std::time::Duration;
fn main() {
    // Get a handle to the main thread so the spawned thread
    // can wake it up
    let main_thread = thread::current();

    let handle = thread::spawn(move || {
        println!("worker: doing some work...");
        thread::sleep(Duration::from_secs(2));
        println!("worker: done, waking up main thead");
        main_thread.unpark(); // wakes the main thread's park() below
    });

    println!("main: parking until worker signals us");
    thread::park(); // blocks here until unpark() is called
    println!("main: woken up!");

    // Now demonstrate JoinHandle: get a handle to the *worker* thread
    // (this has to happen before spawn returns control, so let's spawn another one)
    let handle2 = thread::spawn(|| {
        println!("worker2: about to park");
        thread::park(); // will return immediately if unparked first
        println!("worker2: woken up");
    });

    println!("main: woken up!");
    // handle2.thread() gives us the thread handle for that spawned thread
    thread::sleep(Duration::from_millis(100)); // let worker2 start
    println!("main: here unpark was called");
    //handle2.thread().unpark();

    handle.join().unwrap();
    handle2.join().unwrap();
}