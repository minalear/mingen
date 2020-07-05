use criterion::{ black_box, criterion_group, criterion_main, Criterion };
use std::fs::File;
use std::io::{ Read };
use minmarkdown;

fn criterion_benchmark(c: &mut Criterion) {
  // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
  let mut file = File::open("input-long.md").unwrap();
  let mut input = String::new();
  file.read_to_string(&mut input).unwrap();
  let input = black_box(input);

  let _output = c.bench_function("md_to_html", |b| b.iter(|| minmarkdown::to_html(&input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);