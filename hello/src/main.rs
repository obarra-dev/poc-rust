mod operation;
use std::io;

fn main() {
    println!("Hello, world omar!");
    println!("z is {:?}", 4);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("faild to read line");
    print!("{}", input);
}
