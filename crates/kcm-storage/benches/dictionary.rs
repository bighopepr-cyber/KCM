#![allow(clippy::unwrap_used, clippy::panic)]

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use kcm_storage::dict_cache::DictionaryCache;
use kcm_storage::dict_codec::DictionaryCodec;
use rayon::prelude::*;

fn generate_strings(count: usize, prefix: &str) -> Vec<String> {
    (0..count).map(|i| format!("{}_{:06}", prefix, i)).collect()
}

fn bench_dictionary_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("dictionary_encode");

    for &size in &[1_000, 10_000, 100_000] {
        let strings = generate_strings(size, "entity");
        let mut cache = DictionaryCache::with_capacity(size);

        group.bench_with_input(
            BenchmarkId::new("robin_hood_encode", size),
            &strings,
            |b, strings| {
                b.iter(|| {
                    for s in strings {
                        cache.encode(s);
                    }
                })
            },
        );

        let codec = DictionaryCodec::with_capacity(size);
        group.bench_with_input(
            BenchmarkId::new("ahash_codec_encode", size),
            &strings,
            |b, strings| {
                b.iter(|| {
                    for s in strings {
                        codec.encode(s);
                    }
                })
            },
        );
    }

    group.finish();
}

fn bench_dictionary_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("dictionary_lookup");

    for &size in &[1_000, 10_000, 100_000] {
        let strings = generate_strings(size, "entity");
        let mut cache = DictionaryCache::with_capacity(size);
        for s in &strings {
            cache.encode(s);
        }

        group.bench_with_input(
            BenchmarkId::new("robin_hood_single_lookup", size),
            &strings,
            |b, strings| {
                b.iter(|| {
                    for s in strings {
                        cache.lookup(s);
                    }
                })
            },
        );

        let codec = DictionaryCodec::with_capacity(size);
        for s in &strings {
            codec.encode(s);
        }

        group.bench_with_input(
            BenchmarkId::new("ahash_codec_single_lookup", size),
            &strings,
            |b, strings| {
                b.iter(|| {
                    for s in strings {
                        codec.lookup(s);
                    }
                })
            },
        );
    }

    group.finish();
}

fn bench_dictionary_batch_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("dictionary_batch_lookup");

    for &size in &[1_000, 10_000, 100_000] {
        let strings = generate_strings(size, "entity");
        let lookup_strings: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();

        let mut cache = DictionaryCache::with_capacity(size);
        for s in &strings {
            cache.encode(s);
        }

        group.bench_with_input(
            BenchmarkId::new("batch_prefetch", size),
            &lookup_strings,
            |b, strings| {
                let mut results = vec![None; strings.len()];
                b.iter(|| {
                    cache.lookup_batch_prefetch(strings, &mut results);
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("batch_simd", size),
            &lookup_strings,
            |b, strings| {
                let mut results = vec![None; strings.len()];
                b.iter(|| {
                    cache.lookup_batch_simd(strings, &mut results);
                })
            },
        );

        let codec = DictionaryCodec::with_capacity(size);
        for s in &strings {
            codec.encode(s);
        }

        group.bench_with_input(
            BenchmarkId::new("codec_batch", size),
            &lookup_strings,
            |b, strings| {
                let mut results = vec![None; strings.len()];
                b.iter(|| {
                    codec.lookup_batch(strings, &mut results);
                })
            },
        );
    }

    group.finish();
}

fn bench_dictionary_parallel_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("dictionary_parallel_lookup");

    for &size in &[1_000, 10_000, 100_000] {
        let strings = generate_strings(size, "entity");
        let lookup_strings: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();

        let codec = DictionaryCodec::with_capacity(size);
        for s in &strings {
            codec.encode(s);
        }

        group.bench_with_input(
            BenchmarkId::new("sequential_lookup", size),
            &lookup_strings,
            |b, strings| {
                b.iter(|| {
                    let mut results = vec![None; strings.len()];
                    for (i, s) in strings.iter().enumerate() {
                        results[i] = codec.lookup(s);
                    }
                    results
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("parallel_lookup_4_threads", size),
            &lookup_strings,
            |b, strings| {
                let codec_clone = codec.clone();
                b.iter(|| {
                    let chunk_size = strings.len().div_ceil(4);
                    let results: Vec<Vec<Option<u32>>> = strings
                        .par_chunks(chunk_size)
                        .map(|chunk| {
                            let mut local_results = vec![None; chunk.len()];
                            for (i, s) in chunk.iter().enumerate() {
                                local_results[i] = codec_clone.lookup(s);
                            }
                            local_results
                        })
                        .collect();
                    results.concat()
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("parallel_lookup_rayon", size),
            &lookup_strings,
            |b, strings| {
                let codec_clone = codec.clone();
                b.iter(|| {
                    let results: Vec<Option<u32>> =
                        strings.par_iter().map(|s| codec_clone.lookup(s)).collect();
                    results
                })
            },
        );
    }

    group.finish();
}

fn bench_dictionary_warm_up(c: &mut Criterion) {
    let mut group = c.benchmark_group("dictionary_warm_up");

    for &size in &[1_000, 10_000, 100_000] {
        let strings = generate_strings(size, "entity");

        group.bench_with_input(BenchmarkId::new("warm_up", size), &strings, |b, strings| {
            b.iter_batched(
                DictionaryCache::new,
                |mut cache| {
                    let refs: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();
                    cache.warm_up(&refs);
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}

fn bench_robin_hood_vs_ahash(c: &mut Criterion) {
    let mut group = c.benchmark_group("robin_hood_vs_ahash");

    let size = 10_000;
    let strings = generate_strings(size, "entity");
    let lookup_strings: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();

    let mut rh_map = kcm_storage::RobinHoodMap::new();
    for (i, s) in strings.iter().enumerate() {
        rh_map.insert(s.clone(), i as u32);
    }

    let mut ah_map = ahash::AHashMap::new();
    for (i, s) in strings.iter().enumerate() {
        ah_map.insert(s.clone(), i as u32);
    }

    group.bench_function(BenchmarkId::new("robin_hood_lookup", size), |b| {
        b.iter(|| {
            for s in &lookup_strings {
                rh_map.get(*s);
            }
        })
    });

    group.bench_function(BenchmarkId::new("ahash_lookup", size), |b| {
        b.iter(|| {
            for s in &lookup_strings {
                ah_map.get(*s);
            }
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_dictionary_encode,
    bench_dictionary_lookup,
    bench_dictionary_batch_lookup,
    bench_dictionary_parallel_lookup,
    bench_dictionary_warm_up,
    bench_robin_hood_vs_ahash,
);
criterion_main!(benches);
