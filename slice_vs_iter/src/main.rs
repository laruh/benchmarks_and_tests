use sha2::{Digest, Sha256};
use slice_vs_iter::{extend_from_slice, extend_with_chain, IterWrap};
use std::io::Write;
use std::time::Instant;

pub fn main() {
    let slice = &[1, 2, 3, 4, 5];
    let iter = slice.iter();
    println!("{:?}", iter.size_hint()); // Output: (5, Some(5))

    let wrapped_iter = IterWrap(iter);
    println!("{:?}", wrapped_iter.size_hint()); // Output: (0, None)

    let funding_timelock_bytes = vec![1; 1000];
    let payment_timelock_bytes = vec![2; 1000];
    let secret_hash = vec![3; 1000];

    let iterations = 10000;

    // Test extend_from_slice
    let start = Instant::now();
    for _ in 0..iterations {
        let res = extend_from_slice(
            &funding_timelock_bytes,
            &payment_timelock_bytes,
            &secret_hash,
        );
        // Write the first element of res to /dev/null
        // This simulates using the result without keeping it
        if let Some(first_element) = res.first() {
            let _ = std::io::sink().write_all(&[*first_element]);
        }
    }
    let duration_extend_from_slice = start.elapsed();
    println!(
        "Time taken for extend_from_slice: {:?}",
        duration_extend_from_slice
    );

    // Test extend_with_chain
    let start = Instant::now();
    for _ in 0..iterations {
        let res = extend_with_chain(
            &funding_timelock_bytes,
            &payment_timelock_bytes,
            &secret_hash,
        );
        // Write the first element of res to /dev/null
        // This simulates using the result without keeping it
        if let Some(first_element) = res.first() {
            let _ = std::io::sink().write_all(&[*first_element]);
        }
    }
    let duration_extend_with_chain = start.elapsed();
    println!(
        "Time taken for extend_with_chain: {:?}",
        duration_extend_with_chain
    );

    // Calculate hash for extend_from_slice
    let input_from_slice = extend_from_slice(
        &funding_timelock_bytes,
        &payment_timelock_bytes,
        &secret_hash,
    );
    let hash_from_slice = Sha256::digest(input_from_slice).to_vec();

    // Calculate hash for extend_with_chain
    let input_with_chain = extend_with_chain(
        &funding_timelock_bytes,
        &payment_timelock_bytes,
        &secret_hash,
    );
    let hash_with_chain = Sha256::digest(input_with_chain).to_vec();

    // Verify both hashes are the same
    assert_eq!(hash_from_slice, hash_with_chain, "Hashes do not match!");
}
