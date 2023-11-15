use integer_sqrt::IntegerSquareRoot;

/// This function finds an invertible n and its inverse in a moduler space of divisor, if the space
/// has no divisors, returns (1,1).
pub fn find_n_and_inverse(divisor: u64) -> (u64, u64){
    for number in divisor.integer_sqrt()..divisor {
        let mut bigger: u64 = divisor;
        let mut smaller: u64 = number % divisor;

        let mut actualq: i128;
        let mut lastq: i128 = 1;
        let mut lastlastq: i128 = 0;

        loop {
            let rest: u64 = bigger % smaller;
            actualq = lastlastq - (lastq * i128::from(bigger / smaller));

            match rest { 
                1 => {
                    let inverse = ((divisor as i128 + actualq) % divisor as i128) as u64;
                    if inverse < number {
                        return (number, ((divisor as i128 + actualq) % divisor as i128) as u64)
                    }                
                }
                
                0 => break,
                _ => (),
            }

            bigger = smaller;
            smaller = rest; 

            lastlastq = lastq;
            lastq = actualq;
        }
    }
    return (1,1);
}

/// This functions tries to invert a number in a modular space of divisor, if it is not inversible
/// returns 1.
pub fn inverse(number: u64, divisor: u64) -> u64{

    let mut bigger: u64 = divisor;
    let mut smaller: u64 = number % divisor;

    let mut actualq: i128;
    let mut lastq: i128 = 1;
    let mut lastlastq: i128 = 0;

    loop {
        let rest: u64 = bigger % smaller;
        actualq = lastlastq - (lastq * i128::from(bigger / smaller));

        match rest { 
            1 => {
                let inverse = ((divisor as i128 + actualq) % divisor as i128) as u64;
                if inverse < number {
                    return ((divisor as i128 + actualq) % divisor as i128) as u64;
                }                
            }
            
            0 => break,
            _ => (),
        }

        bigger = smaller;
        smaller = rest; 

        lastlastq = lastq;
        lastq = actualq;
    }
    return 1;
}

/// This function calculates the power of a number in a modular space. It calculates number^exp %
/// module
pub fn power_mod(number: u128, exp: u128, module: u128) -> u128 {
    let mut result: u128 = 1;
    for _ in 1..=exp{
        result = (result * number) % module;
    }

    return result;
}

