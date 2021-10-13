use std::convert::TryInto;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time;

extern crate num_cpus;

fn thread_demo() {
    let (sender, receiver) = mpsc::channel();
    let num: u64 = num_cpus::get().try_into().unwrap();
    let mut handles = vec![];
    println!("cpu num is {}", num);
    for id in 1..=num {
        let sd = sender.clone();
        let handle = thread::spawn(move || {
            thread::sleep(time::Duration::from_secs(id));
            sd.send(format!("thread id {} send msd", id)).unwrap();
        });

        handles.push(handle);
    }

    for received in receiver {
        println!("{}", received);
    }

    for handle in handles {
        handle.join().unwrap();
    }

}

// 互斥锁
fn thread_mutex() {
    // mutex demo
    let counter = Arc::new(Mutex::new(0));
    let num: u64 = num_cpus::get().try_into().unwrap();
    let mut handles = vec![];
    for id in 1..=num {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut val = counter.lock().unwrap();
            *val += id;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("mutex demo value is {}", *counter.lock().unwrap());
}

fn thread_mutex_mpsc() {
    // mutex demo
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));
    let mut handles = vec![];
    let num: u64 = num_cpus::get().try_into().unwrap();
    for id in 1..=num {
        let receiver = Arc::clone(&receiver);
        let handle = thread::spawn(move || {
            let receiver = receiver.lock().unwrap();
            println!("thread {} msg is {}", id, receiver.recv().unwrap());
        });
        handles.push(handle);
    }

    sender.send(String::from("你好")).unwrap();

    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    thread_mutex_mpsc();
    thread_mutex();
    thread_demo();
}
