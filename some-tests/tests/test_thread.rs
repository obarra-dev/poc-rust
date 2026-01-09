use std::thread::spawn;

#[test]
fn simple_spawn_thread() {
    let function = || {
        let mut x = 0u128;
        for i in 1..500_000_000 {
            x += i ;
        }

        println!("{:?}", x);
    };

    let handle = spawn(function);
    let handle2 = spawn(function);
    handle.join().unwrap();
    handle2.join().unwrap();
}
