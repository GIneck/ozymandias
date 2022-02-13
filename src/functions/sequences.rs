pub fn factorial(input: u8) -> u128 {
    let mut counter = input as u128;
    let mut output: u128 = 1;

    while counter > 1 {

        output = output * counter;
        counter -= 1;
    }

    output
}

pub fn fibonacchi(term: u8) -> u128 {
    let mut counter = term;
    let mut output = 0;
    let mut latter = 0;
    let mut former = 1;

    while counter > 0 {

        output = latter + former;
        former = latter;
        latter = output;

        counter -= 1;
    }

    output
}