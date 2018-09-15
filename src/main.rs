extern crate reqwest;
extern crate select;
extern crate thread_pool;

use select::document::Document;
use select::predicate::{Name};
// use select::predicate::{Predicate, Attr, Class, Name};
use std::thread;
use std::sync::Arc;
use thread_pool::ThreadPool;
use std::time::Duration;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("config.txt").unwrap();
    let mut config = String::new();
    file.read_to_string(&mut config).unwrap();
    let arr = config.split(";").collect::<Vec<&str>>();
    let host = Arc::new(arr[0].to_owned());
    let size = arr[1].parse::<usize>().unwrap();
    let page = arr[2].parse::<u8>().unwrap();
    let url = Arc::new(format!("{}/thread0806.php?fid=8", host));
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
    let pool = Arc::new(ThreadPool::new(size));

    for i in 1..=page {
        let pool = Arc::clone(&pool);
        let url = Arc::clone(&url);
        let host = Arc::clone(&host);

        let handle = thread::spawn(move || {
            let foo = format!("{}&page={}", url, i);
            let text = reqwest::get(&foo).unwrap().text().unwrap();

            println!("analysing page {}...", i);

            for node in Document::from(&text[..]).find(Name("a")) {
                let pool = Arc::clone(&pool);
                let too = match node.attr("href") {
                    Some(x) if x.contains("htm_data") && !node.text().contains("歐美") => x,
                    _ => continue,
                };
                iter_url(&format!("{}/{}", host, too), pool);
            }
        });
        handles.push(handle);
    }

    loop {
        // if too == 0 {
        //     println!("Done!");
        // }
        thread::sleep(Duration::from_millis(500));
    }
}

fn iter_url(url: &str, pool: Arc<ThreadPool>) {
    let res = reqwest::get(url).unwrap()
        .text().expect("text error");

    let document = Document::from(&res[..]);

    for node in document.find(Name("input")) {
        let src = match node.attr("data-src") {
            Some(x) => x,
            _ => continue,
        };
        
        pool.execute(src.to_string());
    }
}