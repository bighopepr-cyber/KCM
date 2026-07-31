use kcm_compute::algebra::*;
use kcm_compute::simd::*;
use kcm_core::types::*;
use kcm_storage::column::Schema;

fn setup_schema() -> Schema {
    let mut schema = Schema::new(100).unwrap();

    for i in 0..20u32 {
        let fact = Fact::new(
            SubjectID(i % 5),
            PredicateID((i % 3) as u8),
            ObjectID(i * 10),
            0.3 + (i as f64 * 0.03),
        )
        .unwrap();
        schema.append_fact(&fact).unwrap();
    }

    schema
}

#[test]
fn test_scan_op() {
    let schema = setup_schema();
    let scan = ScanOp::new(&schema);

    let result = scan.execute().unwrap();
    assert_eq!(result.len(), 20);
}

#[test]
fn test_scan_op_with_confidence() {
    let schema = setup_schema();
    let scan = ScanOp::new(&schema).with_confidence(0.5);

    let result = scan.execute().unwrap();
    assert!(result.len() < 20);

    for &idx in &result {
        let confidence = schema.confidence_col.get(idx).unwrap();
        assert!(confidence >= 0.5);
    }
}

#[test]
fn test_filter_op_equal_subject() {
    let schema = setup_schema();
    let scan = ScanOp::new(&schema);
    let rowids = scan.execute().unwrap();

    let filter = FilterOp::new(rowids, &schema, FilterPredicate::EqualSubject(0));

    let result = filter.execute().unwrap();
    assert!(!result.is_empty());

    for &idx in &result {
        assert_eq!(schema.subject_col.get(idx), Some(0));
    }
}

#[test]
fn test_filter_op_equal_predicate() {
    let schema = setup_schema();
    let scan = ScanOp::new(&schema);
    let rowids = scan.execute().unwrap();

    let filter = FilterOp::new(rowids, &schema, FilterPredicate::EqualPredicate(1));

    let result = filter.execute().unwrap();
    assert!(!result.is_empty());

    for &idx in &result {
        assert_eq!(schema.predicate_col.get(idx), Some(1));
    }
}

#[test]
fn test_filter_op_in_set() {
    let schema = setup_schema();
    let scan = ScanOp::new(&schema);
    let rowids = scan.execute().unwrap();

    let filter = FilterOp::new(rowids, &schema, FilterPredicate::InSet(vec![0, 10, 20]));

    let result = filter.execute().unwrap();
    assert!(!result.is_empty());

    for &idx in &result {
        let obj = schema.object_col.get(idx).unwrap();
        assert!([0, 10, 20].contains(&obj));
    }
}

#[test]
fn test_filter_op_range_timestamp() {
    let schema = setup_schema();
    let scan = ScanOp::new(&schema);
    let rowids = scan.execute().unwrap();

    let ts_low = schema.timestamp_col.get(0).unwrap();
    let ts_high = schema.timestamp_col.get(5).unwrap();

    let filter = FilterOp::new(
        rowids,
        &schema,
        FilterPredicate::RangeTimestamp(ts_low, ts_high),
    );

    let result = filter.execute().unwrap();
    assert!(!result.is_empty());
    assert!(result.len() <= 6);
}

#[test]
fn test_project_op() {
    let schema = setup_schema();
    let rowids = vec![0, 1, 2, 3, 4];

    let project = ProjectOp::new(
        rowids.clone(),
        &schema,
        vec![ColumnID::Subject, ColumnID::Object],
    );

    let result = project.execute().unwrap();
    assert_eq!(result, rowids);
}

#[test]
fn test_join_op() {
    let mut schema = Schema::new(100).unwrap();

    for i in 0..10u32 {
        let fact = Fact::new(SubjectID(i), PredicateID(0), ObjectID(i + 100), 0.9).unwrap();
        schema.append_fact(&fact).unwrap();
    }

    let left: Vec<usize> = (0..5).collect();
    let right: Vec<usize> = (3..8).collect();

    let join = JoinOp::new(left, right, &schema, ColumnID::Object);

    let result = join.execute().unwrap();
    assert!(!result.is_empty());
    assert!(result.len() % 2 == 0);
}

#[test]
fn test_aggregate_op_count() {
    let schema = setup_schema();
    let rowids: Vec<usize> = (0..20).collect();

    let agg = AggregateOp::new(rowids, &schema, None, AggregateFunc::Count);

    let result = agg.execute().unwrap();
    assert_eq!(result.len(), 20);
}

#[test]
fn test_aggregate_op_sum() {
    let schema = setup_schema();
    let rowids: Vec<usize> = (0..20).collect();

    let agg = AggregateOp::new(rowids, &schema, None, AggregateFunc::Sum);

    let result = agg.execute().unwrap();
    assert_eq!(result.len(), 20);
}

#[test]
fn test_simd_filter_eq_u8() {
    let data = vec![1u8, 2, 3, 4, 5, 2, 2, 6];
    let result = data.simd_filter_eq(2);

    assert_eq!(result.len(), 8);
    assert!(!result[0]);
    assert!(result[1]);
    assert!(!result[2]);
    assert!(!result[3]);
    assert!(!result[4]);
    assert!(result[5]);
    assert!(result[6]);
    assert!(!result[7]);
}

#[test]
fn test_simd_filter_eq_u32() {
    let data = vec![10u32, 20, 30, 20, 50];
    let result = data.simd_filter_eq(20);

    assert!(!result[0]);
    assert!(result[1]);
    assert!(!result[2]);
    assert!(result[3]);
    assert!(!result[4]);
}

#[test]
fn test_simd_filter_ge_u32() {
    let data = vec![10u32, 20, 30, 40, 50];
    let result = data.simd_filter_ge(30);

    assert!(!result[0]);
    assert!(!result[1]);
    assert!(result[2]);
    assert!(result[3]);
    assert!(result[4]);
}

#[test]
fn test_simd_filter_ge_f64() {
    let data = vec![1.0f64, 2.5, 3.0, 4.5, 5.0];
    let result = data.simd_filter_ge(3.0);

    assert!(!result[0]);
    assert!(!result[1]);
    assert!(result[2]);
    assert!(result[3]);
    assert!(result[4]);
}

#[test]
fn test_operator_estimated_rows() {
    let schema = setup_schema();
    let scan = ScanOp::new(&schema);
    assert_eq!(scan.estimated_rows(), 20);

    let agg = AggregateOp::new(vec![], &schema, None, AggregateFunc::Count);
    assert_eq!(agg.estimated_rows(), 1);

    let agg_grouped = AggregateOp::new(
        vec![],
        &schema,
        Some(ColumnID::Subject),
        AggregateFunc::Count,
    );
    assert_eq!(agg_grouped.estimated_rows(), 256);
}
