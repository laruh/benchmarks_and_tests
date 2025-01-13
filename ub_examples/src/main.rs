#[allow(unused_imports)]
use std::mem::size_of_val;

fn main() {
    // Разименование нулевого указателя
    // unsafe {
    //     let null_ptr: *const i32 = std::ptr::null();
    //     println!("{:?}", *null_ptr); // Process finished with exit code 139 (interrupted by signal 11:SIGSEGV)
    // }

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

    // Нарушение инвариантов типов
    // unsafe {
    //     let x: u32 = 42;
    //     let ptr = &x as *const u32 as *const f32; // Приведение указателя с u32 на f32
    //     let y = *ptr; // Нарушение: `u32` интерпретируется как `f32`, UB
    //     println!("{}", y); // Вывод 0.000000000000000000000000000000000000000000059
    // }

    // Чтение неинициализированной памяти
    // unsafe {
    //     // Этот тип позволяет вручную работать с неинициализированной памятью, обходя проверки компилятора
    //     use std::mem::MaybeUninit;
    //     // Выделяет память для переменной, но не инициализирует её. Содержимое памяти остаётся мусором.
    //     let uninit: MaybeUninit<i32> = MaybeUninit::uninit();
    //     // Читаем память без инициализации. assume_init Говорит компилятору, что память инициализирована, хотя это не так.
    //     let value = uninit.assume_init();
    //     // Некорректное поведение. Чтение "мусора" из памяти, что может привести к некорректным значениям или даже сбоям.
    //     println!("Value: {}", value);
    // }

    // Чтение неинициализированной памяти в safe коде
    // let x: i32;
    // println!("{}", x); // Ошибка: переменная `x` используется до инициализации
}
