use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::hash::Hash;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {

    if input.is_empty() {
        return HashMap::new();
    }

    do_work(get_frequency, input, worker_count)
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

pub fn do_work<T>(work: fn(Vec<T>) -> HashMap<char,usize>, workload: &[&str], worker_count: usize) -> HashMap<char,usize> 
where
    T: 'static + Send + std::marker::Sync + Clone,
    Vec<T>: FromIterator<String>
{
    let mut vec_chunks: Vec<Vec<T>> = Vec::with_capacity(worker_count);
    let chunk_size: f32 = (workload.len() / worker_count) as f32;
    let mut chunk_size = chunk_size.ceil() as usize;
    if chunk_size == 0 {
        chunk_size = 1;
    }

    let mut threads = Vec::with_capacity(worker_count);

    for chunk in workload.chunks(chunk_size) {
        let c = chunk.iter().map(|&x| x.to_string()).collect();
        let t = thread::spawn(move || work(c));
        threads.push(t);
    }
    
    let mut hash = HashMap::new();

    for t in threads {
        let h = t.join().unwrap();
        for (key, val) in h {
            let counter = hash.entry(key).or_insert(0);
            *counter += val;
        }
    }

    hash
}
