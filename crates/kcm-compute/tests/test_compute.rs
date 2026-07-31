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
fn test_scan_op_all() {
    let schema = setup_schema();
    let scan = ScanOp::new(&schema);
    let result = scan.execute().unwrap();
    assert_eq!(result.len(), 20);
}

#[test]
fn test_scan_op_with_context() {
    let mut schema = Schema::new(100).unwrap();
    for i in 0..10u32 {
        let mut fact = Fact::new(SubjectID(i), PredicateID(0), ObjectID(i), 0.9).unwrap();
        fact.context = ContextID((i % 3) as u8);
        schema.append_fact(&fact).unwrap();
    }
    let scan = ScanOp::new(&schema).with_context(1);
    let result = scan.execute().unwrap();
    assert!(!result.is_empty());
    for &idx in &result {
        assert_eq!(schema.context_col.get(idx), Some(1u8));
    }
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
fn test_scan_op_with_both_filters() {
    let mut schema = Schema::new(100).unwrap();
    for i in 0..10u32 {
        let mut fact = Fact::new(
            SubjectID(i),
            PredicateID(0),
            ObjectID(i),
            0.1 + (i as f64 * 0.1),
        )
        .unwrap();
        fact.context = ContextID((i % 2) as u8);
        schema.append_fact(&fact).unwrap();
    }
    let scan = ScanOp::new(&schema).with_context(0).with_confidence(0.5);
    let result = scan.execute().unwrap();
    for &idx in &result {
        assert_eq!(schema.context_col.get(idx), Some(0u8));
        assert!(schema.confidence_col.get(idx).unwrap() >= 0.5);
    }
}

#[test]
fn test_filter_op_equal_subject() {
    let schema = setup_schema();
    let rowids = ScanOp::new(&schema).execute().unwrap();
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
    let rowids = ScanOp::new(&schema).execute().unwrap();
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
    let rowids = ScanOp::new(&schema).execute().unwrap();
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
    let rowids = ScanOp::new(&schema).execute().unwrap();
    let ts_low = schema.timestamp_col.get(0).unwrap();
    let ts_high = schema.timestamp_col.get(5).unwrap();
    let filter = FilterOp::new(
        rowids,
        &schema,
        FilterPredicate::RangeTimestamp(ts_low, ts_high),
    );
    let result = filter.execute().unwrap();
    assert!(!result.is_empty());
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
fn test_project_op_execute_projection() {
    let schema = setup_schema();
    let rowids = vec![0, 1, 2];
    let project = ProjectOp::new(rowids, &schema, vec![ColumnID::Subject, ColumnID::Object]);
    let projected = project.execute_projection().unwrap();
    assert_eq!(projected.len(), 3);
    assert_eq!(projected[0].len(), 2);
    assert_eq!(projected[0][0], schema.subject_col.get(0).unwrap() as u64);
    assert_eq!(projected[0][1], schema.object_col.get(0).unwrap() as u64);
}

#[test]
fn test_join_op() {
    let mut schema = Schema::new(100).unwrap();
    for i in 0..10u32 {
        Fact::new(SubjectID(i), PredicateID(0), ObjectID(i + 100), 0.9)
            .and_then(|f| schema.append_fact(&f).map_err(|e| format!("{}", e)))
            .ok();
    }
    let left: Vec<usize> = (0..5).collect();
    let right: Vec<usize> = (3..8).collect();
    let join = JoinOp::new(left, right, &schema, ColumnID::Object);
    let result = join.execute().unwrap();
    assert!(result.len().is_multiple_of(2));
}

#[test]
fn test_aggregate_count() {
    let schema = setup_schema();
    let rowids: Vec<usize> = (0..20).collect();
    let agg = AggregateOp::new(rowids, &schema, None, AggregateFunc::Count);
    let result = agg.execute_aggregate().unwrap();
    assert_eq!(result, 20.0);
}

#[test]
fn test_aggregate_sum() {
    let schema = setup_schema();
    let rowids: Vec<usize> = (0..20).collect();
    let agg = AggregateOp::new(rowids, &schema, None, AggregateFunc::Sum);
    let result = agg.execute_aggregate().unwrap();
    assert!(result > 0.0);
}

#[test]
fn test_aggregate_avg() {
    let schema = setup_schema();
    let rowids: Vec<usize> = (0..20).collect();
    let agg = AggregateOp::new(rowids, &schema, None, AggregateFunc::Avg);
    let result = agg.execute_aggregate().unwrap();
    assert!(result > 0.0 && result <= 1.0);
}

#[test]
fn test_aggregate_min() {
    let schema = setup_schema();
    let rowids: Vec<usize> = (0..20).collect();
    let agg = AggregateOp::new(rowids, &schema, None, AggregateFunc::Min);
    let result = agg.execute_aggregate().unwrap();
    assert!(result > 0.0);
}

#[test]
fn test_aggregate_max() {
    let schema = setup_schema();
    let rowids: Vec<usize> = (0..20).collect();
    let agg = AggregateOp::new(rowids, &schema, None, AggregateFunc::Max);
    let result = agg.execute_aggregate().unwrap();
    assert!(result > 0.0);
}

#[test]
fn test_aggregate_empty() {
    let schema = setup_schema();
    let agg = AggregateOp::new(vec![], &schema, None, AggregateFunc::Count);
    let result = agg.execute_aggregate().unwrap();
    assert_eq!(result, 0.0);
}

#[test]
fn test_aggregate_grouped() {
    let schema = setup_schema();
    let rowids: Vec<usize> = (0..20).collect();
    let agg = AggregateOp::new(
        rowids,
        &schema,
        Some(ColumnID::Subject),
        AggregateFunc::Count,
    );
    let groups = agg.execute_grouped().unwrap();
    assert!(!groups.is_empty());
    let total: f64 = groups.iter().map(|(_, c)| c).sum();
    assert_eq!(total, 20.0);
}

#[test]
fn test_simd_filter_eq_u8() {
    let data = [1u8, 2, 3, 2, 5, 2, 7];
    assert_eq!(
        data.simd_filter_eq(2),
        [false, true, false, true, false, true, false]
    );
}

#[test]
fn test_simd_filter_ge_u8() {
    let data = [10u8, 20, 30, 40, 50];
    assert_eq!(data.simd_filter_ge(30), [false, false, true, true, true]);
}

#[test]
fn test_simd_count_u32() {
    let data = [0u32, 1, 0, 3, 5, 0];
    assert_eq!(data.simd_count(), 3);
}

#[test]
fn test_simd_count_f64() {
    let data = [0.0f64, 1.5, 0.0, 3.0];
    assert_eq!(data.simd_count(), 2);
}

#[test]
fn test_operator_estimated_rows() {
    let schema = setup_schema();
    let scan = ScanOp::new(&schema);
    assert_eq!(scan.estimated_rows(), 20);
}
