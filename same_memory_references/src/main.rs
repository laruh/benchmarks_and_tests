use std::convert::TryInto;
struct PaymentSecret<'a>(&'a [u8; 32]);

impl<'a> PaymentSecret<'a> {
    fn new(data: &'a [u8; 32]) -> PaymentSecret<'a> {
        PaymentSecret(data)
    }
}

fn main() {
    // The below demonstrates multiple references to the same memory location.

    // slice is a reference to a contiguous sequence of elements.
    // In this case, the slice refers to the memory where [0; 32] is stored.
    let slice: &[u8] = &[0; 32];

    // This is a reference to a fixed-size array, created from the slice via try_into().
    // Both the slice and the array reference point to the same memory block in this example. No copying is involved.
    let array: &[u8; 32] = slice.try_into().expect("Not 32 bytes long");

    // The PaymentSecret struct encapsulates the reference to the same [u8; 32] array.
    // Internally, it stores a reference to the same memory block.
    let paymentsecret = PaymentSecret::new(array);

    // Compare memory content directly
    let sec_raw: *const [u8] = paymentsecret.0;
    println!("{:?}", sec_raw);
    let slice_raw: *const u8 = slice.as_ptr();
    println!("{:?}", slice_raw);
    let array_raw: *const u8 = array.as_ptr();
    println!("{:?}", array_raw);

    assert_eq!(slice_raw, array_raw); // raw pointers must be identical
}
