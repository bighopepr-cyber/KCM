use kcm_core::bitmap::Bitmap;
use kcm_core::dictionary::Dictionary;
use kcm_core::types::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn fuzz_confidence_multiply_bounds(
        c1 in 0.0f64..=1.0,
        c2 in 0.0f64..=1.0,
    ) {
        let conf1 = Confidence::new(c1).unwrap();
        let conf2 = Confidence::new(c2).unwrap();
        let result = conf1.multiply(conf2);
        prop_assert!(result.0 >= 0.0 && result.0 <= 1.0);
    }

    #[test]
    fn fuzz_confidence_combine_or_bounds(
        c1 in 0.0f64..=1.0,
        c2 in 0.0f64..=1.0,
    ) {
        let conf1 = Confidence::new(c1).unwrap();
        let conf2 = Confidence::new(c2).unwrap();
        let result = conf1.combine_or(conf2);
        prop_assert!(result.0 >= 0.0 && result.0 <= 1.0);
    }

    #[test]
    fn fuzz_confidence_multiply_commutative(
        c1 in 0.0f64..=1.0,
        c2 in 0.0f64..=1.0,
    ) {
        let conf1 = Confidence::new(c1).unwrap();
        let conf2 = Confidence::new(c2).unwrap();
        let r1 = conf1.multiply(conf2);
        let r2 = conf2.multiply(conf1);
        prop_assert!((r1.0 - r2.0).abs() < 1e-10);
    }

    #[test]
    fn fuzz_confidence_multiply_identity(
        c in 0.0f64..=1.0,
    ) {
        let conf = Confidence::new(c).unwrap();
        let one = Confidence::new(1.0).unwrap();
        let result = conf.multiply(one);
        prop_assert!((result.0 - c).abs() < 1e-10);
    }

    #[test]
    fn fuzz_confidence_multiply_absorption(
        c in 0.0f64..=1.0,
    ) {
        let conf = Confidence::new(c).unwrap();
        let zero = Confidence::new(0.0).unwrap();
        let result = conf.multiply(zero);
        prop_assert!((result.0 - 0.0).abs() < 1e-10);
    }

    #[test]
    fn fuzz_fact_creation(
        subject in 0u32..1_000_000,
        predicate in 0u8..=255,
        object in 0u32..1_000_000,
        confidence in 0.0f64..=1.0,
    ) {
        let result = Fact::new(SubjectID(subject), PredicateID(predicate), ObjectID(object), confidence);
        prop_assert!(result.is_ok());
        let fact = result.unwrap();
        prop_assert_eq!(fact.subject.0, subject);
        prop_assert_eq!(fact.predicate.0, predicate);
        prop_assert_eq!(fact.object.0, object);
    }

    #[test]
    fn fuzz_bitmap_set_get(
        indices in prop::collection::vec(0usize..10000, 0..500),
    ) {
        let mut bitmap = Bitmap::new(10000);
        for &idx in &indices {
            bitmap.set(idx);
        }
        for &idx in &indices {
            prop_assert!(bitmap.get(idx));
        }
    }

    #[test]
    fn fuzz_bitmap_set_clear(
        indices in prop::collection::vec(0usize..10000, 0..500),
    ) {
        let mut bitmap = Bitmap::new(10000);
        for &idx in &indices {
            bitmap.set(idx);
        }
        for &idx in &indices {
            bitmap.clear(idx);
            prop_assert!(!bitmap.get(idx));
        }
    }

    #[test]
    fn fuzz_bitmap_and_or(
        set_a in prop::collection::vec(0usize..10000, 0..200),
        set_b in prop::collection::vec(0usize..10000, 0..200),
    ) {
        let mut a = Bitmap::new(10000);
        let mut b = Bitmap::new(10000);
        for &idx in &set_a { a.set(idx); }
        for &idx in &set_b { b.set(idx); }

        let mut and_result = a.clone();
        and_result.and_inplace(&b);

        let mut or_result = a.clone();
        or_result.or_inplace(&b);

        for &idx in &set_a {
            if set_b.contains(&idx) {
                prop_assert!(and_result.get(idx));
            }
            prop_assert!(or_result.get(idx));
        }
        for &idx in &set_b {
            prop_assert!(or_result.get(idx));
        }
    }

    #[test]
    fn fuzz_row_id_next(
        id in 0u64..u64::MAX - 1,
    ) {
        let row = RowID::new(id);
        let next = row.next();
        prop_assert_eq!(next.0, id + 1);
    }

    #[test]
    fn fuzz_dictionary_idempotence(
        values in prop::collection::vec("[a-z]{1,10}", 0..100),
    ) {
        let mut dict = Dictionary::new();
        for value in &values {
            dict.insert(value).unwrap();
        }
        for value in &values {
            let id1 = dict.lookup(value);
            let id2 = dict.lookup(value);
            prop_assert_eq!(id1, id2);
        }
    }

    #[test]
    fn fuzz_dictionary_bijection(
        values in prop::collection::vec("[a-z]{1,10}", 1..100),
    ) {
        let mut dict = Dictionary::new();
        let mut seen = std::collections::HashSet::new();
        let mut unique_count = 0;
        for value in &values {
            if seen.insert(value.clone()) {
                let id = dict.insert(value).unwrap();
                prop_assert!(id >= unique_count as u32);
                unique_count += 1;
            }
        }
    }

    #[test]
    fn fuzz_dictionary_retrieval(
        values in prop::collection::vec("[a-z]{1,10}", 1..100),
    ) {
        let mut dict = Dictionary::new();
        for value in &values {
            let id = dict.insert(value).unwrap();
            let retrieved = dict.get(id);
            prop_assert_eq!(retrieved, Some(value.as_str()));
        }
    }
}
