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

pub fn factorial(input: u8) -> u128{
    let mut counter = input as u128;
    let mut output: u128 = 0;

    while counter > 1 {

        output = output * counter;
        counter -= 1;
    }

    output
}

fn fibonacchi() {

}

pub fn create_polynomial() {

}