fn main() {
    // Висячий указатель
    unsafe {
        let dangling_ptr: *const i32;
        {
            let value = 42;
            dangling_ptr = &value;
        } // `value` goes out of scope here
          // Здесь `dangling_ptr` указывает на освобожденную память
        println!("{:?}", *dangling_ptr); // Ошибка: неопределенное поведение
    }
}
