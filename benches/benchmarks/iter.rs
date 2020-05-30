use criterion::{black_box, BatchSize, BenchmarkId, Criterion, Throughput};
use std::collections::{BTreeSet, BinaryHeap, HashSet};

use crate::{rand::Rand, util};
use byte_set::ByteSet;

pub fn benches() {
    let mut criterion = Criterion::default().configure_from_args();
    let mut group = criterion.benchmark_group("Iter");

    let mut rng = rand::thread_rng();

    for &size in util::SIZES {
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function(BenchmarkId::new("ByteSet", size), |b| {
            b.iter_batched(
                || black_box(ByteSet::rand_len(size, &mut rng)),
                |byte_set| {
                    for byte in byte_set {
                        black_box(byte);
                    }
                },
                BatchSize::SmallInput,
            )
        });

        let range_inclusive = black_box(0u8..=util::saturating_cast(size));
        group.bench_with_input(
            BenchmarkId::new("RangeInclusive<u8>", size),
            &range_inclusive,
            |b, range_inclusive| {
                b.iter(|| {
                    for byte in range_inclusive.clone() {
                        black_box(byte);
                    }
                })
            },
        );

        group.bench_function(BenchmarkId::new("HashSet<u8>", size), |b| {
            b.iter_batched_ref(
                || black_box(HashSet::<u8>::rand_len(size, &mut rng)),
                |hash_set| {
                    for &byte in hash_set.iter() {
                        black_box(byte);
                    }
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("BTreeSet<u8>", size), |b| {
            b.iter_batched_ref(
                || black_box(BTreeSet::<u8>::rand_len(size, &mut rng)),
                |byte_set| {
                    for &byte in byte_set.iter() {
                        black_box(byte);
                    }
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("Vec<u8>", size), |b| {
            b.iter_batched_ref(
                || black_box(Vec::<u8>::rand_len(size, &mut rng)),
                |byte_set| {
                    for &byte in byte_set.iter() {
                        black_box(byte);
                    }
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("BinaryHeap<u8>", size), |b| {
            b.iter_batched_ref(
                || black_box(BinaryHeap::<u8>::rand_len(size, &mut rng)),
                |binary_heap| {
                    for &byte in binary_heap.iter() {
                        black_box(byte);
                    }
                },
                BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}
