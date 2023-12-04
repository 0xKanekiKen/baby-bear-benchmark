use std::ops::{Mul, Add};

use criterion::{black_box, criterion_group, criterion_main, Criterion, BatchSize};
use r0_baby_bear::{baby_bear_canonical, baby_bear_montgomery, Elem};
use rand_core::SeedableRng;

type Canonical = baby_bear_canonical::BabyBearElem;
type Montgomery = baby_bear_montgomery::BabyBearElem;
type CanonicalExt = baby_bear_canonical::BabyBearExtElem;
type MontgomeryExt = baby_bear_montgomery::BabyBearExtElem;

pub fn benchmark_canonical(c: &mut Criterion)
{
    let mut rng = rand::rngs::SmallRng::seed_from_u64(2);
    c.bench_function("canonical square", |b| {
        let x: Canonical = Elem::random(&mut rng);
        b.iter(|| black_box(black_box(x).mul(black_box(x))))
    });

    c.bench_function("canonical inv", |b| {
        let x: Canonical = Elem::random(&mut rng);
        b.iter(|| black_box(black_box(x).inv()))
    });

    c.bench_function("canonical mul", |b| {
        let x: Canonical = Elem::random(&mut rng);
        let y: Canonical = Elem::random(&mut rng);
        b.iter(|| black_box(black_box(x).mul(black_box(y))))
    });

    c.bench_function("canonical add", |b| {
        let x: Canonical = Elem::random(&mut rng);
        let y: Canonical = Elem::random(&mut rng);
        b.iter(|| black_box(black_box(x).add(black_box(y))))
    });

    c.bench_function("canonical add-latency", |b| {
        b.iter_batched(
            || {
                let mut vec: Vec<Canonical> = Vec::new();
                for _ in 0..10000 {
                    vec.push(Elem::random(&mut rng));
                }
                vec
            },
            |x| x.iter().fold(Canonical::new(0), |x, y| x + *y),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("canonical add-throughput", |b| {
        b.iter_batched(
            || {
                (
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                )
            },
            |(mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h, mut i, mut j): (Canonical, Canonical, Canonical, Canonical, Canonical, Canonical, Canonical, Canonical, Canonical, Canonical)| {
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

    c.bench_function("canonical sub-latency", |b| {
        b.iter_batched(
            || {
                let mut vec: Vec<Canonical> = Vec::new();
                for _ in 0..10000 {
                    vec.push(Elem::random(&mut rng));
                }
                vec
            },
            |x| x.iter().fold(Canonical::new(0), |x, y| x - *y),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("Canonical sub-throughput", |b| {
        b.iter_batched(
            || {
                (
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                )
            },
            |(mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h, mut i, mut j): (Canonical, Canonical, Canonical, Canonical, Canonical, Canonical, Canonical, Canonical, Canonical, Canonical)| {
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

    c.bench_function("canonical mul-throughput", |b| {
        b.iter_batched(
            || (Elem::random(&mut rng),
            Elem::random(&mut rng),
            Elem::random(&mut rng),
            Elem::random(&mut rng),),
            |(mut x, mut y, mut z, mut w) : (Canonical, Canonical, Canonical, Canonical)| {
                for _ in 0..25 {
                    (x, y, z, w) = (x * y, y * z, z * w, w * x);
                }
                (x, y, z, w)
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function("canonical mul-latency", |b| {
        b.iter_batched(
            || Elem::random(&mut rng),
            |mut x: Canonical| {
                for _ in 0..100 {
                    x = x * x;
                }
                x
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function("canonical sqr-throughput>", |b| {
        b.iter_batched(
            || (Elem::random(&mut rng),
            Elem::random(&mut rng),
            Elem::random(&mut rng),
            Elem::random(&mut rng),),
            |(mut x, mut y, mut z, mut w) : (Canonical, Canonical, Canonical, Canonical)| {
                for _ in 0..25 {
                    (x, y, z, w) = (x.clone() * x.clone(), y.clone() * y.clone(), z.clone() * z.clone(), w.clone() * w.clone());
                }
                (x, y, z, w)
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function("canonical sqr-latency", |b| {
        b.iter_batched(
            || Elem::random(&mut rng),
            |mut x: Canonical| {
                for _ in 0..100 {
                    x = x.clone() * x.clone();
                }
                x
            },
            BatchSize::SmallInput,
        )
    });

}

pub fn benchmark_montgomery(c: &mut Criterion)
{
    let mut rng = rand::rngs::SmallRng::seed_from_u64(2);
    c.bench_function("montgomery square", |b| {
        let x: Montgomery = Elem::random(&mut rng);
        b.iter(|| black_box(black_box(x).mul(black_box(x))))
    });

    c.bench_function("montgomery inv", |b| {
        let x: Montgomery = Elem::random(&mut rng);
        b.iter(|| black_box(black_box(x).inv()))
    });

    c.bench_function("montgomery mul", |b| {
        let x: Montgomery = Elem::random(&mut rng);
        let y: Montgomery = Elem::random(&mut rng);
        b.iter(|| black_box(black_box(x).mul(black_box(y))))
    });

    c.bench_function("montgomery add", |b| {
        let x: Montgomery = Elem::random(&mut rng);
        let y: Montgomery = Elem::random(&mut rng);
        b.iter(|| black_box(black_box(x).add(black_box(y))))
    });

    c.bench_function("montgomery add-latency", |b| {
        b.iter_batched(
            || {
                let mut vec: Vec<Montgomery> = Vec::new();
                for _ in 0..10000 {
                    vec.push(Elem::random(&mut rng));
                }
                vec
            },
            |x| x.iter().fold(Montgomery::new(0), |x, y| x + *y),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("montgomery add-throughput", |b| {
        b.iter_batched(
            || {
                (
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                )
            },
            |(mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h, mut i, mut j): (Montgomery, Montgomery, Montgomery, Montgomery, Montgomery, Montgomery, Montgomery, Montgomery, Montgomery, Montgomery)| {
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

    c.bench_function("montgomery sub-latency", |b| {
        b.iter_batched(
            || {
                let mut vec: Vec<Montgomery> = Vec::new();
                for _ in 0..10000 {
                    vec.push(Elem::random(&mut rng));
                }
                vec
            },
            |x| x.iter().fold(Montgomery::new(0), |x, y| x - *y),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("montgomery sub-throughput", |b| {
        b.iter_batched(
            || {
                (
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                    Elem::random(&mut rng),
                )
            },
            |(mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h, mut i, mut j): (Montgomery, Montgomery, Montgomery, Montgomery, Montgomery, Montgomery, Montgomery, Montgomery, Montgomery, Montgomery)| {
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

    c.bench_function("montgomery mul-throughput", |b| {
        b.iter_batched(
            || (Elem::random(&mut rng),
            Elem::random(&mut rng),
            Elem::random(&mut rng),
            Elem::random(&mut rng),),
            |(mut x, mut y, mut z, mut w) : (Montgomery, Montgomery, Montgomery, Montgomery)| {
                for _ in 0..25 {
                    (x, y, z, w) = (x * y, y * z, z * w, w * x);
                }
                (x, y, z, w)
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function("montgomery mul-latency", |b| {
        b.iter_batched(
            || Elem::random(&mut rng),
            |mut x: Montgomery| {
                for _ in 0..100 {
                    x = x * x;
                }
                x
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function("montgomery sqr-throughput>", |b| {
        b.iter_batched(
            || (Elem::random(&mut rng),
            Elem::random(&mut rng),
            Elem::random(&mut rng),
            Elem::random(&mut rng),),
            |(mut x, mut y, mut z, mut w) : (Montgomery, Montgomery, Montgomery, Montgomery)| {
                for _ in 0..25 {
                    (x, y, z, w) = (x.clone() * x.clone(), y.clone() * y.clone(), z.clone() * z.clone(), w.clone() * w.clone());
                }
                (x, y, z, w)
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function("montgomery sqr-latency", |b| {
        b.iter_batched(
            || Elem::random(&mut rng),
            |mut x: Montgomery| {
                for _ in 0..100 {
                    x = x.clone() * x.clone();
                }
                x
            },
            BatchSize::SmallInput,
        )
    });

}

pub fn benchmark_canonical_ext(c: &mut Criterion)
{
    let mut rng = rand::rngs::SmallRng::seed_from_u64(2);
    c.bench_function("canonical_ext square", |b| {
        let x: CanonicalExt = Elem::random(&mut rng);
        b.iter(|| black_box(black_box(x).mul(black_box(x))))
    });

    c.bench_function("canonical_ext inv", |b| {
        let x: CanonicalExt = Elem::random(&mut rng);
        b.iter(|| black_box(black_box(x).inv()))
    });

    c.bench_function("canonical_ext mul", |b| {
        let x: CanonicalExt = Elem::random(&mut rng);
        let y: CanonicalExt = Elem::random(&mut rng);
        b.iter(|| black_box(black_box(x).mul(black_box(y))))
    });

    c.bench_function("canonical_ext add", |b| {
        let x: CanonicalExt = Elem::random(&mut rng);
        let y: CanonicalExt = Elem::random(&mut rng);
        b.iter(|| black_box(black_box(x).add(black_box(y))))
    });

}

pub fn benchmark_montgomery_ext(c: &mut Criterion)
{
    let mut rng = rand::rngs::SmallRng::seed_from_u64(2);
    c.bench_function("montgomery_ext square", |b| {
        let x: MontgomeryExt = Elem::random(&mut rng);
        b.iter(|| black_box(black_box(x).mul(black_box(x))))
    });

    c.bench_function("montgomery_ext inv", |b| {
        let x: MontgomeryExt = Elem::random(&mut rng);
        b.iter(|| black_box(black_box(x).inv()))
    });

    c.bench_function("montgomery_ext mul", |b| {
        let x: MontgomeryExt = Elem::random(&mut rng);
        let y: MontgomeryExt = Elem::random(&mut rng);
        b.iter(|| black_box(black_box(x).mul(black_box(y))))
    });

    c.bench_function("montgomery_ext add", |b| {
        let x: MontgomeryExt = Elem::random(&mut rng);
        let y: MontgomeryExt = Elem::random(&mut rng);
        b.iter(|| black_box(black_box(x).add(black_box(y))))
    });

}

fn bench_babybear(c: &mut Criterion) {
    benchmark_canonical(c);
    benchmark_montgomery(c);
    benchmark_canonical_ext(c);
    benchmark_montgomery_ext(c);
}

criterion_group!(arithmetic, bench_babybear);
criterion_main!(arithmetic);
