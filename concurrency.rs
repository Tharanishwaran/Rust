use std::thread;
use std::sync::{mpsc, Arc, Mutex};

fn concurrency() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let (tx, rx) = mpsc::channel();
    for _ in 0..5 {
        let (data, tx) = (Arc::clone(&data), tx.clone());
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data.push(4); 
            tx.send(()).unwrap();
        });
    }
    for _ in 0..5 {
        rx.recv().unwrap();
    }
    println!("Final Data: {:?}", data);
}
