use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::mpsc;

struct Worker {
    id: u8,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: u8, receiver: mpsc::Receiver<Job>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });
        Worker {
            id,
            thread,
        }
    }
}

struct Job;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    fn new(count: u8) -> ThreadPool {
        assert!(count > 0);

        let mut workers = Vec::<Worker>::with_capacity(4);

        let (sender, receiver) = mpsc::channel();

        for id in 0..count {
            workers.push(Worker::new(id, receiver));
        }

        ThreadPool {
            workers,
            sender
        }
    }
}

fn handle_stream(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "src/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "src/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8001").unwrap();

    let thread_pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_stream(stream);
    }
}
