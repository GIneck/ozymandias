use std::io;


pub fn user_factorial(){

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let fn_input: u8 = user_input.trim().parse().expect("Please type an Integer");

    crate::functions::factorial(fn_input);

}

pub fn user_fibonacchi() {
    
}