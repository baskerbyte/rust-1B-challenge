use rand::random;
use std::fs::File;
use std::io::{BufReader, Read, Seek};

pub fn txt_file(batch_size: usize, file: String) -> bool {
    log::info!("Starting to read {} for {} random numbers", file, crate::format_num(batch_size));

    let bytes_per_batch = 4 + 1;
    let threads_count = std::thread::available_parallelism().unwrap().into();
    let batches_per_thread = batch_size / threads_count;
    let remainder = batch_size % threads_count;

    let mut handles = Vec::with_capacity(threads_count);

    let search = random::<u32>().to_be_bytes();

    for i in 0..threads_count {
        let batches_per_thread = if i < remainder {
            batches_per_thread + 1
        } else {
            batches_per_thread
        };

        let start_offset = i * batches_per_thread * bytes_per_batch;
        let end_offset = (i + 1) * batches_per_thread * bytes_per_batch;
        let file = file.clone();

        let handle = std::thread::spawn(move || -> bool {
            let mut reader = open_file(file);
            reader.seek(std::io::SeekFrom::Start(start_offset as u64)).unwrap();

            let mut local_buffer = vec![0; end_offset - start_offset];
            reader.read_exact(&mut local_buffer).unwrap();

            local_buffer.chunks_exact(5).find(|chunk|
                chunk[0..4] == search
            ).is_some()
        });

        handles.push(handle);
    }

    let mut found = false;
    let start = std::time::Instant::now();

    for handle in handles {
        if handle.join().unwrap() {
            found = true
        }
    }

    log::info!("Read {} of numbers in {}", crate::format_num(batch_size), crate::format_time(start.elapsed()));

    found
}

fn open_file(file: String) -> BufReader<File> {
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(file)
        .unwrap();

    BufReader::new(file)
}