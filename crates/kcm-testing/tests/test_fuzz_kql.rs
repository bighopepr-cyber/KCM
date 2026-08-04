use kcm_core::types::*;
use kcm_core::vec::DenseVec;
use kcm_core::bitmap::Bitmap;
use kcm_core::dictionary::Dictionary;

#[test]
fn test_fuzz_densevec_various_sizes() {
    for size in [0, 1, 10, 100, 1000, 10000] {
        let mut vec = DenseVec::<u32>::new(size).unwrap();
        for i in 0..size {
            vec.push(i as u32).unwrap();
        }
        assert_eq!(vec.len(), size);
    }
}

#[test]
fn test_fuzz_bitmap_various_sizes() {
    for size in [0, 1, 63, 64, 65, 128, 256, 1000, 10000] {
        let mut bitmap = Bitmap::new(size);
        for i in 0..size {
            bitmap.set(i);
        }
        for i in 0..size {
            assert!(bitmap.get(i));
        }
        bitmap.clear_all();
        for i in 0..size {
            assert!(!bitmap.get(i));
        }
    }
}

#[test]
fn test_fuzz_dictionary_various_sizes() {
    for size in [0, 1, 10, 100, 1000] {
        let mut dict = Dictionary::new();
        for i in 0..size {
            let _ = dict.insert(&format!("key_{}", i));
        }
        for i in 0..size {
            let id = dict.insert(&format!("key_{}", i)).unwrap();
            let val = dict.get(id);
            assert!(val.is_some());
        }
    }
}

#[test]
fn test_fuzz_fact_creation() {
    let invalid_confs = [-1.0, -0.001, 1.001, f64::NAN, f64::INFINITY, f64::NEG_INFINITY];
    for c in invalid_confs {
        let result = Fact::new(SubjectID(1), PredicateID(0), ObjectID(1), c);
        assert!(result.is_err(), "Should reject confidence {}", c);
    }

    let valid_confs = [0.0, 0.001, 0.5, 0.999, 0.999999, 1.0];
    for c in valid_confs {
        let result = Fact::new(SubjectID(1), PredicateID(0), ObjectID(1), c);
        assert!(result.is_ok(), "Should accept confidence {}", c);
    }
}

#[test]
fn test_fuzz_kcm_error_display() {
    let errors = [
        KcmError::NotFound("test".into()),
        KcmError::OutOfMemory,
        KcmError::InvalidArgument("bad".into()),
        KcmError::Io("disk".into()),
        KcmError::Corrupted("data".into()),
        KcmError::Conflict("tx".into()),
        KcmError::TransactionAborted,
    ];
    for err in &errors {
        let msg = format!("{}", err);
        assert!(!msg.is_empty(), "Error message should not be empty");
    }
}
