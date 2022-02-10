use std::io;

fn main() {
    let mut selection = String::new();
    let mut failed_attempts = 0;
    
    begin_introductions();

    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line");
    
    let selection: u8 = selection.trim().parse().expect("Please type an Integer");

    match selection {

        1 => user_factorial(),
        2 => fibonacchi(),
        _ => {failure_to_comply(failed_attempts); failed_attempts += 1}

    }
}

fn failure_to_comply(attempts: i8) {
    match attempts {
        0 => println!("Hey, out of bounds inputs make me agitated. "),
        _ => println!("Very well") //TODO fix
    }
}

fn fibonacchi() {

}

fn user_factorial(){

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let fn_input: u8 = user_input.trim().parse().expect("Please type an Integer");

    factorial(fn_input);

}

fn factorial(input: u8) -> u128{
    let mut counter = input as u128;
    let mut output: u128 = 0;

    while counter > 1 {

        output = output * counter;
        counter -= 1;
    }

    output
}

fn create_polynomial() {

}

fn begin_introductions() {
    println!("Welcome to the Ozymandius approximation system!");

    println!();

    println!("Unfortunaltely because my programmer is lazy and apparantely has better things to do, not all trancendentals in the world have been implemented.");

    println!();
    
    println!("Today on the menu we have:");

    println!();

    println!("Doctorial Factorials");

    println!("Please select the function you would like to approximate:");

    println!();

    println!("(1) Doctorial Factorials");
    println!("(2) Foggy Fibonacchi");
}