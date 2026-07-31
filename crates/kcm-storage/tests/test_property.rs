use kcm_core::types::*;
use kcm_storage::codec::{Codec, DeltaCodec, GorillaCodec, RleCodec};
use kcm_storage::compress::{Compressor, Lz4Compressor, ZstdCompressor};
use kcm_storage::wal::WriteAheadLog;
use std::sync::Arc;

#[test]
fn test_delta_codec_roundtrip_random() {
    let codec = DeltaCodec;
    let data: Vec<i64> = (0..1000).map(|i| (i * 7 % 500) as i64).collect();
    let encoded = codec.encode(&data).unwrap();
    let decoded = codec.decode(&encoded, data.len()).unwrap();
    assert_eq!(data, decoded);
}

#[test]
fn test_delta_codec_single_value() {
    let codec = DeltaCodec;
    let data = vec![42i64];
    let encoded = codec.encode(&data).unwrap();
    let decoded = codec.decode(&encoded, 1).unwrap();
    assert_eq!(data, decoded);
}

#[test]
fn test_delta_codec_negative_values() {
    let codec = DeltaCodec;
    let data = vec![-100i64, 50, -200, 300, 0];
    let encoded = codec.encode(&data).unwrap();
    let decoded = codec.decode(&encoded, data.len()).unwrap();
    assert_eq!(data, decoded);
}

#[test]
fn test_delta_codec_empty() {
    let codec = DeltaCodec;
    let data: Vec<i64> = vec![];
    let encoded = codec.encode(&data).unwrap();
    let decoded = codec.decode(&encoded, 0).unwrap();
    assert!(decoded.is_empty());
}

#[test]
fn test_rle_codec_repeated() {
    let codec = RleCodec;
    let data = vec![1u8; 1000];
    let encoded = codec.encode(&data).unwrap();
    let decoded = codec.decode(&encoded, 1000).unwrap();
    assert_eq!(data, decoded);
}

#[test]
fn test_rle_codec_alternating() {
    let codec = RleCodec;
    let data: Vec<u8> = (0..200).map(|i| (i % 2) as u8).collect();
    let encoded = codec.encode(&data).unwrap();
    let decoded = codec.decode(&encoded, 200).unwrap();
    assert_eq!(data, decoded);
}

#[test]
fn test_rle_codec_single() {
    let codec = RleCodec;
    let data = vec![42u8];
    let encoded = codec.encode(&data).unwrap();
    let decoded = codec.decode(&encoded, 1).unwrap();
    assert_eq!(data, decoded);
}

#[test]
fn test_gorilla_codec_slowly_changing() {
    let codec = GorillaCodec;
    let data: Vec<f64> = (0..500).map(|i| 1.0 + i as f64 * 0.001).collect();
    let encoded = codec.encode(&data).unwrap();
    let decoded = codec.decode(&encoded, 500).unwrap();
    assert_eq!(data.len(), decoded.len());
    for (a, b) in data.iter().zip(decoded.iter()) {
        assert!((a - b).abs() < 1e-10);
    }
}

#[test]
fn test_gorilla_codec_single_value() {
    let codec = GorillaCodec;
    let data = vec![1.5f64];
    let encoded = codec.encode(&data).unwrap();
    let decoded = codec.decode(&encoded, 1).unwrap();
    assert_eq!(data.len(), decoded.len());
    assert!((data[0] - decoded[0]).abs() < 1e-10);
}

#[test]
fn test_zstd_roundtrip() {
    let compressor = ZstdCompressor::default_level();
    let data: Vec<u8> = (0..10000).map(|i| (i % 256) as u8).collect();
    let compressed = compressor.compress(&data).unwrap();
    let decompressed = compressor.decompress(&compressed, data.len()).unwrap();
    assert_eq!(data, decompressed);
}

#[test]
fn test_zstd_compresses_repetitive() {
    let compressor = ZstdCompressor::default_level();
    let data = vec![42u8; 10000];
    let compressed = compressor.compress(&data).unwrap();
    assert!(compressed.len() < data.len());
    let decompressed = compressor.decompress(&compressed, data.len()).unwrap();
    assert_eq!(data, decompressed);
}

#[test]
fn test_lz4_roundtrip() {
    let compressor = Lz4Compressor::default_level();
    let data: Vec<u8> = (0..10000).map(|i| (i % 256) as u8).collect();
    let compressed = compressor.compress(&data).unwrap();
    let decompressed = compressor.decompress(&compressed, data.len()).unwrap();
    assert_eq!(data, decompressed);
}

#[test]
fn test_wal_concurrent_append() {
    let dir = tempfile::tempdir().unwrap();
    let wal_path = dir.path().join("test.wal");
    let wal = Arc::new(WriteAheadLog::new(&wal_path).unwrap());

    let mut handles = Vec::new();
    for t in 0..4 {
        let wal = wal.clone();
        handles.push(std::thread::spawn(move || {
            for i in 0..100u32 {
                let fact =
                    Fact::new(SubjectID(t * 100 + i), PredicateID(0), ObjectID(i), 0.9).unwrap();
                wal.append_fact(&fact).unwrap();
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    wal.flush_buffer().unwrap();

    let mut count = 0;
    wal.replay(|_entry| {
        count += 1;
        Ok(())
    })
    .unwrap();
    assert_eq!(count, 400);
}

#[test]
fn test_wal_checksum_validation() {
    let dir = tempfile::tempdir().unwrap();
    let wal_path = dir.path().join("test.wal");
    let wal = WriteAheadLog::new(&wal_path).unwrap();

    for i in 0..5u32 {
        let fact = Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), 0.9).unwrap();
        wal.append_fact(&fact).unwrap();
    }
    wal.flush_buffer().unwrap();

    let mut data = std::fs::read(&wal_path).unwrap();
    assert!(
        data.len() > 40,
        "WAL data should be > 40 bytes, got {}",
        data.len()
    );
    data[40] = data[40].wrapping_add(1);
    std::fs::write(&wal_path, &data).unwrap();

    let wal2 = WriteAheadLog::new(&wal_path).unwrap();
    let result = wal2.replay(|_| Ok(()));
    assert!(result.is_err());
}
