use std::sync::{Arc, Mutex};
use std::thread;

pub fn update_count() {
    let counter = Arc::new(Mutex::new(2));
    let counter_clone = Arc::clone(&counter);

    let handle = thread::spawn(move || {
        let mut value = counter_clone.lock().unwrap();
        *value += 4;
    });

    handle.join().unwrap();

    let value = counter.lock().unwrap();
    println!("Result: {value}");
}
