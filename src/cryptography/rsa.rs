use num_bigint::BigUint;

fn transform(a: &BigUint, e: &BigUint, d: &BigUint) -> BigUint{
    a.modpow(e, d)
}

/// This function encrypts a vector of u64 using keys e and denominator n.
pub fn process(vector : &Vec<BigUint>, e: &BigUint, n: &BigUint) -> Vec<BigUint>{
    let mut result: Vec<BigUint> = Vec::new();
    vector.clone_into(&mut result);

    for i in &mut result {
        *i = transform(i, e, n);
    }
    result
}

