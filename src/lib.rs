extern crate integer_sqrt;

use integer_sqrt::IntegerSquareRoot;

fn transform(a: u64, e: u64, d: u64) -> u64{
    let mut result: u64 = 1;
    for _ in 1..=e {
        result = (result as u128 * a as u128) as u64 % d;
    }
    return result;
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

pub fn composite_test_miller_rabin(number: u128, basis: u128) -> bool{

}

fn power_mod(number: u128, exp: u128, module: u128) -> u128 {
    let mut result: u128 = 1;
    for _ in 1..=exp{
        result = (result * number) % module;
    }

    return result;
} 
