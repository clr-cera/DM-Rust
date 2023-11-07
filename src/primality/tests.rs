use crate::modular;

use integer_sqrt::IntegerSquareRoot;

pub fn composite_test_fermat(number: u128, test: u128) -> bool{
    let fermat_result: u128 = modular::power_mod(test, number-1, number);

    if fermat_result != 1 {return true;}
    else {return false;}
}

pub fn composite_test_miller_rabin(number: u128, base: u128) -> bool{
    let (exp, constant) =  miller_decompose(number);

    let first = modular::power_mod(base, constant, number);
    if first == 1 {return false}

    for r in 0..exp {
        let result = modular::power_mod(base, constant * modular::power_mod(2, r as u128, number), number);
        if result == number-1 {return false}
    }

    return true;

}

pub fn is_prime(number: u128) -> bool {
    for i in 2..number/16{
        if composite_test_miller_rabin(number, i) == true {
            return false;
        }
    }
    if composite_test_bruteforce(number) == true {
        return false;
    }

    return true;
}

pub fn is_pseudo_prime(number: u128, base: u128) -> u16 {
    let fermat_result = composite_test_fermat(number as u128, base as u128);
    let brute_result = composite_test_bruteforce(number as u128);

    if fermat_result == false && brute_result == true { return 1;}
    else if brute_result == false { return 2;}
    else {return 0;}

}

pub fn composite_test_bruteforce(number: u128) -> bool{
    for i in 2..=number.integer_sqrt() {
        if number % i == 0 {return true;}
    }
    return false;
} 

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


