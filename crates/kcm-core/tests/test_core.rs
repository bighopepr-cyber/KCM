use kcm_core::bitmap::Bitmap;
use kcm_core::dictionary::Dictionary;
use kcm_core::types::*;
use kcm_core::vec::DenseVec;

#[test]
fn test_types() {
    let subject = SubjectID::new(42);
    assert_eq!(subject.0, 42);

    let confidence = Confidence::new(0.75).unwrap();
    assert_eq!(confidence.0, 0.75);

    assert!(Confidence::new(1.5).is_err());
    assert!(Confidence::new(f64::NAN).is_err());
}

#[test]
fn test_dense_vec() {
    let mut vec = DenseVec::<u32>::new(100).unwrap();

    vec.push(42).unwrap();
    vec.push(43).unwrap();

    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], 42);
    assert_eq!(vec[1], 43);
}

#[test]
fn test_bitmap() {
    let mut bitmap = Bitmap::new(256);

    bitmap.set(0);
    bitmap.set(100);
    bitmap.set(255);

    assert!(bitmap.get(0));
    assert!(bitmap.get(100));
    assert!(bitmap.get(255));
    assert!(!bitmap.get(1));

    assert_eq!(bitmap.count_ones(), 3);
}

#[test]
fn test_dictionary() {
    let mut dict = Dictionary::new();

    let id1 = dict.insert("hello").unwrap();
    let id2 = dict.insert("world").unwrap();
    let id1_again = dict.insert("hello").unwrap();

    assert_eq!(id1, id1_again);
    assert_ne!(id1, id2);

    assert_eq!(dict.get(id1), Some("hello"));
    assert_eq!(dict.get(id2), Some("world"));
}

#[test]
fn test_fact_creation() {
    let fact = Fact::new(SubjectID(1), PredicateID(0), ObjectID(2), 0.9).unwrap();

    assert_eq!(fact.subject, SubjectID(1));
    assert_eq!(fact.predicate, PredicateID(0));
    assert_eq!(fact.object, ObjectID(2));
    assert_eq!(fact.confidence, 0.9);
}

#[test]
fn test_confidence_operations() {
    let c1 = Confidence::new(0.5).unwrap();
    let c2 = Confidence::new(0.6).unwrap();

    let product = c1.multiply(c2);
    assert!((product.0 - 0.3).abs() < 0.0001);

    let combined = c1.combine_or(c2);
    assert!((combined.0 - 0.8).abs() < 0.0001);
}

#[test]
fn test_bitmap_boolean_ops() {
    let mut a = Bitmap::new(64);
    let mut b = Bitmap::new(64);

    a.set(0);
    a.set(1);
    a.set(2);

    b.set(1);
    b.set(2);
    b.set(3);

    a.and_inplace(&b);
    assert!(a.get(1));
    assert!(a.get(2));
    assert!(!a.get(0));
    assert!(!a.get(3));
    assert_eq!(a.count_ones(), 2);
}

#[test]
fn test_dense_vec_full() {
    let mut vec = DenseVec::<u64>::new(3).unwrap();

    vec.push(10).unwrap();
    vec.push(20).unwrap();
    vec.push(30).unwrap();

    assert!(vec.push(40).is_err());

    assert_eq!(vec.as_slice(), &[10, 20, 30]);
}

#[test]
fn test_column_id_all() {
    let all = ColumnID::all();
    assert_eq!(all.len(), 11);
}
