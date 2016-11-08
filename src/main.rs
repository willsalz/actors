extern crate rand;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::{thread, time};
use rand::Rng;

static NTHREADS: i32 = 64;

fn main() {
    // Channels have two endpoints: the `Sender<T>` and the `Receiver<T>`,
    // where `T` is the type of the message to be transferred
    // (type annotation is superfluous)
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for id in 0..NTHREADS {
        // The sender endpoint can be copied
        let thread_tx = tx.clone();

        // Each thread will send its id via the channel
        thread::spawn(move || {

            let sleep = rand::thread_rng().gen_range(1, 1001);
            thread::sleep(time::Duration::from_millis(sleep));

            // The thread takes ownership over `thread_tx`
            // Each thread queues a message in the channel
            thread_tx.send(id).unwrap();

            // Sending is a non-blocking operation, the thread will continue
            // immediately after sending its message
            println!("thread {} finished", id);
        });
    }

    // Here, all the messages are collected
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there no messages available
        ids.push(rx.recv());
    }

    // Show the order in which the messages were sent
    println!("{:?}", ids);
}
