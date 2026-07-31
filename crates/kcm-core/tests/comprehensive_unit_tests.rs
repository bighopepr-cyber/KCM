use kcm_core::bitmap::Bitmap;
use kcm_core::dictionary::{Dictionary, SharedDictionary};
use kcm_core::types::*;
use kcm_core::vec::DenseVec;

#[test]
fn test_row_id_operations() {
    let id1 = RowID::new(0);
    let id2 = RowID::new(1);
    assert!(id1 < id2);
    assert_eq!(id2.next(), RowID::new(2));
    assert_eq!(id1.as_usize(), 0);
}

#[test]
fn test_subject_id_boundary() {
    let min = SubjectID::new(0);
    let max = SubjectID::new(u32::MAX);
    assert_eq!(min.0, 0);
    assert_eq!(max.0, u32::MAX);
}

#[test]
fn test_predicate_id_max_256() {
    let max_valid = PredicateID::new(255);
    assert_eq!(max_valid.as_usize(), 255);
}

#[test]
fn test_confidence_bounds() {
    assert!(Confidence::new(0.0).is_ok());
    assert!(Confidence::new(0.5).is_ok());
    assert!(Confidence::new(1.0).is_ok());
    assert!(Confidence::new(-0.1).is_err());
    assert!(Confidence::new(1.1).is_err());
    assert!(Confidence::new(f64::NAN).is_err());
    assert!(Confidence::new(f64::INFINITY).is_err());
}

#[test]
fn test_confidence_multiply() {
    let c1 = Confidence::new(0.5).unwrap();
    let c2 = Confidence::new(0.8).unwrap();
    let result = c1.multiply(c2);
    assert!((result.0 - 0.4).abs() < 1e-10);
}

#[test]
fn test_confidence_combine_or() {
    let c1 = Confidence::new(0.3).unwrap();
    let c2 = Confidence::new(0.4).unwrap();
    let result = c1.combine_or(c2);
    let expected = 0.3 + 0.4 - (0.3 * 0.4);
    assert!((result.0 - expected).abs() < 1e-10);
}

#[test]
fn test_confidence_multiply_boundary() {
    let c_zero = Confidence::new(0.0).unwrap();
    let c_one = Confidence::new(1.0).unwrap();
    let c_half = Confidence::new(0.5).unwrap();
    assert!((c_zero.multiply(c_half).0 - 0.0).abs() < 1e-10);
    assert!((c_one.multiply(c_half).0 - 0.5).abs() < 1e-10);
    assert!((c_half.multiply(c_one).0 - 0.5).abs() < 1e-10);
}

#[test]
fn test_confidence_combine_or_boundary() {
    let c_zero = Confidence::new(0.0).unwrap();
    let c_one = Confidence::new(1.0).unwrap();
    assert!((c_zero.combine_or(c_zero).0 - 0.0).abs() < 1e-10);
    assert!((c_one.combine_or(c_half()).0 - 1.0).abs() < 1e-10);
}

fn c_half() -> Confidence {
    Confidence::new(0.5).unwrap()
}

#[test]
fn test_fact_creation() {
    let fact = Fact::new(SubjectID(1), PredicateID(5), ObjectID(10), 0.95).unwrap();
    assert_eq!(fact.subject.0, 1);
    assert_eq!(fact.predicate.0, 5);
    assert_eq!(fact.object.0, 10);
    assert_eq!(fact.confidence, 0.95);
    assert_eq!(fact.version, 1);
}

#[test]
fn test_fact_invalid_confidence() {
    assert!(Fact::new(SubjectID(1), PredicateID(5), ObjectID(10), 1.5).is_err());
}

#[test]
fn test_column_id_all() {
    assert_eq!(ColumnID::all().len(), 11);
}

#[test]
fn test_error_display() {
    let err = KcmError::NotFound("Test".to_string());
    assert!(err.to_string().contains("NotFound"));
}

#[test]
fn test_error_from_string() {
    let err: KcmError = "something".to_string().into();
    assert!(matches!(err, KcmError::InvalidArgument(_)));
}

