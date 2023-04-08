use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let input_str: String = input
        .join(" ")
        .chars()
        .filter(|x| x.is_alphabetic())
        .map(|x| x.to_ascii_lowercase())
        .collect();

    let mut input_splitted: Vec<String> = vec!["".to_string(); worker_count];

    input_str.chars().enumerate().for_each(|(ind, v)| {
        input_splitted[ind % worker_count].push(v);
    });

    let shared_input = Arc::new(RwLock::new(input_splitted));

    let ans_lock = Arc::new(Mutex::new(HashMap::<char, usize>::new()));

    let mut handles = vec![];

    for index in 0..worker_count {
        let input_lock_cloned = shared_input.clone();
        let ans_lock_cloned = ans_lock.clone();
        let handle = thread::spawn(move || {
            let input_t = input_lock_cloned.read().unwrap();

            let mut local_ans = HashMap::<char, usize>::new();
            for cur in input_t[index].chars() {
                *local_ans.entry(cur).or_insert(0) += 1;
            }

            let mut ans_t = ans_lock_cloned.lock().unwrap();
            for cur in local_ans {
                *ans_t.entry(cur.0).or_insert(0) += cur.1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let x = ans_lock.lock().unwrap().clone();
    return x;
}
