use crate::modular;

use num_bigint::{BigUint, BigInt};
use integer_sqrt::IntegerSquareRoot;

/// This function implements fermat test of composites, returning true if the number is a composite
/// and false if the number is a probable prime.
pub fn composite_test_fermat(number: &BigUint, test: &BigUint) -> bool{
    let fermat_result: BigUint = test.modpow(&(number-(1u32)), number);

    if fermat_result.to_u32_digits()[0] != 1 {return true;}
    else {return false;}
}

/// This function implements miller-rabin test of composites, returning true if the number is a composite
/// and false if the number is a strong probable prime.
pub fn composite_test_miller_rabin(number: &BigUint, base: &BigUint) -> bool{
    let (exp, constant) =  miller_decompose(number);

    let first = base.modpow(&constant, number);
    if first.to_u32_digits()[0] == 1 {return false}

    let mut r = BigUint::from(0u32);
    loop {
        let result = base.modpow(&(constant * BigUint::from(2u32).pow(r.to_u32_digits()[0])), number);
        if result == number-1u32 {return false}
        if r == exp-1u32 {break};
        r += 1u32
    }

    return true;

}

/// This function checks if a number is prime using miller-rabin test of composites, if it returns
/// true, then the informed number has 1/4^10 of probability of being a prime
pub fn is_prime_miller(number: &BigUint, base: &BigUint) -> bool {
    if number.to_u32_digits()[0] == 0 || number.to_u32_digits()[0] == 1 || number.to_u32_digits()[0] % 2 == 0 { 
        return false;
    }

    if composite_test_miller_rabin(number, base) == true {
        return false;
    }

    return true;
}

pub fn is_prime_fermat(number: &BigUint, base: &BigUint) -> bool {
    if number.to_u32_digits()[0] == 0 || number.to_u32_digits()[0] == 1 || number.to_u32_digits()[0] % 2 == 0 { 
        return false;
    }

    if composite_test_fermat(number, base) == true {
        return false;
    }


    return true;
}

pub fn is_prime_baillie_psw(number: &BigUint) -> bool {
    if number.to_u32_digits()[0] == 0 || number.to_u32_digits()[0] == 1 || number.to_u32_digits()[0] % 2 == 0 { 
        return false;
    }

    if composite_test_miller_rabin(number, &BigUint::from(2u32)) == true {
        return false;
    }

    let mut d: BigInt = BigInt::from(5);

    while jacobi(d, number) != -1{
        if d < BigInt::from(0) {
            d = (d-2) * -1;
        }
        else {
            d = (d+2) * -1;
        }
    }
    
    let p: BigInt = BigInt::from(1);
    let q: BigInt = (1 - d)/4;

    if composite_test_lucas(&number, &d, &p, &q) == true {
        return false;
    }


    return true;
}

pub fn is_prime(number: &BigUint) -> bool {
    if number.to_u32_digits()[0] == 0 || number.to_u32_digits()[0] == 1 || number.to_u32_digits()[0] % 2 == 0 { 
        return false;
    }
   
    let mut base = BigUint::from(2u32);
    loop {
        if composite_test_miller_rabin(number, &base) == true {
            return false;
        }
        if base.to_u32_digits()[0] == 10{
            break;
        }
        base +=1u32;

    }

    return true;
}

/// This function checks if a number is a pseudoprime. It returns 1 if the number is a pseudoprime,
/// 2 if the number is prime and 0 if the number is composite.
pub fn is_pseudo_prime(number: &BigUint, base: &BigUint) -> u16 {
    let fermat_result = composite_test_fermat(number , base);
    if fermat_result {return 0;}

    let brute_result = composite_test_bruteforce(number);

    if brute_result { return 1;}
    else { return 2;}
}

/// This function checks if a number is a strong pseudoprime. It returns 1 if the number is a strong pseudoprime,
/// 2 if the number is prime and 0 if the number is composite.
pub fn is_strong_pseudo_prime(number: &BigUint, base: &BigUint) -> u16 {
    let miller_result = composite_test_miller_rabin(number, base);
    if miller_result {return 0;}
    
    let brute_result = composite_test_bruteforce(number);

    if brute_result { return 1;}
    else { return 2;}

}

/// This function bruteforces its way on checking if a number is composite, it never mistakes but
/// has root of n complexity.
pub fn composite_test_bruteforce(number: &BigUint) -> bool{
    let mut i = BigUint::from(2u32);
    let zero = BigUint::from(0u32);

    loop {
        if number % i == zero {return true;}

        if i >= number.sqrt() {break}
        i += 1u32
    }
    return false;
} 

/// This function checks if a number is a composite using lucas probable prime test
pub fn composite_test_lucas(number: &BigUint, d: &BigInt, p: &BigInt, q: &BigInt) -> bool {
    let one = BigInt::from(1);
    if (p.pow(2) - 4*q) / BigInt::from_biguint(num_bigint::Sign::Plus, *number) != one {
        return false
    }


    return true;
}

/// This function decomposes a number into number = 2^exp + q. It returns (exp, q).
fn miller_decompose(number: &BigUint) -> (BigUint, BigUint){
    let mut q = number.clone();
    if (q % 2u32).to_u32_digits()[0] == 0 {return (BigUint::from(0u32), BigUint::from(0u32));}

    let mut exp: BigUint = BigUint::from(0u32);
    q = q - 1u32;

    loop {
        if (q % 2u32).to_u32_digits()[0] != 0u32 {break;}

        q = q / 2u32;
        exp += 1u32;
    }

    (exp, q)
}

/// This function calculates the jacobi symbol of a and n.
fn jacobi(mut a: i128, mut n:i128) -> i16 {
    a = a % n;
    
    let mut t = 1;
    let mut r;

    while a != 0 {
        while a % 2 == 0 {
            a /= 2;
            r = n % 8;
            
            if r == 3 || r == 5{
                t = -t;
            }
        }

        r = n;
        n =a;
        a = r;
        
        if a % 4 == 3 && n % 4 == 3 {
            t = -t;
        }
        a = a % n;
    }

    if n == 1 {return t;} 
    else {return 0}
}
