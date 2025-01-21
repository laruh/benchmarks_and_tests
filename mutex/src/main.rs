mod non_sync_mutex_one_thread;
mod rc_in_mutex;
mod share_mutex_between_threads;
mod share_non_sync_mutex;

fn main() {
    non_sync_mutex_one_thread::run();
    share_mutex_between_threads::run();
}
