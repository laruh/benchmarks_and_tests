use criterion::{black_box, criterion_group, criterion_main, Criterion};
use slice_vs_iter::{extend_from_slice, extend_with_chain};

fn bench_slice_vs_iter(c: &mut Criterion) {
    let funding_timelock_bytes = vec![1; 1000];
    let payment_timelock_bytes = vec![2; 1000];
    let secret_hash = vec![3; 1000];

    c.bench_function("extend_from_slice", |b| {
        b.iter(|| {
            extend_from_slice(
                black_box(&funding_timelock_bytes),
                black_box(&payment_timelock_bytes),
                black_box(&secret_hash),
            )
        })
    });

    c.bench_function("extend_with_chain", |b| {
        b.iter(|| {
            extend_with_chain(
                black_box(&funding_timelock_bytes),
                black_box(&payment_timelock_bytes),
                black_box(&secret_hash),
            )
        })
    });
}

criterion_group!(benches, bench_slice_vs_iter);
criterion_main!(benches);
