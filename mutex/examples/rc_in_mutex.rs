#[allow(dead_code)]
fn main() {
    println!("Rc in Mutex example won't compile because Rc<T> is not Send or Sync.");
    println!("(Uncomment the code to see the compilation error.)");
    // use std::rc::Rc;
    // use std::sync::{Arc, Mutex};
    // use std::thread;
    //
    // let rc_value = Rc::new(42);
    // let mutex = Arc::new(Mutex::new(rc_value));
    //
    // let handles: Vec<_> = (0..2)
    //     .map(|_| {
    //         let mutex_clone = Arc::clone(&mutex);
    //         thread::spawn(move || {
    //             // Rc<i32> cannot be sent between threads safely
    //             // use unsafe block to bypass the Send requirement
    //             unsafe {
    //                 let _guard = mutex_clone.lock().unwrap();
    //             }
    //         })
    //     })
    //     .collect();
    //
    // for handle in handles {
    //     handle.join().unwrap();
    // }
}
