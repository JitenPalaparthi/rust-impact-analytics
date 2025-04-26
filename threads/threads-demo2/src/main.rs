use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let file = File::open("input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    // Create a shared count variable (protected by Mutex and shared with Arc)
    let count = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for (line_no, line_result) in reader.lines().enumerate() {
        let line = line_result.expect("Failed to read line");
        let count = Arc::clone(&count); // Clone the Arc to share with the thread

        let handle = thread::spawn(move || {
            let word_count = line.split_whitespace().count();

            // Lock the mutex to update the shared count safely
            let mut total = count.lock().unwrap();
            *total += word_count;

            println!("Line {}: {} words", line_no + 1, word_count);
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Access the final word count after all threads are done
    println!("Total word count: {}", *count.lock().unwrap());
}