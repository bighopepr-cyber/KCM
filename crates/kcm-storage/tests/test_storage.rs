use kcm_core::types::*;
use kcm_storage::codec::{Codec, DeltaCodec, GorillaCodec, RleCodec};
use kcm_storage::column::{Column, ColumnEncoding, CompressionCodec, Schema};
use kcm_storage::index::{BitmapIndex, BloomFilter, ZoneMap};

#[test]
fn test_column_append_and_get() {
    let mut col =
        Column::<u32>::new(100, ColumnEncoding::Identity, CompressionCodec::None).unwrap();
    col.append(10).unwrap();
    col.append(20).unwrap();
    col.append(30).unwrap();

    assert_eq!(col.len(), 3);
    assert_eq!(col.get(0), Some(10));
    assert_eq!(col.get(1), Some(20));
    assert_eq!(col.get(2), Some(30));
    assert_eq!(col.get(3), None);
}

#[test]
fn test_column_full() {
    let mut col = Column::<u8>::new(2, ColumnEncoding::Identity, CompressionCodec::None).unwrap();
    col.append(1).unwrap();
    col.append(2).unwrap();
    assert!(col.append(3).is_err());
}

#[test]
fn test_column_as_slice() {
    let mut col = Column::<u64>::new(10, ColumnEncoding::Identity, CompressionCodec::None).unwrap();
    for i in 0..5 {
        col.append(i * 100).unwrap();
    }
    assert_eq!(col.as_slice(), &[0, 100, 200, 300, 400]);
}

#[test]
fn test_schema_append_and_get_fact() {
    let mut schema = Schema::new(100).unwrap();

    let fact = Fact::new(SubjectID(1), PredicateID(2), ObjectID(3), 0.85).unwrap();

    schema.append_fact(&fact).unwrap();

    assert_eq!(schema.len(), 1);
    let retrieved = schema.get_fact(0).unwrap();
    assert_eq!(retrieved.subject, SubjectID(1));
    assert_eq!(retrieved.predicate, PredicateID(2));
    assert_eq!(retrieved.object, ObjectID(3));
    assert_eq!(retrieved.confidence, 0.85);
}

#[test]
fn test_schema_multiple_facts() {
    let mut schema = Schema::new(100).unwrap();

    for i in 0..10 {
        let fact = Fact::new(
            SubjectID(i),
            PredicateID((i % 5) as u8),
            ObjectID(i * 10),
            0.5 + (i as f64 * 0.05),
        )
        .unwrap();
        schema.append_fact(&fact).unwrap();
    }

    assert_eq!(schema.len(), 10);

    let fact5 = schema.get_fact(5).unwrap();
    assert_eq!(fact5.subject, SubjectID(5));
    assert_eq!(fact5.predicate, PredicateID(0));
    assert_eq!(fact5.object, ObjectID(50));
}

