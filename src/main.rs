extern crate reqwest;
extern crate select;

use std::fs::File;
use std::io::prelude::*;
use select::document::Document;
use select::predicate::{Predicate, Attr, Class, Name};


fn main() {
    // get_req("https://github.com/utkarshkukreti/select.rs/blob/master/examples/stackoverflow.rs");
    save_image("https://www.baidu.com/img/superlogo_c4d7df0a003d3db9b65e9ef0fe6da1ec.png?where=super");
}

fn get_req(url: &str) {
    let res = reqwest::get(url)
        .expect("get error")
        .text()
        .expect("text error");

    let document = Document::from(&res[..]);

    for node in document.find(Name("img")) {
        println!("{:?}", node);
    }
}

fn save_image(url: &str) {
    let mut buff: Vec<u8> = Vec::new();
    let image_path = "test.png";
    let mut res = reqwest::get(url).expect("get errpr");
    res.read_to_end(&mut buff).unwrap();

    let mut file = File::create(image_path).expect("create file failed");
    file.write_all(&mut buff).unwrap();
}