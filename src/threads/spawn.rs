use std::sync::{Arc, Mutex};

#[allow(dead_code)]
pub fn spawn() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            for _ in 0..100000 {
                let mut value = counter_clone.lock().unwrap();
                *value += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter: {}", *counter.lock().unwrap());
}
