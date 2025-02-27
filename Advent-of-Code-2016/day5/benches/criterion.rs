use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day5::{read_input, part1, part2};

fn benchmark_part1(c: &mut Criterion) {
    let input = read_input("input.txt");
    let result = part1::part1(&input);
    println!("\x1b[34m\x1b[1mPart 1 result: {}\x1b[0m", result);
    let mut group = c.benchmark_group("part1");
    group.sample_size(10);
    group.bench_function("part1", |b| b.iter(|| part1::part1(black_box(&input))));
    group.finish()
}

fn benchmark_part2(c: &mut Criterion) {
    let input = read_input("input.txt");
    let result = part2::part2(&input);
    println!("\x1b[34m\x1b[1mPart 2 result: {}\x1b[0m", result);
    let mut group = c.benchmark_group("part2");
    group.sample_size(10);
    group.bench_function("part2", |b| b.iter(|| part2::part2(black_box(&input))));
    group.finish()
}

criterion_group!(benches, benchmark_part1, benchmark_part2);
criterion_main!(benches);