use std::io;
use ozymandias::user_calls;

fn main() {
    let mut input = String::new();
        loop {
            user_calls::begin_introductions();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let selection: u8 = input.trim().parse().expect("Please type an Integer");

    match selection {

        0 => break,
        1 => user_calls::user_factorial(),
        2 => user_calls::user_fibonacchi(),

        _ => println!("Give me a proper selection please")

    }
    input.drain(..);
}
}
