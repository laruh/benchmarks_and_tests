fn main() {
    // Single Box
    let boxed_arr = Box::new([0u64; 1_000_000_000]); // Allocates 8 GB on the heap
    println!(
        "Boxed array allocated on heap with length: {}",
        boxed_arr.len()
    );
    println!(
        "Size of pointer: {} bytes",
        std::mem::size_of_val(&boxed_arr)
    );

    // Multiple Boxes
    let mut pointers = Vec::with_capacity(1_000_000);
    for _ in 0..1_000_000 {
        // Each box call allocates memory on the heap to store u64
        let boxed = Box::new(0u64);
        pointers.push(boxed);
    }
    println!(
        "Size of pointers: {} bytes",
        std::mem::size_of_val(&pointers)
    );

    let boxed = Box::new(0u64);
    println!(
        "Size of Box pointer: {} bytes",
        std::mem::size_of_val(&boxed)
    );
    println!(
        "Size of value inside Box: {} bytes",
        std::mem::size_of_val(&*boxed)
    );
    println!("Address of Box pointer on stack: {:p}", &boxed);
    println!("Address of value in Box on heap: {:p}", &*boxed);
}

#[allow(dead_code)]
fn recursive_function(depth: usize) {
    let array = [0u8; 1_000]; // Allocate 1 KB on the stack
    println!(
        "Simulating stack overflow with array of size: {}",
        array.len()
    );
    if depth > 0 {
        recursive_function(depth - 1);
    }
}

#[allow(dead_code, unconditional_recursion)]
fn simulate_stack_overflow() {
    let large_array = [0u8; 1_000_000]; // Allocate a large array on the stack
    println!(
        "Simulating stack overflow with array of size: {}",
        large_array.len()
    );
    simulate_stack_overflow(); // Recursive call to cause stack overflow
}
