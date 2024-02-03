use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use r0_baby_bear::{baby_bear_canonical, baby_bear_montgomery, Elem};
use rand_core::SeedableRng;

type Canonical = baby_bear_canonical::BabyBearElem;
type Montgomery = baby_bear_montgomery::BabyBearElem;
type CanonicalExt = baby_bear_canonical::BabyBearExtElem;
type MontgomeryExt = baby_bear_montgomery::BabyBearExtElem;

pub fn benchmark<F: Elem>(c: &mut Criterion, name: &str) {
    let mut rng = rand::rngs::SmallRng::seed_from_u64(2);
    c.bench_function(&format!("{} square", name), |b| {
        let x = F::random(&mut rng);
        b.iter(|| black_box(black_box(x).mul(black_box(x))))
    });

    c.bench_function(&format!("{} inv", name), |b| {
        let x = F::random(&mut rng);
        b.iter(|| black_box(black_box(x).inv()))
    });

    c.bench_function(&format!("{} mul", name), |b| {
        let x = F::random(&mut rng);
        let y = F::random(&mut rng);
        b.iter(|| black_box(black_box(x).mul(black_box(y))))
    });

    c.bench_function(&format!("{} add", name), |b| {
        let x = F::random(&mut rng);
        let y = F::random(&mut rng);
        b.iter(|| black_box(black_box(x).add(black_box(y))))
    });

    c.bench_function(&format!("{} add-latency-10k", name), |b| {
        b.iter_batched(
            || {
                let mut vec = Vec::new();
                for _ in 0..10000 {
                    vec.push(F::random(&mut rng));
                }
                vec
            },
            |x| x.iter().fold(F::ZERO, |x, y| x + *y),
            BatchSize::SmallInput,
        )
    });

    c.bench_function(&format!("{} add-throughput-10k", name), |b| {
        b.iter_batched(
            || {
                (
                    F::random(&mut rng),
                    F::random(&mut rng),
                    F::random(&mut rng),
                    F::random(&mut rng),
                    F::random(&mut rng),
                    F::random(&mut rng),
                    F::random(&mut rng),
                    F::random(&mut rng),
                    F::random(&mut rng),
                    F::random(&mut rng),
                )
            },
            |(mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h, mut i, mut j)| {
                for _ in 0..10000 {
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

    c.bench_function(&format!("{} sub-latency-10k", name), |b| {
        b.iter_batched(
            || {
                let mut vec = Vec::new();
                for _ in 0..10000 {
                    vec.push(F::random(&mut rng));
                }
                vec
            },
            |x| x.iter().fold(F::ZERO, |x, y| x - *y),
            BatchSize::SmallInput,
        )
    });

    c.bench_function(&format!("{} sub-throughput-10k", name), |b| {
        b.iter_batched(
            || {
                (
                    F::random(&mut rng),
                    F::random(&mut rng),
                    F::random(&mut rng),
                    F::random(&mut rng),
                    F::random(&mut rng),
                    F::random(&mut rng),
                    F::random(&mut rng),
                    F::random(&mut rng),
                    F::random(&mut rng),
                    F::random(&mut rng),
                )
            },
            |(mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h, mut i, mut j)| {
                for _ in 0..10000 {
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

    c.bench_function(&format!("{} mul-throughput-10k", name), |b| {
        b.iter_batched(
            || {
                (
                    F::random(&mut rng),
                    F::random(&mut rng),
                    F::random(&mut rng),
                    F::random(&mut rng),
                )
            },
            |(mut x, mut y, mut z, mut w)| {
                for _ in 0..10000 {
                    (x, y, z, w) = (x * y, y * z, z * w, w * x);
                }
                (x, y, z, w)
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function(&format!("{} mul-latency-10k", name), |b| {
        b.iter_batched(
            || F::random(&mut rng),
            |mut x| {
                for _ in 0..10000 {
                    x = x * x;
                }
                x
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function(&format!("{} sqr-throughput-10k", name), |b| {
        b.iter_batched(
            || {
                (
                    F::random(&mut rng),
                    F::random(&mut rng),
                    F::random(&mut rng),
                    F::random(&mut rng),
                )
            },
            |(mut x, mut y, mut z, mut w)| {
                for _ in 0..10000 {
                    (x, y, z, w) = (
                        x.clone() * x.clone(),
                        y.clone() * y.clone(),
                        z.clone() * z.clone(),
                        w.clone() * w.clone(),
                    );
                }
                (x, y, z, w)
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function(&format!("{} sqr-latency-10k", name), |b| {
        b.iter_batched(
            || F::random(&mut rng),
            |mut x| {
                for _ in 0..10000 {
                    x = x.clone() * x.clone();
                }
                x
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_babybear(c: &mut Criterion) {
    benchmark::<Canonical>(c, "canonical");
    benchmark::<Montgomery>(c, "montgomery");
    benchmark::<CanonicalExt>(c, "canonical ext");
    benchmark::<MontgomeryExt>(c, "montgomery ext");
}

criterion_group!(arithmetic, bench_babybear);
criterion_main!(arithmetic);
