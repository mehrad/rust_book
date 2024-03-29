use std::thread;
use std::time::Duration;

fn main() {

    let v = vec![1, 2, 3];
    // try remove move 
    // also check the panic!
    thread::spawn(move || {
        for i in 1..10 {
            println!("not joined {} ", v[i] - i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("joined {} ", i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    handle.join().unwrap();
    for i in 1..5 {
        println!("main {} ", i);
        thread::sleep(Duration::from_millis(1));
    }


}