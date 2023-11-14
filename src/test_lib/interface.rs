use std::io;
use std::{thread::sleep, time::Duration};

use num_bigint::BigUint;
use num_traits::Zero;


pub enum JobChoice {
    Crypto,
    CheckPrime,
    CheckPseudoPrime,
    CheckStrongPseudoPrime,
    GeneratePrimeFrom,
    Quit,
}

impl JobChoice {
    fn generate(choice: BigUint) -> Self {
        if choice.is_zero() {return Self::Crypto;}
        match choice.to_u32_digits()[0] {
            1 => Self::CheckPrime,
            2 => Self::CheckPseudoPrime,
            3 => Self::CheckStrongPseudoPrime,
            4 => Self::GeneratePrimeFrom,
            5 | _=> Self::Quit,
        }
    }
}

pub enum DataType {
    Number,
    Text,
}

impl DataType {
    pub fn print(&self, vector: &Vec<BigUint>){
        match self {
            Self::Number => println!("{:?}", vector),
            Self::Text => {
                for i in vector{
                    print!("{}", char::from_u32(i.to_u32_digits()[0]).unwrap());
                }
            }
        }
    }
}


pub fn receive_work() -> JobChoice {
    sleep(Duration::from_millis(500));
    println!("\nSelect the type of job you want to do:\n0 for cryptography\n1 to check a prime\n2 to check a pseudoprime\n3 to check a strong pseudoprime\n4 to generate a prime\n5 to quit");
    let choice = receive_number();

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    sleep(Duration::from_millis(500));
    JobChoice::generate(choice)
}

pub fn receive_crypto() -> (Vec<BigUint>, DataType, (BigUint, BigUint, BigUint)) {
    println!("Select 0 if you want to enter numbers or 1 if you want to enter text!");
    let data_type: DataType = loop {
        match receive_number().to_u32_digits()[0] {
            0 => break DataType::Number,
            1 => break DataType::Text,
            _ => println!("Please enter 0 or 1"),
        };
    };

    let mut number_vector: Vec<BigUint> = Vec::new();

    match data_type {
        DataType::Text => {
            println!("Enter the text:");
            let text: String = read_string();
            for char in text.chars() {
                number_vector.push(BigUint::from(char as u64));
            }
        }

        DataType::Number => {
            println!("Digit numbers, use a character to stop");
            loop {
                let input = match receive_number_check() {
                    Some(num) => num,
                    None => break,
                };
                number_vector.push(input)
            } 
        }
    }

    println!("Do you want to enter your own keys? If you want, enter 1, else enter 0");
    let keys_choice = receive_number();

    let mut keys: (BigUint, BigUint, BigUint) = (BigUint::from(1 as u32), BigUint::from(1 as u32), BigUint::from(1 as u32));
    if !keys_choice.is_zero() {
        println!("Please enter your keys! First enter the module, then the encoding exponent, then the decoding exponent, respectively");
        keys.0 = receive_number();
        keys.1 = receive_number();
        keys.2 = receive_number();
    }
    
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    sleep(Duration::from_millis(500));
    return (number_vector,data_type, keys);
}

pub fn receive_prime_check() -> BigUint {
    println!("Enter the number to check if it is a prime:");
    let num = receive_number();

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    sleep(Duration::from_millis(500));
    num
}

pub fn receive_pseudoprime_check() -> (BigUint, BigUint){
    println!("Enter the number to check if it is a pseudoprime and its base:");
    let result = (receive_number(), receive_number());
    
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    sleep(Duration::from_millis(500));
    result
}

pub fn receive_strong_pseudoprime_check() -> (BigUint, BigUint){
    println!("Enter the number to check if it is a strong pseudoprime and its base:");
    let result = (receive_number(), receive_number());
    
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    sleep(Duration::from_millis(500));
    result
}

pub fn receive_prime_generation() -> BigUint{
    println!("Enter the number of binary digits of your desired prime");
    let result = receive_number();
    
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    sleep(Duration::from_millis(500));
    result
}

fn receive_number() -> BigUint{
    let mut string = String::new();
    loop {
        io::stdin()
            .read_line(&mut string)
            .expect("Please enter a string");

        match string.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Please enter a number"),
        }; 
    }
}

fn receive_number_check() -> Option<BigUint>{
    let mut string = String::new();
    io::stdin()
        .read_line(&mut string)
        .expect("Please enter a string");

    let number: Option<BigUint> = match string.trim().parse(){
        Ok(num) => Some(num),
        Err(_) => {return None},
    };

    number
}

fn read_string() -> String {
    let mut str = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line");
    return str;
}

