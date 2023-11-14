use discrete_mathematics::modular;
use discrete_mathematics::primality::{tests, gen};
use discrete_mathematics::cryptography::rsa;
use num_bigint::BigUint;

use crate::test_lib::interface;

use std::time::Instant;

pub fn cryptography_job() {
    let (init_info, data_type, keys) = interface::receive_crypto();
    
    let n: BigUint; 
    let e: BigUint;
    let f: BigUint;
   
    if keys.0.to_u32_digits()[0] != 1 {
        n = keys.0;
        e = keys.1;
        f = keys.2;
    }

    else {
        let p: BigUint = BigUint::from(51001u32);
        let q: BigUint = BigUint::from(41843u32);
        let theta: BigUint = (&p - 1u32) * (&q - 1u32);
        n = p*q;
        (f, e) = modular::find_n_and_inverse(&theta);
    }

    let encoded = rsa::process(&init_info, &e, &n);
    println!("{:?}", encoded);

    let decoded = rsa::process(&encoded, &f, &n);
    data_type.print(&decoded)
}

pub fn check_prime_job() {
    let number = interface::receive_prime_check();
    let primality = tests::is_prime(&number);

    if primality == true {println!("Your number {number} is prime!");}
    else {println!("Your number {number} is not prime!\n:/");}
}

pub fn check_pseudoprime_job() {
    let (number, base) = interface::receive_pseudoprime_check();

    let pseudoprimality: u16 = tests::is_pseudo_prime(&number, &base);

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

    let strong_pseudoprimality: u16 = tests::is_strong_pseudo_prime(&number, &base);

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
    
    let start = Instant::now();
    let prime = gen::generate_first_prime_from(&digits);
    let duration = start.elapsed(); 
    
    println!("The first prime of {digits} digits is {prime}!");
    println!("It was discovered after {:?}.", duration);
}

