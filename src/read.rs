use rand::random;
use std::fs::File;
use std::io::{BufReader, Read, Seek};

pub fn txt_file(batch_size: usize) -> bool {
    let bytes_per_batch: usize = 4 + 1;
    let threads_count = 10;
    let batches_per_thread = batch_size / threads_count;

    let mut handles = vec![];

    let search = random::<u32>();

    for i in 0..threads_count {
        let start_offset = i * batches_per_thread * bytes_per_batch;
        let end_offset = (i + 1) * batches_per_thread * bytes_per_batch;

        let handle = std::thread::spawn(move || -> bool {
            let mut reader = BufReader::new(File::open("output.txt").unwrap());
            reader.seek(std::io::SeekFrom::Start(start_offset as u64)).unwrap();

            let mut local_buffer = vec![0; end_offset - start_offset];
            reader.read_exact(&mut local_buffer).unwrap();

            local_buffer.chunks_exact(5).find(|chunk|
                u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]) == search
            ).is_some()
        });

        handles.push(handle);
    }

    let mut found = false;
    for handle in handles {
        if handle.join().unwrap() {
            found = true
        }
    }

    found
}