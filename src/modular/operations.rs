use num_bigint::BigUint;

/// This function finds an invertible n and its inverse in a moduler space of divisor, if the space
/// has no divisors, returns (1,1).
pub fn find_n_and_inverse(divisor: &BigUint) -> (BigUint, BigUint){
    let mut number = divisor.sqrt();
    loop {
        let mut bigger: BigUint = (*divisor).clone();
        let mut smaller: BigUint = &number % divisor;

        let mut actualq: BigUint;
        let mut lastq: BigUint = BigUint::from(1u32);
        let mut lastlastq: BigUint = BigUint::from(0u32);

        loop {
            let rest: BigUint = &bigger % &smaller;
            actualq = lastlastq - (&lastq * (&bigger / &smaller));

            match rest.to_u64_digits()[0] { 
                1 => {
                    let inverse = (divisor + &actualq) % divisor;
                    if inverse < number {
                        return (number, ((divisor + actualq) % divisor))
                    }                
                }
                
                0 => break,
                _ => (),
            }

            bigger = smaller.clone();
            smaller = rest.clone(); 

            lastlastq = lastq.clone();
            lastq = actualq.clone();
        }

        if number == divisor-1u32 {break}
        number +=1u32;
    }
    return (BigUint::from(1u32),BigUint::from(1u32));
}

/// This functions tries to invert a number in a modular space of divisor, if it is not inversible
/// returns 1.
pub fn inverse(number: &BigUint, divisor: &BigUint) -> BigUint{

    let mut bigger: BigUint = divisor.clone();
    let mut smaller: BigUint = number % divisor;

    let mut actualq: BigUint;
    let mut lastq: BigUint = BigUint::from(1 as u32);
    let mut lastlastq: BigUint = BigUint::from(0 as u32);

    loop {
        let rest: BigUint = &bigger % &smaller;
        actualq = lastlastq - (&lastq * (&bigger / &smaller));

        match rest.to_u32_digits()[0] { 
            1 => {
                let inverse = (divisor + &actualq) % divisor;
                if inverse < *number {
                    return (divisor + &actualq) % divisor;
                }                
            }
            
            0 => break,
            _ => (),
        }

        bigger = smaller.clone();
        smaller = rest.clone(); 

        lastlastq = lastq.clone();
        lastq = actualq.clone();
    }
    return BigUint::from(1 as u32);
}
