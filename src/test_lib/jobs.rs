use rsa::*;
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
        (f, e) = find_n_and_inverse(theta);
    }

    let encoded = process(&init_info, e, n);
    println!("{:?}", encoded);

    let decoded = process(&encoded, f, n);
    data_type.print(&decoded)
}

pub fn check_prime_job() {
    let number = interface::receive_prime_check();
    let primality = is_prime(number as u128);

    if primality == true {println!("Your number {number} is prime!");}
    else {println!("Your number {number} is not prime!\n:/");}
}

pub fn check_pseudoprime_job() {
    let (number, base) = interface::receive_pseudoprime_check();

    let pseudoprimality: u16 = is_pseudo_prime(number as u128, base as u128);

    if pseudoprimality == 1 {
        println!("Your number is a pseudoprime!");
    }

    else if pseudoprimality == 2 {
        println!("Your number is a prime!");
    }

    else {println!("Your number is just composite!");}
}

pub fn generate_prime_job() {

}

