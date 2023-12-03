use std::ops::{Mul, Add};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
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
