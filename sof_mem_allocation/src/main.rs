use std::mem::size_of_val;

fn main() {
    // Single Box of array
    let boxed_arr = Box::new([0u64; 1_000_000_000]); // Allocates 8 GB on the heap
    println!(
        "----- Boxed Array -----\n\
        Length of boxed array: {}\n\
        Size of the pointer to the boxed array: {} bytes",
        boxed_arr.len(),
        size_of_val(&boxed_arr)
    );

    // Multiple Boxes in Vector
    let mut pointers = Vec::with_capacity(1_000_000);
    for _ in 0..1_000_000 {
        // Each Box call allocates memory on the heap to store a single u64
        let boxed = Box::new(0u64);
        pointers.push(boxed);
    }
    println!(
        "\n----- Vector of Boxes -----\n\
        Total number of pointers in vector: {}\n\
        Total size of pointer vector: {} bytes",
        pointers.len(),
        size_of_val(&pointers)
    );

    // Single Box of u64
    let boxed = Box::new(0u64);
    println!(
        "\n----- Single Box -----\n\
        Size of a single Box pointer: {} bytes\n\
        Size of the value inside the Box: {} bytes\n\
        Address of the Box pointer on the stack: {:p}\n\
        Address of the value in the Box on the heap: {:p}",
        size_of_val(&boxed),
        size_of_val(&*boxed),
        &boxed,
        &*boxed
    );

    // Uncomment to simulate a stack overflow
    // simulate_stack_overflow();

    println!("\nProcess completed.");
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
    let large_array = [0u8; 1_000_000]; // Allocates a 1 MB array on the stack
    println!(
        "Simulating stack overflow with array of size: {}",
        large_array.len()
    );
    simulate_stack_overflow(); // Recursive call to cause stack overflow
}
