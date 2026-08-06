//! KCM Rust SDK — Error Handling Example.
//!
//! Demonstrates: proper error handling patterns with SdkError.

use kcm_sdk::{Database, Fact, SdkError};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== KCM Rust SDK — Error Handling Example ===\n");

    let db = Database::new()?;

    // --- INVALID CONFIDENCE ---
    println!("--- Invalid Confidence (out of range) ---");
    match Fact::new(1, 0, 2, 1.5) {
        Ok(_) => println!("  FAIL: Should have returned error"),
        Err(e) => {
            println!("  Caught: {}", e);
            println!("  Code: {} ({})", e.name(), e.code());
            assert_eq!(e.error_code(), kcm_sdk::ErrorCode::InvalidArgument);
        }
    }

    // --- NOT FOUND (update non-existent row) ---
    println!("\n--- Not Found (update non-existent row) ---");
    match db.update(99999, &Fact::new(1, 0, 2, 0.5)?) {
        Ok(_) => println!("  FAIL: Should have returned error"),
        Err(e) => {
            println!("  Caught: {}", e);
            println!("  Code: {} ({})", e.name(), e.code());
        }
    }

    // --- NOT FOUND (delete non-existent row) ---
    println!("\n--- Not Found (delete non-existent row) ---");
    match db.delete(99999) {
        Ok(_) => println!("  Delete succeeded"),
        Err(e) => println!("  Caught: {}", e),
    }

    // --- GET FACT (non-existent) ---
    println!("\n--- Get Fact (non-existent) ---");
    match db.get_fact(99999)? {
        Some(_) => println!("  FAIL: Should have returned None"),
        None => println!("  get_fact returned None (expected)"),
    }

    // --- ALL ERROR CODES ---
    println!("\n--- All Error Codes ---");
    let error_codes = [
        (SdkError::NotFound("test".into()), "NotFound"),
        (SdkError::OutOfMemory, "OutOfMemory"),
        (SdkError::InvalidArgument("test".into()), "InvalidArgument"),
        (SdkError::Io("test".into()), "Io"),
        (SdkError::Corrupted("test".into()), "Corrupted"),
        (SdkError::Conflict("test".into()), "Conflict"),
        (SdkError::TransactionAborted, "TransactionAborted"),
    ];
    for (err, name) in &error_codes {
        println!("  {} ({}): {}", name, err.code(), err);
    }

    // --- JSON SERIALIZATION ---
    println!("\n--- Error JSON Serialization ---");
    let err = SdkError::NotFound("row 42 not found".into());
    println!("  JSON: {}", err.to_json());

    // --- TRY-CATCH PATTERN ---
    println!("\n--- Try-Catch Pattern ---");
    match (|| -> Result<(), SdkError> {
        db.insert(&Fact::new(1, 0, 2, 0.95)?)?;
        db.insert(&Fact::new(2, 1, 3, 0.90)?)?;
        let results = db.query("all")?;
        println!("  Query returned {} results", results.count());
        Ok(())
    })() {
        Ok(()) => {}
        Err(e) => println!("  Database error: {}", e),
    }

    // --- SAVE ERROR ---
    println!("\n--- Save to Invalid Path ---");
    match db.save("/nonexistent/dir/db.kcm") {
        Ok(_) => println!("  Save succeeded"),
        Err(e) => println!("  Caught: {}", e),
    }

    db.close();
    println!("\n=== All error handling patterns completed ===");
    Ok(())
}
