use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use kcm_storage::compress::{Compressor, Lz4Compressor, RleCompressor, ZstdCompressor};

fn bench_compression(c: &mut Criterion) {
    let mut group = c.benchmark_group("compression");

    let data_sizes = [1_000, 10_000, 100_000];
    for &size in &data_sizes {
        let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();

        group.bench_with_input(
            BenchmarkId::new("zstd_compress", size),
            &data,
            |b, data| b.iter(|| ZstdCompressor::default_level().compress(data).unwrap()),
        );

        group.bench_with_input(
            BenchmarkId::new("lz4_compress", size),
            &data,
            |b, data| b.iter(|| Lz4Compressor::default_level().compress(data).unwrap()),
        );

        group.bench_with_input(
            BenchmarkId::new("rle_compress", size),
            &data,
            |b, data| b.iter(|| RleCompressor::default_level().compress(data).unwrap()),
        );
    }

    group.finish();
}

fn bench_compression_ratio(c: &mut Criterion) {
    let mut group = c.benchmark_group("compression_ratio");

    for &size in &[10_000, 100_000] {
        let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();

        let zstd = ZstdCompressor::default_level().compress(&data).unwrap();
        let lz4 = Lz4Compressor::default_level().compress(&data).unwrap();
        let rle = RleCompressor::default_level().compress(&data).unwrap();

        group.bench_function(BenchmarkId::new("zstd_ratio", size), |b| {
            b.iter(|| zstd.len() as f64 / data.len() as f64)
        });
        group.bench_function(BenchmarkId::new("lz4_ratio", size), |b| {
            b.iter(|| lz4.len() as f64 / data.len() as f64)
        });
        group.bench_function(BenchmarkId::new("rle_ratio", size), |b| {
            b.iter(|| rle.len() as f64 / data.len() as f64)
        });
    }

    group.finish();
}

criterion_group!(benches, bench_compression, bench_compression_ratio);
criterion_main!(benches);
