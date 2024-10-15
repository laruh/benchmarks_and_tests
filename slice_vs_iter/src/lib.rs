/// Extends a vector using slices directly.
pub fn extend_from_slice(
    funding_timelock_bytes: &[u8],
    payment_timelock_bytes: &[u8],
    secret_hash: &[u8],
) -> Vec<u8> {
    let mut input = Vec::with_capacity(
        funding_timelock_bytes.len() + payment_timelock_bytes.len() + secret_hash.len(),
    );
    input.extend_from_slice(funding_timelock_bytes);
    input.extend_from_slice(payment_timelock_bytes);
    input.extend_from_slice(secret_hash);
    input
}

pub fn extend_with_iterwrap(
    funding_timelock_bytes: &[u8],
    payment_timelock_bytes: &[u8],
    secret_hash: &[u8],
) -> Vec<u8> {
    let mut input = Vec::with_capacity(
        funding_timelock_bytes.len() + payment_timelock_bytes.len() + secret_hash.len(),
    );

    // Wrap each slice iterator in IterWrap
    let funding_iter = IterWrap(funding_timelock_bytes.iter());
    let payment_iter = IterWrap(payment_timelock_bytes.iter());
    let secret_iter = IterWrap(secret_hash.iter());

    // Extend the vector using the wrapped iterators
    input.extend(funding_iter);
    input.extend(payment_iter);
    input.extend(secret_iter);

    input
}

/// Extends a vector using an iterator chain.
pub fn extend_with_chain(
    funding_timelock_bytes: &[u8],
    payment_timelock_bytes: &[u8],
    secret_hash: &[u8],
) -> Vec<u8> {
    let mut input = Vec::with_capacity(
        funding_timelock_bytes.len() + payment_timelock_bytes.len() + secret_hash.len(),
    );
    input.extend(IterWrap(
        funding_timelock_bytes
            .iter()
            .chain(payment_timelock_bytes.iter())
            .chain(secret_hash.iter()),
    ));
    input
}

pub struct IterWrap<I>(pub I);

impl<I: Iterator> Iterator for IterWrap<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    // fn size_hint(&self) -> (usize, Option<usize>) {
    //     self.0.size_hint() // Explicitly delegate size_hint to the underlying iterator
    // }
}
