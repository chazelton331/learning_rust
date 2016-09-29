use std::thread;
use std::sync::{Arc,Mutex};
use std::vec::Vec;

fn main() {
    let number_of_threads   = 20;
    let mut thread_names    = Arc::new(Mutex::new(Vec::new()));

    for n in 0..number_of_threads {
        let thread_name = format!("thread-{}", n + 1);
        thread_names.push(thread_name);
    }

    for tn in &thread_names {
        thread::Builder::new().name(tn).spawn(move || {
            println!("{}", tn);
            //println!("{} -- {} -- {}", mp4_path_copy.lock(), ip_addr.lock(), thread_name.lock());
            //let mut command = ffmpeg_command(&mp4_path, &ip_addr, &thread_name);

            /*
            match command.output() {
                Err(why) => panic!("\n\nError running command from thread {}: {}\n\n", thread, why.desc),

                Ok(Output { error: err, output: _, status: exit }) => {
                    if exit.success() {
                        print!("\n\nThread {} finished with success\n\n", thread);
                    } else {
                        let string = String::from_utf8_lossy(&err);
                        print!("\n\nThread {} failed because {}\n\n", thread, string);
                    }
                },
            }
            */
        });

    }
}
