// Use of Arc -> https://doc.rust-lang.org/std/sync/struct.Arc.html
// A thread-safe reference-counting pointer. ‘Arc’ stands for ‘Atomically Reference Counted’.
// !! Unlike Rc<T>, Arc<T> uses atomic operations for its reference counting.

use std::sync::{Arc};
use std::{thread, time};

// Run multiple threads, so we can be more efficient

fn main() {

    // Docs example
    println!("=========");

    let five = Arc::new(5);
    for _ in 0..10 {
        let five = Arc::clone(&five);
        let mut x = 1 as u8;
        thread::spawn(move || {
            x = x+1;
            println!("{}", x);
            println!("{}", five);
        });
    }

    // BHR book example, modified
    println!("=========");

    let pointer = Arc::new(6);
    let second_pointer = pointer.clone(); // or Arc::clone(&pointer)

    thread::spawn(move || {
    println!("{}", *second_pointer); // 6
    });

    thread::sleep(time::Duration::from_secs(2));
    println!("{}", *pointer); // 6

}
