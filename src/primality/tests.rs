use crate::modular;

use integer_sqrt::IntegerSquareRoot;

/// This function implements fermat test of composites, returning true if the number is a composite
/// and false if the number is a probable prime.
pub fn composite_test_fermat(number: u128, test: u128) -> bool{
    let fermat_result: u128 = modular::power_mod(test, number-1, number);

    if fermat_result != 1 {return true;}
    else {return false;}
}

/// This function implements miller-rabin test of composites, returning true if the number is a composite
/// and false if the number is a strong probable prime.
pub fn composite_test_miller_rabin(number: u128, base: u128) -> bool{
    let (exp, constant) =  miller_decompose(number);

    let first = modular::power_mod(base, constant, number);
    if first == 1 {return false}

    for r in 0..exp {
        let result = modular::power_mod(base, constant * 2u128.pow(r as u32), number);
        if result == number-1 {return false}
    }

    return true;

}

/// This function checks if a number is prime using miller-rabin test of composites, if it returns
/// true, then the informed number has 1/4^10 of probability of being a prime
pub fn is_prime(number: u128) -> bool {
    if number % 2 == 0 {return false;}

    for i in 2..=11{
        if composite_test_miller_rabin(number, i) == true {
            return false;
        }
    }

    return true;
}

/// This function checks if a number is a pseudoprime. It returns 1 if the number is a pseudoprime,
/// 2 if the number is prime and 0 if the number is composite.
pub fn is_pseudo_prime(number: u128, base: u128) -> u16 {
    let fermat_result = composite_test_fermat(number as u128, base as u128);
    let brute_result = composite_test_bruteforce(number as u128);

    if fermat_result == false && brute_result == true { return 1;}
    else if brute_result == false { return 2;}
    else {return 0;}

}

/// This function checks if a number is a strong pseudoprime. It returns 1 if the number is a strong pseudoprime,
/// 2 if the number is prime and 0 if the number is composite.
pub fn is_strong_pseudo_prime(number: u128, base: u128) -> u16 {
    let miller_result = composite_test_miller_rabin(number as u128, base as u128);
    let brute_result = composite_test_bruteforce(number as u128);

    if miller_result == false && brute_result == true { return 1;}
    else if brute_result == false { return 2;}
    else {return 0;}

}

/// This function bruteforces its way on checking if a number is composite, it never mistakes but
/// has root of n complexity.
pub fn composite_test_bruteforce(number: u128) -> bool{
    for i in 2..=number.integer_sqrt() {
        if number % i == 0 {return true;}
    }
    return false;
} 

/// This function decomposes a number into number = 2^exp + q. It returns (exp, q).
fn miller_decompose(mut number: u128) -> (u16, u128){
    if number % 2 == 0 {return (0, 0);}

    let mut exp: u16 = 0;
    number = number - 1;

    loop {
        if number % 2 != 0 {break;}

        number = number / 2;
        exp += 1;
    }
    (exp, number)
}


