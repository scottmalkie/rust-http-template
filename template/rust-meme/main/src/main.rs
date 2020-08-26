use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading from stdin!");

    let input: Vec<&str> = input.as_str().split_whitespace().collect();

    let mut result = String::new();

    for sub in &input {
        for (i, c) in sub.char_indices() {
            if i % 2 == 0 {
                result.push(c.to_lowercase().nth(0).unwrap());
            } else if i % 2 == 1 {
                result.push(c.to_uppercase().nth(0).unwrap());
            } else {
                result.push(c);
            }
        }
        result.push_str(" ");
    }

    println!("{}", result);
}
