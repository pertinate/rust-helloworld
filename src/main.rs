use std::io;

fn main() {
    println!("Please type the string: \"Hello World!\"");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    post_main(&input)
}

fn post_main(param1: &str) {
    println!("You wrote: {param1}");
}
