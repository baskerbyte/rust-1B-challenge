use rand::random;
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};

pub fn txt_file(numbers_amount: usize, file: String) {
    log::info!("Starting to write {} for {} random numbers", file, crate::format_num(numbers_amount));

    let threads_count = std::thread::available_parallelism().unwrap().into();
    let batch_size = numbers_amount / 10000;
    let numbers_per_thread = numbers_amount / threads_count;
    let remainder = numbers_amount % threads_count;
    let batches_per_thread = numbers_per_thread / batch_size;

    let writer = open_file(file);
    let mut handles = Vec::with_capacity(threads_count);

    for i in 0..threads_count {
        let batches_per_thread = if i < remainder {
            batches_per_thread + 1
        } else {
            batches_per_thread
        };
        let writer_clone = Arc::clone(&writer);

        let handle = std::thread::spawn(move || {
            for _ in 0..batches_per_thread {
                let buffer = generate_numbers(batches_per_thread, batch_size);

                let mut writer = writer_clone.lock().unwrap();
                writer.write_all(&buffer).unwrap();
            }
        });

        handles.push(handle);
    }

    let start = std::time::Instant::now();

    for handle in handles {
        handle.join().unwrap();
    }

    log::info!("Write {} of numbers in {}", crate::format_num(numbers_amount), crate::format_time(start.elapsed()));
}

fn open_file(file: String) -> Arc<Mutex<BufWriter<std::fs::File>>> {
    let file = std::fs::OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .open(file)
        .unwrap();

    Arc::new(Mutex::new(BufWriter::new(file)))
}

fn generate_numbers(
    batches_per_thread: usize,
    batch_size: usize,
) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(batches_per_thread * 5);

    for _ in 0..batch_size {
        buffer.extend_from_slice(&random::<u32>().to_be_bytes());
        buffer.push(b'\n');
    }

    buffer
}