use std::sync::{Arc, Mutex};
use std::thread;

pub(crate) fn run() {
    let mutex = Arc::new(Mutex::new(42));

    //If you pass the original Arc directly into a thread (without calling Arc::clone),
    // it will move the Arc into the thread, making it unavailable in the current scope.
    // By explicitly cloning the Arc, you retain ownership of the original Arc in the current scope.
    let handle1 = {
        let mutex = mutex.clone(); // Clone inline
        thread::spawn(move || {
            let mut guard = mutex.lock().unwrap();
            *guard += 1;
        })
    };

    let handle2 = {
        let mutex = mutex.clone(); // Clone inline
        thread::spawn(move || {
            let mut guard = mutex.lock().unwrap();
            *guard += 2;
        })
    };

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Final value: {:?}", *mutex.lock().unwrap());
}
