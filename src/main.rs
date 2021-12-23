// The current version, shown below, uses the daemon function to cause
// the current program to run as a child process. The parent process
// apparently then terminates. The surviving process then continues to run
// and proceeds to append lines to the file named "fork.log" in the 
// current directory. The first arg to the daemon function, named `nochdir`,
// is set to true, otherwise the file io would target / and most likely
// fail if not run with root authority.
//
// The process id (pid) for both the parent is displayed at startup.
// The child process pids are included in the "fork.log" output.
// Testing showed this process will survive logging out from a Gnome Desktop
// session. Issuing a bash kill without params will terminate it.

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
