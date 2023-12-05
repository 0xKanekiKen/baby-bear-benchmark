use criterion::{black_box, criterion_group, criterion_main, Criterion, BatchSize};
use p3_baby_bear::{BabyBear, PackedBabyBearNeon};
use p3_field::AbstractField;
use rand::Rng;

type Base = BabyBear;

fn bench_scalar(c: &mut Criterion, iteration: u32)
{
    let mut rng = rand::thread_rng();

    c.bench_function("scalar add", |b| {
        let x = [rng.gen::<Base>(); 4];
        let y = [rng.gen::<Base>(); 4];
        let mut res = [Base::zero(); 4];
        b.iter(|| for i in 0..4 {
            res[i] = black_box(black_box(x[i]) + black_box(y[i]));
        })
    });

    c.bench_function("scalar mul", |b| {
        let x = [rng.gen::<Base>(); 4];
        let y = [rng.gen::<Base>(); 4];
        let mut res = [Base::zero(); 4];
        b.iter(|| for i in 0..4 {
            res[i] = black_box(black_box(x[i]) * black_box(y[i]));
        })
    });

    c.bench_function("scalar sub", |b| {
        let x = [rng.gen::<Base>(); 4];
        let y = [rng.gen::<Base>(); 4];
        let mut res = [Base::zero(); 4];
        b.iter(|| for i in 0..4 {
            res[i] = black_box(black_box(x[i]) - black_box(y[i]));
        })
    });

    c.bench_function("scalar square", |b| {
        let x = [rng.gen::<Base>(); 4];
        let mut res = [Base::zero(); 4];
        b.iter(|| for i in 0..4 {
            res[i] = black_box(black_box(x[i]).square());
        })
    });

    c.bench_function(&format!("scalar add-latency {}k", iteration/1000), |b| {
        b.iter_batched(
            || {
                let mut vec = Vec::new();
                for _ in 0..iteration {
                    vec.push(rng.gen::<Base>())
                }
                vec
            },
            |x| x.iter().fold(Base::zero(), |x, y| x + *y),
            BatchSize::SmallInput,
        )
    });

    c.bench_function(&format!("scalar add-throughput {}k", iteration/1000), |b| {
        let (mut w, mut x, mut y, mut z) = (
            Base::zero(), Base::zero(), Base::zero(), Base::zero(),
        );
        b.iter_batched(
            || {
                (
                    rng.gen::<Base>(),
                    rng.gen::<Base>(),
                    rng.gen::<Base>(),
                    rng.gen::<Base>(),
                    rng.gen::<Base>(),
                    rng.gen::<Base>(),
                    rng.gen::<Base>(),
                    rng.gen::<Base>(),
                )
            },
            
            |(a, b, c, d, e, f, g, h)| {
                for _ in 0..iteration {
                    (w, x, y, z) = (
                        a + e,
                        b + f,
                        c + g,
                        d + h,
                    );
                }
                (w, x, y, z)
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function(&format!("scalar mul-latency {}k", iteration/1000), |b| {
        b.iter_batched(
            || {
                let mut vec = Vec::new();
                for _ in 0..iteration {
                    vec.push(rng.gen::<Base>())
                }
                vec
            },
            |x| x.iter().fold(Base::one(), |x, y| x * *y),
            BatchSize::SmallInput,
        )
    });

    c.bench_function(&format!("scalar mul-throughput {}k", iteration/1000), |b| {
        let (mut w, mut x, mut y, mut z) = (
            Base::zero(), Base::zero(), Base::zero(), Base::zero(),
        );
        b.iter_batched(
            || {
                (
                    rng.gen::<Base>(),
                    rng.gen::<Base>(),
                    rng.gen::<Base>(),
                    rng.gen::<Base>(),
                    rng.gen::<Base>(),
                    rng.gen::<Base>(),
                    rng.gen::<Base>(),
                    rng.gen::<Base>(),
                )
            },
            |(a, b, c, d, e, f, g, h)| {
                for _ in 0..iteration {
                    (w, x, y, z) = (
                        a * e,
                        b * f,
                        c * g,
                        d * h,
                    );
                }
                (w, x, y, z)
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_neon(c: &mut Criterion, iteration: u32)
{
    let mut rng = rand::thread_rng();

    c.bench_function("neon add", |b| {
        let x = [rng.gen::<Base>(); 4];
        let y = [rng.gen::<Base>(); 4];
        let mut res = PackedBabyBearNeon([Base::zero(); 4]);
        b.iter(|| res = black_box(PackedBabyBearNeon(black_box(x)) + PackedBabyBearNeon(black_box(y))))
    });

    c.bench_function("neon mul", |b| {
        let x = [rng.gen::<Base>(); 4];
        let y = [rng.gen::<Base>(); 4];
        let mut res = PackedBabyBearNeon([Base::zero(); 4]);
        b.iter(|| res = black_box(PackedBabyBearNeon(black_box(x)) * PackedBabyBearNeon(black_box(y))))
    });

    c.bench_function("neon sub", |b| {
        let x = [rng.gen::<Base>(); 4];
        let y = [rng.gen::<Base>(); 4];
        let mut res = PackedBabyBearNeon([Base::zero(); 4]);
        b.iter(|| res = black_box(PackedBabyBearNeon(black_box(x)) - PackedBabyBearNeon(black_box(y))))
    });

    c.bench_function("neon square", |b| {
        let x = [rng.gen::<Base>(); 4];
        let mut res = PackedBabyBearNeon([Base::zero(); 4]);
        b.iter(|| res = PackedBabyBearNeon(black_box(x)).square())
    });

    c.bench_function(&format!("neon add-latency {}k", iteration/1000), |b| {
        let mut res = Base::zero();
        b.iter_batched(
            || {
                let mut vec = Vec::new();
                for _ in 0..iteration/4 {
                    vec.push(PackedBabyBearNeon([rng.gen::<Base>(); 4]))
                }
                vec
            },
            |x| {let m = x.iter().fold(PackedBabyBearNeon([Base::zero(); 4]), |x, y| x + *y);
                for i in 0..4 {
                    res += m.0[i];
                }
                res
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function(&format!("neon add-throughput {}k", iteration/1000), |b| {
        let mut res = PackedBabyBearNeon([Base::zero(); 4]);
        b.iter_batched(
            || {
                (
                    PackedBabyBearNeon([rng.gen::<Base>(); 4]),
                    PackedBabyBearNeon([rng.gen::<Base>(); 4]),
                )
            },
            |(a, b)| {
                for _ in 0..iteration/4 {
                    res = a + b;
                }
                res
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function(&format!("neon mul-latency {}k", iteration/1000), |b| {
        let mut res = Base::one();
        b.iter_batched(
            || {
                let mut vec = Vec::new();
                for _ in 0..iteration/4 {
                    vec.push(PackedBabyBearNeon([rng.gen::<Base>(); 4]))
                }
                vec
            },
            |x| {let m = x.iter().fold(PackedBabyBearNeon([Base::one(); 4]), |x, y| x * *y);
                for i in 0..4 {
                    res *= m.0[i];
                }
                res
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function(&format!("neon mul-throughput {}k", iteration/1000), |b| {
        let mut res = PackedBabyBearNeon([Base::zero(); 4]);
        b.iter_batched(
            || {
                (
                    PackedBabyBearNeon([rng.gen::<Base>(); 4]),
                    PackedBabyBearNeon([rng.gen::<Base>(); 4]),
                )
            },
            |(a, b)| {
                for _ in 0..iteration/4 {
                    res = a * b;
                }
                res
            },
            BatchSize::SmallInput,
        )
    });
}

fn tenk_bench(c: &mut Criterion) {
    bench_scalar(c, 10000);
    bench_neon(c, 10000);
}

fn thousandk_bench(c: &mut Criterion) {
    bench_scalar(c, 100000);
    bench_neon(c, 100000);
}

criterion_group!(
    neon,
    tenk_bench,
    thousandk_bench,
);

criterion_main!(neon);