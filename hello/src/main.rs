mod operations;
mod helpers;
use std::io;

fn main() {
    println!("Hello, world omar!");
    println!("z is {:?}", 4);

    let res = operations::sub(4, 5);
    println!("res is {:?}", res);

    let full_name = helpers::sub_module_helpers::get_full_name("omar", "barra");
    println!("full name is {:?}", full_name);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("faild to read line");
    print!("echo: {}", input);
}
