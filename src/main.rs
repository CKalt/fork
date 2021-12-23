use fork::{Fork, daemon};
use std::{thread, time, process};
use std::io::Write;
use std::fs::OpenOptions;

fn main() {
    println!("creating daemon process: pid={}", process::id());
    if let Ok(Fork::Child) = daemon(true, false) {
        let mut file = OpenOptions::new()
              .write(true)
              .create(true)
              .append(true)
              .open("fork.log")
              .unwrap();

        for i in 1..100 {
            if let Err(_) = 
                writeln!(file, "{}: I'm a new child process pid={}",
                        i, process::id()) {
               break;
            }
            thread::sleep(time::Duration::from_secs(5));
        }
    }
}
