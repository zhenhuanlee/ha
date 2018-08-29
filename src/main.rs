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


fn main() {
    let host = "http://dd.dety.men";
    let url = Arc::new(format!("{}/thread0806.php?fid=8", host));
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
    let pool = Arc::new(ThreadPool::new(1));
   
    for i in 1..10 {
        let pool = Arc::clone(&pool);
        let url = Arc::clone(&url);
        let handle = thread::spawn(move || {
            let foo = format!("{}&page={}", url, i);
            let text = reqwest::get(&foo).unwrap().text().unwrap();

            println!("analysing page {} ......", i);

            for node in Document::from(&text[..]).find(Name("a")) {
                let pool = Arc::clone(&pool);
                let too = match node.attr("href") {
                    Some(x) if x.contains("htm_data") => x,
                    _ => continue,
                };
                iter_url(&format!("{}/{}", host, too), pool);
            }
        });
        handles.push(handle);
    }

    // for h in handles {
    //     h.join().unwrap();
    // }

    loop {
        for w in &pool.workers {
            // if w.working {
            //     break;
            // }
        }
        // println!("{}", &pool.workers.len());
        thread::sleep(Duration::from_secs(1));
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
        
        // save_image(src, name);
        pool.execute(src.to_string());
    }
}