#[test]
fn test_delta_codec() {
    let codec = DeltaCodec;
    let data = vec![100i64, 105, 110, 120, 100];

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
fn test_rle_codec() {
    let codec = RleCodec;
    let data = vec![1u8, 1, 1, 2, 2, 3, 3, 3, 3];

    let encoded = codec.encode(&data).unwrap();
    let decoded = codec.decode(&encoded, data.len()).unwrap();

    assert_eq!(data, decoded);
}

#[test]
fn test_rle_codec_single_values() {
    let codec = RleCodec;
    let data = vec![5u8, 10, 15, 20];

    let encoded = codec.encode(&data).unwrap();
    let decoded = codec.decode(&encoded, data.len()).unwrap();

    assert_eq!(data, decoded);
}

#[test]
fn test_gorilla_codec() {
    let codec = GorillaCodec;
    let data = vec![1.0f64, 1.5, 2.0, 2.5, 3.0];

    let encoded = codec.encode(&data).unwrap();
    let decoded = codec.decode(&encoded, data.len()).unwrap();

    assert_eq!(data.len(), decoded.len());
    for (a, b) in data.iter().zip(decoded.iter()) {
        assert!((a - b).abs() < f64::EPSILON);
    }
}

#[test]
fn test_gorilla_codec_empty() {
    let codec = GorillaCodec;
    let data: Vec<f64> = vec![];

    let encoded = codec.encode(&data).unwrap();
    let decoded = codec.decode(&encoded, 0).unwrap();

    assert!(decoded.is_empty());
}

#[test]
fn test_bitmap_index() {
    let column = vec![0u8, 1, 0, 2, 1, 0, 2, 2];
    let index = BitmapIndex::new(&column, 8).unwrap();

    let bitmap0 = index.lookup(0).unwrap();
    assert_eq!(bitmap0.count_ones(), 3);
    assert!(bitmap0.get(0));
    assert!(bitmap0.get(2));
    assert!(bitmap0.get(5));

    let bitmap1 = index.lookup(1).unwrap();
    assert_eq!(bitmap1.count_ones(), 2);
    assert!(bitmap1.get(1));
    assert!(bitmap1.get(4));

    assert!(index.lookup(3).is_none());
}

#[test]
fn test_bitmap_index_range_query() {
    let column = vec![0u8, 1, 2, 3, 4, 5];
    let index = BitmapIndex::new(&column, 6).unwrap();

    let result = index.range_query(1, 3).unwrap();
    assert_eq!(result.count_ones(), 3);
    assert!(result.get(1));
    assert!(result.get(2));
    assert!(result.get(3));
}

#[test]
fn test_zone_map() {
    let column = vec![10i64, 20, 30, 5, 15, 25];
    let zone_map = ZoneMap::new(&column, 3).unwrap();

    let ranges = zone_map.range_query(15, 25);
    assert!(!ranges.is_empty());

    let ranges = zone_map.range_query(100, 200);
    assert!(ranges.is_empty());
}

#[test]
fn test_bloom_filter() {
    let mut filter = BloomFilter::new(100);

    filter.insert(42);
    filter.insert(100);
    filter.insert(999);

    assert!(filter.contains(42));
    assert!(filter.contains(100));
    assert!(filter.contains(999));

    assert!(!filter.contains(1));
    assert!(!filter.contains(50));
}

#[test]
fn test_schema_get_fact_out_of_bounds() {
    let schema = Schema::new(10).unwrap();
    assert!(schema.get_fact(0).is_none());
    assert!(schema.get_fact(100).is_none());
}

#[test]
fn test_schema_delete_fact() {
    let mut schema = Schema::new(100).unwrap();

    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap();
    schema.append_fact(&fact).unwrap();

    assert!(schema.get_fact(0).is_some());
    assert!(!schema.is_deleted(0));

    schema.delete_fact(0).unwrap();

    assert!(schema.get_fact(0).is_none());
    assert!(schema.is_deleted(0));
    assert_eq!(schema.len(), 1);
    assert_eq!(schema.active_count(), 0);
}

#[test]
fn test_schema_delete_out_of_bounds() {
    let mut schema = Schema::new(10).unwrap();
    assert!(schema.delete_fact(0).is_err());
    assert!(schema.delete_fact(100).is_err());
}

#[test]
fn test_schema_update_fact() {
    let mut schema = Schema::new(100).unwrap();

    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap();
    schema.append_fact(&fact).unwrap();

    let updated = Fact::new(SubjectID(5), PredicateID(2), ObjectID(10), 0.7).unwrap();
    schema.update_fact(0, &updated).unwrap();

    let retrieved = schema.get_fact(0).unwrap();
    assert_eq!(retrieved.subject, SubjectID(5));
    assert_eq!(retrieved.predicate, PredicateID(2));
    assert_eq!(retrieved.object, ObjectID(10));
    assert_eq!(retrieved.confidence, 0.7);
}

#[test]
fn test_schema_update_out_of_bounds() {
    let mut schema = Schema::new(10).unwrap();
    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap();
    assert!(schema.update_fact(0, &fact).is_err());
    assert!(schema.update_fact(100, &fact).is_err());
}

#[test]
fn test_schema_iter_active() {
    let mut schema = Schema::new(100).unwrap();

    for i in 0..5u32 {
        let fact = Fact::new(SubjectID(i), PredicateID(0), ObjectID(i * 10), 0.9).unwrap();
        schema.append_fact(&fact).unwrap();
    }

    schema.delete_fact(1).unwrap();
    schema.delete_fact(3).unwrap();

    let active: Vec<(usize, Fact)> = schema.iter_active().collect();
    assert_eq!(active.len(), 3);
    assert_eq!(active[0].0, 0);
    assert_eq!(active[1].0, 2);
    assert_eq!(active[2].0, 4);
}
