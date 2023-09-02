use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn vowel(vowels: [char; 5]) -> Result<usize, usize> {
    vowels.binary_search(&'a')
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("vowels_binV2", |b| {
        b.iter(|| vowel(black_box(['a', 'e', 'i', 'o', 'u'])));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
