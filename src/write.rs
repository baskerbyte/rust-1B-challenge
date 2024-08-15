use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};
use rand::random;

pub fn txt_file(numbers_amount: usize) {
    let threads_count = 10;
    let batch_size = numbers_amount / 10000;
    let numbers_per_thread = numbers_amount / threads_count;
    let batches_per_thread = numbers_per_thread / batch_size;

    let file = OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .open("output.txt")
        .unwrap();

    let writer = Arc::new(Mutex::new(BufWriter::new(file)));
    let mut handles = Vec::with_capacity(threads_count);

    let start = std::time::Instant::now();
    for _ in 0..threads_count {
        let writer_clone = Arc::clone(&writer);

        let handle = std::thread::spawn(move || {
            for _ in 0..batches_per_thread {
                let mut buffer = Vec::with_capacity(batches_per_thread * 5);

                for _ in 0..batch_size {
                    buffer.extend_from_slice(&random::<u32>().to_be_bytes());
                    buffer.push(b'\n');
                }

                let mut writer = writer_clone.lock().unwrap();
                writer.write_all(&buffer).unwrap();
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Generated {} numbers in {}s", numbers_amount, start.elapsed().as_secs());
}