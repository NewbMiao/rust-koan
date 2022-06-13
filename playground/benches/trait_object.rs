mod executor;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use executor::{execute_boxed_trait_object, execute_generics, execute_trait_object, Shell};

pub fn generics_benchmark(c: &mut Criterion) {
    c.bench_function("generics", |b| {
        b.iter(|| {
            let cmd = Shell::new("ls", &[]);
            execute_generics(black_box(&cmd)).unwrap();
        })
    });
}

pub fn trait_object_benchmark(c: &mut Criterion) {
    c.bench_function("trait object", |b| {
        b.iter(|| {
            let cmd = Shell::new("ls", &[]);
            execute_trait_object(black_box(&cmd)).unwrap();
        })
    });
}

pub fn boxed_object_benchmark(c: &mut Criterion) {
    c.bench_function("boxed object", |b| {
        b.iter(|| {
            let cmd = Box::new(Shell::new("ls", &[]));
            execute_boxed_trait_object(black_box(cmd)).unwrap();
        })
    });
}

pub fn empty_generics_benchmark(c: &mut Criterion) {
    c.bench_function("empty generics", |b| {
        b.iter(|| {
            let cmd = Shell::new("none", &[]);
            execute_generics(black_box(&cmd)).unwrap();
        })
    });
}

pub fn empty_trait_object_benchmark(c: &mut Criterion) {
    c.bench_function("empty trait object", |b| {
        b.iter(|| {
            let cmd = Shell::new("none", &[]);
            execute_trait_object(black_box(&cmd)).unwrap();
        })
    });
}

pub fn empty_boxed_object_benchmark(c: &mut Criterion) {
    c.bench_function("empty boxed object", |b| {
        b.iter(|| {
            let cmd = Box::new(Shell::new("none", &[]));
            execute_boxed_trait_object(black_box(cmd)).unwrap();
        })
    });
}

criterion_group!(
    benches,
    generics_benchmark,
    trait_object_benchmark,
    boxed_object_benchmark,
    empty_generics_benchmark,
    empty_trait_object_benchmark,
    empty_boxed_object_benchmark
);
criterion_main!(benches);
