fn main() {
    // Разименование нулевого указателя
    unsafe {
        let null_ptr: *const i32 = std::ptr::null();
        println!("{:?}", *null_ptr); // Process finished with exit code 139 (interrupted by signal 11:SIGSEGV)
    }
}
