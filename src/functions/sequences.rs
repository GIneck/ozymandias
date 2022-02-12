pub fn factorial(input: u8) -> u128{
    let mut counter = input as u128;
    let mut output: u128 = 0;

    while counter > 1 {

        output = output * counter;
        counter -= 1;
    }

    output
}

fn fibonacchi(term: u8) -> u128 {
    let mut counter = term;
    let mut output = 1;
    let mut previous;

    while counter != 0 {
        previous = output;
        output += previous;
        counter -= 1;
    }

    output
}