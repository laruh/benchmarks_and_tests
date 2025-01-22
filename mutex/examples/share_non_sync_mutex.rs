#[allow(dead_code)]
fn main() {
    println!("This example won't compile because the Mutex<T> is moved into the first thread, leaving it unavailable for the second thread.");
    println!(
        "(To fix this, wrap the Mutex<T> in an Arc<T> to allow shared ownership between threads.)"
    );

    // use std::sync::Mutex;
    // use std::thread;
    // let mutex = Mutex::new(42);
    //
    // let handle1 = thread::spawn(move || {
    //     // Value moved here
    //     let mut guard = mutex.lock().unwrap();
    //     *guard += 1;
    // });
    //
    // let handle2 = thread::spawn(move || {
    //     let mut guard = mutex.lock().unwrap();
    //     *guard += 2;
    // });
    //
    // handle1.join().unwrap();
    // handle2.join().unwrap();
}
