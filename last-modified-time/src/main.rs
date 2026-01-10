use std::fs;
use std::time::{SystemTime, Duration};
use std::thread::sleep;

fn main() {
    let path = "./tmp";
    let mut alerted = false;

    loop {
        let entries = fs::read_dir(path).expect("failed to read dir");
        let mut all_files_old = true; // assume all are old unless we find a new one
        let time = SystemTime::now();
        let cutoff = std::time::Duration::new(300, 0); // 5 minutes
        for entry in entries {
            let entry = entry.expect("failed to get entry");
            let metadata = entry.metadata().expect("failed to get metadata");
            let modified_time = metadata.modified().expect("failed to get modified time");
    
            if (modified_time) > (time - cutoff) {
                all_files_old = false;
                break;
            } 
        }
        
        if all_files_old {
            println!("No new files for more than 5 minutes!");
        } else {
            println!("Process continuing");
        }
        sleep(Duration::from_secs(60));
    }

}
