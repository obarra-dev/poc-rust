use std::thread;
use std::time::Duration;

#[test]
fn simple_spawn_thread() {
    let function = || {
        let mut x = 0u128;
        for i in 1..500_000_000 {
            x += i ;
        }

        println!("{x}");
    };

    let max = 10u128;
    let function2 = move || {
        let mut x = 0u128;
        for i in 1..max {
            x += i ;
            thread::sleep(Duration::from_millis(10));
        }

        println!("{x}");
    };

    // TODO why still I can use max if the ownership was moved, if max is a String it does not compile which is expected
    println!("{max}");

    // argument of spaw function is a moving closure
    let handle = thread::spawn(function);
    let handle2 = thread::spawn(function2);
    handle.join().unwrap();
    handle2.join().unwrap();
}


#[test]
fn simple_spawssn_thread() {
    let max = 10u128;

    // The 'move' keyword transfers ownership of 'greeting' to the new thread
    let handle = thread::spawn(move || {
        println!("{}", max);
    });

    println!("{max}");

    // You can no longer use 'greeting' in the main thread after the move

    handle.join().unwrap();
}