#[test]
fn test_dense_vec_allocation() {
    let vec: DenseVec<u32> = DenseVec::new(100).unwrap();
    assert_eq!(vec.capacity(), 100);
    assert_eq!(vec.len(), 0);
    assert!(vec.is_empty());
}

#[test]
fn test_dense_vec_push() {
    let mut vec: DenseVec<u32> = DenseVec::new(10).unwrap();
    vec.push(42).unwrap();
    vec.push(43).unwrap();
    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], 42);
    assert_eq!(vec[1], 43);
}

#[test]
fn test_dense_vec_overflow() {
    let mut vec: DenseVec<u32> = DenseVec::new(1).unwrap();
    vec.push(1).unwrap();
    assert!(vec.push(2).is_err());
}

#[test]
fn test_dense_vec_as_slice() {
    let mut vec: DenseVec<u32> = DenseVec::new(5).unwrap();
    for i in 0..5 {
        vec.push(i).unwrap();
    }
    assert_eq!(vec.as_slice(), &[0, 1, 2, 3, 4]);
}

#[test]
fn test_dense_vec_iterator() {
    let mut vec: DenseVec<u32> = DenseVec::new(3).unwrap();
    vec.push(10).unwrap();
    vec.push(20).unwrap();
    vec.push(30).unwrap();
    let sum: u32 = vec.iter().sum();
    assert_eq!(sum, 60);
}

#[test]
fn test_dense_vec_clone() {
    let mut vec1: DenseVec<u32> = DenseVec::new(3).unwrap();
    vec1.push(1).unwrap();
    vec1.push(2).unwrap();
    let vec2 = vec1.clone();
    assert_eq!(vec1.len(), vec2.len());
    assert_eq!(vec1.as_slice(), vec2.as_slice());
}

#[test]
fn test_dense_vec_alignment() {
    let vec: DenseVec<u64> = DenseVec::with_alignment(100, 64).unwrap();
    assert_eq!(vec.capacity(), 100);
}

#[test]
fn test_dense_vec_mutable_index() {
    let mut vec: DenseVec<u32> = DenseVec::new(5).unwrap();
    for i in 0..5 {
        vec.push(i * 10).unwrap();
    }
    vec[2] = 999;
    assert_eq!(vec[2], 999);
    assert_eq!(vec[0], 0);
    assert_eq!(vec[4], 40);
}

#[test]
fn test_dense_vec_empty_capacity() {
    let mut vec: DenseVec<u32> = DenseVec::new(0).unwrap();
    assert!(vec.is_empty());
    assert_eq!(vec.capacity(), 0);
    assert!(vec.push(1).is_err());
}

#[test]
fn test_dense_vec_cache_aligned() {
    let vec: DenseVec<u64> = DenseVec::new_cache_aligned(100).unwrap();
    assert_eq!(vec.capacity(), 100);
}

#[test]
fn test_bitmap_set_get() {
    let mut bitmap = Bitmap::new(256);
    bitmap.set(0);
    bitmap.set(127);
    bitmap.set(255);
    assert!(bitmap.get(0));
    assert!(bitmap.get(127));
    assert!(bitmap.get(255));
    assert!(!bitmap.get(1));
}

#[test]
fn test_bitmap_clear() {
    let mut bitmap = Bitmap::new(64);
    bitmap.set(10);
    assert!(bitmap.get(10));
    bitmap.clear(10);
    assert!(!bitmap.get(10));
}

#[test]
fn test_bitmap_count_ones() {
    let mut bitmap = Bitmap::new(1024);
    bitmap.set(0);
    bitmap.set(100);
    bitmap.set(500);
    bitmap.set(999);
    assert_eq!(bitmap.count_ones(), 4);
}

#[test]
fn test_bitmap_and() {
    let mut a = Bitmap::new(64);
    let mut b = Bitmap::new(64);
    a.set(0);
    a.set(10);
    b.set(10);
    b.set(20);
    a.and_inplace(&b);
    assert!(a.get(10));
    assert!(!a.get(0));
    assert!(!a.get(20));
}

