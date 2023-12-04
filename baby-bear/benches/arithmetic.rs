use criterion::{black_box, criterion_group, criterion_main, Criterion, BatchSize};
use p3_baby_bear::BabyBear;
use p3_field::{extension::BinomialExtensionField, Field};
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;

type Base = BabyBear;
type EF4 = BinomialExtensionField<BabyBear, 4>;
type EF5 = BinomialExtensionField<BabyBear, 5>;

fn bench_field<F: Field>(c: &mut Criterion, name: &str) 
where
    Standard: Distribution<F>,
{
    let mut rng = rand::thread_rng();

    c.bench_function(&format!("{} add", name), |b| {
        let x = rng.gen::<F>();
        let y = rng.gen::<F>();
        b.iter(|| black_box(black_box(x) + black_box(y)))
    });

    c.bench_function(&format!("{} sub", name), |b| {
        let x = rng.gen::<F>();
        let y = rng.gen::<F>();
        b.iter(|| black_box(black_box(x) - black_box(y)))
    });

    c.bench_function(&format!("{} mul", name), |b| {
        let x = rng.gen::<F>();
        let y = rng.gen::<F>();
        b.iter(|| black_box(black_box(x) * black_box(y)))
    });

    c.bench_function(&format!("{} square", name), |b| {
        let x = rng.gen::<F>();
        b.iter(|| black_box(black_box(x).square()))
    });

    c.bench_function(&format!("{} inv", name), |b| {
        let x = rng.gen::<F>();
        b.iter(|| black_box(black_box(x)).inverse())
    });

    c.bench_function(&format!("{} add-latency", name), |b| {
        b.iter_batched(
            || {
                let mut vec = Vec::new();
                for _ in 0..10000 {
                    vec.push(rng.gen::<F>())
                }
                vec
            },
            |x| x.iter().fold(F::zero(), |x, y| x + *y),
            BatchSize::SmallInput,
        )
    });

    c.bench_function(&format!("{} add-throughput", name), |b| {
        b.iter_batched(
            || {
                (
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                )
            },
            |(mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h, mut i, mut j)| {
                for _ in 0..1000 {
                    (a, b, c, d, e, f, g, h, i, j) = (
                        a + b,
                        b + c,
                        c + d,
                        d + e,
                        e + f,
                        f + g,
                        g + h,
                        h + i,
                        i + j,
                        j + a,
                    );
                }
                (a, b, c, d, e, f, g, h, i, j)
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function(&format!("{} sub-latency", name), |b| {
        b.iter_batched(
            || {
                let mut vec = Vec::new();
                for _ in 0..10000 {
                    vec.push(rng.gen::<F>())
                }
                vec
            },
            |x| x.iter().fold(F::zero(), |x, y| x - *y),
            BatchSize::SmallInput,
        )
    });

    c.bench_function(&format!("{} sub-throughput", name), |b| {
        b.iter_batched(
            || {
                (
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                    rng.gen::<F>(),
                )
            },
            |(mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h, mut i, mut j)| {
                for _ in 0..1000 {
                    (a, b, c, d, e, f, g, h, i, j) = (
                        a - b,
                        b - c,
                        c - d,
                        d - e,
                        e - f,
                        f - g,
                        g - h,
                        h - i,
                        i - j,
                        j - a,
                    );
                }
                (a, b, c, d, e, f, g, h, i, j)
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function(&format!("mul-throughput<{}>", name), |b| {
        b.iter_batched(
            || (rng.gen::<F>(), rng.gen::<F>(), rng.gen::<F>(), rng.gen::<F>()) ,
            |(mut x, mut y, mut z, mut w)| {
                for _ in 0..25 {
                    (x, y, z, w) = (x * y, y * z, z * w, w * x);
                }
                (x, y, z, w)
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function(&format!("mul-latency<{}>", name), |b| {
        b.iter_batched(
            || rng.gen::<F>(),
            |mut x| {
                for _ in 0..100 {
                    x = x * x;
                }
                x
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function(&format!("square-throughput<{}>", name), |b| {
        b.iter_batched(
            || (rng.gen::<F>(), rng.gen::<F>(), rng.gen::<F>(), rng.gen::<F>()) ,
            |(mut x, mut y, mut z, mut w)| {
                for _ in 0..25 {
                    (x, y, z, w) = (x.square(), y.square(), z.square(), w.square());
                }
                (x, y, z, w)
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function(&format!("square-latency<{}>", name), |b| {
        b.iter_batched(
            || rng.gen::<F>(),
            |mut x| {
                for _ in 0..100 {
                    x = x.square();
                }
                x
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_babybear(c: &mut Criterion) {
    let name = "BabyBear";
    bench_field::<Base>(c, name);
}

fn bench_quartic_extension(c: &mut Criterion) {
    let name = "BinomialExtensionField<BabyBear, 4>";
    bench_field::<EF4>(c, name);
}

fn bench_qunitic_extension(c: &mut Criterion) {
    let name = "BinomialExtensionField<BabyBear, 5>";
    bench_field::<EF5>(c, name);
}

criterion_group!(
    bench_babybear_ef,
    bench_babybear,
    bench_quartic_extension,
    bench_qunitic_extension
);

criterion_group!(arithmetic, bench_babybear);
criterion_main!(arithmetic);
