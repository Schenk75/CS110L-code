extern crate reqwest;
extern crate select;
#[macro_use]
extern crate error_chain;

use std::sync::{Arc, Mutex};
use std::{thread};
use select::document::Document;
use select::predicate::Name;

error_chain! {
   foreign_links {
       ReqError(reqwest::Error);
       IoError(std::io::Error);
   }
}

struct Article {
    url: String,
    len: usize,
}

const BATCH_SIZE: usize = 60;

// https://rust-lang-nursery.github.io/rust-cookbook/web/scraping.html
fn main() -> Result<()> {
    let body = reqwest::blocking::get("https://en.wikipedia.org/wiki/Multithreading_(computer_architecture)")?
    .text()?;
    // Identify all linked wikipedia pages
    let links = Document::from_read(body.as_bytes())?
        .find(Name("a"))
        .filter_map(|n| {
            if let Some(link_str) = n.attr("href") {
                if link_str.starts_with("/wiki/") {
                    Some(format!("{}/{}", "https://en.wikipedia.org",
                        &link_str[1..]))
                } else {
                    None
                }
            } else {
                None
            }
        }).collect::<Vec<String>>();
    let longest_article = Arc::new(Mutex::new(Article {url: "".to_string(),
        len: 0}));
    let num_batches = links.len()/BATCH_SIZE;
    println!("num_batches: {}", num_batches);
    for batch_idx in 0..num_batches {
        // println!("link: {}", link);
        println!("batch_idx: {}", batch_idx);
        let mut reqwesters = Vec::new();
        let start = batch_idx * BATCH_SIZE;
        let end = std::cmp::min((batch_idx + 1) * BATCH_SIZE, links.len());
        for link in &links[start..end] {
            let longest_article_clone = longest_article.clone();
            let link_clone = link.clone();
            reqwesters.push(thread::spawn(move || {
                let body = reqwest::blocking::get(&link_clone).expect("").text().expect("");
                let curr_len = body.len();
                let mut longest_article_ref = longest_article_clone.lock().unwrap();
                if curr_len > longest_article_ref.len {
                    longest_article_ref.len = curr_len;
                    longest_article_ref.url = link_clone.to_string();
                }
            }));
        }

        for handle in reqwesters {
            handle.join().expect("Panic occurred in thread!");
        }
        //println!("page length: {}", curr_len);
    }



    let longest_article_ref = longest_article.lock().unwrap();
    println!("{} was the longest article with length {}", longest_article_ref.url,
        longest_article_ref.len);
    Ok(())
}
