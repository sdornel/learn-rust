use std::fs;
use std::io;
use chrono::{DateTime, Utc};
use std::time::SystemTime;

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());

    let now = SystemTime::now();
    let files: Vec<(std::path::PathBuf, std::time::Duration, Option<std::time::SystemTime>, std::time::SystemTime)> =
        fs::read_dir(&path)
            .unwrap()
            .map(|res| {
                let entry = res.unwrap();
                let path = entry.path();
                let metadata = entry.metadata().unwrap();
                let modified = metadata.modified().unwrap();
                
                let created = metadata.created().ok();
                let age = now.duration_since(modified).unwrap();

                (path, age, created, modified)
            })
            .collect();

    println!("Files analyzed: {}", files.len());

    // print_file_metadata(&files, &now);

    let youngest = files.iter().min_by_key(|(_, age, _, _)| *age).unwrap();
    let oldest = files.iter().max_by_key(|(_, age, _, _)| *age).unwrap();
    let mut ages: Vec<std::time::Duration> = files.iter().map(|(_, age, _, _)| *age).collect();
    ages.sort();
    let median_age = ages[ages.len() / 2];

    println!("Youngest:");
    println!("  {} — {} seconds old", youngest.0.display(), youngest.1.as_secs());
    match youngest.2 {
        Some(created) => {
            let created_utc: DateTime<Utc> = (created).into();
            println!("  Created: {}", created_utc.format("%Y-%m-%d %H:%M:%S UTC"));
        }
        None => {
            println!("  Created: <not available>");
        }
    }
    println!("  Modified: {}", DateTime::<Utc>::from(youngest.3).format("%Y-%m-%d %H:%M:%S UTC"));
    println!("  Days ago: {}", youngest.1.as_secs_f64() / 86_400.0);
    println!();

    println!("Oldest:");
    println!("  {} — {} seconds old", oldest.0.display(), oldest.1.as_secs());
    match oldest.2 {
        Some(created) => {
            let created_utc: DateTime<Utc> = (created).into();
            println!("  Created: {}", created_utc.format("%Y-%m-%d %H:%M:%S UTC"));
        }
        None => {
            println!("  Created: <not available>");
        }
    }
    println!("  Modified: {}", DateTime::<Utc>::from(oldest.3).format("%Y-%m-%d %H:%M:%S UTC"));
    println!("  Days ago: {}", oldest.1.as_secs_f64() / 86_400.0);
    println!();

    println!("Median age: {} days)", median_age.as_secs_f64() / 86_400.0);
}

fn print_file_metadata(files: &Vec<(std::path::PathBuf, std::time::Duration, Option<std::time::SystemTime>, std::time::SystemTime)>, now: &std::time::SystemTime) {
    for (path, age, created, modified) in files {
        let modified_utc: DateTime<Utc> = DateTime::from(*modified);

        println!("File: {}", path.display());
        println!("  Age: {} seconds", age.as_secs());
        println!("  Modified: {}", modified_utc.format("%Y-%m-%d %H:%M:%S UTC"));

        match created {
            Some(ct) => {
                let created_utc: DateTime<Utc> = (*ct).into();
                println!("  Created: {}", created_utc.format("%Y-%m-%d %H:%M:%S UTC"));
            }
            None => {
                println!("  Created: <not available>");
            }
        }

        println!();
    }
}
