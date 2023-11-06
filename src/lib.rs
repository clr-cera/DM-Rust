extern crate integer_sqrt;

use integer_sqrt::IntegerSquareRoot;


fn transform(a: u64, e: u64, d: u64) -> u64{
    power_mod(a as u128, e as u128, d as u128) as u64
}

pub fn process(vector : &Vec<u64>, e: u64, n: u64) -> Vec<u64>{
    let mut result: Vec<u64> = Vec::new();
    vector.clone_into(&mut result);

    for i in &mut result {
        *i = transform(*i, e, n);
    }
    result
}

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


pub fn composite_test_fermat(number: u128, test: u128) -> bool{
    let fermat_result: u128 = power_mod(test, number-1, number);

    if fermat_result != 1 {return true;}
    else {return false;}
}

pub fn composite_test_miller_rabin(number: u128, base: u128) -> bool{
    let (exp, constant) =  miller_decompose(number);

    let first = power_mod(base, constant, number);
    if first == 1 {return false}

    for r in 0..exp {
        let result = power_mod(base, constant * power_mod(2, r as u128, number), number);
        if result == number-1 {return false}
    }

    return true;

}

fn power_mod(number: u128, exp: u128, module: u128) -> u128 {
    let mut result: u128 = 1;
    for _ in 1..=exp{
        result = (result * number) % module;
    }

    return result;
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
