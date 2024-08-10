use std::thread;

pub fn learning_threads() {
    let v = vec![1,23, 243];


    let handle = thread::spawn(move || {
        println!("vector {v:?}" );
    });

    handle.join().unwrap();
}