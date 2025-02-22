use std::sync::Mutex;
use std::thread;

mod rc_in_mutex;
mod share_non_sync_mutex;

struct NotSync; // Тип, который не безопасен для одновременного доступа.

fn main() {
    unsafe impl Send for NotSync {} // Мы гарантируем, что его можно передавать между потоками.

    let data = Mutex::new(NotSync); // Это работает, так как `T: Send`.

    let handle = thread::spawn(move || {
        let _guard = data.lock().unwrap();
        // Мы можем безопасно работать с данными внутри Mutex,
        // потому что блокировка гарантирует, что никто другой не может
        // получить доступ к `NotSync` в этот момент.
        println!("GUESS THAT THE SOFTWARE!");
    });

    // Wait for the spawned thread to complete
    handle.join().unwrap();
}
