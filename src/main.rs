extern crate reqwest;
extern crate select;

use std::fs::File;
use std::io::prelude::*;
use select::document::Document;
use select::predicate::{Predicate, Attr, Class, Name};
use std::thread;


fn main() {
    let host = "http://ab.cbcb.us";
    let url = "http://ab.cbcb.us/thread0806.php?fid=8";
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
   
    for i in 1..10 {
        let handle = thread::spawn(move || {
            println!("thread {} is working", i);
            let foo = format!("{}&page={}", url, i);
            let text = reqwest::get(&foo).unwrap().text().unwrap();
            for node in Document::from(&text[..]).find(Name("a")) {
                let too = match node.attr("href") {
                    Some(x) if x.contains("htm_data") => x,
                    _ => continue,
                };
                iter_url(&format!("{}/{}", host, too));
            }
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
}

fn iter_url(url: &str) {
    let res = reqwest::get(url).unwrap()
        .text().expect("text error");

    let document = Document::from(&res[..]);

    for node in document.find(Name("input")) {
        let src = match node.attr("data-src") {
            Some(x) => x,
            _ => continue,
        };
        let name = src.to_string();
        let name = name.split("/").last().expect("get image name failed");
        
        save_image(src, name);
    }
}

fn save_image(src: &str, fname: &str) {
    let mut buff: Vec<u8> = Vec::new();
    println!("{}", src);
    let res = reqwest::get(src);
    let mut res = match res {
        Ok(x) => x,
        Err(y) => {
            println!("{:?}", y);
            return ();
        },
    };
    // std::fs::create_dir("images").expect("create folder failed");
    res.read_to_end(&mut buff).expect("read buffer faield");
    
    if buff.len() < 30000 { return (); }

    let fname = format!("images/{}", fname);
    let mut file = File::create(fname).expect("create file failed");
    file.write_all(&mut buff).expect("write to file failed");
}