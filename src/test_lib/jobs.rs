use discrete_mathematics::modular;
use discrete_mathematics::primality::{tests, gen};
use discrete_mathematics::cryptography::rsa;

use crate::test_lib::interface;

pub fn cryptography_job() {
    let (init_info, data_type, keys) = interface::receive_crypto();
    
    let n: u64; 
    let e: u64;
    let f: u64;
   
    if keys.0 != 0 {
        n = keys.0;
        e = keys.1;
        f = keys.2;
    }

    else {
        let p: u64 = 51001;
        let q: u64 = 41843;
        let theta: u64 = u64::from((p - 1) * (q - 1));
        n = u64::from(p * q);
        (f, e) = modular::find_n_and_inverse(theta);
    }

    let encoded = rsa::process(&init_info, e, n);
    println!("{:?}", encoded);

    let decoded = rsa::process(&encoded, f, n);
    data_type.print(&decoded)
}

pub fn check_prime_job() {
    let number = interface::receive_prime_check();
    let primality = tests::is_prime(number as u128);

    if primality == true {println!("Your number {number} is prime!");}
    else {println!("Your number {number} is not prime!\n:/");}
}

pub fn check_pseudoprime_job() {
    let (number, base) = interface::receive_pseudoprime_check();

    let pseudoprimality: u16 = tests::is_pseudo_prime(number as u128, base as u128);

    if pseudoprimality == 1 {
        println!("{number} is a pseudoprime in base {base}!");
    }

    else if pseudoprimality == 2 {
        println!("{number} is a prime!");
    }

    else {println!("{number} is just composite and {base} is one of its witnesses!");}
}

pub fn check_strong_pseudoprime_job() {
    let (number, base) = interface::receive_strong_pseudoprime_check();

    let strong_pseudoprimality: u16 = tests::is_strong_pseudo_prime(number as u128, base as u128);

    if strong_pseudoprimality == 1 {
        println!("{number} is a strong pseudoprime in base {base}!");
    }

    else if strong_pseudoprimality == 2 {
        println!("{number} is a prime!");
    }

    else {println!("{number} is just composite and {base} is one of its witnesses!");}
}
pub fn generate_prime_job() {
    let digits = interface::receive_prime_generation();
    let prime = gen::generate_prime_from(digits);
    println!("The first prime of {digits} digits is {prime}!");
}

