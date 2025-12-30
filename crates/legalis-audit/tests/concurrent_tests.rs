//! Concurrent access pattern tests for legalis-audit.
//!
//! These tests verify thread safety and concurrent access patterns.

use legalis_audit::{Actor, AuditRecord, AuditTrail, DecisionContext, DecisionResult, EventType};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use uuid::Uuid;

/// Creates a test audit record.
fn create_test_record() -> AuditRecord {
    AuditRecord::new(
        EventType::AutomaticDecision,
        Actor::System {
            component: "concurrent_test".to_string(),
        },
        "statute-test".to_string(),
        Uuid::new_v4(),
        DecisionContext::default(),
        DecisionResult::Deterministic {
            effect_applied: "approved".to_string(),
            parameters: HashMap::new(),
        },
        None,
    )
}

#[test]
fn test_concurrent_writes() {
    let trail = Arc::new(Mutex::new(AuditTrail::new()));
    let num_threads = 10;
    let records_per_thread = 100;

    let mut handles = vec![];

    for _ in 0..num_threads {
        let trail_clone = Arc::clone(&trail);
        let handle = thread::spawn(move || {
            for _ in 0..records_per_thread {
                let record = create_test_record();
                trail_clone.lock().unwrap().record(record).unwrap();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let count = trail.lock().unwrap().count();
    assert_eq!(count, num_threads * records_per_thread);
}

#[test]
fn test_concurrent_reads_and_writes() {
    let trail = Arc::new(Mutex::new(AuditTrail::new()));

    // Pre-populate with some records
    for _ in 0..50 {
        let record = create_test_record();
        trail.lock().unwrap().record(record).unwrap();
    }

    let num_writer_threads = 5;
    let num_reader_threads = 5;
    let mut handles = vec![];

    // Writer threads
    for _ in 0..num_writer_threads {
        let trail_clone = Arc::clone(&trail);
        let handle = thread::spawn(move || {
            for _ in 0..20 {
                let record = create_test_record();
                trail_clone.lock().unwrap().record(record).unwrap();
            }
        });
        handles.push(handle);
    }

    // Reader threads
    for _ in 0..num_reader_threads {
        let trail_clone = Arc::clone(&trail);
        let handle = thread::spawn(move || {
            for _ in 0..30 {
                let count = trail_clone.lock().unwrap().count();
                assert!(count >= 50); // At least the initial records
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_count = trail.lock().unwrap().count();
    assert_eq!(final_count, 50 + (num_writer_threads * 20));
}

#[test]
fn test_concurrent_integrity_verification() {
    let trail = Arc::new(Mutex::new(AuditTrail::new()));

    // Populate with records
    for _ in 0..100 {
        let record = create_test_record();
        trail.lock().unwrap().record(record).unwrap();
    }

    let num_threads = 10;
    let mut handles = vec![];

    for _ in 0..num_threads {
        let trail_clone = Arc::clone(&trail);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let is_valid = trail_clone.lock().unwrap().verify_integrity().unwrap();
                assert!(is_valid);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_concurrent_queries() {
    let trail = Arc::new(Mutex::new(AuditTrail::new()));
    let statute_ids = ["statute-1", "statute-2", "statute-3"];

    // Populate with records
    for i in 0..300 {
        let mut record = create_test_record();
        record.statute_id = statute_ids[i % statute_ids.len()].to_string();
        trail.lock().unwrap().record(record).unwrap();
    }

    let num_threads = 15;
    let mut handles = vec![];

    for i in 0..num_threads {
        let trail_clone = Arc::clone(&trail);
        let statute_id = statute_ids[i % statute_ids.len()];
        let handle = thread::spawn(move || {
            for _ in 0..20 {
                let results = trail_clone
                    .lock()
                    .unwrap()
                    .query_by_statute(statute_id)
                    .unwrap();
                assert!(!results.is_empty());
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_high_contention_scenario() {
    let trail = Arc::new(Mutex::new(AuditTrail::new()));
    let num_threads = 20;
    let operations_per_thread = 50;

    let mut handles = vec![];

    for thread_id in 0..num_threads {
        let trail_clone = Arc::clone(&trail);
        let handle = thread::spawn(move || {
            for _ in 0..operations_per_thread {
                match thread_id % 4 {
                    0 => {
                        // Write
                        let record = create_test_record();
                        trail_clone.lock().unwrap().record(record).unwrap();
                    }
                    1 => {
                        // Count
                        let _ = trail_clone.lock().unwrap().count();
                    }
                    2 => {
                        // Verify integrity
                        let _ = trail_clone.lock().unwrap().verify_integrity();
                    }
                    _ => {
                        // Generate report
                        let _ = trail_clone.lock().unwrap().generate_report();
                    }
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_count = trail.lock().unwrap().count();
    assert!(final_count > 0);
}
