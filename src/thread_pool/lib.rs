extern crate reqwest;
extern crate select;

use std::thread;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct ThreadPool {
  pub workers: Vec<Worker>,
  sender: Arc<Mutex<mpsc::Sender<String>>>,
}

pub struct Worker {
  pub id: usize,
  pub thread: std::thread::JoinHandle<String>,
  pub working: bool,
}

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));
    let sender   = Arc::new(Mutex::new(sender));
    let mut receivers = Vec::new();

    for i in 0..size {
      receivers.push(Worker::new(i, Arc::clone(&receiver)));
    }

    ThreadPool {
      workers: receivers,
      sender: sender,
    }
  }

  pub fn execute(&self, url: String) {
    self.sender.lock().unwrap().send(url).unwrap();
  }
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<String>>>) -> Worker {
    let mut working = false;

    let thread = thread::spawn(move || {
        loop {
          working = false;

          let lock = match receiver.lock() {
            Ok(r) => r,
            _ => continue,
          };

          if let Ok(r) = lock.recv() {
            match save_image(&r) {
              Ok(_) => println!("thread: {} - {} OK!", id, &r),
              err   => println!("{:#?}", err),
            };
          }

          // working = true;
        }
      });

    Worker{
      id: id,
      thread: thread,
      working: working,
    }
  }
}

fn save_image(src: &String) -> Result<(), Box<Error>> {
    if src.is_empty() {
      return Ok(());
    }
    let mut buff: Vec<u8> = Vec::new();
    let mut res = reqwest::get(src)?;

    // std::fs::create_dir("images").expect("create folder failed");
    res.read_to_end(&mut buff)?;
    
    if buff.len() < 30000 {
      return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "too small")));
    }

    let fname = src.split("/").last().unwrap();
    let fname = format!("images/{}", fname);

    let mut file = File::create(fname)?;
    file.write_all(&mut buff)?;
    Ok(())
}
