#![feature(portable_simd)]
#![feature(is_sorted)]
#![feature(sort_floats)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simd_itertools::FilterSimd;
use std::fmt::Debug;
use std::simd::cmp::SimdPartialOrd;
use std::simd::Mask;
use std::simd::{Simd, SimdElement};
use std::time::Duration;

fn benchmark_contains<'a, T: 'static + Copy + PartialEq + Default + Debug>(
    _c: &mut Criterion,
    name: &str,
    len: usize,
) where
    T: SimdElement + std::cmp::PartialEq + TryFrom<i32> + Debug + PartialOrd,
    Simd<T, 8>: SimdPartialOrd<Mask = Mask<T::Mask, 8>>,
    <T as TryFrom<i32>>::Error: Debug,
{
    let v1 = vec![T::default(); len];
    let needle: T = 55.try_into().unwrap();

    let mut group = Criterion::default()
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(1));
    group.bench_function(&format!("SIMD filter {} {}", name, len), |b| {
        b.iter(|| black_box(v1.iter().filter_simd_lt(needle)))
    });
    group.bench_function(&format!("Scalar filter {} {}", name, len), |b| {
        b.iter(|| black_box(v1.iter().filter(|x| **x < needle)))
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    for n in (0..200).map(|x| x * 10) {
        benchmark_contains::<u8>(c, "u8", n);
        benchmark_contains::<u16>(c, "u16", n);
        benchmark_contains::<u32>(c, "u32", n);
        benchmark_contains::<u64>(c, "u64", n);
        benchmark_contains::<u8>(c, "u8", n);
        benchmark_contains::<i8>(c, "i8", n);
        benchmark_contains::<u16>(c, "u16", n);
        benchmark_contains::<i16>(c, "i16", n);
        benchmark_contains::<i32>(c, "i32", n);
        benchmark_contains::<u64>(c, "u64", n);
        benchmark_contains::<i64>(c, "i64", n);
        benchmark_contains::<isize>(c, "isize", n);
        benchmark_contains::<usize>(c, "usize", n);
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
