use std::collections::VecDeque;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time;

const DURATION_WAITER_TRYQUEUE: u64 = 10;
const DURATION_WAITER_TAKEFOOD: u64 = 100;
const DURATION_WAITER_SERVEFOOD: u64 = 100;
const DURATION_WAITER_RESTUP: u64 = 500;
const DURATION_HOST_CONSUME: u64 = 1000;

const COURSES: [&str; 3] = ["POLÉVKA", "HLAVNÍ JÍDLO", "ZÁKUSEK"];
const N_HOSTS: u32 = 5;
const N_WAITERS: u32 = 7;

struct Request {
    host_id: u32,
    course_index: usize,
    transmitter: mpsc::Sender<()>,
}

fn main() {
    println!("Startujem.\n");

    let queue = Arc::new(Mutex::new(VecDeque::<Request>::new()));

    let mut host_handles = vec![];

    for host_id in 0..N_HOSTS {
        let queue = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            for (course_index, course) in COURSES.iter().enumerate() {
                let (transmitter, receiver) = mpsc::channel::<()>();
                {
                    let mut requests = queue.lock().unwrap();
                    requests.push_back(Request {
                        host_id,
                        course_index,
                        transmitter,
                    });
                }
                receiver.recv().unwrap();

                println!("Host {host_id} konzumuje {course}.");
                thread::sleep(time::Duration::from_millis(DURATION_HOST_CONSUME));
                println!("Host {host_id} dojedl {course}.");
            }
        });
        host_handles.push(handle);
    }

    for waiter_n in 0..N_WAITERS {
        let queue = Arc::clone(&queue);
        thread::spawn(move || {
            loop {
                let request = loop {
                    {
                        let mut requests = queue.lock().unwrap();
                        if !requests.is_empty() {
                            break requests.pop_front().unwrap();
                        }
                    }
                    thread::sleep(time::Duration::from_millis(DURATION_WAITER_TRYQUEUE));
                };

                println!(
                    "Číšník {} bere {} pro hosta {}.",
                    waiter_n, COURSES[request.course_index], request.host_id
                );
                thread::sleep(time::Duration::from_millis(DURATION_WAITER_TAKEFOOD));

                println!(
                    "Číšník {} servíruje {} pro hosta {}.",
                    waiter_n, COURSES[request.course_index], request.host_id
                );
                thread::sleep(time::Duration::from_millis(DURATION_WAITER_SERVEFOOD));

                request.transmitter.send(()).unwrap();

                println!("Číšník {waiter_n} jde na cigáro.");
                thread::sleep(time::Duration::from_millis(DURATION_WAITER_RESTUP));
            }
        });
    }

    for host_handle in host_handles {
        host_handle.join().unwrap();
    }

    println!("\nHotovo.");
}
