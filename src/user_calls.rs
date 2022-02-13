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

    loop {

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");


        user_input.make_ascii_lowercase();
        let s = user_input.as_str();

        if s.contains("yes") | s.contains("y") {
            
            user_factorial();
        
        } else if s.contains("no") | s.contains("n") {

            println!("Thanks for working with me, come back anytime! (I could use the practice)"); 
        
            return

        } else {

            println!("Sorry, I cannot understand that. Could you reply with: 'Yes' or 'No' ")
        
        }     

    }

}

pub fn user_fibonacchi() {

    println!("the art of the fibonacci is one that some consider to be unnatural...");
    println!("we have mastered that art... we ... can share our knowledge...");
    println!("be warned, once you start down the path of sequences, you will not be able to stop...");
    println!("for sequences last ... forever");
    println!("if you are brave enough, enter the index of this sequence that you wish to find...");


    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let fn_input: u8 = user_input.trim().parse().expect("we require an integer ...");
    
    println!("the value of the fibonacci sequence at index {}, is {}", fn_input, crate::functions::sequences::fibonacchi(fn_input));

    println!("may we be of further service");

    loop {

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");


        user_input.make_ascii_lowercase();
        let s = user_input.as_str();

        if s.contains("yes") | s.contains("y") {
            
            user_factorial();
        
        } else if s.contains("no") | s.contains("n") {

            println!("we see, you may return later if you seek more knowledge"); 
        
            break

        } else {

            println!("we can only comprehend integers or two words: 'Yes' or 'No' ")
        
        }     

    }

}

pub fn begin_introductions() {
    println!("Welcome to the Ozymandias approximation system!");

    println!();

    println!("Unfortunaltely because my programmer is lazy and apparantely has better things to do, not all trancendentals in the world have been implemented.");

    println!();
    
    println!("Today on the menu we have:");

    println!();

    println!("Doctorial Factorials");

    println!("The Foggy Fibonacci");

    println!("Please select the function you would like to approximate:");

    println!();

    println!("(1) Doctorial Factorials");
    println!("(2) Foggy Fibonacchi");

    println!();

    println!("If you are done here, please enter 0");
}