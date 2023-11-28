use criterion::{black_box, criterion_group, criterion_main, Criterion};
use p3_baby_bear::BabyBear;
use p3_field::{extension::BinomialExtensionField, Field};
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;

type Base = BabyBear;

pub fn benchmark_square<F: Field>(c: &mut Criterion, name: &str)
where
    Standard: Distribution<F>,
{
    let mut rng = rand::thread_rng();
    let x = rng.gen::<F>();
    c.bench_function(&format!("{} square", name), |b| {
        b.iter(|| black_box(black_box(x).square()))
    });
}

pub fn benchmark_inv<F: Field>(c: &mut Criterion, name: &str)
where
    Standard: Distribution<F>,
{
    let mut rng = rand::thread_rng();
    let x = rng.gen::<F>();
    c.bench_function(&format!("{} inv", name), |b| {
        b.iter(|| black_box(black_box(x)).inverse())
    });
}

pub fn benchmark_mul<F: Field>(c: &mut Criterion, name: &str)
where
    Standard: Distribution<F>,
{
    let mut rng = rand::thread_rng();
    let x = rng.gen::<F>();
    let y = rng.gen::<F>();
    c.bench_function(&format!("{} mul", name), |b| {
        b.iter(|| black_box(black_box(x) * black_box(y)))
    });
}

pub fn benchmark_add<F: Field>(c: &mut Criterion, name: &str)
where
    Standard: Distribution<F>,
{
    let mut rng = rand::thread_rng();
    let x = rng.gen::<F>();
    let y = rng.gen::<F>();
    c.bench_function(&format!("{} add", name), |b| {
        b.iter(|| black_box(black_box(x) + black_box(y)))
    });
}

fn bench_babybear(c: &mut Criterion) {
    let name = "BabyBear";
    benchmark_square::<Base>(c, name);
    benchmark_inv::<Base>(c, name);
    benchmark_mul::<Base>(c, name);
    benchmark_add::<Base>(c, name);
}

criterion_group!(arithmetic, bench_babybear);
criterion_main!(arithmetic);
