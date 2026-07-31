use kcm_core::dictionary::SharedDictionary;
use kcm_core::types::*;
use kcm_storage::column::Schema;
use kcm_storage::index::{BitmapIndex, ZoneMap};

#[test]
fn test_schema_creation() {
    let schema = Schema::new(1000).unwrap();
    assert_eq!(schema.len(), 0);
    assert!(schema.is_empty());
}

#[test]
fn test_schema_append_single() {
    let mut schema = Schema::new(100).unwrap();
    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.95).unwrap();
    schema.append_fact(&fact).unwrap();
    assert_eq!(schema.len(), 1);
}

#[test]
fn test_schema_append_multiple() {
    let mut schema = Schema::new(1000).unwrap();
    for i in 0..100u32 {
        let fact = Fact::new(
            SubjectID(i),
            PredicateID(0),
            ObjectID(i * 2),
            0.5 + (i as f64 * 0.001),
        )
        .unwrap();
        schema.append_fact(&fact).unwrap();
    }
    assert_eq!(schema.len(), 100);
}

#[test]
fn test_schema_get_fact() {
    let mut schema = Schema::new(100).unwrap();
    let original = Fact::new(SubjectID(42), PredicateID(5), ObjectID(100), 0.85).unwrap();
    schema.append_fact(&original).unwrap();
    let retrieved = schema.get_fact(0).unwrap();
    assert_eq!(retrieved.subject, original.subject);
    assert_eq!(retrieved.predicate, original.predicate);
    assert_eq!(retrieved.object, original.object);
    assert!((retrieved.confidence - original.confidence).abs() < 1e-10);
}

#[test]
fn test_schema_get_fact_out_of_bounds() {
    let schema = Schema::new(100).unwrap();
    assert!(schema.get_fact(0).is_none());
    assert!(schema.get_fact(999).is_none());
}

#[test]
fn test_schema_column_independence() {
    let mut schema = Schema::new(100).unwrap();
    schema
        .append_fact(&Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap())
        .unwrap();
    schema
        .append_fact(&Fact::new(SubjectID(3), PredicateID(1), ObjectID(4), 0.8).unwrap())
        .unwrap();
    assert_eq!(schema.subject_col.get(0), Some(1u32));
    assert_eq!(schema.subject_col.get(1), Some(3u32));
    assert_eq!(schema.predicate_col.get(0), Some(0u8));
    assert_eq!(schema.predicate_col.get(1), Some(1u8));
}

#[test]
fn test_schema_delete_and_active_count() {
    let mut schema = Schema::new(100).unwrap();
    for i in 0..10u32 {
        schema
            .append_fact(&Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), 0.9).unwrap())
            .unwrap();
    }
    assert_eq!(schema.len(), 10);
    assert_eq!(schema.active_count(), 10);
    schema.delete_fact(0).unwrap();
    schema.delete_fact(5).unwrap();
    assert_eq!(schema.len(), 10);
    assert_eq!(schema.active_count(), 8);
    assert!(schema.get_fact(0).is_none());
    assert!(schema.get_fact(1).is_some());
}

#[test]
fn test_schema_update() {
    let mut schema = Schema::new(100).unwrap();
    schema
        .append_fact(&Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap())
        .unwrap();
    let updated = Fact::new(SubjectID(99), PredicateID(9), ObjectID(88), 0.1).unwrap();
    schema.update_fact(0, &updated).unwrap();
    let fact = schema.get_fact(0).unwrap();
    assert_eq!(fact.subject, SubjectID(99));
    assert_eq!(fact.predicate, PredicateID(9));
    assert!((fact.confidence - 0.1).abs() < 1e-10);
}

#[test]
fn test_schema_iter_active() {
    let mut schema = Schema::new(100).unwrap();
    for i in 0..5u32 {
        schema
            .append_fact(&Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), 0.9).unwrap())
            .unwrap();
    }
    schema.delete_fact(2).unwrap();
    let active: Vec<(usize, Fact)> = schema.iter_active().collect();
    assert_eq!(active.len(), 4);
}

#[test]
fn test_dictionary_with_schema() {
    let mut schema = Schema::new(100).unwrap();
    let dict = SharedDictionary::new();
    for i in 0..50u32 {
        let fact = Fact::new(
            SubjectID(dict.insert(&format!("s_{}", i)).unwrap()),
            PredicateID((i % 10) as u8),
            ObjectID(dict.insert(&format!("o_{}", i)).unwrap()),
            0.5,
        )
        .unwrap();
        schema.append_fact(&fact).unwrap();
    }
    assert_eq!(schema.len(), 50);
    assert!(dict.len() > 50);
}

#[test]
fn test_bitmap_index_from_schema() {
    let mut schema = Schema::new(1000).unwrap();
    for i in 0..100u32 {
        let fact = Fact::new(
            SubjectID(i),
            PredicateID((i % 10) as u8),
            ObjectID(i * 2),
            0.5,
        )
        .unwrap();
        schema.append_fact(&fact).unwrap();
    }
    let index = BitmapIndex::new(schema.predicate_col.as_slice(), schema.len()).unwrap();
    for pred in 0u8..10 {
        let bitmap = index.lookup(pred);
        assert!(bitmap.is_some());
        assert!(bitmap.unwrap().count_ones() > 0);
    }
}

#[test]
fn test_bitmap_index_range_query() {
    let mut schema = Schema::new(1000).unwrap();
    for i in 0..100u32 {
        schema
            .append_fact(
                &Fact::new(
                    SubjectID(i),
                    PredicateID((i % 10) as u8),
                    ObjectID(i * 2),
                    0.5,
                )
                .unwrap(),
            )
            .unwrap();
    }
    let index = BitmapIndex::new(schema.predicate_col.as_slice(), schema.len()).unwrap();
    let range_result = index.range_query(3, 7).unwrap();
    assert!(range_result.count_ones() > 0);
}

#[test]
fn test_zone_map() {
    let column: Vec<i64> = (0..1000).map(|i| (i * 7 % 500) as i64).collect();
    let zone_map = ZoneMap::new(&column, 100).unwrap();
    let ranges = zone_map.range_query(100, 200);
    assert!(!ranges.is_empty());
    let no_ranges = zone_map.range_query(1000, 2000);
    assert!(no_ranges.is_empty());
}

#[test]
fn test_compression_roundtrip() {
    use kcm_storage::compress::{Compressor, Lz4Compressor, ZstdCompressor};
    let data: Vec<u8> = (0..10000).map(|i| (i % 256) as u8).collect();
    let zstd = ZstdCompressor::default_level();
    let compressed = zstd.compress(&data).unwrap();
    let decompressed = zstd.decompress(&compressed, data.len()).unwrap();
    assert_eq!(data, decompressed);
    let lz4 = Lz4Compressor::default_level();
    let compressed = lz4.compress(&data).unwrap();
    let decompressed = lz4.decompress(&compressed, data.len()).unwrap();
    assert_eq!(data, decompressed);
}
