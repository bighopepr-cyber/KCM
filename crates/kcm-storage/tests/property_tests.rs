#![allow(clippy::unwrap_used, clippy::panic)]

use kcm_core::types::*;
use kcm_storage::compress::{Compressor, Lz4Compressor, ZstdCompressor};
use kcm_storage::robin_hood::RobinHoodMap;
use kcm_storage::wal::WriteAheadLog;
use proptest::prelude::*;

proptest! {
    #[test]
    fn fuzz_wal_append_replay_roundtrip(
        facts in prop::collection::vec(
            (0u32..1_000, 0u8..=255, 0u32..1_000, 0.0f64..=1.0),
            1..50
        ),
    ) {
        let dir = tempfile::tempdir().unwrap();
        let wal_path = dir.path().join("test.wal");
        let wal = WriteAheadLog::new(&wal_path).unwrap();

        let mut expected_facts = Vec::new();
        for (s, p, o, c) in &facts {
            let fact = Fact::new(SubjectID(*s), PredicateID(*p), ObjectID(*o), *c).unwrap();
            wal.append_fact(&fact).unwrap();
            expected_facts.push(fact);
        }
        wal.flush_buffer().unwrap();

        let mut replayed = Vec::new();
        wal.replay(|entry| {
            if let Some(fact) = entry.to_fact() {
                replayed.push(fact);
            }
            Ok(())
        }).unwrap();

        prop_assert_eq!(replayed.len(), expected_facts.len());
        for (original, replayed) in expected_facts.iter().zip(replayed.iter()) {
            prop_assert_eq!(original.subject, replayed.subject);
            prop_assert_eq!(original.predicate, replayed.predicate);
            prop_assert_eq!(original.object, replayed.object);
            prop_assert!((original.confidence - replayed.confidence).abs() < 1e-10);
        }
    }

    #[test]
    fn fuzz_wal_delete_replay(
        row_ids in prop::collection::vec(0u64..10_000, 1..30),
    ) {
        let dir = tempfile::tempdir().unwrap();
        let wal_path = dir.path().join("test_delete.wal");
        let wal = WriteAheadLog::new(&wal_path).unwrap();

        for &rid in &row_ids {
            wal.append_delete(rid).unwrap();
        }
        wal.flush_buffer().unwrap();

        let mut deleted_ids = Vec::new();
        wal.replay(|entry| {
            if let kcm_storage::wal::WALEntry::Delete { row_id } = entry {
                deleted_ids.push(row_id);
            }
            Ok(())
        }).unwrap();

        prop_assert_eq!(deleted_ids, row_ids);
    }

    #[test]
    fn fuzz_zstd_compress_decompress_roundtrip(
        data in prop::collection::vec(0u8..=255, 0..10_000),
    ) {
        let compressor = ZstdCompressor::default_level();
        let compressed = compressor.compress(&data).unwrap();
        let decompressed = compressor.decompress(&compressed, data.len()).unwrap();
        prop_assert_eq!(data, decompressed);
    }

    #[test]
    fn fuzz_lz4_compress_decompress_roundtrip(
        data in prop::collection::vec(0u8..=255, 0..10_000),
    ) {
        let compressor = Lz4Compressor::default_level();
        let compressed = compressor.compress(&data).unwrap();
        let decompressed = compressor.decompress(&compressed, data.len()).unwrap();
        prop_assert_eq!(data, decompressed);
    }

    #[test]
    fn fuzz_zstd_empty_data(
        data in prop::collection::vec(0u8..=255, 0..100),
    ) {
        let compressor = ZstdCompressor::default_level();
        let compressed = compressor.compress(&data).unwrap();
        let decompressed = compressor.decompress(&compressed, data.len()).unwrap();
        prop_assert_eq!(data, decompressed);
    }

    #[test]
    fn fuzz_robin_hood_insert_get_consistency(
        entries in prop::collection::vec(
            ("[a-z]{1,8}", 0i32..1_000),
            0..200
        ),
    ) {
        let mut map = RobinHoodMap::new();
        let mut last_value_per_key = std::collections::HashMap::new();

        for (key, value) in &entries {
            map.insert(key.clone(), *value);
            last_value_per_key.insert(key.clone(), *value);
        }

        prop_assert_eq!(map.len(), last_value_per_key.len());

        for (key, expected_value) in &last_value_per_key {
            let result = map.get(key);
            prop_assert_eq!(result, Some(expected_value));
        }
    }

    #[test]
    fn fuzz_robin_hood_overwrite_returns_old_value(
        entries in prop::collection::vec(
            ("[a-z]{1,5}", 0i32..1_000),
            1..100
        ),
    ) {
        let mut map = RobinHoodMap::new();
        let mut last_value_per_key = std::collections::HashMap::new();

        for (key, value) in &entries {
            let old = map.insert(key.clone(), *value);
            let stored = map.get(key);
            prop_assert_eq!(stored, Some(value));

            if let Some(prev) = old {
                let expected_old = last_value_per_key.get(key).unwrap();
                prop_assert_eq!(prev, *expected_old);
            }
            last_value_per_key.insert(key.clone(), *value);
        }
    }

    #[test]
    fn fuzz_robin_hood_get_nonexistent_returns_none(
        existing in prop::collection::vec("[a-z]{1,5}", 0..50),
        queries in prop::collection::vec("[0-9]{1,5}", 0..50),
    ) {
        let mut map = RobinHoodMap::new();
        for key in &existing {
            map.insert(key.clone(), 1i32);
        }

        for key in &queries {
            if !existing.contains(key) {
                prop_assert_eq!(map.get(key.as_str()), None);
            }
        }
    }

    #[test]
    fn fuzz_robin_hood_contains_key_matches_get(
        keys in prop::collection::vec("[a-z]{1,8}", 0..100),
    ) {
        let mut map = RobinHoodMap::new();
        for key in &keys {
            map.insert(key.clone(), 1i32);
        }

        for key in &keys {
            prop_assert_eq!(map.contains_key(key.as_str()), map.get(key.as_str()).is_some());
        }
    }

    #[test]
    fn fuzz_bitmap_count_ones_accuracy(
        indices in prop::collection::vec(0usize..10_000, 0..500),
    ) {
        let mut bitmap = kcm_core::bitmap::Bitmap::new(10_000);
        let mut unique = std::collections::HashSet::new();
        for &idx in &indices {
            bitmap.set(idx);
            unique.insert(idx);
        }
        prop_assert_eq!(bitmap.count_ones(), unique.len());
    }

    #[test]
    fn fuzz_bitmap_not_inplace_complement(
        indices in prop::collection::vec(0usize..1_000, 0..200),
    ) {
        let mut bitmap = kcm_core::bitmap::Bitmap::new(1_000);
        for &idx in &indices {
            bitmap.set(idx);
        }

        bitmap.not_inplace();

        for &idx in &indices {
            prop_assert!(!bitmap.get(idx), "Cleared bit {} should be 0 after NOT", idx);
        }

        for idx in 0..1_000 {
            if !indices.contains(&idx) {
                prop_assert!(bitmap.get(idx), "Unset bit {} should be 1 after NOT", idx);
            }
        }
    }

    #[test]
    fn fuzz_dictionary_encode_decode_roundtrip(
        values in prop::collection::vec("[a-z]{1,20}", 0..200),
    ) {
        let mut dict = kcm_core::dictionary::Dictionary::new();
        let mut ids = Vec::new();
        for value in &values {
            let id = dict.insert(value).unwrap();
            ids.push(id);
        }

        for (value, id) in values.iter().zip(ids.iter()) {
            let retrieved = dict.get(*id);
            prop_assert_eq!(retrieved, Some(value.as_str()));
        }
    }
}
