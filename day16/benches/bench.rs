use criterion::{criterion_group, criterion_main, Criterion};
use day16::*;

fn bench_partner(c: &mut Criterion) {
    let mut group = c.benchmark_group("move_comparison");
    let a = 'o';
    let b = 'p';
    let partner = Moves::Partner(a, b);

    group.bench_function("partner", |b| {
        let mut programs = Programs::new();
        b.iter(|| partner.execute_move(&mut programs));
    });

    group.bench_function("first matches", |b| {
        let mut programs = Programs::new();
        let mut indices: Vec<usize> = Vec::with_capacity(2);
        b.iter(|| {
            indices = programs
                .0
                .iter()
                .enumerate()
                .filter(|(_, &c)| c == 'o' || c == 'p')
                .take(2)
                .map(|(i, _)| i)
                .collect();
            programs.0.swap(indices[0], indices[1]);
        });
    });

    group.bench_function("swap in place", |b| {
        let mut programs = Programs::new();
        b.iter(|| {
            for i in 0..16 {
                if programs.0[i] == 'o' {
                    programs.0[i] = 'p';
                }
                else if programs.0[i] == 'p' {
                    programs.0[i] = 'o';
                }
            }
        })
    });

    let swap = Moves::Exchange(14, 15);
    group.bench_function("exchange", |b| {
        let mut programs = Programs::new();
        b.iter(|| swap.execute_move(&mut programs));
    });

    group.finish();
}

criterion_group!(benches, bench_partner);
criterion_main!(benches);
