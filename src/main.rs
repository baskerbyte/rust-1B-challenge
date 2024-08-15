mod write;
mod read;

const MILLION: usize = 1_000_000;
const BILLION: usize = 1_000_000_000;
const TRILLION: usize = 1_000_000_000_000;

fn main() {
    env_logger::init();

    let batch_size = BILLION;
    let file = "output.txt".to_string();

    if batch_size == TRILLION {
        let batches = batch_size / MILLION;

        for _ in 0..batches {
            write::txt_file(batches, file.clone());
            if read::txt_file(batches, file.clone()) {
                break;
            } else {
                std::fs::remove_file(file.clone()).unwrap();
            }
        }
    } else {
        write::txt_file(batch_size, file.clone());
        read::txt_file(batch_size, file.clone());
    }
}

fn format_num(num: usize) -> String {
    num.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(",")
}

fn format_time(duration: std::time::Duration) -> String {
    let total_ms = duration.as_millis();

    match total_ms {
        ms if ms < 1000 => format!("{}ms", ms),
        ms if ms >= 1000 && ms < 60000 => format!("{}s", total_ms / 1000),
        _ => format!("{}m", total_ms / 60000),
    }
}