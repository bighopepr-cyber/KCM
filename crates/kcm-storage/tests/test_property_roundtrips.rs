use kcm_core::types::*;
use kcm_storage::codec::{Codec, DeltaCodec, GorillaCodec, RleCodec};
use kcm_storage::column::Schema;
use kcm_storage::compress::{Compressor, Lz4Compressor, ZstdCompressor};
use kcm_storage::file_format::DatabaseFile;

#[test]
fn test_delta_codec_large_dataset() {
    let codec = DeltaCodec;
    for n in [10, 100, 1000, 10_000] {
        let data: Vec<i64> = (0..n).map(|i| (i as i64 * 7) % 5000 - 2500).collect();
        let encoded = codec.encode(&data).unwrap();
        let decoded = codec.decode(&encoded, data.len()).unwrap();
        assert_eq!(data, decoded, "DeltaCodec roundtrip failed for n={}", n);
    }
}

#[test]
fn test_delta_codec_extreme_values() {
    let codec = DeltaCodec;
    let data = vec![i64::MAX, i64::MAX - 1, 0, -1, 1, 0];
    let encoded = codec.encode(&data).unwrap();
    let decoded = codec.decode(&encoded, data.len()).unwrap();
    assert_eq!(data, decoded);
}

#[test]
fn test_rle_codec_uniform_data() {
    let codec = RleCodec;
    let data = vec![42u8; 10_000];
    let encoded = codec.encode(&data).unwrap();
    assert!(
        encoded.len() < data.len(),
        "RLE should compress uniform data"
    );
    let decoded = codec.decode(&encoded, 10_000).unwrap();
    assert_eq!(data, decoded);
}

#[test]
fn test_rle_codec_random_data() {
    let codec = RleCodec;
    let data: Vec<u8> = (0..10_000).map(|i| (i * 37 % 256) as u8).collect();
    let encoded = codec.encode(&data).unwrap();
    let decoded = codec.decode(&encoded, 10_000).unwrap();
    assert_eq!(data, decoded);
}

#[test]
fn test_gorilla_codec_steady_floats() {
    let codec = GorillaCodec;
    let data: Vec<f64> = (0..5000).map(|i| 1.0 + i as f64 * 0.0001).collect();
    let encoded = codec.encode(&data).unwrap();
    let decoded = codec.decode(&encoded, 5000).unwrap();
    assert_eq!(data.len(), decoded.len());
    for (a, b) in data.iter().zip(decoded.iter()) {
        assert!((a - b).abs() < 1e-10, "Gorilla mismatch: {} vs {}", a, b);
    }
}

#[test]
fn test_gorilla_codec_volatile_floats() {
    let codec = GorillaCodec;
    let data = vec![1.0f64, 100.0, 0.001, 999.9, 0.5];
    let encoded = codec.encode(&data).unwrap();
    let decoded = codec.decode(&encoded, data.len()).unwrap();
    for (a, b) in data.iter().zip(decoded.iter()) {
        assert!((a - b).abs() < 1e-10);
    }
}

#[test]
fn test_zstd_roundtrip_various_sizes() {
    let compressor = ZstdCompressor::default_level();
    for n in [100, 1_000, 10_000, 100_000] {
        let data: Vec<u8> = (0..n).map(|i| (i % 256) as u8).collect();
        let compressed = compressor.compress(&data).unwrap();
        let decompressed = compressor.decompress(&compressed, data.len()).unwrap();
        assert_eq!(data, decompressed, "Zstd roundtrip failed for n={}", n);
    }
}

#[test]
fn test_lz4_roundtrip_various_sizes() {
    let compressor = Lz4Compressor::default_level();
    for n in [100, 1_000, 10_000, 100_000] {
        let data: Vec<u8> = (0..n).map(|i| (i % 256) as u8).collect();
        let compressed = compressor.compress(&data).unwrap();
        let decompressed = compressor.decompress(&compressed, data.len()).unwrap();
        assert_eq!(data, decompressed, "LZ4 roundtrip failed for n={}", n);
    }
}

#[test]
fn test_file_format_roundtrip_various_sizes() {
    for n in [1, 5, 50, 500] {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.kcm");
        let mut schema = Schema::new(n).unwrap();
        for i in 0..n {
            schema
                .append_fact(
                    &Fact::new(
                        SubjectID(i as u32),
                        PredicateID((i % 10) as u8),
                        ObjectID((i * 3) as u32),
                        0.1 + (i as f64 * 0.001),
                    )
                    .unwrap(),
                )
                .unwrap();
        }
        DatabaseFile::save(&schema, &path).unwrap();
        let loaded = DatabaseFile::load(&path).unwrap();
        assert_eq!(schema.len(), loaded.len());
        for i in 0..n {
            let orig = schema.get_fact(i).unwrap();
            let rest = loaded.get_fact(i).unwrap();
            assert_eq!(orig.subject, rest.subject);
            assert_eq!(orig.predicate, rest.predicate);
            assert_eq!(orig.object, rest.object);
            assert!((orig.confidence - rest.confidence).abs() < 1e-10);
        }
    }
}

#[test]
fn test_codec_encode_decode_empty() {
    assert_eq!(DeltaCodec.encode(&Vec::<i64>::new()).unwrap().len(), 0);
    assert_eq!(RleCodec.encode(&Vec::<u8>::new()).unwrap().len(), 0);
    assert_eq!(GorillaCodec.encode(&Vec::<f64>::new()).unwrap().len(), 0);
}

#[test]
fn test_codec_encode_single_element() {
    let codec = DeltaCodec;
    let data = vec![42i64];
    let encoded = codec.encode(&data).unwrap();
    let decoded = codec.decode(&encoded, 1).unwrap();
    assert_eq!(data, decoded);

    let codec_rle = RleCodec;
    let data_rle = vec![7u8];
    let encoded = codec_rle.encode(&data_rle).unwrap();
    let decoded = codec_rle.decode(&encoded, 1).unwrap();
    assert_eq!(data_rle, decoded);
}

#[test]
fn test_zstd_compression_ratio() {
    let compressor = ZstdCompressor::default_level();
    let data = vec![0u8; 100_000];
    let compressed = compressor.compress(&data).unwrap();
    assert!(
        compressed.len() < data.len(),
        "Zstd should compress repetitive data"
    );
    let ratio = compressed.len() as f64 / data.len() as f64;
    assert!(
        ratio < 0.1,
        "Zstd ratio {} should be < 10% for zeros",
        ratio
    );
}

#[test]
fn test_database_file_checksum_integrity() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.kcm");
    let schema = Schema::new(50).unwrap();
    DatabaseFile::save(&schema, &path).unwrap();
    assert!(DatabaseFile::verify(&path).unwrap());

    let mut data = std::fs::read(&path).unwrap();
    let last = data.len() - 1;
    data[last] ^= 0xFF;
    std::fs::write(&path, &data).unwrap();
    assert!(!DatabaseFile::verify(&path).unwrap());
}
