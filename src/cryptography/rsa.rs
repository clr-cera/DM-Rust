use crate::modular;

fn transform(a: u64, e: u64, d: u64) -> u64{
    modular::power_mod(a as u128, e as u128, d as u128) as u64
}

pub fn process(vector : &Vec<u64>, e: u64, n: u64) -> Vec<u64>{
    let mut result: Vec<u64> = Vec::new();
    vector.clone_into(&mut result);

    for i in &mut result {
        *i = transform(*i, e, n);
    }
    result
}

