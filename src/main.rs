mod write;
mod read;

const MILLION: usize = 1_000_000;
const BILLION: usize = 1_000_000_000;
const TRILLION: usize = 1_000_000_000_000;

fn main() {
    let batch_size: usize = BILLION;

    if batch_size == TRILLION {
        for _ in 0..(batch_size / MILLION) {
            write::txt_file(batch_size);
            if read::txt_file(batch_size) {
                break
            } else {
                std::fs::remove_file("output.txt").unwrap();
            }
        }
    } else {
        write::txt_file(batch_size);
        read::txt_file(batch_size);
    }
}