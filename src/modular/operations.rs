use integer_sqrt::IntegerSquareRoot;

use num_bigint::BigUint;

/// This function finds an invertible n and its inverse in a moduler space of divisor, if the space
/// has no divisors, returns (1,1).
pub fn find_n_and_inverse(divisor: &BigUint) -> (BigUint, BigUint){
    for number in divisor.integer_sqrt()..divisor {
        let mut bigger: BigUint = *divisor;
        let mut smaller: BigUint = number % divisor;

        let mut actualq: BigUint;
        let mut lastq: BigUint = BigUint::from(1);
        let mut lastlastq: BigUint = BigUint::from(0);

        loop {
            let rest: BigUint = bigger % smaller;
            actualq = lastlastq - (lastq * (bigger / smaller));

            match rest.to_u64_digits()[0] { 
                1 => {
                    let inverse = (divisor + actualq) % divisor;
                    if inverse < number {
                        return (number, ((divisor + actualq) % divisor))
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
    return (BigUint::from(1),BigUint::from(1));
}

/// This functions tries to invert a number in a modular space of divisor, if it is not inversible
/// returns 1.
pub fn inverse(number: &BigUint, divisor: &BigUint) -> BigUint{

    let mut bigger: BigUint = *divisor;
    let mut smaller: BigUint = number % divisor;

    let mut actualq: BigUint;
    let mut lastq: BigUint = BigUint::from(1 as u32);
    let mut lastlastq: BigUint = BigUint::from(0 as u32);

    loop {
        let rest: BigUint = bigger % smaller;
        actualq = lastlastq - (lastq * (bigger / smaller));

        match rest.to_u32_digits()[0] { 
            1 => {
                let inverse = (divisor + actualq) % divisor;
                if inverse < *number {
                    return (divisor + actualq) % divisor;
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
    return BigUint::from(1 as u32);
}
