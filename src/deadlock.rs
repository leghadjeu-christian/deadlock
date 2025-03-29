use std::{
    sync::{Arc, Mutex},
    thread, time::Duration,
};

pub fn deadlock() {
    let file = Arc::new(Mutex::new(String::new()));
    let printer = Arc::new(Mutex::new(String::from("value")));

    let filea = file.clone();
    let printerb = printer.clone();

    let thread_a = thread::spawn(move || {
        let _printing = printer.lock().unwrap();
        println!("printer aquired by thread a ");

        thread::sleep(Duration::from_secs(1));

        let _file = filea.lock().unwrap();
        println!("file aquired by thread a");
    });
    let thread_b = thread::spawn(move || {
        let _file = file.lock().unwrap();
        println!("file aquired by thread b ");

        let _printting = printerb.lock().unwrap();
        println!("printer aquired by thread b");
    });

    thread_a.join().unwrap();
    thread_b.join().unwrap();
}
