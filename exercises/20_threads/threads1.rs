// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    (0..10).for_each(|i| {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis()
        }));
    });

    let mut results: Vec<u128> = vec![];
    handles.into_iter().for_each(|handle| {
        // TODO: a struct is returned from thread::spawn, can you use it?
        results.push(handle.join().unwrap())
    });

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    results.into_iter().enumerate().for_each(|(i, result)| {
        println!("thread {} took {}ms", i, result);
    });
}
