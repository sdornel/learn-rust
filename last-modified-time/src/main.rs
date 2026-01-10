use std::fs;
use std::time::SystemTime;

fn main() {
    let path = "./tmp";

    let entries = fs::read_dir(path).expect("failed to read dir");

    let mut modified_since_threshold = false;
    let time = SystemTime::now();
    let cutoff = std::time::Duration::new(300, 0); // 5 minutes
    for entry in entries {
        let entry = entry.expect("failed to get entry");
        let metadata = entry.metadata().expect("failed to get metadata");
        let modified_time = metadata.modified().expect("failed to get modified time");

        if (modified_time) < (time - cutoff) {
            modified_since_threshold = true;
        } else {
            modified_since_threshold = false;
            break;
        }
    }

    if modified_since_threshold {
        println!("No new files for more than 5 minutes!");
    } else {
        println!("Process continuing");
    }
}
