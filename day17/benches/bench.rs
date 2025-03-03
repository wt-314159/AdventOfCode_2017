use std::collections::VecDeque;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
const VEC_SIZE: usize = 10_000_000;
const REPETITIONS: usize = 10;

fn bench_inserts(c: &mut Criterion) {
    let mut group = c.benchmark_group("inserts");

    group.bench_function("vec insert", |b| {
        b.iter(|| {
            let mut vec = vec![0; VEC_SIZE];
            for i in 0..REPETITIONS {
                vec.insert(black_box(1), black_box(i));
            }
        })
    });

    group.bench_function("vec grow", |b| {
        b.iter(|| {
            let mut vec = vec![0, 1, 2];
            for i in 0..REPETITIONS  {
                vec.insert(black_box(1), black_box(i));
            }
        })
    });

    group.bench_function("vec deque insert", |b| {
        b.iter(|| {
            let vec: Vec<usize> = vec![0; VEC_SIZE];
            let mut vecdq: VecDeque<usize> = vec.into_iter().collect();
            for i in 0..REPETITIONS {
                vecdq.insert(black_box(1), black_box(i));
            }
        })
    });

    group.bench_function("vec deque shift", |b| {
        b.iter(|| {
            let vec: Vec<usize> = vec![0; VEC_SIZE];
            let mut vecdq: VecDeque<usize> = vec.into_iter().collect();
            for i in 0..REPETITIONS  {
                vecdq.rotate_left(i);
                vecdq.push_front(i);
                vecdq.rotate_right(i);
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bench_inserts);
criterion_main!(benches);
