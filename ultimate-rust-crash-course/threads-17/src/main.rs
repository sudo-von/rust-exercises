use std::thread;

fn main() {
    let handle = thread::spawn(move || {
        // Do stuff in a child thread.
        for x in 0..=100  {
            println!("Child thread... {}", x);
        }
    });

    // do stuff simultaneously in the main thread.
    println!("Main thread...");
    // wait until thread has exited.
    handle.join().unwrap();
}
