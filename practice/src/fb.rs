use criterion::{criterion_group, criterion_main, Criterion};

fn fibonacci_recursive(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

fn fibonacci_iterative(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let tmp = a;
        a = b;
        b = tmp + b;
    }
    a
}

fn fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("fibonacci_recursive", |b| b.iter(|| fibonacci_recursive(20)));
    c.bench_function("fibonacci_iterative", |b| b.iter(|| fibonacci_iterative(20)));
}

criterion_group!(benches, fibonacci_benchmark);
criterion_main!(benches);
