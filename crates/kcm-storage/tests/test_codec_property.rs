#![allow(clippy::unwrap_used, clippy::panic)]
use kcm_storage::compress::{Compressor, Lz4Compressor, RleCompressor, ZstdCompressor};

#[test]
fn test_zstd_roundtrip_various_sizes() {
    let compressor = ZstdCompressor::default_level();
    for n in [100, 1_000, 10_000, 100_000] {
        let data: Vec<u8> = (0..n).map(|i| (i % 256) as u8).collect();
        let compressed = compressor.compress(&data).unwrap();
        let decompressed = compressor.decompress(&compressed, data.len()).unwrap();
        assert_eq!(data, decompressed, "Zstd failed for n={}", n);
    }
}

#[test]
fn test_zstd_repetitive_high_ratio() {
    let compressor = ZstdCompressor::default_level();
    let data = vec![42u8; 100_000];
    let compressed = compressor.compress(&data).unwrap();
    let ratio = data.len() as f64 / compressed.len() as f64;
    assert!(ratio > 10.0, "Zstd ratio should be > 10x, got {}", ratio);
    let decompressed = compressor.decompress(&compressed, data.len()).unwrap();
    assert_eq!(data, decompressed);
}

#[test]
fn test_zstd_large_data() {
    let compressor = ZstdCompressor::default_level();
    let data: Vec<u8> = (0..100_000).map(|i| (i * 37 % 256) as u8).collect();
    let compressed = compressor.compress(&data).unwrap();
    let decompressed = compressor.decompress(&compressed, data.len()).unwrap();
    assert_eq!(data, decompressed);
}

#[test]
fn test_lz4_roundtrip_various_sizes() {
    let compressor = Lz4Compressor::default_level();
    for n in [100, 1_000, 10_000, 100_000] {
        let data: Vec<u8> = (0..n).map(|i| (i % 256) as u8).collect();
        let compressed = compressor.compress(&data).unwrap();
        let decompressed = compressor.decompress(&compressed, data.len()).unwrap();
        assert_eq!(data, decompressed, "LZ4 failed for n={}", n);
    }
}

#[test]
fn test_lz4_repetitive_data() {
    let compressor = Lz4Compressor::default_level();
    let data = vec![99u8; 100_000];
    let compressed = compressor.compress(&data).unwrap();
    let decompressed = compressor.decompress(&compressed, data.len()).unwrap();
    assert_eq!(data, decompressed);
}

#[test]
fn test_lz4_random_data() {
    let compressor = Lz4Compressor::default_level();
    let data: Vec<u8> = (0..100_000).map(|i| (i * 47 % 256) as u8).collect();
    let compressed = compressor.compress(&data).unwrap();
    let decompressed = compressor.decompress(&compressed, data.len()).unwrap();
    assert_eq!(data, decompressed);
}

#[test]
fn test_rle_compressor_roundtrip() {
    let rle = RleCompressor;
    let data: Vec<u8> = (0..10_000).map(|i| (i % 10) as u8).collect();
    let compressed = rle.compress(&data).unwrap();
    let decompressed = rle.decompress(&compressed, data.len()).unwrap();
    assert_eq!(data, decompressed);
}

#[test]
fn test_compression_ratio_zstd_vs_lz4() {
    let data = vec![0u8; 100_000];
    let zstd = ZstdCompressor::default_level();
    let lz4 = Lz4Compressor::default_level();
    let zstd_ratio = data.len() as f64 / zstd.compress(&data).unwrap().len() as f64;
    let lz4_ratio = data.len() as f64 / lz4.compress(&data).unwrap().len() as f64;
    assert!(
        zstd_ratio > lz4_ratio,
        "Zstd should compress better than LZ4 for repetitive data"
    );
}

#[test]
fn test_zstd_level_comparison() {
    let data: Vec<u8> = (0..10_000).map(|i| (i % 256) as u8).collect();
    let low = ZstdCompressor::new(1).compress(&data).unwrap();
    let high = ZstdCompressor::new(10).compress(&data).unwrap();
    assert!(
        high.len() <= low.len(),
        "Higher compression level should produce smaller output"
    );
}
