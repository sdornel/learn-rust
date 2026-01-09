use std::fs;
use std::time::SystemTime;

fn main() {
    let path = "./tmp";

    let entries = fs::read_dir(path).expect("failed to read dir");
    println!("entries: {:?}", entries);
    for entry in entries {
        let entry = entry.expect("failed to get entry");
        let metadata = entry.metadata().expect("failed to get metadata");
        let modified_time = metadata.modified().expect("failed to get modified time");
        println!("modified_time: {:?}", modified_time);
        println!("5 mins ago: {:?}", SystemTime::now() - std::time::Duration::new(300, 0));
        if (modified_time) < (SystemTime::now() - std::time::Duration::new(300, 0)) {
            println!("File: {} was modified more than 5 minutes ago", entry.path().display());
        } else {
            println!("File: {} was modified within the last 5 minutes", entry.path().display());
        }
    }
}
