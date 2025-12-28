mod helpers;
mod operations;
use std::io;
use std::io::Write;

fn main() {
    println!("Hello, world omar!");
    let greeting = greet("Barra");
    println!("{}", greeting);
    println!("z is {:?}", 4);

    let res = operations::sub(4, 5);
    println!("res is {:?}", res);

    let full_name: String = helpers::sub_module_helpers::get_full_name("omar", "barra");
    println!("full name is {:?}", full_name);

    // Prompt the user for input
    print!("Enter input: ");
    // immediately flush the output buffer to ensure the prompt is displayed before waiting for input.
    io::stdout().flush().unwrap();

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
