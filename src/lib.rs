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

pub fn extend_with_chain(
    funding_timelock_bytes: &[u8],
    payment_timelock_bytes: &[u8],
    secret_hash: &[u8],
) -> Vec<u8> {
    let mut input = Vec::with_capacity(
        funding_timelock_bytes.len() + payment_timelock_bytes.len() + secret_hash.len(),
    );
    input.extend(
        funding_timelock_bytes
            .iter()
            .chain(payment_timelock_bytes.iter())
            .chain(secret_hash.iter()),
    );
    input
}
