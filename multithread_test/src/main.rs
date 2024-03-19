use std::thread;
use std::time::Duration;

fn main() {
    let a = ["qwertyuiop"];

    let b = [1, 2, 3, 4];

    // let result: Vec<_> = vec![];

    for i in b {
        let new_i = i;
        let handle = thread::spawn(move || {
            println!("hi number {} from the spawned thread!", new_i);
            thread::sleep(Duration::from_millis(1));
            new_i
        });

        let res = handle.join().unwrap();
        println!("Return {:?}", res);
    }


}
