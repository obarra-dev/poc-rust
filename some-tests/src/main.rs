mod helpers;
mod operations;
use std::io;

fn main() {
    println!("Hello, world omar!");
    let greeting = greet("Barra");
    println!("{}", greeting);
    println!("z is {:?}", 4);

    let res = operations::sub(4, 5);
    println!("res is {:?}", res);

    let full_name: String = helpers::sub_module_helpers::get_full_name("omar", "barra");
    println!("full name is {:?}", full_name);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("faild to read line");
    print!("echo: {}", input);
}

fn greet(name: &str) -> String {
    format!("Hey {}, Hello, world!", name)
}

#[cfg(test)]
mod tests {
    use crate::greet;

    #[test]
    fn test_something_simple() {
        let greeting = greet("Omar");
        assert_eq!(greeting, "Hey Omar, Hello, world!");
    }
}
