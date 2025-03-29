use std::{
    collections::HashMap, process::exit, sync::{Arc, Mutex}, thread, time::Duration
};

pub fn livelock() {
    
        let file = Arc::new(Mutex::new(String::new()));
        let printer = Arc::new(Mutex::new(String::new()));
        let file_a = file.clone();
        let printer_b = printer.clone();

        let thread_a = thread::spawn(move || loop {
            let printing = printer.lock().unwrap();
            println!("Printer aquired by thread a ");
            thread::sleep(Duration::from_secs(1));

            match file_a.try_lock() {
                Ok(_) => {
                    println!("thread a has both printer and file");
                    exit(0) //exits if thread 'a' locks the file and printer.
                }
                Err(e) => {
                    println!("Thread a failed to aquire file: {}", e);
                    thread::sleep(Duration::from_secs(1));
                    drop(printing);
                    thread::sleep(Duration::from_secs(1));
                    if let Ok(_) = file_a.try_lock() {
                        println!("Now thread a  has the file but no printer")
                    }
                }
            };
        });

        let thread_b = thread::spawn(move || loop {
            let file = file.lock().unwrap();
            println!("File aquired by thread b ");
            thread::sleep(Duration::from_secs(1));

            match printer_b.try_lock() {
                Ok(_) => {
                    println!("Thread b has both file and printer");
                    exit(0) //exits if thread 'b' locks the file and printer.
                }
                Err(e) => {
                    println!("Thread b failed to aquire printer: {}", e);
                    thread::sleep(Duration::from_secs(2));
                    drop(file);
                    if let Ok(_) = printer_b.try_lock() {
                        println!("Now thread b has the printer but no file")
                    }
                }
            }
        });

        thread_a.join().unwrap();
        thread_b.join().unwrap();
    }