#[test]
fn test_bitmap_or() {
    let mut a = Bitmap::new(64);
    let mut b = Bitmap::new(64);
    a.set(0);
    b.set(10);
    a.or_inplace(&b);
    assert!(a.get(0));
    assert!(a.get(10));
}

#[test]
fn test_bitmap_not() {
    let mut b = Bitmap::new(64);
    b.set(0);
    b.set(63);
    b.not_inplace();
    assert!(!b.get(0));
    assert!(!b.get(63));
    assert!(b.get(1));
    assert!(b.get(62));
}

#[test]
fn test_bitmap_set_all_clear_all() {
    let mut b = Bitmap::new(128);
    b.set_all();
    assert_eq!(b.count_ones(), 128);
    b.clear_all();
    assert_eq!(b.count_ones(), 0);
}

#[test]
fn test_bitmap_iter_set_bits() {
    let mut b = Bitmap::new(100);
    b.set(5);
    b.set(25);
    b.set(75);
    let bits: Vec<usize> = b.iter_set_bits().collect();
    assert_eq!(bits, vec![5, 25, 75]);
}

#[test]
fn test_bitmap_clone() {
    let mut b1 = Bitmap::new(64);
    b1.set(42);
    let b2 = b1.clone();
    assert!(b2.get(42));
    assert_eq!(b1.count_ones(), b2.count_ones());
}

#[test]
fn test_bitmap_out_of_bounds() {
    let b = Bitmap::new(64);
    assert!(!b.get(64));
    assert!(!b.get(1000));
}

#[test]
fn test_bitmap_len() {
    let b = Bitmap::new(200);
    assert_eq!(b.len(), 200);
}

#[test]
fn test_bitmap_is_empty() {
    assert!(Bitmap::new(0).is_empty());
    assert!(!Bitmap::new(1).is_empty());
}

#[test]
fn test_dictionary_insert_lookup() {
    let mut dict = Dictionary::new();
    let id1 = dict.insert("hello");
    let id2 = dict.insert("world");
    let id1_again = dict.insert("hello");
    assert_eq!(id1, id1_again);
    assert_ne!(id1, id2);
    assert_eq!(dict.len(), 3);
}

#[test]
fn test_dictionary_get() {
    let mut dict = Dictionary::new();
    let id = dict.insert("test");
    assert_eq!(dict.get(id), Some("test"));
}

#[test]
fn test_dictionary_null_id() {
    let dict = Dictionary::new();
    assert_eq!(dict.get(0), Some(""));
}

#[test]
fn test_dictionary_lookup() {
    let mut dict = Dictionary::new();
    let id = dict.insert("foo");
    assert_eq!(dict.lookup("foo"), Some(id));
    assert_eq!(dict.lookup("bar"), None);
}

#[test]
fn test_dictionary_empty() {
    let dict = Dictionary::new();
    assert!(dict.is_empty());
    assert_eq!(dict.len(), 1);
}

#[test]
fn test_dictionary_many_entries() {
    let mut dict = Dictionary::new();
    for i in 0..1000 {
        dict.insert(&format!("entry_{}", i));
    }
    assert_eq!(dict.len(), 1001);
    assert_eq!(dict.get(500), Some("entry_499"));
}

#[test]
fn test_shared_dictionary() {
    let dict = SharedDictionary::new();
    let id1 = dict.insert("foo");
    let id2 = dict.insert("bar");
    assert_eq!(dict.get(id1), Some("foo".to_string()));
    assert_eq!(dict.get(id2), Some("bar".to_string()));
    assert_eq!(dict.lookup("foo"), Some(id1));
    assert_eq!(dict.lookup("missing"), None);
}

#[test]
fn test_shared_dictionary_clone() {
    let dict = SharedDictionary::new();
    let id = dict.insert("test");
    let dict2 = dict.clone();
    assert_eq!(dict2.get(id), Some("test".to_string()));
}
