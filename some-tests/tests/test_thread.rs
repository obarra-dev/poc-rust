use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
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

#[test]
fn channel_same_thread() {
    let (tx, rx) = mpsc::channel();

    tx.send("Omar").unwrap();
    tx.send("Barra").unwrap();

    let result = rx.recv();
    match result {
        Ok(v) => assert_eq!(v, "Omar"),
        Err(_) => panic!("this will not happen"),
    }

    let result = rx.recv();
    match result {
        Ok(v) => assert_eq!(v, "Barra"),
        Err(_) => panic!("this will not happen"),
    }

    // this compile but block since Sender was not closed
    //let result = rx.recv();
}

#[test]
fn channel_in_thread_error_closed_channel() {
    let (tx, rx)  = mpsc::channel();

    //it this is uncommented the rx is blocked again, TODO why?
   // let thread_tx = tx.clone();

    let child = thread::spawn(move || {
        tx.send("Omar").unwrap();
    });

    let result = rx.recv();
    match result {
        Ok(v) => assert_eq!(v, "Omar"),
        Err(_) => panic!("this will not happen"),
    }

    // for some reason, if the ownership of Sender is moved to a thread this is not a blocker
    // instead rx return an error
    // BUT if you do  let thread_tx = tx.clone(); the rx is blocked again, TODO why?
    let result = rx.recv();
    match result {
        Ok(v) => panic!("this will not happen"),
        Err(e) => {
            eprintln!("An error occurred: {}", e);
            // TODO is the correct way to get the message from error?
            assert_eq!(e.to_string(), "receiving on a closed channel");
        },
    }
}


#[test]
fn simple_channel_thread() {
    static NTHREADS: i32 = 3;

    // Channels have two endpoints: the `Sender<T>` and the `Receiver<T>`,
    // where `T` is the type of the message to be transferred
    // (type annotation is superfluous)
    let (tx, rx) = mpsc::channel();
    let mut children = Vec::new();

    for id in 0..NTHREADS {
        // The sender endpoint can be copied
        // If you do not clone,  the spawn does not compile as The thread takes ownership over `thread_tx`
        let thread_tx = tx.clone();


        // Each thread will send its id via the channel
        let child = thread::spawn(move || {
            // The thread takes ownership over `thread_tx`
            // Each thread queues a message in the channel
            thread_tx.send(id).unwrap();

            // Sending is a non-blocking operation, the thread will continue
            // immediately after sending its message
            println!("thread {} finished", id);
        });

        children.push(child);
    }

    // Here, all the messages are collected
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there are no messages available
        ids.push(rx.recv());
    }

    // Wait for the threads to complete any remaining work
    for child in children {
        child.join().expect("oops! the child thread panicked");
    }

    // Show the order in which the messages were sent
    println!("{:?}", ids);
}