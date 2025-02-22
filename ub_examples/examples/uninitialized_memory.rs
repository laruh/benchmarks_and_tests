fn main() {
    // Чтение неинициализированной памяти в unsafe коде
    unsafe {
        // Этот тип позволяет вручную работать с неинициализированной памятью, обходя проверки компилятора
        use std::mem::MaybeUninit;
        // Выделяет память для переменной, но не инициализирует её. Содержимое памяти остаётся мусором.
        let uninit: MaybeUninit<i32> = MaybeUninit::uninit();
        // Читаем память без инициализации. assume_init Говорит компилятору, что память инициализирована, хотя это не так.
        let value = uninit.assume_init();
        // Некорректное поведение. Чтение "мусора" из памяти, что может привести к некорректным значениям или даже сбоям.
        println!("Value: {}", value);
    }

    // Чтение неинициализированной памяти в safe коде
    // let x: i32;
    // println!("{}", x); // Ошибка: переменная `x` используется до инициализации
}
