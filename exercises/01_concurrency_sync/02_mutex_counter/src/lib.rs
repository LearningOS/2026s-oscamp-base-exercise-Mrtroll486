//! # Mutex Shared State
//!
//! In this exercise, you will use `Arc<Mutex<T>>` to safely share and modify data between multiple threads.
//!
//! ## Concepts
//! - `Mutex<T>` mutex protects shared data
//! - `Arc<T>` atomic reference counting enables cross-thread sharing
//! - `lock()` acquires the lock and accesses data

use std::sync::{Arc, Mutex};
use std::thread;

/// Increment a counter concurrently using `n_threads` threads.
/// Each thread increments the counter `count_per_thread` times.
/// Returns the final counter value.
///
/// Hint: Use `Arc<Mutex<usize>>` as the shared counter.
pub fn concurrent_counter(n_threads: usize, count_per_thread: usize) -> usize {
    // Create Arc<Mutex<usize>> with initial value 0
    let counter = Arc::new(Mutex::new(0));
    // Spawn n_threads threads
    let mut threads = Vec::new();
    for _ in 0..n_threads {
        let counter = counter.clone();
        threads.push(thread::spawn(move || {
            // In each thread, lock() and increment count_per_thread times
            *counter.lock().unwrap() += count_per_thread;
        }))
    }
    // Join all threads, return final value
    for i in threads {
        i.join().unwrap();
    }
    
    Arc::try_unwrap(counter).unwrap().into_inner().unwrap()
}

/// Add elements to a shared vector concurrently using multiple threads.
/// Each thread pushes its own id (0..n_threads) to the vector.
/// Returns the sorted vector.
///
/// Hint: Use `Arc<Mutex<Vec<usize>>>`.
pub fn concurrent_collect(n_threads: usize) -> Vec<usize> {
    // Create Arc<Mutex<Vec<usize>>>
    let collected = Arc::new(Mutex::new(Vec::new()));
    // Each thread pushes its own id
    let mut pool = Vec::new();
    for i in 0..n_threads {
        let collected = collected.clone();
        pool.push(thread::spawn(move || {
            collected.lock().unwrap().push(i);
        }));
    }
    // After joining all threads, sort the result and return
    for i in pool {
        i.join().unwrap();
    }
    collected.lock().unwrap().sort_unstable();
    
    Arc::try_unwrap(collected).unwrap().into_inner().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_single_thread() {
        assert_eq!(concurrent_counter(1, 100), 100);
    }

    #[test]
    fn test_counter_multi_thread() {
        assert_eq!(concurrent_counter(10, 100), 1000);
    }

    #[test]
    fn test_counter_zero() {
        assert_eq!(concurrent_counter(5, 0), 0);
    }

    #[test]
    fn test_collect() {
        let result = concurrent_collect(5);
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_collect_single() {
        assert_eq!(concurrent_collect(1), vec![0]);
    }
}
