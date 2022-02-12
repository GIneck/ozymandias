use std::io;


pub fn user_factorial(){

    println!();
    
    println!("Hi, I am a Doctorial student studying factorials. If you give an integer, I will give you that factorial.");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let fn_input: u8 = user_input.trim().parse().expect("Please type an Integer");

    println!("{}! is equal to: {}", fn_input, crate::functions::sequences::factorial(fn_input));

    println!();

    println!("Can I do anything else for you hon'?");

    let mut completion = false;

    while completion == false{

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");


        user_input.make_ascii_lowercase();
        let s = user_input.as_str();

        if s.contains("yes") | s.contains("y") {
            
            user_factorial();
        
        } else if s.contains("no") | s.contains("n") {

            println!("Thanks for working with me, come back anytime! (I could use the practice)"); 
        
            completion = true

        } else {

            println!("Sorry, I cannot understand that. Could you reply with: 'Yes' or 'No' ")
        
        }     

    }

}

pub fn user_fibonacchi() {
    
}

pub fn begin_introductions() {
    println!("Welcome to the Ozymandias approximation system!");

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