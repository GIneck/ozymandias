use std::io;
use ozymandias::functions;
use ozymandias::user_calls;

fn main() {
    let mut selection = String::new();
    let mut failed_attempts = 0;
    
    functions::begin_introductions();

    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line");
    
    let selection: u8 = selection.trim().parse().expect("Please type an Integer");

    match selection {

        1 => user_calls::user_factorial(),
        2 => user_calls::user_fibonacchi(),
        _ => {failure_to_comply(failed_attempts); failed_attempts += 1;}

    }
}

fn failure_to_comply(attempts: i8) {
    match attempts {
        0 => println!("Hey, out of bounds inputs make me agitated. "),
        _ => println!("Very well") //TODO fix
    }
}