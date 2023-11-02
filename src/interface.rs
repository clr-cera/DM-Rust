use std::io;

pub enum DataType {
    Number,
    Text,
}

impl DataType {
    pub fn print(&self, vector: &Vec<u64>){
        match self {
            Self::Number => println!("{:?}", vector),
            Self::Text => {
                for i in vector{
                    print!("{}", char::from_u32(*i as u32).unwrap());
                }

            }
        }
    }
}


pub fn receive() -> (Vec<u64>, DataType, (u64, u64, u64)) {
    println!("Select 0 if you want to enter numbers or 1 if you want to enter text!");
    let data_type: DataType = loop {
        match receive_number() {
            0 => break DataType::Number,
            1 => break DataType::Text,
            _ => println!("Please enter 0 or 1"),
        };
    };

    let mut number_vector: Vec<u64> = Vec::new();

    match data_type {
        DataType::Text => {
            let text: String = read_string();
            for char in text.chars() {
                number_vector.push(char as u64);
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

    let mut keys: (u64, u64, u64) = (0,0,0);
    if keys_choice == 1 {
        println!("Please enter your keys! First enter the module, then the encoding exponent, then the decoding exponent, respectively");
        keys.0 = receive_number();
        keys.1 = receive_number();
        keys.2 = receive_number();
    }

    return (number_vector,data_type, keys);

}

fn receive_number() -> u64{
    let mut string = String::new();
    io::stdin()
        .read_line(&mut string)
        .expect("Please enter a string");

    let number: u64 = string.trim().parse().expect("Please enter a number!"); 

    number
}

fn receive_number_check() -> Option<u64>{
    let mut string = String::new();
    io::stdin()
        .read_line(&mut string)
        .expect("Please enter a string");

    let number: Option<u64> = match string.trim().parse(){
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

