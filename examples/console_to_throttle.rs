extern crate libraildriver;

use std::io;

fn main() {
    let context = libraildriver::Context::new();

    loop {
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).expect("Failed to read from stdin.");

        let trimmed = inp.trim();
        if trimmed.eq_ignore_ascii_case("quit") {
            break;
        }
        match trimmed.parse::<i32>() {
            Ok(i) => {
                context.set_value(libraildriver::Value::Throttle, i).expect("Failed to set value.");
            },
            Err(_) => {
                println!("Failed to read {} as a number. Please enter another...", trimmed);
            }
        };
    }
}
