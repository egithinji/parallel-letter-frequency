use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut h: HashMap<char, usize> = HashMap::new();

    if input.is_empty() {
        return h;
    }

    type Tally = HashMap<char, usize>;

    let (sender, receiver): (mpsc::Sender<Tally>, mpsc::Receiver<Tally>) = mpsc::channel();

    let workload = input.iter().map(|value| (*value).to_string()).collect();

    do_work(get_frequency, workload, sender, worker_count);

    for received in receiver {
        for (key, val) in received {
            let counter = h.entry(key).or_insert(0);
            *counter += val;
        }
    }

    h
}

fn get_frequency(input: Vec<String>) -> HashMap<char, usize> {
    let mut h = HashMap::new();
    for s in input {
        for char in s
            .chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_ascii_lowercase())
        {
            let counter = h.entry(char).or_insert(0);
            *counter += 1;
        }
    }

    h
}

pub fn do_work<T, U>(
    work: fn(Vec<T>) -> U,
    workload: Vec<T>,
    sender: mpsc::Sender<U>,
    worker_count: usize,
) where
    T: 'static + Send + std::marker::Sync + Clone,
    U: 'static + Send,
{
    let mut vec_chunks: Vec<Vec<T>> = Vec::with_capacity(worker_count);
    let chunk_size: f32 = (workload.len() / worker_count) as f32;
    let mut chunk_size = chunk_size.ceil() as usize;
    if chunk_size == 0 {
        chunk_size = 1;
    }

    for chunk in workload.chunks(chunk_size) {
        let sender = sender.clone();
        let c = chunk.to_vec();
        thread::spawn(move || {
            sender.send(work(c)).unwrap();
        });
    }
}
