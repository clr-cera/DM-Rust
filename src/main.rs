use rsa::*;
mod interface;



fn main() {
    let (init_info, data_type, keys) = interface::receive();
    
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



