// FIRST TEST
// basic scraping of URLs
// use futures::stream::{self, StreamExt};
// use reqwest::Client;
// use std::time::Instant;

// #[tokio::main]
// async fn main() {
//     let urls = std::fs::read_to_string("urls.txt")
//         .unwrap()
//         .lines()
//         .map(String::from)
//         .collect::<Vec<_>>();

//     let client = Client::new();
//     let concurrency = 100;
//     let now = Instant::now();

//     stream::iter(urls)
//         .map(|url| {
//             let client = client.clone();
//             async move {
//                 let _ = client.get(&url).send().await;
//             }
//         })
//         .buffer_unordered(concurrency)
//         .collect::<Vec<_>>()
//         .await;

//     println!("Rust: {:?}", now.elapsed());
// }



// SECOND TEST
// includes scraping and word counting
// use futures::stream::{self, StreamExt};
// use reqwest::Client;
// use scraper::{Html, Selector};
// use std::time::Instant;

// #[tokio::main]
// async fn main() {
//     let urls: Vec<String> = std::fs::read_to_string("urls.txt")
//         .unwrap()
//         .lines()
//         .map(String::from)
//         .collect();

//     let client = Client::new();
//     let selector = Selector::parse("p").unwrap();
//     let now = Instant::now();

//     stream::iter(urls.into_iter())
//         .map(|url| {
//             let client = client.clone();
//             let selector = selector.clone();
//             async move {
//                 if let Ok(resp) = client.get(&url).send().await {
//                     if let Ok(body) = resp.text().await {
//                         let document = Html::parse_document(&body);
//                         let text: String = document
//                             .select(&selector)
//                             .map(|e| e.text().collect::<String>())
//                             .collect();
//                         let word_count = text.split_whitespace().count();
//                         println!("{:<60} -> {} words", url, word_count);
//                     }
//                 }
//             }
//         })
//         .buffer_unordered(50)
//         .collect::<Vec<_>>()
//         .await;

//     println!("Rust Done in: {:?}", now.elapsed());
// }



// THIRD TEST
// print longest 3 summaries per page
use futures::stream::{self, StreamExt};
use reqwest::Client;
use scraper::{Html, Selector};
use std::time::Instant;

fn summarize(texts: Vec<String>) -> Vec<String> {
    let mut paragraphs: Vec<String> = texts
        .into_iter()
        .filter(|p| p.len() > 50)
        .collect();

    paragraphs.sort_by_key(|p| -(p.len() as isize));
    paragraphs.into_iter().take(3).collect()
}

#[tokio::main]
async fn main() {
    let urls: Vec<String> = std::fs::read_to_string("urls.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let client = Client::new();
    let selector = Selector::parse("p").unwrap();
    let now = Instant::now();

    stream::iter(urls)
        .for_each_concurrent(50, |url| {
            let client = client.clone();
            let selector = selector.clone();
            async move {
                if let Ok(resp) = client.get(&url).send().await {
                    if let Ok(body) = resp.text().await {
                        let doc = Html::parse_document(&body);
                        let paragraphs = doc
                            .select(&selector)
                            .map(|e| e.text().collect::<String>())
                            .collect::<Vec<_>>();

                        let summary = summarize(paragraphs);
                        println!("\n🦀 Summary for: {}\n", url);
                        for (i, p) in summary.iter().enumerate() {
                            println!("{}. {}\n", i + 1, p.trim());
                        }
                    }
                }
            }
        })
        .await;

    println!("Rust Done in: {:?}", now.elapsed());
}